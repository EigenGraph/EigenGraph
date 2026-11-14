//! views/parametric/detailing_2d.rs — Automated RC Rebar BBS & Architectural Elevation / MEP Riser Slots.

use super::*;
use egui::{Align2, FontId, Pos2, Rect, ScrollArea, Sense};

impl ParametricView {
    pub(super) fn render_rebar_bbs_detailing(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(ui, theme, features, FeatureId::AutomatedRcRebarBbsDrafter, |ui| {
            let cards = [
                ("MAIN BEAM B-101 (400×700)", "3D22 Bot + 2D22 Top", "ACI 318-19 §18.6.3 Flexural Bounds", theme.accent_blue),
                ("CONFINEMENT STIRRUPS", "2D10-100 / 200mm", "135° Seismic Hooks (l_dh = 10db)", theme.accent_green),
                ("DEVELOPMENT LENGTH (ld)", "920 mm Tension Lap", "f'c = 35 MPa • fy = 420 MPa Class B", theme.accent_orange),
                ("TOTAL BEAM STEEL MASS", "1,482 kg • 3.8% Scrap", "Exported to 9D CSO Optimizer", theme.text_primary),
            ];
            ui_kit::responsive_grid_n(ui, cards.len(), 175.0, |ui, idx| {
                let (t, v, s, c) = cards[idx];
                ui_kit::stat_card(ui, theme, t, v, s, c);
            });

            ui.add_space(10.0);
            ui.horizontal_wrapped(|ui| {
                let _ = ui.selectable_label(true, "SNI 2847 / ACI 318 Special Moment Frame");
                let _ = ui.selectable_label(false, "135-deg Seismic Hooks");
                let _ = icons::icon_button(
                    ui,
                    AppIcon::Rebar,
                    "Auto-Draft Rebar Elevations & BBS Table",
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
                    ui.label(RichText::new("BEAM B-101 LONGITUDINAL REBAR ELEVATION").size(11.0).strong().color(theme.accent_blue));
                    ui.add_space(6.0);
                    let desired = Vec2::new(ui.available_width(), 145.0);
                    let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
                    let painter = ui.painter_at(rect);
                    painter.rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(12, 16, 22));
                    let beam = Rect::from_min_max(
                        Pos2::new(rect.left() + 22.0, rect.top() + 34.0),
                        Pos2::new(rect.right() - 22.0, rect.bottom() - 36.0),
                    );
                    painter.rect_stroke(beam, CornerRadius::ZERO, Stroke::new(1.5, theme.text_muted), StrokeKind::Inside);
                    painter.line_segment(
                        [Pos2::new(beam.left() + 6.0, beam.top() + 10.0), Pos2::new(beam.right() - 6.0, beam.top() + 10.0)],
                        Stroke::new(2.2, theme.accent_orange),
                    );
                    painter.line_segment(
                        [Pos2::new(beam.left() + 6.0, beam.bottom() - 10.0), Pos2::new(beam.right() - 6.0, beam.bottom() - 10.0)],
                        Stroke::new(2.2, theme.accent_green),
                    );
                    for i in 1..20 {
                        let sx = beam.left() + (i as f32 / 20.0) * beam.width();
                        let col = if i <= 5 || i >= 15 { theme.accent_blue } else { theme.text_secondary };
                        painter.line_segment(
                            [Pos2::new(sx, beam.top() + 6.0), Pos2::new(sx, beam.bottom() - 6.0)],
                            Stroke::new(1.2, col),
                        );
                    }
                    painter.text(
                        Pos2::new(beam.center().x, rect.bottom() - 12.0),
                        Align2::CENTER_CENTER,
                        "Lo = 1,400mm Confined Ends (D10-100)  •  Midspan Zone (D10-200)",
                        FontId::proportional(10.0),
                        theme.text_secondary,
                    );
                }
                _ => {
                    ui.label(RichText::new("BAR BENDING SCHEDULE (ISO 3766 SHAPE CODES)").size(11.0).strong().color(theme.accent_blue));
                    ui.add_space(6.0);
                    ScrollArea::horizontal().id_salt("bbs_preview_table").show(ui, |ui| {
                        egui::Grid::new("bbs_preview_grid")
                            .striped(true)
                            .spacing(Vec2::new(12.0, 5.0))
                            .show(ui, |ui| {
                                for h in ["Mark", "Shape", "Dia", "Qty", "Cut Len", "Total (kg)"] {
                                    ui.label(RichText::new(h).size(10.5).strong().color(theme.accent_blue));
                                }
                                ui.end_row();
                                for (mk, sh, dia, qty, cut, wt) in [
                                    ("B101-T1", "SC-11 (L-Hook)", "D22", "12", "8,920 mm", "319.4"),
                                    ("B101-B1", "SC-00 (Straight)", "D22", "18", "8,450 mm", "453.7"),
                                    ("B101-EX", "SC-21 (Top Ext)", "D19", "16", "3,150 mm", "112.1"),
                                    ("B101-ST", "SC-51 (135° Stir)", "D10", "144", "2,120 mm", "188.4"),
                                ] {
                                    ui.label(RichText::new(mk).size(10.5).monospace().color(theme.text_primary));
                                    ui.label(RichText::new(sh).size(10.5).color(theme.text_secondary));
                                    ui.label(RichText::new(dia).size(10.5).strong().color(theme.accent_orange));
                                    ui.label(RichText::new(qty).size(10.5).color(theme.text_primary));
                                    ui.label(RichText::new(cut).size(10.5).monospace().color(theme.text_muted));
                                    ui.label(RichText::new(wt).size(10.5).strong().color(theme.accent_green));
                                    ui.end_row();
                                }
                            });
                    });
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
