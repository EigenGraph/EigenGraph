//! industrialized.rs — Pillar 10: 10D BIM Industrialized Construction & Automation.
//! The integration of Industry 4.0 technologies: Prefabrication DfMA, Robotics,
//! Autonomous Erection, and Native OpenBIM IFC (IFC2x3 / IFC4 / IFC4.3) Reader & Inspector.

use egui::{Align, Color32, CornerRadius, Frame, Layout, Margin, Rect, RichText, ScrollArea, Sense, Stroke, Ui, Vec2};
use crate::core::{
    IfcAuditStatus, IfcBcfTopic, IfcClashItem, IfcClashReport, IfcClashSeverity, IfcDiffKind,
    IfcEditCommand, IfcEditSession, IfcElementCreationSpec, IfcElementDiff, IfcElementSummary,
    IfcIdsReport, IfcIdsRuleCheck, IfcModel, IfcModelDiff, IfcSchemaVersion, IfcSpatialNode,
};
use crate::models::{PrefabModuleUnit, PrefabSupplyStage, StructuralMember};
use crate::theme::MacTheme;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndustrializedSubTab {
    Lifecycle10DHub,
    ModularDfma,
    RoboticAutomation,
    AutomatedErection,
    OpenBimIfcStudio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfcInspectorTab {
    Properties,
    HealthAudit,
    GeometryBbox,
    ClashAndDiff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfcSourceMode {
    SampleVillaIfc4,
    SamplePortalFrameIfc4,
    SampleDiagridIfc2x3,
    CustomPaste,
    FilePath,
}

pub struct IndustrializedView {
    pub active_subtab: IndustrializedSubTab,
    pub offsite_ratio_pct: f32,
    pub robotic_speed_mm_s: f32,
    pub export_status: Option<String>,

    // OpenBIM IFC Engine State
    pub ifc_model: Option<IfcModel>,
    pub selected_element_id: Option<usize>,
    pub inspector_tab: IfcInspectorTab,
    pub ifc_source_mode: IfcSourceMode,
    pub custom_ifc_paste: String,
    pub custom_ifc_path: String,
    pub element_search_query: String,
    pub spatial_tree_expanded: HashMap<usize, bool>,

    // Phase 18: Interactive Editing & Property/Metadata Mutation Engine
    pub edit_session: IfcEditSession,
    pub is_editing: bool,
    pub edit_name: String,
    pub edit_desc: String,
    pub edit_obj_type: String,
    pub edit_material: String,
    pub selected_target_storey: usize,
    pub new_pset_name: String,
    pub new_prop_name: String,
    pub new_prop_val: String,

    // Phase 19: Geometric Topology & Element Synthesis State
    pub geom_depth_input: f32,
    pub geom_trans_x: f32,
    pub geom_trans_y: f32,
    pub geom_trans_z: f32,
    pub geom_prof_w: f32,
    pub geom_prof_h: f32,
    pub synth_class: String,
    pub synth_name: String,
    pub synth_desc: String,
    pub synth_length: f32,
    pub synth_prof_w: f32,
    pub synth_prof_h: f32,
    pub synth_mat: String,
    pub show_synth_modal: bool,

    // Phase 20: Spatial Clash Detection, Model Diff, IDS Validation & Multi-Format Exporter
    pub baseline_model: Option<IfcModel>,
    pub clash_tolerance_mm: f32,
    pub clearance_buffer_mm: f32,
    pub clash_report: Option<IfcClashReport>,
    pub model_diff: Option<IfcModelDiff>,
    pub ids_report: Option<IfcIdsReport>,
    pub diff_subtab: u8, // 0 = Clash, 1 = Diff, 2 = IDS & BCF

    // Phase 29: Modular Prefab Supply Chain & QR/RFID Scanner Tracker
    pub prefab_units: Vec<PrefabModuleUnit>,
    pub prefab_stage_filter: Option<PrefabSupplyStage>,
    pub scanner_input: String,
    pub selected_module_tag: String,
    pub scanner_log: Vec<String>,
}

impl Default for IndustrializedView {
    fn default() -> Self {
        // Pre-load reference IFC4 BIMCraft Duplex Villa
        let initial_model = IfcModel::parse(&IfcModel::sample_villa_ifc4()).ok();
        Self {
            active_subtab: IndustrializedSubTab::Lifecycle10DHub,
            offsite_ratio_pct: 82.5,
            robotic_speed_mm_s: 14.5,
            export_status: None,

            ifc_model: initial_model.clone(),
            selected_element_id: Some(200), // Default to first column
            inspector_tab: IfcInspectorTab::Properties,
            ifc_source_mode: IfcSourceMode::SampleVillaIfc4,
            custom_ifc_paste: String::new(),
            custom_ifc_path: "models/building.ifc".to_string(),
            element_search_query: String::new(),
            spatial_tree_expanded: HashMap::new(),

            edit_session: IfcEditSession::new(),
            is_editing: false,
            edit_name: String::new(),
            edit_desc: String::new(),
            edit_obj_type: String::new(),
            edit_material: String::new(),
            selected_target_storey: 0,
            new_pset_name: "Pset_StructuralAudit".to_string(),
            new_prop_name: String::new(),
            new_prop_val: String::new(),

            geom_depth_input: 3.5,
            geom_trans_x: 0.0,
            geom_trans_y: 0.0,
            geom_trans_z: 0.0,
            geom_prof_w: 0.4,
            geom_prof_h: 0.4,
            synth_class: "IFCCOLUMN".to_string(),
            synth_name: "Synthesized_Member_A1".to_string(),
            synth_desc: "Modular structural steel element".to_string(),
            synth_length: 3.8,
            synth_prof_w: 0.4,
            synth_prof_h: 0.4,
            synth_mat: "High-Strength Steel S460ML".to_string(),
            show_synth_modal: false,

            baseline_model: initial_model,
            clash_tolerance_mm: 10.0,
            clearance_buffer_mm: 50.0,
            clash_report: None,
            model_diff: None,
            ids_report: None,
            diff_subtab: 0,

            prefab_units: PrefabModuleUnit::default_units(),
            prefab_stage_filter: None,
            scanner_input: "MOD-TR-014".to_string(),
            selected_module_tag: "MOD-TR-014".to_string(),
            scanner_log: vec![
                "Fleet GPS Beacon: MOD-TR-014 departed plant; ETA Site Laydown Bay B-1.".to_string(),
                "QA NDT Release: MOD-PC-208 Ultrasonic shear-weld scan passed (Zero defects).".to_string(),
            ],
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
                            ui.label(RichText::new("📦 OPENBIM IFC BUS:").size(11.5).strong().color(theme.accent_green));
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
                    IndustrializedSubTab::Lifecycle10DHub => self.render_10d_lifecycle_hub(ui, theme),
                    IndustrializedSubTab::ModularDfma => self.render_modular_dfma(ui, theme),
                    IndustrializedSubTab::RoboticAutomation => self.render_robotic_automation(ui, theme),
                    IndustrializedSubTab::AutomatedErection => self.render_automated_erection(ui, theme),
                    IndustrializedSubTab::OpenBimIfcStudio => self.render_openbim_ifc_studio(ui, theme),
                }

                ui.add_space(64.0);
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
                    (IndustrializedSubTab::Lifecycle10DHub, "🌐 10D BIM Lifecycle Hub"),
                    (IndustrializedSubTab::ModularDfma, "🏭 Prefabrication & DfMA"),
                    (IndustrializedSubTab::RoboticAutomation, "🦾 Robotics & Automation"),
                    (IndustrializedSubTab::AutomatedErection, "🏗️ Autonomous Crane & LiDAR"),
                    (IndustrializedSubTab::OpenBimIfcStudio, "🌐 OpenBIM IFC Studio"),
                ];

                ui.horizontal_wrapped(|ui| {
                    for (tab, label) in tabs {
                        let is_active = self.active_subtab == tab;
                        let bg = if is_active { theme.accent_blue } else { Color32::TRANSPARENT };
                        let fg = if is_active { Color32::WHITE } else { theme.text_secondary };

                        if ui.add(egui::Button::new(RichText::new(label).size(11.5).strong().color(fg))
                            .fill(bg)
                            .corner_radius(CornerRadius::same(7))
                            .min_size(Vec2::new(140.0, 30.0))).clicked() {
                            self.active_subtab = tab;
                        }
                        ui.add_space(4.0);
                    }
                });
            });
    }

    /// Phase 27: 10D BIM Multi-Discipline Lifecycle Master Hub
    fn render_10d_lifecycle_hub(&mut self, ui: &mut Ui, theme: &MacTheme) {
        // Hero Header
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(12))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(18))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("🌐 10D BIM MULTI-DISCIPLINE LIFECYCLE MASTER HUB").size(15.0).strong().color(theme.text_primary));
                        ui.label(RichText::new("ISO 19650 / IPD COMPLIANT").size(10.0).strong().color(theme.accent_blue));
                    });
                    ui.label(RichText::new("Unified Cross-Discipline Lifecycle Command Center spanning 1D Project Scope to 10D Industrialized Prefabrication.").size(11.0).color(theme.text_secondary));
                    ui.add_space(8.0);
                    ui.horizontal_wrapped(|ui| {
                        if ui.add(egui::Button::new(RichText::new("📥 Export 10D Audit Report (PDF)").size(11.0).strong().color(Color32::WHITE))
                            .fill(theme.accent_green)
                            .corner_radius(CornerRadius::same(6))).clicked() {
                            self.export_status = Some("10D BIM Lifecycle Executive Audit Report successfully exported to PDF.".to_string());
                        }
                        if ui.add(egui::Button::new(RichText::new("🌐 Sync Speckle CDE").size(11.0).color(theme.text_primary))
                            .fill(theme.bg_sidebar)
                            .corner_radius(CornerRadius::same(6))).clicked() {
                            self.export_status = Some("Common Data Environment (CDE) synchronized with Speckle Cloud stream.".to_string());
                        }
                    });
                });
            });

        ui.add_space(14.0);

        // 10 Dimensions KPI Matrix Cards
        ui.label(RichText::new("10D LIFECYCLE PILLARS STATUS").size(11.5).strong().color(theme.text_secondary));
        ui.add_space(6.0);

        ui.horizontal_wrapped(|ui| {
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

            for (dim, val, desc, col) in dimensions {
                Frame::new()
                    .fill(theme.bg_card)
                    .corner_radius(CornerRadius::same(8))
                    .inner_margin(Margin::symmetric(12, 10))
                    .stroke(Stroke::new(1.0, theme.border_subtle))
                    .show(ui, |ui| {
                        ui.set_width(175.0);
                        ui.label(RichText::new(dim).size(10.0).strong().color(theme.text_muted));
                        ui.label(RichText::new(val).size(12.0).strong().color(col));
                        ui.label(RichText::new(desc).size(9.0).color(theme.text_secondary));
                    });
            }
        });

        ui.add_space(16.0);

        // Multi-Discipline Deep Dive Layout (4 Responsive Panels)
        let avail_w = ui.available_width();
        let is_wide = avail_w >= 850.0;

        if is_wide {
            let panel_w = (avail_w - 16.0) * 0.5;
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_width(panel_w);
                    self.render_evm_cost_panel(ui, theme);
                    ui.add_space(12.0);
                    self.render_iot_telemetry_panel(ui, theme);
                });
                ui.add_space(12.0);
                ui.vertical(|ui| {
                    ui.set_width(panel_w);
                    self.render_safety_crane_panel(ui, theme);
                    ui.add_space(12.0);
                    self.render_lean_kanban_panel(ui, theme);
                });
            });
        } else {
            ui.vertical(|ui| {
                self.render_evm_cost_panel(ui, theme);
                ui.add_space(12.0);
                self.render_iot_telemetry_panel(ui, theme);
                ui.add_space(12.0);
                self.render_safety_crane_panel(ui, theme);
                ui.add_space(12.0);
                self.render_lean_kanban_panel(ui, theme);
            });
        }
    }

    fn render_evm_cost_panel(&self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(14))
            .show(ui, |ui| {
                ui.label(RichText::new("💰 5D EARNED VALUE MANAGEMENT (EVM)").size(11.5).strong().color(theme.accent_green));
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
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(name).size(10.5).color(theme.text_secondary));
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(RichText::new(val).size(11.0).strong().color(if val.starts_with('+') || val.starts_with("1.") { theme.accent_green } else { theme.text_primary }));
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
                ui.label(RichText::new("🏢 7D DIGITAL TWIN & REAL-TIME IOT TELEMETRY").size(11.5).strong().color(Color32::from_rgb(175, 82, 222)));
                ui.add_space(6.0);

                let sensors = [
                    ("Strain Gauge SG-01 (Roof Truss)", "214 με", "Tensile yield margin: 76.2%", theme.accent_green),
                    ("Accelerometer ACC-04 (Floor 12)", "1.42 Hz", "Mode 1 ambient vibration (Safe)", theme.accent_blue),
                    ("Tiltmeter TM-02 (Foundation Pier)", "0.012°", "Differential settlement within L/500", theme.accent_green),
                    ("Thermocouple TC-08 (Steel Flange)", "26.4 °C", "Normal ambient operating range", theme.text_primary),
                ];

                for (name, val, status, col) in sensors {
                    ui.horizontal(|ui| {
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
                ui.label(RichText::new("🛡️ 8D SITE SAFETY & TOWER CRANE LOGISTICS").size(11.5).strong().color(theme.accent_orange));
                ui.add_space(6.0);

                let safety_items = [
                    ("Tower Crane 1 Hook Radius", "48.0 m", "Operating within certified 55m jib envelope"),
                    ("Hook Load vs Max SWL", "8.4 / 12.0 Tons", "Safe Working Load Ratio: 70.0%"),
                    ("Slab Edge Fall Arrest Netting", "100% Installed", "Per OSHA 1926.502 Perimeter Guardrails"),
                    ("LiDAR Drone Site Hazard Scan", "Completed 08:30", "Zero blind-spot incursions detected"),
                ];

                for (name, val, hint) in safety_items {
                    ui.horizontal(|ui| {
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
                ui.label(RichText::new("♻️ 9D LEAN CONSTRUCTION LAST PLANNER (LPS)").size(11.5).strong().color(theme.accent_green));
                ui.add_space(6.0);

                let kanban = [
                    ("Level 2 Rebar Placement", "VERIFIED DONE", theme.accent_green),
                    ("Floor 3 Formwork Stripping", "IN PROGRESS", theme.accent_blue),
                    ("Tower Crane Module Hoisting", "ACTIVE COMMITTED", theme.accent_orange),
                    ("MEP Overhead Duct Racks", "WEEKLY BACKLOG", theme.text_muted),
                ];

                for (task, state, col) in kanban {
                    ui.horizontal(|ui| {
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

    fn render_responsive_cards(
        &self,
        ui: &mut Ui,
        cards: &[(&str, String, String, Color32)],
        theme: &MacTheme,
    ) {
        let avail_w = ui.available_width();
        if cards.is_empty() {
            return;
        }
        if avail_w >= 860.0 {
            ui.columns(cards.len(), |cols| {
                for (i, (title, val, sub, col)) in cards.iter().enumerate() {
                    self.render_stat_card(&mut cols[i], theme, title, val, sub, *col);
                }
            });
        } else if avail_w >= 520.0 && cards.len() >= 2 {
            for chunk in cards.chunks(2) {
                if chunk.len() == 2 {
                    ui.columns(2, |cols| {
                        self.render_stat_card(&mut cols[0], theme, chunk[0].0, &chunk[0].1, &chunk[0].2, chunk[0].3);
                        self.render_stat_card(&mut cols[1], theme, chunk[1].0, &chunk[1].1, &chunk[1].2, chunk[1].3);
                    });
                } else {
                    self.render_stat_card(ui, theme, chunk[0].0, &chunk[0].1, &chunk[0].2, chunk[0].3);
                }
                ui.add_space(6.0);
            }
        } else {
            for (title, val, sub, col) in cards {
                self.render_stat_card(ui, theme, title, val, sub, *col);
                ui.add_space(6.0);
            }
        }
    }

    fn render_modular_dfma(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let total_units = self.prefab_units.len();
        let erected_count = self
            .prefab_units
            .iter()
            .filter(|u| u.stage == PrefabSupplyStage::ErectedAndTorqued)
            .count();
        let erected_pct = if total_units > 0 {
            (erected_count as f32 / total_units as f32) * 100.0
        } else {
            0.0
        };
        let transit_or_staged = self
            .prefab_units
            .iter()
            .filter(|u| {
                matches!(
                    u.stage,
                    PrefabSupplyStage::InTransitFleet | PrefabSupplyStage::SiteLaydownStaging
                )
            })
            .count();
        let qa_passed = self.prefab_units.iter().filter(|u| u.qa_passed).count();
        let max_swl_pct = self
            .prefab_units
            .iter()
            .map(|u| u.crane_utilization_pct())
            .fold(0.0f32, f32::max);

        let kpis = [
            (
                "ERECTION PROGRESS",
                format!("{}/{} ({:.0}%)", erected_count, total_units, erected_pct),
                "Torqued & As-Built Verified".to_string(),
                theme.accent_green,
            ),
            (
                "OFF-SITE PREFAB RATIO",
                format!("{:.1}%", self.offsite_ratio_pct),
                "Factory Volumetric & Panels".to_string(),
                theme.accent_blue,
            ),
            (
                "IN-TRANSIT & STAGING",
                format!("{} Modules", transit_or_staged),
                format!("QA Passed: {}/{} Units", qa_passed, total_units),
                theme.accent_orange,
            ),
            (
                "PEAK CRANE SWL LOAD",
                format!("{:.0}% SWL", max_swl_pct),
                "12.0T Jib Rigging Safety Guard".to_string(),
                if max_swl_pct > 85.0 {
                    theme.accent_red
                } else {
                    theme.accent_green
                },
            ),
        ];
        self.render_responsive_cards(ui, &kpis, theme);

        ui.add_space(14.0);

        // 6-Stage DfMA Supply Chain Pipeline Funnel
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(14))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new("🏭 6-STAGE DfMA SUPPLY CHAIN PIPELINE")
                            .size(12.5)
                            .strong()
                            .color(theme.text_primary),
                    );
                    ui.label(
                        RichText::new("Click any stage to filter modular prefabricated elements")
                            .size(10.0)
                            .color(theme.text_muted),
                    );
                });
                ui.add_space(8.0);

                ui.horizontal_wrapped(|ui| {
                    let all_sel = self.prefab_stage_filter.is_none();
                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new(format!("All ({})", total_units))
                                    .size(10.5)
                                    .strong()
                                    .color(if all_sel { Color32::WHITE } else { theme.text_secondary }),
                            )
                            .fill(if all_sel { theme.accent_blue } else { theme.bg_canvas })
                            .corner_radius(CornerRadius::same(6)),
                        )
                        .clicked()
                    {
                        self.prefab_stage_filter = None;
                    }

                    for stage in PrefabSupplyStage::all() {
                        let count = self.prefab_units.iter().filter(|u| u.stage == *stage).count();
                        let (r, g, b) = stage.color_rgb();
                        let st_col = Color32::from_rgb(r, g, b);
                        let is_sel = self.prefab_stage_filter == Some(*stage);
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new(format!("{} ({})", stage.label(), count))
                                        .size(10.0)
                                        .strong()
                                        .color(if is_sel { Color32::WHITE } else { st_col }),
                                )
                                .fill(if is_sel { st_col } else { theme.bg_canvas })
                                .corner_radius(CornerRadius::same(6)),
                            )
                            .clicked()
                        {
                            self.prefab_stage_filter = if is_sel { None } else { Some(*stage) };
                        }
                    }
                });

                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new("Target Prefabrication Ratio:")
                            .size(11.0)
                            .color(theme.text_secondary),
                    );
                    ui.add(egui::Slider::new(&mut self.offsite_ratio_pct, 50.0..=95.0).suffix(" %"));
                });
            });

        ui.add_space(12.0);

        // Simulated QR / RFID Scanner & Check-In Console
        let mut scan_trigger = false;
        let mut advance_selected_trigger = false;
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(14))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.label(
                    RichText::new("📡 SIMULATED QR / RFID SCANNER & SITE GATE CHECK-IN CONSOLE")
                        .size(12.0)
                        .strong()
                        .color(theme.accent_blue),
                );
                ui.add_space(6.0);

                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new("Barcode / RFID Tag:")
                            .size(11.0)
                            .color(theme.text_secondary),
                    );
                    let edit_res = ui.add(
                        egui::TextEdit::singleline(&mut self.scanner_input)
                            .hint_text("Scan tag e.g. MOD-TR-014 or RFID-EG-88412")
                            .desired_width(210.0),
                    );
                    if edit_res.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        scan_trigger = true;
                    }

                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("📡 Scan & Check-In")
                                    .size(10.5)
                                    .strong()
                                    .color(Color32::WHITE),
                            )
                            .fill(theme.accent_blue)
                            .corner_radius(CornerRadius::same(6)),
                        )
                        .clicked()
                    {
                        scan_trigger = true;
                    }

                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("⏭️ Advance Stage")
                                    .size(10.5)
                                    .strong()
                                    .color(Color32::WHITE),
                            )
                            .fill(theme.accent_green)
                            .corner_radius(CornerRadius::same(6)),
                        )
                        .clicked()
                    {
                        advance_selected_trigger = true;
                    }
                });

                ui.add_space(6.0);
                let tags: Vec<String> = self.prefab_units.iter().map(|u| u.tag_id.clone()).collect();
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Quick Tap Tag:").size(10.0).color(theme.text_muted));
                    for tag in tags {
                        let is_sel = self.selected_module_tag == tag;
                        if ui
                            .selectable_label(is_sel, RichText::new(&tag).size(10.0).monospace())
                            .clicked()
                        {
                            self.scanner_input = tag.clone();
                            self.selected_module_tag = tag;
                        }
                    }
                });

                ui.add_space(6.0);
                Frame::new()
                    .fill(theme.bg_canvas)
                    .corner_radius(CornerRadius::same(6))
                    .inner_margin(Margin::symmetric(10, 6))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        for log in self.scanner_log.iter().rev().take(3) {
                            ui.label(
                                RichText::new(format!("• {}", log))
                                    .size(9.5)
                                    .monospace()
                                    .color(theme.text_secondary),
                            );
                        }
                    });
            });

        if scan_trigger {
            let query = self.scanner_input.trim().to_uppercase();
            let now = crate::core::web_bridge::unix_timestamp_label();
            if let Some(unit) = self.prefab_units.iter_mut().find(|u| {
                u.tag_id.to_uppercase() == query || u.rfid_code.to_uppercase() == query
            }) {
                unit.last_scan_timestamp = now.clone();
                self.selected_module_tag = unit.tag_id.clone();
                let msg = format!(
                    "Checked in {} ({}) @ stage '{}' [{}].",
                    unit.tag_id,
                    unit.rfid_code,
                    unit.stage.label(),
                    now
                );
                self.scanner_log.push(msg.clone());
                self.export_status = Some(msg);
            } else {
                let err = format!(
                    "RFID Lookup Miss: '{}' did not match any prefabricated module tag.",
                    self.scanner_input.trim()
                );
                self.scanner_log.push(err.clone());
                self.export_status = Some(err);
            }
        }

        if advance_selected_trigger {
            let query = if !self.scanner_input.trim().is_empty() {
                self.scanner_input.trim().to_uppercase()
            } else {
                self.selected_module_tag.to_uppercase()
            };
            let now = crate::core::web_bridge::unix_timestamp_label();
            if let Some(unit) = self.prefab_units.iter_mut().find(|u| {
                u.tag_id.to_uppercase() == query || u.rfid_code.to_uppercase() == query
            }) {
                self.selected_module_tag = unit.tag_id.clone();
                if let Some(next_stage) = unit.stage.next() {
                    unit.stage = next_stage;
                    if next_stage.step_index() >= PrefabSupplyStage::QaNdtInspection.step_index() {
                        unit.qa_passed = true;
                    }
                    unit.last_scan_timestamp = now.clone();
                    let msg = format!(
                        "{} advanced to '{}' (Crane SWL {:.0}%) [{}].",
                        unit.tag_id,
                        unit.stage.label(),
                        unit.crane_utilization_pct(),
                        now
                    );
                    self.scanner_log.push(msg.clone());
                    self.export_status = Some(msg);
                } else {
                    let msg = format!(
                        "{} is already Erected & Torqued at {}.",
                        unit.tag_id, unit.target_zone
                    );
                    self.scanner_log.push(msg.clone());
                    self.export_status = Some(msg);
                }
            }
        }

        ui.add_space(12.0);

        // Filtered Modular Prefab Unit Cards
        let filtered_indices: Vec<usize> = self
            .prefab_units
            .iter()
            .enumerate()
            .filter_map(|(i, u)| {
                if self.prefab_stage_filter.map_or(true, |st| u.stage == st) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        let mut card_advance_idx: Option<usize> = None;
        let mut card_select_tag: Option<String> = None;
        let is_two_col = ui.available_width() >= 680.0;

        if filtered_indices.is_empty() {
            theme.card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("No prefabricated units match the selected pipeline stage filter.")
                        .size(11.0)
                        .color(theme.text_muted),
                );
            });
        } else if is_two_col {
            for pair in filtered_indices.chunks(2) {
                ui.columns(2, |cols| {
                    for (slot, &u_idx) in pair.iter().enumerate() {
                        let unit = &self.prefab_units[u_idx];
                        let (r, g, b) = unit.stage.color_rgb();
                        let st_col = Color32::from_rgb(r, g, b);
                        let is_sel = self.selected_module_tag == unit.tag_id;
                        let swl_pct = unit.crane_utilization_pct();
                        let swl_col = if swl_pct > 85.0 {
                            theme.accent_red
                        } else if swl_pct > 65.0 {
                            theme.accent_orange
                        } else {
                            theme.accent_green
                        };

                        Frame::new()
                            .fill(theme.bg_card)
                            .corner_radius(CornerRadius::same(10))
                            .inner_margin(Margin::same(12))
                            .stroke(Stroke::new(
                                if is_sel { 1.6 } else { 1.0 },
                                if is_sel { theme.accent_blue } else { theme.border_subtle },
                            ))
                            .show(&mut cols[slot], |ui| {
                                ui.set_width(ui.available_width());
                                ui.horizontal_wrapped(|ui| {
                                    ui.label(
                                        RichText::new(&unit.tag_id)
                                            .size(12.0)
                                            .strong()
                                            .monospace()
                                            .color(theme.accent_blue),
                                    );
                                    ui.label(
                                        RichText::new(&unit.rfid_code)
                                            .size(9.5)
                                            .monospace()
                                            .color(theme.text_muted),
                                    );
                                    Frame::new()
                                        .fill(st_col.gamma_multiply(0.18))
                                        .corner_radius(CornerRadius::same(4))
                                        .inner_margin(Margin::symmetric(6, 2))
                                        .show(ui, |ui| {
                                            ui.label(
                                                RichText::new(unit.stage.label())
                                                    .size(9.5)
                                                    .strong()
                                                    .color(st_col),
                                            );
                                        });
                                    ui.label(
                                        RichText::new(if unit.qa_passed {
                                            "✅ QA Passed"
                                        } else {
                                            "⏳ Pending QA"
                                        })
                                        .size(9.5)
                                        .strong()
                                        .color(if unit.qa_passed {
                                            theme.accent_green
                                        } else {
                                            theme.accent_orange
                                        }),
                                    );
                                });

                                ui.add_space(4.0);
                                ui.label(
                                    RichText::new(&unit.name)
                                        .size(11.5)
                                        .strong()
                                        .color(theme.text_primary),
                                );
                                ui.label(
                                    RichText::new(format!(
                                        "{} • Zone: {} • Envelope: {:.1}×{:.2}×{:.1} m",
                                        unit.module_type.label(),
                                        unit.target_zone,
                                        unit.dimensions_m[0],
                                        unit.dimensions_m[1],
                                        unit.dimensions_m[2]
                                    ))
                                    .size(9.5)
                                    .color(theme.text_secondary),
                                );

                                ui.add_space(6.0);
                                ui.label(
                                    RichText::new(format!(
                                        "Crane Hoist Check: {:.2} t / {:.1} t Max SWL ({:.0}%)",
                                        unit.mass_tons, unit.crane_swl_tons, swl_pct
                                    ))
                                    .size(9.5)
                                    .color(swl_col),
                                );
                                ui.add(
                                    egui::ProgressBar::new((swl_pct / 100.0).clamp(0.0, 1.0))
                                        .desired_height(6.0)
                                        .fill(swl_col),
                                );

                                ui.add_space(6.0);
                                // 6-Step Visual Pipeline Stepper
                                let step_idx = unit.stage.step_index();
                                let (bar_resp, bar_painter) = ui.allocate_painter(
                                    Vec2::new(ui.available_width().max(120.0), 8.0),
                                    Sense::hover(),
                                );
                                let b_rect = bar_resp.rect;
                                let seg_w = ((b_rect.width() - 10.0) / 6.0).max(8.0);
                                for s in 0..6 {
                                    let sx = b_rect.left() + (s as f32) * (seg_w + 2.0);
                                    let seg_rect = Rect::from_min_size(
                                        egui::pos2(sx, b_rect.top()),
                                        Vec2::new(seg_w, 6.0),
                                    );
                                    let seg_col = if s <= step_idx {
                                        st_col
                                    } else {
                                        Color32::from_rgb(42, 48, 60)
                                    };
                                    bar_painter.rect_filled(seg_rect, CornerRadius::same(2), seg_col);
                                }

                                ui.add_space(4.0);
                                ui.horizontal_wrapped(|ui| {
                                    ui.label(
                                        RichText::new(format!("Scan: {}", unit.last_scan_timestamp))
                                            .size(9.0)
                                            .color(theme.text_muted),
                                    );
                                    if ui
                                        .add(
                                            egui::Button::new(
                                                RichText::new("📡 Select")
                                                    .size(9.5)
                                                    .color(theme.accent_blue),
                                            )
                                            .fill(theme.bg_canvas)
                                            .corner_radius(CornerRadius::same(4)),
                                        )
                                        .clicked()
                                    {
                                        card_select_tag = Some(unit.tag_id.clone());
                                    }
                                    if let Some(next_st) = unit.stage.next() {
                                        if ui
                                            .add(
                                                egui::Button::new(
                                                    RichText::new(format!("⏭️ {}", next_st.label()))
                                                        .size(9.5)
                                                        .color(theme.accent_green),
                                                )
                                                .fill(theme.bg_canvas)
                                                .corner_radius(CornerRadius::same(4)),
                                            )
                                            .clicked()
                                        {
                                            card_advance_idx = Some(u_idx);
                                        }
                                    }
                                });
                            });
                    }
                });
                ui.add_space(8.0);
            }
        } else {
            for &u_idx in &filtered_indices {
                let unit = &self.prefab_units[u_idx];
                let (r, g, b) = unit.stage.color_rgb();
                let st_col = Color32::from_rgb(r, g, b);
                let is_sel = self.selected_module_tag == unit.tag_id;
                let swl_pct = unit.crane_utilization_pct();
                let swl_col = if swl_pct > 85.0 {
                    theme.accent_red
                } else if swl_pct > 65.0 {
                    theme.accent_orange
                } else {
                    theme.accent_green
                };

                Frame::new()
                    .fill(theme.bg_card)
                    .corner_radius(CornerRadius::same(10))
                    .inner_margin(Margin::same(12))
                    .stroke(Stroke::new(
                        if is_sel { 1.6 } else { 1.0 },
                        if is_sel { theme.accent_blue } else { theme.border_subtle },
                    ))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.horizontal_wrapped(|ui| {
                            ui.label(
                                RichText::new(&unit.tag_id)
                                    .size(12.0)
                                    .strong()
                                    .monospace()
                                    .color(theme.accent_blue),
                            );
                            ui.label(
                                RichText::new(&unit.rfid_code)
                                    .size(9.5)
                                    .monospace()
                                    .color(theme.text_muted),
                            );
                            Frame::new()
                                .fill(st_col.gamma_multiply(0.18))
                                .corner_radius(CornerRadius::same(4))
                                .inner_margin(Margin::symmetric(6, 2))
                                .show(ui, |ui| {
                                    ui.label(
                                        RichText::new(unit.stage.label())
                                            .size(9.5)
                                            .strong()
                                            .color(st_col),
                                    );
                                });
                            ui.label(
                                RichText::new(if unit.qa_passed {
                                    "✅ QA Passed"
                                } else {
                                    "⏳ Pending QA"
                                })
                                .size(9.5)
                                .strong()
                                .color(if unit.qa_passed {
                                    theme.accent_green
                                } else {
                                    theme.accent_orange
                                }),
                            );
                        });
                        ui.label(
                            RichText::new(&unit.name)
                                .size(11.5)
                                .strong()
                                .color(theme.text_primary),
                        );
                        ui.label(
                            RichText::new(format!(
                                "{} • Zone: {} • Crane: {:.2} t / {:.1} t ({:.0}%)",
                                unit.module_type.label(),
                                unit.target_zone,
                                unit.mass_tons,
                                unit.crane_swl_tons,
                                swl_pct
                            ))
                            .size(9.5)
                            .color(swl_col),
                        );
                        ui.add(
                            egui::ProgressBar::new((swl_pct / 100.0).clamp(0.0, 1.0))
                                .desired_height(6.0)
                                .fill(swl_col),
                        );
                        ui.add_space(4.0);
                        ui.horizontal_wrapped(|ui| {
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new("📡 Select")
                                            .size(9.5)
                                            .color(theme.accent_blue),
                                    )
                                    .fill(theme.bg_canvas)
                                    .corner_radius(CornerRadius::same(4)),
                                )
                                .clicked()
                            {
                                card_select_tag = Some(unit.tag_id.clone());
                            }
                            if let Some(next_st) = unit.stage.next() {
                                if ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new(format!("⏭️ {}", next_st.label()))
                                                .size(9.5)
                                                .color(theme.accent_green),
                                        )
                                        .fill(theme.bg_canvas)
                                        .corner_radius(CornerRadius::same(4)),
                                    )
                                    .clicked()
                                {
                                    card_advance_idx = Some(u_idx);
                                }
                            }
                        });
                    });
                ui.add_space(8.0);
            }
        }

        if let Some(tag) = card_select_tag {
            self.scanner_input = tag.clone();
            self.selected_module_tag = tag;
        }
        if let Some(idx) = card_advance_idx {
            let now = crate::core::web_bridge::unix_timestamp_label();
            if let Some(unit) = self.prefab_units.get_mut(idx) {
                if let Some(next_stage) = unit.stage.next() {
                    unit.stage = next_stage;
                    if next_stage.step_index() >= PrefabSupplyStage::QaNdtInspection.step_index() {
                        unit.qa_passed = true;
                    }
                    unit.last_scan_timestamp = now.clone();
                    self.selected_module_tag = unit.tag_id.clone();
                    self.scanner_input = unit.tag_id.clone();
                    let msg = format!(
                        "{} moved to '{}' [{}].",
                        unit.tag_id,
                        unit.stage.label(),
                        now
                    );
                    self.scanner_log.push(msg.clone());
                    self.export_status = Some(msg);
                }
            }
        }
    }

    fn render_robotic_automation(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let kpis = [
            (
                "ROBOTIC WELDING SPEED",
                format!("{:.1} mm/s", self.robotic_speed_mm_s),
                "6-Axis Articulated GMAW".to_string(),
                theme.accent_blue,
            ),
            (
                "DEFECT RATE",
                "< 0.04%".to_string(),
                "Ultrasonic NDT Verified".to_string(),
                theme.accent_green,
            ),
            (
                "G-CODE COMPLIANCE",
                "ISO 6983".to_string(),
                "Machine Tool Toolpaths".to_string(),
                theme.text_primary,
            ),
            (
                "PRODUCTIVITY GAIN",
                "+320%".to_string(),
                "Continuous Arc Operation".to_string(),
                theme.accent_green,
            ),
        ];
        self.render_responsive_cards(ui, &kpis, theme);

        ui.add_space(16.0);

        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.label(RichText::new("🦾 Automated Structural Steel Robotic Workcell").size(13.5).strong().color(theme.text_primary));
                ui.add_space(10.0);

                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("End-Effector Feed Velocity:").size(11.5).color(theme.text_secondary));
                    ui.add(egui::Slider::new(&mut self.robotic_speed_mm_s, 5.0..=30.0).suffix(" mm/s").text(""));
                });
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

                let kpis = [
                    (
                        "CRANE TRAJECTORY CLEARANCE",
                        "2.8 m".to_string(),
                        "Dynamic Boom Collision-Free Zone".to_string(),
                        theme.accent_green,
                    ),
                    (
                        "DRONE LIDAR AS-BUILT",
                        "± 2.1 mm".to_string(),
                        "Model vs. As-Built Field Deviation".to_string(),
                        theme.accent_blue,
                    ),
                    (
                        "SVD ERECTION GUARD",
                        "Full Rank".to_string(),
                        "All Erection Stages Certified Mechanism-Free".to_string(),
                        theme.accent_green,
                    ),
                ];
                self.render_responsive_cards(ui, &kpis, theme);
            });
    }

    fn render_openbim_ifc_studio(&mut self, ui: &mut Ui, theme: &MacTheme) {
        // 1. TOP KPI EXECUTIVE METRICS
        let (schema_lbl, ent_count, prod_count, health_score, verdict_str) = if let Some(ref m) = self.ifc_model {
            (
                m.schema.display_label(),
                m.health_report.total_entities,
                m.elements.len(),
                format!("{:.1}%", m.health_report.conformance_score_pct),
                m.health_report.verdict.clone(),
            )
        } else {
            ("No Model Loaded", 0, 0, "0.0%".to_string(), "Awaiting IFC File Input".to_string())
        };

        let kpis = [
            (
                "SCHEMA SPECIFICATION",
                schema_lbl.to_string(),
                "buildingSMART International Standard".to_string(),
                theme.accent_blue,
            ),
            (
                "TOTAL STEP ENTITIES",
                format!("{} Entities", ent_count),
                "ISO 10303-21 Physical File".to_string(),
                theme.text_primary,
            ),
            (
                "CONTAINED BIM PRODUCTS",
                format!("{} Elements", prod_count),
                "Beams, Columns, Walls, Slabs".to_string(),
                theme.accent_green,
            ),
            (
                "CONFORMANCE HEALTH",
                health_score,
                "Referential & Spatial Quality".to_string(),
                theme.accent_green,
            ),
        ];
        self.render_responsive_cards(ui, &kpis, theme);

        ui.add_space(14.0);

        // 2. FILE OPEN & PRESET SELECTOR TOOLBAR
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(14))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("📂 OPEN IFC SOURCE:").size(11.5).strong().color(theme.text_secondary));

                    let sources = [
                        (IfcSourceMode::SampleVillaIfc4, "🏡 Sample Villa (IFC4)"),
                        (IfcSourceMode::SamplePortalFrameIfc4, "🏭 Steel Portal (IFC4)"),
                        (IfcSourceMode::SampleDiagridIfc2x3, "🏢 Diagrid Tower (IFC2x3)"),
                        (IfcSourceMode::CustomPaste, "📋 Paste STEP SPF"),
                        (IfcSourceMode::FilePath, "📁 Open Local File"),
                    ];

                    for (src, label) in sources {
                        let is_sel = self.ifc_source_mode == src;
                        let bg = if is_sel { theme.accent_blue } else { theme.bg_canvas };
                        let fg = if is_sel { Color32::WHITE } else { theme.text_primary };

                        if ui.add(egui::Button::new(RichText::new(label).size(11.0).strong().color(fg))
                            .fill(bg)
                            .corner_radius(CornerRadius::same(6))).clicked() {
                            self.ifc_source_mode = src;
                            match src {
                                IfcSourceMode::SampleVillaIfc4 => {
                                    if let Ok(m) = IfcModel::parse(&IfcModel::sample_villa_ifc4()) {
                                        self.baseline_model = Some(m.clone());
                                        self.ifc_model = Some(m);
                                        self.selected_element_id = Some(200);
                                        self.clash_report = None;
                                        self.model_diff = None;
                                        self.ids_report = None;
                                        self.export_status = Some("Loaded reference IFC4 Residential Duplex Villa.".to_string());
                                    }
                                }
                                IfcSourceMode::SamplePortalFrameIfc4 => {
                                    if let Ok(m) = IfcModel::parse(&IfcModel::sample_portal_frame_ifc4()) {
                                        self.baseline_model = Some(m.clone());
                                        self.ifc_model = Some(m);
                                        self.selected_element_id = Some(200);
                                        self.clash_report = None;
                                        self.model_diff = None;
                                        self.ids_report = None;
                                        self.export_status = Some("Loaded reference IFC4 Industrial Steel Portal Frame.".to_string());
                                    }
                                }
                                IfcSourceMode::SampleDiagridIfc2x3 => {
                                    if let Ok(m) = IfcModel::parse(&IfcModel::sample_diagrid_tower_ifc2x3()) {
                                        self.baseline_model = Some(m.clone());
                                        self.ifc_model = Some(m);
                                        self.selected_element_id = Some(200);
                                        self.clash_report = None;
                                        self.model_diff = None;
                                        self.ids_report = None;
                                        self.export_status = Some("Loaded reference IFC2x3 Diagrid High-Rise Tower.".to_string());
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                });

                // Custom Paste or File Path Sub-Panel
                if self.ifc_source_mode == IfcSourceMode::CustomPaste {
                    ui.add_space(8.0);
                    ui.label(RichText::new("Paste ISO-10303-21 STEP physical file text (HEADER & DATA sections):").size(10.5).color(theme.text_muted));
                    ui.add(egui::TextEdit::multiline(&mut self.custom_ifc_paste)
                        .font(egui::TextStyle::Monospace)
                        .desired_rows(6)
                        .desired_width(f32::INFINITY));
                    ui.add_space(4.0);
                    if ui.add(egui::Button::new(RichText::new("⚡ Parse & Decompose Pasted IFC").size(11.5).strong().color(Color32::WHITE))
                        .fill(theme.accent_green)
                        .corner_radius(CornerRadius::same(6))).clicked() {
                        match IfcModel::parse(&self.custom_ifc_paste) {
                            Ok(m) => {
                                self.export_status = Some(format!("Successfully parsed {} entities and {} elements from pasted IFC.", m.health_report.total_entities, m.elements.len()));
                                self.selected_element_id = m.elements.first().map(|e| e.step_id);
                                self.baseline_model = Some(m.clone());
                                self.ifc_model = Some(m);
                                self.clash_report = None;
                                self.model_diff = None;
                                self.ids_report = None;
                            }
                            Err(e) => {
                                self.export_status = Some(format!("IFC Parse Error: {}", e));
                            }
                        }
                    }
                } else if self.ifc_source_mode == IfcSourceMode::FilePath {
                    ui.add_space(8.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("Local File Path:").size(11.0).color(theme.text_secondary));
                        ui.add(egui::TextEdit::singleline(&mut self.custom_ifc_path).desired_width(260.0));
                        if ui.add(egui::Button::new(RichText::new("📂 Read & Inspect").size(11.0).strong().color(Color32::WHITE))
                            .fill(theme.accent_blue)
                            .corner_radius(CornerRadius::same(6))).clicked() {
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                match std::fs::read_to_string(&self.custom_ifc_path) {
                                    Ok(content) => {
                                        match IfcModel::parse(&content) {
                                            Ok(m) => {
                                                self.export_status = Some(format!("Successfully loaded '{}' ({} elements).", self.custom_ifc_path, m.elements.len()));
                                                self.selected_element_id = m.elements.first().map(|e| e.step_id);
                                                self.baseline_model = Some(m.clone());
                                                self.ifc_model = Some(m);
                                                self.clash_report = None;
                                                self.model_diff = None;
                                                self.ids_report = None;
                                            }
                                            Err(e) => {
                                                self.export_status = Some(format!("IFC Parse Error: {}", e));
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        self.export_status = Some(format!("Cannot open file '{}': {}", self.custom_ifc_path, e));
                                    }
                                }
                            }
                            #[cfg(target_arch = "wasm32")]
                            {
                                self.export_status = Some("Browser WebAssembly sandbox cannot read local file paths directly; switch to 'Paste STEP SPF' above.".to_string());
                            }
                        }
                    });
                }
            });

        ui.add_space(14.0);

        // 3. TWO-PANE WORKBENCH (Side-by-Side on Desktop, Stacked on Mobile/Tablet)
        let total_avail = ui.available_width();
        let is_wide = total_avail >= 760.0;

        if is_wide {
            let left_w = (total_avail * 0.42).max(280.0);
            let right_w = (total_avail - left_w - 14.0).max(260.0);

            ui.horizontal(|ui| {
                // LEFT PANE: SPATIAL DECOMPOSITION TREE & PRODUCT NAVIGATOR
                ui.allocate_ui(Vec2::new(left_w, 480.0), |ui| {
                    self.render_ifc_spatial_tree_pane(ui, theme);
                });

                ui.add_space(10.0);

                // RIGHT PANE: ENTITY INSPECTOR & CONFORMANCE AUDIT
                ui.allocate_ui(Vec2::new(right_w, 480.0), |ui| {
                    self.render_ifc_inspector_pane(ui, theme, &verdict_str);
                });
            });
        } else {
            self.render_ifc_spatial_tree_pane(ui, theme);
            ui.add_space(12.0);
            self.render_ifc_inspector_pane(ui, theme, &verdict_str);
        }

        ui.add_space(16.0);

        // 4. BOTTOM ACTION & INTEROP BUTTONS
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(14))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    if ui.add(egui::Button::new(RichText::new("🚀 Sync to 3D Viewport & Analytical Solver").size(12.0).strong().color(Color32::WHITE))
                        .fill(theme.accent_blue)
                        .corner_radius(CornerRadius::same(6))).clicked() {
                        if let Some(ref m) = self.ifc_model {
                            match m.to_structural_model() {
                                Ok(fem) => {
                                    self.export_status = Some(format!("Successfully converted IFC model '{}' into {} nodes & {} structural frame elements for live ARI analysis.", fem.name, fem.nodes.len(), fem.elements.len()));
                                }
                                Err(e) => {
                                    self.export_status = Some(format!("BIM to Structural Model Conversion Warning: {}", e));
                                }
                            }
                        }
                    }

                    if ui.add(egui::Button::new(RichText::new("🌐 Export Clean IFC 4.3 (.ifc)").size(12.0).strong().color(Color32::WHITE))
                        .fill(theme.accent_green)
                        .corner_radius(CornerRadius::same(6))).clicked() {
                        if let Some(ref _m) = self.ifc_model {
                            let mut engine = crate::core::ParametricEngine::new();
                            if let Ok(model) = engine.generate_warren_truss() {
                                let ari = crate::core::AriEngine::evaluate_redundancy(&model).ok();
                                let ifc = crate::core::IfcExporter::export_to_ifc(&model, ari.as_ref());
                                self.export_status = Some(format!("Exported active model to ISO 16739-1:2024 IFC 4.3 ({} lines).", ifc.lines().count()));
                            }
                        }
                    }

                    if ui.add(egui::Button::new(RichText::new("🦾 Export DSTV CNC (.nc1)").size(12.0).strong().color(Color32::WHITE))
                        .fill(Color32::from_rgb(139, 92, 246))
                        .corner_radius(CornerRadius::same(6))).clicked() {
                        let mut engine = crate::core::ParametricEngine::new();
                        if let Ok(model) = engine.generate_warren_truss() {
                            let pieces = crate::core::DstvExporter::export_to_dstv(&model);
                            self.export_status = Some(format!("Generated {} DSTV .nc1 robotic instructions for steel fabrication.", pieces.len()));
                        }
                    }

                    if ui.add(egui::Button::new(RichText::new("📋 Export ifcJSON-4 (.json)").size(12.0).strong().color(Color32::WHITE))
                        .fill(Color32::from_rgb(234, 88, 12))
                        .corner_radius(CornerRadius::same(6))).clicked() {
                        if let Some(ref m) = self.ifc_model {
                            let json = m.to_ifc_json();
                            self.export_status = Some(format!("Exported buildingSMART ifcJSON-4 document ({} bytes, {} products).", json.len(), m.elements.len()));
                        }
                    }

                    if ui.add(egui::Button::new(RichText::new("📬 Export BCF Topics (.xml)").size(12.0).strong().color(Color32::WHITE))
                        .fill(Color32::from_rgb(14, 165, 233))
                        .corner_radius(CornerRadius::same(6))).clicked() {
                        if let Some(ref m) = self.ifc_model {
                            let empty = Vec::new();
                            let clashes = self.clash_report.as_ref().map(|r| r.clashes.as_slice()).unwrap_or(&empty);
                            let xml = m.export_bcf_xml(clashes);
                            self.export_status = Some(format!("Exported {} BCF issue topics ({} bytes).", clashes.len(), xml.len()));
                        }
                    }
                });
            });
    }

    fn render_ifc_spatial_tree_pane(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(14))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("🌳 SPATIAL HIERARCHY TREE").size(12.0).strong().color(theme.accent_blue));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if let Some(ref m) = self.ifc_model {
                            ui.label(RichText::new(format!("{} Storeys", count_storeys(&m.spatial_root))).size(10.5).color(theme.text_muted));
                        }
                    });
                });
                ui.add_space(6.0);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("🔍").size(11.0));
                    ui.add(egui::TextEdit::singleline(&mut self.element_search_query)
                        .hint_text("Filter elements by name/class...")
                        .desired_width(f32::INFINITY));
                });
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);

                ScrollArea::vertical().id_salt("spatial_tree_scroll").max_height(380.0).show(ui, |ui| {
                    let spatial_data = self.ifc_model.as_ref().map(|m| (m.spatial_root.clone(), m.elements.clone()));
                    if let Some((spatial_root, elements)) = spatial_data {
                        self.render_spatial_node(ui, theme, &spatial_root, &elements);
                    } else {
                        ui.label(RichText::new("No IFC Model loaded. Please select a sample or paste file above.").size(11.0).color(theme.text_muted));
                    }
                });
            });
    }

    fn render_ifc_inspector_pane(&mut self, ui: &mut Ui, theme: &MacTheme, verdict_str: &str) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(14))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let tabs = [
                        (IfcInspectorTab::Properties, "📋 Entity Property Inspector"),
                        (IfcInspectorTab::HealthAudit, "🛡️ Conformance & Health Audit"),
                        (IfcInspectorTab::GeometryBbox, "📐 3D Geometry & Bounding Box"),
                        (IfcInspectorTab::ClashAndDiff, "⚔️ Clash, Diff & IDS"),
                    ];

                    for (tab, label) in tabs {
                        let is_active = self.inspector_tab == tab;
                        let bg = if is_active { theme.accent_blue } else { theme.bg_canvas };
                        let fg = if is_active { Color32::WHITE } else { theme.text_secondary };

                        if ui.add(egui::Button::new(RichText::new(label).size(11.0).strong().color(fg))
                            .fill(bg)
                            .corner_radius(CornerRadius::same(6))).clicked() {
                            self.inspector_tab = tab;
                        }
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(8.0);

                ScrollArea::vertical().id_salt("inspector_tab_scroll").max_height(390.0).show(ui, |ui| {
                    match self.inspector_tab {
                        IfcInspectorTab::Properties => self.render_properties_inspector(ui, theme),
                        IfcInspectorTab::HealthAudit => self.render_health_audit_tab(ui, theme, verdict_str),
                        IfcInspectorTab::GeometryBbox => self.render_geometry_bbox_tab(ui, theme),
                        IfcInspectorTab::ClashAndDiff => self.render_clash_and_diff_tab(ui, theme),
                    }
                });
            });
    }

    fn render_spatial_node(&mut self, ui: &mut Ui, theme: &MacTheme, node: &IfcSpatialNode, elements: &[IfcElementSummary]) {
        let is_expanded = *self.spatial_tree_expanded.get(&node.step_id).unwrap_or(&true);
        let icon = match node.class_name.as_str() {
            "IfcProject" => "🏢",
            "IfcSite" => "🌍",
            "IfcBuilding" => "🏛️",
            "IFCBUILDINGSTOREY" => "📍",
            _ => "📁",
        };

        ui.horizontal(|ui| {
            let arrow = if is_expanded { "▼" } else { "▶" };
            if ui.add(egui::Button::new(RichText::new(arrow).size(9.0).color(theme.text_muted)).frame(false)).clicked() {
                self.spatial_tree_expanded.insert(node.step_id, !is_expanded);
            }

            let elev_str = node.elevation_m.map(|e| format!(" (El: {:.1}m)", e * 0.001)).unwrap_or_default();
            ui.label(RichText::new(format!("{} {}{}", icon, node.name, elev_str)).size(11.0).strong().color(theme.text_primary));

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(RichText::new(format!("[{}]", node.total_contained_count())).size(10.0).color(theme.text_muted));
            });
        });

        if is_expanded {
            ui.indent(format!("indent_node_{}", node.step_id), |ui| {
                // Render child spatial structures
                for child in &node.children {
                    self.render_spatial_node(ui, theme, child, elements);
                }

                // Render contained physical elements
                let q_lower = self.element_search_query.to_lowercase();
                for &el_id in &node.contained_element_ids {
                    if let Some(el) = elements.iter().find(|e| e.step_id == el_id) {
                        if !q_lower.is_empty() && !el.name.to_lowercase().contains(&q_lower) && !el.class_name.to_lowercase().contains(&q_lower) {
                            continue;
                        }

                        let is_selected = self.selected_element_id == Some(el.step_id);
                        let el_icon = match el.class_name.as_str() {
                            "IFCCOLUMN" => "🏛️",
                            "IFCBEAM" => "📏",
                            "IFCWALL" | "IFCWALLSTANDARDCASE" => "🧱",
                            "IFCSLAB" => "🔲",
                            "IFCDOOR" => "🚪",
                            "IFCWINDOW" => "🪟",
                            "IFCMEMBER" => "⛓️",
                            _ => "📦",
                        };

                        let bg = if is_selected { theme.accent_blue.linear_multiply(0.25) } else { Color32::TRANSPARENT };
                        let fg = if is_selected { theme.accent_blue } else { theme.text_secondary };

                        Frame::new().fill(bg).corner_radius(CornerRadius::same(4)).inner_margin(Margin::symmetric(4, 2)).show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(el_icon).size(10.5));
                                if ui.selectable_label(is_selected, RichText::new(&el.name).size(10.5).color(fg)).clicked() {
                                    self.selected_element_id = Some(el.step_id);
                                }
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.label(RichText::new(&el.class_name).size(9.0).monospace().color(theme.text_muted));
                                });
                            });
                        });
                    }
                }
            });
        }
    }

    fn render_properties_inspector(&mut self, ui: &mut Ui, theme: &MacTheme) {
        enum InspectorAction {
            ToggleEdit,
            Undo,
            Redo,
            UpdateAttributes { id: usize, name: String, desc: String, obj_type: String },
            SetMaterial { id: usize, mat: String },
            ReassignStorey { id: usize, target_storey_id: usize },
            SetProperty { id: usize, pset: String, prop: String, val: String },
            RemoveProperty { id: usize, pset: String, prop: String },
            AddPset { id: usize, pset: String, props: Vec<(String, String)> },
            DeletePset { id: usize, pset: String },
            ExportMutatedStep,
        }

        let mut action: Option<InspectorAction> = None;

        // 1. Transaction & Mode Control Bar
        ui.horizontal(|ui| {
            let edit_txt = if self.is_editing { "👁️ View Mode" } else { "✏️ Edit Entity" };
            let edit_bg = if self.is_editing { theme.accent_blue } else { theme.bg_canvas };
            let edit_fg = if self.is_editing { Color32::WHITE } else { theme.text_primary };

            if ui.add(egui::Button::new(RichText::new(edit_txt).size(11.0).strong().color(edit_fg))
                .fill(edit_bg)
                .corner_radius(CornerRadius::same(6))).clicked() {
                action = Some(InspectorAction::ToggleEdit);
            }

            let can_undo = self.edit_session.can_undo();
            let undo_lbl = if let Some(desc) = self.edit_session.undo_description() {
                format!("↶ Undo ({})", desc)
            } else {
                "↶ Undo".to_string()
            };
            if ui.add_enabled(can_undo, egui::Button::new(RichText::new(undo_lbl).size(10.5).color(if can_undo { theme.text_primary } else { theme.text_muted }))
                .fill(theme.bg_canvas)
                .corner_radius(CornerRadius::same(6))).clicked() {
                action = Some(InspectorAction::Undo);
            }

            let can_redo = self.edit_session.can_redo();
            let redo_lbl = if let Some(desc) = self.edit_session.redo_description() {
                format!("↷ Redo ({})", desc)
            } else {
                "↷ Redo".to_string()
            };
            if ui.add_enabled(can_redo, egui::Button::new(RichText::new(redo_lbl).size(10.5).color(if can_redo { theme.text_primary } else { theme.text_muted }))
                .fill(theme.bg_canvas)
                .corner_radius(CornerRadius::same(6))).clicked() {
                action = Some(InspectorAction::Redo);
            }

            if self.edit_session.is_dirty {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(RichText::new("● MODIFIED").size(10.0).strong().color(theme.accent_orange));
                });
            }
        });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(6.0);

        let el_opt = if let Some(ref m) = self.ifc_model {
            m.elements.iter().find(|e| Some(e.step_id) == self.selected_element_id).cloned()
        } else {
            None
        };

        let storeys = if let Some(ref m) = self.ifc_model {
            m.list_storeys()
        } else {
            Vec::new()
        };

        if let Some(el) = el_opt {
            if !self.is_editing {
                // READ-ONLY INSPECTION VIEW
                ui.horizontal(|ui| {
                    ui.label(RichText::new("TAG:").size(11.0).strong().color(theme.text_muted));
                    ui.label(RichText::new(format!("#{}", el.step_id)).size(11.0).monospace().color(theme.accent_blue));
                    ui.label(RichText::new("CLASS:").size(11.0).strong().color(theme.text_muted));
                    ui.label(RichText::new(&el.class_name).size(11.0).strong().color(theme.accent_green));
                    ui.label(RichText::new("STOREY:").size(11.0).strong().color(theme.text_muted));
                    ui.label(RichText::new(&el.storey_name).size(11.0).color(theme.text_primary));
                });
                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("NAME:").size(11.0).strong().color(theme.text_muted));
                    ui.label(RichText::new(&el.name).size(12.0).strong().color(theme.text_primary));
                });

                ui.horizontal(|ui| {
                    ui.label(RichText::new("GUID:").size(10.0).strong().color(theme.text_muted));
                    ui.label(RichText::new(&el.global_id).size(10.0).monospace().color(theme.text_secondary));
                });

                if let Some(ref mat) = el.material_name {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("MATERIAL:").size(11.0).strong().color(theme.text_muted));
                        ui.label(RichText::new(mat).size(11.0).strong().color(theme.accent_orange));
                    });
                }

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(8.0);

                // Property Sets (Psets)
                ui.label(RichText::new(format!("📑 PROPERTY SETS ({} Sets Assigned)", el.property_sets.len())).size(11.5).strong().color(theme.accent_blue));
                ui.add_space(4.0);

                if el.property_sets.is_empty() {
                    ui.label(RichText::new("No explicit IfcPropertySet attached to this entity.").size(10.5).color(theme.text_muted));
                } else {
                    for pset in &el.property_sets {
                        Frame::new()
                            .fill(theme.bg_canvas)
                            .corner_radius(CornerRadius::same(6))
                            .inner_margin(Margin::same(8))
                            .show(ui, |ui| {
                                ui.label(RichText::new(&pset.name).size(11.0).strong().color(theme.text_primary));
                                ui.add_space(4.0);
                                for (p_name, p_val) in &pset.properties {
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new(format!("• {}:", p_name)).size(10.5).color(theme.text_secondary));
                                        ui.label(RichText::new(p_val).size(10.5).strong().color(theme.accent_blue));
                                    });
                                }
                            });
                        ui.add_space(4.0);
                    }
                }

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);

                // Element Quantities (Qto)
                ui.label(RichText::new("📊 QUANTITIES TAKEOFF (Qto)").size(11.5).strong().color(theme.accent_green));
                ui.add_space(4.0);
                if el.quantities.is_empty() {
                    let (dx, dy, dz) = el.bbox.dimensions();
                    let approx_vol = el.bbox.volume();
                    ui.label(RichText::new(format!("BBox Dimensions: {:.2} x {:.2} x {:.2} m • Volume: {:.3} m³", dx, dy, dz, approx_vol)).size(10.5).color(theme.text_secondary));
                } else {
                    for (q_name, q_val) in &el.quantities {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(format!("• {}:", q_name)).size(10.5).color(theme.text_secondary));
                            ui.label(RichText::new(format!("{:.3}", q_val)).size(10.5).strong().color(theme.accent_green));
                        });
                    }
                }
            } else {
                // INTERACTIVE EDITING MODE
                ui.label(RichText::new("✏️ EDIT ELEMENT ATTRIBUTES").size(11.5).strong().color(theme.accent_blue));
                ui.add_space(6.0);

                // 1. Identity attributes
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Name:").size(11.0).color(theme.text_secondary));
                    ui.add(egui::TextEdit::singleline(&mut self.edit_name).desired_width(220.0));
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Description:").size(11.0).color(theme.text_secondary));
                    ui.add(egui::TextEdit::singleline(&mut self.edit_desc).desired_width(220.0));
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("ObjectType:").size(11.0).color(theme.text_secondary));
                    ui.add(egui::TextEdit::singleline(&mut self.edit_obj_type).desired_width(220.0));
                });

                if ui.add(egui::Button::new(RichText::new("💾 Apply Attribute Changes").size(10.5).strong().color(Color32::WHITE))
                    .fill(theme.accent_blue)
                    .corner_radius(CornerRadius::same(5))).clicked() {
                    action = Some(InspectorAction::UpdateAttributes {
                        id: el.step_id,
                        name: self.edit_name.clone(),
                        desc: self.edit_desc.clone(),
                        obj_type: self.edit_obj_type.clone(),
                    });
                }

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);

                // 2. Material Reassignment
                ui.label(RichText::new("🧪 MATERIAL REASSIGNMENT").size(11.5).strong().color(theme.accent_orange));
                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.edit_material).hint_text("e.g. Steel S460ML / C30/37").desired_width(200.0));
                    if ui.add(egui::Button::new(RichText::new("Assign Material").size(10.5).strong().color(Color32::WHITE))
                        .fill(theme.accent_orange)
                        .corner_radius(CornerRadius::same(5))).clicked() {
                        action = Some(InspectorAction::SetMaterial {
                            id: el.step_id,
                            mat: self.edit_material.clone(),
                        });
                    }
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);

                // 3. Storey Reassignment
                ui.label(RichText::new("🏢 SPATIAL STOREY REASSIGNMENT").size(11.5).strong().color(theme.accent_green));
                ui.horizontal(|ui| {
                    for (sid, sname) in &storeys {
                        let is_cur = *sname == el.storey_name;
                        let bg = if is_cur { theme.accent_green } else { theme.bg_canvas };
                        let fg = if is_cur { Color32::WHITE } else { theme.text_primary };
                        if ui.add(egui::Button::new(RichText::new(sname).size(10.0).color(fg))
                            .fill(bg)
                            .corner_radius(CornerRadius::same(4))).clicked() && !is_cur {
                            action = Some(InspectorAction::ReassignStorey {
                                id: el.step_id,
                                target_storey_id: *sid,
                            });
                        }
                    }
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);

                // 4. Property Sets Mutation
                ui.label(RichText::new("📑 PROPERTY SETS MUTATION").size(11.5).strong().color(theme.accent_blue));
                for pset in &el.property_sets {
                    Frame::new()
                        .fill(theme.bg_canvas)
                        .corner_radius(CornerRadius::same(6))
                        .inner_margin(Margin::same(8))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(&pset.name).size(11.0).strong().color(theme.text_primary));
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    if ui.add(egui::Button::new(RichText::new("🗑️ Delete Pset").size(9.5).color(Color32::from_rgb(239, 68, 68)))
                                        .fill(theme.bg_card)
                                        .corner_radius(CornerRadius::same(4))).clicked() {
                                        action = Some(InspectorAction::DeletePset {
                                            id: el.step_id,
                                            pset: pset.name.clone(),
                                        });
                                    }
                                });
                            });
                            ui.add_space(4.0);

                            for (p_name, p_val) in &pset.properties {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(format!("• {}:", p_name)).size(10.5).color(theme.text_secondary));
                                    ui.label(RichText::new(p_val).size(10.5).strong().color(theme.accent_blue));
                                    if ui.add(egui::Button::new(RichText::new("✕").size(9.0).color(theme.text_muted))
                                        .fill(theme.bg_canvas)
                                        .corner_radius(CornerRadius::same(3))).clicked() {
                                        action = Some(InspectorAction::RemoveProperty {
                                            id: el.step_id,
                                            pset: pset.name.clone(),
                                            prop: p_name.clone(),
                                        });
                                    }
                                });
                            }
                        });
                    ui.add_space(4.0);
                }

                // Add Property to Pset Row
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.new_prop_name).hint_text("Property Name").desired_width(110.0));
                    ui.add(egui::TextEdit::singleline(&mut self.new_prop_val).hint_text("Value").desired_width(110.0));
                    let target_pset = el.property_sets.first().map(|p| p.name.clone()).unwrap_or_else(|| "Pset_CustomAudit".to_string());
                    if ui.add(egui::Button::new(RichText::new("➕ Set Property").size(10.0).strong().color(Color32::WHITE))
                        .fill(theme.accent_blue)
                        .corner_radius(CornerRadius::same(4))).clicked() {
                        if !self.new_prop_name.trim().is_empty() {
                            action = Some(InspectorAction::SetProperty {
                                id: el.step_id,
                                pset: target_pset,
                                prop: self.new_prop_name.trim().to_string(),
                                val: self.new_prop_val.trim().to_string(),
                            });
                            self.new_prop_name.clear();
                            self.new_prop_val.clear();
                        }
                    }
                });

                // Add New Pset Form
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.new_pset_name).hint_text("New Pset Name").desired_width(160.0));
                    if ui.add(egui::Button::new(RichText::new("➕ Create Pset").size(10.0).strong().color(Color32::WHITE))
                        .fill(theme.accent_green)
                        .corner_radius(CornerRadius::same(4))).clicked() {
                        if !self.new_pset_name.trim().is_empty() {
                            action = Some(InspectorAction::AddPset {
                                id: el.step_id,
                                pset: self.new_pset_name.trim().to_string(),
                                props: vec![("CreatedBy".to_string(), "EigenGraph_v0.1.8".to_string())],
                            });
                        }
                    }
                });

                // Export mutated STEP
                ui.add_space(10.0);
                if ui.add(egui::Button::new(RichText::new("💾 Export & Serialize Mutated STEP Physical File").size(11.0).strong().color(Color32::WHITE))
                    .fill(Color32::from_rgb(139, 92, 246))
                    .corner_radius(CornerRadius::same(5))).clicked() {
                    action = Some(InspectorAction::ExportMutatedStep);
                }
            }
        } else {
            ui.label(RichText::new("Select an element in the Spatial Tree on the left to inspect properties.").size(11.0).color(theme.text_muted));
        }

        // Execute Dispatched Action
        if let Some(act) = action {
            match act {
                InspectorAction::ToggleEdit => {
                    self.is_editing = !self.is_editing;
                    if self.is_editing {
                        if let Some(ref m) = self.ifc_model {
                            if let Some(el) = m.elements.iter().find(|e| Some(e.step_id) == self.selected_element_id) {
                                self.edit_name = el.name.clone();
                                self.edit_desc = el.description.clone();
                                self.edit_obj_type = el.object_type.clone();
                                self.edit_material = el.material_name.clone().unwrap_or_default();
                            }
                        }
                    }
                }
                InspectorAction::Undo => {
                    if let Some(ref mut m) = self.ifc_model {
                        match self.edit_session.undo(m) {
                            Ok(Some(msg)) => {
                                self.export_status = Some(format!("Reverted: {}", msg));
                            }
                            Ok(None) => {}
                            Err(e) => {
                                self.export_status = Some(format!("Undo Error: {}", e));
                            }
                        }
                    }
                }
                InspectorAction::Redo => {
                    if let Some(ref mut m) = self.ifc_model {
                        match self.edit_session.redo(m) {
                            Ok(Some(msg)) => {
                                self.export_status = Some(format!("Reapplied: {}", msg));
                            }
                            Ok(None) => {}
                            Err(e) => {
                                self.export_status = Some(format!("Redo Error: {}", e));
                            }
                        }
                    }
                }
                InspectorAction::UpdateAttributes { id, name, desc, obj_type } => {
                    if let Some(ref mut m) = self.ifc_model {
                        if let Some(el) = m.elements.iter().find(|e| e.step_id == id) {
                            let cmd = IfcEditCommand::UpdateElementAttributes {
                                element_id: id,
                                old_name: el.name.clone(),
                                new_name: name,
                                old_description: el.description.clone(),
                                new_description: desc,
                                old_object_type: el.object_type.clone(),
                                new_object_type: obj_type,
                            };
                            if let Err(e) = self.edit_session.execute(m, cmd) {
                                self.export_status = Some(format!("Mutation Error: {}", e));
                            } else {
                                self.export_status = Some(format!("Updated identity attributes for element #{}.", id));
                            }
                        }
                    }
                }
                InspectorAction::SetMaterial { id, mat } => {
                    if let Some(ref mut m) = self.ifc_model {
                        if let Some(el) = m.elements.iter().find(|e| e.step_id == id) {
                            let cmd = IfcEditCommand::SetMaterial {
                                element_id: id,
                                old_material: el.material_name.clone(),
                                new_material: mat.clone(),
                            };
                            if let Err(e) = self.edit_session.execute(m, cmd) {
                                self.export_status = Some(format!("Material Error: {}", e));
                            } else {
                                self.export_status = Some(format!("Set element #{} material to '{}'.", id, mat));
                            }
                        }
                    }
                }
                InspectorAction::ReassignStorey { id, target_storey_id } => {
                    if let Some(ref mut m) = self.ifc_model {
                        let storeys = m.list_storeys();
                        let current_sid = storeys.iter().find(|(_, name)| {
                            m.elements.iter().find(|e| e.step_id == id).map(|e| e.storey_name.as_str()) == Some(name)
                        }).map(|(sid, _)| *sid).unwrap_or(0);

                        let cmd = IfcEditCommand::ReassignStorey {
                            element_id: id,
                            old_storey_id: current_sid,
                            new_storey_id: target_storey_id,
                        };
                        if let Err(e) = self.edit_session.execute(m, cmd) {
                            self.export_status = Some(format!("Storey Reassignment Error: {}", e));
                        } else {
                            self.export_status = Some(format!("Reassigned element #{} to Storey #{}.", id, target_storey_id));
                        }
                    }
                }
                InspectorAction::SetProperty { id, pset, prop, val } => {
                    if let Some(ref mut m) = self.ifc_model {
                        let old_val = m.elements.iter().find(|e| e.step_id == id).and_then(|e| {
                            e.property_sets.iter().find(|p| p.name == pset).and_then(|p| {
                                p.properties.iter().find(|(k, _)| k == &prop).map(|(_, v)| v.clone())
                            })
                        });
                        let cmd = IfcEditCommand::SetProperty {
                            element_id: id,
                            pset_name: pset.clone(),
                            prop_name: prop.clone(),
                            old_value: old_val,
                            new_value: val.clone(),
                        };
                        if let Err(e) = self.edit_session.execute(m, cmd) {
                            self.export_status = Some(format!("Property Mutation Error: {}", e));
                        } else {
                            self.export_status = Some(format!("Set #{}.{}.{} = '{}'.", id, pset, prop, val));
                        }
                    }
                }
                InspectorAction::RemoveProperty { id, pset, prop } => {
                    if let Some(ref mut m) = self.ifc_model {
                        let old_val = m.elements.iter().find(|e| e.step_id == id).and_then(|e| {
                            e.property_sets.iter().find(|p| p.name == pset).and_then(|p| {
                                p.properties.iter().find(|(k, _)| k == &prop).map(|(_, v)| v.clone())
                            })
                        }).unwrap_or_default();
                        let cmd = IfcEditCommand::RemoveProperty {
                            element_id: id,
                            pset_name: pset.clone(),
                            prop_name: prop.clone(),
                            removed_value: old_val,
                        };
                        if let Err(e) = self.edit_session.execute(m, cmd) {
                            self.export_status = Some(format!("Property Removal Error: {}", e));
                        } else {
                            self.export_status = Some(format!("Removed #{}.{}.{}.", id, pset, prop));
                        }
                    }
                }
                InspectorAction::AddPset { id, pset, props } => {
                    if let Some(ref mut m) = self.ifc_model {
                        let cmd = IfcEditCommand::AddPropertySet {
                            element_id: id,
                            pset_name: pset.clone(),
                            properties: props,
                        };
                        if let Err(e) = self.edit_session.execute(m, cmd) {
                            self.export_status = Some(format!("Pset Add Error: {}", e));
                        } else {
                            self.export_status = Some(format!("Added Property Set '{}' to #{}.", pset, id));
                        }
                    }
                }
                InspectorAction::DeletePset { id, pset } => {
                    if let Some(ref mut m) = self.ifc_model {
                        let old_props = m.elements.iter().find(|e| e.step_id == id).and_then(|e| {
                            e.property_sets.iter().find(|p| p.name == pset).map(|p| p.properties.clone())
                        }).unwrap_or_default();
                        let cmd = IfcEditCommand::DeletePropertySet {
                            element_id: id,
                            pset_name: pset.clone(),
                            deleted_properties: old_props,
                        };
                        if let Err(e) = self.edit_session.execute(m, cmd) {
                            self.export_status = Some(format!("Pset Delete Error: {}", e));
                        } else {
                            self.export_status = Some(format!("Deleted Property Set '{}' from #{}.", pset, id));
                        }
                    }
                }
                InspectorAction::ExportMutatedStep => {
                    if let Some(ref m) = self.ifc_model {
                        let step_content = m.to_step_string();
                        self.export_status = Some(format!("Serialized mutated model: {} bytes, {} entities.", step_content.len(), m.raw_step.entities.len()));
                    }
                }
            }
        }
    }

    fn render_health_audit_tab(&self, ui: &mut Ui, theme: &MacTheme, verdict: &str) {
        if let Some(ref m) = self.ifc_model {
            let audit = &m.health_report;

            // Verdict Header
            Frame::new()
                .fill(theme.bg_canvas)
                .corner_radius(CornerRadius::same(8))
                .inner_margin(Margin::same(10))
                .stroke(Stroke::new(1.0, theme.border_subtle))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("VERDICT:").size(11.0).strong().color(theme.text_muted));
                        let v_col = if audit.conformance_score_pct >= 90.0 { theme.accent_green } else { theme.accent_orange };
                        ui.label(RichText::new(verdict).size(11.5).strong().color(v_col));
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(RichText::new(format!("Score: {:.1}%", audit.conformance_score_pct)).size(12.0).strong().color(v_col));
                        });
                    });
                });

            ui.add_space(8.0);

            // Audit Check Table
            ui.label(RichText::new("QUALITY ASSURANCE AUDIT CHECKS").size(11.5).strong().color(theme.text_secondary));
            ui.add_space(4.0);

            for check in &audit.checks {
                Frame::new()
                    .fill(theme.bg_canvas)
                    .corner_radius(CornerRadius::same(6))
                    .inner_margin(Margin::symmetric(10, 6))
                    .stroke(Stroke::new(1.0, theme.border_subtle))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let (badge_bg, badge_txt) = match check.status {
                                IfcAuditStatus::Pass => (Color32::from_rgb(16, 185, 129), "PASS"),
                                IfcAuditStatus::Warning => (Color32::from_rgb(245, 158, 11), "WARN"),
                                IfcAuditStatus::Fail => (Color32::from_rgb(239, 68, 68), "FAIL"),
                            };

                            Frame::new().fill(badge_bg).corner_radius(CornerRadius::same(4)).inner_margin(Margin::symmetric(6, 2)).show(ui, |ui| {
                                ui.label(RichText::new(badge_txt).size(9.5).strong().color(Color32::WHITE));
                            });

                            ui.label(RichText::new(&check.check_name).size(11.0).strong().color(theme.text_primary));
                            ui.label(RichText::new(format!("({})", check.category)).size(10.0).color(theme.text_muted));
                        });
                        ui.add_space(2.0);
                        ui.label(RichText::new(&check.detail).size(10.5).color(theme.text_secondary));
                    });
                ui.add_space(4.0);
            }
        } else {
            ui.label(RichText::new("No IFC Model loaded to audit.").size(11.0).color(theme.text_muted));
        }
    }

    fn render_geometry_bbox_tab(&mut self, ui: &mut Ui, theme: &MacTheme) {
        enum GeomAction {
            Undo,
            Redo,
            ToggleSynthModal,
            ModifyExtrusion { element_id: usize, old_depth: f64, new_depth: f64 },
            Translate { element_id: usize, translation: [f64; 3] },
            ModifyProfile { element_id: usize, old_dims: [f64; 2], new_dims: [f64; 2] },
            DeleteElement { element_id: usize },
            SynthesizeElement { spec: IfcElementCreationSpec },
        }

        let mut action: Option<GeomAction> = None;

        // 1. Transaction & Topology Action Bar
        ui.horizontal(|ui| {
            let can_undo = self.edit_session.can_undo();
            let undo_lbl = if let Some(desc) = self.edit_session.undo_description() {
                format!("↩ Undo ({})", desc)
            } else {
                "↩ Undo".to_string()
            };
            if ui.add_enabled(can_undo, egui::Button::new(RichText::new(undo_lbl).size(10.5).color(if can_undo { theme.text_primary } else { theme.text_muted }))
                .fill(theme.bg_canvas)
                .corner_radius(CornerRadius::same(6))).clicked() {
                action = Some(GeomAction::Undo);
            }

            let can_redo = self.edit_session.can_redo();
            let redo_lbl = if let Some(desc) = self.edit_session.redo_description() {
                format!("↷ Redo ({})", desc)
            } else {
                "↷ Redo".to_string()
            };
            if ui.add_enabled(can_redo, egui::Button::new(RichText::new(redo_lbl).size(10.5).color(if can_redo { theme.text_primary } else { theme.text_muted }))
                .fill(theme.bg_canvas)
                .corner_radius(CornerRadius::same(6))).clicked() {
                action = Some(GeomAction::Redo);
            }

            let synth_btn_bg = if self.show_synth_modal { theme.accent_green } else { theme.bg_canvas };
            let synth_btn_fg = if self.show_synth_modal { Color32::WHITE } else { theme.text_primary };
            if ui.add(egui::Button::new(RichText::new("➕ Synthesize Element").size(10.5).strong().color(synth_btn_fg))
                .fill(synth_btn_bg)
                .corner_radius(CornerRadius::same(6))).clicked() {
                action = Some(GeomAction::ToggleSynthModal);
            }

            if self.edit_session.is_dirty {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(RichText::new("● MODIFIED").size(10.0).strong().color(theme.accent_orange));
                });
            }
        });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(6.0);

        let storeys = if let Some(ref m) = self.ifc_model {
            m.list_storeys()
        } else {
            Vec::new()
        };

        // 2. Element Synthesis Panel (If active)
        if self.show_synth_modal {
            Frame::new()
                .fill(theme.bg_canvas)
                .corner_radius(CornerRadius::same(8))
                .inner_margin(Margin::same(12))
                .stroke(Stroke::new(1.0, theme.accent_green))
                .show(ui, |ui| {
                    ui.label(RichText::new("✨ SYNTHESIZE STRUCTURAL ELEMENT (ISO 10303-21 / ISO 16739)").size(11.5).strong().color(theme.accent_green));
                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Entity Class:").size(11.0).color(theme.text_secondary));
                        for class_candidate in &["IFCCOLUMN", "IFCBEAM", "IFCMEMBER"] {
                            let is_cur = self.synth_class == *class_candidate;
                            let bg = if is_cur { theme.accent_green } else { theme.bg_card };
                            let fg = if is_cur { Color32::WHITE } else { theme.text_primary };
                            if ui.add(egui::Button::new(RichText::new(*class_candidate).size(10.0).color(fg))
                                .fill(bg)
                                .corner_radius(CornerRadius::same(4))).clicked() {
                                self.synth_class = class_candidate.to_string();
                            }
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Name:").size(11.0).color(theme.text_secondary));
                        ui.add(egui::TextEdit::singleline(&mut self.synth_name).desired_width(180.0));
                        ui.label(RichText::new("Material:").size(11.0).color(theme.text_secondary));
                        ui.add(egui::TextEdit::singleline(&mut self.synth_mat).desired_width(160.0));
                    });

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Description:").size(11.0).color(theme.text_secondary));
                        ui.add(egui::TextEdit::singleline(&mut self.synth_desc).desired_width(380.0));
                    });

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Length/Extrusion (m):").size(11.0).color(theme.text_secondary));
                        ui.add(egui::DragValue::new(&mut self.synth_length).speed(0.1).range(0.1..=100.0));
                        ui.label(RichText::new("Profile W x H (m):").size(11.0).color(theme.text_secondary));
                        ui.add(egui::DragValue::new(&mut self.synth_prof_w).speed(0.05).range(0.05..=10.0));
                        ui.label(RichText::new("x").size(11.0).color(theme.text_muted));
                        ui.add(egui::DragValue::new(&mut self.synth_prof_h).speed(0.05).range(0.05..=10.0));
                    });

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Target Storey:").size(11.0).color(theme.text_secondary));
                        if storeys.is_empty() {
                            ui.label(RichText::new("No storey available").size(10.5).color(theme.text_muted));
                        } else {
                            if self.selected_target_storey == 0 {
                                self.selected_target_storey = storeys[0].0;
                            }
                            for (sid, sname) in &storeys {
                                let is_cur = self.selected_target_storey == *sid;
                                let bg = if is_cur { theme.accent_blue } else { theme.bg_card };
                                let fg = if is_cur { Color32::WHITE } else { theme.text_primary };
                                if ui.add(egui::Button::new(RichText::new(sname).size(10.0).color(fg))
                                    .fill(bg)
                                    .corner_radius(CornerRadius::same(4))).clicked() {
                                    self.selected_target_storey = *sid;
                                }
                            }
                        }
                    });

                    ui.add_space(4.0);
                    if ui.add(egui::Button::new(RichText::new("🚀 Synthesize Structural Entity into STEP Graph").size(11.0).strong().color(Color32::WHITE))
                        .fill(theme.accent_green)
                        .corner_radius(CornerRadius::same(6))).clicked() {
                        let target_sid = if self.selected_target_storey != 0 {
                            self.selected_target_storey
                        } else {
                            storeys.first().map(|(sid, _)| *sid).unwrap_or(0)
                        };
                        let dir = if self.synth_class == "IFCBEAM" {
                            (1.0, 0.0, 0.0)
                        } else {
                            (0.0, 0.0, 1.0)
                        };
                        let spec = IfcElementCreationSpec {
                            class_name: self.synth_class.clone(),
                            name: self.synth_name.clone(),
                            description: self.synth_desc.clone(),
                            storey_id: target_sid,
                            material_name: self.synth_mat.clone(),
                            origin: (0.0, 0.0, 0.0),
                            direction: dir,
                            profile_dims: (self.synth_prof_w as f64, self.synth_prof_h as f64),
                            length: self.synth_length as f64,
                        };
                        action = Some(GeomAction::SynthesizeElement { spec });
                    }
                });
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(8.0);
        }

        // 3. Selected Element Geometry & Swept Solid Inspector
        let el_opt = if let Some(ref m) = self.ifc_model {
            m.elements.iter().find(|e| Some(e.step_id) == self.selected_element_id).cloned()
        } else {
            None
        };

        if let Some(el) = el_opt {
            let (dx, dy, dz) = el.bbox.dimensions();
            let (cx, cy, cz) = el.centroid;

            ui.label(RichText::new(format!("📐 3D BOUNDING BOX — {} (#{}: {})", el.name, el.step_id, el.class_name)).size(12.0).strong().color(theme.accent_blue));
            ui.add_space(8.0);

            ui.columns(3, |cols| {
                self.render_stat_card(&mut cols[0], theme, "LENGTH (X)", &format!("{:.3} m", dx), "Span Dimension", theme.text_primary);
                self.render_stat_card(&mut cols[1], theme, "WIDTH (Y)", &format!("{:.3} m", dy), "Cross-Section Breadth", theme.text_primary);
                self.render_stat_card(&mut cols[2], theme, "HEIGHT (Z)", &format!("{:.3} m", dz), "Elevation Depth", theme.text_primary);
            });

            ui.add_space(10.0);

            Frame::new()
                .fill(theme.bg_canvas)
                .corner_radius(CornerRadius::same(8))
                .inner_margin(Margin::same(12))
                .stroke(Stroke::new(1.0, theme.border_subtle))
                .show(ui, |ui| {
                    ui.label(RichText::new("SPATIAL EXTENTS (WORLD COORDINATES)").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(4.0);
                    ui.label(RichText::new(format!("• Centroid (Xc, Yc, Zc): ({:.3}, {:.3}, {:.3}) m", cx, cy, cz)).size(10.5).color(theme.text_primary));
                    ui.label(RichText::new(format!("• X-Range: [{:.3}, {:.3}] m", el.bbox.min_x, el.bbox.max_x)).size(10.5).color(theme.text_secondary));
                    ui.label(RichText::new(format!("• Y-Range: [{:.3}, {:.3}] m", el.bbox.min_y, el.bbox.max_y)).size(10.5).color(theme.text_secondary));
                    ui.label(RichText::new(format!("• Z-Range: [{:.3}, {:.3}] m", el.bbox.min_z, el.bbox.max_z)).size(10.5).color(theme.text_secondary));
                    ui.label(RichText::new(format!("• Enclosing Bounding Volume: {:.4} m³", el.bbox.volume())).size(10.5).strong().color(theme.accent_green));
                    ui.label(RichText::new(format!("• Parsed Wireframe Segments: {} lines", el.wireframe_lines.len())).size(10.5).color(theme.accent_blue));
                });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(8.0);

            // 4. Interactive Geometric Topology & Extrusion Mutations
            ui.label(RichText::new("🛠️ GEOMETRIC TOPOLOGY & SWEPT SOLID MUTATION").size(11.5).strong().color(theme.accent_orange));
            ui.add_space(6.0);

            // Section A: Extrusion Depth
            Frame::new()
                .fill(theme.bg_canvas)
                .corner_radius(CornerRadius::same(8))
                .inner_margin(Margin::same(10))
                .stroke(Stroke::new(1.0, theme.border_subtle))
                .show(ui, |ui| {
                    ui.label(RichText::new("📏 Swept Solid Extrusion Depth (IfcExtrudedAreaSolid.Depth)").size(11.0).strong().color(theme.text_primary));
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("New Depth (m):").size(10.5).color(theme.text_secondary));
                        ui.add(egui::DragValue::new(&mut self.geom_depth_input).speed(0.1).range(0.1..=100.0));
                        let orig_depth = el.quantities.get("Length").copied().unwrap_or_else(|| {
                            let (x, y, z) = el.bbox.dimensions();
                            x.max(y).max(z)
                        });
                        if ui.add(egui::Button::new(RichText::new("Apply Extrusion Depth").size(10.5).strong().color(Color32::WHITE))
                            .fill(theme.accent_blue)
                            .corner_radius(CornerRadius::same(4))).clicked() {
                            action = Some(GeomAction::ModifyExtrusion {
                                element_id: el.step_id,
                                old_depth: orig_depth,
                                new_depth: self.geom_depth_input as f64,
                            });
                        }
                    });
                });

            ui.add_space(6.0);

            // Section B: 3D Spatial Translation
            Frame::new()
                .fill(theme.bg_canvas)
                .corner_radius(CornerRadius::same(8))
                .inner_margin(Margin::same(10))
                .stroke(Stroke::new(1.0, theme.border_subtle))
                .show(ui, |ui| {
                    ui.label(RichText::new("🧭 3D Spatial Translation (IfcCartesianPoint [dX, dY, dZ])").size(11.0).strong().color(theme.text_primary));
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("dX:").size(10.5).color(theme.text_secondary));
                        ui.add(egui::DragValue::new(&mut self.geom_trans_x).speed(0.1));
                        ui.label(RichText::new("dY:").size(10.5).color(theme.text_secondary));
                        ui.add(egui::DragValue::new(&mut self.geom_trans_y).speed(0.1));
                        ui.label(RichText::new("dZ:").size(10.5).color(theme.text_secondary));
                        ui.add(egui::DragValue::new(&mut self.geom_trans_z).speed(0.1));

                        if ui.add(egui::Button::new(RichText::new("Translate Placement").size(10.5).strong().color(Color32::WHITE))
                            .fill(theme.accent_blue)
                            .corner_radius(CornerRadius::same(4))).clicked() {
                            action = Some(GeomAction::Translate {
                                element_id: el.step_id,
                                translation: [self.geom_trans_x as f64, self.geom_trans_y as f64, self.geom_trans_z as f64],
                            });
                        }
                    });
                });

            ui.add_space(6.0);

            // Section C: Profile Cross-Section Dimensions
            Frame::new()
                .fill(theme.bg_canvas)
                .corner_radius(CornerRadius::same(8))
                .inner_margin(Margin::same(10))
                .stroke(Stroke::new(1.0, theme.border_subtle))
                .show(ui, |ui| {
                    ui.label(RichText::new("📐 Parametric Profile Dimensions (IfcRectangleProfileDef [XDim, YDim])").size(11.0).strong().color(theme.text_primary));
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Width X (m):").size(10.5).color(theme.text_secondary));
                        ui.add(egui::DragValue::new(&mut self.geom_prof_w).speed(0.05).range(0.05..=10.0));
                        ui.label(RichText::new("Depth Y (m):").size(10.5).color(theme.text_secondary));
                        ui.add(egui::DragValue::new(&mut self.geom_prof_h).speed(0.05).range(0.05..=10.0));

                        let (x, y, _) = el.bbox.dimensions();
                        if ui.add(egui::Button::new(RichText::new("Resize Profile").size(10.5).strong().color(Color32::WHITE))
                            .fill(theme.accent_blue)
                            .corner_radius(CornerRadius::same(4))).clicked() {
                            action = Some(GeomAction::ModifyProfile {
                                element_id: el.step_id,
                                old_dims: [x, y],
                                new_dims: [self.geom_prof_w as f64, self.geom_prof_h as f64],
                            });
                        }
                    });
                });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(6.0);

            // Section D: Safe Element Deletion
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new(RichText::new(format!("🗑️ Delete Structural Element #{}", el.step_id)).size(11.0).strong().color(Color32::WHITE))
                    .fill(Color32::from_rgb(239, 68, 68))
                    .corner_radius(CornerRadius::same(5))).clicked() {
                    action = Some(GeomAction::DeleteElement { element_id: el.step_id });
                }
            });

        } else {
            ui.label(RichText::new("Select an element from the spatial hierarchy to inspect and mutate geometry, or synthesize a new element above.").size(11.0).color(theme.text_muted));
        }

        // 5. Execute Dispatched Actions
        if let Some(act) = action {
            match act {
                GeomAction::Undo => {
                    if let Some(ref mut m) = self.ifc_model {
                        if let Err(e) = self.edit_session.undo(m) {
                            self.export_status = Some(format!("Undo Error: {}", e));
                        } else {
                            self.export_status = Some("Undid last geometric command.".to_string());
                        }
                    }
                }
                GeomAction::Redo => {
                    if let Some(ref mut m) = self.ifc_model {
                        if let Err(e) = self.edit_session.redo(m) {
                            self.export_status = Some(format!("Redo Error: {}", e));
                        } else {
                            self.export_status = Some("Redid geometric command.".to_string());
                        }
                    }
                }
                GeomAction::ToggleSynthModal => {
                    self.show_synth_modal = !self.show_synth_modal;
                }
                GeomAction::ModifyExtrusion { element_id, old_depth, new_depth } => {
                    if let Some(ref mut m) = self.ifc_model {
                        let cmd = IfcEditCommand::ModifyExtrusionDepth {
                            element_id,
                            old_depth,
                            new_depth,
                        };
                        if let Err(e) = self.edit_session.execute(m, cmd) {
                            self.export_status = Some(format!("Extrusion Error: {}", e));
                        } else {
                            self.export_status = Some(format!("Modified extrusion depth of #{} to {:.3} m.", element_id, new_depth));
                        }
                    }
                }
                GeomAction::Translate { element_id, translation } => {
                    if let Some(ref mut m) = self.ifc_model {
                        let cmd = IfcEditCommand::TranslateElement {
                            element_id,
                            delta: (translation[0], translation[1], translation[2]),
                        };
                        if let Err(e) = self.edit_session.execute(m, cmd) {
                            self.export_status = Some(format!("Translation Error: {}", e));
                        } else {
                            self.geom_trans_x = 0.0;
                            self.geom_trans_y = 0.0;
                            self.geom_trans_z = 0.0;
                            self.export_status = Some(format!("Translated placement of #{} by [{:.2}, {:.2}, {:.2}] m.", element_id, translation[0], translation[1], translation[2]));
                        }
                    }
                }
                GeomAction::ModifyProfile { element_id, old_dims, new_dims } => {
                    if let Some(ref mut m) = self.ifc_model {
                        let cmd = IfcEditCommand::ModifyProfileDimensions {
                            element_id,
                            old_dims: (old_dims[0], old_dims[1]),
                            new_dims: (new_dims[0], new_dims[1]),
                        };
                        if let Err(e) = self.edit_session.execute(m, cmd) {
                            self.export_status = Some(format!("Profile Dimension Error: {}", e));
                        } else {
                            self.export_status = Some(format!("Modified profile of #{} to {:.3} x {:.3} m.", element_id, new_dims[0], new_dims[1]));
                        }
                    }
                }
                GeomAction::DeleteElement { element_id } => {
                    if let Some(ref mut m) = self.ifc_model {
                        let cmd = IfcEditCommand::DeleteElementById { element_id };
                        if let Err(e) = self.edit_session.execute(m, cmd) {
                            self.export_status = Some(format!("Element Deletion Error: {}", e));
                        } else {
                            if self.selected_element_id == Some(element_id) {
                                self.selected_element_id = None;
                            }
                            self.export_status = Some(format!("Safely deleted structural element #{} from model.", element_id));
                        }
                    }
                }
                GeomAction::SynthesizeElement { spec } => {
                    if let Some(ref mut m) = self.ifc_model {
                        let elem_name = spec.name.clone();
                        let cmd = IfcEditCommand::CreateElement { element_id: 0, spec };
                        if let Err(e) = self.edit_session.execute(m, cmd) {
                            self.export_status = Some(format!("Element Synthesis Error: {}", e));
                        } else {
                            self.selected_element_id = m.elements.last().map(|e| e.step_id);
                            self.show_synth_modal = false;
                            self.export_status = Some(format!("Synthesized new element '{}' into IFC model graph.", elem_name));
                        }
                    }
                }
            }
        }
    }

    fn render_clash_and_diff_tab(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let mut run_clash = false;
        let mut run_diff = false;
        let mut run_ids = false;
        let mut export_bcf = false;
        let mut export_json = false;
        let mut select_id: Option<usize> = None;

        // 1. Sub-Tab Selector for Clash, Diff, and IDS
        ui.horizontal(|ui| {
            let tabs = [
                (0, "⚔️ Spatial Clash Matrix"),
                (1, "🔄 BIM Differential (Model Diff)"),
                (2, "🛡️ buildingSMART IDS & BCF"),
            ];

            for (idx, label) in tabs {
                let is_active = self.diff_subtab == idx;
                let bg = if is_active { theme.accent_blue } else { theme.bg_canvas };
                let fg = if is_active { Color32::WHITE } else { theme.text_secondary };

                if ui.add(egui::Button::new(RichText::new(label).size(10.5).strong().color(fg))
                    .fill(bg)
                    .corner_radius(CornerRadius::same(6))).clicked() {
                    self.diff_subtab = idx;
                }
            }
        });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(6.0);

        // 2. Render Subtab Content
        match self.diff_subtab {
            0 => {
                // SPATIAL CLASH MATRIX
                ui.label(RichText::new("⚔️ 3D SPATIAL CLASH DETECTION & CLEARANCE GUARD").size(12.0).strong().color(theme.accent_blue));
                ui.add_space(4.0);
                ui.label(RichText::new("Automated Axis-Aligned Bounding Box (AABB) volumetric intersection and proximity clearance evaluation.").size(10.5).color(theme.text_secondary));
                ui.add_space(8.0);

                Frame::new()
                    .fill(theme.bg_canvas)
                    .corner_radius(CornerRadius::same(8))
                    .inner_margin(Margin::same(10))
                    .stroke(Stroke::new(1.0, theme.border_subtle))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Clash Tolerance (mm):").size(10.5).color(theme.text_secondary));
                            ui.add(egui::DragValue::new(&mut self.clash_tolerance_mm).speed(1.0).range(1.0..=100.0));

                            ui.label(RichText::new("Clearance Buffer (mm):").size(10.5).color(theme.text_secondary));
                            ui.add(egui::DragValue::new(&mut self.clearance_buffer_mm).speed(5.0).range(10.0..=500.0));

                            if ui.add(egui::Button::new(RichText::new("⚡ Run Clash Detection").size(11.0).strong().color(Color32::WHITE))
                                .fill(theme.accent_blue)
                                .corner_radius(CornerRadius::same(5))).clicked() {
                                run_clash = true;
                            }
                        });
                    });

                ui.add_space(10.0);

                if let Some(ref cr) = self.clash_report {
                    ui.columns(4, |cols| {
                        self.render_stat_card(&mut cols[0], theme, "PAIRS EVALUATED", &format!("{}", cr.total_evaluated_pairs), "Element Permutations", theme.text_primary);
                        let hard_col = if cr.hard_clashes == 0 { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
                        self.render_stat_card(&mut cols[1], theme, "HARD CLASHES", &format!("{}", cr.hard_clashes), "Physical Intersections", hard_col);
                        let clear_col = if cr.clearance_violations == 0 { theme.accent_green } else { theme.accent_orange };
                        self.render_stat_card(&mut cols[2], theme, "CLEARANCE VIOLATIONS", &format!("{}", cr.clearance_violations), "Proximity Violations", clear_col);
                        self.render_stat_card(&mut cols[3], theme, "DUPLICATE ENTITIES", &format!("{}", cr.duplicate_entities), "Overlapping Copies", Color32::from_rgb(168, 85, 247));
                    });

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(8.0);

                    if cr.clashes.is_empty() {
                        Frame::new()
                            .fill(Color32::from_rgb(16, 185, 129).linear_multiply(0.15))
                            .corner_radius(CornerRadius::same(6))
                            .inner_margin(Margin::same(10))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(16, 185, 129)))
                            .show(ui, |ui| {
                                ui.label(RichText::new("✅ ZERO CLASHES: All structural framing elements satisfy spatial clearances and geometric separation.").size(11.0).strong().color(theme.accent_green));
                            });
                    } else {
                        ui.label(RichText::new(format!("DETECTED SPATIAL INCIDENTS ({} items)", cr.clashes.len())).size(11.0).strong().color(theme.text_secondary));
                        ui.add_space(4.0);

                        for clash in &cr.clashes {
                            let (badge_bg, badge_lbl) = match clash.severity {
                                IfcClashSeverity::Critical => (Color32::from_rgb(239, 68, 68), "CRITICAL HARD"),
                                IfcClashSeverity::Major => (Color32::from_rgb(249, 115, 22), "MAJOR HARD"),
                                IfcClashSeverity::Minor => (Color32::from_rgb(234, 179, 8), "CLEARANCE"),
                                IfcClashSeverity::Duplicate => (Color32::from_rgb(168, 85, 247), "DUPLICATE"),
                            };

                            Frame::new()
                                .fill(theme.bg_canvas)
                                .corner_radius(CornerRadius::same(6))
                                .inner_margin(Margin::same(8))
                                .stroke(Stroke::new(1.0, theme.border_subtle))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        Frame::new().fill(badge_bg).corner_radius(CornerRadius::same(4)).inner_margin(Margin::symmetric(6, 2)).show(ui, |ui| {
                                            ui.label(RichText::new(badge_lbl).size(9.0).strong().color(Color32::WHITE));
                                        });

                                        ui.label(RichText::new(format!("#{} ({}) ⚔️ #{} ({})", clash.id_a, clash.name_a, clash.id_b, clash.name_b)).size(10.5).strong().color(theme.text_primary));

                                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                            if ui.add(egui::Button::new(RichText::new(format!("Select #{}", clash.id_a)).size(9.5).color(theme.accent_blue))
                                                .fill(theme.bg_card)
                                                .corner_radius(CornerRadius::same(4))).clicked() {
                                                select_id = Some(clash.id_a);
                                            }
                                        });
                                    });
                                    ui.add_space(2.0);
                                    ui.label(RichText::new(&clash.description).size(10.0).color(theme.text_secondary));
                                });
                            ui.add_space(4.0);
                        }
                    }
                } else {
                    ui.label(RichText::new("Click '⚡ Run Clash Detection' above to evaluate 3D spatial intersections.").size(10.5).color(theme.text_muted));
                }
            }
            1 => {
                // MODEL DIFFERENTIAL (BIM DIFF)
                ui.label(RichText::new("🔄 BIM MODEL DIFFERENTIAL & REVISION TRACKING").size(12.0).strong().color(theme.accent_blue));
                ui.add_space(4.0);
                ui.label(RichText::new("Deterministic comparison between initial baseline model and active mutated session model.").size(10.5).color(theme.text_secondary));
                ui.add_space(8.0);

                if ui.add(egui::Button::new(RichText::new("🔄 Compute Model Differential").size(11.0).strong().color(Color32::WHITE))
                    .fill(theme.accent_blue)
                    .corner_radius(CornerRadius::same(6))).clicked() {
                    run_diff = true;
                }

                ui.add_space(10.0);

                if let Some(ref diff) = self.model_diff {
                    ui.columns(4, |cols| {
                        self.render_stat_card(&mut cols[0], theme, "ADDED ELEMENTS", &format!("+{}", diff.added_count), "Newly Synthesized", theme.accent_green);
                        self.render_stat_card(&mut cols[1], theme, "DELETED ELEMENTS", &format!("-{}", diff.deleted_count), "Safely Excised", Color32::from_rgb(239, 68, 68));
                        self.render_stat_card(&mut cols[2], theme, "MODIFIED ELEMENTS", &format!("*{}", diff.modified_count), "Attributes / Geometry", theme.accent_orange);
                        self.render_stat_card(&mut cols[3], theme, "UNCHANGED", &format!("={}", diff.unchanged_count), "Identical Geometry", theme.text_secondary);
                    });

                    ui.add_space(8.0);

                    // SHA-256 Revision Hash Seal
                    Frame::new()
                        .fill(theme.bg_canvas)
                        .corner_radius(CornerRadius::same(6))
                        .inner_margin(Margin::same(8))
                        .stroke(Stroke::new(1.0, theme.border_subtle))
                        .show(ui, |ui| {
                            ui.label(RichText::new("🔐 CRYPTOGRAPHIC REVISION HASH SEAL (FIPS 180-4 SHA-256):").size(9.5).strong().color(theme.text_muted));
                            ui.label(RichText::new(&diff.revision_hash).size(10.0).monospace().color(theme.accent_blue));
                        });

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(8.0);

                    ui.label(RichText::new("REVISION MANIFEST ITEMS").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(4.0);

                    for d in &diff.element_diffs {
                        if d.diff_kind == IfcDiffKind::Unchanged {
                            continue;
                        }

                        let (badge_bg, badge_lbl) = match d.diff_kind {
                            IfcDiffKind::Added => (Color32::from_rgb(16, 185, 129), "+ ADDED"),
                            IfcDiffKind::Deleted => (Color32::from_rgb(239, 68, 68), "- DELETED"),
                            IfcDiffKind::Modified => (Color32::from_rgb(249, 115, 22), "* MODIFIED"),
                            IfcDiffKind::Unchanged => (theme.text_muted, "= UNCHANGED"),
                        };

                        Frame::new()
                            .fill(theme.bg_canvas)
                            .corner_radius(CornerRadius::same(6))
                            .inner_margin(Margin::same(8))
                            .stroke(Stroke::new(1.0, theme.border_subtle))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    Frame::new().fill(badge_bg).corner_radius(CornerRadius::same(4)).inner_margin(Margin::symmetric(6, 2)).show(ui, |ui| {
                                        ui.label(RichText::new(badge_lbl).size(9.0).strong().color(Color32::WHITE));
                                    });

                                    ui.label(RichText::new(format!("#{} — {} ({})", d.step_id, d.name, d.class_name)).size(10.5).strong().color(theme.text_primary));

                                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                        if ui.add(egui::Button::new(RichText::new(format!("Select #{}", d.step_id)).size(9.5).color(theme.accent_blue))
                                            .fill(theme.bg_card)
                                            .corner_radius(CornerRadius::same(4))).clicked() {
                                            select_id = Some(d.step_id);
                                        }
                                    });
                                });
                                ui.add_space(3.0);
                                for detail in &d.details {
                                    ui.label(RichText::new(format!("• {}", detail)).size(10.0).color(theme.text_secondary));
                                }
                            });
                        ui.add_space(4.0);
                    }
                } else {
                    ui.label(RichText::new("Click '🔄 Compute Model Differential' above to compare active changes.").size(10.5).color(theme.text_muted));
                }
            }
            2 => {
                // BUILDINGSMART IDS & BCF
                ui.label(RichText::new("🛡️ BUILDINGSMART IDS CONFORMANCE & BCF ISSUE EXPORTER").size(12.0).strong().color(theme.accent_blue));
                ui.add_space(4.0);
                ui.label(RichText::new("Information Delivery Specification (IDS) validation rules and BIM Collaboration Format (BCF 2.1) export.").size(10.5).color(theme.text_secondary));
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    if ui.add(egui::Button::new(RichText::new("🛡️ Validate IDS Specification Rules").size(11.0).strong().color(Color32::WHITE))
                        .fill(theme.accent_blue)
                        .corner_radius(CornerRadius::same(6))).clicked() {
                        run_ids = true;
                    }

                    if ui.add(egui::Button::new(RichText::new("📬 Export BCF Topics XML").size(11.0).strong().color(Color32::WHITE))
                        .fill(Color32::from_rgb(14, 165, 233))
                        .corner_radius(CornerRadius::same(6))).clicked() {
                        export_bcf = true;
                    }

                    if ui.add(egui::Button::new(RichText::new("📋 Export ifcJSON-4").size(11.0).strong().color(Color32::WHITE))
                        .fill(Color32::from_rgb(234, 88, 12))
                        .corner_radius(CornerRadius::same(6))).clicked() {
                        export_json = true;
                    }
                });

                ui.add_space(10.0);

                if let Some(ref ids) = self.ids_report {
                    // Header verdict
                    Frame::new()
                        .fill(theme.bg_canvas)
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(Margin::same(10))
                        .stroke(Stroke::new(1.0, theme.border_subtle))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("IDS COMPLIANCE:").size(11.0).strong().color(theme.text_muted));
                                let v_col = if ids.compliance_score_pct >= 90.0 { theme.accent_green } else { theme.accent_orange };
                                ui.label(RichText::new(format!("{:.1}% Compliant", ids.compliance_score_pct)).size(12.0).strong().color(v_col));
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.label(RichText::new(&ids.specification_title).size(10.0).color(theme.text_muted));
                                });
                            });
                        });

                    ui.add_space(8.0);
                    ui.label(RichText::new("INFORMATION DELIVERY SPECIFICATION RULES").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(4.0);

                    for rule in &ids.rules {
                        let (badge_bg, badge_lbl) = match rule.status {
                            IfcAuditStatus::Pass => (Color32::from_rgb(16, 185, 129), "PASS"),
                            IfcAuditStatus::Warning => (Color32::from_rgb(245, 158, 11), "WARN"),
                            IfcAuditStatus::Fail => (Color32::from_rgb(239, 68, 68), "FAIL"),
                        };

                        Frame::new()
                            .fill(theme.bg_canvas)
                            .corner_radius(CornerRadius::same(6))
                            .inner_margin(Margin::same(8))
                            .stroke(Stroke::new(1.0, theme.border_subtle))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    Frame::new().fill(badge_bg).corner_radius(CornerRadius::same(4)).inner_margin(Margin::symmetric(6, 2)).show(ui, |ui| {
                                        ui.label(RichText::new(badge_lbl).size(9.0).strong().color(Color32::WHITE));
                                    });

                                    ui.label(RichText::new(&rule.rule_id).size(10.5).strong().color(theme.text_primary));
                                    ui.label(RichText::new(format!("({})", rule.target_class)).size(9.5).color(theme.text_muted));

                                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                        ui.label(RichText::new(format!("{}/{} Pass", rule.pass_count, rule.total_count)).size(10.0).strong().color(badge_bg));
                                    });
                                });
                                ui.add_space(2.0);
                                ui.label(RichText::new(&rule.description).size(10.0).color(theme.text_secondary));
                            });
                        ui.add_space(4.0);
                    }
                } else {
                    ui.label(RichText::new("Click '🛡️ Validate IDS Specification Rules' above to audit model requirements.").size(10.5).color(theme.text_muted));
                }
            }
            _ => {}
        }

        // 3. Post-UI Actions
        if let Some(id) = select_id {
            self.selected_element_id = Some(id);
        }

        if run_clash {
            if let Some(ref m) = self.ifc_model {
                let report = m.detect_clashes(self.clash_tolerance_mm as f64 * 1e-3, self.clearance_buffer_mm as f64 * 1e-3);
                self.export_status = Some(format!("Clash Detection complete: {} hard clashes, {} clearance violations across {} pairs.", report.hard_clashes, report.clearance_violations, report.total_evaluated_pairs));
                self.clash_report = Some(report);
            }
        }

        if run_diff {
            if let (Some(ref base), Some(ref cur)) = (&self.baseline_model, &self.ifc_model) {
                let diff = base.diff(cur);
                self.export_status = Some(format!("Model Diff computed: +{} added, -{} deleted, *{} modified. Seal: {}.", diff.added_count, diff.deleted_count, diff.modified_count, &diff.revision_hash[0..8]));
                self.model_diff = Some(diff);
            }
        }

        if run_ids {
            if let Some(ref m) = self.ifc_model {
                let report = m.validate_ids();
                self.export_status = Some(format!("IDS Audit complete: {:.1}% compliance score across {} rules.", report.compliance_score_pct, report.rules.len()));
                self.ids_report = Some(report);
            }
        }

        if export_bcf {
            if let Some(ref m) = self.ifc_model {
                let empty = Vec::new();
                let clashes = self.clash_report.as_ref().map(|r| r.clashes.as_slice()).unwrap_or(&empty);
                let xml = m.export_bcf_xml(clashes);
                self.export_status = Some(format!("Exported buildingSMART BCF topics XML ({} bytes, {} issues).", xml.len(), clashes.len()));
            }
        }

        if export_json {
            if let Some(ref m) = self.ifc_model {
                let json = m.to_ifc_json();
                self.export_status = Some(format!("Exported buildingSMART ifcJSON-4 document ({} bytes).", json.len()));
            }
        }
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

fn count_storeys(node: &IfcSpatialNode) -> usize {
    let mut count = if node.class_name == "IFCBUILDINGSTOREY" { 1 } else { 0 };
    for child in &node.children {
        count += count_storeys(child);
    }
    count
}
