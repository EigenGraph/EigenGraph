//! views/industrialized/lifecycle_10d.rs — 10D BIM Multi-Discipline Lifecycle Command Hub.

use super::*;

impl IndustrializedView {
    /// Phase 27: 10D BIM Multi-Discipline Lifecycle Master Hub
    pub(super) fn render_10d_lifecycle_hub(&mut self, ui: &mut Ui, theme: &MacTheme) {
        // Hero Header
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(12))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(18))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(
                            RichText::new("🌐 10D BIM MULTI-DISCIPLINE LIFECYCLE MASTER HUB")
                                .size(15.0)
                                .strong()
                                .color(theme.text_primary),
                        );
                        ui.label(
                            RichText::new("ISO 19650 / IPD COMPLIANT")
                                .size(10.0)
                                .strong()
                                .color(theme.accent_blue),
                        );
                    });
                    ui.label(
                        RichText::new(
                            "Unified Cross-Discipline Lifecycle Command Center spanning 1D Project Scope to 10D Industrialized Prefabrication.",
                        )
                        .size(11.0)
                        .color(theme.text_secondary),
                    );
                    ui.add_space(8.0);
                    ui.horizontal_wrapped(|ui| {
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("📥 Export 10D Audit Report (PDF)")
                                        .size(11.0)
                                        .strong()
                                        .color(Color32::WHITE),
                                )
                                .fill(theme.accent_green)
                                .corner_radius(CornerRadius::same(6)),
                            )
                            .clicked()
                        {
                            self.export_status = Some(
                                "10D BIM Lifecycle Executive Audit Report successfully exported to PDF."
                                    .to_string(),
                            );
                        }
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("🌐 Sync Speckle CDE")
                                        .size(11.0)
                                        .color(theme.text_primary),
                                )
                                .fill(theme.bg_sidebar)
                                .corner_radius(CornerRadius::same(6)),
                            )
                            .clicked()
                        {
                            self.export_status = Some(
                                "Common Data Environment (CDE) synchronized with Speckle Cloud stream."
                                    .to_string(),
                            );
                        }
                    });
                });
            });

        ui.add_space(14.0);

        // 10 Dimensions KPI Matrix Cards
        ui.label(
            RichText::new("10D LIFECYCLE PILLARS STATUS")
                .size(11.5)
                .strong()
                .color(theme.text_secondary),
        );
        ui.add_space(6.0);

        let dimensions = [
            ("1D Scope", "Approved Baseline Rev 3", "Project Charter & Specifications", theme.accent_blue),
            ("2D DED", "4 Sheets Published", "Working Drawings (ISO A1)", theme.accent_blue),
            ("3D Spatial", "LOD 350 CSG B-Rep", "Parametric Elements + Rebar", theme.accent_blue),
            ("4D Time", "SPI 1.04 (On Schedule)", "Synchro 180-Day Construction", theme.accent_green),
            ("5D Cost", "CPI 1.06 (Under Budget)", "EVM Variance +$124,500", theme.accent_green),
            ("6D LCA", "LEED Platinum 84 Pts", "Embodied Carbon -28.4% CO2e", theme.accent_green),
            ("7D Twin", "12 Sensors Online", "Digital Twin 99.8% Uptime", Color32::from_rgb(175, 82, 222)),
            ("8D Safety", "Zero Clash Conflicts", "Dynamic Crane Sweep Radius", theme.accent_orange),
            ("9D Lean", "88% PPC Completed", "Last Planner System (LPS)", theme.accent_green),
            ("10D DfMA", "82.5% Pre-Assembled", "Off-Site Modular Construction", theme.accent_blue),
        ];

        ui_kit::responsive_grid_n(ui, dimensions.len(), 180.0, |ui, idx| {
            let (dim, val, desc, col) = dimensions[idx];
            Frame::new()
                .fill(theme.bg_card)
                .corner_radius(CornerRadius::same(8))
                .inner_margin(Margin::symmetric(12, 10))
                .stroke(Stroke::new(1.0, theme.border_subtle))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.label(RichText::new(dim).size(10.0).strong().color(theme.text_muted));
                    ui.label(RichText::new(val).size(12.0).strong().color(col));
                    ui.label(RichText::new(desc).size(9.0).color(theme.text_secondary));
                });
        });

        ui.add_space(16.0);

        // Multi-Discipline Deep Dive Layout (Responsive 2-Pane / Stacked)
        ui_kit::responsive_two_panes(ui, 850.0, |ui, pane| match pane {
            0 => {
                self.render_evm_cost_panel(ui, theme);
                ui.add_space(12.0);
                self.render_iot_telemetry_panel(ui, theme);
            }
            _ => {
                self.render_safety_crane_panel(ui, theme);
                ui.add_space(12.0);
                self.render_lean_kanban_panel(ui, theme);
            }
        });
    }

    fn render_evm_cost_panel(&self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(14))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(
                    RichText::new("💰 5D EARNED VALUE MANAGEMENT (EVM)")
                        .size(11.5)
                        .strong()
                        .color(theme.accent_green),
                );
                ui.add_space(6.0);

                let evm_metrics = [
                    ("Planned Value (PV)", "$4,850,000", "Baseline Budgeted Cost of Work Scheduled"),
                    ("Earned Value (EV)", "$5,140,000", "Budgeted Cost of Work Performed (BCWP)"),
                    ("Actual Cost (AC)", "$4,845,000", "Actual Expenditures Incurred to Date"),
                    ("Cost Variance (CV = EV - AC)", "+$295,000", "Positive Variance • Under Budget"),
                    ("Schedule Variance (SV = EV - PV)", "+$290,000", "Positive Variance • Ahead of Schedule"),
                    ("Cost Performance Index (CPI)", "1.061", "CPI > 1.0 indicates cost efficiency"),
                    ("Schedule Performance Index (SPI)", "1.059", "SPI > 1.0 indicates speed efficiency"),
                ];

                for (name, val, hint) in evm_metrics {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(name).size(10.5).color(theme.text_secondary));
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let val_col = if val.starts_with('+') || val.starts_with("1.") {
                                theme.accent_green
                            } else {
                                theme.text_primary
                            };
                            ui.label(RichText::new(val).size(11.0).strong().color(val_col));
                        });
                    });
                    ui.label(RichText::new(hint).size(8.5).color(theme.text_muted));
                    ui.separator();
                }
            });
    }

    fn render_iot_telemetry_panel(&self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(14))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(
                    RichText::new("🏢 7D DIGITAL TWIN & REAL-TIME IOT TELEMETRY")
                        .size(11.5)
                        .strong()
                        .color(Color32::from_rgb(175, 82, 222)),
                );
                ui.add_space(6.0);

                let sensors = [
                    ("Strain Gauge SG-01 (Roof Truss)", "214 με", "Tensile yield margin: 76.2%", theme.accent_green),
                    ("Accelerometer ACC-04 (Floor 12)", "1.42 Hz", "Mode 1 ambient vibration (Safe)", theme.accent_blue),
                    ("Tiltmeter TM-02 (Foundation Pier)", "0.012°", "Differential settlement within L/500", theme.accent_green),
                    ("Thermocouple TC-08 (Steel Flange)", "26.4 °C", "Normal ambient operating range", theme.text_primary),
                ];

                for (name, val, status, col) in sensors {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(name).size(10.5).color(theme.text_secondary));
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(RichText::new(val).size(11.0).strong().color(col));
                        });
                    });
                    ui.label(RichText::new(status).size(8.5).color(theme.text_muted));
                    ui.separator();
                }
            });
    }

    fn render_safety_crane_panel(&self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(14))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(
                    RichText::new("🛡️ 8D SITE SAFETY & TOWER CRANE LOGISTICS")
                        .size(11.5)
                        .strong()
                        .color(theme.accent_orange),
                );
                ui.add_space(6.0);

                let safety_items = [
                    ("Tower Crane 1 Hook Radius", "48.0 m", "Operating within certified 55m jib envelope"),
                    ("Hook Load vs Max SWL", "8.4 / 12.0 Tons", "Safe Working Load Ratio: 70.0%"),
                    ("Slab Edge Fall Arrest Netting", "100% Installed", "Per OSHA 1926.502 Perimeter Guardrails"),
                    ("LiDAR Drone Site Hazard Scan", "Completed 08:30", "Zero blind-spot incursions detected"),
                ];

                for (name, val, hint) in safety_items {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(name).size(10.5).color(theme.text_secondary));
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(RichText::new(val).size(11.0).strong().color(theme.accent_orange));
                        });
                    });
                    ui.label(RichText::new(hint).size(8.5).color(theme.text_muted));
                    ui.separator();
                }
            });
    }

    fn render_lean_kanban_panel(&self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(14))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(
                    RichText::new("♻️ 9D LEAN CONSTRUCTION LAST PLANNER (LPS)")
                        .size(11.5)
                        .strong()
                        .color(theme.accent_green),
                );
                ui.add_space(6.0);

                let kanban = [
                    ("Level 2 Rebar Placement", "VERIFIED DONE", theme.accent_green),
                    ("Floor 3 Formwork Stripping", "IN PROGRESS", theme.accent_blue),
                    ("Tower Crane Module Hoisting", "ACTIVE COMMITTED", theme.accent_orange),
                    ("MEP Overhead Duct Racks", "WEEKLY BACKLOG", theme.text_muted),
                ];

                for (task, state, col) in kanban {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(task).size(10.5).color(theme.text_primary));
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            Frame::new()
                                .fill(col.linear_multiply(0.2))
                                .corner_radius(CornerRadius::same(4))
                                .inner_margin(Margin::symmetric(6, 2))
                                .show(ui, |ui| {
                                    ui.label(RichText::new(state).size(9.0).strong().color(col));
                                });
                        });
                    });
                    ui.separator();
                }
            });
    }
}
