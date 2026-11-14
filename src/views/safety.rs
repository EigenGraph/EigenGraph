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
    // Fall Hazard Parameters
    pub guardrail_installed: bool,
    pub floor_openings_covered: bool,
    pub safety_harness_anchors_certified: bool,
    pub toe_boards_in_place: bool,
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
            guardrail_installed: true,
            floor_openings_covered: true,
            safety_harness_anchors_certified: true,
            toe_boards_in_place: true,
        }
    }
}

impl SafetyView {
    pub fn show(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        _members: &[StructuralMember],
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
                    SafetySubTab::CraneEnvelope => self.render_crane_envelope(ui, theme, features),
                    SafetySubTab::FallProtection => self.render_fall_protection(ui, theme),
                    SafetySubTab::SiteLogistics => self.render_site_logistics(ui, theme),
                    SafetySubTab::SafetyAuditMatrix => self.render_safety_audit_matrix(ui, theme),
                    SafetySubTab::ScaffoldingReshoring => {
                        self.render_scaffolding_reshoring(ui, theme, features);
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
        features: &mut FeatureRegistry,
    ) {
        let q_allow = self.calculate_crane_allowable_load();
        let is_overloaded = self.crane_current_load_ton > q_allow;
        let dcr = self.crane_current_load_ton / q_allow.max(0.01);

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

        // Phase 33 Locked Slot: 3D Crane Jib vs BIM Perimeter Clearance Check
        ui_kit::feature_slot_card(
            ui,
            theme,
            features,
            FeatureId::SitePerimeter3DCraneCollision,
            |ui| {
                ui.horizontal_wrapped(|ui| {
                    let _ = ui.selectable_label(true, "Guard: Active BimProject Envelopes");
                    let _ = ui.selectable_label(false, "Site Property Boundary Buffer (3.0 m)");
                    let _ = icons::icon_button(ui, AppIcon::Shield, "Sweep 360° 3D Boom vs Building Envelope", 11.5, theme.text_primary, theme.bg_canvas, Stroke::new(1.0, theme.border_subtle), CornerRadius::same(6), Vec2::ZERO);
                });
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
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(
            ui,
            theme,
            features,
            FeatureId::ScaffoldingFormworkReshoring,
            |ui| {
                let cards = [
                    ("STANDARD & SHORE RATIO", "2 Floors Shored + 1 Reshored", "ACI 347-14 Multi-Level Transfer", theme.accent_blue),
                    ("EARLY CONCRETE MATURITY", "74.2% f'c @ Day 7", "Min Strike Threshold >= 70% (Safe)", theme.accent_green),
                    ("PEAK PROP AXIAL LOAD", "24.8 kN / Post", "Ring-Lock Capacity = 38.0 kN (DCR 0.65)", theme.accent_green),
                    ("EULER SLENDERNESS CHECK", "FS_buckling = 2.45", "Braced @ 1.50m Vertical Lifts", theme.accent_orange),
                ];
                ui_kit::responsive_grid_n(ui, cards.len(), 185.0, |ui, idx| {
                    let (t, v, s, c) = cards[idx];
                    ui_kit::stat_card(ui, theme, t, v, s, c);
                });

                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    let _ = ui.selectable_label(true, "ACI 347 Slab Maturity Redistribution");
                    let _ = ui.selectable_label(false, "300mm Post-Tensioned Transfer Slab");
                    let _ = icons::icon_button(
                        ui,
                        AppIcon::CheckCircle,
                        "Verify Shoring Strip Sequence & Buckling FS",
                        12.0,
                        theme.text_primary,
                        theme.bg_card,
                        Stroke::new(1.0, theme.border_subtle),
                        CornerRadius::same(6),
                        Vec2::ZERO,
                    );
                });
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
                                let prop_col = if lvl == 0 { theme.accent_orange } else { theme.accent_green };
                                for p in 1..6 {
                                    let px = box_r.left() + (p as f32 / 6.0) * box_r.width();
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
                            "Fresh Pour (1.35x Shore Load) -> 7-Day Cured Slab -> Reshored Level",
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
                                    for (lvl, age, fc, sr, st, col) in [
                                        ("Level 06 Pour", "0 Days (Wet)", "0% f'c", "1.32x DL -> Shored", "SHORED", theme.accent_orange),
                                        ("Level 05 Deck", "7 Days", "74% f'c", "0.68x Req Cap", "RESHORE READY", theme.accent_green),
                                        ("Level 04 Deck", "14 Days", "91% f'c", "0.41x Nominal", "STRIPPED SAFE", theme.accent_green),
                                    ] {
                                        ui.label(RichText::new(lvl).size(10.5).strong().color(theme.text_primary));
                                        ui.label(RichText::new(age).size(10.5).color(theme.text_secondary));
                                        ui.label(RichText::new(fc).size(10.5).color(theme.accent_blue));
                                        ui.label(RichText::new(sr).size(10.5).color(theme.text_secondary));
                                        ui.label(RichText::new(st).size(10.5).strong().color(col));
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
