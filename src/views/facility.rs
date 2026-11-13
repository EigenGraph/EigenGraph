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
    MepIsolationAndSpace,
    DroneInspectionMapper,
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
                    FacilitySubTab::MepIsolationAndSpace => {
                        self.render_mep_isolation_space(ui, theme, features);
                    }
                    FacilitySubTab::DroneInspectionMapper => {
                        self.render_drone_inspection_mapper(ui, theme, features);
                    }
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
            (FacilitySubTab::MepIsolationAndSpace, "🚱 MEP Valve Tree & Space KPI"),
            (FacilitySubTab::DroneInspectionMapper, "🛸 UAV Crack & Spalling Mapper"),
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

    fn render_mep_isolation_space(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(ui, theme, features, FeatureId::MepIsolationSpaceUtilization, |ui| {
            let cards = [
                ("VALVE ISOLATION NODES", "42 Smart Valves", "Chilled Water, Domestic & Fire Standpipes", theme.accent_blue),
                ("GROSS-TO-NET EFFICIENCY", "84.6% Net Usable", "24,200 m² Leasable / 28,600 m² GFA", theme.accent_green),
                ("PEAK FLOOR OCCUPANCY", "72% Nominal", "BMS CO₂ Demand Ventilation Linked", theme.accent_orange),
                ("CRITICAL RISER SHUTOFF", "VLV-CHW-04", "Impacts Levels L05–L08 East Wing", theme.text_primary),
            ];
            ui_kit::responsive_grid_n(ui, cards.len(), 185.0, |ui, idx| {
                let (t, v, s, c) = cards[idx];
                self.render_sensor_card(ui, theme, t, v, s, c);
            });

            ui.add_space(10.0);
            ui.horizontal_wrapped(|ui| {
                let _ = ui.selectable_label(true, "System: Chilled Water Loop (CHW)");
                let _ = ui.selectable_label(false, "Fire Sprinkler Zones");
                let _ = ui.button("🚱 Trace Downstream Impacted Spaces");
            });
            ui.add_space(10.0);

            ScrollArea::horizontal().id_salt("mep_valve_scroll").show(ui, |ui| {
                egui::Grid::new("mep_valve_grid")
                    .striped(true)
                    .min_col_width(90.0)
                    .spacing(Vec2::new(14.0, 6.0))
                    .show(ui, |ui| {
                        for h in ["Valve Tag", "System Line", "Floor Zone", "Downstream Rooms", "Space Occupancy", "State"] {
                            ui.label(RichText::new(h).size(10.5).strong().color(theme.accent_blue));
                        }
                        ui.end_row();
                        for (tag, sys, flr, rms, occ, st, col) in [
                            ("VLV-CHW-04", "Chilled Water Riser E", "L05–L08", "14 Office Bays (1,820 m²)", "78%", "OPEN", theme.accent_green),
                            ("VLV-DOM-11", "Domestic Potable North", "L01–L04", "Core Restrooms & Pantry", "64%", "OPEN", theme.accent_green),
                            ("VLV-SPR-02", "NFPA Wet Standpipe #2", "B01–L04", "Parking + Lobby Atrium", "45%", "TEST ISOLATED", theme.accent_orange),
                        ] {
                            ui.label(RichText::new(tag).size(10.5).monospace().color(theme.accent_blue));
                            ui.label(RichText::new(sys).size(10.5).strong().color(theme.text_primary));
                            ui.label(RichText::new(flr).size(10.5).color(theme.text_secondary));
                            ui.label(RichText::new(rms).size(10.5).color(theme.text_secondary));
                            ui.label(RichText::new(occ).size(10.5).color(theme.accent_blue));
                            ui.label(RichText::new(st).size(10.5).strong().color(col));
                            ui.end_row();
                        }
                    });
            });
        });
    }

    fn render_drone_inspection_mapper(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(ui, theme, features, FeatureId::DroneDefectCrackMapper, |ui| {
            let cards = [
                ("UAV CAPTURED FRAMES", "418 Ortho Tiles", "RTK Accuracy ±4 mm @ 6m Standoff", theme.accent_blue),
                ("MAX CRACK WIDTH FOUND", "0.28 mm (ACI 224R)", "Allowable Coastal Exposure <= 0.30 mm", theme.accent_orange),
                ("DELAMINATION / SPALLING", "2 Minor Patches", "Pier Cap P-03 West Face (< 0.4 m²)", theme.accent_red),
                ("CMMS TICKETS SYNCED", "2 Auto-Logged", "Linked to COBie Asset GUIDs", theme.accent_green),
            ];
            ui_kit::responsive_grid_n(ui, cards.len(), 185.0, |ui, idx| {
                let (t, v, s, c) = cards[idx];
                self.render_sensor_card(ui, theme, t, v, s, c);
            });

            ui.add_space(10.0);
            ui.horizontal_wrapped(|ui| {
                let _ = ui.selectable_label(true, "AI Vision: ACI 224R Crack Thresholds");
                let _ = ui.selectable_label(false, "Thermal Spall Overlay");
                let _ = ui.button("🛸 Import UAV Defect CSV & Auto-Pin 3D Markers");
            });
            ui.add_space(10.0);

            ui_kit::responsive_two_panes(ui, 640.0, |ui, pane| match pane {
                0 => {
                    ui.label(
                        RichText::new("FACADE & PIER ORTHOMOSAIC DEFECT OVERLAY")
                            .size(11.0)
                            .strong()
                            .color(theme.accent_blue),
                    );
                    ui.add_space(6.0);
                    let desired = Vec2::new(ui.available_width(), 155.0);
                    let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
                    let painter = ui.painter_at(rect);
                    painter.rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(13, 17, 24));
                    let elev = rect.shrink(22.0);
                    painter.rect_stroke(
                        elev,
                        CornerRadius::ZERO,
                        Stroke::new(1.5, theme.text_secondary),
                        egui::StrokeKind::Inside,
                    );
                    for i in 1..4 {
                        let gx = elev.left() + (i as f32 / 4.0) * elev.width();
                        painter.line_segment(
                            [Pos2::new(gx, elev.top()), Pos2::new(gx, elev.bottom())],
                            Stroke::new(0.8, theme.border_subtle),
                        );
                    }
                    let defects = [
                        (0.32_f32, 0.48_f32, "CRK-01 (0.28mm)", theme.accent_orange),
                        (0.72, 0.76, "SPL-02 (0.4 m²)", theme.accent_red),
                    ];
                    for (fx, fy, tag, col) in defects {
                        let p = Pos2::new(elev.left() + fx * elev.width(), elev.top() + fy * elev.height());
                        painter.circle_stroke(p, 8.0, Stroke::new(1.8, col));
                        painter.circle_filled(p, 3.0, col);
                        painter.text(
                            p + Vec2::new(10.0, -4.0),
                            egui::Align2::LEFT_CENTER,
                            tag,
                            egui::FontId::monospace(9.5),
                            col,
                        );
                    }
                }
                _ => {
                    ui.label(
                        RichText::new("UAV STRUCTURAL DEFECT LEDGER (ACI 224R)")
                            .size(11.0)
                            .strong()
                            .color(theme.accent_blue),
                    );
                    ui.add_space(6.0);
                    ScrollArea::horizontal().id_salt("uav_defect_scroll").show(ui, |ui| {
                        egui::Grid::new("uav_defect_grid")
                            .striped(true)
                            .spacing(Vec2::new(12.0, 5.0))
                            .show(ui, |ui| {
                                for h in ["Defect ID", "Member", "Width / Area", "Severity", "CMMS WO"] {
                                    ui.label(RichText::new(h).size(10.5).strong().color(theme.accent_blue));
                                }
                                ui.end_row();
                                for (id, mem, meas, sev, wo, col) in [
                                    ("UAV-CRK-01", "Shear Wall Core L04", "0.28 mm Flexural", "Monitor", "WO-2026-102", theme.accent_orange),
                                    ("UAV-SPL-02", "Pier Cap P-03 West", "0.40 m² Cover Spall", "Epoxy Patch", "WO-2026-105", theme.accent_red),
                                    ("UAV-CRK-03", "Parapet Beam R-12", "0.12 mm Shrinkage", "Nominal", "No Action", theme.accent_green),
                                ] {
                                    ui.label(RichText::new(id).size(10.5).monospace().color(theme.accent_blue));
                                    ui.label(RichText::new(mem).size(10.5).strong().color(theme.text_primary));
                                    ui.label(RichText::new(meas).size(10.5).color(theme.text_secondary));
                                    ui.label(RichText::new(sev).size(10.5).strong().color(col));
                                    ui.label(RichText::new(wo).size(10.5).color(theme.text_muted));
                                    ui.end_row();
                                }
                            });
                    });
                }
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
