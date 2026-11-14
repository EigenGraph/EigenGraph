//! views/parametric/detailing_2d.rs — Automated RC Rebar BBS & Architectural Elevation / MEP Riser Slots.

use super::*;
use crate::core::{BimCommand, BimProject, RebarPreset, RunCategory};
use egui::{Align2, FontId, Pos2, Rect, ScrollArea, Sense};

#[derive(Clone, Debug, PartialEq)]
pub struct RebarBbsRow {
    pub bar_mark: String,
    pub iso_shape: String,
    pub host_id: usize,
    pub host_kind: String,
    pub diameter_mm: f64,
    pub count: usize,
    pub cut_length_mm: f64,
    pub total_weight_kg: f64,
}

/// Computes live ISO 3766 Bar Bending Schedule rows directly from active `BimProject.rebars`.
pub fn compute_rebar_bbs_rows(
    project: &BimProject,
    host_filter: Option<usize>,
    seismic_135_hooks: bool,
) -> Vec<RebarBbsRow> {
    project
        .rebars
        .iter()
        .filter(|b| host_filter.is_none_or(|hid| b.host_id == hid))
        .map(|b| {
            let prefix = match b.host_kind.as_str() {
                "Column" => "C",
                "Footing" => "F",
                _ => "B",
            };
            let suffix = if b.closed_stirrup { "ST" } else { "MN" };
            let shape = if b.closed_stirrup {
                if seismic_135_hooks {
                    "SC-51 (135° Seismic Stirrup)"
                } else {
                    "SC-50 (90° Standard Stirrup)"
                }
            } else if b.points_3d.len() > 2 {
                "SC-11 (L-Hook)"
            } else {
                "SC-00 (Straight)"
            };
            RebarBbsRow {
                bar_mark: format!("{}{}-{}", prefix, b.host_id, suffix),
                iso_shape: shape.to_string(),
                host_id: b.host_id,
                host_kind: b.host_kind.clone(),
                diameter_mm: b.diameter_mm,
                count: b.count,
                cut_length_mm: b.single_length_mm(),
                total_weight_kg: b.total_weight_kg(),
            }
        })
        .collect()
}

impl ParametricView {
    pub(super) fn render_rebar_bbs_detailing(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(ui, theme, features, FeatureId::AutomatedRcRebarBbsDrafter, |ui| {
            // Collect live RC structural hosts (Beams + Columns + Footings)
            #[derive(Clone)]
            struct RcHostInfo {
                id: usize,
                kind: &'static str,
                label: String,
                span_mm: f64,
                width_mm: f64,
                depth_mm: f64,
            }
            let mut hosts: Vec<RcHostInfo> = Vec::new();
            for r in &self.bim_bus.project.runs {
                if r.category == RunCategory::StructuralBeam && !r.is_wf_profile {
                    let len = r.start.dist(r.end).max(1000.0);
                    hosts.push(RcHostInfo {
                        id: r.id,
                        kind: "Beam",
                        label: format!("{} #{} ({:.0}×{:.0}, L={:.2}m)", r.name, r.id, r.width_mm, r.depth_mm, len * 1e-3),
                        span_mm: len,
                        width_mm: r.width_mm,
                        depth_mm: r.depth_mm,
                    });
                }
            }
            for c in &self.bim_bus.project.columns {
                if !c.is_wf_profile {
                    let h = (self.bim_bus.project.level_elevation(c.top_level_id)
                        - self.bim_bus.project.level_elevation(c.base_level_id))
                    .abs()
                    .max(3000.0);
                    hosts.push(RcHostInfo {
                        id: c.id,
                        kind: "Column",
                        label: format!("{} #{} ({:.0}×{:.0}, H={:.2}m)", c.name, c.id, c.width_mm, c.depth_mm, h * 1e-3),
                        span_mm: h,
                        width_mm: c.width_mm,
                        depth_mm: c.depth_mm,
                    });
                }
            }

            if !hosts.is_empty() && self.selected_rebar_host_idx >= hosts.len() {
                self.selected_rebar_host_idx = 0;
            }
            let selected_host = hosts.get(self.selected_rebar_host_idx).cloned();
            let host_rows = compute_rebar_bbs_rows(
                &self.bim_bus.project,
                selected_host.as_ref().map(|h| h.id),
                self.rebar_seismic_hooks_135,
            );
            let all_rows = compute_rebar_bbs_rows(&self.bim_bus.project, None, self.rebar_seismic_hooks_135);
            let display_rows = if host_rows.is_empty() { &all_rows } else { &host_rows };
            let host_weight: f64 = host_rows.iter().map(|r| r.total_weight_kg).sum();
            let total_weight = self.bim_bus.project.total_rebar_weight_kg();
            let main_bar_dia = display_rows
                .iter()
                .filter(|r| !r.iso_shape.starts_with("SC-5"))
                .map(|r| r.diameter_mm)
                .fold(22.0_f64, f64::max);
            let ld_mm = (main_bar_dia * 42.0).round();

            let host_title = selected_host
                .as_ref()
                .map(|h| format!("SELECTED RC {} ({:.0}×{:.0})", h.kind.to_uppercase(), h.width_mm, h.depth_mm))
                .unwrap_or_else(|| "RC WORKSPACE STATUS".to_string());
            let host_val = selected_host
                .as_ref()
                .map(|h| format!("Span {:.2} m • {} Bars", h.span_mm * 1e-3, host_rows.iter().map(|r| r.count).sum::<usize>()))
                .unwrap_or_else(|| "0 RC Members".to_string());
            let cards = [
                (
                    host_title.as_str(),
                    host_val.as_str(),
                    "ACI 318-19 §18.6.3 / SNI 2847 Flexural Bounds",
                    theme.accent_blue,
                ),
                (
                    "CONFINEMENT STIRRUPS",
                    if self.rebar_seismic_hooks_135 {
                        "D10-100 / 200mm (135°)"
                    } else {
                        "D10-150 Standard (90°)"
                    },
                    "l_dh = 10db Seismic End Hoops",
                    theme.accent_green,
                ),
                (
                    "DEVELOPMENT LENGTH (ld)",
                    &format!("{:.0} mm Tension Lap", ld_mm),
                    "f'c = 35 MPa • fy = 420 MPa BJTS 420B",
                    theme.accent_orange,
                ),
                (
                    "LIVE BIM REBAR STEEL MASS",
                    &format!("{:.1} kg (Host {:.1} kg)", total_weight, host_weight),
                    &format!("{} ISO 3766 Marks -> 9D CSO Sync", all_rows.len()),
                    theme.text_primary,
                ),
            ];
            ui_kit::responsive_grid_n(ui, cards.len(), 175.0, |ui, idx| {
                let (t, v, s, c) = cards[idx];
                ui_kit::stat_card(ui, theme, t, v, s, c);
            });

            ui.add_space(10.0);
            ui.horizontal_wrapped(|ui| {
                ui.checkbox(&mut self.rebar_seismic_hooks_135, "135° Seismic Hooks (SNI 2847 SMF)");

                if !hosts.is_empty() {
                    let cur_lbl = hosts[self.selected_rebar_host_idx].label.clone();
                    crate::ui_kit::wrap_before(ui, 220.0);
                    egui::ComboBox::from_id_salt("rebar_bbs_host_combo")
                        .width(crate::ui_kit::clamped_row_width(ui, 120.0, 260.0, 32.0))
                        .selected_text(RichText::new(cur_lbl).size(10.5))
                        .show_ui(ui, |ui| {
                            for (i, h) in hosts.iter().enumerate() {
                                if ui.selectable_label(self.selected_rebar_host_idx == i, &h.label).clicked() {
                                    self.selected_rebar_host_idx = i;
                                }
                            }
                        });
                }

                if icons::icon_button(
                    ui,
                    AppIcon::Rebar,
                    "Auto-Arm All RC Elements",
                    10.5,
                    Color32::WHITE,
                    theme.accent_blue,
                    Stroke::NONE,
                    CornerRadius::same(5),
                    Vec2::new(0.0, 24.0),
                )
                .on_hover_text("Generate live longitudinal bars & confinement stirrups for every concrete beam, column & footing")
                .clicked()
                {
                    let mut preset = RebarPreset::default();
                    if self.rebar_seismic_hooks_135 {
                        preset.stirrup_spacing_mm = 100.0;
                    }
                    match self.bim_bus.execute(BimCommand::ReinforceAllConcrete { preset }) {
                        Ok(msg) => {
                            self.command_history.push(msg.clone());
                            self.save_notification = Some(msg);
                            self.refresh_live_ari();
                        }
                        Err(e) => {
                            self.save_notification = Some(e);
                        }
                    }
                }
            });
            ui.add_space(10.0);

            ui_kit::responsive_two_panes(ui, 620.0, |ui, pane| match pane {
                0 => {
                    let elev_title = selected_host
                        .as_ref()
                        .map(|h| format!("{} REBAR ELEVATION ({:.0}×{:.0} mm)", h.kind.to_uppercase(), h.width_mm, h.depth_mm))
                        .unwrap_or_else(|| "RC LONGITUDINAL REBAR ELEVATION".to_string());
                    ui.label(RichText::new(elev_title).size(11.0).strong().color(theme.accent_blue));
                    ui.add_space(6.0);
                    let desired = Vec2::new(ui.available_width(), 155.0);
                    let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
                    let painter = ui.painter_at(rect);
                    painter.rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(12, 16, 22));
                    let beam = Rect::from_min_max(
                        Pos2::new(rect.left() + 22.0, rect.top() + 30.0),
                        Pos2::new(rect.right() - 22.0, rect.bottom() - 36.0),
                    );
                    painter.rect_stroke(beam, CornerRadius::ZERO, Stroke::new(1.5, theme.text_muted), StrokeKind::Inside);

                    if let Some(ref host) = selected_host {
                        painter.line_segment(
                            [Pos2::new(beam.left() + 6.0, beam.top() + 10.0), Pos2::new(beam.right() - 6.0, beam.top() + 10.0)],
                            Stroke::new(2.4, theme.accent_orange),
                        );
                        painter.line_segment(
                            [Pos2::new(beam.left() + 6.0, beam.bottom() - 10.0), Pos2::new(beam.right() - 6.0, beam.bottom() - 10.0)],
                            Stroke::new(2.4, theme.accent_green),
                        );
                        let hoop_n = if self.rebar_seismic_hooks_135 { 24 } else { 16 };
                        for i in 1..hoop_n {
                            let sx = beam.left() + (i as f32 / hoop_n as f32) * beam.width();
                            let in_lo = (i as f32 / hoop_n as f32) <= 0.25 || (i as f32 / hoop_n as f32) >= 0.75;
                            let col = if in_lo { theme.accent_blue } else { theme.text_secondary };
                            painter.line_segment(
                                [Pos2::new(sx, beam.top() + 6.0), Pos2::new(sx, beam.bottom() - 6.0)],
                                Stroke::new(1.2, col),
                            );
                        }
                        let lo_mm = (host.depth_mm * 2.0).max(600.0);
                        painter.text(
                            Pos2::new(beam.center().x, rect.bottom() - 12.0),
                            Align2::CENTER_CENTER,
                            format!(
                                "Span L = {:.2}m • Lo = {:.0}mm Confined Ends • {} Bars Armed",
                                host.span_mm * 1e-3,
                                lo_mm,
                                host_rows.iter().map(|r| r.count).sum::<usize>()
                            ),
                            FontId::proportional(10.0),
                            theme.text_secondary,
                        );
                    } else {
                        painter.text(
                            beam.center(),
                            Align2::CENTER_CENTER,
                            "Blank Workspace — Author RC Beams/Columns or instantiate a Structural Template to draft Rebar.",
                            FontId::proportional(10.5),
                            theme.text_muted,
                        );
                    }
                }
                _ => {
                    ui.label(RichText::new("LIVE BAR BENDING SCHEDULE (ISO 3766)").size(11.0).strong().color(theme.accent_blue));
                    ui.add_space(6.0);
                    if display_rows.is_empty() {
                        ui.label(
                            RichText::new("No reinforcement bars generated yet. Click 'Auto-Arm All RC Elements' once concrete hosts exist.")
                                .size(11.0)
                                .color(theme.text_muted),
                        );
                    } else {
                        ScrollArea::horizontal().id_salt("bbs_preview_table").show(ui, |ui| {
                            egui::Grid::new("bbs_preview_grid")
                                .striped(true)
                                .spacing(Vec2::new(12.0, 5.0))
                                .show(ui, |ui| {
                                    for h in ["Mark", "ISO Shape", "Dia", "Qty", "Cut Len", "Total (kg)"] {
                                        ui.label(RichText::new(h).size(10.5).strong().color(theme.accent_blue));
                                    }
                                    ui.end_row();
                                    for row in display_rows.iter().take(24) {
                                        ui.label(RichText::new(&row.bar_mark).size(10.5).monospace().color(theme.text_primary));
                                        ui.label(RichText::new(&row.iso_shape).size(10.5).color(theme.text_secondary));
                                        ui.label(RichText::new(format!("D{:.0}", row.diameter_mm)).size(10.5).strong().color(theme.accent_orange));
                                        ui.label(RichText::new(format!("{}", row.count)).size(10.5).color(theme.text_primary));
                                        ui.label(RichText::new(format!("{:.0} mm", row.cut_length_mm)).size(10.5).monospace().color(theme.text_muted));
                                        ui.label(RichText::new(format!("{:.1}", row.total_weight_kg)).size(10.5).strong().color(theme.accent_green));
                                        ui.end_row();
                                    }
                                });
                        });
                    }
                }
            });
        });
    }

    pub(super) fn render_elevations_mep_riser(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(ui, theme, features, FeatureId::BuildingElevationsMepRiser, |ui| {
            ui.horizontal_wrapped(|ui| {
                let _ = ui.selectable_label(true, "North Facade Elevation");
                let _ = ui.selectable_label(false, "East Facade Elevation");
                let _ = ui.selectable_label(false, "HVAC + Chilled Water Riser");
                let _ = ui.selectable_label(false, "Fire Standpipe + Sprinkler");
                let _ = icons::icon_button(
                    ui,
                    AppIcon::RulerSquare,
                    "Project All Facades & Riser Stacks to Sheets",
                    10.5,
                    theme.text_primary,
                    theme.bg_card,
                    Stroke::NONE,
                    CornerRadius::same(5),
                    Vec2::new(0.0, 24.0),
                );
            });
            ui.add_space(10.0);

            ui_kit::responsive_two_panes(ui, 620.0, |ui, pane| match pane {
                0 => {
                    ui.label(RichText::new("ORTHOGRAPHIC FACADE DATUM & BAY ELEVATION").size(11.0).strong().color(theme.accent_blue));
                    ui.add_space(6.0);
                    let desired = Vec2::new(ui.available_width(), 165.0);
                    let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
                    let painter = ui.painter_at(rect);
                    painter.rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(13, 17, 24));
                    let left = rect.left() + 58.0;
                    let right = rect.right() - 24.0;
                    for (i, lvl) in ["Roof +19.2m", "L5 +15.6m", "L4 +12.0m", "L3 +8.4m", "L2 +4.8m", "GF ±0.0m"].iter().enumerate() {
                        let y = rect.top() + 20.0 + (i as f32) * 23.0;
                        painter.line_segment([Pos2::new(left, y), Pos2::new(right, y)], Stroke::new(1.0, theme.text_muted.gamma_multiply(0.5)));
                        painter.text(Pos2::new(left - 6.0, y), Align2::RIGHT_CENTER, *lvl, FontId::proportional(9.5), theme.accent_blue);
                    }
                    let frame_rect = Rect::from_min_max(Pos2::new(left + 12.0, rect.top() + 20.0), Pos2::new(right - 12.0, rect.top() + 135.0));
                    painter.rect_stroke(frame_rect, CornerRadius::ZERO, Stroke::new(1.8, theme.accent_green), StrokeKind::Inside);
                    for col in 1..4 {
                        let cx = frame_rect.left() + (col as f32 / 4.0) * frame_rect.width();
                        painter.line_segment([Pos2::new(cx, frame_rect.top()), Pos2::new(cx, frame_rect.bottom())], Stroke::new(1.2, theme.text_secondary));
                    }
                }
                _ => {
                    ui.label(RichText::new("VERTICAL MEP SHAFT & FIRESTOP RISER SCHEDULE").size(11.0).strong().color(theme.accent_blue));
                    ui.add_space(6.0);
                    ScrollArea::horizontal().id_salt("mep_riser_table").show(ui, |ui| {
                        egui::Grid::new("mep_riser_grid")
                            .striped(true)
                            .spacing(Vec2::new(12.0, 5.0))
                            .show(ui, |ui| {
                                for h in ["Riser ID", "System", "Size / Spec", "Firestop Collar", "Head Loss"] {
                                    ui.label(RichText::new(h).size(10.5).strong().color(theme.accent_blue));
                                }
                                ui.end_row();
                                for (id, sys, spec, collar, drop) in [
                                    ("CHW-R01", "Chilled Water Supply", "DN200 Sch40", "2-Hr Intumescent UL", "14.2 kPa/100m"),
                                    ("FSP-R02", "Class I Fire Standpipe", "DN150 Galv.", "2-Hr Cast-In Sleeve", "38.0 kPa static"),
                                    ("HVC-D01", "Fresh Air Pressurization", "800×500 mm", "Fire Damper FD-90", "0.85 Pa/m"),
                                    ("ELV-B01", "Main 1600A Busway", "Cu IP55", "3-Hr Mortar Seal", "1.1% dV"),
                                ] {
                                    ui.label(RichText::new(id).size(10.5).monospace().color(theme.text_primary));
                                    ui.label(RichText::new(sys).size(10.5).color(theme.text_secondary));
                                    ui.label(RichText::new(spec).size(10.5).color(theme.text_muted));
                                    ui.label(RichText::new(collar).size(10.5).strong().color(theme.accent_green));
                                    ui.label(RichText::new(drop).size(10.5).monospace().color(theme.accent_orange));
                                    ui.end_row();
                                }
                            });
                    });
                }
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_rebar_bbs_rows_from_live_project() {
        let mut proj = BimProject::sample_bimcraft_building();
        let armed = proj.reinforce_all_concrete(&RebarPreset::default());
        assert!(armed > 0);
        let rows = compute_rebar_bbs_rows(&proj, None, true);
        assert!(!rows.is_empty());
        let sum_wt: f64 = rows.iter().map(|r| r.total_weight_kg).sum();
        assert!((sum_wt - proj.total_rebar_weight_kg()).abs() < 1e-3);
        assert!(rows.iter().any(|r| r.iso_shape.contains("SC-51")));
    }
}
