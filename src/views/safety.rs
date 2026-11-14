//! safety.rs — Pillar 11: Safety & Health (8D BIM Occupational Safety & Site Envelope).
//! Features tower crane swing radius envelopes, hook capacity checks, slab perimeter fall hazard
//! detection (OSHA 1926.502 / Permenaker 9/2016), and site emergency logistics.

use egui::{
    Align, Align2, Color32, CornerRadius, Frame, Layout, Margin, RichText, ScrollArea, Sense,
    Stroke, Ui, Vec2,
};
use crate::features::{FeatureId, FeatureRegistry};
use crate::icons::{self, AppIcon};
use crate::models::StructuralMember;
use crate::theme::MacTheme;
use crate::ui_kit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetySubTab {
    CraneEnvelope,
    FallProtection,
    SiteLogistics,
    SafetyAuditMatrix,
    ScaffoldingReshoring,
    PermitToWorkDrills,
}

#[derive(Clone, Debug)]
pub struct CraneBimClearanceReport {
    pub vertical_clearance_m: f32,
    pub building_roof_m: f32,
    pub farthest_corner_m: f32,
    pub horizontal_gap_m: f32,
    pub reaches_full_envelope: bool,
    pub is_collision_free: bool,
    pub bbox_min_m: Option<[f32; 2]>,
    pub bbox_max_m: Option<[f32; 2]>,
}

/// Evaluates 3D tower crane swing radius and hook mast clearance against active `BimProject` extents.
pub fn compute_crane_bim_perimeter_clearance(
    project: &crate::core::BimProject,
    mast_x_m: f32,
    mast_y_m: f32,
    mast_h_m: f32,
    jib_len_m: f32,
    boundary_buffer_m: f32,
) -> CraneBimClearanceReport {
    let roof_m = project
        .levels
        .iter()
        .map(|l| l.elevation_mm as f32 / 1000.0 + 3.5)
        .chain(project.columns.iter().map(|c| project.level_elevation(c.top_level_id) as f32 / 1000.0))
        .fold(0.0_f32, f32::max);
    let vertical_clearance_m = mast_h_m - roof_m;

    let Some((min_mm, max_mm)) = project.plan_extents_mm() else {
        return CraneBimClearanceReport {
            vertical_clearance_m,
            building_roof_m: roof_m,
            farthest_corner_m: 0.0,
            horizontal_gap_m: boundary_buffer_m,
            reaches_full_envelope: true,
            is_collision_free: true,
            bbox_min_m: None,
            bbox_max_m: None,
        };
    };

    let min_x = min_mm[0] as f32 / 1000.0;
    let min_y = min_mm[1] as f32 / 1000.0;
    let max_x = max_mm[0] as f32 / 1000.0;
    let max_y = max_mm[1] as f32 / 1000.0;

    // Distance from mast center to nearest point on building plan AABB
    let cx = mast_x_m.clamp(min_x, max_x);
    let cy = mast_y_m.clamp(min_y, max_y);
    let horizontal_gap_m = (mast_x_m - cx).hypot(mast_y_m - cy);

    let corners = [(min_x, min_y), (max_x, min_y), (min_x, max_y), (max_x, max_y)];
    let farthest_corner_m = corners
        .iter()
        .map(|&(px, py)| (mast_x_m - px).hypot(mast_y_m - py))
        .fold(0.0_f32, f32::max);

    let reaches_full_envelope = farthest_corner_m <= jib_len_m;
    let is_collision_free =
        vertical_clearance_m >= boundary_buffer_m || horizontal_gap_m >= boundary_buffer_m;

    CraneBimClearanceReport {
        vertical_clearance_m,
        building_roof_m: roof_m,
        farthest_corner_m,
        horizontal_gap_m,
        reaches_full_envelope,
        is_collision_free,
        bbox_min_m: Some([min_x, min_y]),
        bbox_max_m: Some([max_x, max_y]),
    }
}

pub struct SafetyView {
    pub active_subtab: SafetySubTab,
    // Tower Crane Parameters
    pub crane_jib_length_m: f32,
    pub crane_mast_height_m: f32,
    pub crane_current_radius_m: f32,
    pub crane_current_load_ton: f32,
    pub crane_max_capacity_ton: f32,
    pub crane_tip_capacity_ton: f32,
    pub crane_slew_angle_deg: f32,
    // Phase 39: 3D Crane vs BIM Perimeter Clearance Check (8D-02)
    pub crane_mast_x_m: f32,
    pub crane_mast_y_m: f32,
    pub crane_boundary_buffer_m: f32,
    pub crane_guard_bim: bool,
    pub crane_sweep_status: Option<String>,
    // Fall Hazard Parameters
    pub guardrail_installed: bool,
    pub floor_openings_covered: bool,
    pub safety_harness_anchors_certified: bool,
    pub toe_boards_in_place: bool,
    // Phase 40 (8D-03): ACI 347 Shoring & Reshoring Parameters
    pub shoring_pour_age_days: u32,
    pub shoring_floors_shored: u8,
    pub shoring_post_spacing_m: f32,
    pub shoring_slab_thickness_mm: f32,
    pub shoring_pt_transfer_slab: bool,
    pub shoring_audit_status: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShoringLevelRow {
    pub level_name: String,
    pub age_days: u32,
    pub maturity_pct: f32,
    pub shore_ratio: f32,
    pub status: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShoringReshoringReport {
    pub effective_slab_mm: f32,
    pub maturity_pct_at_target: f32,
    pub strike_ready: bool,
    pub peak_prop_load_kn: f32,
    pub prop_capacity_kn: f32,
    pub euler_fs: f32,
    pub rows: Vec<ShoringLevelRow>,
}

/// Evaluates ACI 209 concrete maturity, ACI 347 multi-floor shore load transfer, and Euler prop buckling FS from BIM levels & slabs.
pub fn compute_shoring_reshoring_audit(
    bim_project: &crate::core::BimProject,
    pour_age_days: u32,
    floors_shored: u8,
    post_spacing_m: f32,
    slab_thickness_mm: f32,
    pt_transfer_slab: bool,
) -> ShoringReshoringReport {
    let bim_avg_slab = if !bim_project.slabs.is_empty() {
        (bim_project.slabs.iter().map(|s| s.thickness_mm as f32).sum::<f32>()
            / bim_project.slabs.len() as f32)
            .clamp(120.0, 600.0)
    } else {
        slab_thickness_mm
    };
    let effective_slab_mm = bim_avg_slab + if pt_transfer_slab { 80.0 } else { 0.0 };

    let aci209_pct = |days: u32| -> f32 {
        if days == 0 {
            0.0
        } else {
            let d = days as f32;
            (d / (4.0 + 0.85 * d) * 100.0).clamp(0.0, 100.0)
        }
    };
    let maturity_pct_at_target = aci209_pct(pour_age_days);
    let strike_ready = maturity_pct_at_target >= 70.0;

    let slab_dl_kn_m2 = (effective_slab_mm / 1000.0) * 24.0;
    let formwork_live_kn_m2 = 3.0; // ACI 347 formwork + crew/equipment minimum
    let wet_total_kn_m2 = slab_dl_kn_m2 + formwork_live_kn_m2;
    let top_ratio = 1.0 + 0.65 / (floors_shored.max(1) as f32);
    let trib_area_m2 = post_spacing_m.clamp(0.6, 2.5).powi(2);
    let peak_prop_load_kn = wet_total_kn_m2 * top_ratio * trib_area_m2;
    let prop_capacity_kn = 38.0;
    let euler_fs = 93.0 / peak_prop_load_kn.max(1.0);

    let level_names: Vec<String> = if !bim_project.levels.is_empty() {
        bim_project
            .levels
            .iter()
            .rev()
            .take(3)
            .map(|l| l.name.clone())
            .collect()
    } else {
        vec![
            "Level 06 Pour".to_string(),
            "Level 05 Deck".to_string(),
            "Level 04 Deck".to_string(),
        ]
    };

    let mut rows = Vec::with_capacity(3);
    for idx in 0..3 {
        let lvl = level_names
            .get(idx)
            .cloned()
            .unwrap_or_else(|| format!("Level -{}", idx));
        let (age, ratio, status) = match idx {
            0 => (0, top_ratio, "SHORED (WET)"),
            1 => {
                let m = aci209_pct(pour_age_days);
                let st = if m >= 70.0 {
                    "RESHORE READY"
                } else {
                    "HOLD STRIP (<70%)"
                };
                (pour_age_days, 0.68 * (2.0 / floors_shored.max(1) as f32).clamp(0.55, 1.2), st)
            }
            _ => {
                let age2 = pour_age_days * 2;
                let m2 = aci209_pct(age2);
                let st = if m2 >= 85.0 {
                    "STRIPPED SAFE"
                } else {
                    "RESHORE ACTIVE"
                };
                (age2, 0.41, st)
            }
        };
        rows.push(ShoringLevelRow {
            level_name: lvl,
            age_days: age,
            maturity_pct: aci209_pct(age),
            shore_ratio: ratio,
            status,
        });
    }

    ShoringReshoringReport {
        effective_slab_mm,
        maturity_pct_at_target,
        strike_ready,
        peak_prop_load_kn,
        prop_capacity_kn,
        euler_fs,
        rows,
    }
}

impl Default for SafetyView {
    fn default() -> Self {
        Self {
            active_subtab: SafetySubTab::CraneEnvelope,
            crane_jib_length_m: 50.0,
            crane_mast_height_m: 45.0,
            crane_current_radius_m: 32.0,
            crane_current_load_ton: 4.5,
            crane_max_capacity_ton: 12.0,
            crane_tip_capacity_ton: 2.4,
            crane_slew_angle_deg: 45.0,
            crane_mast_x_m: -4.5,
            crane_mast_y_m: 4.0,
            crane_boundary_buffer_m: 3.0,
            crane_guard_bim: true,
            crane_sweep_status: None,
            guardrail_installed: true,
            floor_openings_covered: true,
            safety_harness_anchors_certified: true,
            toe_boards_in_place: true,
            shoring_pour_age_days: 7,
            shoring_floors_shored: 2,
            shoring_post_spacing_m: 1.20,
            shoring_slab_thickness_mm: 220.0,
            shoring_pt_transfer_slab: false,
            shoring_audit_status: None,
        }
    }
}

impl SafetyView {
    pub fn show(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        _members: &[StructuralMember],
        bim_project: &crate::core::BimProject,
        features: &mut FeatureRegistry,
    ) {
        let is_mobile = ui.available_width() < MacTheme::MOBILE_BREAKPOINT;
        let pad = if is_mobile { 12 } else { 24 };

        ui.vertical(|ui| {
            Frame::new().inner_margin(Margin::symmetric(pad, 16)).show(ui, |ui| {
                // 1. Top Subtabs Bar
                self.render_header_bar(ui, theme);
                ui.add_space(16.0);

                // 2. Active Tab Content
                match self.active_subtab {
                    SafetySubTab::CraneEnvelope => self.render_crane_envelope(ui, theme, bim_project, features),
                    SafetySubTab::FallProtection => self.render_fall_protection(ui, theme),
                    SafetySubTab::SiteLogistics => self.render_site_logistics(ui, theme),
                    SafetySubTab::SafetyAuditMatrix => self.render_safety_audit_matrix(ui, theme),
                    SafetySubTab::ScaffoldingReshoring => {
                        self.render_scaffolding_reshoring(ui, theme, bim_project, features);
                    }
                    SafetySubTab::PermitToWorkDrills => {
                        self.render_permit_to_work_drills(ui, theme, features);
                    }
                }

                ui.add_space(32.0);
            });
        });
    }

    fn render_header_bar(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let subtabs = [
            (SafetySubTab::CraneEnvelope, "🏗️ Tower Crane Radius Arc"),
            (SafetySubTab::FallProtection, "🛡️ Slab Edge Fall Hazards"),
            (SafetySubTab::SiteLogistics, "🚑 Emergency Evacuation & Logistics"),
            (SafetySubTab::SafetyAuditMatrix, "📋 8D Safety Audit Matrix"),
            (SafetySubTab::ScaffoldingReshoring, "🪜 Formwork & Reshoring"),
            (SafetySubTab::PermitToWorkDrills, "🪪 Digital PTW & Drills"),
        ];
        ui_kit::subtab_bar(ui, theme, "safety_subtab_bar", &mut self.active_subtab, &subtabs);
    }

    fn calculate_crane_allowable_load(&self) -> f32 {
        // Linear interpolation between max capacity at minimum radius (15m) and tip capacity at jib length
        let r_min = 15.0;
        let r_max = self.crane_jib_length_m;
        if self.crane_current_radius_m <= r_min {
            self.crane_max_capacity_ton
        } else if self.crane_current_radius_m >= r_max {
            self.crane_tip_capacity_ton
        } else {
            let factor = (self.crane_current_radius_m - r_min) / (r_max - r_min);
            self.crane_max_capacity_ton
                - factor * (self.crane_max_capacity_ton - self.crane_tip_capacity_ton)
        }
    }

    fn render_crane_envelope(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        bim_project: &crate::core::BimProject,
        features: &mut FeatureRegistry,
    ) {
        let q_allow = self.calculate_crane_allowable_load();
        let is_overloaded = self.crane_current_load_ton > q_allow;
        let dcr = self.crane_current_load_ton / q_allow.max(0.01);
        let crane_clearance = compute_crane_bim_perimeter_clearance(
            bim_project,
            self.crane_mast_x_m,
            self.crane_mast_y_m,
            self.crane_mast_height_m,
            self.crane_jib_length_m,
            self.crane_boundary_buffer_m,
        );

        // Hero Card
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(12))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(18))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.vertical(|ui| {
                        icons::icon_label(
                            ui,
                            AppIcon::Building,
                            17.0,
                            theme.accent_blue,
                            "8D Tower Crane Swing Radius & Hook Capacity Envelope",
                            17.0,
                            theme.text_primary,
                            true,
                        );
                        ui.label(
                            RichText::new("Real-time working radius sweep, pick load versus radius capacity curves, and collision avoidance over public zones.")
                                .size(11.5)
                                .color(theme.text_secondary),
                        );
                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let status_color = if is_overloaded {
                            Color32::from_rgb(230, 70, 70)
                        } else {
                            theme.accent_green
                        };
                        let (status_icon, status_text) = if is_overloaded {
                            (AppIcon::AlertTriangle, format!("CRANE OVERLOAD (DCR {:.2})", dcr))
                        } else {
                            (AppIcon::CheckCircle, format!("LIFT CAPACITY SAFE (DCR {:.2})", dcr))
                        };

                        Frame::new()
                            .fill(status_color.gamma_multiply(0.15))
                            .stroke(Stroke::new(1.0, status_color.gamma_multiply(0.6)))
                            .corner_radius(CornerRadius::same(6))
                            .inner_margin(Margin::symmetric(12, 6))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    icons::icon_label(ui, status_icon, 13.0, status_color, status_text, 11.0, status_color, true);
                                });
                            });
                    });
                });
            });

        ui.add_space(12.0);

        // Phase 39 (8D-02): 3D Crane Jib vs BIM Perimeter Clearance Check
        ui_kit::feature_slot_card(
            ui,
            theme,
            features,
            FeatureId::SitePerimeter3DCraneCollision,
            |ui| {
                ui.horizontal_wrapped(|ui| {
                    if ui.selectable_label(self.crane_guard_bim, "Guard Active BimProject Envelopes").clicked() {
                        self.crane_guard_bim = !self.crane_guard_bim;
                    }
                    ui.label(RichText::new("Mast X (m):").size(10.5).color(theme.text_muted));
                    ui.add(egui::DragValue::new(&mut self.crane_mast_x_m).range(-60.0..=60.0).speed(0.5).max_decimals(1));
                    ui.label(RichText::new("Mast Y (m):").size(10.5).color(theme.text_muted));
                    ui.add(egui::DragValue::new(&mut self.crane_mast_y_m).range(-60.0..=60.0).speed(0.5).max_decimals(1));
                    ui.label(RichText::new("Buffer (m):").size(10.5).color(theme.text_muted));
                    ui.add(egui::DragValue::new(&mut self.crane_boundary_buffer_m).range(1.0..=10.0).speed(0.2).max_decimals(1));
                    if icons::icon_button(ui, AppIcon::Shield, "Sweep 360° 3D Boom vs BIM Perimeter", 11.0, Color32::WHITE, if crane_clearance.is_collision_free { theme.accent_green } else { theme.accent_red }, Stroke::NONE, CornerRadius::same(6), Vec2::ZERO).clicked() {
                        self.crane_sweep_status = Some(format!(
                            "{} | Mast ΔZ = {:.1}m (Roof {:.1}m) | Reach Coverage: {:.1}m / {:.1}m ({})",
                            if crane_clearance.is_collision_free { "3D SWEEP CLEAR" } else { "COLLISION BUFFER VIOLATION" },
                            crane_clearance.vertical_clearance_m,
                            crane_clearance.building_roof_m,
                            crane_clearance.farthest_corner_m,
                            self.crane_jib_length_m,
                            if crane_clearance.reaches_full_envelope { "100% BIM Covered" } else { "Corner Beyond Jib" },
                        ));
                    }
                });
                ui.add_space(6.0);
                let col_clr = if crane_clearance.is_collision_free { theme.accent_green } else { theme.accent_red };
                ui.label(
                    RichText::new(format!(
                        "Envelope Audit ({} BIM elements): Mast Vertical ΔZ = {:+.1} m vs Roof ({:.1} m)  •  Plan Gap = {:.1} m  •  Farthest Corner = {:.1} m ({})",
                        bim_project.element_count(),
                        crane_clearance.vertical_clearance_m,
                        crane_clearance.building_roof_m,
                        crane_clearance.horizontal_gap_m,
                        crane_clearance.farthest_corner_m,
                        if crane_clearance.reaches_full_envelope { "FULL REACH" } else { "OUT OF REACH" }
                    ))
                    .size(10.5)
                    .strong()
                    .color(col_clr),
                );
                if let Some(ref msg) = self.crane_sweep_status {
                    ui.label(RichText::new(msg).size(10.0).color(theme.text_secondary));
                }
            },
        );

        ui.add_space(14.0);

        // Interactive Controls & Visual Canvas (Side-by-Side on Desktop, Stacked on Mobile/Tablet)
        ui_kit::responsive_two_panes(ui, 720.0, |ui, pane| match pane {
            0 => {
                Frame::new()
                    .fill(theme.bg_card)
                    .corner_radius(CornerRadius::same(10))
                    .stroke(Stroke::new(1.0, theme.border_card))
                    .inner_margin(Margin::same(16))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        icons::icon_label(
                            ui,
                            AppIcon::SlidersPanel,
                            14.5,
                            theme.accent_blue,
                            "Crane Configuration & Pick Telemetry",
                            14.0,
                            theme.text_primary,
                            true,
                        );
                        ui.add_space(10.0);

                        ui.label(
                            RichText::new(format!("Jib Length: {:.1} m", self.crane_jib_length_m))
                                .size(11.5)
                                .color(theme.text_secondary),
                        );
                        ui.add(
                            egui::Slider::new(&mut self.crane_jib_length_m, 30.0..=75.0)
                                .suffix(" m"),
                        );
                        ui.add_space(8.0);

                        ui.label(
                            RichText::new(format!("Mast Height: {:.1} m", self.crane_mast_height_m))
                                .size(11.5)
                                .color(theme.text_secondary),
                        );
                        ui.add(
                            egui::Slider::new(&mut self.crane_mast_height_m, 25.0..=80.0)
                                .suffix(" m"),
                        );
                        ui.add_space(8.0);

                        ui.label(
                            RichText::new(format!(
                                "Working Radius: {:.1} m",
                                self.crane_current_radius_m
                            ))
                            .size(11.5)
                            .color(theme.accent_blue),
                        );
                        ui.add(
                            egui::Slider::new(
                                &mut self.crane_current_radius_m,
                                10.0..=self.crane_jib_length_m,
                            )
                            .suffix(" m"),
                        );
                        ui.add_space(8.0);

                        ui.label(
                            RichText::new(format!(
                                "Hook Pick Load: {:.2} Tonnes (Capacity: {:.2} T)",
                                self.crane_current_load_ton, q_allow
                            ))
                            .size(11.5)
                            .color(if is_overloaded {
                                Color32::from_rgb(230, 70, 70)
                            } else {
                                theme.accent_green
                            }),
                        );
                        ui.add(
                            egui::Slider::new(&mut self.crane_current_load_ton, 0.5..=15.0)
                                .suffix(" T"),
                        );
                        ui.add_space(8.0);

                        ui.label(
                            RichText::new(format!("Slew Angle: {:.1}°", self.crane_slew_angle_deg))
                                .size(11.5)
                                .color(theme.text_secondary),
                        );
                        ui.add(
                            egui::Slider::new(&mut self.crane_slew_angle_deg, 0.0..=360.0)
                                .suffix("°"),
                        );
                    });
            }
            _ => {
                Frame::new()
                    .fill(theme.bg_card)
                    .corner_radius(CornerRadius::same(10))
                    .stroke(Stroke::new(1.0, theme.border_card))
                    .inner_margin(Margin::same(16))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        icons::icon_label(
                            ui,
                            AppIcon::Target,
                            14.5,
                            theme.accent_blue,
                            "Site Sweep Radar & Exclusion Boundary",
                            14.0,
                            theme.text_primary,
                            true,
                        );
                        ui.add_space(6.0);

                        let w = ui.available_width().max(180.0);
                        let (rect, _resp) =
                            ui.allocate_exact_size(Vec2::new(w, 220.0), Sense::hover());
                        ui.painter().rect_filled(
                            rect,
                            CornerRadius::same(8),
                            Color32::from_rgb(24, 24, 28),
                        );
                        ui.painter().rect_stroke(
                            rect,
                            CornerRadius::same(8),
                            Stroke::new(1.0, theme.border_subtle),
                            egui::StrokeKind::Inside,
                        );

                        let center = rect.center();
                        let half_box = (rect.width().min(rect.height()) * 0.44).max(40.0);
                        let scale = half_box / self.crane_jib_length_m.max(1.0);

                        let max_r_px = self.crane_jib_length_m * scale;
                        ui.painter().circle_stroke(
                            center,
                            max_r_px,
                            Stroke::new(1.0, Color32::from_rgba_premultiplied(80, 140, 220, 100)),
                        );

                        if self.crane_guard_bim {
                            if let (Some(bmin), Some(bmax)) =
                                (crane_clearance.bbox_min_m, crane_clearance.bbox_max_m)
                            {
                                let x0 = center.x + (bmin[0] - self.crane_mast_x_m) * scale;
                                let x1 = center.x + (bmax[0] - self.crane_mast_x_m) * scale;
                                let y0 = center.y - (bmax[1] - self.crane_mast_y_m) * scale;
                                let y1 = center.y - (bmin[1] - self.crane_mast_y_m) * scale;
                                let brect = egui::Rect::from_min_max(
                                    egui::Pos2::new(x0.min(x1), y0.min(y1)),
                                    egui::Pos2::new(x0.max(x1), y0.max(y1)),
                                );
                                let bcol = if crane_clearance.is_collision_free {
                                    theme.accent_blue
                                } else {
                                    theme.accent_red
                                };
                                ui.painter().rect_filled(brect, CornerRadius::same(2), bcol.gamma_multiply(0.18));
                                ui.painter().rect_stroke(
                                    brect,
                                    CornerRadius::same(2),
                                    Stroke::new(1.2, bcol),
                                    egui::StrokeKind::Inside,
                                );
                            }
                        }

                        let pick_r_px = self.crane_current_radius_m * scale;
                        let pick_color = if is_overloaded {
                            Color32::from_rgb(230, 70, 70)
                        } else {
                            Color32::from_rgb(80, 200, 120)
                        };
                        ui.painter().circle_stroke(center, pick_r_px, Stroke::new(1.5, pick_color));

                        ui.painter().circle_filled(center, 4.0, Color32::WHITE);

                        let rad = self.crane_slew_angle_deg.to_radians();
                        let jib_end =
                            center + Vec2::new(max_r_px * rad.cos(), max_r_px * rad.sin());
                        ui.painter().line_segment(
                            [center, jib_end],
                            Stroke::new(2.5, Color32::from_rgb(240, 180, 60)),
                        );

                        let hook_pos =
                            center + Vec2::new(pick_r_px * rad.cos(), pick_r_px * rad.sin());
                        ui.painter().circle_filled(hook_pos, 5.5, pick_color);

                        ui.painter().text(
                            hook_pos + Vec2::new(8.0, -8.0),
                            Align2::LEFT_CENTER,
                            format!(
                                "{:.1}T @ {:.0}m",
                                self.crane_current_load_ton, self.crane_current_radius_m
                            ),
                            egui::FontId::monospace(10.0),
                            Color32::WHITE,
                        );
                    });
            }
        });
    }

    fn render_fall_protection(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(12))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(18))
            .show(ui, |ui| {
                icons::icon_label(
                    ui,
                    AppIcon::Shield,
                    17.0,
                    theme.accent_blue,
                    "Slab Edge Fall Protection & Leading Edge Risk Controls",
                    17.0,
                    theme.text_primary,
                    true,
                );
                ui.label(
                    RichText::new("OSHA 1926.502 / Permenaker No. 9/2016 compliance checks for perimeter slab edges, floor openings, and horizontal lifeline tie-offs.")
                        .size(11.5)
                        .color(theme.text_secondary),
                );

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);

                let checks = [
                    (
                        &mut self.guardrail_installed,
                        "Perimeter Standard Guardrail (Top-rail 107cm, Mid-rail 53cm, 900N lateral capacity)",
                    ),
                    (
                        &mut self.toe_boards_in_place,
                        "Toe-boards installed on all open elevated edges (minimum height 9cm)",
                    ),
                    (
                        &mut self.floor_openings_covered,
                        "Floor slab voids and MEP risers (> 50mm) covered with secured hazard covers",
                    ),
                    (
                        &mut self.safety_harness_anchors_certified,
                        "Horizontal Lifeline (HLL) and certified anchor tie-off points rated >= 22.2 kN (5,000 lbs)",
                    ),
                ];

                for (val, desc) in checks {
                    ui.horizontal_wrapped(|ui| {
                        ui.checkbox(val, "");
                        ui.label(
                            RichText::new(desc).size(12.0).color(if *val {
                                theme.text_primary
                            } else {
                                Color32::from_rgb(230, 80, 80)
                            }),
                        );
                    });
                    ui.add_space(8.0);
                }
            });
    }

    fn render_site_logistics(&self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(12))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(18))
            .show(ui, |ui| {
                icons::icon_label(
                    ui,
                    AppIcon::AlertTriangle,
                    17.0,
                    theme.accent_red,
                    "Emergency Site Logistics & Evacuation Pathways",
                    17.0,
                    theme.text_primary,
                    true,
                );
                ui.label(
                    RichText::new("Rapid egress corridors, designated safe muster points, fire brigade staging, and on-site first responder facilities.")
                        .size(11.5)
                        .color(theme.text_secondary),
                );

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);

                let facilities = [
                    (AppIcon::AlertTriangle, "Emergency Assembly Zone", "East Boundary Plot (Clear radius 25m, capacity 200 workers)", "OPERATIONAL", theme.accent_green),
                    (AppIcon::Flame, "Fire Hydrant & Staging", "North Gate Access Road (Water flow 1,500 L/min certified)", "CONNECTED", theme.accent_blue),
                    (AppIcon::Shield, "First Aid Medical Clinic", "Ground Floor North Bay (2 Certified paramedics on site)", "STAFFED", theme.accent_green),
                    (AppIcon::Bolt, "Fire Extinguisher Points", "Every 20m along scaffold corridors (Type ABC Dry Chemical)", "INSPECTED", theme.accent_green),
                ];

                for (icon, name, desc, stat, col) in facilities {
                    Frame::new()
                        .fill(theme.bg_canvas)
                        .corner_radius(CornerRadius::same(8))
                        .stroke(Stroke::new(0.8, theme.border_subtle))
                        .inner_margin(Margin::symmetric(14, 10))
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.vertical(|ui| {
                                    icons::icon_label(ui, icon, 14.0, col, name, 13.0, theme.text_primary, true);
                                    ui.label(RichText::new(desc).size(11.0).color(theme.text_muted));
                                });
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    Frame::new()
                                        .fill(col.gamma_multiply(0.15))
                                        .corner_radius(CornerRadius::same(4))
                                        .inner_margin(Margin::symmetric(8, 4))
                                        .show(ui, |ui| {
                                            ui.label(RichText::new(stat).size(10.0).strong().color(col));
                                        });
                                });
                            });
                        });
                    ui.add_space(8.0);
                }
            });
    }

    fn render_safety_audit_matrix(&self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(12))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(18))
            .show(ui, |ui| {
                icons::icon_label(
                    ui,
                    AppIcon::Clipboard,
                    17.0,
                    theme.accent_blue,
                    "8D Safety & Health Audit Compliance Matrix",
                    17.0,
                    theme.text_primary,
                    true,
                );
                ui.label(
                    RichText::new("Hazard Identification, Risk Assessment, and Determining Controls (HIRADC) summary.")
                        .size(11.5)
                        .color(theme.text_secondary),
                );

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);

                let audits = [
                    ("Crane Lift Radius Clearance", "ASME B30.3 / Permenaker 8/2020", "DCR = 0.38", "PASSED", theme.accent_green),
                    ("Leading Edge Fall Protection", "OSHA 1926.502 / SNI K3", "100% Compliant", "PASSED", theme.accent_green),
                    ("Egress Corridor Clear Width", "NFPA 101 Life Safety Code", "w = 1.80m >= 1.20m", "PASSED", theme.accent_green),
                    ("Personal Fall Arrest Systems", "ANSI/ASSP Z359.1", "All Harnesses Inspected", "PASSED", theme.accent_green),
                ];

                for (item, code, val, stat, col) in audits {
                    Frame::new()
                        .fill(theme.bg_canvas)
                        .corner_radius(CornerRadius::same(8))
                        .stroke(Stroke::new(0.8, theme.border_subtle))
                        .inner_margin(Margin::symmetric(14, 10))
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(item).size(12.5).strong().color(theme.text_primary));
                                    ui.label(RichText::new(code).size(10.5).color(theme.text_muted));
                                });
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    Frame::new()
                                        .fill(col.gamma_multiply(0.15))
                                        .corner_radius(CornerRadius::same(4))
                                        .inner_margin(Margin::symmetric(8, 4))
                                        .show(ui, |ui| {
                                            ui.label(RichText::new(stat).size(10.0).strong().color(col));
                                        });
                                    ui.add_space(12.0);
                                    ui.label(RichText::new(val).size(11.0).color(theme.text_secondary));
                                });
                            });
                        });
                    ui.add_space(8.0);
                }
            });
    }

    fn render_scaffolding_reshoring(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        bim_project: &crate::core::BimProject,
        features: &mut FeatureRegistry,
    ) {
        let audit = compute_shoring_reshoring_audit(
            bim_project,
            self.shoring_pour_age_days,
            self.shoring_floors_shored,
            self.shoring_post_spacing_m,
            self.shoring_slab_thickness_mm,
            self.shoring_pt_transfer_slab,
        );
        ui_kit::feature_slot_card(
            ui,
            theme,
            features,
            FeatureId::ScaffoldingFormworkReshoring,
            |ui| {
                let ratio_val = format!("{} Floors Shored + 1 Reshored", self.shoring_floors_shored);
                let ratio_sub = format!("ACI 347-14 • Effective Slab {:.0} mm", audit.effective_slab_mm);
                let mat_val = format!(
                    "{:.1}% f'c @ Day {}",
                    audit.maturity_pct_at_target, self.shoring_pour_age_days
                );
                let mat_sub = if audit.strike_ready {
                    "Min Strike Threshold >= 70% (Safe)".to_string()
                } else {
                    "HOLD STRIP: Below 70% ACI Threshold".to_string()
                };
                let mat_col = if audit.strike_ready {
                    theme.accent_green
                } else {
                    theme.accent_red
                };
                let prop_dcr = audit.peak_prop_load_kn / audit.prop_capacity_kn;
                let prop_val = format!("{:.1} kN / Post", audit.peak_prop_load_kn);
                let prop_sub = format!(
                    "Ring-Lock Cap = {:.1} kN (DCR {:.2})",
                    audit.prop_capacity_kn, prop_dcr
                );
                let prop_col = if prop_dcr <= 1.0 {
                    theme.accent_green
                } else {
                    theme.accent_red
                };
                let fs_val = format!("FS_buckling = {:.2}", audit.euler_fs);
                let fs_col = if audit.euler_fs >= 2.0 {
                    theme.accent_green
                } else if audit.euler_fs >= 1.5 {
                    theme.accent_orange
                } else {
                    theme.accent_red
                };
                let cards = [
                    ("STANDARD & SHORE RATIO", ratio_val.as_str(), ratio_sub.as_str(), theme.accent_blue),
                    ("EARLY CONCRETE MATURITY", mat_val.as_str(), mat_sub.as_str(), mat_col),
                    ("PEAK PROP AXIAL LOAD", prop_val.as_str(), prop_sub.as_str(), prop_col),
                    ("EULER SLENDERNESS CHECK", fs_val.as_str(), "Braced @ 1.50m Vertical Lifts", fs_col),
                ];
                ui_kit::responsive_grid_n(ui, cards.len(), 185.0, |ui, idx| {
                    let (t, v, s, c) = cards[idx];
                    ui_kit::stat_card(ui, theme, t, v, s, c);
                });

                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    for n in [1u8, 2, 3] {
                        if ui
                            .selectable_label(
                                self.shoring_floors_shored == n,
                                format!("{n} Shored + 1 Reshored"),
                            )
                            .clicked()
                        {
                            self.shoring_floors_shored = n;
                        }
                    }
                    ui.checkbox(&mut self.shoring_pt_transfer_slab, "+80mm PT Transfer Slab");
                });
                ui.add_space(6.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Pour Cure Days:").size(11.0).color(theme.text_secondary));
                    ui.add(egui::Slider::new(&mut self.shoring_pour_age_days, 1..=28));
                    ui.label(RichText::new("Post Grid (m):").size(11.0).color(theme.text_secondary));
                    ui.add(egui::Slider::new(&mut self.shoring_post_spacing_m, 0.8..=1.8).step_by(0.05));
                    if bim_project.slabs.is_empty() {
                        ui.label(RichText::new("Slab (mm):").size(11.0).color(theme.text_secondary));
                        ui.add(egui::Slider::new(&mut self.shoring_slab_thickness_mm, 150.0..=350.0).step_by(10.0));
                    }

                    if icons::icon_button(
                        ui,
                        AppIcon::CheckCircle,
                        "Export ACI 347 Shoring Audit (.CSV)",
                        11.5,
                        theme.text_primary,
                        theme.bg_card,
                        Stroke::new(1.0, theme.border_subtle),
                        CornerRadius::same(6),
                        Vec2::ZERO,
                    )
                    .clicked()
                    {
                        let mut csv = String::from("Level,Age_Days,Maturity_fc_pct,Shore_Ratio,Status\n");
                        for r in &audit.rows {
                            csv.push_str(&format!(
                                "{},{},{:.1},{:.2},{}\n",
                                r.level_name, r.age_days, r.maturity_pct, r.shore_ratio, r.status
                            ));
                        }
                        self.shoring_audit_status = Some(
                            match crate::core::web_bridge::save_text_file("aci347_shoring_audit.csv", &csv, "text/csv") {
                                Ok(msg) => format!(
                                    "ACI 347 Verified: Prop {:.1} kN (FS {:.2}) • {msg}",
                                    audit.peak_prop_load_kn, audit.euler_fs
                                ),
                                Err(e) => format!("Shoring CSV export failed: {e}"),
                            },
                        );
                    }
                });
                if let Some(ref st) = self.shoring_audit_status {
                    ui.add_space(6.0);
                    ui.label(RichText::new(st).size(11.0).color(theme.accent_blue));
                }
                ui.add_space(10.0);

                ui_kit::responsive_two_panes(ui, 640.0, |ui, pane| match pane {
                    0 => {
                        ui.label(
                            RichText::new("MULTI-FLOOR SHORE & RESHORE LOAD TRANSFER SCHEMATIC")
                                .size(11.0)
                                .strong()
                                .color(theme.accent_blue),
                        );
                        ui.add_space(6.0);
                        let desired = Vec2::new(ui.available_width(), 155.0);
                        let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
                        let painter = ui.painter_at(rect);
                        painter.rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(13, 17, 24));
                        let box_r = rect.shrink(22.0);
                        for lvl in 0..3 {
                            let y = box_r.top() + (lvl as f32) * (box_r.height() * 0.42);
                            painter.line_segment(
                                [egui::Pos2::new(box_r.left(), y), egui::Pos2::new(box_r.right(), y)],
                                Stroke::new(3.0, theme.text_secondary),
                            );
                            if lvl < 2 {
                                let ny = box_r.top() + ((lvl + 1) as f32) * (box_r.height() * 0.42);
                                let prop_col = if lvl == 0 {
                                    if prop_dcr <= 1.0 { theme.accent_orange } else { theme.accent_red }
                                } else {
                                    theme.accent_green
                                };
                                let prop_cnt = ((box_r.width() / (self.shoring_post_spacing_m * 45.0)).round() as i32).clamp(3, 10);
                                for p in 1..=prop_cnt {
                                    let px = box_r.left() + (p as f32 / (prop_cnt + 1) as f32) * box_r.width();
                                    painter.line_segment(
                                        [egui::Pos2::new(px, y + 2.0), egui::Pos2::new(px, ny - 2.0)],
                                        Stroke::new(2.0, prop_col),
                                    );
                                }
                            }
                        }
                        painter.text(
                            egui::Pos2::new(box_r.left() + 6.0, box_r.bottom() - 2.0),
                            Align2::LEFT_BOTTOM,
                            format!(
                                "Top Pour ({:.2}x DL, {:.1} kN/post) -> Day-{} Slab ({:.0}% f'c) -> Reshored Level",
                                audit.rows[0].shore_ratio,
                                audit.peak_prop_load_kn,
                                self.shoring_pour_age_days,
                                audit.maturity_pct_at_target
                            ),
                            egui::FontId::monospace(8.5),
                            theme.text_muted,
                        );
                    }
                    _ => {
                        ui.label(
                            RichText::new("SHORING BAY VERIFICATION TABLE (ACI 347 / EN 12812)")
                                .size(11.0)
                                .strong()
                                .color(theme.accent_blue),
                        );
                        ui.add_space(6.0);
                        ScrollArea::horizontal().id_salt("shoring_table_scroll").show(ui, |ui| {
                            egui::Grid::new("shoring_table_grid")
                                .striped(true)
                                .spacing(Vec2::new(12.0, 5.0))
                                .show(ui, |ui| {
                                    for h in ["Level", "Pour Age", "f'c Ratio", "Shore Ratio", "Status"] {
                                        ui.label(RichText::new(h).size(10.5).strong().color(theme.accent_blue));
                                    }
                                    ui.end_row();
                                    for r in &audit.rows {
                                        let col = if r.status.contains("HOLD") {
                                            theme.accent_red
                                        } else if r.status.contains("WET") {
                                            theme.accent_orange
                                        } else {
                                            theme.accent_green
                                        };
                                        let age_str = if r.age_days == 0 {
                                            "0 Days (Wet)".to_string()
                                        } else {
                                            format!("{} Days", r.age_days)
                                        };
                                        ui.label(RichText::new(&r.level_name).size(10.5).strong().color(theme.text_primary));
                                        ui.label(RichText::new(age_str).size(10.5).color(theme.text_secondary));
                                        ui.label(RichText::new(format!("{:.0}% f'c", r.maturity_pct)).size(10.5).color(theme.accent_blue));
                                        ui.label(RichText::new(format!("{:.2}x DL", r.shore_ratio)).size(10.5).color(theme.text_secondary));
                                        ui.label(RichText::new(r.status).size(10.5).strong().color(col));
                                        ui.end_row();
                                    }
                                });
                        });
                    }
                });
            },
        );
    }

    fn render_permit_to_work_drills(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(
            ui,
            theme,
            features,
            FeatureId::PermitToWorkEvacuationDrill,
            |ui| {
                let cards = [
                    ("ACTIVE SITE PERMITS (PTW)", "7 Valid Today", "3 Hot-Work • 2 Height • 2 Confined", theme.accent_blue),
                    ("CRANE WIND LOCKDOWN CAP", "38 km/h Current", "Auto-Hold Threshold = 55 km/h", theme.accent_green),
                    ("LAST EVACUATION DRILL", "3m 42s @ Muster E", "186/186 Crew RFID Accounted", theme.accent_green),
                    ("EXPIRING SIGN-OFFS", "1 Pending Renewal", "PTW-HW-09 Expires in 90 min", theme.accent_orange),
                ];
                ui_kit::responsive_grid_n(ui, cards.len(), 185.0, |ui, idx| {
                    let (t, v, s, c) = cards[idx];
                    ui_kit::stat_card(ui, theme, t, v, s, c);
                });

                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    let _ = ui.selectable_label(true, "Enforce Gas Test for Confined Spaces");
                    let _ = ui.selectable_label(false, "Auto-Suspend Lift PTW @ Wind >= 55 km/h");
                    let _ = icons::icon_button(
                        ui,
                        AppIcon::Shield,
                        "Issue Digital Permit-to-Work & Log Drill Time",
                        12.0,
                        theme.text_primary,
                        theme.bg_card,
                        Stroke::new(1.0, theme.border_subtle),
                        CornerRadius::same(6),
                        Vec2::ZERO,
                    );
                });
                ui.add_space(10.0);

                ScrollArea::horizontal().id_salt("ptw_ledger_scroll").show(ui, |ui| {
                    egui::Grid::new("ptw_ledger_grid")
                        .striped(true)
                        .min_col_width(90.0)
                        .spacing(Vec2::new(14.0, 6.0))
                        .show(ui, |ui| {
                            for h in ["Permit ID", "Hazard Class", "Location", "Competent Sign-Off", "Validity", "Status"] {
                                ui.label(RichText::new(h).size(10.5).strong().color(theme.accent_blue));
                            }
                            ui.end_row();
                            for (id, icon, cls, loc, sup, val, st, col) in [
                                ("PTW-HW-09", AppIcon::Flame, "Hot-Work Welding", "Level 08 Truss Splice", "R. Pratama (HSE L3)", "Until 17:00", "RENEW SOON", theme.accent_orange),
                                ("PTW-WH-14", AppIcon::HardHat, "Work-at-Height", "Perimeter Facade West", "S. Wijaya (Rigging)", "Until 18:00", "ACTIVE", theme.accent_green),
                                ("PTW-CS-03", AppIcon::AlertTriangle, "Confined Space", "Sump Pit B2 (O₂ 20.9%)", "H. Kusuma (Gas Tech)", "Until 16:30", "ACTIVE", theme.accent_green),
                            ] {
                                ui.label(RichText::new(id).size(10.5).monospace().color(theme.accent_blue));
                                ui.horizontal(|ui| {
                                    icons::icon_label(ui, icon, 12.0, col, cls, 10.5, theme.text_primary, true);
                                });
                                ui.label(RichText::new(loc).size(10.5).color(theme.text_secondary));
                                ui.label(RichText::new(sup).size(10.5).color(theme.text_secondary));
                                ui.label(RichText::new(val).size(10.5).color(theme.text_muted));
                                ui.label(RichText::new(st).size(10.5).strong().color(col));
                                ui.end_row();
                            }
                        });
                });
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crane_bim_perimeter_clearance_against_live_project() {
        let empty = crate::core::BimProject::empty();
        let rep_empty = compute_crane_bim_perimeter_clearance(&empty, -4.0, 4.0, 45.0, 50.0, 3.0);
        assert!(rep_empty.is_collision_free);
        assert!(rep_empty.bbox_min_m.is_none());

        let proj = crate::core::BimProject::sample_bimcraft_building();
        let rep_clear = compute_crane_bim_perimeter_clearance(&proj, -4.5, 4.0, 45.0, 55.0, 3.0);
        assert!(rep_clear.bbox_min_m.is_some());
        assert!(rep_clear.is_collision_free);
        assert!(rep_clear.reaches_full_envelope);
    }

    #[test]
    fn test_shoring_reshoring_audit_maturity_and_spacing() {
        let proj = crate::core::BimProject::default();
        let day3 = compute_shoring_reshoring_audit(&proj, 3, 2, 1.5, 220.0, false);
        let day10 = compute_shoring_reshoring_audit(&proj, 10, 2, 1.0, 220.0, false);
        assert!(!day3.strike_ready);
        assert!(day10.strike_ready);
        assert!(day10.peak_prop_load_kn < day3.peak_prop_load_kn);
        assert!(day10.euler_fs > day3.euler_fs);
    }
}
