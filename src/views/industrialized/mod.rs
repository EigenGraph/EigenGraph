//! views/industrialized/mod.rs — Pillar 13: 10D BIM Industrialized Construction & Automation.
//! Integrates Industry 4.0 workflows: Prefabrication DfMA, Robotics, Autonomous Erection,
//! and Native OpenBIM IFC (IFC2x3 / IFC4 / IFC4.3) Reader, Mutator, Clash, Diff & IDS Studio.

pub mod automation_slots;
pub mod dfma_robotics;
pub mod ifc_inspector;
pub mod lifecycle_10d;
pub mod openbim_studio;

use std::collections::HashMap;
use egui::{Align, Color32, CornerRadius, Frame, Layout, Margin, Rect, RichText, ScrollArea, Sense, Stroke, Ui, Vec2};
use crate::core::{
    IfcAuditStatus, IfcClashReport, IfcClashSeverity, IfcDiffKind, IfcEditCommand, IfcEditSession,
    IfcElementCreationSpec, IfcElementSummary, IfcIdsReport, IfcModel, IfcModelDiff, IfcSpatialNode,
};
use crate::features::{FeatureId, FeatureRegistry};
use crate::models::{PrefabModuleUnit, PrefabSupplyStage, StructuralMember};
use crate::theme::MacTheme;
use crate::ui_kit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndustrializedSubTab {
    Lifecycle10DHub,
    ModularDfma,
    RoboticAutomation,
    AutomatedErection,
    OpenBimIfcStudio,
    PrecastLogisticsTolerance,
    Concrete3DPrinting,
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
        let initial_model = IfcModel::parse(&IfcModel::sample_villa_ifc4()).ok();
        Self {
            active_subtab: IndustrializedSubTab::Lifecycle10DHub,
            offsite_ratio_pct: 82.5,
            robotic_speed_mm_s: 14.5,
            export_status: None,

            ifc_model: initial_model.clone(),
            selected_element_id: Some(200),
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
    pub fn show(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        _members: &[StructuralMember],
        features: &mut FeatureRegistry,
    ) {
        ui.vertical(|ui| {
            Frame::new().inner_margin(Margin::symmetric(20, 12)).show(ui, |ui| {
                // 1. TOP SUBTABS SELECTOR
                self.render_header_bar(ui, theme);

                let mut dismiss_status = false;
                if let Some(ref status_msg) = self.export_status {
                    ui.add_space(10.0);
                    theme.card_frame().show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.label(
                                RichText::new("📦 OPENBIM IFC BUS:")
                                    .size(11.5)
                                    .strong()
                                    .color(theme.accent_green),
                            );
                            ui.label(RichText::new(status_msg).size(11.5).color(theme.text_primary));
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                if ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new("✕").size(11.0).color(theme.text_muted),
                                        )
                                        .fill(theme.bg_card)
                                        .corner_radius(CornerRadius::same(4)),
                                    )
                                    .clicked()
                                {
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
                    IndustrializedSubTab::OpenBimIfcStudio => {
                        self.render_openbim_ifc_studio(ui, theme, features)
                    }
                    IndustrializedSubTab::PrecastLogisticsTolerance => {
                        self.render_precast_logistics_tolerance(ui, theme, features)
                    }
                    IndustrializedSubTab::Concrete3DPrinting => {
                        self.render_concrete_3d_printing(ui, theme, features)
                    }
                }

                ui.add_space(64.0);
            });
        });
    }

    fn render_header_bar(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let tabs = [
            (IndustrializedSubTab::Lifecycle10DHub, "🌐 10D BIM Lifecycle Hub"),
            (IndustrializedSubTab::ModularDfma, "🏭 Prefabrication & DfMA"),
            (IndustrializedSubTab::RoboticAutomation, "🦾 Robotics & Automation"),
            (IndustrializedSubTab::AutomatedErection, "🏗️ Autonomous Crane & LiDAR"),
            (IndustrializedSubTab::OpenBimIfcStudio, "🌐 OpenBIM IFC Studio"),
            (IndustrializedSubTab::PrecastLogisticsTolerance, "🚛 Precast Hauling & Splice (10D-04)"),
            (IndustrializedSubTab::Concrete3DPrinting, "🖨️ 3DCP G-Code Slicer (10D-05)"),
        ];
        ui_kit::subtab_bar(ui, theme, "ind_subtab_bar", &mut self.active_subtab, &tabs);
    }

    pub(super) fn render_responsive_cards(
        &self,
        ui: &mut Ui,
        cards: &[(&str, String, String, Color32)],
        theme: &MacTheme,
    ) {
        ui_kit::responsive_grid_n(ui, cards.len(), 210.0, |ui, idx| {
            let (title, val, sub, col) = &cards[idx];
            self.render_stat_card(ui, theme, title, val, sub, *col);
        });
    }

    pub(super) fn render_stat_card(
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

pub(super) fn count_storeys(node: &IfcSpatialNode) -> usize {
    let mut count = if node.class_name == "IFCBUILDINGSTOREY" { 1 } else { 0 };
    for child in &node.children {
        count += count_storeys(child);
    }
    count
}
