//! facility.rs — Pillar 7: 7D BIM Facility Management.
//! Lifecycle operation and maintenance data used by building owners to manage assets, repairs, and components after construction is complete.

use egui::{Align, Color32, CornerRadius, Frame, Layout, Margin, RichText, ScrollArea, Stroke, Ui, Vec2};
use crate::models::StructuralMember;
use crate::theme::MacTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FacilitySubTab {
    DigitalTwinShm,
    AssetRegistry,
    PredictiveMaintenance,
    LifecycleCosting,
}

pub struct FacilityView {
    pub active_subtab: FacilitySubTab,
    pub live_telemetry_active: bool,
    pub simulated_strain_micro: f32,
    pub simulated_freq_hz: f32,
    pub simulated_degradation_pct: f32,
}

impl Default for FacilityView {
    fn default() -> Self {
        Self {
            active_subtab: FacilitySubTab::DigitalTwinShm,
            live_telemetry_active: true,
            simulated_strain_micro: 342.0,
            simulated_freq_hz: 3.48,
            simulated_degradation_pct: 1.8,
        }
    }
}

impl FacilityView {
    pub fn show(&mut self, ui: &mut Ui, theme: &MacTheme, members: &[StructuralMember]) {
        ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
            Frame::new().inner_margin(Margin::symmetric(20, 12)).show(ui, |ui| {
                // 1. TOP SUBTABS SELECTOR
                self.render_header_bar(ui, theme);

                ui.add_space(16.0);

                // 2. ACTIVE SUB-TAB CONTENT
                match self.active_subtab {
                    FacilitySubTab::DigitalTwinShm => self.render_digital_twin_shm(ui, theme, members),
                    FacilitySubTab::AssetRegistry => self.render_asset_registry(ui, theme, members),
                    FacilitySubTab::PredictiveMaintenance => self.render_predictive_maintenance(ui, theme),
                    FacilitySubTab::LifecycleCosting => self.render_lifecycle_costing(ui, theme),
                }

                ui.add_space(24.0);
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
                let tabs = [
                    (FacilitySubTab::DigitalTwinShm, "📡 Digital Twin & SHM Sensors"),
                    (FacilitySubTab::AssetRegistry, "📑 COBie Asset Registry"),
                    (FacilitySubTab::PredictiveMaintenance, "🛠️ Predictive Maintenance (ARI Alert)"),
                    (FacilitySubTab::LifecycleCosting, "💵 30-Year Lifecycle Cost (LCC)"),
                ];

                ui.columns(4, |cols| {
                    for (idx, (tab, label)) in tabs.into_iter().enumerate() {
                        let is_active = self.active_subtab == tab;
                        let bg = if is_active { theme.accent_blue } else { Color32::TRANSPARENT };
                        let fg = if is_active { Color32::WHITE } else { theme.text_secondary };

                        let w = cols[idx].available_width();
                        if cols[idx].add(egui::Button::new(RichText::new(label).size(12.0).strong().color(fg))
                            .fill(bg)
                            .corner_radius(CornerRadius::same(7))
                            .min_size(Vec2::new(w, 30.0))).clicked() {
                            self.active_subtab = tab;
                        }
                    }
                });
            });
    }

    fn render_digital_twin_shm(&mut self, ui: &mut Ui, theme: &MacTheme, _members: &[StructuralMember]) {
        // Top Real-time Sensor Status Row
        ui.columns(4, |cols| {
            self.render_sensor_card(&mut cols[0], theme, "STRAIN GAUGE SG-01", &format!("{:.0} µε", self.simulated_strain_micro), "Member E1 Strain (Safe < 1200 µε)", theme.accent_green);
            self.render_sensor_card(&mut cols[1], theme, "NATURAL FREQUENCY (ACC-01)", &format!("{:.2} Hz", self.simulated_freq_hz), "Mode 1 Transverse (Resonance Free)", theme.accent_blue);
            self.render_sensor_card(&mut cols[2], theme, "LIVE ARI DEGRADATION", "0.938", "Real-Time ARI(t) Degradation (-0.4%)", theme.accent_green);
            self.render_sensor_card(&mut cols[3], theme, "HEALTH STATUS (SHM)", "🟢 NORMAL", "16 IoT Sensors Operating Active", theme.accent_green);
        });

        ui.add_space(16.0);

        // Digital Twin Bridge Telemetry Canvas
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("📡 Digital Twin Sensor Telemetry & Dynamic ARI Mapping").size(13.5).strong().color(theme.text_primary));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new("Sampling: 100 Hz • MQTT/TLS Protocol").size(11.0).color(theme.accent_blue));
                    });
                });
                ui.add_space(12.0);

                let sensors = [
                    ("SG-01 (Mid-Span Bottom Chord)", "Fiber Optic Strain Gauge", "342 µε", "68.4 MPa", "ARI: 0.94", "🟢 Operational Normal"),
                    ("SG-02 (Mid-Span Top Chord)", "Electrical Resistance Strain Gauge", "-285 µε", "-57.0 MPa", "ARI: 0.94", "🟢 Elastic Compression Safe"),
                    ("ACC-01 (Mid-Span Girder)", "Triaxial Piezoelectric Accelerometer", "0.042 g", "3.48 Hz", "Condition: Rigid", "🟢 Controlled Dynamic Vibration"),
                    ("TILT-01 (Right Roller Support)", "Dual-Axis Digital Inclinometer", "0.018°", "Deflection 1.4 mm", "Rotation Safe", "🟢 Free Roller Expansion"),
                    ("UT-01 (Critical Gusset Plate)", "Ultrasonic Thickness Sensor", "15.94 mm", "Corrosion Rate: 0.01 mm/yr", "Remaining Life: 94 Yrs", "🟢 Paint Protective Coating Intact"),
                ];

                for (id, stype, val, derived, ari_info, status) in sensors {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(RichText::new(id).size(12.0).strong().color(theme.text_primary));
                            ui.label(RichText::new(stype).size(11.0).color(theme.text_secondary));
                        });
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(RichText::new(status).size(11.5).color(theme.accent_green));
                            ui.add_space(16.0);
                            ui.label(RichText::new(ari_info).size(11.5).strong().color(theme.accent_blue));
                            ui.add_space(16.0);
                            ui.label(RichText::new(derived).size(11.5).color(theme.text_secondary));
                            ui.add_space(16.0);
                            ui.label(RichText::new(val).size(11.5).strong().color(theme.text_primary));
                        });
                    });
                    ui.separator();
                }
            });
    }

    fn render_asset_registry(&mut self, ui: &mut Ui, theme: &MacTheme, _members: &[StructuralMember]) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("📑 Facility Asset Registry (COBie / IFC 4.3 Standard)").size(13.5).strong().color(theme.text_primary));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new("Total Assets: 48 Indexed Components").size(11.5).color(theme.text_muted));
                    });
                });
                ui.add_space(12.0);

                let assets = [
                    ("AST-CH-001", "Bottom Chord W14x90", "Structural Steel", "ASTM A992", "2026-10-15", "100 Years", "Every 2 Years", "🟢 Grade 1 (New)"),
                    ("AST-CH-002", "Top Chord W14x90", "Structural Steel", "ASTM A992", "2026-10-15", "100 Years", "Every 2 Years", "🟢 Grade 1 (New)"),
                    ("AST-DG-003", "Diagonal Web HSS8x8x1/2", "Secondary Bracing", "ASTM A500 Gr.B", "2026-10-15", "100 Years", "Every 3 Years", "🟢 Grade 1 (New)"),
                    ("AST-BRG-01", "Elastomeric Pot Bearing 150T", "Bridge Bearing", "Neoprene/Steel", "2026-10-15", "25 Years", "Every Year", "🟢 Grade 1 (Prime)"),
                    ("AST-EXP-01", "Modular Expansion Joint 80mm", "Floor Expansion Joint", "Galvanized Steel", "2026-10-15", "20 Years", "Every 6 Months", "🟢 Grade 1 (Gap-Free)"),
                ];

                for (tag, name, cat, spec, comm, life, interval, cond) in assets {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(RichText::new(format!("{} • {}", tag, name)).size(12.0).strong().color(theme.text_primary));
                            ui.label(RichText::new(format!("{} | {} | Design Life: {}", cat, spec, life)).size(11.0).color(theme.text_secondary));
                        });
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(RichText::new(cond).size(11.5).color(theme.accent_green));
                            ui.add_space(16.0);
                            ui.label(RichText::new(interval).size(11.5).color(theme.accent_blue));
                            ui.add_space(16.0);
                            ui.label(RichText::new(comm).size(11.0).color(theme.text_muted));
                        });
                    });
                    ui.separator();
                }
            });
    }

    fn render_predictive_maintenance(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.label(RichText::new("🛠️ Predictive Maintenance Protocols via Redundancy Alerts (ARI Alert)").size(13.5).strong().color(theme.text_primary));
                ui.add_space(6.0);
                ui.label(RichText::new("Automated Work Orders (WO) issued when material degradation or fatigue metrics approach critical resilience thresholds.").size(11.5).color(theme.text_secondary));
                ui.add_space(14.0);

                let work_orders = [
                    ("WO-2026-088", "Visual Inspection & Torque Verification of Gusset Joint J-04 Bolts", "Bi-annual Routine", "Scheduled: Apr 15, 2027", "🟢 Planned", theme.accent_blue),
                    ("WO-2026-089", "NDT Ultrasonic Testing of Critical Tension Member E1 (Bottom Chord)", "Based on ARI Sensitivity (CI=0.42)", "Scheduled: Oct 10, 2027", "🟢 Ready to Execute", theme.accent_green),
                    ("WO-2026-090", "Drainage Channel Cleaning & Bearing Expansion Clearing", "Prevent Chloride/Moisture Corrosion", "Scheduled: Dec 01, 2026", "🟡 Upcoming", theme.text_secondary),
                ];

                for (code, task, reason, sched, status, col) in work_orders {
                    Frame::new()
                        .fill(theme.bg_canvas)
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(Margin::same(12))
                        .stroke(Stroke::new(1.0, theme.border_subtle))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(format!("{} — {}", code, task)).size(12.5).strong().color(theme.text_primary));
                                    ui.label(RichText::new(format!("Rationale: {} | {}", reason, sched)).size(11.0).color(theme.text_secondary));
                                });
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.label(RichText::new(status).size(12.0).strong().color(col));
                                });
                            });
                        });
                    ui.add_space(8.0);
                }
            });
    }

    fn render_lifecycle_costing(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.label(RichText::new("💵 30-Year Life Cycle Costing (LCC) Analysis").size(13.5).strong().color(theme.text_primary));
                ui.add_space(10.0);

                ui.columns(3, |cols| {
                    self.render_sensor_card(&mut cols[0], theme, "INITIAL CAPITAL (CAPEX)", "$3.84 M", "Initial Construction Verified via 5D BoQ", theme.text_primary);
                    self.render_sensor_card(&mut cols[1], theme, "ESTIMATED OPEX (30 YRS)", "$645 K", "Routine Maintenance & SHM Sensors", theme.accent_blue);
                    self.render_sensor_card(&mut cols[2], theme, "EARLY ARI SAVINGS", "$1.45 M", "Catastrophic Failure Avoidance", theme.accent_green);
                });
            });
    }

    fn render_sensor_card(&self, ui: &mut Ui, theme: &MacTheme, title: &str, val: &str, subtitle: &str, val_color: Color32) {
        Frame::new()
            .fill(theme.bg_canvas)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(14))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(RichText::new(title).size(10.0).strong().color(theme.text_muted));
                ui.add_space(4.0);
                ui.label(RichText::new(val).size(18.0).strong().color(val_color));
                ui.add_space(4.0);
                ui.label(RichText::new(subtitle).size(10.5).color(theme.text_secondary));
            });
    }
}
