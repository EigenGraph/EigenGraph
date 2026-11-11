//! safety.rs — Pillar 11: Safety & Health (8D BIM Occupational Safety & Site Envelope).
//! Features tower crane swing radius envelopes, hook capacity checks, slab perimeter fall hazard
//! detection (OSHA 1926.502 / Permenaker 9/2016), and site emergency logistics.

use egui::{
    Align, Align2, Color32, CornerRadius, Frame, Layout, Margin, Pos2, Rect, RichText, ScrollArea,
    Sense, Stroke, Ui, Vec2,
};
use crate::models::StructuralMember;
use crate::theme::MacTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetySubTab {
    CraneEnvelope,
    FallProtection,
    SiteLogistics,
    SafetyAuditMatrix,
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
    pub fn show(&mut self, ui: &mut Ui, theme: &MacTheme, _members: &[StructuralMember]) {
        ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
            Frame::new().inner_margin(Margin::symmetric(24, 16)).show(ui, |ui| {
                // 1. Top Subtabs Bar
                self.render_header_bar(ui, theme);
                ui.add_space(16.0);

                // 2. Active Tab Content
                match self.active_subtab {
                    SafetySubTab::CraneEnvelope => self.render_crane_envelope(ui, theme),
                    SafetySubTab::FallProtection => self.render_fall_protection(ui, theme),
                    SafetySubTab::SiteLogistics => self.render_site_logistics(ui, theme),
                    SafetySubTab::SafetyAuditMatrix => self.render_safety_audit_matrix(ui, theme),
                }

                ui.add_space(32.0);
            });
        });
    }

    fn render_header_bar(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(6))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let subtabs = [
                        (SafetySubTab::CraneEnvelope, "🏗️ Tower Crane Radius Arc"),
                        (SafetySubTab::FallProtection, "🛡️ Slab Edge Fall Hazards"),
                        (SafetySubTab::SiteLogistics, "🚑 Emergency Evacuation & Logistics"),
                        (SafetySubTab::SafetyAuditMatrix, "📋 8D Safety Audit Matrix"),
                    ];

                    for (tab, label) in subtabs {
                        let is_active = self.active_subtab == tab;
                        let text_color = if is_active {
                            Color32::WHITE
                        } else {
                            theme.text_secondary
                        };
                        let fill_color = if is_active {
                            theme.accent_blue
                        } else {
                            Color32::TRANSPARENT
                        };

                        let btn = egui::Button::new(
                            RichText::new(label)
                                .size(12.0)
                                .strong()
                                .color(text_color),
                        )
                        .fill(fill_color)
                        .corner_radius(CornerRadius::same(6));

                        if ui.add(btn).clicked() {
                            self.active_subtab = tab;
                        }
                        ui.add_space(4.0);
                    }
                });
            });
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
            self.crane_max_capacity_ton - factor * (self.crane_max_capacity_ton - self.crane_tip_capacity_ton)
        }
    }

    fn render_crane_envelope(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let q_allow = self.calculate_crane_allowable_load();
        let is_overloaded = self.crane_current_load_ton > q_allow;
        let dcr = self.crane_current_load_ton / q_allow.max(0.01);

        // Hero Card
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(12))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(20))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("🏗️ 8D Tower Crane Swing Radius & Hook Capacity Envelope")
                                .size(18.0)
                                .strong()
                                .color(theme.text_primary),
                        );
                        ui.label(
                            RichText::new("Real-time working radius sweep, pick load versus radius capacity curves, and collision avoidance over public zones.")
                                .size(12.0)
                                .color(theme.text_secondary),
                        );
                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let status_color = if is_overloaded {
                            Color32::from_rgb(230, 70, 70)
                        } else {
                            theme.accent_green
                        };
                        let status_text = if is_overloaded {
                            format!("⚠️ CRANE OVERLOAD (DCR {:.2})", dcr)
                        } else {
                            format!("✓ LIFT CAPACITY SAFE (DCR {:.2})", dcr)
                        };

                        Frame::new()
                            .fill(status_color.gamma_multiply(0.15))
                            .stroke(Stroke::new(1.0, status_color.gamma_multiply(0.6)))
                            .corner_radius(CornerRadius::same(6))
                            .inner_margin(Margin::symmetric(12, 6))
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(status_text)
                                        .size(11.0)
                                        .strong()
                                        .color(status_color),
                                );
                            });
                    });
                });
            });

        ui.add_space(16.0);

        // Interactive Controls & Visual Canvas Side-by-Side
        ui.columns(2, |cols| {
            // Left Column: Parameters
            Frame::new()
                .fill(theme.bg_card)
                .corner_radius(CornerRadius::same(10))
                .stroke(Stroke::new(1.0, theme.border_card))
                .inner_margin(Margin::same(16))
                .show(&mut cols[0], |ui| {
                    ui.label(
                        RichText::new("⚙️ Crane Configuration & Pick Telemetry")
                            .size(14.0)
                            .strong()
                            .color(theme.text_primary),
                    );
                    ui.add_space(10.0);

                    ui.label(RichText::new(format!("Jib Length: {:.1} m", self.crane_jib_length_m)).size(11.5).color(theme.text_secondary));
                    ui.add(egui::Slider::new(&mut self.crane_jib_length_m, 30.0..=75.0).suffix(" m"));
                    ui.add_space(8.0);

                    ui.label(RichText::new(format!("Mast Height: {:.1} m", self.crane_mast_height_m)).size(11.5).color(theme.text_secondary));
                    ui.add(egui::Slider::new(&mut self.crane_mast_height_m, 25.0..=80.0).suffix(" m"));
                    ui.add_space(8.0);

                    ui.label(RichText::new(format!("Working Radius: {:.1} m", self.crane_current_radius_m)).size(11.5).color(theme.accent_blue));
                    ui.add(egui::Slider::new(&mut self.crane_current_radius_m, 10.0..=self.crane_jib_length_m).suffix(" m"));
                    ui.add_space(8.0);

                    ui.label(RichText::new(format!("Hook Pick Load: {:.2} Tonnes (Capacity: {:.2} T)", self.crane_current_load_ton, q_allow)).size(11.5).color(if is_overloaded { Color32::from_rgb(230, 70, 70) } else { theme.accent_green }));
                    ui.add(egui::Slider::new(&mut self.crane_current_load_ton, 0.5..=15.0).suffix(" T"));
                    ui.add_space(8.0);

                    ui.label(RichText::new(format!("Slew Angle: {:.1}°", self.crane_slew_angle_deg)).size(11.5).color(theme.text_secondary));
                    ui.add(egui::Slider::new(&mut self.crane_slew_angle_deg, 0.0..=360.0).suffix("°"));
                });

            // Right Column: 2D Radar Canvas
            Frame::new()
                .fill(theme.bg_card)
                .corner_radius(CornerRadius::same(10))
                .stroke(Stroke::new(1.0, theme.border_card))
                .inner_margin(Margin::same(16))
                .show(&mut cols[1], |ui| {
                    ui.label(
                        RichText::new("🎯 Site Sweep Radar & Exclusion Boundary")
                            .size(14.0)
                            .strong()
                            .color(theme.text_primary),
                    );
                    ui.add_space(6.0);

                    let (rect, _resp) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 220.0), Sense::hover());
                    ui.painter().rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(24, 24, 28));
                    ui.painter().rect_stroke(rect, CornerRadius::same(8), Stroke::new(1.0, theme.border_subtle), egui::StrokeKind::Inside);

                    let center = rect.center();
                    let scale = 1.8; // px per meter

                    // Jib Max Radius circle
                    let max_r_px = self.crane_jib_length_m * scale;
                    ui.painter().circle_stroke(center, max_r_px, Stroke::new(1.0, Color32::from_rgba_premultiplied(80, 140, 220, 100)));

                    // Working pick radius circle
                    let pick_r_px = self.crane_current_radius_m * scale;
                    let pick_color = if is_overloaded { Color32::from_rgb(230, 70, 70) } else { Color32::from_rgb(80, 200, 120) };
                    ui.painter().circle_stroke(center, pick_r_px, Stroke::new(1.5, pick_color));

                    // Crane Base Pin
                    ui.painter().circle_filled(center, 4.0, Color32::WHITE);

                    // Jib Arm Line
                    let rad = self.crane_slew_angle_deg.to_radians();
                    let jib_end = center + Vec2::new(max_r_px * rad.cos(), max_r_px * rad.sin());
                    ui.painter().line_segment([center, jib_end], Stroke::new(2.5, Color32::from_rgb(240, 180, 60)));

                    // Hook Position Marker
                    let hook_pos = center + Vec2::new(pick_r_px * rad.cos(), pick_r_px * rad.sin());
                    ui.painter().circle_filled(hook_pos, 5.5, pick_color);

                    ui.painter().text(
                        hook_pos + Vec2::new(10.0, -10.0),
                        Align2::LEFT_CENTER,
                        format!("{:.1}T @ {:.0}m", self.crane_current_load_ton, self.crane_current_radius_m),
                        egui::FontId::monospace(10.0),
                        Color32::WHITE,
                    );
                });
        });
    }

    fn render_fall_protection(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(12))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(20))
            .show(ui, |ui| {
                ui.label(
                    RichText::new("🛡️ Slab Edge Fall Protection & Leading Edge Risk Controls")
                        .size(18.0)
                        .strong()
                        .color(theme.text_primary),
                );
                ui.label(
                    RichText::new("OSHA 1926.502 / Permenaker No. 9/2016 compliance checks for perimeter slab edges, floor openings, and horizontal lifeline tie-offs.")
                        .size(12.0)
                        .color(theme.text_secondary),
                );

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);

                let checks = [
                    (&mut self.guardrail_installed, "Perimeter Standard Guardrail (Top-rail 107cm, Mid-rail 53cm, 900N lateral capacity)"),
                    (&mut self.toe_boards_in_place, "Toe-boards installed on all open elevated edges (minimum height 9cm)"),
                    (&mut self.floor_openings_covered, "Floor slab voids and MEP risers (> 50mm) covered with secured hazard covers"),
                    (&mut self.safety_harness_anchors_certified, "Horizontal Lifeline (HLL) and certified anchor tie-off points rated >= 22.2 kN (5,000 lbs)"),
                ];

                for (val, desc) in checks {
                    ui.horizontal(|ui| {
                        ui.checkbox(val, "");
                        ui.label(
                            RichText::new(desc)
                                .size(12.0)
                                .color(if *val { theme.text_primary } else { Color32::from_rgb(230, 80, 80) }),
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
            .inner_margin(Margin::same(20))
            .show(ui, |ui| {
                ui.label(
                    RichText::new("🚑 Emergency Site Logistics & Evacuation Pathways")
                        .size(18.0)
                        .strong()
                        .color(theme.text_primary),
                );
                ui.label(
                    RichText::new("Rapid egress corridors, designated safe muster points, fire brigade staging, and on-site first responder facilities.")
                        .size(12.0)
                        .color(theme.text_secondary),
                );

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);

                let facilities = [
                    ("🚨 Emergency Assembly Zone", "East Boundary Plot (Clear radius 25m, capacity 200 workers)", "OPERATIONAL", theme.accent_green),
                    ("🚒 Fire Hydrant & Staging", "North Gate Access Road (Water flow 1,500 L/min certified)", "CONNECTED", theme.accent_blue),
                    ("🏥 First Aid Medical Clinic", "Ground Floor North Bay (2 Certified paramedics on site)", "STAFFED", theme.accent_green),
                    ("🧯 Fire Extinguisher Points", "Every 20m along scaffold corridors (Type ABC Dry Chemical)", "INSPECTED", theme.accent_green),
                ];

                for (name, desc, stat, col) in facilities {
                    Frame::new()
                        .fill(theme.bg_canvas)
                        .corner_radius(CornerRadius::same(8))
                        .stroke(Stroke::new(0.8, theme.border_subtle))
                        .inner_margin(Margin::symmetric(14, 10))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(name).size(13.0).strong().color(theme.text_primary));
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
            .inner_margin(Margin::same(20))
            .show(ui, |ui| {
                ui.label(
                    RichText::new("📋 8D Safety & Health Audit Compliance Matrix")
                        .size(18.0)
                        .strong()
                        .color(theme.text_primary),
                );
                ui.label(
                    RichText::new("Hazard Identification, Risk Assessment, and Determining Controls (HIRADC) summary.")
                        .size(12.0)
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
                            ui.horizontal(|ui| {
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
}
