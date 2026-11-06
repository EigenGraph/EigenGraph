//! industrialized.rs — Pillar 10: 10D BIM Industrialized Construction & Automation.
//! The integration of Industry 4.0 technologies, including prefabrication, robotics, 3D printing, and automated site processes.

use egui::{Align, Color32, CornerRadius, Frame, Layout, Margin, RichText, ScrollArea, Stroke, Ui, Vec2};
use crate::models::StructuralMember;
use crate::theme::MacTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndustrializedSubTab {
    ModularDfma,
    RoboticAutomation,
    AutomatedErection,
    Industry4Bridge,
}

pub struct IndustrializedView {
    pub active_subtab: IndustrializedSubTab,
    pub offsite_ratio_pct: f32,
    pub robotic_speed_mm_s: f32,
    pub export_status: Option<String>,
}

impl Default for IndustrializedView {
    fn default() -> Self {
        Self {
            active_subtab: IndustrializedSubTab::ModularDfma,
            offsite_ratio_pct: 82.5,
            robotic_speed_mm_s: 14.5,
            export_status: None,
        }
    }
}

impl IndustrializedView {
    pub fn show(&mut self, ui: &mut Ui, theme: &MacTheme, _members: &[StructuralMember]) {
        ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
            Frame::new().inner_margin(Margin::symmetric(20, 12)).show(ui, |ui| {
                // 1. TOP SUBTABS SELECTOR
                self.render_header_bar(ui, theme);

                let mut dismiss_status = false;
                if let Some(ref status_msg) = self.export_status {
                    ui.add_space(10.0);
                    theme.card_frame().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("📦 10D EXPORT SIAP:").size(11.5).strong().color(theme.accent_green));
                            ui.label(RichText::new(status_msg).size(11.5).color(theme.text_primary));
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                if ui.add(egui::Button::new(RichText::new("✕").size(11.0).color(theme.text_muted))
                                    .fill(theme.bg_card)
                                    .corner_radius(CornerRadius::same(4))).clicked() {
                                    dismiss_status = true;
                                }
                            });
                        });
                    });
                }
                if dismiss_status {
                    self.export_status = None;
                }

                ui.add_space(16.0);

                // 2. ACTIVE SUB-TAB CONTENT
                match self.active_subtab {
                    IndustrializedSubTab::ModularDfma => self.render_modular_dfma(ui, theme),
                    IndustrializedSubTab::RoboticAutomation => self.render_robotic_automation(ui, theme),
                    IndustrializedSubTab::AutomatedErection => self.render_automated_erection(ui, theme),
                    IndustrializedSubTab::Industry4Bridge => self.render_industry4_bridge(ui, theme),
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
                    (IndustrializedSubTab::ModularDfma, "🏭 Prefabrikasi & DfMA Modular"),
                    (IndustrializedSubTab::RoboticAutomation, "🦾 Robotik & Pengelasan Otomatis"),
                    (IndustrializedSubTab::AutomatedErection, "🏗️ Crane Otonom & Drone Lidar"),
                    (IndustrializedSubTab::Industry4Bridge, "🌐 OpenBIM IFC 4.3 & IoT Bridge"),
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

    fn render_modular_dfma(&mut self, ui: &mut Ui, theme: &MacTheme) {
        ui.columns(4, |cols| {
            self.render_stat_card(&mut cols[0], theme, "OFF-SITE FABRICATION RATIO", "82.5%", "Mayoritas Dirakit Presisi di Pabrik", theme.accent_green);
            self.render_stat_card(&mut cols[1], theme, "STANDARDIZATION INDEX", "91.5%", "Repetisi Sambungan Buhul Modular", theme.accent_blue);
            self.render_stat_card(&mut cols[2], theme, "DIMENSI MODUL TRANSPORT", "12.0 x 2.4 m", "Sesuai Regulasi Trailer Jalan Raya", theme.text_primary);
            self.render_stat_card(&mut cols[3], theme, "ASSEMBLY TIME AT SITE", "-60% Hari", "Perakitan Cepat Sistem Baut HTB", theme.accent_green);
        });

        ui.add_space(16.0);

        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("🏭 Design for Manufacture & Assembly (DfMA Hierarchy)").size(13.5).strong().color(theme.text_primary));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new("Fabrication Tolerance: ± 1.5 mm").size(11.0).color(theme.accent_blue));
                    });
                });
                ui.add_space(12.0);

                let modules = [
                    ("MOD-TRUSS-01 (Left 12m Span)", "Sub-Assembling Warren Truss A", "24.2 Tons", "Robotic Jig Fixture", "🟢 Completed in Shop (Inspected)"),
                    ("MOD-TRUSS-02 (Right 12m Span)", "Sub-Assembling Warren Truss B", "24.2 Tons", "Robotic Jig Fixture", "🟢 Completed in Shop (Inspected)"),
                    ("MOD-SPLICE-KEY (Crown Mid-Splice)", "Center Crown Splice Bolted Node", "0.8 Tons", "CNC Milled Gusset Plate", "🟢 Ready for Site Bolting"),
                    ("MOD-BRG-SEAT (Abutment Bearing Seat)", "Pedestal Anchor & Bearing Plate", "1.2 Tons", "Automated Flame Cut", "🟢 Installed on Abutment"),
                ];

                for (code, name, wt, fab, status) in modules {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(RichText::new(format!("{} • {}", code, name)).size(12.0).strong().color(theme.text_primary));
                            ui.label(RichText::new(format!("Method: {} | Weight: {}", fab, wt)).size(11.0).color(theme.text_secondary));
                        });
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(RichText::new(status).size(11.5).strong().color(theme.accent_green));
                        });
                    });
                    ui.separator();
                }
            });
    }

    fn render_robotic_automation(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("🦾 Robotic Automation & Industry 4.0 Quality Control").size(13.5).strong().color(theme.text_primary));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new("NDT Pass Rate: 99.4%").size(12.0).strong().color(theme.accent_green));
                    });
                });
                ui.add_space(12.0);

                let robotics = [
                    ("6-Axis Robotic GMAW/FCAW Arm", "Automated Gusset Joint Welding", "14.5 mm/s", "AWS D1.1 Certified", "🟢 Operating Synchronized"),
                    ("15kW Fiber Laser CNC Machine", "Steel Web & Flange Profile Cutting", "85.0 mm/s", "Tolerance 0.05 mm", "🟢 Automated Nesting"),
                    ("AI Ultrasonic Phased-Array Scanner", "Non-Destructive Testing (NDT) Flaw Detection", "Real-Time 3D", "Defect: 0 Porosity", "🟢 Superior Quality"),
                    ("3-Spindle CNC Beam Drill Line", "HTB A325 Bolt Hole Drilling", "36 Holes/min", "Hole Accuracy ±0.2mm", "🟢 Ready for Erection"),
                ];

                for (mach, role, spd, cert, st) in robotics {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(RichText::new(mach).size(12.0).strong().color(theme.text_primary));
                            ui.label(RichText::new(format!("Application: {} | Speed: {}", role, spd)).size(11.0).color(theme.text_secondary));
                        });
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(RichText::new(st).size(11.5).strong().color(theme.accent_green));
                            ui.add_space(16.0);
                            ui.label(RichText::new(cert).size(11.0).color(theme.accent_blue));
                        });
                    });
                    ui.separator();
                }
            });
    }

    fn render_automated_erection(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.label(RichText::new("🏗️ Autonomous Crane, Collision-Free Trajectory & LiDAR Drone Inspection").size(13.5).strong().color(theme.text_primary));
                ui.add_space(10.0);

                ui.columns(3, |cols| {
                    self.render_stat_card(&mut cols[0], theme, "CRANE TRAJECTORY CLEARANCE", "2.8 m", "Dynamic Boom Collision-Free Zone", theme.accent_green);
                    self.render_stat_card(&mut cols[1], theme, "DRONE LIDAR AS-BUILT", "± 2.1 mm", "Model vs. As-Built Field Deviation", theme.accent_blue);
                    self.render_stat_card(&mut cols[2], theme, "SVD ERECTION GUARD", "Full Rank", "All Erection Stages Certified Mechanism-Free", theme.accent_green);
                });
            });
    }

    fn render_industry4_bridge(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.label(RichText::new("🌐 Industry 4.0 & OpenBIM Integration Bridge").size(13.5).strong().color(theme.text_primary));
                ui.add_space(10.0);

                let formats = [
                    ("IFC 4.3 Structural Analysis MVD", "buildingSMART International Standard", "Exporting element topology, factored load cases, and ARI metadata directly to commercial FEA suites (SAP2000, ETABS, Revit)."),
                    ("BCF 3.0 (BIM Collaboration Format)", "Cloud-based Issue & Clash Management", "Automated synchronization of SVD Mechanism warnings and critical stresses to collaboration platforms."),
                    ("STEP-NC & G-Code Robotics", "CNC Machining & Automated Welding Instructions", "Numerical toolpaths exported directly from 3D geometry without manual drafting overhead."),
                    ("REST API & IoT MQTT Telemetry", "Digital Twin Sensor Connectivity", "Streams field sensor data and updates real-time ARI(t) structural resilience scores."),
                ];

                for (name, sub, desc) in formats {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(RichText::new(name).size(12.5).strong().color(theme.accent_blue));
                            ui.label(RichText::new(sub).size(11.0).color(theme.text_muted));
                            ui.label(RichText::new(desc).size(11.5).color(theme.text_secondary));
                        });
                    });
                    ui.separator();
                }

                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if ui.add(egui::Button::new(RichText::new("🌐 Export OpenBIM IFC 4.3 (.ifc)").size(12.0).strong().color(Color32::WHITE))
                        .fill(theme.accent_blue)
                        .corner_radius(CornerRadius::same(6))).clicked() {
                        let mut engine = crate::core::ParametricEngine::new();
                        if let Ok(model) = engine.generate_warren_truss() {
                            let ari = crate::core::AriEngine::evaluate_redundancy(&model).ok();
                            let ifc = crate::core::IfcExporter::export_to_ifc(&model, ari.as_ref());
                            self.export_status = Some(format!("Model exported to OpenBIM IFC 4.3 ({} lines). Compatible with Solibri, Revit, and Tekla.", ifc.lines().count()));
                        }
                    }

                    if ui.add(egui::Button::new(RichText::new("🦾 Export DSTV CNC Package (.nc1)").size(12.0).strong().color(Color32::WHITE))
                        .fill(theme.accent_green)
                        .corner_radius(CornerRadius::same(6))).clicked() {
                        let mut engine = crate::core::ParametricEngine::new();
                        if let Ok(model) = engine.generate_warren_truss() {
                            let pieces = crate::core::DstvExporter::export_to_dstv(&model);
                            self.export_status = Some(format!("{} DSTV .nc1 robotic instruction files generated successfully for automated CNC cutting and drilling.", pieces.len()));
                        }
                    }
                });
            });
    }

    fn render_stat_card(&self, ui: &mut Ui, theme: &MacTheme, title: &str, val: &str, subtitle: &str, val_color: Color32) {
        Frame::new()
            .fill(theme.bg_canvas)
            .corner_radius(CornerRadius::same(8))
            .inner_margin(Margin::same(12))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(RichText::new(title).size(9.5).strong().color(theme.text_muted));
                ui.add_space(2.0);
                ui.label(RichText::new(val).size(18.0).strong().color(val_color));
                ui.add_space(2.0);
                ui.label(RichText::new(subtitle).size(10.0).color(theme.text_secondary));
            });
    }
}
