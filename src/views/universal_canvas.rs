//! views/universal_canvas.rs — Unified Synchronized 15-Pillar Drawing Canvas & Single-File Project Archive.
//! Provides a canvas-first interactive 2D drafting & domain-overlay workspace across all 15 sidebar
//! navigation modules, featuring a Freeze Canvas guard for uninterrupted vertical page scrolling,
//! a floating collapsible domain inspector, and tamper-evident single-file `.eigen.json` sync.

use egui::{
    Align, Align2, Color32, CornerRadius, Frame, Layout, Margin, Pos2, Rect, RichText, Sense,
    Stroke, StrokeKind, Ui, Vec2,
};
use serde::{Deserialize, Serialize};

use crate::core::{BimProject, DiskExporter, Sha256Hasher};
use crate::icons::{self, AppIcon};
use crate::models::{NavigationPillar, ParametricVariable, StructuralMember};
use crate::theme::{MacTheme, ThemeMode};
use crate::ui_kit;

/// Interactive drafting and markup tools available across every sidebar module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UniversalCanvasTool {
    SelectPan,
    Freehand,
    Line,
    Rectangle,
    Circle,
    Dimension,
    Callout,
    Eraser,
}

impl UniversalCanvasTool {
    pub const ALL: [Self; 8] = [
        Self::SelectPan,
        Self::Freehand,
        Self::Line,
        Self::Rectangle,
        Self::Circle,
        Self::Dimension,
        Self::Callout,
        Self::Eraser,
    ];

    pub fn app_icon(self) -> AppIcon {
        match self {
            Self::SelectPan => AppIcon::CursorPan,
            Self::Freehand => AppIcon::Pencil,
            Self::Line => AppIcon::LineSeg,
            Self::Rectangle => AppIcon::RectShape,
            Self::Circle => AppIcon::CircleShape,
            Self::Dimension => AppIcon::DimensionRuler,
            Self::Callout => AppIcon::CommentNote,
            Self::Eraser => AppIcon::Eraser,
        }
    }

    pub fn short_label(self) -> &'static str {
        match self {
            Self::SelectPan => "Pan",
            Self::Freehand => "Draw",
            Self::Line => "Line",
            Self::Rectangle => "Rect",
            Self::Circle => "Circle",
            Self::Dimension => "Dim",
            Self::Callout => "Note",
            Self::Eraser => "Erase",
        }
    }

    pub fn tooltip(self) -> &'static str {
        match self {
            Self::SelectPan => "Pan canvas with primary drag (or right-drag in any tool)",
            Self::Freehand => "Freehand engineering sketch synced across all 15 modules",
            Self::Line => "Draw straight vector line segment (click-drag)",
            Self::Rectangle => "Draw rectangular zone / bay boundary (click-drag)",
            Self::Circle => "Draw radial envelope / circle (drag center to radius)",
            Self::Dimension => "Measure and pin world distance in millimetres",
            Self::Callout => "Click canvas to place text annotation callout",
            Self::Eraser => "Click any user-drawn object to remove it",
        }
    }
}

/// Geometry primitive stored in world coordinates (millimetres) and synchronized across all modules.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CanvasShapeKind {
    Freehand { points_mm: Vec<[f64; 2]> },
    Line { start_mm: [f64; 2], end_mm: [f64; 2] },
    Rectangle { min_mm: [f64; 2], max_mm: [f64; 2] },
    Circle { center_mm: [f64; 2], radius_mm: f64 },
    Dimension { start_mm: [f64; 2], end_mm: [f64; 2] },
    Callout { pos_mm: [f64; 2], text: String },
}

/// Synchronized drawing entity tagged with its originating module layer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanvasEntity {
    pub id: usize,
    pub pillar_badge: String,
    pub pillar_name: String,
    pub kind: CanvasShapeKind,
    pub color_rgb: [u8; 3],
    pub stroke_width: f32,
}

/// Workspace and canvas preferences persisted inside the single-file archive.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceCanvasSettings {
    pub theme_mode: ThemeMode,
    pub canvas_height_ratio: f32,
    pub freeze_by_default: bool,
    pub show_grid: bool,
    pub snap_to_grid: bool,
    pub grid_spacing_mm: f64,
    pub show_bim_base_layer: bool,
    pub show_domain_overlay: bool,
    pub sync_all_layers: bool,
    pub show_floating_inspector: bool,
    pub show_ai_copilot: bool,
    pub unit_system: String,
    pub design_standard: String,
    pub solver_threads: usize,
    pub enable_svd_guard: bool,
    pub enable_anti_leakage: bool,
}

/// Unified single-file `.eigen.json` workspace archive synchronizing BIM, structural analysis,
/// parametric variables, all 15-module canvas drawings, and workspace settings with SHA-256 verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedWorkspaceArchive {
    pub format_version: String,
    pub engine_version: String,
    pub project_name: String,
    pub bim_project: BimProject,
    pub members: Vec<StructuralMember>,
    pub parametric_variables: Vec<ParametricVariable>,
    pub canvas_entities: Vec<CanvasEntity>,
    pub workspace_settings: WorkspaceCanvasSettings,
    #[serde(default)]
    pub world_origin_lat_lon: Option<(f64, f64)>,
    #[serde(default)]
    pub map_center_lat_lon: Option<(f64, f64)>,
    #[serde(default)]
    pub map_zoom: Option<f32>,
    pub sha256_seal: String,
}

#[derive(Serialize)]
struct UnifiedArchivePayloadRef<'a> {
    project_name: &'a str,
    bim_project: &'a BimProject,
    members: &'a [StructuralMember],
    parametric_variables: &'a [ParametricVariable],
    canvas_entities: &'a [CanvasEntity],
    workspace_settings: &'a WorkspaceCanvasSettings,
}

impl UnifiedWorkspaceArchive {
    /// Serializes the synchronized workspace into a single SHA-256 sealed JSON document.
    pub fn to_pretty_json(&self) -> Result<String, String> {
        let payload = UnifiedArchivePayloadRef {
            project_name: &self.project_name,
            bim_project: &self.bim_project,
            members: &self.members,
            parametric_variables: &self.parametric_variables,
            canvas_entities: &self.canvas_entities,
            workspace_settings: &self.workspace_settings,
        };
        let canonical = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
        let seal = Sha256Hasher::digest_hex(canonical.as_bytes());
        let mut sealed = self.clone();
        sealed.sha256_seal = seal;
        serde_json::to_string_pretty(&sealed).map_err(|e| e.to_string())
    }

    /// Parses and cryptographically verifies a single-file `.eigen.json` workspace archive.
    pub fn from_verified_json(json_str: &str) -> Result<Self, String> {
        let archive: Self = serde_json::from_str(json_str).map_err(|e| e.to_string())?;
        let payload = UnifiedArchivePayloadRef {
            project_name: &archive.project_name,
            bim_project: &archive.bim_project,
            members: &archive.members,
            parametric_variables: &archive.parametric_variables,
            canvas_entities: &archive.canvas_entities,
            workspace_settings: &archive.workspace_settings,
        };
        let canonical = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
        let computed = Sha256Hasher::digest_hex(canonical.as_bytes());
        if computed != archive.sha256_seal {
            return Err(
                "SHA-256 integrity seal mismatch: unified workspace file was modified or corrupted."
                    .to_string(),
            );
        }
        Ok(archive)
    }
}

/// Optional split export payload when the user explicitly requests per-module separation.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ModuleLayerSplitExport {
    format_version: String,
    pillar_badge: String,
    pillar_name: String,
    entities: Vec<CanvasEntity>,
}

const PALETTE_SWATCHES: [(&str, [u8; 3]); 6] = [
    ("Cyan", [56, 189, 248]),
    ("Blue", [59, 130, 246]),
    ("Green", [16, 185, 129]),
    ("Amber", [245, 158, 11]),
    ("Red", [239, 68, 68]),
    ("Slate", [226, 232, 240]),
];

/// Shared interactive drawing canvas state synchronized across all 15 sidebar navigation modules.
pub struct UniversalCanvasState {
    pub pan: Vec2,
    pub zoom: f32,
    pub is_frozen: bool,
    pub active_tool: UniversalCanvasTool,
    pub active_color_idx: usize,
    pub stroke_width: f32,
    pub show_grid: bool,
    pub snap_to_grid: bool,
    pub grid_spacing_mm: f64,
    pub show_bim_base_layer: bool,
    pub show_domain_overlay: bool,
    pub sync_all_layers: bool,
    pub inspector_open: bool,
    pub canvas_height_ratio: f32,
    pub callout_draft: String,
    pub entities: Vec<CanvasEntity>,
    pub next_entity_id: usize,
    pub undo_stack: Vec<Vec<CanvasEntity>>,
    pending_start_mm: Option<[f64; 2]>,
    active_freehand_mm: Vec<[f64; 2]>,
    pub cursor_world_mm: [f64; 2],
    pub cursor_lat_lon: (f64, f64),
    pub world_map: crate::core::gis::SlippyMapState,
    pub ground_origin_lat_lon: (f64, f64),
    pub ground_origin_anchored: bool,
    pub earth_search_query: String,
    pub measure_active: bool,
    pub measure_area_mode: bool,
    pub measure_points_lat_lon: Vec<(f64, f64)>,
    last_canvas_rect: Option<Rect>,
    last_inspector_rect: Option<Rect>,
    pub status_message: Option<String>,
    pub import_json_open: bool,
    pub import_json_buffer: String,
    pub pending_imported_settings: Option<WorkspaceCanvasSettings>,
    pub request_scroll_to_workbench: bool,
}

impl Default for UniversalCanvasState {
    fn default() -> Self {
        Self {
            pan: Vec2::ZERO,
            zoom: 1.0,
            // Phase 35: Frozen by default so vertical page scroll never zooms unintentionally; unfreeze to pan/draw on Earth map
            is_frozen: true,
            active_tool: UniversalCanvasTool::SelectPan,
            active_color_idx: 0,
            stroke_width: 2.2,
            show_grid: true,
            snap_to_grid: true,
            grid_spacing_mm: 500.0,
            show_bim_base_layer: true,
            show_domain_overlay: true,
            sync_all_layers: true,
            inspector_open: false,
            canvas_height_ratio: 0.82,
            callout_draft: "Note".to_string(),
            entities: Vec::new(),
            next_entity_id: 1,
            undo_stack: Vec::new(),
            pending_start_mm: None,
            active_freehand_mm: Vec::new(),
            cursor_world_mm: [0.0, 0.0],
            cursor_lat_lon: (15.0, 15.0),
            world_map: crate::core::gis::SlippyMapState::default(),
            ground_origin_lat_lon: (0.0, 0.0),
            ground_origin_anchored: false,
            earth_search_query: String::new(),
            measure_active: false,
            measure_area_mode: false,
            measure_points_lat_lon: Vec::new(),
            last_canvas_rect: None,
            last_inspector_rect: None,
            status_message: None,
            import_json_open: false,
            import_json_buffer: String::new(),
            pending_imported_settings: None,
            request_scroll_to_workbench: false,
        }
    }
}

impl UniversalCanvasState {
    /// Theme-aware canvas viewport background matching Universal Canvas across all modules.
    pub fn canvas_bg(theme: &MacTheme) -> Color32 {
        if theme.is_dark() {
            Color32::from_rgb(12, 15, 21)
        } else {
            Color32::from_rgb(248, 250, 253)
        }
    }

    /// Unified height calculation used by Universal Canvas, Parametric, and Geometry viewports.
    pub fn preferred_canvas_height(&self, ctx: &egui::Context) -> f32 {
        let vp = MacTheme::viewport_size(ctx);
        let is_mobile = vp.x < MacTheme::MOBILE_BREAKPOINT;
        (vp.y * self.canvas_height_ratio).clamp(if is_mobile { 380.0 } else { 540.0 }, 960.0)
    }

    /// Frosted translucent floating HUD overlay frame (matches Universal Canvas inspector).
    pub fn floating_panel_frame(theme: &MacTheme, is_overlay: bool) -> Frame {
        let fill = if is_overlay {
            if theme.is_dark() {
                Color32::from_rgba_premultiplied(20, 24, 34, 240)
            } else {
                Color32::from_rgba_premultiplied(255, 255, 255, 244)
            }
        } else {
            theme.bg_card
        };

        Frame::new()
            .fill(fill)
            .corner_radius(CornerRadius::same(10))
            .stroke(Stroke::new(1.2, theme.accent_blue.gamma_multiply(0.55)))
            .inner_margin(Margin::same(12))
    }

    /// Shared dismissible status toast strip rendered above the canvas viewport.
    pub fn render_toast_bar(ui: &mut Ui, theme: &MacTheme, status_message: &mut Option<String>) {
        if let Some(msg) = status_message.clone() {
            Frame::new()
                .fill(theme.accent_green.gamma_multiply(0.14))
                .stroke(Stroke::new(1.0, theme.accent_green.gamma_multiply(0.5)))
                .inner_margin(Margin::symmetric(12, 5))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        icons::icon_label(
                            ui,
                            AppIcon::CheckCircle,
                            13.0,
                            theme.accent_green,
                            msg,
                            11.0,
                            theme.accent_green,
                            false,
                        );
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if icons::icon_only_button(
                                ui,
                                AppIcon::Close,
                                theme.text_secondary,
                                Color32::TRANSPARENT,
                                Stroke::NONE,
                                CornerRadius::same(4),
                                Vec2::splat(20.0),
                                "Dismiss notification",
                            )
                            .clicked()
                            {
                                *status_message = None;
                            }
                        });
                    });
                });
        }
    }

    /// Shared Freeze Canvas toggle button across Universal Canvas, Parametric, and Geometry.
    pub fn render_freeze_button(
        ui: &mut Ui,
        theme: &MacTheme,
        is_frozen: &mut bool,
        height: f32,
    ) {
        let freeze_icon = if *is_frozen {
            AppIcon::Snowflake
        } else {
            AppIcon::Unlock
        };
        let freeze_label = if *is_frozen {
            "Frozen (Scroll Page)"
        } else {
            "Freeze Canvas"
        };
        let freeze_bg = if *is_frozen {
            theme.accent_blue
        } else {
            theme.bg_card
        };
        let freeze_fg = if *is_frozen {
            Color32::WHITE
        } else {
            theme.accent_orange
        };
        let freeze_stroke = Stroke::new(
            1.0,
            if *is_frozen {
                theme.accent_blue
            } else {
                theme.accent_orange.gamma_multiply(0.6)
            },
        );
        if icons::icon_button(
            ui,
            freeze_icon,
            freeze_label,
            10.5,
            freeze_fg,
            freeze_bg,
            freeze_stroke,
            CornerRadius::same(6),
            Vec2::new(0.0, height),
        )
        .on_hover_text(
            "Freeze canvas zoom/pan so vertical scrolling moves the page without zooming in or out",
        )
        .clicked()
        {
            *is_frozen = !*is_frozen;
            ui_kit::set_canvas_frozen(ui.ctx(), *is_frozen);
        }
    }

    /// Shared Freeze Canvas badge painted inside the viewport when scroll guard is active.
    pub fn paint_frozen_badge(
        painter: &egui::Painter,
        rect: Rect,
        theme: &MacTheme,
        bottom_offset_px: f32,
    ) {
        let badge_pos = Pos2::new(rect.left() + 14.0, rect.bottom() - bottom_offset_px);
        let text_rect = Rect::from_min_size(
            Pos2::new(badge_pos.x, badge_pos.y - 24.0),
            Vec2::new(280.0, 24.0),
        );
        painter.rect_filled(
            text_rect,
            CornerRadius::same(6),
            theme.accent_blue.gamma_multiply(0.22),
        );
        painter.rect_stroke(
            text_rect,
            CornerRadius::same(6),
            Stroke::new(1.0, theme.accent_blue.gamma_multiply(0.7)),
            StrokeKind::Inside,
        );
        let icon_rect = Rect::from_min_size(
            Pos2::new(text_rect.left() + 7.0, text_rect.center().y - 6.0),
            Vec2::splat(12.0),
        );
        icons::paint_icon(painter, icon_rect, AppIcon::Snowflake, theme.accent_blue);
        painter.text(
            Pos2::new(text_rect.left() + 24.0, text_rect.center().y),
            Align2::LEFT_CENTER,
            "CANVAS FROZEN • Scroll moves page vertically",
            egui::FontId::proportional(10.5),
            theme.text_primary,
        );
    }

    /// Masks outer sharp corners of an interactive canvas/map viewport to a smooth `radius`
    /// using spandrel triangles in `outer_bg`, then strokes a crisp rounded border over it.
    pub fn paint_rounded_viewport_border(
        painter: &egui::Painter,
        rect: Rect,
        radius: f32,
        outer_bg: Color32,
        stroke_color: Color32,
    ) {
        let r = radius.clamp(2.0, rect.width().min(rect.height()) * 0.45);
        let segs = 7usize;
        let corners = [
            // (outer_corner, arc_center, start_angle, end_angle)
            (
                Pos2::new(rect.left(), rect.top()),
                Pos2::new(rect.left() + r, rect.top() + r),
                std::f32::consts::PI,
                std::f32::consts::FRAC_PI_2 * 3.0,
            ),
            (
                Pos2::new(rect.right(), rect.top()),
                Pos2::new(rect.right() - r, rect.top() + r),
                std::f32::consts::FRAC_PI_2 * 3.0,
                std::f32::consts::TAU,
            ),
            (
                Pos2::new(rect.right(), rect.bottom()),
                Pos2::new(rect.right() - r, rect.bottom() - r),
                0.0,
                std::f32::consts::FRAC_PI_2,
            ),
            (
                Pos2::new(rect.left(), rect.bottom()),
                Pos2::new(rect.left() + r, rect.bottom() - r),
                std::f32::consts::FRAC_PI_2,
                std::f32::consts::PI,
            ),
        ];

        for (outer, center, a0, a1) in corners {
            let mut pts = Vec::with_capacity(segs + 2);
            pts.push(outer);
            for s in 0..=segs {
                let t = s as f32 / segs as f32;
                let ang = a0 + (a1 - a0) * t;
                pts.push(Pos2::new(center.x + r * ang.cos(), center.y + r * ang.sin()));
            }
            painter.add(egui::Shape::convex_polygon(pts, outer_bg, Stroke::NONE));
        }

        painter.rect_stroke(
            rect,
            CornerRadius::same(r.round() as u8),
            Stroke::new(1.0, stroke_color),
            StrokeKind::Inside,
        );
    }

    /// Returns `true` when the pointer hovers over the live, unfrozen canvas area (excluding the floating inspector).
    pub fn is_pointer_in_live_canvas(&self, ctx: &egui::Context) -> bool {
        if self.is_frozen {
            return false;
        }
        let Some(pointer) = ctx.input(|i| i.pointer.hover_pos()) else {
            return false;
        };
        let in_canvas = self.last_canvas_rect.is_some_and(|r| r.contains(pointer));
        let in_inspector = self.inspector_open
            && self
                .last_inspector_rect
                .is_some_and(|r| r.contains(pointer));
        in_canvas && !in_inspector
    }

    pub fn active_color(&self) -> Color32 {
        let [r, g, b] = PALETTE_SWATCHES
            .get(self.active_color_idx)
            .map(|(_, c)| *c)
            .unwrap_or([56, 189, 248]);
        Color32::from_rgb(r, g, b)
    }

    pub fn active_color_rgb(&self) -> [u8; 3] {
        PALETTE_SWATCHES
            .get(self.active_color_idx)
            .map(|(_, c)| *c)
            .unwrap_or([56, 189, 248])
    }

    /// Pushes current entity list to undo history before mutating.
    fn push_undo(&mut self) {
        self.undo_stack.push(self.entities.clone());
        if self.undo_stack.len() > 30 {
            self.undo_stack.remove(0);
        }
    }

    pub fn undo(&mut self) -> bool {
        if let Some(prev) = self.undo_stack.pop() {
            self.entities = prev;
            true
        } else {
            false
        }
    }

    pub fn clear_entities(&mut self, active_pillar: Option<NavigationPillar>) {
        self.push_undo();
        match active_pillar {
            Some(p) => {
                let badge = p.badge();
                self.entities.retain(|e| e.pillar_badge != badge);
            }
            None => self.entities.clear(),
        }
    }

    pub fn count_for_pillar(&self, pillar: NavigationPillar) -> usize {
        let badge = pillar.badge();
        self.entities
            .iter()
            .filter(|e| e.pillar_badge == badge)
            .count()
    }

    /// Builds a single synchronized workspace archive across all 15 modules.
    pub fn build_unified_archive(
        &self,
        bim_project: &BimProject,
        members: &[StructuralMember],
        variables: &[ParametricVariable],
        workspace_settings: WorkspaceCanvasSettings,
    ) -> UnifiedWorkspaceArchive {
        UnifiedWorkspaceArchive {
            format_version: "EIGEN-UNIFIED-1.0".to_string(),
            engine_version: env!("CARGO_PKG_VERSION").to_string(),
            project_name: bim_project.project_name.clone(),
            bim_project: bim_project.clone(),
            members: members.to_vec(),
            parametric_variables: variables.to_vec(),
            canvas_entities: self.entities.clone(),
            workspace_settings,
            world_origin_lat_lon: self
                .ground_origin_anchored
                .then_some(self.ground_origin_lat_lon),
            map_center_lat_lon: Some(self.world_map.center_lat_lon),
            map_zoom: Some(self.world_map.zoom_level),
            sha256_seal: String::new(),
        }
    }

    /// Exports the entire synchronized workspace as 1 unified `.eigen.json` file.
    pub fn export_unified_file(
        &mut self,
        bim_project: &BimProject,
        members: &[StructuralMember],
        variables: &[ParametricVariable],
        workspace_settings: WorkspaceCanvasSettings,
    ) {
        let archive =
            self.build_unified_archive(bim_project, members, variables, workspace_settings);
        match archive.to_pretty_json() {
            Ok(json) => match DiskExporter::write_export_file(
                "EigenGraph_Unified_Workspace.eigen.json",
                &json,
            ) {
                Ok(dest) => {
                    self.status_message = Some(format!(
                        "Synced 1-file workspace ({} drawings, {} members) -> {dest}",
                        self.entities.len(),
                        members.len()
                    ));
                }
                Err(err) => {
                    self.status_message = Some(format!("Workspace export failed: {err}"));
                }
            },
            Err(err) => {
                self.status_message = Some(format!("Serialization error: {err}"));
            }
        }
    }

    /// Applies a verified unified archive back into the active application state.
    pub fn apply_unified_archive(
        &mut self,
        json_str: &str,
        bim_project: &mut BimProject,
        members: &mut Vec<StructuralMember>,
        variables: &mut Vec<ParametricVariable>,
    ) -> Result<WorkspaceCanvasSettings, String> {
        let archive = UnifiedWorkspaceArchive::from_verified_json(json_str)?;
        self.push_undo();
        *bim_project = archive.bim_project;
        *members = archive.members;
        *variables = archive.parametric_variables;
        self.entities = archive.canvas_entities;
        self.next_entity_id = self
            .entities
            .iter()
            .map(|e| e.id)
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        self.is_frozen = archive.workspace_settings.freeze_by_default;
        self.canvas_height_ratio = archive
            .workspace_settings
            .canvas_height_ratio
            .clamp(0.45, 1.15);
        self.show_grid = archive.workspace_settings.show_grid;
        self.snap_to_grid = archive.workspace_settings.snap_to_grid;
        self.grid_spacing_mm = archive.workspace_settings.grid_spacing_mm.max(100.0);
        self.show_bim_base_layer = archive.workspace_settings.show_bim_base_layer;
        self.show_domain_overlay = archive.workspace_settings.show_domain_overlay;
        self.sync_all_layers = archive.workspace_settings.sync_all_layers;
        self.inspector_open = archive.workspace_settings.show_floating_inspector;
        if let Some(orig) = archive.world_origin_lat_lon {
            self.ground_origin_lat_lon = orig;
            self.ground_origin_anchored = true;
        }
        if let Some(center) = archive.map_center_lat_lon {
            self.world_map.set_center(center.0, center.1);
        }
        if let Some(zoom) = archive.map_zoom {
            self.world_map.zoom_level = zoom.clamp(1.0, 19.0);
        }
        self.status_message = Some(format!(
            "Imported verified 1-file workspace '{}' ({} canvas objects)",
            archive.project_name,
            self.entities.len()
        ));
        Ok(archive.workspace_settings)
    }

    /// Exports only the active module's entities when the user explicitly chooses to split files.
    pub fn export_split_module_file(&mut self, pillar: NavigationPillar) {
        let badge = pillar.badge();
        let layer_entities: Vec<CanvasEntity> = self
            .entities
            .iter()
            .filter(|e| e.pillar_badge == badge)
            .cloned()
            .collect();
        let payload = ModuleLayerSplitExport {
            format_version: "EIGEN-LAYER-SPLIT-1.0".to_string(),
            pillar_badge: badge.to_string(),
            pillar_name: pillar.clean_name().to_string(),
            entities: layer_entities,
        };
        match serde_json::to_string_pretty(&payload) {
            Ok(json) => {
                let filename = format!("EigenGraph_Layer_{badge}.json");
                match DiskExporter::write_export_file(&filename, &json) {
                    Ok(dest) => {
                        self.status_message = Some(format!(
                            "Split module layer [{badge}] exported -> {dest}"
                        ));
                    }
                    Err(err) => {
                        self.status_message = Some(format!("Split export failed: {err}"));
                    }
                }
            }
            Err(err) => {
                self.status_message = Some(format!("Split serialization failed: {err}"));
            }
        }
    }

    /// Exports synchronized canvas entities and BIM grids to vector SVG.
    pub fn export_svg(&self, bim_project: &BimProject, active_pillar: NavigationPillar) -> String {
        let mut svg = String::from(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-2000 -11500 16000 13500\" width=\"1200\" height=\"900\" style=\"background:#0f1117\">\n",
        );
        svg.push_str(&format!(
            "  <!-- EigenGraph Unified Canvas | Active Pillar: {} -->\n",
            active_pillar.clean_name()
        ));
        for g in &bim_project.grids {
            svg.push_str(&format!(
                "  <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#38bdf8\" stroke-opacity=\"0.35\" stroke-width=\"18\" stroke-dasharray=\"180,120\"/>\n",
                g.start.0, -g.start.1, g.end.0, -g.end.1
            ));
        }
        for w in &bim_project.walls {
            svg.push_str(&format!(
                "  <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#94a3b8\" stroke-width=\"{:.1}\"/>\n",
                w.start.0, -w.start.1, w.end.0, -w.end.1, w.thickness_mm.max(80.0)
            ));
        }
        for ent in &self.entities {
            if !self.sync_all_layers && ent.pillar_badge != active_pillar.badge() {
                continue;
            }
            let [r, g, b] = ent.color_rgb;
            let col = format!("rgb({r},{g},{b})");
            let sw = ent.stroke_width * 24.0;
            match &ent.kind {
                CanvasShapeKind::Line { start_mm, end_mm }
                | CanvasShapeKind::Dimension { start_mm, end_mm } => {
                    svg.push_str(&format!(
                        "  <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{col}\" stroke-width=\"{sw:.1}\"/>\n",
                        start_mm[0], -start_mm[1], end_mm[0], -end_mm[1]
                    ));
                }
                CanvasShapeKind::Rectangle { min_mm, max_mm } => {
                    let x = min_mm[0].min(max_mm[0]);
                    let y = (-max_mm[1]).min(-min_mm[1]);
                    let w = (max_mm[0] - min_mm[0]).abs();
                    let h = (max_mm[1] - min_mm[1]).abs();
                    svg.push_str(&format!(
                        "  <rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{w:.1}\" height=\"{h:.1}\" fill=\"none\" stroke=\"{col}\" stroke-width=\"{sw:.1}\"/>\n"
                    ));
                }
                CanvasShapeKind::Circle {
                    center_mm,
                    radius_mm,
                } => {
                    svg.push_str(&format!(
                        "  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"none\" stroke=\"{col}\" stroke-width=\"{sw:.1}\"/>\n",
                        center_mm[0], -center_mm[1], radius_mm
                    ));
                }
                CanvasShapeKind::Freehand { points_mm } => {
                    if points_mm.len() >= 2 {
                        let pts: Vec<String> = points_mm
                            .iter()
                            .map(|p| format!("{:.1},{:.1}", p[0], -p[1]))
                            .collect();
                        svg.push_str(&format!(
                            "  <polyline fill=\"none\" stroke=\"{col}\" stroke-width=\"{sw:.1}\" points=\"{}\"/>\n",
                            pts.join(" ")
                        ));
                    }
                }
                CanvasShapeKind::Callout { pos_mm, text } => {
                    let safe = text.replace('&', "&amp;").replace('<', "&lt;");
                    svg.push_str(&format!(
                        "  <text x=\"{:.1}\" y=\"{:.1}\" fill=\"{col}\" font-size=\"260\" font-family=\"monospace\">{safe}</text>\n",
                        pos_mm[0], -pos_mm[1]
                    ));
                }
            }
        }
        svg.push_str("</svg>\n");
        svg
    }

    /// Renders the unified canvas-first drawing workspace for the active sidebar module.
    #[allow(clippy::too_many_arguments)]
    pub fn show(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        active_pillar: NavigationPillar,
        bim_project: &mut BimProject,
        members: &mut Vec<StructuralMember>,
        variables: &mut Vec<ParametricVariable>,
        workspace_settings: WorkspaceCanvasSettings,
        sidebar_toggle_inset_px: f32,
    ) {
        ui_kit::set_canvas_frozen(ui.ctx(), self.is_frozen);

        let canvas_h = self.preferred_canvas_height(ui.ctx());

        Frame::new()
            .fill(theme.bg_sidebar)
            .corner_radius(CornerRadius::same(12))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(8))
            .show(ui, |ui| {
                let outer_w = ui.available_width().max(160.0);
                ui.set_width(outer_w);
                ui.set_max_width(outer_w);

                // 1. Top Canvas HUD Toolbar (wraps cleanly and yields room for floating ☰ toggle)
                self.render_canvas_toolbar(
                    ui,
                    theme,
                    active_pillar,
                    bim_project,
                    members,
                    variables,
                    workspace_settings.clone(),
                    sidebar_toggle_inset_px,
                );

                // Optional status toast
                Self::render_toast_bar(ui, theme, &mut self.status_message);
                ui.add_space(6.0);

                // 2. Main Interactive Drawing Viewport
                let avail_w = ui.available_width().max(200.0);
                let (response, painter) =
                    ui.allocate_painter(Vec2::new(avail_w, canvas_h), Sense::click_and_drag());
                let rect = response.rect;
                self.last_canvas_rect = Some(rect);

                painter.rect_filled(rect, CornerRadius::same(10), Self::canvas_bg(theme));

                // Reserve floating inspector rect on wide screens (>= 720px)
                let floating_inspector_rect = if self.inspector_open && avail_w >= 720.0 {
                    let insp_w = (avail_w * 0.28).clamp(240.0, 290.0);
                    let insp_h = (rect.height() - 24.0).max(200.0);
                    let r = Rect::from_min_size(
                        Pos2::new(rect.right() - insp_w - 12.0, rect.top() + 12.0),
                        Vec2::new(insp_w, insp_h),
                    );
                    self.last_inspector_rect = Some(r);
                    Some(r)
                } else {
                    self.last_inspector_rect = None;
                    None
                };

                let pointer_in_inspector = floating_inspector_rect.is_some_and(|ir| {
                    ui.ctx()
                        .input(|i| i.pointer.hover_pos().is_some_and(|p| ir.contains(p)))
                });

                // Handle Pan / Zoom / Drawing Interaction (respecting Freeze Canvas)
                self.handle_canvas_interaction(
                    ui,
                    &response,
                    rect,
                    active_pillar,
                    pointer_in_inspector,
                );

                // 2a. Phase 35: Earth World Map Ground Basemap (Vector Continents + Online Raster Tiles + Geodetic/Metric Grid)
                self.world_map.render_world_background(
                    ui,
                    &painter,
                    rect,
                    theme,
                    self.ground_origin_lat_lon,
                    self.show_grid,
                    false,
                );

                // 2b. Render Fine Millimeter Engineering Grid (site scale), BIM Base Layer, Domain Overlay, Entities & Preview
                let world_to_screen = |w: [f64; 2]| self.world_to_screen(rect, w);
                if self.show_grid && self.world_map.zoom_level >= 15.5 {
                    self.paint_grid(&painter, rect, theme, &world_to_screen);
                }
                if self.show_bim_base_layer {
                    self.paint_bim_base_layer(
                        &painter,
                        rect,
                        theme,
                        bim_project,
                        members,
                        active_pillar,
                        &world_to_screen,
                    );
                }
                if self.show_domain_overlay {
                    self.paint_domain_overlay(
                        &painter,
                        rect,
                        theme,
                        active_pillar,
                        bim_project,
                        members,
                        &world_to_screen,
                    );
                }
                self.paint_entities(&painter, rect, theme, active_pillar, &world_to_screen);
                self.paint_live_preview(&painter, rect, theme, &world_to_screen);
                let map_ref = &self.world_map;
                self.render_geodetic_measure_overlay(&painter, rect, theme, &|lat, lon| {
                    map_ref.lat_lon_to_screen(lat, lon, rect)
                });

                // Highlight geodetic site anchor when zoomed out on World Map
                if self.world_map.zoom_level < 12.5
                    && (self.ground_origin_anchored || !self.entities.is_empty() || !bim_project.is_empty())
                {
                    let site_pos = world_to_screen([0.0, 0.0]);
                    if rect.expand(16.0).contains(site_pos) {
                        painter.circle_filled(site_pos, 6.0, theme.accent_blue);
                        painter.circle_stroke(site_pos, 11.0, Stroke::new(1.6, theme.accent_blue));
                        painter.text(
                            site_pos + Vec2::new(12.0, -6.0),
                            Align2::LEFT_BOTTOM,
                            format!(
                                "Site Origin ({:.3}°, {:.3}°)",
                                self.ground_origin_lat_lon.0, self.ground_origin_lat_lon.1
                            ),
                            egui::FontId::monospace(10.0),
                            theme.text_primary,
                        );
                    }
                } else if !self.is_frozen
                    && !self.ground_origin_anchored
                    && self.entities.is_empty()
                    && bim_project.is_empty()
                {
                    painter.text(
                        Pos2::new(rect.left() + 14.0, rect.top() + 14.0),
                        Align2::LEFT_TOP,
                        "World Map Ground Canvas • Pan/Zoom to any site or click Ground 1:200 to draw directly on terrain",
                        egui::FontId::proportional(11.0),
                        theme.text_primary.gamma_multiply(0.85),
                    );
                }

                // Floating Freeze Banner inside canvas when frozen
                if self.is_frozen {
                    Self::paint_frozen_badge(&painter, rect, theme, 14.0);
                }

                // Render Floating Collapsible Inspector Overlay (Desktop / Tablet >= 720px)
                if let Some(insp_rect) = floating_inspector_rect {
                    ui.scope_builder(egui::UiBuilder::new().max_rect(insp_rect), |ui| {
                        self.render_floating_inspector_panel(
                            ui,
                            theme,
                            active_pillar,
                            bim_project,
                            members,
                            variables,
                            workspace_settings.clone(),
                            true,
                        );
                    });
                }

                Self::paint_rounded_viewport_border(
                    &painter,
                    rect,
                    10.0,
                    theme.bg_sidebar,
                    theme.border_subtle,
                );

                ui.add_space(6.0);

                // 3. Bottom Canvas Status Strip
                self.render_status_footer(ui, theme, active_pillar, bim_project);

                // Stacked Collapsible Inspector on narrower viewports (< 720px)
                if self.inspector_open && floating_inspector_rect.is_none() {
                    Frame::new()
                        .inner_margin(Margin::symmetric(10, 8))
                        .show(ui, |ui| {
                            self.render_floating_inspector_panel(
                                ui,
                                theme,
                                active_pillar,
                                bim_project,
                                members,
                                variables,
                                workspace_settings,
                                false,
                            );
                        });
                }
            });
    }

    #[allow(clippy::too_many_arguments)]
    fn render_canvas_toolbar(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        active_pillar: NavigationPillar,
        bim_project: &BimProject,
        members: &[StructuralMember],
        variables: &[ParametricVariable],
        workspace_settings: WorkspaceCanvasSettings,
        sidebar_toggle_inset_px: f32,
    ) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(8))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .inner_margin(Margin::symmetric(10, 8))
            .show(ui, |ui| {
                let outer_w = ui.available_width().max(160.0);
                ui.set_width(outer_w);
                ui.set_max_width(outer_w);
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                ui.horizontal_wrapped(|ui| {
                    if sidebar_toggle_inset_px > 0.0 {
                        ui.add_space(sidebar_toggle_inset_px);
                    }

                    // Active Module Pill
                    icons::pill_badge(
                        ui,
                        active_pillar.app_icon(),
                        format!(
                            "{} • {}",
                            active_pillar.clean_name(),
                            active_pillar.badge()
                        ),
                        theme.accent_blue,
                    );

                    ui.separator();

                    // Drawing Tools Palette
                    self.render_sketch_tools_bar(ui, theme, 26.0, false);

                    ui.separator();

                    // Color Swatches
                    self.render_color_swatches(ui, theme);

                    ui.separator();

                    // Undo, Grid, Snap, Fit
                    if icons::icon_button(
                        ui,
                        AppIcon::Undo,
                        "Undo",
                        10.5,
                        theme.text_secondary,
                        theme.bg_card,
                        Stroke::NONE,
                        CornerRadius::same(6),
                        Vec2::new(0.0, 26.0),
                    )
                    .on_hover_text("Undo last canvas change")
                    .clicked()
                    {
                        self.undo();
                    }

                    if icons::icon_button(
                        ui,
                        AppIcon::Grid,
                        "Grid",
                        10.5,
                        if self.show_grid {
                            Color32::WHITE
                        } else {
                            theme.text_secondary
                        },
                        if self.show_grid {
                            theme.accent_blue.gamma_multiply(0.75)
                        } else {
                            theme.bg_card
                        },
                        Stroke::NONE,
                        CornerRadius::same(6),
                        Vec2::new(0.0, 26.0),
                    )
                    .clicked()
                    {
                        self.show_grid = !self.show_grid;
                    }

                    if icons::icon_button(
                        ui,
                        AppIcon::MagnetSnap,
                        "Snap",
                        10.5,
                        if self.snap_to_grid {
                            Color32::WHITE
                        } else {
                            theme.text_secondary
                        },
                        if self.snap_to_grid {
                            theme.accent_blue.gamma_multiply(0.75)
                        } else {
                            theme.bg_card
                        },
                        Stroke::NONE,
                        CornerRadius::same(6),
                        Vec2::new(0.0, 26.0),
                    )
                    .clicked()
                    {
                        self.snap_to_grid = !self.snap_to_grid;
                    }

                    if icons::icon_button(
                        ui,
                        AppIcon::ZoomFit,
                        "Fit",
                        10.5,
                        theme.text_secondary,
                        theme.bg_card,
                        Stroke::NONE,
                        CornerRadius::same(6),
                        Vec2::new(0.0, 26.0),
                    )
                    .on_hover_text("Fit Ground Site (or Global World Map when empty)")
                    .clicked()
                    {
                        self.fit_world_or_site(!bim_project.is_empty());
                    }

                    ui.separator();

                    // Phase 35: World Map Basemap & Ground Scale Controls
                    self.render_world_map_toolbar_controls(ui, theme, 26.0);

                    ui.separator();

                    // Prominent Freeze Canvas Toggle (Prevents scroll zoom so vertical page scroll works smoothly)
                    Self::render_freeze_button(ui, theme, &mut self.is_frozen, 26.0);

                    // Collapsible Floating Inspector Panel Toggle
                    let insp_bg = if self.inspector_open {
                        theme.accent_green
                    } else {
                        theme.bg_card
                    };
                    let insp_fg = if self.inspector_open {
                        Color32::WHITE
                    } else {
                        theme.text_secondary
                    };
                    if icons::icon_button(
                        ui,
                        AppIcon::SlidersPanel,
                        "Panel",
                        10.5,
                        insp_fg,
                        insp_bg,
                        Stroke::NONE,
                        CornerRadius::same(6),
                        Vec2::new(0.0, 26.0),
                    )
                    .on_hover_text("Show or hide floating canvas & domain inspector panel")
                    .clicked()
                    {
                        self.inspector_open = !self.inspector_open;
                    }

                    // Single-File Unified Sync Save
                    if icons::icon_button(
                        ui,
                        AppIcon::SaveDisk,
                        "Save 1-File",
                        10.5,
                        Color32::WHITE,
                        theme.accent_green,
                        Stroke::NONE,
                        CornerRadius::same(6),
                        Vec2::new(0.0, 26.0),
                    )
                    .on_hover_text(
                        "Save all 15 modules, BIM geometry, and canvas drawings into 1 unified .eigen.json file",
                    )
                    .clicked()
                    {
                        self.export_unified_file(
                            bim_project,
                            members,
                            variables,
                            workspace_settings,
                        );
                    }
                });
            });
    }

    /// World Map basemap selector, Global overview, Ground 1:200 site zoom, and Site Origin anchor controls.
    pub fn render_world_map_toolbar_controls(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        btn_h: f32,
    ) {
        use crate::core::gis::TileSource;
        let short_name = |src: TileSource| -> &'static str {
            match src {
                TileSource::OpenStreetMap => "OSM Streets",
                TileSource::EsriWorldImagery => "Satellite",
                TileSource::OpenTopoMap => "Topo",
                TileSource::CartoDbDarkMatter => "Dark Map",
                TileSource::SyntheticVectorTerrain => "Offline Vector",
            }
        };

        let combo_w = crate::ui_kit::clamped_row_width(ui, 64.0, 102.0, 28.0);
        crate::ui_kit::wrap_before(ui, combo_w + 32.0);
        egui::ComboBox::from_id_salt("universal_world_basemap_combo")
            .width(combo_w)
            .selected_text(short_name(self.world_map.tile_provider))
            .show_ui(ui, |ui| {
                for src in [
                    TileSource::OpenStreetMap,
                    TileSource::EsriWorldImagery,
                    TileSource::OpenTopoMap,
                    TileSource::CartoDbDarkMatter,
                    TileSource::SyntheticVectorTerrain,
                ] {
                    ui.selectable_value(
                        &mut self.world_map.tile_provider,
                        src,
                        src.display_name(),
                    );
                }
            });

        if icons::icon_button(
            ui,
            AppIcon::Globe,
            "World",
            10.5,
            theme.text_secondary,
            theme.bg_card,
            Stroke::NONE,
            CornerRadius::same(6),
            Vec2::new(0.0, btn_h),
        )
        .on_hover_text("Reset view to full Global World Map showing all continents")
        .clicked()
        {
            self.world_map.reset_to_global_view();
        }

        if icons::icon_button(
            ui,
            AppIcon::Target,
            "Ground 1:200",
            10.5,
            theme.text_secondary,
            theme.bg_card,
            Stroke::NONE,
            CornerRadius::same(6),
            Vec2::new(0.0, btn_h),
        )
        .on_hover_text("Zoom directly onto Earth surface at architectural/structural drawing scale")
        .clicked()
        {
            let target = if self.ground_origin_anchored {
                self.ground_origin_lat_lon
            } else {
                self.world_map.center_lat_lon
            };
            self.ground_origin_lat_lon = target;
            self.ground_origin_anchored = true;
            self.world_map.zoom_to_ground_site(target.0, target.1, 17.5);
        }

        if icons::icon_button(
            ui,
            AppIcon::MapPin,
            "Set Origin",
            10.5,
            if self.ground_origin_anchored {
                theme.accent_blue
            } else {
                theme.text_secondary
            },
            theme.bg_card,
            Stroke::NONE,
            CornerRadius::same(6),
            Vec2::new(0.0, btn_h),
        )
        .on_hover_text("Anchor local ground CAD origin (0, 0) mm at current World Map center")
        .clicked()
        {
            let center = self.world_map.center_lat_lon;
            self.ground_origin_lat_lon = center;
            self.ground_origin_anchored = true;
            self.status_message = Some(format!(
                "Ground origin (0, 0) mm anchored at {:.5}°, {:.5}°",
                center.0, center.1
            ));
        }

        // Phase 37 (1D-04): Earth City / Geodetic Coordinate Search & Distance/Area Measurement
        let search_w = crate::ui_kit::clamped_row_width(ui, 64.0, 110.0, 82.0);
        crate::ui_kit::wrap_before(ui, search_w + 80.0);
        let search_resp = ui.add(
            egui::TextEdit::singleline(&mut self.earth_search_query)
                .hint_text("City or lat,lon...")
                .desired_width(search_w),
        );
        let search_enter =
            search_resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
        let search_click = icons::icon_button(
            ui,
            AppIcon::Search,
            "Go",
            10.5,
            theme.text_primary,
            theme.bg_card,
            Stroke::new(1.0, theme.border_subtle),
            CornerRadius::same(6),
            Vec2::new(0.0, btn_h),
        )
        .on_hover_text("Jump Earth map to a city (e.g. Jakarta, Tokyo, London) or 'lat, lon'")
        .clicked();

        if search_enter || search_click {
            if let Some((label, lat, lon, zoom)) =
                crate::core::gis::SlippyMapState::resolve_location_query(&self.earth_search_query)
            {
                self.world_map.zoom_to_ground_site(lat, lon, zoom);
                self.ground_origin_lat_lon = (lat, lon);
                self.ground_origin_anchored = true;
                self.status_message = Some(format!(
                    "Located '{}' -> {:.4}°, {:.4}° (Origin anchored)",
                    label, lat, lon
                ));
            } else if !self.earth_search_query.trim().is_empty() {
                self.status_message = Some(
                    "Location not matched; enter 'lat, lon' (e.g. -6.2088, 106.8456) or city name."
                        .to_string(),
                );
            }
        }

        if icons::icon_button(
            ui,
            AppIcon::DimensionRuler,
            "Measure",
            10.5,
            if self.measure_active {
                Color32::WHITE
            } else {
                theme.text_secondary
            },
            if self.measure_active {
                theme.accent_orange
            } else {
                theme.bg_card
            },
            Stroke::NONE,
            CornerRadius::same(6),
            Vec2::new(0.0, btn_h),
        )
        .on_hover_text("Click Earth surface points to measure geodesic distance (m/km) or parcel area (m²/ha)")
        .clicked()
        {
            self.measure_active = !self.measure_active;
        }

        if self.measure_active || !self.measure_points_lat_lon.is_empty() {
            let area_lbl = if self.measure_area_mode { "Area" } else { "Dist" };
            if ui
                .add(
                    egui::Button::new(RichText::new(area_lbl).size(10.0).color(Color32::WHITE))
                        .fill(if self.measure_area_mode {
                            theme.accent_green
                        } else {
                            theme.accent_blue
                        })
                        .corner_radius(CornerRadius::same(5))
                        .min_size(Vec2::new(0.0, btn_h)),
                )
                .on_hover_text("Switch between Geodesic Distance and Closed Parcel Area mode")
                .clicked()
            {
                self.measure_area_mode = !self.measure_area_mode;
            }
            if !self.measure_points_lat_lon.is_empty()
                && icons::icon_button(
                    ui,
                    AppIcon::Trash,
                    "Clear Meas",
                    10.0,
                    theme.accent_red,
                    theme.bg_card,
                    Stroke::NONE,
                    CornerRadius::same(5),
                    Vec2::new(0.0, btn_h),
                )
                .clicked()
            {
                self.measure_points_lat_lon.clear();
            }
        }
    }

    /// Fits either the active site drawing footprint (if non-empty) or resets to the Global World Map overview.
    pub fn fit_world_or_site(&mut self, has_bim_elements: bool) {
        if self.ground_origin_anchored || !self.entities.is_empty() || has_bim_elements {
            self.world_map.zoom_to_ground_site(
                self.ground_origin_lat_lon.0,
                self.ground_origin_lat_lon.1,
                17.5,
            );
        } else {
            self.world_map.reset_to_global_view();
        }
        self.pan = Vec2::ZERO;
        self.zoom = 1.0;
    }

    /// Anchors ground `(0, 0) mm` origin to `screen_pos` on Earth if the workspace is still blank.
    pub fn ensure_ground_origin_at_screen(&mut self, screen_pos: Pos2, rect: Rect) {
        if !self.ground_origin_anchored && self.entities.is_empty() {
            let (lat, lon) = self.world_map.screen_to_lat_lon(screen_pos, rect);
            self.ground_origin_lat_lon = (lat, lon);
            self.ground_origin_anchored = true;
        }
    }

    /// Renders Universal Canvas drawing tools palette (optionally omitting `SelectPan`).
    pub fn render_sketch_tools_bar(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        button_height: f32,
        skip_select_pan: bool,
    ) {
        for tool in UniversalCanvasTool::ALL {
            if skip_select_pan && tool == UniversalCanvasTool::SelectPan {
                continue;
            }
            let is_sel = self.active_tool == tool;
            let bg = if is_sel {
                theme.accent_blue
            } else {
                theme.bg_card
            };
            let fg = if is_sel {
                Color32::WHITE
            } else {
                theme.text_secondary
            };
            if icons::icon_button(
                ui,
                tool.app_icon(),
                tool.short_label(),
                10.5,
                fg,
                bg,
                Stroke::NONE,
                CornerRadius::same(6),
                Vec2::new(0.0, button_height),
            )
            .on_hover_text(tool.tooltip())
            .clicked()
            {
                self.active_tool = tool;
                self.pending_start_mm = None;
                self.active_freehand_mm.clear();
            }
        }
    }

    /// Renders compact circular ink color swatches shared across Universal Canvas views.
    pub fn render_color_swatches(&mut self, ui: &mut Ui, theme: &MacTheme) {
        for (idx, (cname, rgb)) in PALETTE_SWATCHES.iter().enumerate() {
            let c = Color32::from_rgb(rgb[0], rgb[1], rgb[2]);
            let sel = self.active_color_idx == idx;
            let stroke = if sel {
                Stroke::new(2.0, theme.text_primary)
            } else {
                Stroke::new(1.0, theme.border_card)
            };
            let (srect, sresp) = ui.allocate_exact_size(Vec2::new(18.0, 18.0), Sense::click());
            ui.painter().circle_filled(srect.center(), 7.0, c);
            ui.painter().circle_stroke(srect.center(), 7.5, stroke);
            if sresp.on_hover_text(format!("Ink: {cname}")).clicked() {
                self.active_color_idx = idx;
            }
        }
    }

    pub fn world_origin_and_scale(&self, rect: Rect) -> (Pos2, f32) {
        let origin = self.world_to_screen(rect, [0.0, 0.0]);
        let scale = self.world_map.pixels_per_mm_at_origin(self.ground_origin_lat_lon);
        (origin, scale)
    }

    pub fn world_to_screen(&self, rect: Rect, world_mm: [f64; 2]) -> Pos2 {
        self.world_map
            .world_mm_to_screen(self.ground_origin_lat_lon, world_mm, rect)
    }

    pub fn screen_to_world(&self, rect: Rect, screen: Pos2) -> [f64; 2] {
        self.world_map
            .screen_to_world_mm(self.ground_origin_lat_lon, screen, rect)
    }

    pub fn snap_point(&self, raw_mm: [f64; 2]) -> [f64; 2] {
        if !self.snap_to_grid || self.grid_spacing_mm <= 1.0 {
            return raw_mm;
        }
        let step = self.grid_spacing_mm;
        [
            (raw_mm[0] / step).round() * step,
            (raw_mm[1] / step).round() * step,
        ]
    }

    fn handle_canvas_interaction(
        &mut self,
        ui: &Ui,
        response: &egui::Response,
        rect: Rect,
        active_pillar: NavigationPillar,
        pointer_in_inspector: bool,
    ) {
        if self.is_frozen || pointer_in_inspector {
            self.pending_start_mm = None;
            self.active_freehand_mm.clear();
            return;
        }

        // Scroll Wheel Zoom around cursor on World Map
        if response.hovered() {
            let scroll_y = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll_y.abs() > 0.2 {
                self.world_map
                    .zoom_at_screen_pos(scroll_y * 0.0028, response.hover_pos(), rect);
            }
        }

        if let Some(hover) = response.hover_pos() {
            self.cursor_lat_lon = self.world_map.screen_to_lat_lon(hover, rect);
            self.cursor_world_mm = self.snap_point(self.screen_to_world(rect, hover));
        }

        // Right-drag, Middle-drag, or Shift-drag pans Earth map in any tool
        if response.dragged_by(egui::PointerButton::Secondary)
            || response.dragged_by(egui::PointerButton::Middle)
            || (response.dragged() && ui.input(|i| i.modifiers.shift))
        {
            let delta = response.drag_delta();
            self.world_map.pan_by_pixels(delta.x, delta.y);
            return;
        }

        if self.measure_active && response.clicked_by(egui::PointerButton::Primary) {
            if let Some(pos) = response.interact_pointer_pos() {
                let lat_lon = self.world_map.screen_to_lat_lon(pos, rect);
                self.measure_points_lat_lon.push(lat_lon);
            }
            return;
        }

        if self.active_tool == UniversalCanvasTool::SelectPan {
            if response.dragged_by(egui::PointerButton::Primary) {
                let delta = response.drag_delta();
                self.world_map.pan_by_pixels(delta.x, delta.y);
            }
            return;
        }

        // Auto-anchor ground origin on first drawing stroke anywhere on Earth
        if self.active_tool != UniversalCanvasTool::Eraser
            && (response.drag_started_by(egui::PointerButton::Primary)
                || response.clicked_by(egui::PointerButton::Primary))
        {
            if let Some(pos) = response.interact_pointer_pos() {
                self.ensure_ground_origin_at_screen(pos, rect);
            }
        }

        let scale = self.world_map.pixels_per_mm_at_origin(self.ground_origin_lat_lon);
        let hit_radius_mm = (16.0 / scale.max(1e-9)) as f64;
        let world_map_snapshot = self.world_map.clone();
        let origin_lat_lon = self.ground_origin_lat_lon;
        let to_world = |pos: Pos2| -> [f64; 2] {
            world_map_snapshot.screen_to_world_mm(origin_lat_lon, pos, rect)
        };
        self.handle_world_sketch_interaction(response, active_pillar, hit_radius_mm, &to_world);
    }

    /// Renders interactive WGS84 geodesic polyline/parcel area measurement overlay across 2D & 3D World canvases.
    pub fn render_geodetic_measure_overlay(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        theme: &MacTheme,
        lat_lon_to_screen: &impl Fn(f64, f64) -> Pos2,
    ) {
        if self.measure_points_lat_lon.is_empty() && !self.measure_active {
            return;
        }
        let clip = painter.with_clip_rect(rect);
        let pts: Vec<Pos2> = self
            .measure_points_lat_lon
            .iter()
            .map(|&(lat, lon)| lat_lon_to_screen(lat, lon))
            .collect();

        if self.measure_area_mode && pts.len() >= 3 {
            clip.add(egui::Shape::convex_polygon(
                pts.clone(),
                theme.accent_orange.gamma_multiply(0.16),
                Stroke::new(2.0, theme.accent_orange),
            ));
        } else if pts.len() >= 2 {
            for win in pts.windows(2) {
                clip.line_segment([win[0], win[1]], Stroke::new(2.4, theme.accent_orange));
            }
        }

        for (idx, p) in pts.iter().enumerate() {
            clip.circle_filled(*p, 4.5, theme.accent_orange);
            clip.circle_stroke(*p, 4.5, Stroke::new(1.5, Color32::WHITE));
            if idx > 0 {
                let seg_m = crate::core::gis::SlippyMapState::geodetic_distance_m(
                    self.measure_points_lat_lon[idx - 1],
                    self.measure_points_lat_lon[idx],
                );
                let mid = Pos2::new(0.5 * (pts[idx - 1].x + p.x), 0.5 * (pts[idx - 1].y + p.y));
                let lbl = if seg_m >= 1000.0 {
                    format!("{:.2} km", seg_m / 1000.0)
                } else {
                    format!("{:.1} m", seg_m)
                };
                clip.text(
                    mid + Vec2::new(0.0, -8.0),
                    Align2::CENTER_BOTTOM,
                    lbl,
                    egui::FontId::monospace(10.0),
                    theme.accent_orange,
                );
            }
        }

        let total_m = crate::core::gis::SlippyMapState::geodetic_polyline_length_m(&self.measure_points_lat_lon);
        let summary = if self.measure_points_lat_lon.is_empty() {
            "Geodesic Measure Active: Click points on Earth ground".to_string()
        } else if self.measure_area_mode && self.measure_points_lat_lon.len() >= 3 {
            let area_m2 = crate::core::gis::SlippyMapState::geodetic_polygon_area_m2(
                &self.measure_points_lat_lon,
            );
            format!(
                "Parcel Area: {:.1} m² ({:.3} ha) • Perimeter {:.1} m ({} pts)",
                area_m2,
                area_m2 / 10_000.0,
                total_m,
                self.measure_points_lat_lon.len()
            )
        } else if total_m >= 1000.0 {
            format!(
                "Geodesic Distance: {:.3} km ({:.0} m) • {} pts",
                total_m / 1000.0,
                total_m,
                self.measure_points_lat_lon.len()
            )
        } else {
            format!(
                "Geodesic Distance: {:.2} m • {} pts",
                total_m,
                self.measure_points_lat_lon.len()
            )
        };

        let chip_pos = Pos2::new(rect.left() + 14.0, rect.bottom() - 26.0);
        clip.rect_filled(
            Rect::from_min_size(chip_pos, Vec2::new((summary.len() as f32 * 6.4 + 20.0).min(rect.width() - 28.0), 22.0)),
            CornerRadius::same(6),
            theme.bg_card.gamma_multiply(0.90),
        );
        clip.text(
            chip_pos + Vec2::new(8.0, 11.0),
            Align2::LEFT_CENTER,
            summary,
            egui::FontId::monospace(10.5),
            theme.accent_orange,
        );
    }

    /// Interactive drawing & erasing handler for synchronized Universal Canvas shapes using any 2D/3D ground projection.
    pub fn handle_world_sketch_interaction(
        &mut self,
        response: &egui::Response,
        active_pillar: NavigationPillar,
        hit_radius_mm: f64,
        screen_to_world: &impl Fn(Pos2) -> [f64; 2],
    ) {
        if let Some(hover) = response.hover_pos() {
            self.cursor_world_mm = self.snap_point(screen_to_world(hover));
        }

        let badge = active_pillar.badge().to_string();
        let pname = active_pillar.clean_name().to_string();
        let color_rgb = self.active_color_rgb();
        let stroke_width = self.stroke_width;

        match self.active_tool {
            UniversalCanvasTool::SelectPan => {}
            UniversalCanvasTool::Freehand => {
                if response.drag_started_by(egui::PointerButton::Primary) {
                    if let Some(pos) = response.interact_pointer_pos() {
                        self.active_freehand_mm.clear();
                        self.active_freehand_mm.push(screen_to_world(pos));
                    }
                }
                if response.dragged_by(egui::PointerButton::Primary) {
                    if let Some(pos) = response.interact_pointer_pos() {
                        let pt = screen_to_world(pos);
                        let push_it = self
                            .active_freehand_mm
                            .last()
                            .is_none_or(|last| (last[0] - pt[0]).hypot(last[1] - pt[1]) >= 40.0);
                        if push_it {
                            self.active_freehand_mm.push(pt);
                        }
                    }
                }
                if response.drag_stopped_by(egui::PointerButton::Primary)
                    && self.active_freehand_mm.len() >= 2
                {
                    let pts = std::mem::take(&mut self.active_freehand_mm);
                    self.push_undo();
                    let id = self.next_entity_id;
                    self.next_entity_id += 1;
                    self.entities.push(CanvasEntity {
                        id,
                        pillar_badge: badge,
                        pillar_name: pname,
                        kind: CanvasShapeKind::Freehand { points_mm: pts },
                        color_rgb,
                        stroke_width,
                    });
                }
            }
            UniversalCanvasTool::Line
            | UniversalCanvasTool::Rectangle
            | UniversalCanvasTool::Circle
            | UniversalCanvasTool::Dimension => {
                if response.drag_started_by(egui::PointerButton::Primary) {
                    if let Some(pos) = response.interact_pointer_pos() {
                        self.pending_start_mm = Some(self.snap_point(screen_to_world(pos)));
                    }
                }
                if response.drag_stopped_by(egui::PointerButton::Primary) {
                    if let (Some(start), Some(pos)) =
                        (self.pending_start_mm.take(), response.interact_pointer_pos())
                    {
                        let end = self.snap_point(screen_to_world(pos));
                        let dist = (end[0] - start[0]).hypot(end[1] - start[1]);
                        if dist >= 50.0 {
                            let kind = match self.active_tool {
                                UniversalCanvasTool::Line => CanvasShapeKind::Line {
                                    start_mm: start,
                                    end_mm: end,
                                },
                                UniversalCanvasTool::Dimension => CanvasShapeKind::Dimension {
                                    start_mm: start,
                                    end_mm: end,
                                },
                                UniversalCanvasTool::Rectangle => CanvasShapeKind::Rectangle {
                                    min_mm: [start[0].min(end[0]), start[1].min(end[1])],
                                    max_mm: [start[0].max(end[0]), start[1].max(end[1])],
                                },
                                UniversalCanvasTool::Circle => CanvasShapeKind::Circle {
                                    center_mm: start,
                                    radius_mm: dist,
                                },
                                _ => unreachable!(),
                            };
                            self.push_undo();
                            let id = self.next_entity_id;
                            self.next_entity_id += 1;
                            self.entities.push(CanvasEntity {
                                id,
                                pillar_badge: badge,
                                pillar_name: pname,
                                kind,
                                color_rgb,
                                stroke_width,
                            });
                        }
                    } else {
                        self.pending_start_mm = None;
                    }
                }
            }
            UniversalCanvasTool::Callout => {
                if response.clicked_by(egui::PointerButton::Primary) {
                    if let Some(pos) = response.interact_pointer_pos() {
                        let w = self.snap_point(screen_to_world(pos));
                        let note = if self.callout_draft.trim().is_empty() {
                            format!("{} Note", active_pillar.badge())
                        } else {
                            self.callout_draft.trim().to_string()
                        };
                        self.push_undo();
                        let id = self.next_entity_id;
                        self.next_entity_id += 1;
                        self.entities.push(CanvasEntity {
                            id,
                            pillar_badge: badge,
                            pillar_name: pname,
                            kind: CanvasShapeKind::Callout {
                                pos_mm: w,
                                text: note,
                            },
                            color_rgb,
                            stroke_width,
                        });
                    }
                }
            }
            UniversalCanvasTool::Eraser => {
                if response.clicked_by(egui::PointerButton::Primary)
                    || response.dragged_by(egui::PointerButton::Primary)
                {
                    if let Some(pos) = response.interact_pointer_pos() {
                        let w = screen_to_world(pos);
                        if let Some(idx) = self.find_nearest_entity_idx(w, hit_radius_mm) {
                            self.push_undo();
                            self.entities.remove(idx);
                        }
                    }
                }
            }
        }
    }

    fn find_nearest_entity_idx(&self, pt_mm: [f64; 2], max_dist_mm: f64) -> Option<usize> {
        let mut best: Option<(usize, f64)> = None;
        for (idx, ent) in self.entities.iter().enumerate() {
            let d = match &ent.kind {
                CanvasShapeKind::Line { start_mm, end_mm }
                | CanvasShapeKind::Dimension { start_mm, end_mm } => {
                    point_to_segment_dist(pt_mm, *start_mm, *end_mm)
                }
                CanvasShapeKind::Rectangle { min_mm, max_mm } => {
                    let c0 = *min_mm;
                    let c1 = [max_mm[0], min_mm[1]];
                    let c2 = *max_mm;
                    let c3 = [min_mm[0], max_mm[1]];
                    point_to_segment_dist(pt_mm, c0, c1)
                        .min(point_to_segment_dist(pt_mm, c1, c2))
                        .min(point_to_segment_dist(pt_mm, c2, c3))
                        .min(point_to_segment_dist(pt_mm, c3, c0))
                }
                CanvasShapeKind::Circle {
                    center_mm,
                    radius_mm,
                } => ((pt_mm[0] - center_mm[0]).hypot(pt_mm[1] - center_mm[1]) - radius_mm).abs(),
                CanvasShapeKind::Freehand { points_mm } => points_mm
                    .windows(2)
                    .map(|w| point_to_segment_dist(pt_mm, w[0], w[1]))
                    .fold(f64::MAX, f64::min),
                CanvasShapeKind::Callout { pos_mm, .. } => {
                    (pt_mm[0] - pos_mm[0]).hypot(pt_mm[1] - pos_mm[1])
                }
            };
            if d <= max_dist_mm && best.is_none_or(|(_, bd)| d < bd) {
                best = Some((idx, d));
            }
        }
        best.map(|(i, _)| i)
    }

    fn paint_grid(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        theme: &MacTheme,
        to_screen: &impl Fn([f64; 2]) -> Pos2,
    ) {
        let step = self.grid_spacing_mm.max(250.0);
        let top_left_mm = self.screen_to_world(rect, rect.min);
        let bot_right_mm = self.screen_to_world(rect, rect.max);
        let min_x = (top_left_mm[0].min(bot_right_mm[0]) / step).floor() as i64 - 1;
        let max_x = (top_left_mm[0].max(bot_right_mm[0]) / step).ceil() as i64 + 1;
        let min_y = (top_left_mm[1].min(bot_right_mm[1]) / step).floor() as i64 - 1;
        let max_y = (top_left_mm[1].max(bot_right_mm[1]) / step).ceil() as i64 + 1;

        let line_col = if theme.is_dark() {
            Color32::from_rgba_premultiplied(70, 90, 125, 28)
        } else {
            Color32::from_rgba_premultiplied(120, 140, 170, 42)
        };
        let major_col = if theme.is_dark() {
            Color32::from_rgba_premultiplied(90, 120, 165, 52)
        } else {
            Color32::from_rgba_premultiplied(100, 125, 165, 70)
        };

        // Limit line count on extreme zoom out
        if (max_x - min_x) <= 120 {
            for ix in min_x..=max_x {
                let x = ix as f64 * step;
                let p0 = to_screen([x, min_y as f64 * step]);
                let p1 = to_screen([x, max_y as f64 * step]);
                let col = if ix % 4 == 0 { major_col } else { line_col };
                painter.line_segment([p0, p1], Stroke::new(0.7, col));
            }
        }
        if (max_y - min_y) <= 120 {
            for iy in min_y..=max_y {
                let y = iy as f64 * step;
                let p0 = to_screen([min_x as f64 * step, y]);
                let p1 = to_screen([max_x as f64 * step, y]);
                let col = if iy % 4 == 0 { major_col } else { line_col };
                painter.line_segment([p0, p1], Stroke::new(0.7, col));
            }
        }

        // World Origin Axes
        let o = to_screen([0.0, 0.0]);
        if rect.expand(40.0).contains(o) {
            painter.line_segment(
                [o, o + Vec2::new(36.0, 0.0)],
                Stroke::new(1.8, theme.accent_red),
            );
            painter.line_segment(
                [o, o - Vec2::new(0.0, 36.0)],
                Stroke::new(1.8, theme.accent_green),
            );
            painter.circle_filled(o, 3.0, theme.accent_blue);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn paint_bim_base_layer(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        theme: &MacTheme,
        bim_project: &BimProject,
        members: &[StructuralMember],
        active_pillar: NavigationPillar,
        to_screen: &impl Fn([f64; 2]) -> Pos2,
    ) {
        let clip_painter = painter.with_clip_rect(rect);

        // 1. Structural Grid Axes & Bubbles
        for g in &bim_project.grids {
            let s0 = to_screen([g.start.0, g.start.1]);
            let s1 = to_screen([g.end.0, g.end.1]);
            clip_painter.line_segment(
                [s0, s1],
                Stroke::new(1.0, theme.accent_blue.gamma_multiply(0.40)),
            );
            clip_painter.circle_filled(s1, 9.5, theme.bg_card);
            clip_painter.circle_stroke(s1, 9.5, Stroke::new(1.2, theme.accent_blue));
            clip_painter.text(
                s1,
                Align2::CENTER_CENTER,
                &g.label,
                egui::FontId::monospace(9.5),
                theme.accent_blue,
            );
        }

        // 2. Rooms Halftone Fill
        for rm in &bim_project.rooms {
            let pts: Vec<Pos2> = rm.outer_ring.iter().map(|p| to_screen([p.0, p.1])).collect();
            if pts.len() >= 3 {
                clip_painter.add(egui::Shape::convex_polygon(
                    pts,
                    theme.accent_green.gamma_multiply(0.08),
                    Stroke::NONE,
                ));
            }
            let c = to_screen([rm.seed_point.0, rm.seed_point.1]);
            clip_painter.text(
                c,
                Align2::CENTER_CENTER,
                format!("{} ({:.0} m²)", rm.number, rm.area_m2),
                egui::FontId::proportional(9.5),
                theme.text_muted,
            );
        }

        // 3. Structural Beams / Runs
        for (idx, run) in bim_project.runs.iter().enumerate() {
            let s0 = to_screen([run.start.0, run.start.1]);
            let s1 = to_screen([run.end.0, run.end.1]);
            let col = if active_pillar == NavigationPillar::AnalysisSimulation {
                let util = members.get(idx).map(|m| m.utilization).unwrap_or(0.45);
                if util >= 0.85 {
                    theme.accent_red
                } else if util >= 0.65 {
                    theme.accent_orange
                } else {
                    theme.accent_green
                }
            } else {
                theme.accent_green.gamma_multiply(0.65)
            };
            clip_painter.line_segment([s0, s1], Stroke::new(1.8, col));
        }

        // 4. Compound Walls
        for w in &bim_project.walls {
            let ring: Vec<Pos2> = w
                .footprint_ring()
                .into_iter()
                .map(|p| to_screen([p.0, p.1]))
                .collect();
            let fill = if theme.is_dark() {
                Color32::from_rgb(65, 80, 102)
            } else {
                Color32::from_rgb(148, 163, 184)
            };
            clip_painter.add(egui::Shape::convex_polygon(
                ring,
                fill.gamma_multiply(0.75),
                Stroke::new(1.2, theme.text_secondary.gamma_multiply(0.7)),
            ));
        }

        // 5. Structural Columns
        for col in &bim_project.columns {
            let ring: Vec<Pos2> = col
                .footprint_ring()
                .into_iter()
                .map(|p| to_screen([p.0, p.1]))
                .collect();
            clip_painter.add(egui::Shape::convex_polygon(
                ring,
                theme.accent_blue.gamma_multiply(0.75),
                Stroke::new(1.2, theme.text_primary),
            ));
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn paint_domain_overlay(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        theme: &MacTheme,
        active_pillar: NavigationPillar,
        bim_project: &BimProject,
        members: &[StructuralMember],
        to_screen: &impl Fn([f64; 2]) -> Pos2,
    ) {
        if bim_project.is_empty() && members.is_empty() && self.entities.is_empty() {
            return;
        }
        let clip_painter = painter.with_clip_rect(rect);

        match active_pillar {
            NavigationPillar::MyLibrary => {
                // Graph topology links connecting structural column nodes
                let pts: Vec<Pos2> = bim_project
                    .columns
                    .iter()
                    .take(12)
                    .map(|c| to_screen([c.pos.0, c.pos.1]))
                    .collect();
                for pair in pts.windows(2) {
                    clip_painter.line_segment(
                        [pair[0], pair[1]],
                        Stroke::new(1.0, theme.accent_blue.gamma_multiply(0.45)),
                    );
                }
            }
            NavigationPillar::AiAgent => {
                // AI focus nodes highlighting primary bays
                for &(x, y, tag) in &[
                    (3000.0, 2250.0, "AI-OPT Bay A1"),
                    (9000.0, 6750.0, "AI-OPT Bay B2"),
                ] {
                    let p = to_screen([x, y]);
                    clip_painter.circle_stroke(p, 22.0, Stroke::new(1.4, theme.accent_blue));
                    clip_painter.text(
                        p + Vec2::new(0.0, -28.0),
                        Align2::CENTER_BOTTOM,
                        tag,
                        egui::FontId::monospace(9.5),
                        theme.accent_blue,
                    );
                }
            }
            NavigationPillar::ResearchConcept => {
                // Site boundary setback & geodetic CRS datum box
                let p0 = to_screen([-1000.0, -1000.0]);
                let p1 = to_screen([13000.0, 10000.0]);
                clip_painter.rect_stroke(
                    Rect::from_two_pos(p0, p1),
                    CornerRadius::ZERO,
                    Stroke::new(1.5, theme.accent_orange),
                    StrokeKind::Inside,
                );
                clip_painter.text(
                    Pos2::new(p0.x.min(p1.x) + 8.0, p0.y.min(p1.y) + 8.0),
                    Align2::LEFT_TOP,
                    "WGS84 / UTM Site Boundary • Seismic SDC-D",
                    egui::FontId::monospace(9.5),
                    theme.accent_orange,
                );
            }
            NavigationPillar::ParametricDED => {
                let p0 = to_screen([0.0, -500.0]);
                let p1 = to_screen([6000.0, -500.0]);
                clip_painter.line_segment([p0, p1], Stroke::new(1.6, theme.accent_orange));
                clip_painter.text(
                    Pos2::new((p0.x + p1.x) * 0.5, p0.y + 12.0),
                    Align2::CENTER_TOP,
                    "Bay X1 = 6000 mm (Parametric)",
                    egui::FontId::monospace(9.5),
                    theme.accent_orange,
                );
            }
            NavigationPillar::GeometryAssembly => {
                let p = to_screen([12000.0, 9000.0]);
                clip_painter.text(
                    p + Vec2::new(8.0, -10.0),
                    Align2::LEFT_BOTTOM,
                    format!(
                        "LOD 350 • {} Walls • {} Cols • {} Beams",
                        bim_project.walls.len(),
                        bim_project.columns.len(),
                        bim_project.runs.len()
                    ),
                    egui::FontId::monospace(9.5),
                    theme.accent_green,
                );
            }
            NavigationPillar::AnalysisSimulation => {
                let avg_ari = if members.is_empty() {
                    0.942
                } else {
                    members.iter().map(|m| m.ari_score).sum::<f64>() / members.len() as f64
                };
                for col in &bim_project.columns {
                    let cp = to_screen([col.pos.0, col.pos.1]);
                    clip_painter.circle_stroke(
                        cp,
                        12.0,
                        Stroke::new(1.4, theme.accent_green.gamma_multiply(0.8)),
                    );
                }
                let p = to_screen([6000.0, 9500.0]);
                clip_painter.text(
                    p,
                    Align2::CENTER_BOTTOM,
                    format!("FEA Overlay • Mean ARI {:.3} • SVD Guard Active", avg_ari),
                    egui::FontId::monospace(10.0),
                    theme.accent_green,
                );
            }
            NavigationPillar::TimeScheduling => {
                for (idx, (x, y, label)) in [
                    (3000.0, 2250.0, "4D Phase 1 (W1-W4)"),
                    (9000.0, 2250.0, "4D Phase 2 (W5-W8)"),
                    (3000.0, 6750.0, "4D Phase 3 (W9-W12)"),
                    (9000.0, 6750.0, "4D Phase 4 (W13-W16)"),
                ]
                .into_iter()
                .enumerate()
                {
                    let c = to_screen([x, y]);
                    let col = if idx < 2 {
                        theme.accent_green
                    } else {
                        theme.accent_blue
                    };
                    clip_painter.circle_stroke(c, 16.0, Stroke::new(1.5, col));
                    clip_painter.text(
                        c + Vec2::new(0.0, 20.0),
                        Align2::CENTER_TOP,
                        label,
                        egui::FontId::monospace(9.0),
                        col,
                    );
                }
            }
            NavigationPillar::CostBudgeting => {
                let p0 = to_screen([0.0, 0.0]);
                let p1 = to_screen([6000.0, 4500.0]);
                clip_painter.rect_filled(
                    Rect::from_two_pos(p0, p1),
                    CornerRadius::ZERO,
                    theme.accent_orange.gamma_multiply(0.10),
                );
                clip_painter.text(
                    Rect::from_two_pos(p0, p1).center(),
                    Align2::CENTER_CENTER,
                    "5D High-Cost Core Zone (42% BoQ)",
                    egui::FontId::monospace(9.5),
                    theme.accent_orange,
                );
            }
            NavigationPillar::SustainabilityEnergy => {
                // 6D Solar Heliodon Arc
                let center = to_screen([6000.0, 4500.0]);
                let edge = to_screen([12500.0, 4500.0]);
                let rad = (edge.x - center.x).abs().max(40.0);
                clip_painter.circle_stroke(
                    center,
                    rad,
                    Stroke::new(1.2, theme.accent_orange.gamma_multiply(0.65)),
                );
                let sun = center + Vec2::new(rad * 0.72, -rad * 0.68);
                clip_painter.circle_filled(sun, 7.0, theme.accent_orange);
                clip_painter.line_segment(
                    [sun, center],
                    Stroke::new(1.2, theme.accent_orange.gamma_multiply(0.8)),
                );
                clip_painter.text(
                    sun + Vec2::new(10.0, -4.0),
                    Align2::LEFT_CENTER,
                    "Heliodon Azimuth 52° • OTTV 31 W/m²",
                    egui::FontId::monospace(9.5),
                    theme.accent_orange,
                );
            }
            NavigationPillar::FacilityManagement => {
                for (x, y, tag) in [
                    (1500.0, 4500.0, "SHM-01"),
                    (6000.0, 4500.0, "SHM-02"),
                    (10500.0, 4500.0, "MEP Riser"),
                ] {
                    let p = to_screen([x, y]);
                    clip_painter.circle_filled(p, 5.0, theme.accent_green);
                    clip_painter.text(
                        p + Vec2::new(8.0, -8.0),
                        Align2::LEFT_BOTTOM,
                        tag,
                        egui::FontId::monospace(9.5),
                        theme.accent_green,
                    );
                }
            }
            NavigationPillar::SafetyHealth => {
                // 8D Tower Crane Sweep Radius & Boom
                let mast = to_screen([6000.0, 10200.0]);
                let tip = to_screen([12800.0, 10200.0]);
                let rad = (tip.x - mast.x).abs().max(40.0);
                clip_painter.circle_stroke(
                    mast,
                    rad,
                    Stroke::new(1.6, theme.accent_orange.gamma_multiply(0.8)),
                );
                let boom_end = mast + Vec2::new(rad * 0.7, rad * 0.68);
                clip_painter.line_segment([mast, boom_end], Stroke::new(2.6, theme.accent_red));
                clip_painter.circle_filled(mast, 5.5, theme.accent_red);
                clip_painter.text(
                    mast + Vec2::new(10.0, -10.0),
                    Align2::LEFT_BOTTOM,
                    "TC-01 Jib Sweep Radius (45m)",
                    egui::FontId::monospace(9.5),
                    theme.accent_orange,
                );
            }
            NavigationPillar::LeanConstruction => {
                let l0 = to_screen([-1400.0, 0.0]);
                let l1 = to_screen([-200.0, 4500.0]);
                let laydown = Rect::from_two_pos(l0, l1);
                clip_painter.rect_stroke(
                    laydown,
                    CornerRadius::same(4),
                    Stroke::new(1.5, theme.accent_green),
                    StrokeKind::Inside,
                );
                clip_painter.text(
                    laydown.center(),
                    Align2::CENTER_CENTER,
                    "JIT Laydown",
                    egui::FontId::monospace(9.0),
                    theme.accent_green,
                );
            }
            NavigationPillar::IndustrializedAutomation => {
                let p0 = to_screen([0.0, 0.0]);
                let p1 = to_screen([6000.0, 4500.0]);
                clip_painter.rect_stroke(
                    Rect::from_two_pos(p0, p1),
                    CornerRadius::ZERO,
                    Stroke::new(1.6, theme.accent_blue),
                    StrokeKind::Inside,
                );
                clip_painter.text(
                    Rect::from_two_pos(p0, p1).center(),
                    Align2::CENTER_CENTER,
                    "10D DfMA Pod Module M-01",
                    egui::FontId::monospace(9.5),
                    theme.accent_blue,
                );
            }
            NavigationPillar::Prediction => {
                let p = to_screen([6000.0, 4500.0]);
                clip_painter.circle_stroke(p, 28.0, Stroke::new(1.6, theme.accent_red));
                clip_painter.circle_stroke(p, 42.0, Stroke::new(1.0, theme.accent_orange));
                clip_painter.text(
                    p + Vec2::new(0.0, -48.0),
                    Align2::CENTER_BOTTOM,
                    "Drift Hotspot Watch • Pf < 1e-4",
                    egui::FontId::monospace(9.5),
                    theme.accent_orange,
                );
            }
            NavigationPillar::ReportPublishing => {
                let margin_rect = rect.shrink(14.0);
                clip_painter.rect_stroke(
                    margin_rect,
                    CornerRadius::ZERO,
                    Stroke::new(1.4, theme.text_muted),
                    StrokeKind::Inside,
                );
                clip_painter.text(
                    Pos2::new(margin_rect.right() - 8.0, margin_rect.bottom() - 8.0),
                    Align2::RIGHT_BOTTOM,
                    "ISO A3 SHEET BORDER • EIGENGRAPH DED",
                    egui::FontId::monospace(9.0),
                    theme.text_muted,
                );
            }
        }
    }

    /// Paint synchronized sketch/annotation entities onto any compatible 2D world canvas
    /// (used to sync Universal Canvas markup inside ParametricView's single 2D canvas).
    pub fn paint_entities_world(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        theme: &MacTheme,
        active_pillar: NavigationPillar,
        to_screen: &impl Fn([f64; 2]) -> Pos2,
    ) {
        self.paint_entities(painter, rect, theme, active_pillar, to_screen);
    }

    fn paint_entities(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        theme: &MacTheme,
        active_pillar: NavigationPillar,
        to_screen: &impl Fn([f64; 2]) -> Pos2,
    ) {
        let clip_painter = painter.with_clip_rect(rect);
        let active_badge = active_pillar.badge();

        for ent in &self.entities {
            if !self.sync_all_layers && ent.pillar_badge != active_badge {
                continue;
            }
            let base_col = Color32::from_rgb(ent.color_rgb[0], ent.color_rgb[1], ent.color_rgb[2]);
            let col = if ent.pillar_badge == active_badge {
                base_col
            } else {
                base_col.gamma_multiply(0.82)
            };
            let stroke = Stroke::new(ent.stroke_width, col);

            match &ent.kind {
                CanvasShapeKind::Freehand { points_mm } => {
                    let spts: Vec<Pos2> = points_mm.iter().map(|p| to_screen(*p)).collect();
                    for w in spts.windows(2) {
                        clip_painter.line_segment([w[0], w[1]], stroke);
                    }
                }
                CanvasShapeKind::Line { start_mm, end_mm } => {
                    clip_painter.line_segment([to_screen(*start_mm), to_screen(*end_mm)], stroke);
                }
                CanvasShapeKind::Rectangle { min_mm, max_mm } => {
                    let r = Rect::from_two_pos(to_screen(*min_mm), to_screen(*max_mm));
                    clip_painter.rect_filled(r, CornerRadius::ZERO, col.gamma_multiply(0.08));
                    clip_painter.rect_stroke(r, CornerRadius::ZERO, stroke, StrokeKind::Inside);
                }
                CanvasShapeKind::Circle {
                    center_mm,
                    radius_mm,
                } => {
                    let c = to_screen(*center_mm);
                    let edge = to_screen([center_mm[0] + radius_mm, center_mm[1]]);
                    let r_px = (edge.x - c.x).abs().max(2.0);
                    clip_painter.circle_stroke(c, r_px, stroke);
                }
                CanvasShapeKind::Dimension { start_mm, end_mm } => {
                    let s0 = to_screen(*start_mm);
                    let s1 = to_screen(*end_mm);
                    clip_painter.line_segment([s0, s1], stroke);
                    clip_painter.circle_filled(s0, 3.0, col);
                    clip_painter.circle_filled(s1, 3.0, col);
                    let dist = (end_mm[0] - start_mm[0]).hypot(end_mm[1] - start_mm[1]);
                    let mid = Pos2::new((s0.x + s1.x) * 0.5, (s0.y + s1.y) * 0.5 - 10.0);
                    clip_painter.text(
                        mid,
                        Align2::CENTER_BOTTOM,
                        format!("[{}] {:.0} mm", ent.pillar_badge, dist),
                        egui::FontId::monospace(10.0),
                        col,
                    );
                }
                CanvasShapeKind::Callout { pos_mm, text } => {
                    let p = to_screen(*pos_mm);
                    clip_painter.circle_filled(p, 4.0, col);
                    let box_pos = p + Vec2::new(8.0, -18.0);
                    let bg = if theme.is_dark() {
                        Color32::from_rgba_premultiplied(20, 24, 34, 225)
                    } else {
                        Color32::from_rgba_premultiplied(255, 255, 255, 235)
                    };
                    let tag = format!("[{}] {}", ent.pillar_badge, text);
                    let approx_w = (tag.len() as f32 * 6.3 + 14.0).clamp(80.0, 280.0);
                    let tag_rect = Rect::from_min_size(box_pos, Vec2::new(approx_w, 20.0));
                    clip_painter.rect_filled(tag_rect, CornerRadius::same(5), bg);
                    clip_painter.rect_stroke(
                        tag_rect,
                        CornerRadius::same(5),
                        Stroke::new(1.0, col),
                        StrokeKind::Inside,
                    );
                    clip_painter.text(
                        tag_rect.center(),
                        Align2::CENTER_CENTER,
                        tag,
                        egui::FontId::monospace(9.5),
                        theme.text_primary,
                    );
                }
            }
        }
    }

    /// Paints live rubber-band sketch/dimension/rectangle/circle preview using any world-to-screen mapping.
    pub fn paint_live_preview_world(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        theme: &MacTheme,
        to_screen: &impl Fn([f64; 2]) -> Pos2,
    ) {
        self.paint_live_preview(painter, rect, theme, to_screen);
    }

    fn paint_live_preview(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        theme: &MacTheme,
        to_screen: &impl Fn([f64; 2]) -> Pos2,
    ) {
        let clip_painter = painter.with_clip_rect(rect);
        let preview_col = self.active_color();
        let stroke = Stroke::new(self.stroke_width, preview_col);

        if self.active_freehand_mm.len() >= 2 {
            let spts: Vec<Pos2> = self
                .active_freehand_mm
                .iter()
                .map(|p| to_screen(*p))
                .collect();
            for w in spts.windows(2) {
                clip_painter.line_segment([w[0], w[1]], stroke);
            }
        }

        if let Some(start) = self.pending_start_mm {
            let end = self.cursor_world_mm;
            let s0 = to_screen(start);
            let s1 = to_screen(end);
            match self.active_tool {
                UniversalCanvasTool::Line | UniversalCanvasTool::Dimension => {
                    clip_painter.line_segment([s0, s1], stroke);
                }
                UniversalCanvasTool::Rectangle => {
                    clip_painter.rect_stroke(
                        Rect::from_two_pos(s0, s1),
                        CornerRadius::ZERO,
                        stroke,
                        StrokeKind::Inside,
                    );
                }
                UniversalCanvasTool::Circle => {
                    let r_px = (s1 - s0).length().max(2.0);
                    clip_painter.circle_stroke(s0, r_px, stroke);
                }
                _ => {}
            }
            let dist = (end[0] - start[0]).hypot(end[1] - start[1]);
            clip_painter.text(
                s1 + Vec2::new(10.0, -10.0),
                Align2::LEFT_BOTTOM,
                format!("{:.0} mm", dist),
                egui::FontId::monospace(10.0),
                theme.accent_orange,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_floating_inspector_panel(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        active_pillar: NavigationPillar,
        bim_project: &BimProject,
        members: &[StructuralMember],
        variables: &[ParametricVariable],
        workspace_settings: WorkspaceCanvasSettings,
        is_overlay: bool,
    ) {
        Self::floating_panel_frame(theme, is_overlay).show(ui, |ui| {
            ui.set_width(ui.available_width());
            egui::ScrollArea::vertical()
                .id_salt("universal_canvas_inspector_scroll")
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        icons::icon_label(
                            ui,
                            active_pillar.app_icon(),
                            14.0,
                            theme.accent_blue,
                            "Canvas Inspector",
                            12.5,
                            theme.text_primary,
                            true,
                        );
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if icons::icon_only_button(
                                ui,
                                AppIcon::Close,
                                theme.text_secondary,
                                Color32::TRANSPARENT,
                                Stroke::NONE,
                                CornerRadius::same(4),
                                Vec2::splat(20.0),
                                "Close inspector panel",
                            )
                            .clicked()
                            {
                                self.inspector_open = false;
                            }
                        });
                    });
                    ui.separator();
                    ui.add_space(4.0);

                    self.render_canvas_sync_controls(
                        ui,
                        theme,
                        active_pillar,
                        bim_project,
                        members,
                        variables,
                        workspace_settings,
                        true,
                    );
                });
        });
    }

    /// Shared Canvas Sync, ink stroke, layer cleanup, and 1-file export controls reusable inside
    /// floating HUD inspectors across Universal Canvas, Parametric, and Geometry.
    #[allow(clippy::too_many_arguments)]
    pub fn render_canvas_sync_controls(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        active_pillar: NavigationPillar,
        bim_project: &BimProject,
        members: &[StructuralMember],
        variables: &[ParametricVariable],
        workspace_settings: WorkspaceCanvasSettings,
        show_domain_overlay_toggle: bool,
    ) {
        ui.checkbox(&mut self.sync_all_layers, "Sync drawings across all 15 modules");
        ui.checkbox(&mut self.show_bim_base_layer, "Show synchronized BIM base plan");
        if show_domain_overlay_toggle {
            ui.checkbox(&mut self.show_domain_overlay, "Show active module overlay");
        }
        ui.add_space(6.0);

        ui.label(
            RichText::new("Callout Note Draft:")
                .size(11.0)
                .color(theme.text_secondary),
        );
        ui.add(
            egui::TextEdit::singleline(&mut self.callout_draft)
                .desired_width(ui.available_width().max(120.0)),
        );

        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label(RichText::new("Stroke:").size(11.0).color(theme.text_secondary));
            ui.add(egui::Slider::new(&mut self.stroke_width, 1.0..=6.0));
        });

        ui.horizontal(|ui| {
            ui.label(RichText::new("Canvas H:").size(11.0).color(theme.text_secondary));
            ui.add(egui::Slider::new(&mut self.canvas_height_ratio, 0.45..=1.15));
        });

        ui.add_space(6.0);
        ui.label(
            RichText::new(format!(
                "Layer Count: {} active ({} total synced)",
                self.count_for_pillar(active_pillar),
                self.entities.len()
            ))
            .size(10.5)
            .color(theme.text_muted),
        );

        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            if icons::icon_button(
                ui,
                AppIcon::Eraser,
                "Clear Active Layer",
                10.5,
                theme.text_primary,
                theme.bg_sidebar,
                Stroke::new(1.0, theme.border_subtle),
                CornerRadius::same(5),
                Vec2::new(0.0, 22.0),
            )
            .clicked()
            {
                self.clear_entities(Some(active_pillar));
            }
            if icons::icon_button(
                ui,
                AppIcon::Trash,
                "Clear All",
                10.5,
                theme.accent_red,
                theme.bg_sidebar,
                Stroke::new(1.0, theme.accent_red.gamma_multiply(0.45)),
                CornerRadius::same(5),
                Vec2::new(0.0, 22.0),
            )
            .clicked()
            {
                self.clear_entities(None);
            }
        });

        ui.add_space(8.0);
        ui.separator();
        ui.label(
            RichText::new("SINGLE-FILE & SPLIT EXPORT")
                .size(10.0)
                .strong()
                .color(theme.accent_blue),
        );
        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            if icons::icon_button(
                ui,
                AppIcon::SaveDisk,
                "Save 1-File (.eigen.json)",
                10.5,
                Color32::WHITE,
                theme.accent_green,
                Stroke::NONE,
                CornerRadius::same(5),
                Vec2::new(0.0, 24.0),
            )
            .clicked()
            {
                self.export_unified_file(bim_project, members, variables, workspace_settings);
            }
            if icons::icon_button(
                ui,
                AppIcon::UploadImport,
                "Import 1-File",
                10.5,
                theme.text_primary,
                theme.bg_sidebar,
                Stroke::new(1.0, theme.border_subtle),
                CornerRadius::same(5),
                Vec2::new(0.0, 24.0),
            )
            .clicked()
            {
                self.import_json_open = true;
            }
            if icons::icon_button(
                ui,
                AppIcon::Palette,
                "Export SVG",
                10.5,
                theme.text_primary,
                theme.bg_sidebar,
                Stroke::new(1.0, theme.border_subtle),
                CornerRadius::same(5),
                Vec2::new(0.0, 24.0),
            )
            .clicked()
            {
                let svg = self.export_svg(bim_project, active_pillar);
                match DiskExporter::write_export_file("EigenGraph_Unified_Canvas.svg", &svg) {
                    Ok(dest) => {
                        self.status_message = Some(format!("Exported SVG canvas -> {dest}"));
                    }
                    Err(err) => {
                        self.status_message = Some(format!("SVG export failed: {err}"));
                    }
                }
            }
            if icons::icon_button(
                ui,
                AppIcon::Folder,
                "Split Active Module",
                10.5,
                theme.text_primary,
                theme.bg_sidebar,
                Stroke::new(1.0, theme.border_subtle),
                CornerRadius::same(5),
                Vec2::new(0.0, 24.0),
            )
            .on_hover_text("Export only this module's drawing layer separately")
            .clicked()
            {
                self.export_split_module_file(active_pillar);
            }
        });
    }

    fn render_status_footer(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        active_pillar: NavigationPillar,
        bim_project: &BimProject,
    ) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(8))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .inner_margin(Margin::symmetric(12, 6))
            .show(ui, |ui| {
                let inner_w = ui.available_width().max(120.0);
                ui.set_width(inner_w);
                ui.set_max_width(inner_w);
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                let telem = self.world_map.telemetry();
                let geo_txt = format!(
                    "{:.4}°, {:.4}° • z{:.1} ({:.1} m/px)",
                    self.cursor_lat_lon.0,
                    self.cursor_lat_lon.1,
                    self.world_map.zoom_level,
                    telem.ground_resolution_m_px,
                );
                let coord_txt = format!(
                    "({:.0}, {:.0}) mm",
                    self.cursor_world_mm[0], self.cursor_world_mm[1],
                );
                let counts_txt = format!(
                    "{} Draw ({} {}) • {} BIM",
                    self.entities.len(),
                    self.count_for_pillar(active_pillar),
                    active_pillar.badge(),
                    bim_project.walls.len()
                        + bim_project.columns.len()
                        + bim_project.runs.len()
                );
                ui.horizontal_wrapped(|ui| {
                    icons::icon_label(
                        ui,
                        AppIcon::Globe,
                        12.0,
                        theme.accent_blue,
                        geo_txt,
                        10.5,
                        theme.text_secondary,
                        false,
                    );
                    icons::icon_label(
                        ui,
                        AppIcon::Compass,
                        11.0,
                        theme.text_muted,
                        coord_txt,
                        10.5,
                        theme.text_secondary,
                        false,
                    );
                    icons::icon_label(
                        ui,
                        AppIcon::Layers,
                        11.0,
                        theme.accent_blue,
                        counts_txt,
                        10.5,
                        theme.text_secondary,
                        false,
                    );
                    if icons::icon_button(
                        ui,
                        AppIcon::ArrowDown,
                        format!("{} Workbench", active_pillar.clean_name()),
                        10.5,
                        theme.accent_blue,
                        theme.accent_blue.gamma_multiply(0.14),
                        Stroke::NONE,
                        CornerRadius::same(6),
                        Vec2::new(0.0, 22.0),
                    )
                    .on_hover_text("Scroll down to the detailed module workbench")
                    .clicked()
                    {
                        self.request_scroll_to_workbench = true;
                    }
                });
            });
    }

    /// Render the Unified Single-File Import Modal whenever `import_json_open` is true.
    /// Invoked from the root app loop so single-file import works seamlessly across all 15 modules,
    /// including single-canvas Parametric and Geometry modules.
    pub fn render_import_modal_if_open(
        &mut self,
        ctx: &egui::Context,
        theme: &MacTheme,
        bim_project: &mut BimProject,
        members: &mut Vec<StructuralMember>,
        variables: &mut Vec<ParametricVariable>,
    ) {
        if self.import_json_open {
            self.render_import_modal(ctx, theme, bim_project, members, variables);
        }
    }

    fn render_import_modal(
        &mut self,
        ctx: &egui::Context,
        theme: &MacTheme,
        bim_project: &mut BimProject,
        members: &mut Vec<StructuralMember>,
        variables: &mut Vec<ParametricVariable>,
    ) {
        let mut open = self.import_json_open;
        let mut close_now = false;
        egui::Window::new("Import Unified Workspace (.eigen.json)")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .default_size(MacTheme::clamp_modal_size(ctx, Vec2::new(620.0, 420.0)))
            .max_size(MacTheme::modal_max_size(ctx))
            .frame(
                Frame::new()
                    .fill(theme.bg_card)
                    .stroke(Stroke::new(1.0, theme.border_subtle))
                    .corner_radius(CornerRadius::same(12))
                    .inner_margin(Margin::same(14)),
            )
            .show(ctx, |ui| {
                ui.label(
                    RichText::new(
                        "Paste the contents of a single-file EigenGraph_Unified_Workspace.eigen.json archive to verify its SHA-256 seal and restore all synchronized modules.",
                    )
                    .size(11.5)
                    .color(theme.text_secondary),
                );
                ui.add_space(8.0);
                egui::ScrollArea::vertical()
                    .max_height(240.0)
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.import_json_buffer)
                                .desired_width(ui.available_width())
                                .desired_rows(12),
                        );
                    });
                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    if icons::icon_button(
                        ui,
                        AppIcon::CheckCircle,
                        "Verify & Restore Workspace",
                        11.5,
                        Color32::WHITE,
                        theme.accent_green,
                        Stroke::NONE,
                        CornerRadius::same(7),
                        Vec2::new(0.0, 28.0),
                    )
                    .clicked()
                    {
                        match self.apply_unified_archive(
                            &self.import_json_buffer.clone(),
                            bim_project,
                            members,
                            variables,
                        ) {
                            Ok(settings) => {
                                self.pending_imported_settings = Some(settings);
                                close_now = true;
                            }
                            Err(err) => {
                                self.status_message = Some(format!("Import error: {err}"));
                            }
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        close_now = true;
                    }
                });
            });
        self.import_json_open = open && !close_now;
    }
}

fn point_to_segment_dist(p: [f64; 2], a: [f64; 2], b: [f64; 2]) -> f64 {
    let vx = b[0] - a[0];
    let vy = b[1] - a[1];
    let len_sq = vx * vx + vy * vy;
    if len_sq <= 1e-9 {
        return (p[0] - a[0]).hypot(p[1] - a[1]);
    }
    let t = (((p[0] - a[0]) * vx + (p[1] - a[1]) * vy) / len_sq).clamp(0.0, 1.0);
    let proj = [a[0] + t * vx, a[1] + t * vy];
    (p[0] - proj[0]).hypot(p[1] - proj[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_settings() -> WorkspaceCanvasSettings {
        WorkspaceCanvasSettings {
            theme_mode: ThemeMode::ProDark,
            canvas_height_ratio: 0.74,
            freeze_by_default: true,
            show_grid: true,
            snap_to_grid: true,
            grid_spacing_mm: 500.0,
            show_bim_base_layer: true,
            show_domain_overlay: true,
            sync_all_layers: true,
            show_floating_inspector: false,
            show_ai_copilot: true,
            unit_system: "kN, mm, MPa (Structural Metric)".to_string(),
            design_standard: "SNI 1729 / AISC 360-16".to_string(),
            solver_threads: 8,
            enable_svd_guard: true,
            enable_anti_leakage: true,
        }
    }

    #[test]
    fn test_unified_single_file_archive_roundtrip_and_tamper_detection() {
        let canvas = UniversalCanvasState::default();
        assert!(canvas.is_frozen);
        assert!(canvas.entities.is_empty());
        assert!(canvas.world_map.zoom_level < 3.0);

        let bim = BimProject::sample_bimcraft_building();
        let members = vec![StructuralMember::default()];
        let vars = vec![ParametricVariable {
            name: "L".to_string(),
            expression: "24000".to_string(),
            evaluated_value: 24000.0,
            unit: "mm".to_string(),
        }];

        let archive = canvas.build_unified_archive(&bim, &members, &vars, sample_settings());
        let json = archive.to_pretty_json().expect("serialize archive");
        let verified = UnifiedWorkspaceArchive::from_verified_json(&json).expect("verify archive");
        assert_eq!(verified.canvas_entities.len(), 0);
        assert_eq!(verified.members.len(), 1);
        assert_eq!(verified.bim_project.walls.len(), bim.walls.len());

        // Tampering with JSON content must fail SHA-256 verification
        let tampered = json.replace("24000", "99999");
        assert!(UnifiedWorkspaceArchive::from_verified_json(&tampered).is_err());
    }

    #[test]
    fn test_canvas_sync_across_pillars_undo_and_svg_export() {
        let mut canvas = UniversalCanvasState::default();
        let initial_len = canvas.entities.len();
        canvas.push_undo();
        canvas.entities.push(CanvasEntity {
            id: 99,
            pillar_badge: NavigationPillar::SafetyHealth.badge().to_string(),
            pillar_name: NavigationPillar::SafetyHealth.clean_name().to_string(),
            kind: CanvasShapeKind::Circle {
                center_mm: [6000.0, 4500.0],
                radius_mm: 3000.0,
            },
            color_rgb: [245, 158, 11],
            stroke_width: 2.5,
        });
        assert_eq!(canvas.count_for_pillar(NavigationPillar::SafetyHealth), 1);
        assert_eq!(canvas.entities.len(), initial_len + 1);

        let bim = BimProject::sample_bimcraft_building();
        let svg = canvas.export_svg(&bim, NavigationPillar::SafetyHealth);
        assert!(svg.contains("<circle"));

        assert!(canvas.undo());
        assert_eq!(canvas.entities.len(), initial_len);
    }
}
