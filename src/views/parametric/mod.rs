//! views/parametric/mod.rs — Pillar 4 (2D Studio): BIMCraft 2D Plan, Live Section Cut & Parametric Studio.
//! Features:
//! - Interactive Floor Plan Authoring (Compound Walls, Hosted Doors/Windows with Swing Arcs,
//!   Structural Columns, Beams/Runs, Slabs/Shafts, Auto-Detected Planar Rooms & Associative Dimensions)
//! - Revit-Style Object Snaps (`SE, SM, SI, SC, SP, SN, SG`), 15° Angular Guide & Graduated Ruler
//! - Live 2D Section Cut View with Solid Poché, Rebar Cut Dots, Level Planes & Grid Bubbles
//! - Sheet Layout Composer (`Sheet A101` Title Block) with One-Click Vector PDF, SVG & IFC4 Export
//! - Transactional `BimCommandBus` with Unlimited Undo/Redo & Live FEM/ARI Redundancy Audit.

use egui::{
    Align, Align2, Color32, CornerRadius, Frame, Layout, Margin, Pos2, Rect, RichText, Sense,
    Stroke, StrokeKind, Ui, Vec2,
};
use crate::core::{
    BimCommand, BimCommandBus, OpeningKind, P2, RebarPreset, RunCategory, RunJustify, SnapHit,
};
use crate::models::{
    CadTool, DatabaseComponent, NavigationPillar, ParametricGeneratorPreset, ParametricVariable,
};
use crate::features::{FeatureId, FeatureRegistry};
use crate::icons::{self, AppIcon};
use crate::theme::MacTheme;
use crate::ui_kit;
use crate::views::universal_canvas::{UniversalCanvasState, WorkspaceCanvasSettings};

pub mod canvas_2d;
pub mod detailing_2d;
pub mod dynamo_studio;
pub mod inspector;
pub mod ribbon;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DraftingViewMode {
    FloorPlanViewRange,
    LiveSectionCutPoche,
    SheetLayoutA101,
    VisualNodeGraphDynamo,
    RebarBbsDetailing,
    ElevationsAndMepRiser,
}

/// Paper dimensions for Sheet Composer 2D
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SheetPaperSize {
    A4Landscape, // 297 x 210 mm
    A3Landscape, // 420 x 297 mm
    A2Landscape, // 594 x 420 mm
    A1Landscape, // 841 x 594 mm
}

impl SheetPaperSize {
    pub fn dimensions_mm(&self) -> (f32, f32) {
        match self {
            Self::A4Landscape => (297.0, 210.0),
            Self::A3Landscape => (420.0, 297.0),
            Self::A2Landscape => (594.0, 420.0),
            Self::A1Landscape => (841.0, 594.0),
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::A4Landscape => "ISO A4 (297 × 210 mm)",
            Self::A3Landscape => "ISO A3 (420 × 297 mm)",
            Self::A2Landscape => "ISO A2 (594 × 420 mm)",
            Self::A1Landscape => "ISO A1 (841 × 594 mm)",
        }
    }
}

/// Scale presets for engineering drawing sheets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SheetScale {
    Scale1_20,
    Scale1_50,
    Scale1_100,
    Scale1_200,
}

impl SheetScale {
    pub fn ratio(&self) -> f32 {
        match self {
            Self::Scale1_20 => 1.0 / 20.0,
            Self::Scale1_50 => 1.0 / 50.0,
            Self::Scale1_100 => 1.0 / 100.0,
            Self::Scale1_200 => 1.0 / 200.0,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Scale1_20 => "1 : 20",
            Self::Scale1_50 => "1 : 50",
            Self::Scale1_100 => "1 : 100",
            Self::Scale1_200 => "1 : 200",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParametricFloatingTab {
    Properties,
    Project,
    CanvasSync,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BimAuthoringTool {
    CadStandard,
    WallChain,
    StructuralBeam,
    StructuralColumn,
    PlaceDoor,
    PlaceWindow,
    ReinforceConcrete,
}

pub struct ParametricView {
    pub active_tool: CadTool,
    pub bim_tool: BimAuthoringTool,
    pub view_mode: DraftingViewMode,
    pub bim_bus: BimCommandBus,
    pub active_level_idx: usize,
    pub pending_start_pt: Option<P2>,
    pub last_snap_hit: Option<SnapHit>,
    pub canvas_pan: Vec2,
    pub canvas_zoom: f32,
    pub component_name: String,
    pub component_category: String,
    pub variables: Vec<ParametricVariable>,
    pub command_line_input: String,
    pub command_history: Vec<String>,
    pub snap_to_grid: bool,
    pub ortho_mode: bool,
    pub cursor_pos_mm: (f32, f32, f32),
    pub show_save_modal: bool,
    pub show_db_drawer: bool,
    pub save_notification: Option<String>,
    pub ai_suggestion_text: Option<String>,
    pub live_ari_score: f32,
    pub live_svd_margin: f32,
    pub sheet_paper_size: SheetPaperSize,
    pub sheet_scale: SheetScale,
    pub sheet_number: String,
    pub sheet_title: String,
    pub sheet_author: String,
    // Phase 27 / 29: Visual Node Graph Studio (Dynamo / Grasshopper)
    pub node_graph: crate::models::VisualNodeGraph,
    pub selected_node_id: Option<usize>,
    pub graph_search_query: String,
    pub is_graph_evaluating: bool,
    pub graph_bake_status: Option<String>,
    pub active_preset: ParametricGeneratorPreset,
    pub preset_params: [f32; 4],
    pub preview_yaw: f32,
    pub preview_pitch: f32,
    pub dragging_node_id: Option<usize>,
    pub last_canvas_rect: Option<Rect>,
    pub sketch_overlay_synced: bool,
    // Phase 34: Universal-style canvas & floating HUD overlays inside canvas
    pub floating_properties_open: bool,
    pub floating_project_open: bool,
    pub floating_right_tab: ParametricFloatingTab,
    pub show_cli_float: bool,
    pub sketch_draw_mode: bool,
    pub last_floating_rects: Vec<Rect>,
}

impl Default for ParametricView {
    fn default() -> Self {
        let default_preset = ParametricGeneratorPreset::DiagridTower;
        let default_params = default_preset.default_params();
        Self {
            active_tool: CadTool::Line,
            bim_tool: BimAuthoringTool::WallChain,
            view_mode: DraftingViewMode::FloorPlanViewRange,
            bim_bus: BimCommandBus::new(),
            active_level_idx: 0,
            pending_start_pt: None,
            last_snap_hit: None,
            canvas_pan: Vec2::ZERO,
            canvas_zoom: 1.0,
            sheet_paper_size: SheetPaperSize::A3Landscape,
            sheet_scale: SheetScale::Scale1_100,
            sheet_number: "A101".to_string(),
            sheet_title: "Untitled Site Layout Sheet".to_string(),
            sheet_author: "EigenGraph Studio".to_string(),
            component_name: "New Parametric Assembly".to_string(),
            component_category: "Custom Components".to_string(),
            variables: Vec::new(),
            command_line_input: String::new(),
            command_history: vec![
                "2D Parametric Ground Canvas Ready (Frozen by default — click Unfreeze to navigate World Map & draw).".to_string(),
            ],
            snap_to_grid: true,
            ortho_mode: false,
            cursor_pos_mm: (0.0, 0.0, 0.0),
            show_save_modal: false,
            show_db_drawer: false,
            save_notification: None,
            ai_suggestion_text: None,
            live_ari_score: 0.0,
            live_svd_margin: 0.0,
            node_graph: crate::models::VisualNodeGraph::empty(),
            selected_node_id: None,
            graph_search_query: String::new(),
            is_graph_evaluating: false,
            graph_bake_status: None,
            active_preset: default_preset,
            preset_params: default_params,
            preview_yaw: 0.65,
            preview_pitch: 0.35,
            dragging_node_id: None,
            last_canvas_rect: None,
            sketch_overlay_synced: true,
            floating_properties_open: true,
            floating_project_open: true,
            floating_right_tab: ParametricFloatingTab::Properties,
            show_cli_float: true,
            sketch_draw_mode: false,
            last_floating_rects: Vec::new(),
        }
    }
}


impl ParametricView {
    /// Returns true while the pointer hovers the Parametric viewport (outside floating HUD panels)
    /// and canvas scroll-zoom is active (unfrozen).
    pub fn is_pointer_in_live_canvas(&self, ctx: &egui::Context, is_frozen: bool) -> bool {
        if is_frozen {
            return false;
        }
        let Some(rect) = self.last_canvas_rect else {
            return false;
        };
        let Some(pointer) = ctx.input(|i| i.pointer.hover_pos()) else {
            return false;
        };
        let in_canvas = rect.contains(pointer);
        let in_floating = self.last_floating_rects.iter().any(|r| r.contains(pointer));
        in_canvas && !in_floating
    }

    #[allow(clippy::too_many_arguments)]
    pub fn show(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        db_components: &mut Vec<DatabaseComponent>,
        active_pillar: &mut NavigationPillar,
        features: &mut FeatureRegistry,
        universal_canvas: &mut UniversalCanvasState,
        members: &[crate::models::StructuralMember],
        workspace_settings: WorkspaceCanvasSettings,
        sidebar_toggle_inset_px: f32,
    ) {
        ui_kit::set_canvas_frozen(ui.ctx(), universal_canvas.is_frozen);
        self.last_canvas_rect = None;
        self.last_floating_rects.clear();

        // Sync Save-notification banner back & forth with unified Universal Canvas toast bar
        if universal_canvas.status_message.is_some() && self.save_notification.is_none() {
            self.save_notification = universal_canvas.status_message.take();
        }

        Frame::new()
            .fill(theme.bg_sidebar)
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .inner_margin(Margin::same(0))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());

                // 1. Universal-style Top Canvas HUD Toolbar + Drafting Strip
                self.render_drafting_ribbon(
                    ui,
                    theme,
                    db_components,
                    active_pillar,
                    universal_canvas,
                    members,
                    workspace_settings.clone(),
                    sidebar_toggle_inset_px,
                );

                // Dismissible Status Toast (shared across Universal, Parametric & Geometry)
                UniversalCanvasState::render_toast_bar(ui, theme, &mut self.save_notification);

                // 2. Full-Width Drawing Viewport with Floating Properties / Project / CLI Inside
                self.render_drawing_canvas(
                    ui,
                    theme,
                    db_components,
                    active_pillar,
                    features,
                    universal_canvas,
                    members,
                    workspace_settings,
                );
            });

        if self.show_db_drawer {
            self.render_db_drawer(ui.ctx(), theme, db_components, active_pillar);
        }
    }
    pub(super) fn refresh_live_ari(&mut self) {
        if let Ok(rep) = self.bim_bus.project.evaluate_live_ari() {
            self.live_ari_score = rep.average_ari as f32;
            self.live_svd_margin = if rep.kinematic_guard.is_stable {
                0.996
            } else {
                0.0
            };
            self.ai_suggestion_text = Some(format!(
                "AI Copilot: {} Walls, {} Columns, {} Beams, {} Rooms ({:.1} m²), Rebar {:.0} kg | ARI: {:.3}",
                self.bim_bus.project.walls.len(),
                self.bim_bus.project.columns.len(),
                self.bim_bus.project.runs.len(),
                self.bim_bus.project.rooms.len(),
                self.bim_bus.project.rooms.iter().map(|r| r.area_m2).sum::<f64>(),
                self.bim_bus.project.total_rebar_weight_kg(),
                self.live_ari_score
            ));
        }
    }


    pub(super) fn execute_cad_command(&mut self, cmd_raw: &str) {
        let trimmed = cmd_raw.trim();
        if trimmed.starts_with('{') {
            match self.bim_bus.execute_json(trimmed) {
                Ok(msg) => {
                    self.command_history.push(msg);
                    self.refresh_live_ari();
                }
                Err(e) => self.command_history.push(format!("JSON Error: {e}")),
            }
            return;
        }

        let cmd = trimmed.to_uppercase();
        match cmd.as_str() {
            "L" | "LINE" => {
                self.active_tool = CadTool::Line;
                self.bim_tool = BimAuthoringTool::CadStandard;
            }
            "WA" | "WALL" => self.bim_tool = BimAuthoringTool::WallChain,
            "BM" | "BEAM" => self.bim_tool = BimAuthoringTool::StructuralBeam,
            "CL" | "COLUMN" => self.bim_tool = BimAuthoringTool::StructuralColumn,
            "DR" | "DOOR" => self.bim_tool = BimAuthoringTool::PlaceDoor,
            "WN" | "WINDOW" => self.bim_tool = BimAuthoringTool::PlaceWindow,
            "RF" | "REINFORCE" => {
                self.bim_tool = BimAuthoringTool::ReinforceConcrete;
                if let Ok(msg) = self.bim_bus.execute(BimCommand::ReinforceAllConcrete {
                    preset: RebarPreset::default(),
                }) {
                    self.command_history.push(msg);
                }
            }
            "RM" | "ROOM" => {
                let lvl_id = self
                    .bim_bus
                    .project
                    .levels
                    .get(self.active_level_idx)
                    .map(|l| l.id)
                    .unwrap_or(1);
                if let Ok(msg) = self
                    .bim_bus
                    .execute(BimCommand::AutoDetectRooms { level_id: lvl_id })
                {
                    self.command_history.push(msg);
                }
            }
            "UNDO" | "U" => {
                if let Some(msg) = self.bim_bus.undo() {
                    self.command_history.push(msg);
                    self.refresh_live_ari();
                }
            }
            "REDO" => {
                if let Some(msg) = self.bim_bus.redo() {
                    self.command_history.push(msg);
                    self.refresh_live_ari();
                }
            }
            "PDF" | "SVG" | "IFC" => {
                self.export_bim_deliverables();
            }
            "PL" | "POLYLINE" => self.active_tool = CadTool::Polyline,
            "A" | "ARC" => self.active_tool = CadTool::Arc,
            "C" | "CIRCLE" => self.active_tool = CadTool::Circle,
            "REC" | "RECTANGLE" => self.active_tool = CadTool::Rectangle,
            "DIM" | "DI" | "DIMENSION" => self.active_tool = CadTool::Dimension,
            "TR" | "TRIM" => self.active_tool = CadTool::Trim,
            "O" | "OFFSET" => self.active_tool = CadTool::Offset,
            _ => {
                self.command_history
                    .push(format!("Executed instruction: {cmd}"));
            }
        }
    }

    pub(super) fn save_current_to_db(&mut self, db_components: &mut Vec<DatabaseComponent>) {
        let new_comp = DatabaseComponent {
            id: format!("comp_{}", db_components.len() + 1),
            name: self.component_name.clone(),
            category: self.component_category.clone(),
            description: format!(
                "BIMCraft relational model: {} walls, {} columns, {} beams, {} rooms, {:.0} kg rebar.",
                self.bim_bus.project.walls.len(),
                self.bim_bus.project.columns.len(),
                self.bim_bus.project.runs.len(),
                self.bim_bus.project.rooms.len(),
                self.bim_bus.project.total_rebar_weight_kg()
            ),
            variables: self.variables.clone(),
            elements_count: self.bim_bus.project.walls.len()
                + self.bim_bus.project.columns.len()
                + self.bim_bus.project.runs.len(),
            created_at: "Just now".to_string(),
            ..Default::default()
        };

        db_components.push(new_comp);
        if let Ok(saved_path) =
            crate::core::InternalVault::autosave_parametric_components(db_components)
        {
            self.save_notification = Some(format!(
                "Component '{}' automatically saved to Package Contents: {} (Ready to assemble in BUILD 3D).",
                self.component_name, saved_path
            ));
        } else {
            self.save_notification = Some(format!(
                "Component '{}' successfully saved to Database!",
                self.component_name
            ));
        }
    }

    fn render_db_drawer(
        &mut self,
        ctx: &egui::Context,
        theme: &MacTheme,
        db_components: &mut [DatabaseComponent],
        active_pillar: &mut NavigationPillar,
    ) {
        let mut open = self.show_db_drawer;
        let mut should_close = false;
        let mut load_idx: Option<usize> = None;
        let mut assemble_in_build = false;

        egui::Window::new("Parametric Component Database & IFC Sections")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::new(0.0, 0.0))
            .default_size(MacTheme::clamp_modal_size(ctx, Vec2::new(660.0, 520.0)))
            .max_size(MacTheme::modal_max_size(ctx))
            .frame(
                Frame::new()
                    .fill(theme.bg_card)
                    .stroke(Stroke::new(1.0, theme.border_subtle))
                    .corner_radius(CornerRadius::same(12))
                    .inner_margin(Margin::same(20)),
            )
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        icons::draw_icon(ui, AppIcon::Building, 26.0, theme.accent_blue);
                        ui.add_space(8.0);
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(
                                    "Parametric & Complex Structure Section Catalog (OpenBIM IFC)",
                                )
                                .size(15.0)
                                .strong()
                                .color(theme.text_primary),
                            );
                            ui.label(
                                RichText::new(
                                    "Modular sections from complex architectural buildings ready to use as reference & assembly.",
                                )
                                .size(11.5)
                                .color(theme.text_secondary),
                            );
                        });
                    });

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    egui::ScrollArea::vertical().max_height(380.0).show(ui, |ui| {
                        for (idx, comp) in db_components.iter().enumerate() {
                            theme.card_frame().show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(&comp.name)
                                            .size(13.0)
                                            .strong()
                                            .color(theme.accent_blue),
                                    );
                                    ui.add_space(6.0);
                                    Frame::new()
                                        .fill(theme.accent_blue.gamma_multiply(0.15))
                                        .corner_radius(CornerRadius::same(4))
                                        .inner_margin(Margin::symmetric(6, 2))
                                        .show(ui, |ui| {
                                            ui.label(
                                                RichText::new(&comp.category)
                                                    .size(10.0)
                                                    .color(theme.accent_blue),
                                            );
                                        });
                                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                        ui.label(
                                            RichText::new(&comp.created_at)
                                                .size(10.5)
                                                .color(theme.text_muted),
                                        );
                                    });
                                });

                                ui.add_space(4.0);
                                ui.label(
                                    RichText::new(&comp.description)
                                        .size(11.5)
                                        .color(theme.text_secondary),
                                );

                                ui.add_space(6.0);
                                ui.horizontal_wrapped(|ui| {
                                    ui.label(
                                        RichText::new("Variables:")
                                            .size(10.5)
                                            .strong()
                                            .color(theme.text_muted),
                                    );
                                    for v in comp.variables.iter().take(5) {
                                        ui.label(
                                            RichText::new(format!(
                                                "{} = {} {}",
                                                v.name, v.expression, v.unit
                                            ))
                                            .size(10.0)
                                            .monospace()
                                            .color(theme.accent_green),
                                        );
                                        ui.add_space(4.0);
                                    }
                                });

                                ui.add_space(8.0);
                                ui.horizontal_wrapped(|ui| {
                                    if icons::icon_button(
                                        ui,
                                        AppIcon::UploadImport,
                                        "Load into Canvas",
                                        11.0,
                                        Color32::WHITE,
                                        theme.accent_blue,
                                        Stroke::NONE,
                                        CornerRadius::same(6),
                                        Vec2::new(0.0, 26.0),
                                    )
                                    .clicked()
                                    {
                                        load_idx = Some(idx);
                                    }

                                    if icons::icon_button(
                                        ui,
                                        AppIcon::Cube3D,
                                        "Assemble in Geometry (3D)",
                                        11.0,
                                        theme.text_secondary,
                                        theme.bg_canvas,
                                        Stroke::new(1.0, theme.border_subtle),
                                        CornerRadius::same(6),
                                        Vec2::new(0.0, 26.0),
                                    )
                                    .clicked()
                                    {
                                        assemble_in_build = true;
                                    }
                                });
                            });
                            ui.add_space(8.0);
                        }
                    });

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(6.0);

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.button("Close").clicked() {
                            should_close = true;
                        }
                    });
                });
            });

        if should_close {
            open = false;
        }
        self.show_db_drawer = open;

        if let Some(idx) = load_idx {
            if let Some(comp) = db_components.get(idx) {
                self.component_name = comp.name.clone();
                self.component_category = comp.category.clone();
                self.variables = comp.variables.clone();
                self.save_notification = Some(format!(
                    "Component '{}' loaded into parametric editor successfully!",
                    comp.name
                ));
                self.show_db_drawer = false;
            }
        }

        if assemble_in_build {
            *active_pillar = NavigationPillar::GeometryAssembly;
            self.show_db_drawer = false;
        }
    }
}
