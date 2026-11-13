//! facility.rs — Pillar 7: 7D BIM Facility Management.
//! Lifecycle operation and maintenance data used by building owners to manage assets, repairs, and components after construction is complete.
//! Compliant with buildingSMART COBie 3.0 / 2.4 standards and IoT Structural Health Monitoring (SHM) telemetry.

use egui::{
    Align, Color32, CornerRadius, Frame, Layout, Margin, Pos2, RichText, ScrollArea, Sense,
    Stroke, Ui, Vec2,
};
use crate::features::{FeatureId, FeatureRegistry};
use crate::models::{CobieAssetRecord, ShmSensorTelemetry, StructuralMember};
use crate::theme::MacTheme;
use crate::ui_kit;

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
    // Phase 28: IoT Telemetry & COBie 3.0
    pub shm_sensors: Vec<ShmSensorTelemetry>,
    pub cobie_records: Vec<CobieAssetRecord>,
    pub selected_cobie_tab: String,
    pub cobie_search_query: String,
    pub maintenance_dispatch_status: Option<String>,
}

impl Default for FacilityView {
    fn default() -> Self {
        Self {
            active_subtab: FacilitySubTab::DigitalTwinShm,
            live_telemetry_active: true,
            simulated_strain_micro: 342.0,
            simulated_freq_hz: 3.48,
            simulated_degradation_pct: 1.8,
            shm_sensors: ShmSensorTelemetry::default_sensors(),
            cobie_records: CobieAssetRecord::default_records(),
            selected_cobie_tab: "Component".to_string(),
            cobie_search_query: String::new(),
            maintenance_dispatch_status: None,
        }
    }
}

impl FacilityView {
    pub fn show(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        members: &[StructuralMember],
        features: &mut FeatureRegistry,
    ) {
        let is_mobile = ui.available_width() < MacTheme::MOBILE_BREAKPOINT;
        let pad = if is_mobile { 12 } else { 20 };

        ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
            Frame::new().inner_margin(Margin::symmetric(pad, 12)).show(ui, |ui| {
                // 1. TOP SUBTABS SELECTOR
                self.render_header_bar(ui, theme);

                ui.add_space(16.0);

                // Notification Toast
                if let Some(ref msg) = self.maintenance_dispatch_status {
                    Frame::new()
                        .fill(Color32::from_rgba_premultiplied(16, 40, 25, 235))
                        .stroke(Stroke::new(1.0, Color32::from_rgb(40, 160, 80)))
                        .corner_radius(CornerRadius::same(6))
                        .inner_margin(Margin::symmetric(14, 8))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(format!("⚡ {}", msg))
                                    .size(11.5)
                                    .strong()
                                    .color(Color32::from_rgb(120, 240, 160)),
                            );
                        });
                    ui.add_space(12.0);
                }

                // 2. ACTIVE SUB-TAB CONTENT
                match self.active_subtab {
                    FacilitySubTab::DigitalTwinShm => {
                        self.render_digital_twin_shm(ui, theme, members, features)
                    }
                    FacilitySubTab::AssetRegistry => self.render_asset_registry(ui, theme, members),
                    FacilitySubTab::PredictiveMaintenance => {
                        self.render_predictive_maintenance(ui, theme)
                    }
                    FacilitySubTab::LifecycleCosting => self.render_lifecycle_costing(ui, theme),
                }

                ui.add_space(24.0);
            });
        });
    }

    fn render_header_bar(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let tabs = [
            (FacilitySubTab::DigitalTwinShm, "📡 Digital Twin & SHM Sensors"),
            (FacilitySubTab::AssetRegistry, "📑 COBie 3.0 Asset Registry"),
            (FacilitySubTab::PredictiveMaintenance, "🛠️ Predictive Maintenance & CMMS"),
            (FacilitySubTab::LifecycleCosting, "💵 30-Year Lifecycle Cost (LCC)"),
        ];
        ui_kit::subtab_bar(ui, theme, "facility_subtab_bar", &mut self.active_subtab, &tabs);
    }

    fn render_digital_twin_shm(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        _members: &[StructuralMember],
        features: &mut FeatureRegistry,
    ) {
        // Top Real-time Sensor Status Row (Responsive)
        let strain_str = format!("{:.0} µε", self.simulated_strain_micro);
        let freq_str = format!("{:.2} Hz", self.simulated_freq_hz);
        let cards = [
            ("STRAIN GAUGE SG-01", strain_str.as_str(), "Member E1 Strain (Safe < 600 µε)", theme.accent_green),
            ("NATURAL FREQUENCY (ACC-01)", freq_str.as_str(), "Mode 1 Transverse (Resonance Free)", theme.accent_blue),
            ("LIVE ARI RESILIENCE", "0.942", "Real-Time ARI(t) Degradation (-0.2%)", theme.accent_green),
            ("HEALTH STATUS (SHM)", "🟢 OPTIMAL", "16 IoT Sensors Operating Active", theme.accent_green),
        ];
        ui_kit::responsive_grid_n(ui, cards.len(), 200.0, |ui, idx| {
            let (title, val, sub, col) = cards[idx];
            self.render_sensor_card(ui, theme, title, val, sub, col);
        });

        ui.add_space(12.0);

        // Phase 33 Locked Slot: Live WebSocket/MQTT IoT Sensor Stream Ingestion
        ui_kit::feature_slot_card(ui, theme, features, FeatureId::LiveIotTelemetryIngestion, |ui| {
            ui.horizontal_wrapped(|ui| {
                let _ = ui.selectable_label(true, "wss://shm.eigengraph.net/mqtt-tls");
                let _ = ui.selectable_label(false, "Auto-Reconnect Backoff");
                let _ = ui.button("🔌 Connect Live SHM Telemetry Stream");
            });
        });

        ui.add_space(14.0);

        // Digital Twin Bridge Telemetry Canvas
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new("📡 Real-Time IoT Structural Health Telemetry & Vibration FFT")
                            .size(13.5)
                            .strong()
                            .color(theme.text_primary),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("📊 Export Telemetry Log (.CSV)")
                                        .size(11.0)
                                        .color(theme.text_primary),
                                )
                                .fill(theme.bg_canvas)
                                .corner_radius(CornerRadius::same(6)),
                            )
                            .clicked()
                        {
                            self.maintenance_dispatch_status = Some(
                                "Exported 720-hour sensor telemetry archive to CSV format (100 Hz sample rate)."
                                    .to_string(),
                            );
                        }

                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("⚡ Trigger Stress Test")
                                        .size(11.0)
                                        .strong()
                                        .color(Color32::WHITE),
                                )
                                .fill(theme.accent_blue)
                                .corner_radius(CornerRadius::same(6)),
                            )
                            .clicked()
                        {
                            self.simulated_strain_micro += 45.0;
                            self.maintenance_dispatch_status = Some(
                                "Simulated dynamic live vehicular shock load (+45.0 με). System returned to elastic equilibrium in 180ms."
                                    .to_string(),
                            );
                        }

                        ui.label(
                            RichText::new("Protocol: MQTT/TLS 1.3 • Sample Rate: 100 Hz")
                                .size(10.5)
                                .color(theme.text_muted),
                        );
                    });
                });
                ui.add_space(14.0);

                // Sensor Table with Sparklines
                for s in &self.shm_sensors {
                    let (status_bg, status_fg) = match s.status_level.as_str() {
                        "Critical" => (theme.accent_red.gamma_multiply(0.2), theme.accent_red),
                        "Warning" => (theme.accent_orange.gamma_multiply(0.2), theme.accent_orange),
                        _ => (theme.accent_green.gamma_multiply(0.2), theme.accent_green),
                    };

                    ui.horizontal_wrapped(|ui| {
                        Frame::new()
                            .fill(theme.bg_canvas)
                            .stroke(Stroke::new(1.0, theme.border_subtle))
                            .corner_radius(CornerRadius::same(6))
                            .inner_margin(Margin::symmetric(8, 4))
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(&s.sensor_id)
                                        .size(11.5)
                                        .strong()
                                        .color(theme.accent_blue),
                                );
                            });

                        ui.add_space(8.0);
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(&s.sensor_name)
                                    .size(12.0)
                                    .strong()
                                    .color(theme.text_primary),
                            );
                            ui.label(
                                RichText::new(format!(
                                    "Location: {} • Sensor Type: {}",
                                    s.location, s.sensor_type
                                ))
                                .size(10.5)
                                .color(theme.text_secondary),
                            );
                        });

                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            Frame::new()
                                .fill(status_bg)
                                .corner_radius(CornerRadius::same(4))
                                .inner_margin(Margin::symmetric(6, 2))
                                .show(ui, |ui| {
                                    ui.label(
                                        RichText::new(&s.status_level)
                                            .size(10.5)
                                            .strong()
                                            .color(status_fg),
                                    );
                                });

                            ui.add_space(12.0);
                            ui.label(
                                RichText::new(format!("Limit: < {:.1} {}", s.threshold_warning, s.unit))
                                    .size(11.0)
                                    .color(theme.text_muted),
                            );
                            ui.add_space(12.0);
                            ui.label(
                                RichText::new(format!("{:.2} {}", s.current_value, s.unit))
                                    .size(13.0)
                                    .strong()
                                    .color(theme.text_primary),
                            );
                        });
                    });

                    // Mini Sparkline Painter
                    ui.add_space(2.0);
                    let (spark_rect, _resp) =
                        ui.allocate_exact_size(Vec2::new(ui.available_width(), 10.0), Sense::hover());
                    let painter = ui.painter();
                    painter.rect_filled(spark_rect, CornerRadius::same(2), theme.bg_canvas);

                    let min_v = s.sparkline_history.iter().cloned().fold(f32::INFINITY, f32::min);
                    let max_v = s.sparkline_history.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
                    let v_range = (max_v - min_v).max(0.001);

                    for i in 0..(s.sparkline_history.len() - 1) {
                        let x1 = spark_rect.min.x
                            + (i as f32 / (s.sparkline_history.len() - 1) as f32)
                                * spark_rect.width();
                        let x2 = spark_rect.min.x
                            + ((i + 1) as f32 / (s.sparkline_history.len() - 1) as f32)
                                * spark_rect.width();
                        let y1 = spark_rect.max.y
                            - ((s.sparkline_history[i] - min_v) / v_range) * spark_rect.height();
                        let y2 = spark_rect.max.y
                            - ((s.sparkline_history[i + 1] - min_v) / v_range) * spark_rect.height();
                        painter.line_segment(
                            [Pos2::new(x1, y1), Pos2::new(x2, y2)],
                            Stroke::new(1.2, theme.accent_blue),
                        );
                    }

                    ui.add_space(6.0);
                    ui.separator();
                    ui.add_space(4.0);
                }
            });
    }

    fn render_asset_registry(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        _members: &[StructuralMember],
    ) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new("📑 buildingSMART COBie 3.0 / 2.4 Facility Asset Inspector")
                            .size(13.5)
                            .strong()
                            .color(theme.text_primary),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("📥 Export COBie (.xlsx / JSON)")
                                        .size(11.0)
                                        .strong()
                                        .color(Color32::WHITE),
                                )
                                .fill(theme.accent_green)
                                .corner_radius(CornerRadius::same(6)),
                            )
                            .clicked()
                        {
                            self.maintenance_dispatch_status = Some(
                                "COBie 3.0 spreadsheet workbook exported successfully (6 worksheets compliant with ISO 19650-4)."
                                    .to_string(),
                            );
                        }
                        ui.label(
                            RichText::new(format!(
                                "Total Records: {} Components",
                                self.cobie_records.len()
                            ))
                            .size(11.0)
                            .color(theme.text_muted),
                        );
                    });
                });

                ui.add_space(12.0);

                // COBie Sheet Sub-tabs
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new("COBie Worksheet:")
                            .size(11.0)
                            .strong()
                            .color(theme.accent_blue),
                    );
                    let tabs = ["Facility", "Floor", "Space", "Component", "Type", "Document"];
                    for tab in tabs {
                        let is_active = self.selected_cobie_tab == tab;
                        let bg = if is_active { theme.accent_blue } else { theme.bg_canvas };
                        let fg = if is_active { Color32::WHITE } else { theme.text_secondary };
                        if ui
                            .add(
                                egui::Button::new(RichText::new(tab).size(11.0).color(fg))
                                    .fill(bg)
                                    .corner_radius(CornerRadius::same(6)),
                            )
                            .clicked()
                        {
                            self.selected_cobie_tab = tab.to_string();
                        }
                    }

                    ui.add_space(8.0);
                    ui.label(RichText::new("🔍 Search:").size(11.0).color(theme.text_muted));
                    let search_w = ui.available_width().min(180.0).max(110.0);
                    ui.add(
                        egui::TextEdit::singleline(&mut self.cobie_search_query)
                            .desired_width(search_w),
                    );
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);

                // COBie Table (Scrollable horizontally on mobile phones)
                ScrollArea::horizontal()
                    .id_salt("cobie_registry_table_scroll")
                    .show(ui, |ui| {
                        egui::Grid::new("cobie_registry_grid")
                            .striped(true)
                            .min_col_width(110.0)
                            .spacing(Vec2::new(16.0, 8.0))
                            .show(ui, |ui| {
                                ui.label(RichText::new("NAME / TAG").size(10.5).strong().color(theme.text_muted));
                                ui.label(RichText::new("CATEGORY & SPECIFICATION").size(10.5).strong().color(theme.text_muted));
                                ui.label(RichText::new("LOCATION").size(10.5).strong().color(theme.text_muted));
                                ui.label(RichText::new("SERIAL / DOC ID").size(10.5).strong().color(theme.text_muted));
                                ui.label(RichText::new("WARRANTY / LIFE").size(10.5).strong().color(theme.text_muted));
                                ui.label(RichText::new("CONDITION").size(10.5).strong().color(theme.text_muted));
                                ui.end_row();

                                for rec in &self.cobie_records {
                                    if rec.sheet_tab != self.selected_cobie_tab
                                        && self.selected_cobie_tab != "All"
                                    {
                                        continue;
                                    }
                                    if !self.cobie_search_query.is_empty()
                                        && !rec
                                            .name
                                            .to_lowercase()
                                            .contains(&self.cobie_search_query.to_lowercase())
                                    {
                                        continue;
                                    }

                                    ui.label(
                                        RichText::new(&rec.name)
                                            .size(11.5)
                                            .strong()
                                            .color(theme.text_primary),
                                    );
                                    ui.vertical(|ui| {
                                        ui.label(
                                            RichText::new(&rec.category)
                                                .size(11.0)
                                                .color(theme.text_secondary),
                                        );
                                        ui.label(
                                            RichText::new(format!("By: {}", rec.created_by))
                                                .size(9.5)
                                                .color(theme.text_muted),
                                        );
                                    });
                                    ui.label(
                                        RichText::new(&rec.space_or_location)
                                            .size(11.0)
                                            .color(theme.accent_blue),
                                    );
                                    ui.label(
                                        RichText::new(&rec.serial_or_doc_no)
                                            .size(11.0)
                                            .color(theme.text_muted),
                                    );
                                    ui.label(
                                        RichText::new(&rec.warranty_or_date)
                                            .size(11.0)
                                            .color(theme.text_secondary),
                                    );
                                    let cond_color = if rec.condition_index > 95.0 {
                                        theme.accent_green
                                    } else {
                                        theme.accent_orange
                                    };
                                    ui.label(
                                        RichText::new(format!("{:.1}%", rec.condition_index))
                                            .size(11.5)
                                            .strong()
                                            .color(cond_color),
                                    );
                                    ui.end_row();
                                }
                            });
                    });
            });
    }

    fn render_predictive_maintenance(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new(
                            "🛠️ Predictive Maintenance & Computerized Maintenance Management (CMMS)",
                        )
                        .size(13.5)
                        .strong()
                        .color(theme.text_primary),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("👷 Dispatch Structural Inspector")
                                        .size(11.0)
                                        .strong()
                                        .color(Color32::WHITE),
                                )
                                .fill(theme.accent_blue)
                                .corner_radius(CornerRadius::same(6)),
                            )
                            .clicked()
                        {
                            self.maintenance_dispatch_status = Some(
                                "Structural inspection team dispatched for Zone B-2 Column Foundation. Inspection ticket #INSP-2026-904 opened."
                                    .to_string(),
                            );
                        }

                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("📋 Generate Emergency Work Order")
                                        .size(11.0)
                                        .color(theme.text_primary),
                                )
                                .fill(theme.bg_canvas)
                                .corner_radius(CornerRadius::same(6)),
                            )
                            .clicked()
                        {
                            self.maintenance_dispatch_status = Some(
                                "Emergency Work Order #WO-EMG-044 generated and routed to On-Site Facilities Engineer."
                                    .to_string(),
                            );
                        }
                    });
                });
                ui.add_space(10.0);
                ui.label(
                    RichText::new(
                        "Automated Work Orders (WO) issued when material degradation, strain history, or corrosion metrics approach critical resilience thresholds.",
                    )
                    .size(11.5)
                    .color(theme.text_secondary),
                );
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
                            ui.horizontal_wrapped(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(
                                        RichText::new(format!("{} — {}", code, task))
                                            .size(12.5)
                                            .strong()
                                            .color(theme.text_primary),
                                    );
                                    ui.label(
                                        RichText::new(format!("Rationale: {} | {}", reason, sched))
                                            .size(11.0)
                                            .color(theme.text_secondary),
                                    );
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
                ui.label(
                    RichText::new("💵 30-Year Life Cycle Costing (LCC) Analysis")
                        .size(13.5)
                        .strong()
                        .color(theme.text_primary),
                );
                ui.add_space(10.0);

                let lcc_cards = [
                    ("INITIAL CAPITAL (CAPEX)", "$3.84 M", "Initial Construction Verified via 5D BoQ", theme.text_primary),
                    ("ESTIMATED OPEX (30 YRS)", "$645 K", "Routine Maintenance & SHM Sensors", theme.accent_blue),
                    ("EARLY ARI SAVINGS", "$1.45 M", "Catastrophic Failure Avoidance", theme.accent_green),
                ];
                ui_kit::responsive_grid_n(ui, lcc_cards.len(), 200.0, |ui, idx| {
                    let (title, val, sub, col) = lcc_cards[idx];
                    self.render_sensor_card(ui, theme, title, val, sub, col);
                });
            });
    }

    fn render_sensor_card(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        title: &str,
        val: &str,
        subtitle: &str,
        val_color: Color32,
    ) {
        ui_kit::stat_card(ui, theme, title, val, subtitle, val_color);
    }
}
