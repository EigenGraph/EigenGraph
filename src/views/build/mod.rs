//! views/build/mod.rs — Pillar 5 (3D BIM): Clean BIM Spatial Assembly & 360-deg CSG B-Rep Viewport.
//! Features a True 360° Interactive 3D Turntable/Orbit Viewport, Solid Profile Extrusions,
//! Depth-Sorted Shaded Rendering (Painter's Algorithm), and Stress/ARI Gradient Contours.

pub mod bottom_deck;
pub mod clash_bcf;
pub mod ribbon;
pub mod sidebar;
pub mod viewport_3d;

use egui::{Align, Align2, Color32, CornerRadius, Frame, Layout, Margin, Pos2, RichText, ScrollArea, Sense, Stroke, Ui, Vec2};
use crate::models::{DatabaseComponent, SidebarTab, StructuralMember};
use crate::features::{FeatureId, FeatureRegistry};
use crate::theme::MacTheme;
use crate::ui_kit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelArchetype {
    BimCraftBuilding3D,      // BIMCraft LOD 350 Relational Building (CSG Walls, WF Mesh, Slabs & Rebar)
    DiagridTower3D,          // 48-Storey Diagrid Skyscraper IFC
    TrussBridge3D,           // 3D Space Truss Arch Bridge
    SpaceDome3D,             // Double-Layer Reticulated Dome
    WarrenTruss2D,           // Warren 24m Standard
    DoubleLayerSpaceTruss3D, // 3D Double-Layer Space Truss (MERO Octet Lattice)
    GeodesicDome3D,          // 3D Schwedler Geodesic Dome
    HyparRuledShell3D,       // 3D Hyperbolic Paraboloid Ruled Shell
    MultiStoryTower3D,       // 3D Multi-Story Building Frame with Core Walls
    PrestressedGirderBridge3D, // 3D Prestressed Post-Tensioned Concrete Bridge Span
    CoupledShearWallCore3D,  // 3D Coupled Special Structural Shear Wall Core Tower
    BaseIsolatedHospital3D,  // 3D Base-Isolated Essential Facility (8-Story Hospital with Base Isolators)
    TimeHistoryTower3D,      // 3D 16-Story NL-THA Seismic Test Tower Archetype
    FemaP58HospitalArchetype, // 3D 10-Story FEMA P-58 Resilience & Loss Assessment Archetype
    BlastResistantBunker3D,  // 3D Hardened Protective Blast Bunker & Standoff Barrier Archetype (UFC 3-340-02)
    FireCompartmentFrame3D,  // 3D Multi-Bay Steel Frame with Localized Fire Compartment (AISC 360-16 / EN 1993-1-2)
    OpenBimIfcModel3D,       // 3D OpenBIM Residential Duplex Villa parsed from IFC4 STEP SPF
}

impl ModelArchetype {
    pub fn label(&self) -> &'static str {
        match self {
            Self::BimCraftBuilding3D => "🏗️ BIMCraft LOD350 (CSG + Rebar)",
            Self::OpenBimIfcModel3D => "🌐 OpenBIM Duplex Villa (IFC4)",
            Self::FireCompartmentFrame3D => "🔥 Fire Compartment Steel Frame (AISC/EC3)",
            Self::BlastResistantBunker3D => "🛡️ Blast Protective Bunker (UFC 3-340-02)",
            Self::FemaP58HospitalArchetype => "🏥 FEMA P-58 Hospital (10-Story)",
            Self::BaseIsolatedHospital3D => "🛡️ Base-Isolated Hospital (8-Story)",
            Self::TimeHistoryTower3D => "🌊 NL-THA Tower (16-Story)",
            Self::DiagridTower3D => "🏢 NVW_DCR-LOD300 (Eng-CON Frame)",
            Self::TrussBridge3D => "🧱 NVW_DCR-LOD200 (Eng-CON-1 & 2)",
            Self::SpaceDome3D => "🔧 NVW_DCR-LOD300 (Eng-HVAC & Roof)",
            Self::WarrenTruss2D => "📐 NVW_DCR-LOD100 (Arch Massing)",
            Self::DoubleLayerSpaceTruss3D => "🕸️ Space Truss (24m MERO Octet)",
            Self::GeodesicDome3D => "🌐 Geodesic Dome (18m Lattice)",
            Self::HyparRuledShell3D => "🎪 Hypar Umbrella (16m Saddle)",
            Self::MultiStoryTower3D => "🏢 12-Story Tower (Core Wall)",
            Self::PrestressedGirderBridge3D => "🌉 Prestressed Bridge (30m PT Girder)",
            Self::CoupledShearWallCore3D => "🏢 Coupled Shear Wall Core (12-Story)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Render3DMode {
    CsgSolidBrep,     // True 3D CSG B-Rep Triangulated Mesh (Boolean Openings, WF Flanges & Rebar)
    SolidShaded,      // 3D Extruded Shaded Polygonal Beams with Lighting
    Wireframe,        // Crisp Vector Wireframe Lines
    AxialForceNFD,    // Normal / Axial Force Diagram Envelope (Tension red / Compression blue)
    ShearForceSFD,    // Shear Force Diagram Envelope (Green stepped polygon)
    BendingMomentBMD, // Bending Moment Diagram Envelope (Amber parabolic curve with peak callouts)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BimRibbonTab {
    Architecture,
    Structure,
    MepSystems,
    CivilInfra,
    AnalysisFea,
    Time4D,
    Cost5D,
    Sustainability6D,
    FacilityTwin7D,
    CoordinationAudit,
    OpenBimSpec,
    SheetsDed,
}

impl BimRibbonTab {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Architecture => "🏛️ Architecture",
            Self::Structure => "🏗️ Structure",
            Self::MepSystems => "⚡ MEP Systems",
            Self::CivilInfra => "🌍 Civil & Terrain",
            Self::AnalysisFea => "🔬 FEA Solver",
            Self::Time4D => "⏱️ 4D Phased",
            Self::Cost5D => "💰 5D Cost",
            Self::Sustainability6D => "🌱 6D Green",
            Self::FacilityTwin7D => "🏢 7D Twin",
            Self::CoordinationAudit => "🛡️ Coordination & Clash",
            Self::OpenBimSpec => "🌐 OpenBIM",
            Self::SheetsDed => "📑 Sheets & DED",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BottomDeckTab {
    ScheduleQto,
    TimeSimulation4D,
    ClashManager,
    BimConsole,
    PointCloudDeviation,
}

impl BottomDeckTab {
    pub fn label(&self) -> &'static str {
        match self {
            Self::ScheduleQto => "📊 Live Schedules & BoQ (QTO)",
            Self::TimeSimulation4D => "⏱️ 4D Construction Simulation",
            Self::ClashManager => "⚔️ Clash & Coordination Deck",
            Self::BimConsole => "💻 BIM Terminal & Automation",
            Self::PointCloudDeviation => "📡 LiDAR Point Cloud & Deviation",
        }
    }
}

pub struct BuildView {
    pub active_sidebar_tab: SidebarTab,
    pub active_model: ModelArchetype,
    pub render_mode: Render3DMode,
    pub perspective: bool,
    pub selected_member_id: Option<usize>,
    pub simulated_eliminated_ids: Vec<usize>,
    pub color_mode: String,
    pub show_grid: bool,
    pub show_axes: bool,
    pub show_node_labels: bool,
    pub show_deformed_shape: bool,
    pub deformation_scale: f32,
    pub animate_vibration: bool,
    pub vibration_phase: f32,
    pub cam_yaw: f32,
    pub cam_pitch: f32,
    pub cam_zoom: f32,
    pub cam_pan: Vec2,
    pub bim_bus: crate::core::BimCommandBus,
    pub show_rebar_cage: bool,
    pub section_box_enabled: bool,
    pub section_box_max_z_m: f32,
    pub modify_status_msg: Option<String>,
    // Phase 26: Master BIM World-Class Suite
    pub active_ribbon_tab: BimRibbonTab,
    pub active_bottom_deck_tab: BottomDeckTab,
    pub bottom_deck_expanded: bool,
    pub active_tool_status: String,
    pub view_scale: String,
    pub lod_level: String,
    pub show_level_datums: bool,
    pub show_grid_bubbles: bool,
    pub show_sun_shadows: bool,
    pub temporary_isolate_active: bool,
    pub section_box_min_x: f32,
    pub section_box_max_x: f32,
    pub section_box_min_y: f32,
    pub section_box_max_y: f32,
    pub section_box_min_z: f32,
    pub simulation_4d_playing: bool,
    pub simulation_4d_day: usize,
    pub simulation_4d_speed: f32,
    pub bim_console_input: String,
    pub bim_console_logs: Vec<String>,
    pub clash_tolerance_mm: f64,
    pub clash_filter_discipline: String,
    pub selected_clash_id: Option<usize>,
    // Phase 29: Stateful Clash Detection & BCF 2.1 Coordination Hub
    pub clash_items: Vec<crate::models::ClashDetectionItem>,
    pub show_3d_clash_markers: bool,
    pub clash_filter_severity: Option<crate::models::ClashSeverity>,
    pub clash_last_run_label: String,
    pub bcf_topics: Vec<crate::models::BcfTopicRecord>,
    pub selected_bcf_topic_id: Option<usize>,
    pub show_new_bcf_form: bool,
    pub new_bcf_title: String,
    pub new_bcf_assignee: String,
    pub new_bcf_priority: crate::models::BcfPriority,
    pub bcf_comment_draft: String,
    pub schedule_search_query: String,
    pub schedule_category_filter: String,
    pub active_workset_visibility: [bool; 7],
    // Phase 9: Modeling & Spatial Geometry Suite
    pub wizard_space_truss_lx: f64,
    pub wizard_space_truss_ly: f64,
    pub wizard_space_truss_depth: f64,
    pub wizard_dome_radius: f64,
    pub wizard_dome_rings: usize,
    pub wizard_dome_sectors: usize,
    pub wizard_hypar_lx: f64,
    pub wizard_hypar_ly: f64,
    pub wizard_hypar_sagitta: f64,
    pub wizard_tower_stories: usize,
    pub wizard_tower_story_h: f64,
    pub wizard_tower_bays_x: usize,
    pub wizard_tower_bays_y: usize,
    pub section_preset_idx: usize,
    pub sec_dim_d: f64,
    pub sec_dim_bf: f64,
    pub sec_dim_tf: f64,
    pub sec_dim_tw: f64,
    pub draft_node_x: f64,
    pub draft_node_y: f64,
    pub draft_node_z: f64,
    pub draft_member_n1: usize,
    pub draft_member_n2: usize,
}

impl Default for BuildView {
    fn default() -> Self {
        Self {
            active_sidebar_tab: SidebarTab::Properties,
            active_model: ModelArchetype::BimCraftBuilding3D,
            render_mode: Render3DMode::CsgSolidBrep,
            perspective: true,
            selected_member_id: None,
            simulated_eliminated_ids: Vec::new(),
            color_mode: "ARI".to_string(),
            show_grid: true,
            show_axes: true,
            show_node_labels: true,
            show_deformed_shape: false,
            deformation_scale: 50.0,
            animate_vibration: false,
            vibration_phase: 0.0,
            cam_yaw: 0.65,
            cam_pitch: 0.38,
            cam_zoom: 1.15,
            cam_pan: Vec2::ZERO,
            bim_bus: crate::core::BimCommandBus::new(),
            show_rebar_cage: true,
            section_box_enabled: false,
            section_box_max_z_m: 6.5,
            modify_status_msg: None,
            // Phase 26 Defaults
            active_ribbon_tab: BimRibbonTab::Architecture,
            active_bottom_deck_tab: BottomDeckTab::ScheduleQto,
            bottom_deck_expanded: false,
            active_tool_status: "Ready • Select a BIM tool or ribbon category above to begin modeling.".to_string(),
            view_scale: "1:100".to_string(),
            lod_level: "LOD 350 (Fine BIM Detail)".to_string(),
            show_level_datums: true,
            show_grid_bubbles: true,
            show_sun_shadows: true,
            temporary_isolate_active: false,
            section_box_min_x: -15.0,
            section_box_max_x: 15.0,
            section_box_min_y: 0.0,
            section_box_max_y: 18.0,
            section_box_min_z: -15.0,
            simulation_4d_playing: false,
            simulation_4d_day: 45,
            simulation_4d_speed: 1.0,
            bim_console_input: String::new(),
            bim_console_logs: vec![
                "⚡ [BIM Kernel] Relational spatial graph initialized with LOD 350 topological constraints.".to_string(),
                "🌐 [OpenBIM] buildingSMART IFC4 Reference View & STEP parser active (ISO 10303-21).".to_string(),
                "🛡️ [Solibri Engine] Automated model quality checker loaded (Rule 101-208 enabled).".to_string(),
                "⚔️ [Clash Solver] 3D AABB spatial interference detector online (tol: 15.0mm).".to_string(),
            ],
            clash_tolerance_mm: 15.0,
            clash_filter_discipline: "All (Arch vs Struct vs MEP)".to_string(),
            selected_clash_id: None,
            clash_items: crate::models::ClashDetectionItem::default_clashes(),
            show_3d_clash_markers: true,
            clash_filter_severity: None,
            clash_last_run_label: "Baseline Federation Check".to_string(),
            bcf_topics: crate::models::BcfTopicRecord::default_topics(),
            selected_bcf_topic_id: Some(1),
            show_new_bcf_form: false,
            new_bcf_title: String::new(),
            new_bcf_assignee: "Structural Lead".to_string(),
            new_bcf_priority: crate::models::BcfPriority::High,
            bcf_comment_draft: String::new(),
            schedule_search_query: String::new(),
            schedule_category_filter: "All Elements".to_string(),
            active_workset_visibility: [true, true, true, true, true, true, true],
            // Phase 9 Defaults
            wizard_space_truss_lx: 24.0,
            wizard_space_truss_ly: 24.0,
            wizard_space_truss_depth: 2.0,
            wizard_dome_radius: 18.0,
            wizard_dome_rings: 6,
            wizard_dome_sectors: 16,
            wizard_hypar_lx: 16.0,
            wizard_hypar_ly: 16.0,
            wizard_hypar_sagitta: 3.0,
            wizard_tower_stories: 12,
            wizard_tower_story_h: 3.5,
            wizard_tower_bays_x: 4,
            wizard_tower_bays_y: 4,
            section_preset_idx: 0,
            sec_dim_d: 300.0,
            sec_dim_bf: 150.0,
            sec_dim_tf: 9.0,
            sec_dim_tw: 6.5,
            draft_node_x: 0.0,
            draft_node_y: 0.0,
            draft_node_z: 0.0,
            draft_member_n1: 0,
            draft_member_n2: 1,
        }
    }
}


impl BuildView {
    pub fn show(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        members: &[StructuralMember],
        db_components: &[DatabaseComponent],
        features: &mut FeatureRegistry,
    ) {
        let avail_w = ui.available_width();
        let deck_h = (MacTheme::viewport_size(ui.ctx()).y * 0.68).clamp(440.0, 680.0);
        if avail_w >= 780.0 {
            let panel_w = (avail_w * 0.30).clamp(250.0, 320.0);
            ui.horizontal(|ui| {
                ui.allocate_ui(egui::Vec2::new(panel_w, deck_h), |ui| {
                    self.render_unified_sidebar(
                        ui,
                        theme,
                        members,
                        db_components,
                        features,
                        Some(panel_w),
                    );
                });
                self.render_canvas(ui, theme, members, features, Some(deck_h));
            });
        } else {
            let stacked_canvas_h = (MacTheme::viewport_size(ui.ctx()).y * 0.48).clamp(260.0, 420.0);
            ui.vertical(|ui| {
                self.render_canvas(ui, theme, members, features, Some(stacked_canvas_h));
                ui.add_space(10.0);
                self.render_unified_sidebar(ui, theme, members, db_components, features, None);
                ui.add_space(24.0);
            });
        }
    }
}
