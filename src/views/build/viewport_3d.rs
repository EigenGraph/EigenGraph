//! views/build/viewport_3d.rs — 360-deg Interactive 3D BIM & Analytical Diagram Viewport.

use super::*;

#[derive(Clone)]
struct Member3D {
    id: usize,
    p1: (f32, f32, f32),
    p2: (f32, f32, f32),
    name: String,
    section: String,
    base_width: f32,
    force_kn: f32,
    capacity_kn: f32,
    utilization: f32,
    ari: f32,
    is_critical: bool,
}


impl BuildView {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn render_canvas(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        members: &[StructuralMember],
        features: &mut FeatureRegistry,
        universal_canvas: &mut crate::views::universal_canvas::UniversalCanvasState,
        parametric_variables: &[crate::models::ParametricVariable],
        workspace_settings: crate::views::universal_canvas::WorkspaceCanvasSettings,
        sidebar_toggle_inset_px: f32,
        stacked_height: Option<f32>,
    ) {
        ui.vertical(|ui| {
            // Master Multi-Discipline BIM Ribbon Toolbar (Phase 26)
            self.render_bim_master_ribbon(ui, theme, sidebar_toggle_inset_px);
            ui.add_space(4.0);

            // Contextual Top Modeling Ribbon
            Frame::new()
                .fill(theme.bg_sidebar)
                .inner_margin(Margin::symmetric(14, 8))
                .stroke(Stroke::new(1.0, theme.border_subtle))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        // Render Style & Internal Force Diagrams Toggle
                        let modes = [
                            (Render3DMode::CsgSolidBrep, AppIcon::Cube3D, "CSG B-Rep", "True 3D CSG Boolean Mesh (Openings, WF Flanges & Rebar)"),
                            (Render3DMode::SolidShaded, AppIcon::SolidBox, "Solid", "Shaded solid structural frame"),
                            (Render3DMode::Wireframe, AppIcon::Wireframe, "Wireframe", "Vector wireframe members"),
                            (Render3DMode::AxialForceNFD, AppIcon::ChartBar, "NFD", "Normal / Axial Force Diagram (Red=Tension, Blue=Compression)"),
                            (Render3DMode::ShearForceSFD, AppIcon::Bolt, "SFD", "Shear Force Diagram (SFD)"),
                            (Render3DMode::BendingMomentBMD, AppIcon::ChartLine, "BMD", "Bending Moment Diagram (BMD)"),
                        ];
                        for (rm, icon, lbl, tip) in modes {
                            let sel = self.render_mode == rm;
                            if icons::icon_button(
                                ui,
                                icon,
                                lbl,
                                10.5,
                                if sel { Color32::WHITE } else { theme.text_secondary },
                                if sel { theme.accent_blue } else { theme.bg_card },
                                Stroke::NONE,
                                CornerRadius::same(5),
                                Vec2::new(0.0, 24.0),
                            )
                            .on_hover_text(tip)
                            .clicked()
                            {
                                self.render_mode = rm;
                            }
                        }

                        ui.add_space(6.0);
                        ui.separator();
                        ui.add_space(6.0);

                        // Compact Model Preset Selector Dropdown (Prevents Horizontal Overflow)
                        ui.label(RichText::new("Model:").size(11.0).color(theme.text_muted));
                        let models = [
                            ModelArchetype::BimCraftBuilding3D,
                            ModelArchetype::OpenBimIfcModel3D,
                            ModelArchetype::FireCompartmentFrame3D,
                            ModelArchetype::BlastResistantBunker3D,
                            ModelArchetype::FemaP58HospitalArchetype,
                            ModelArchetype::BaseIsolatedHospital3D,
                            ModelArchetype::TimeHistoryTower3D,
                            ModelArchetype::CoupledShearWallCore3D,
                            ModelArchetype::PrestressedGirderBridge3D,
                            ModelArchetype::DoubleLayerSpaceTruss3D,
                            ModelArchetype::GeodesicDome3D,
                            ModelArchetype::HyparRuledShell3D,
                            ModelArchetype::MultiStoryTower3D,
                            ModelArchetype::DiagridTower3D,
                            ModelArchetype::TrussBridge3D,
                            ModelArchetype::SpaceDome3D,
                            ModelArchetype::WarrenTruss2D,
                        ];
                        egui::ComboBox::from_id_salt("model_archetype_combo")
                            .selected_text(RichText::new(self.active_model.label()).size(11.5).strong().color(theme.text_primary))
                            .width(ui.available_width().clamp(140.0, 230.0))
                            .show_ui(ui, |ui| {
                                for m in models {
                                    let is_m_sel = self.active_model == m;
                                    if ui.selectable_label(is_m_sel, m.label()).clicked() {
                                        self.active_model = m;
                                        self.selected_member_id = None;
                                        self.simulated_eliminated_ids.clear();
                                        if m == ModelArchetype::BimCraftBuilding3D {
                                            self.render_mode = Render3DMode::CsgSolidBrep;
                                        }
                                    }
                                }
                            });

                        ui.add_space(6.0);
                        ui.separator();
                        ui.add_space(6.0);

                        // ViewCube Camera Presets
                        icons::draw_icon(ui, AppIcon::Camera, 13.0, theme.text_secondary);
                        if ui.add(egui::Button::new(RichText::new("Iso").size(11.0).color(theme.text_secondary))
                            .fill(theme.bg_card).corner_radius(CornerRadius::same(4))).clicked() {
                            self.cam_yaw = 0.65;
                            self.cam_pitch = 0.35;
                        }
                        if ui.add(egui::Button::new(RichText::new("Front").size(11.0).color(theme.text_secondary))
                            .fill(theme.bg_card).corner_radius(CornerRadius::same(4))).clicked() {
                            self.cam_yaw = 0.0;
                            self.cam_pitch = 0.0;
                        }
                        if ui.add(egui::Button::new(RichText::new("Top").size(11.0).color(theme.text_secondary))
                            .fill(theme.bg_card).corner_radius(CornerRadius::same(4))).clicked() {
                            self.cam_yaw = 0.0;
                            self.cam_pitch = 1.50;
                        }
                        if ui.add(egui::Button::new(RichText::new("Side").size(11.0).color(theme.text_secondary))
                            .fill(theme.bg_card).corner_radius(CornerRadius::same(4))).clicked() {
                            self.cam_yaw = 1.57;
                            self.cam_pitch = 0.0;
                        }
                        if icons::icon_button(
                            ui,
                            AppIcon::ZoomFit,
                            "Fit",
                            10.5,
                            theme.accent_blue,
                            theme.bg_card,
                            Stroke::NONE,
                            CornerRadius::same(4),
                            Vec2::new(0.0, 24.0),
                        )
                        .clicked()
                        {
                            self.cam_zoom = 1.15;
                            self.cam_pan = Vec2::ZERO;
                        }

                        // Perspective toggle
                        if ui.selectable_label(self.perspective, "Persp").clicked() {
                            self.perspective = !self.perspective;
                        }

                        ui.separator();

                        // Freeze Canvas Toggle (shared with page scroll guard)
                        let freeze_bg = if universal_canvas.is_frozen {
                            theme.accent_blue
                        } else {
                            theme.bg_card
                        };
                        let freeze_fg = if universal_canvas.is_frozen {
                            Color32::WHITE
                        } else {
                            theme.accent_orange
                        };
                        let freeze_stroke = Stroke::new(
                            1.0,
                            if universal_canvas.is_frozen {
                                theme.accent_blue
                            } else {
                                theme.accent_orange.gamma_multiply(0.6)
                            },
                        );
                        if icons::icon_button(
                            ui,
                            if universal_canvas.is_frozen {
                                AppIcon::Snowflake
                            } else {
                                AppIcon::Unlock
                            },
                            if universal_canvas.is_frozen {
                                "Frozen (Scroll Page)"
                            } else {
                                "Freeze Canvas"
                            },
                            10.5,
                            freeze_fg,
                            freeze_bg,
                            freeze_stroke,
                            CornerRadius::same(5),
                            Vec2::new(0.0, 24.0),
                        )
                        .on_hover_text("Freeze 3D viewport zoom so vertical scrolling moves the page smoothly")
                        .clicked()
                        {
                            universal_canvas.is_frozen = !universal_canvas.is_frozen;
                            ui_kit::set_canvas_frozen(ui.ctx(), universal_canvas.is_frozen);
                        }

                        if icons::icon_button(
                            ui,
                            AppIcon::SaveDisk,
                            "Save 1-File",
                            10.5,
                            Color32::WHITE,
                            theme.accent_green,
                            Stroke::NONE,
                            CornerRadius::same(5),
                            Vec2::new(0.0, 24.0),
                        )
                        .on_hover_text("Save unified workspace (.eigen.json)")
                        .clicked()
                        {
                            universal_canvas.export_unified_file(
                                &self.bim_bus.project,
                                members,
                                parametric_variables,
                                workspace_settings,
                            );
                            if let Some(ref msg) = universal_canvas.status_message {
                                self.modify_status_msg = Some(msg.clone());
                            }
                        }

                        if icons::icon_button(
                            ui,
                            AppIcon::UploadImport,
                            "Import",
                            10.5,
                            theme.text_secondary,
                            theme.bg_card,
                            Stroke::NONE,
                            CornerRadius::same(5),
                            Vec2::new(0.0, 24.0),
                        )
                        .on_hover_text("Import unified .eigen.json workspace archive")
                        .clicked()
                        {
                            universal_canvas.import_json_open = true;
                        }

                        // Right Status Badges
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if !self.simulated_eliminated_ids.is_empty() {
                                icons::pill_badge(
                                    ui,
                                    AppIcon::AlertTriangle,
                                    format!(
                                        "N-{} DISRUPTION: MEMBERS {:?}",
                                        self.simulated_eliminated_ids.len(),
                                        self.simulated_eliminated_ids
                                    ),
                                    theme.accent_red,
                                );
                            } else {
                                icons::pill_badge(
                                    ui,
                                    AppIcon::Palette,
                                    self.color_mode.clone(),
                                    theme.accent_blue,
                                );
                            }

                            if self.selected_member_id.is_some()
                                && icons::icon_button(
                                    ui,
                                    AppIcon::Close,
                                    "Clear",
                                    10.5,
                                    theme.text_secondary,
                                    theme.bg_card,
                                    Stroke::NONE,
                                    CornerRadius::same(4),
                                    Vec2::new(0.0, 22.0),
                                )
                                .clicked()
                            {
                                self.selected_member_id = None;
                            }
                        });
                    });
                });

            // 3D Canvas Viewport Container (Fills Available Screen Space)
            let avail_size = ui.available_size();
            let canvas_width = avail_size.x.max(240.0);
            let canvas_height = match stacked_height {
                Some(h) => h,
                None => (avail_size.y - 10.0).max(360.0),
            };

            Frame::new()
                .fill(Color32::from_rgb(14, 16, 22))
                .stroke(Stroke::new(1.0, theme.border_subtle))
                .inner_margin(Margin::same(0))
                .show(ui, |ui| {
                    let (response, painter) = ui.allocate_painter(Vec2::new(canvas_width, canvas_height), Sense::click_and_drag());
                    let rect = response.rect;
                    self.last_viewport_rect = Some(rect);

                    // Mouse Orbit, Pan, and Zoom Controls
                    if response.dragged_by(egui::PointerButton::Secondary) || (response.dragged() && ui.input(|i| i.modifiers.shift)) {
                        self.cam_pan += response.drag_delta();
                    } else if response.dragged_by(egui::PointerButton::Primary) {
                        self.cam_yaw = (self.cam_yaw + response.drag_delta().x * 0.008).rem_euclid(2.0 * std::f32::consts::PI);
                        self.cam_pitch = (self.cam_pitch - response.drag_delta().y * 0.008).clamp(-1.52, 1.52);
                    }

                    if response.hovered() && !crate::ui_kit::is_canvas_frozen(ui.ctx()) {
                        let scroll_y = ui.input(|i| i.smooth_scroll_delta.y);
                        if scroll_y.abs() > 0.0 {
                            self.cam_zoom = (self.cam_zoom * (1.0 + scroll_y * 0.0025)).clamp(0.15, 8.0);
                        }
                    }

                    // 1. Background Grid & Deep Viewport Shader
                    painter.rect_filled(rect, CornerRadius::same(0), Color32::from_rgb(15, 17, 24));

                    // Geometry Definitions & Model Centers
                    let (center_3d, base_scale_unit, raw_members) = self.build_model_geometry(members);

                    // 3D Projector Function with Camera Transformations and Z-depth
                    let is_persp = self.perspective;
                    let cam_yaw = self.cam_yaw;
                    let cam_pitch = self.cam_pitch;
                    let cam_zoom = self.cam_zoom;
                    let cam_pan = self.cam_pan;
                    let center_screen = rect.center();

                    let project_3d = move |x: f32, y: f32, z: f32| -> (Pos2, f32) {
                        let dx = x - center_3d.0;
                        let dy = y - center_3d.1;
                        let dz = z - center_3d.2;

                        // Yaw rotation around Y axis
                        let cos_y = cam_yaw.cos();
                        let sin_y = cam_yaw.sin();
                        let x1 = dx * cos_y + dz * sin_y;
                        let z1 = -dx * sin_y + dz * cos_y;

                        // Pitch rotation around X axis
                        let cos_p = cam_pitch.cos();
                        let sin_p = cam_pitch.sin();
                        let y2 = dy * cos_p - z1 * sin_p;
                        let z2 = dy * sin_p + z1 * cos_p;

                        // Camera distance & perspective factor
                        let d_cam = 55.0;
                        let persp_factor = if is_persp {
                            (d_cam / (d_cam + z2)).clamp(0.3, 3.0)
                        } else {
                            1.0
                        };

                        let scale = base_scale_unit * cam_zoom * persp_factor;
                        let sx = center_screen.x + cam_pan.x + x1 * scale;
                        let sy = center_screen.y + cam_pan.y - y2 * scale;

                        (Pos2::new(sx, sy), z2)
                    };

                    // 2. 3D Ground Grid Plane
                    if self.show_grid {
                        let grid_size = 6;
                        let grid_step = 4.0;
                        for i in -grid_size..=grid_size {
                            let fi = i as f32 * grid_step;
                            let (p1, _) = project_3d(fi, 0.0, -grid_size as f32 * grid_step);
                            let (p2, _) = project_3d(fi, 0.0, grid_size as f32 * grid_step);
                            let alpha = if i == 0 { 40 } else { 15 };
                            painter.line_segment([p1, p2], Stroke::new(0.6, Color32::from_rgba_premultiplied(100, 150, 220, alpha)));

                            let (p3, _) = project_3d(-grid_size as f32 * grid_step, 0.0, fi);
                            let (p4, _) = project_3d(grid_size as f32 * grid_step, 0.0, fi);
                            painter.line_segment([p3, p4], Stroke::new(0.6, Color32::from_rgba_premultiplied(100, 150, 220, alpha)));
                        }
                    }

                    // Phase 26: 3D Level Elevation Datums (Revit-Style Floor Datum Planes)
                    if self.show_level_datums {
                        let level_data = [
                            ("Level 0 (GF)", 0.0),
                            ("Level 1 (+4.0m)", 4.0),
                            ("Level 2 (+8.0m)", 8.0),
                            ("Level 3 Roof (+12.0m)", 12.0),
                        ];
                        for (lvl_name, elev) in level_data {
                            let (sp1, _) = project_3d(-10.0, elev, 0.0);
                            let (sp2, _) = project_3d(10.0, elev, 0.0);
                            painter.line_segment([sp1, sp2], Stroke::new(1.0, Color32::from_rgba_premultiplied(59, 130, 246, 90)));
                            // Circular blue datum head bubble
                            painter.circle_filled(sp1, 6.5, Color32::from_rgb(30, 58, 138));
                            painter.circle_stroke(sp1, 6.5, Stroke::new(1.0, Color32::from_rgb(96, 165, 250)));
                            painter.text(sp1 + Vec2::new(9.0, 0.0), Align2::LEFT_CENTER, lvl_name, egui::FontId::monospace(9.0), Color32::from_rgb(191, 219, 254));
                        }
                    }

                    // Phase 26: 3D Structural Grid Axis Markers (Bubbles 1-4 & A-D)
                    if self.show_grid_bubbles {
                        let grid_x = [("1", -6.0), ("2", -2.0), ("3", 2.0), ("4", 6.0)];
                        for (lbl, gx) in grid_x {
                            let (gp, _) = project_3d(gx, 0.0, 8.5);
                            painter.circle_filled(gp, 7.5, Color32::from_rgb(20, 24, 34));
                            painter.circle_stroke(gp, 7.5, Stroke::new(1.0, Color32::from_rgb(148, 163, 184)));
                            painter.text(gp, Align2::CENTER_CENTER, lbl, egui::FontId::monospace(9.5), Color32::WHITE);
                        }
                        let grid_z = [("A", -6.0), ("B", -2.0), ("C", 2.0), ("D", 6.0)];
                        for (lbl, gz) in grid_z {
                            let (gp, _) = project_3d(-8.5, 0.0, gz);
                            painter.circle_filled(gp, 7.5, Color32::from_rgb(20, 24, 34));
                            painter.circle_stroke(gp, 7.5, Stroke::new(1.0, Color32::from_rgb(148, 163, 184)));
                            painter.text(gp, Align2::CENTER_CENTER, lbl, egui::FontId::monospace(9.5), Color32::WHITE);
                        }
                    }

                    // Phase 26: 3D Interactive Section Box Cage
                    if self.section_box_enabled {
                        let bx0 = self.section_box_min_x;
                        let bx1 = self.section_box_max_x;
                        let by0 = self.section_box_min_y;
                        let by1 = self.section_box_max_z_m;
                        let bz0 = self.section_box_min_z;
                        let bz1 = 15.0;

                        let corners = [
                            (bx0, by0, bz0), (bx1, by0, bz0), (bx1, by0, bz1), (bx0, by0, bz1),
                            (bx0, by1, bz0), (bx1, by1, bz0), (bx1, by1, bz1), (bx0, by1, bz1),
                        ];
                        let sc: Vec<Pos2> = corners.iter().map(|&(x, y, z)| project_3d(x, y, z).0).collect();
                        let edges = [
                            (0, 1), (1, 2), (2, 3), (3, 0),
                            (4, 5), (5, 6), (6, 7), (7, 4),
                            (0, 4), (1, 5), (2, 6), (3, 7),
                        ];
                        for &(i, j) in &edges {
                            painter.line_segment([sc[i], sc[j]], Stroke::new(1.2, Color32::from_rgba_premultiplied(6, 182, 212, 160)));
                        }
                        for pt in &sc {
                            painter.circle_filled(*pt, 3.5, Color32::from_rgb(6, 182, 212));
                        }
                    }

                    // 3. Process & Depth-Sort Members (Painter's Algorithm)
                    if self.show_deformed_shape && self.animate_vibration {
                        self.vibration_phase = (self.vibration_phase + 0.08).rem_euclid(2.0 * std::f32::consts::PI);
                        ui.ctx().request_repaint();
                    }
                    let vib_factor = if self.show_deformed_shape && self.animate_vibration {
                        self.vibration_phase.sin()
                    } else if self.show_deformed_shape {
                        1.0
                    } else {
                        0.0
                    };
                    let def_scale = self.deformation_scale;

                    let deform_pt = move |p: (f32, f32, f32)| -> (f32, f32, f32) {
                        if vib_factor.abs() < 1e-4 {
                            p
                        } else {
                            let rel_x = (p.0 - center_3d.0).abs() / 12.0;
                            let sag_y = -(1.0 - (rel_x * rel_x).min(1.0)) * 0.015 * def_scale * vib_factor;
                            let sway_x = (p.1 / 20.0) * 0.012 * def_scale * vib_factor;
                            (p.0 + sway_x, p.1 + sag_y, p.2)
                        }
                    };

                    struct RenderItem {
                        id: usize,
                        p1_screen: Pos2,
                        p2_screen: Pos2,
                        depth: f32,
                        p1_3d: (f32, f32, f32),
                        p2_3d: (f32, f32, f32),
                        name: String,
                        section: String,
                        base_width: f32,
                        force_kn: f32,
                        capacity_kn: f32,
                        utilization: f32,
                        ari: f32,
                        is_critical: bool,
                    }

                    let mut render_list = Vec::with_capacity(raw_members.len());
                    for m in &raw_members {
                        let p1_actual = deform_pt(m.p1);
                        let p2_actual = deform_pt(m.p2);
                        let (sp1, z1) = project_3d(p1_actual.0, p1_actual.1, p1_actual.2);
                        let (sp2, z2) = project_3d(p2_actual.0, p2_actual.1, p2_actual.2);
                        let avg_depth = (z1 + z2) * 0.5;

                        render_list.push(RenderItem {
                            id: m.id,
                            p1_screen: sp1,
                            p2_screen: sp2,
                            depth: avg_depth,
                            p1_3d: p1_actual,
                            p2_3d: p2_actual,
                            name: m.name.clone(),
                            section: m.section.clone(),
                            base_width: m.base_width,
                            force_kn: m.force_kn,
                            capacity_kn: m.capacity_kn,
                            utilization: m.utilization,
                            ari: m.ari,
                            is_critical: m.is_critical,
                        });
                    }

                    // Sort back-to-front by depth (most negative z2 first)
                    render_list.sort_by(|a, b| b.depth.partial_cmp(&a.depth).unwrap_or(std::cmp::Ordering::Equal));

                    // Hover & Click Detection
                    let mut hovered_member: Option<usize> = None;
                    if let Some(mouse_pos) = response.hover_pos() {
                        // Check front-to-back for hit detection
                        for item in render_list.iter().rev() {
                            let dist = distance_to_segment(mouse_pos, item.p1_screen, item.p2_screen);
                            let hit_threshold = (item.base_width * cam_zoom * 0.8).clamp(6.0, 18.0);
                            if dist <= hit_threshold {
                                hovered_member = Some(item.id);
                                break;
                            }
                        }
                    }

                    if response.clicked() {
                        if let Some(id) = hovered_member {
                            self.selected_member_id = Some(id);
                            self.active_sidebar_tab = SidebarTab::Properties;
                        } else {
                            self.selected_member_id = None;
                        }
                    }

                    // 4. Render Members (CSG Solid B-Rep Mesh, Solid Shaded, Wireframe, or 3D Diagrams)
                    let sun_dir = (0.577, 0.577, 0.577);

                    // 4A. True 3D CSG B-Rep Triangulated Mesh + Rebar Cage Renderer (BIMCraft LOD 350)
                    if self.active_model == ModelArchetype::BimCraftBuilding3D && self.render_mode == Render3DMode::CsgSolidBrep {
                        struct TriDraw {
                            p0: Pos2,
                            p1: Pos2,
                            p2: Pos2,
                            depth: f32,
                            fill: Color32,
                            stroke: Stroke,
                        }
                        let mut tris_to_draw: Vec<TriDraw> = Vec::new();
                        let max_y_clip = if self.section_box_enabled { self.section_box_max_z_m } else { 999.0 };

                        let mut push_mesh = |mesh: &[[crate::core::P3; 3]], base_col: Color32, edge_alpha: u8| {
                            for tri in mesh {
                                // Convert BIM (X_mm, Y_mm, Z_mm) -> Viewport 3D (X_m, Z_m_up, Y_m)
                                let v0 = (tri[0][0] as f32 * 0.001, tri[0][2] as f32 * 0.001, tri[0][1] as f32 * 0.001);
                                let v1 = (tri[1][0] as f32 * 0.001, tri[1][2] as f32 * 0.001, tri[1][1] as f32 * 0.001);
                                let v2 = (tri[2][0] as f32 * 0.001, tri[2][2] as f32 * 0.001, tri[2][1] as f32 * 0.001);

                                if v0.1 > max_y_clip && v1.1 > max_y_clip && v2.1 > max_y_clip {
                                    continue;
                                }

                                let ux = v1.0 - v0.0;
                                let uy = v1.1 - v0.1;
                                let uz = v1.2 - v0.2;
                                let vx = v2.0 - v0.0;
                                let vy = v2.1 - v0.1;
                                let vz = v2.2 - v0.2;
                                let nx = uy * vz - uz * vy;
                                let ny = uz * vx - ux * vz;
                                let nz = ux * vy - uy * vx;
                                let nlen = (nx * nx + ny * ny + nz * nz).sqrt().max(1e-5);
                                let dot_sun = ((nx / nlen) * sun_dir.0 + (ny / nlen) * sun_dir.1 + (nz / nlen) * sun_dir.2).abs();
                                let shade = 0.58 + 0.42 * dot_sun;

                                let col = Color32::from_rgba_unmultiplied(
                                    ((base_col.r() as f32) * shade).clamp(0.0, 255.0) as u8,
                                    ((base_col.g() as f32) * shade).clamp(0.0, 255.0) as u8,
                                    ((base_col.b() as f32) * shade).clamp(0.0, 255.0) as u8,
                                    base_col.a(),
                                );

                                let (sp0, z0) = project_3d(v0.0, v0.1, v0.2);
                                let (sp1, z1) = project_3d(v1.0, v1.1, v1.2);
                                let (sp2, z2) = project_3d(v2.0, v2.1, v2.2);
                                let avg_z = (z0 + z1 + z2) / 3.0;

                                tris_to_draw.push(TriDraw {
                                    p0: sp0,
                                    p1: sp1,
                                    p2: sp2,
                                    depth: avg_z,
                                    fill: col,
                                    stroke: Stroke::new(0.4, Color32::from_rgba_unmultiplied(210, 225, 245, edge_alpha)),
                                });
                            }
                        };

                        // 1. Slabs with Boolean Shaft Voids
                        for s in &self.bim_bus.project.slabs {
                            let z = self.bim_bus.project.level_elevation(s.level_id);
                            let mesh = s.mesh_3d(z);
                            push_mesh(&mesh, Color32::from_rgba_unmultiplied(80, 105, 135, 195), 35);
                        }
                        // 2. Walls with CSG Door & Window Voids
                        let wall_col = if self.show_rebar_cage {
                            Color32::from_rgba_unmultiplied(120, 145, 175, 135)
                        } else {
                            Color32::from_rgba_unmultiplied(145, 165, 190, 240)
                        };
                        for w in &self.bim_bus.project.walls {
                            let z_base = self.bim_bus.project.level_elevation(w.base_level_id);
                            let z_top = w
                                .top_level_id
                                .map(|id| self.bim_bus.project.level_elevation(id))
                                .unwrap_or(z_base + w.unconnected_height_mm);
                            let mesh = w.mesh_3d(z_base, z_top);
                            push_mesh(&mesh, wall_col, 45);
                        }
                        // 3. Columns & Footings
                        for c in &self.bim_bus.project.columns {
                            let z_base = self.bim_bus.project.level_elevation(c.base_level_id);
                            let z_top = self.bim_bus.project.level_elevation(c.top_level_id);
                            let mesh = c.mesh_3d(z_base, z_top);
                            push_mesh(&mesh, Color32::from_rgba_unmultiplied(65, 160, 220, 215), 55);
                        }
                        // 4. 3D Runs (True Wide-Flange I-Beams, RC Beams, MEP Ducts)
                        for r in &self.bim_bus.project.runs {
                            let z = self.bim_bus.project.level_elevation(r.level_id);
                            let mesh = r.mesh_3d(z);
                            let rcol = match r.category {
                                crate::core::RunCategory::StructuralBeam if r.is_wf_profile => {
                                    Color32::from_rgba_unmultiplied(245, 158, 11, 245)
                                }
                                crate::core::RunCategory::StructuralBeam => {
                                    if self.show_rebar_cage {
                                        Color32::from_rgba_unmultiplied(45, 185, 130, 145)
                                    } else {
                                        Color32::from_rgba_unmultiplied(45, 185, 130, 235)
                                    }
                                }
                                _ => Color32::from_rgba_unmultiplied(56, 189, 248, 220),
                            };
                            push_mesh(&mesh, rcol, 55);
                        }

                        // Depth-sort all CSG B-Rep triangles back-to-front
                        tris_to_draw.sort_by(|a, b| b.depth.partial_cmp(&a.depth).unwrap_or(std::cmp::Ordering::Equal));
                        for td in &tris_to_draw {
                            painter.add(egui::Shape::convex_polygon(vec![td.p0, td.p1, td.p2], td.fill, td.stroke));
                        }

                        // 5. 3D Rebar Cage Overlay (Longitudinal & Stirrup loops)
                        if self.show_rebar_cage {
                            for bar in &self.bim_bus.project.rebars {
                                if bar.points_3d.len() >= 2 {
                                    let bar_col = if bar.closed_stirrup {
                                        Color32::from_rgb(251, 191, 36)
                                    } else {
                                        Color32::from_rgb(244, 63, 94)
                                    };
                                    let stroke_w = if bar.closed_stirrup { 1.1 } else { 2.0 };
                                    for seg in bar.points_3d.windows(2) {
                                        let v0 = (seg[0][0] as f32 * 0.001, seg[0][2] as f32 * 0.001, seg[0][1] as f32 * 0.001);
                                        let v1 = (seg[1][0] as f32 * 0.001, seg[1][2] as f32 * 0.001, seg[1][1] as f32 * 0.001);
                                        if v0.1 <= max_y_clip || v1.1 <= max_y_clip {
                                            let (sp0, _) = project_3d(v0.0, v0.1, v0.2);
                                            let (sp1, _) = project_3d(v1.0, v1.1, v1.2);
                                            painter.line_segment([sp0, sp1], Stroke::new(stroke_w, bar_col));
                                        }
                                    }
                                }
                            }
                        }
                    }

                    for item in &render_list {
                        let is_sel = self.selected_member_id == Some(item.id);
                        let is_hov = hovered_member == Some(item.id);
                        let is_elim = self.simulated_eliminated_ids.contains(&item.id);

                        if self.active_model == ModelArchetype::BimCraftBuilding3D && self.render_mode == Render3DMode::CsgSolidBrep && !is_sel && !is_hov && !is_elim {
                            continue;
                        }

                        // Base Color Determination
                        let base_color = if is_elim {
                            Color32::from_rgb(255, 45, 45)
                        } else if is_sel {
                            Color32::from_rgb(0, 225, 255)
                        } else {
                            match self.color_mode.as_str() {
                                "ARI" => {
                                    if item.ari >= 0.85 {
                                        Color32::from_rgb(26, 188, 120) // Emerald Green Safe
                                    } else if item.ari >= 0.70 {
                                        Color32::from_rgb(14, 165, 233) // Sky Blue
                                    } else if item.ari >= 0.55 {
                                        Color32::from_rgb(245, 158, 11) // Warning Amber
                                    } else {
                                        Color32::from_rgb(239, 68, 68) // Critical Red
                                    }
                                }
                                "DCR_Stress" => {
                                    if item.utilization >= 0.85 {
                                        Color32::from_rgb(239, 68, 68) // Fiery Red Yielding
                                    } else if item.utilization >= 0.60 {
                                        Color32::from_rgb(245, 158, 11) // Amber
                                    } else if item.utilization >= 0.35 {
                                        Color32::from_rgb(34, 197, 94) // Green
                                    } else {
                                        Color32::from_rgb(59, 130, 246) // Blue Low Stress
                                    }
                                }
                                "Criticality" => {
                                    if item.is_critical {
                                        Color32::from_rgb(244, 63, 94) // Magenta Critical
                                    } else {
                                        Color32::from_rgb(71, 85, 105) // Muted Slate
                                    }
                                }
                                "Tension_Compression" => {
                                    if item.force_kn > 5.0 {
                                        Color32::from_rgb(239, 68, 68) // Red Tension
                                    } else if item.force_kn < -5.0 {
                                        Color32::from_rgb(59, 130, 246) // Blue Compression
                                    } else {
                                        Color32::from_rgb(148, 163, 184) // Neutral Grey
                                    }
                                }
                                _ => Color32::from_rgb(90, 140, 210),
                            }
                        };

                        let p1 = item.p1_screen;
                        let p2 = item.p2_screen;
                        let delta = p2 - p1;
                        let len = delta.length();

                        if len < 1.0 {
                            continue;
                        }

                        let dir = delta / len;
                        let normal = Vec2::new(-dir.y, dir.x);

                        match self.render_mode {
                            Render3DMode::CsgSolidBrep | Render3DMode::SolidShaded => {
                                if !is_elim {
                                    // Compute solid extrusion width
                                    let beam_w = (item.base_width * cam_zoom * 0.8).clamp(3.0, 22.0);

                                    // Shading lighting factor
                                    let dx3d = item.p2_3d.0 - item.p1_3d.0;
                                    let dy3d = item.p2_3d.1 - item.p1_3d.1;
                                    let dz3d = item.p2_3d.2 - item.p1_3d.2;
                                    let len3d = (dx3d * dx3d + dy3d * dy3d + dz3d * dz3d).sqrt().max(0.1);
                                    let t3d = (dx3d / len3d, dy3d / len3d, dz3d / len3d);

                                    // Cross product with sun light
                                    let cross_mag = ((t3d.1 * sun_dir.2 - t3d.2 * sun_dir.1).powi(2)
                                        + (t3d.2 * sun_dir.0 - t3d.0 * sun_dir.2).powi(2)
                                        + (t3d.0 * sun_dir.1 - t3d.1 * sun_dir.0).powi(2)).sqrt();

                                    let shade_factor = 0.65 + 0.35 * cross_mag;
                                    let shaded_r = ((base_color.r() as f32) * shade_factor).clamp(0.0, 255.0) as u8;
                                    let shaded_g = ((base_color.g() as f32) * shade_factor).clamp(0.0, 255.0) as u8;
                                    let shaded_b = ((base_color.b() as f32) * shade_factor).clamp(0.0, 255.0) as u8;
                                    let shaded_col = Color32::from_rgb(shaded_r, shaded_g, shaded_b);

                                    // Extruded Quad Corners
                                    let half_w = normal * (beam_w * 0.5);
                                    let q1 = p1 + half_w;
                                    let q2 = p2 + half_w;
                                    let q3 = p2 - half_w;
                                    let q4 = p1 - half_w;

                                    // Fill solid polygon
                                    painter.add(egui::Shape::convex_polygon(vec![q1, q2, q3, q4], shaded_col, Stroke::NONE));

                                    // Chamfer metallic specular highlight edge (top ridge)
                                    let highlight_col = Color32::from_rgba_premultiplied(255, 255, 255, if is_sel { 120 } else { 45 });
                                    painter.line_segment([q1, q2], Stroke::new(1.0, highlight_col));

                                    // Dark drop shadow edge (bottom ridge)
                                    let shadow_col = Color32::from_rgba_premultiplied(0, 0, 0, 80);
                                    painter.line_segment([q4, q3], Stroke::new(1.0, shadow_col));

                                    // Halo if selected or hovered
                                    if is_sel || is_hov {
                                        let halo_col = if is_sel { Color32::from_rgb(0, 230, 255) } else { Color32::from_rgb(255, 255, 255) };
                                        painter.line_segment([q1, q2], Stroke::new(2.0, halo_col));
                                        painter.line_segment([q4, q3], Stroke::new(2.0, halo_col));
                                    }
                                } else {
                                    let stroke_w = if is_sel { 3.5 } else if is_hov { 2.5 } else { 2.8 };
                                    painter.line_segment([p1, p2], Stroke::new(stroke_w, base_color));
                                }
                            }
                            Render3DMode::Wireframe => {
                                let stroke_w = if is_sel { 3.5 } else if is_hov { 2.5 } else if is_elim { 2.8 } else { 1.8 };
                                painter.line_segment([p1, p2], Stroke::new(stroke_w, base_color));
                            }
                            Render3DMode::AxialForceNFD => {
                                // Baseline member axis
                                painter.line_segment([p1, p2], Stroke::new(1.2, Color32::from_rgb(148, 163, 184)));

                                // Diagram height scaled to axial force
                                let is_tension = item.force_kn >= 0.0;
                                let sign = if is_tension { 1.0 } else { -1.0 };
                                let diag_h = (item.force_kn.abs() / 8.0).clamp(4.0, 32.0) * cam_zoom.clamp(0.5, 2.5);
                                let offset_vec = normal * (sign * diag_h);

                                let d1 = p1 + offset_vec;
                                let d2 = p2 + offset_vec;

                                let fill_col = if is_tension {
                                    Color32::from_rgba_premultiplied(239, 68, 68, 140) // Red Tension (+)
                                } else {
                                    Color32::from_rgba_premultiplied(59, 130, 246, 140) // Blue Compression (-)
                                };
                                let border_col = if is_tension {
                                    Color32::from_rgb(239, 68, 68)
                                } else {
                                    Color32::from_rgb(59, 130, 246)
                                };

                                painter.add(egui::Shape::convex_polygon(vec![p1, d1, d2, p2], fill_col, Stroke::NONE));
                                painter.line_segment([d1, d2], Stroke::new(1.8, border_col));
                                painter.line_segment([p1, d1], Stroke::new(1.0, border_col));
                                painter.line_segment([p2, d2], Stroke::new(1.0, border_col));

                                // Midpoint annotation
                                if is_sel || is_hov || item.force_kn.abs() > 40.0 {
                                    let mid = p1 + delta * 0.5 + offset_vec * 1.15;
                                    let sign_str = if is_tension { "(+) " } else { "(-) " };
                                    painter.text(
                                        mid,
                                        Align2::CENTER_CENTER,
                                        format!("{}{:.1} kN", sign_str, item.force_kn.abs()),
                                        egui::FontId::monospace(9.5),
                                        Color32::WHITE,
                                    );
                                }
                            }
                            Render3DMode::ShearForceSFD => {
                                // Baseline member axis
                                painter.line_segment([p1, p2], Stroke::new(1.2, Color32::from_rgb(148, 163, 184)));

                                let vy = (item.force_kn.abs() * 0.28).clamp(3.5, 55.0);
                                let diag_h = (vy / 4.0).clamp(4.0, 26.0) * cam_zoom.clamp(0.5, 2.5);
                                let offset_vec = normal * diag_h;

                                let d1 = p1 + offset_vec;
                                let d2 = p2 - offset_vec;
                                let p_mid = p1 + delta * 0.5;
                                let d_mid1 = p_mid + offset_vec;
                                let d_mid2 = p_mid - offset_vec;

                                let fill_col = Color32::from_rgba_premultiplied(34, 197, 94, 130);
                                let border_col = Color32::from_rgb(34, 197, 94);

                                // Step 1: left half
                                painter.add(egui::Shape::convex_polygon(vec![p1, d1, d_mid1, p_mid], fill_col, Stroke::NONE));
                                painter.line_segment([d1, d_mid1], Stroke::new(1.8, border_col));
                                painter.line_segment([p1, d1], Stroke::new(1.0, border_col));

                                // Step 2: right half (opposite sign)
                                painter.add(egui::Shape::convex_polygon(vec![p_mid, d_mid2, d2, p2], fill_col, Stroke::NONE));
                                painter.line_segment([d_mid2, d2], Stroke::new(1.8, border_col));
                                painter.line_segment([p2, d2], Stroke::new(1.0, border_col));
                                painter.line_segment([d_mid1, d_mid2], Stroke::new(1.5, border_col));

                                if is_sel || is_hov || vy > 20.0 {
                                    painter.text(
                                        d1 + normal * 4.0,
                                        Align2::CENTER_CENTER,
                                        format!("Vy: {:.1} kN", vy),
                                        egui::FontId::monospace(9.5),
                                        Color32::from_rgb(52, 211, 153),
                                    );
                                }
                            }
                            Render3DMode::BendingMomentBMD => {
                                // Baseline member axis
                                painter.line_segment([p1, p2], Stroke::new(1.2, Color32::from_rgb(148, 163, 184)));

                                let my = (item.force_kn.abs() * 0.22).clamp(4.0, 75.0);
                                let diag_h = (my / 3.5).clamp(5.0, 32.0) * cam_zoom.clamp(0.5, 2.5);
                                let offset_vec = normal * diag_h;

                                // Build parabolic polygon with 8 subdivision points
                                let mut poly_pts = vec![p1];
                                let mut curve_pts = Vec::new();
                                let steps = 8;
                                for s in 0..=steps {
                                    let t = s as f32 / steps as f32;
                                    let base_pt = p1 + delta * t;
                                    let sag = 4.0 * t * (1.0 - t); // Parabolic peak at 1.0
                                    let curve_pt = base_pt + offset_vec * sag;
                                    curve_pts.push(curve_pt);
                                    poly_pts.push(curve_pt);
                                }
                                poly_pts.push(p2);

                                let fill_col = Color32::from_rgba_premultiplied(245, 158, 11, 130);
                                let border_col = Color32::from_rgb(245, 158, 11);

                                painter.add(egui::Shape::convex_polygon(poly_pts, fill_col, Stroke::NONE));
                                for w in curve_pts.windows(2) {
                                    painter.line_segment([w[0], w[1]], Stroke::new(2.0, border_col));
                                }
                                painter.line_segment([p1, curve_pts[0]], Stroke::new(1.0, border_col));
                                painter.line_segment([p2, *curve_pts.last().unwrap()], Stroke::new(1.0, border_col));

                                // Midpoint peak callout
                                if is_sel || is_hov || my > 25.0 {
                                    let mid_peak = p1 + delta * 0.5 + offset_vec * 1.15;
                                    painter.text(
                                        mid_peak,
                                        Align2::CENTER_CENTER,
                                        format!("M_max: {:.1} kNm", my),
                                        egui::FontId::monospace(9.5),
                                        Color32::from_rgb(251, 191, 36),
                                    );
                                }
                            }
                        }

                        // Spherical MERO Node Balls at Joint Ends
                        let node_r = (item.base_width * cam_zoom * 0.55).clamp(2.5, 7.5);
                        painter.circle_filled(p1, node_r, Color32::from_rgb(210, 220, 235));
                        painter.circle_stroke(p1, node_r, Stroke::new(0.8, Color32::from_rgb(80, 95, 120)));
                        painter.circle_filled(p2, node_r, Color32::from_rgb(210, 220, 235));
                        painter.circle_stroke(p2, node_r, Stroke::new(0.8, Color32::from_rgb(80, 95, 120)));

                        // Specular dot on sphere
                        painter.circle_filled(p1 - Vec2::new(node_r * 0.3, node_r * 0.3), node_r * 0.35, Color32::WHITE);
                        painter.circle_filled(p2 - Vec2::new(node_r * 0.3, node_r * 0.3), node_r * 0.35, Color32::WHITE);
                    }

                    // 4C. Phase 29: 3D Spatial Clash Wireframe Bounding Boxes & Pins
                    if self.show_3d_clash_markers {
                        for item in &self.clash_items {
                            if item.status != crate::models::ClashStatus::Resolved
                                && !item.is_triggered_at_tolerance(self.clash_tolerance_mm)
                            {
                                continue;
                            }
                            let is_sel = self.selected_clash_id == Some(item.id);
                            let (cr, cg, cb) = if item.status == crate::models::ClashStatus::Resolved {
                                (16, 185, 129)
                            } else {
                                item.severity.color_rgb()
                            };
                            let box_col = Color32::from_rgb(cr, cg, cb);

                            let [cx, cy, cz] = item.point_3d;
                            let [hx, hy, hz] = item.box_half_extents;
                            let corners_3d = [
                                (cx - hx, cy - hy, cz - hz),
                                (cx + hx, cy - hy, cz - hz),
                                (cx + hx, cy + hy, cz - hz),
                                (cx - hx, cy + hy, cz - hz),
                                (cx - hx, cy - hy, cz + hz),
                                (cx + hx, cy - hy, cz + hz),
                                (cx + hx, cy + hy, cz + hz),
                                (cx - hx, cy + hy, cz + hz),
                            ];
                            let pts: [Pos2; 8] = corners_3d.map(|(x, y, z)| project_3d(x, y, z).0);
                            let edges = [
                                (0, 1), (1, 2), (2, 3), (3, 0),
                                (4, 5), (5, 6), (6, 7), (7, 4),
                                (0, 4), (1, 5), (2, 6), (3, 7),
                            ];
                            let edge_stroke = Stroke::new(
                                if is_sel { 2.2 } else { 1.2 },
                                if is_sel { box_col } else { box_col.gamma_multiply(0.85) },
                            );
                            for (a, b) in edges {
                                painter.line_segment([pts[a], pts[b]], edge_stroke);
                            }

                            let (p_c, _) = project_3d(cx, cy, cz);
                            painter.circle_filled(
                                p_c,
                                if is_sel { 12.0 } else { 7.5 },
                                box_col.gamma_multiply(if is_sel { 0.38 } else { 0.24 }),
                            );
                            painter.circle_filled(p_c, if is_sel { 5.5 } else { 3.8 }, box_col);
                            if is_sel {
                                painter.circle_stroke(p_c, 13.5, Stroke::new(1.8, Color32::WHITE));
                            }
                            painter.text(
                                p_c - Vec2::new(0.0, 14.0),
                                Align2::CENTER_BOTTOM,
                                &item.tag,
                                egui::FontId::monospace(if is_sel { 10.5 } else { 9.0 }),
                                if is_sel { Color32::WHITE } else { box_col },
                            );
                        }
                    }

                    // 5. Global Coordinate XYZ Axis Triad (Bottom Left HUD)
                    if self.show_axes {
                        let triad_center = Pos2::new(rect.left() + 45.0, rect.bottom() - 45.0);
                        let triad_len = 28.0;

                        // Project axis directions using camera rotations
                        let (ax_x, _) = project_3d(center_3d.0 + 1.0, center_3d.1, center_3d.2);
                        let (ax_y, _) = project_3d(center_3d.0, center_3d.1 + 1.0, center_3d.2);
                        let (ax_z, _) = project_3d(center_3d.0, center_3d.1, center_3d.2 + 1.0);
                        let (ax_0, _) = project_3d(center_3d.0, center_3d.1, center_3d.2);

                        let dx_screen = (ax_x - ax_0).normalized() * triad_len;
                        let dy_screen = (ax_y - ax_0).normalized() * triad_len;
                        let dz_screen = (ax_z - ax_0).normalized() * triad_len;

                        painter.circle_filled(triad_center, 4.0, Color32::WHITE);
                        painter.line_segment([triad_center, triad_center + dx_screen], Stroke::new(2.5, Color32::from_rgb(244, 63, 94))); // X (Red)
                        painter.line_segment([triad_center, triad_center + dy_screen], Stroke::new(2.5, Color32::from_rgb(34, 197, 94))); // Y (Green)
                        painter.line_segment([triad_center, triad_center + dz_screen], Stroke::new(2.5, Color32::from_rgb(59, 130, 246))); // Z (Blue)

                        painter.text(triad_center + dx_screen * 1.25, Align2::CENTER_CENTER, "X", egui::FontId::monospace(10.0), Color32::from_rgb(244, 63, 94));
                        painter.text(triad_center + dy_screen * 1.25, Align2::CENTER_CENTER, "Y", egui::FontId::monospace(10.0), Color32::from_rgb(34, 197, 94));
                        painter.text(triad_center + dz_screen * 1.25, Align2::CENTER_CENTER, "Z", egui::FontId::monospace(10.0), Color32::from_rgb(59, 130, 246));
                    }

                    // 6. Camera Viewport HUD Overlay (Top Left & Bottom Right)
                    let mode_str = match self.render_mode {
                        Render3DMode::CsgSolidBrep => "CSG SOLID B-REP + 3D REBAR",
                        Render3DMode::SolidShaded => "SOLID EXTRUDED",
                        Render3DMode::Wireframe => "WIREFRAME",
                        Render3DMode::AxialForceNFD => "NFD DIAGRAM (AXIAL)",
                        Render3DMode::ShearForceSFD => "SFD DIAGRAM (SHEAR)",
                        Render3DMode::BendingMomentBMD => "BMD DIAGRAM (MOMENT)",
                    };
                    painter.text(
                        Pos2::new(rect.left() + 14.0, rect.top() + 14.0),
                        Align2::LEFT_TOP,
                        format!("3D VIEWPORT: {} • {}", self.active_model.label(), mode_str),
                        egui::FontId::monospace(10.5),
                        Color32::from_rgb(140, 165, 205),
                    );

                    painter.text(
                        Pos2::new(rect.right() - 14.0, rect.bottom() - 14.0),
                        Align2::RIGHT_BOTTOM,
                        format!("Yaw: {:.0}° | Pitch: {:.0}° | Zoom: {:.2}x | Elements: {}", self.cam_yaw.to_degrees(), self.cam_pitch.to_degrees(), self.cam_zoom, raw_members.len()),
                        egui::FontId::monospace(9.5),
                        Color32::from_rgb(110, 130, 160),
                    );

                    if crate::ui_kit::is_canvas_frozen(ui.ctx()) {
                        let badge_pos = Pos2::new(rect.left() + 14.0, rect.bottom() - 14.0);
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
                        icons::paint_icon(&painter, icon_rect, AppIcon::Snowflake, theme.accent_blue);
                        painter.text(
                            Pos2::new(text_rect.left() + 24.0, text_rect.center().y),
                            Align2::LEFT_CENTER,
                            "CANVAS FROZEN • Scroll moves page vertically",
                            egui::FontId::proportional(10.5),
                            theme.text_primary,
                        );
                    }

                    // 7. Floating Hover Glass Tooltip
                    if let Some(hid) = hovered_member {
                        if let Some(item) = render_list.iter().find(|i| i.id == hid) {
                            if let Some(mpos) = response.hover_pos() {
                                let tip_pos = mpos + Vec2::new(14.0, -18.0);
                                let tip_rect = egui::Rect::from_min_size(tip_pos, Vec2::new(195.0, 90.0));

                                painter.rect_filled(tip_rect, CornerRadius::same(6), Color32::from_rgba_premultiplied(20, 24, 34, 235));
                                painter.rect_stroke(tip_rect, CornerRadius::same(6), Stroke::new(1.0, theme.accent_blue), egui::StrokeKind::Outside);

                                painter.text(tip_pos + Vec2::new(8.0, 6.0), Align2::LEFT_TOP, format!("{} [{}]", item.name, item.section), egui::FontId::proportional(11.0), Color32::WHITE);
                                painter.text(tip_pos + Vec2::new(8.0, 22.0), Align2::LEFT_TOP, format!("Axial Pu: {:.1} kN", item.force_kn), egui::FontId::monospace(10.0), theme.accent_blue);
                                painter.text(tip_pos + Vec2::new(8.0, 38.0), Align2::LEFT_TOP, format!("Capacity ϕPn: {:.1} kN", item.capacity_kn), egui::FontId::monospace(10.0), theme.text_secondary);
                                painter.text(tip_pos + Vec2::new(8.0, 54.0), Align2::LEFT_TOP, format!("Utilization DCR: {:.1}%", item.utilization * 100.0), egui::FontId::monospace(10.0), theme.accent_orange);
                                painter.text(tip_pos + Vec2::new(8.0, 70.0), Align2::LEFT_TOP, format!("ARI Redundancy: {:.3}", item.ari), egui::FontId::monospace(10.0), theme.accent_green);
                            }
                        }
                    }
                });

            ui.add_space(4.0);
            // Phase 26 / 29: Responsive Revit-Style Viewport Control Strip
            Frame::new()
                .fill(theme.bg_sidebar)
                .inner_margin(Margin::symmetric(10, 4))
                .stroke(Stroke::new(1.0, theme.border_subtle))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        // Scale
                        ui.label(RichText::new("Scale:").size(10.0).color(theme.text_muted));
                        egui::ComboBox::from_id_salt("viewport_scale_combo")
                            .selected_text(RichText::new(&self.view_scale).size(10.0).color(theme.text_primary))
                            .show_ui(ui, |ui| {
                                for s in ["1:50", "1:100", "1:200", "1:500"] {
                                    if ui.selectable_label(self.view_scale == s, s).clicked() {
                                        self.view_scale = s.to_string();
                                    }
                                }
                            });

                        ui.separator();
                        // LOD Detail
                        ui.label(RichText::new("Detail:").size(10.0).color(theme.text_muted));
                        egui::ComboBox::from_id_salt("viewport_lod_combo")
                            .selected_text(RichText::new(&self.lod_level).size(10.0).color(theme.accent_blue))
                            .show_ui(ui, |ui| {
                                for l in ["LOD 100 (Massing)", "LOD 200 (Design)", "LOD 350 (Fine BIM Detail)", "LOD 400 (Fabrication)"] {
                                    if ui.selectable_label(self.lod_level == l, l).clicked() {
                                        self.lod_level = l.to_string();
                                    }
                                }
                            });

                        ui.separator();
                        // Visual toggles
                        if icons::icon_button(ui, AppIcon::Sun, "Sun", 10.0, if self.show_sun_shadows { Color32::WHITE } else { theme.text_secondary }, if self.show_sun_shadows { theme.accent_blue } else { theme.bg_card }, Stroke::NONE, CornerRadius::same(4), Vec2::new(0.0, 22.0)).clicked() {
                            self.show_sun_shadows = !self.show_sun_shadows;
                        }
                        if icons::icon_button(ui, AppIcon::RulerSquare, "Datums", 10.0, if self.show_level_datums { Color32::WHITE } else { theme.text_secondary }, if self.show_level_datums { theme.accent_blue } else { theme.bg_card }, Stroke::NONE, CornerRadius::same(4), Vec2::new(0.0, 22.0)).clicked() {
                            self.show_level_datums = !self.show_level_datums;
                        }
                        if icons::icon_button(ui, AppIcon::Grid, "Grids", 10.0, if self.show_grid_bubbles { Color32::WHITE } else { theme.text_secondary }, if self.show_grid_bubbles { theme.accent_blue } else { theme.bg_card }, Stroke::NONE, CornerRadius::same(4), Vec2::new(0.0, 22.0)).clicked() {
                            self.show_grid_bubbles = !self.show_grid_bubbles;
                        }
                        if icons::icon_button(ui, AppIcon::BoxPackage, "Section Box", 10.0, if self.section_box_enabled { Color32::WHITE } else { theme.text_secondary }, if self.section_box_enabled { theme.accent_blue } else { theme.bg_card }, Stroke::NONE, CornerRadius::same(4), Vec2::new(0.0, 22.0)).clicked() {
                            self.section_box_enabled = !self.section_box_enabled;
                        }
                        if icons::icon_button(ui, AppIcon::Target, "Isolate", 10.0, if self.temporary_isolate_active { Color32::WHITE } else { theme.text_secondary }, if self.temporary_isolate_active { theme.accent_blue } else { theme.bg_card }, Stroke::NONE, CornerRadius::same(4), Vec2::new(0.0, 22.0)).clicked() {
                            self.temporary_isolate_active = !self.temporary_isolate_active;
                        }
                        if icons::icon_button(ui, AppIcon::AlertTriangle, "Clashes", 10.0, if self.show_3d_clash_markers { Color32::WHITE } else { theme.text_secondary }, if self.show_3d_clash_markers { theme.accent_blue } else { theme.bg_card }, Stroke::NONE, CornerRadius::same(4), Vec2::new(0.0, 22.0)).clicked() {
                            self.show_3d_clash_markers = !self.show_3d_clash_markers;
                        }

                        ui.separator();
                        icons::icon_label(ui, AppIcon::CheckCircle, 11.0, theme.accent_green, format!("{} Elements", members.len()), 10.0, theme.accent_green, true);
                    });
                });

            ui.add_space(4.0);
            // Phase 26: Expandable Bottom BIM Data Deck
            self.render_bottom_deck(ui, theme, members, features);
        });
    }

    /// Builds 3D coordinates and elements based on active architectural archetype.
    /// Streams real 3D coordinates directly from `InternalVault/Demo/NVW_DCR-LOD/*.ifc` when available!
    fn build_model_geometry(&self, fallback_members: &[StructuralMember]) -> ((f32, f32, f32), f32, Vec<Member3D>) {
        match self.active_model {
            ModelArchetype::BimCraftBuilding3D => {
                let fem = self.bim_bus.project.to_structural_model();
                let mut list = Vec::with_capacity(fem.elements.len());
                for (idx, el) in fem.elements.iter().enumerate() {
                    let n1 = &fem.nodes[el.start_node];
                    let n2 = &fem.nodes[el.end_node];
                    // Convert FEM (X, Y_up, Z) -> Viewport (X, Y_up, Z)
                    let p1 = (n1.x as f32, n1.y as f32, n1.z as f32);
                    let p2 = (n2.x as f32, n2.y as f32, n2.z as f32);
                    let is_col = el.name.contains("Col") || el.name.contains("CL");
                    let ari = 0.88 + ((idx % 7) as f32) * 0.015;
                    list.push(Member3D {
                        id: idx + 1,
                        p1,
                        p2,
                        name: el.name.clone(),
                        section: if is_col { "RC 400x400".to_string() } else { "WF / RC Beam".to_string() },
                        base_width: if is_col { 8.5 } else { 6.5 },
                        force_kn: if is_col { -640.0 } else { 215.0 },
                        capacity_kn: if is_col { 1450.0 } else { 520.0 },
                        utilization: 0.42,
                        ari,
                        is_critical: idx == 0,
                    });
                }
                ((6.0, 2.5, 4.5), 26.0, list)
            }
            ModelArchetype::DiagridTower3D => {
                // Stream real NVW_DCR-LOD 3D building geometry from InternalVault if present
                if let Some(ifc_segs) = crate::core::InternalVault::stream_nvw_dcr_lod200_viewport_segments() {
                    let mut list = Vec::with_capacity(ifc_segs.len());
                    for (idx, (p1, p2, name, section)) in ifc_segs.into_iter().enumerate() {
                        let id = idx + 1;
                        let is_col = name.contains("Column");
                        let ari = if is_col {
                            0.91 + ((id % 7) as f32) * 0.01
                        } else {
                            0.86 + ((id % 9) as f32) * 0.012
                        };
                        list.push(Member3D {
                            id,
                            p1,
                            p2,
                            name,
                            section,
                            base_width: if is_col { 7.5 } else { 5.5 },
                            force_kn: if is_col { -1420.0 } else { 385.0 },
                            capacity_kn: if is_col { 2850.0 } else { 780.0 },
                            utilization: if is_col { 0.498 } else { 0.493 },
                            ari,
                            is_critical: id % 29 == 0,
                        });
                    }
                    return ((0.0, 12.0, 0.0), 14.5, list);
                }

                let mut list = Vec::new();
                let tiers = [
                    (0.0, 14.0, 0.0),
                    (16.0, 12.0, 15.0),
                    (32.0, 10.0, 30.0),
                    (46.0, 8.5, 45.0),
                ];

                let mut tier_coords = Vec::new();
                for (z, r, twist_deg) in tiers {
                    let twist_rad = (twist_deg as f32).to_radians();
                    let mut pts = Vec::new();
                    for i in 0..8 {
                        let angle = twist_rad + (i as f32) * (std::f32::consts::PI * 2.0 / 8.0);
                        let x = r * angle.cos();
                        let y = r * angle.sin();
                        pts.push((x, z, y));
                    }
                    tier_coords.push(pts);
                }

                let mut id = 1;
                // Diagrid cross braces between tiers
                for t in 0..3 {
                    let lower = &tier_coords[t];
                    let upper = &tier_coords[t + 1];
                    for i in 0..8 {
                        let next_i = (i + 1) % 8;
                        list.push(Member3D {
                            id,
                            p1: lower[i],
                            p2: upper[next_i],
                            name: format!("Diagrid T{}-D1", t + 1),
                            section: "CHS 610x25".to_string(),
                            base_width: 8.5,
                            force_kn: 340.0,
                            capacity_kn: 850.0,
                            utilization: 0.40,
                            ari: 0.89,
                            is_critical: false,
                        });
                        id += 1;

                        list.push(Member3D {
                            id,
                            p1: lower[next_i],
                            p2: upper[i],
                            name: format!("Diagrid T{}-D2", t + 1),
                            section: "CHS 610x25".to_string(),
                            base_width: 8.5,
                            force_kn: 320.0,
                            capacity_kn: 850.0,
                            utilization: 0.38,
                            ari: 0.89,
                            is_critical: false,
                        });
                        id += 1;
                    }

                    // Perimeter belt ring
                    for i in 0..8 {
                        let next_i = (i + 1) % 8;
                        list.push(Member3D {
                            id,
                            p1: lower[i],
                            p2: lower[next_i],
                            name: format!("Belt Ring T{}", t + 1),
                            section: "W24x146".to_string(),
                            base_width: 10.0,
                            force_kn: -120.0,
                            capacity_kn: 620.0,
                            utilization: 0.19,
                            ari: 0.94,
                            is_critical: false,
                        });
                        id += 1;
                    }
                }

                // Central elevator core columns
                for (cx, cz) in [(-4.0, -4.0), (4.0, -4.0), (4.0, 4.0), (-4.0, 4.0)] {
                    for t in 0..3 {
                        list.push(Member3D {
                            id,
                            p1: (cx, tiers[t].0, cz),
                            p2: (cx, tiers[t + 1].0, cz),
                            name: "Core Mega-Column".to_string(),
                            section: "CFT 1200x50".to_string(),
                            base_width: 12.0,
                            force_kn: 2450.0,
                            capacity_kn: 5200.0,
                            utilization: 0.47,
                            ari: 0.95,
                            is_critical: true,
                        });
                        id += 1;
                    }
                }

                // Crown MERO dome rafters converging to apex
                let apex = (0.0, 54.0, 0.0);
                for pt in &tier_coords[3] {
                    list.push(Member3D {
                        id,
                        p1: *pt,
                        p2: apex,
                        name: "Crown MERO Strut".to_string(),
                        section: "CHS 219.1".to_string(),
                        base_width: 6.0,
                        force_kn: 85.0,
                        capacity_kn: 340.0,
                        utilization: 0.25,
                        ari: 0.98,
                        is_critical: false,
                    });
                    id += 1;
                }

                ((0.0, 24.0, 0.0), 7.8, list)
            }
            ModelArchetype::TrussBridge3D => {
                // Stream LOD 200 columns & primary grid from InternalVault if present
                if let Some(ifc_segs) = crate::core::InternalVault::stream_nvw_dcr_lod200_viewport_segments() {
                    let mut list = Vec::new();
                    for (idx, (p1, p2, name, section)) in ifc_segs.into_iter().enumerate() {
                        // In LOD 200 view, emphasize primary columns and glulam members
                        if name.contains("Column") || name.contains("Member") || idx % 2 == 0 {
                            let id = list.len() + 1;
                            let is_col = name.contains("Column");
                            list.push(Member3D {
                                id,
                                p1,
                                p2,
                                name,
                                section,
                                base_width: if is_col { 8.0 } else { 5.0 },
                                force_kn: if is_col { -1280.0 } else { 310.0 },
                                capacity_kn: if is_col { 2650.0 } else { 720.0 },
                                utilization: 0.46,
                                ari: 0.88 + ((id % 8) as f32) * 0.01,
                                is_critical: id % 31 == 0,
                            });
                        }
                    }
                    if !list.is_empty() {
                        return ((0.0, 12.0, 0.0), 14.5, list);
                    }
                }

                let mut list = Vec::new();
                let mut id = 1;

                // Two parallel trusses at Z = -3.5 and Z = +3.5
                for &z in &[-3.5, 3.5] {
                    let mut bot_pts = Vec::new();
                    for i in 0..=6 {
                        let x = (i as f32) * 4.0;
                        bot_pts.push((x, 0.0, z));
                    }

                    // Parabolic arch rib
                    let mut top_pts = Vec::new();
                    for i in 0..=6 {
                        let x = (i as f32) * 4.0;
                        let arch_y = 6.2 * (1.0 - ((x - 12.0) / 12.0).powi(2)).max(0.0);
                        top_pts.push((x, arch_y, z));
                    }

                    // Bottom chord
                    for i in 0..6 {
                        list.push(Member3D {
                            id,
                            p1: bot_pts[i],
                            p2: bot_pts[i + 1],
                            name: "Bottom Deck Chord".to_string(),
                            section: "W14x90".to_string(),
                            base_width: 9.0,
                            force_kn: -420.0,
                            capacity_kn: 1100.0,
                            utilization: 0.38,
                            ari: 0.92,
                            is_critical: false,
                        });
                        id += 1;
                    }

                    // Arch rib chords
                    for i in 0..6 {
                        list.push(Member3D {
                            id,
                            p1: top_pts[i],
                            p2: top_pts[i + 1],
                            name: "Parabolic Arch Rib".to_string(),
                            section: "Box 800x800".to_string(),
                            base_width: 11.0,
                            force_kn: 680.0,
                            capacity_kn: 1450.0,
                            utilization: 0.47,
                            ari: 0.96,
                            is_critical: true,
                        });
                        id += 1;
                    }

                    // Vertical cable hangers
                    for i in 1..6 {
                        list.push(Member3D {
                            id,
                            p1: bot_pts[i],
                            p2: top_pts[i],
                            name: "Cable Hanger".to_string(),
                            section: "Steel Strand 65mm".to_string(),
                            base_width: 5.0,
                            force_kn: 185.0,
                            capacity_kn: 520.0,
                            utilization: 0.36,
                            ari: 0.88,
                            is_critical: false,
                        });
                        id += 1;
                    }
                }

                // Transverse floor cross beams connecting the two decks
                for i in 0..=6 {
                    let x = (i as f32) * 4.0;
                    list.push(Member3D {
                        id,
                        p1: (x, 0.0, -3.5),
                        p2: (x, 0.0, 3.5),
                        name: "Transverse Floor Beam".to_string(),
                        section: "W24x84".to_string(),
                        base_width: 8.0,
                        force_kn: 95.0,
                        capacity_kn: 480.0,
                        utilization: 0.20,
                        ari: 0.95,
                        is_critical: false,
                    });
                    id += 1;
                }

                ((12.0, 3.0, 0.0), 16.0, list)
            }
            ModelArchetype::SpaceDome3D => {
                let mut list = Vec::new();
                let mut id = 1;
                let r_outer = 16.0;

                let ring_heights = [(0.0, r_outer), (6.0, 13.5), (10.5, 9.0)];
                let mut ring_pts = Vec::new();

                for (y, r) in ring_heights {
                    let mut pts = Vec::new();
                    for i in 0..12 {
                        let angle = (i as f32) * (std::f32::consts::PI * 2.0 / 12.0);
                        let x = r * angle.cos();
                        let z = r * angle.sin();
                        pts.push((x, y, z));
                    }
                    ring_pts.push(pts);
                }

                let apex = (0.0, 13.5, 0.0);

                // Circumferential rings
                for ring in &ring_pts {
                    for i in 0..12 {
                        let next_i = (i + 1) % 12;
                        list.push(Member3D {
                            id,
                            p1: ring[i],
                            p2: ring[next_i],
                            name: "Dome Hoop Ring".to_string(),
                            section: "CHS 168.3".to_string(),
                            base_width: 6.5,
                            force_kn: -85.0,
                            capacity_kn: 420.0,
                            utilization: 0.20,
                            ari: 0.96,
                            is_critical: false,
                        });
                        id += 1;
                    }
                }

                // Meridional ribs
                for i in 0..12 {
                    list.push(Member3D {
                        id,
                        p1: ring_pts[0][i],
                        p2: ring_pts[1][i],
                        name: "Meridional Rib".to_string(),
                        section: "CHS 219.1".to_string(),
                        base_width: 7.5,
                        force_kn: 140.0,
                        capacity_kn: 520.0,
                        utilization: 0.27,
                        ari: 0.95,
                        is_critical: false,
                    });
                    id += 1;

                    list.push(Member3D {
                        id,
                        p1: ring_pts[1][i],
                        p2: ring_pts[2][i],
                        name: "Meridional Rib".to_string(),
                        section: "CHS 219.1".to_string(),
                        base_width: 7.5,
                        force_kn: 125.0,
                        capacity_kn: 520.0,
                        utilization: 0.24,
                        ari: 0.95,
                        is_critical: false,
                    });
                    id += 1;

                    list.push(Member3D {
                        id,
                        p1: ring_pts[2][i],
                        p2: apex,
                        name: "Apex Rafter".to_string(),
                        section: "CHS 219.1".to_string(),
                        base_width: 7.0,
                        force_kn: 95.0,
                        capacity_kn: 480.0,
                        utilization: 0.20,
                        ari: 0.97,
                        is_critical: false,
                    });
                    id += 1;
                }

                ((0.0, 6.0, 0.0), 16.5, list)
            }
            ModelArchetype::WarrenTruss2D => {
                let mut list = Vec::new();
                let mut bot_nodes = Vec::new();
                for i in 0..=6 {
                    bot_nodes.push(((i as f32) * 4.0, 0.0, 0.0));
                }
                let mut top_nodes = Vec::new();
                for i in 1..=5 {
                    top_nodes.push(((i as f32) * 4.0, 3.2, 0.0));
                }

                let mut id = 1;
                for i in 0..6 {
                    list.push(Member3D {
                        id,
                        p1: bot_nodes[i],
                        p2: bot_nodes[i + 1],
                        name: format!("Bottom Chord E{}", id),
                        section: "W14x90".to_string(),
                        base_width: 8.5,
                        force_kn: -45.2,
                        capacity_kn: 120.0,
                        utilization: 0.38,
                        ari: 0.94,
                        is_critical: false,
                    });
                    id += 1;
                }
                for i in 0..4 {
                    list.push(Member3D {
                        id,
                        p1: top_nodes[i],
                        p2: top_nodes[i + 1],
                        name: format!("Top Chord T{}", i + 1),
                        section: "W14x90".to_string(),
                        base_width: 8.5,
                        force_kn: 38.5,
                        capacity_kn: 95.0,
                        utilization: 0.41,
                        ari: 0.91,
                        is_critical: false,
                    });
                    id += 1;
                }
                for i in 0..5 {
                    list.push(Member3D {
                        id,
                        p1: bot_nodes[i],
                        p2: top_nodes[i],
                        name: format!("Diagonal D{}", i + 1),
                        section: "2L 4x4x3/8".to_string(),
                        base_width: 6.0,
                        force_kn: 82.4,
                        capacity_kn: 90.0,
                        utilization: 0.91,
                        ari: 0.52,
                        is_critical: i == 0,
                    });
                    id += 1;
                }

                if list.is_empty() && !fallback_members.is_empty() {
                    // Fallback to active app members
                }

                ((12.0, 1.6, 0.0), 22.0, list)
            }
            ModelArchetype::DoubleLayerSpaceTruss3D => {
                let fem = crate::core::ParametricSpatialWizard::generate_double_layer_space_truss(
                    "Double-Layer Space Truss", self.wizard_space_truss_lx, self.wizard_space_truss_ly, self.wizard_space_truss_depth, 6, 6
                );
                let mut list = Vec::with_capacity(fem.elements.len());
                for (idx, el) in fem.elements.iter().enumerate() {
                    let n1 = &fem.nodes[el.start_node];
                    let n2 = &fem.nodes[el.end_node];
                    let p1 = (n1.x as f32, n1.z as f32, n1.y as f32);
                    let p2 = (n2.x as f32, n2.z as f32, n2.y as f32);
                    let is_top = el.name.starts_with("TC");
                    let is_bot = el.name.starts_with("BC");
                    list.push(Member3D {
                        id: idx + 1,
                        p1,
                        p2,
                        name: el.name.clone(),
                        section: if is_top { "CHS Tube 300x10".to_string() } else if is_bot { "CHS Pipe 150x6".to_string() } else { "CHS Web 114x4.5".to_string() },
                        base_width: if is_top { 7.5 } else if is_bot { 6.5 } else { 4.5 },
                        force_kn: if is_top { -280.0 } else if is_bot { 240.0 } else { 85.0 },
                        capacity_kn: if is_top { 620.0 } else { 480.0 },
                        utilization: 0.45,
                        ari: 0.94,
                        is_critical: false,
                    });
                }
                let cx = (self.wizard_space_truss_lx * 0.5) as f32;
                let cz = (self.wizard_space_truss_ly * 0.5) as f32;
                let cy = (self.wizard_space_truss_depth * 0.5) as f32;
                ((cx, cy, cz), 16.0, list)
            }
            ModelArchetype::GeodesicDome3D => {
                let fem = crate::core::ParametricSpatialWizard::generate_geodesic_dome(
                    "Geodesic Dome", self.wizard_dome_radius, self.wizard_dome_rings, self.wizard_dome_sectors
                );
                let mut list = Vec::with_capacity(fem.elements.len());
                for (idx, el) in fem.elements.iter().enumerate() {
                    let n1 = &fem.nodes[el.start_node];
                    let n2 = &fem.nodes[el.end_node];
                    let p1 = (n1.x as f32, n1.z as f32, n1.y as f32);
                    let p2 = (n2.x as f32, n2.z as f32, n2.y as f32);
                    list.push(Member3D {
                        id: idx + 1,
                        p1,
                        p2,
                        name: el.name.clone(),
                        section: "CHS Pipe 150x6".to_string(),
                        base_width: 5.5,
                        force_kn: -110.0,
                        capacity_kn: 340.0,
                        utilization: 0.32,
                        ari: 0.96,
                        is_critical: false,
                    });
                }
                let r = self.wizard_dome_radius as f32;
                ((0.0, r * 0.4, 0.0), 14.0, list)
            }
            ModelArchetype::HyparRuledShell3D => {
                let fem = crate::core::ParametricSpatialWizard::generate_hypar_shell(
                    "Hypar Shell", self.wizard_hypar_lx, self.wizard_hypar_ly, self.wizard_hypar_sagitta, 8, 8
                );
                let mut list = Vec::with_capacity(fem.elements.len());
                for (idx, el) in fem.elements.iter().enumerate() {
                    let n1 = &fem.nodes[el.start_node];
                    let n2 = &fem.nodes[el.end_node];
                    let p1 = (n1.x as f32, n1.z as f32, n1.y as f32);
                    let p2 = (n2.x as f32, n2.z as f32, n2.y as f32);
                    list.push(Member3D {
                        id: idx + 1,
                        p1,
                        p2,
                        name: el.name.clone(),
                        section: "WF 200x100".to_string(),
                        base_width: 5.0,
                        force_kn: 65.0,
                        capacity_kn: 280.0,
                        utilization: 0.23,
                        ari: 0.95,
                        is_critical: false,
                    });
                }
                let cx = (self.wizard_hypar_lx * 0.5) as f32;
                let cz = (self.wizard_hypar_ly * 0.5) as f32;
                ((cx, 0.0, cz), 16.0, list)
            }
            ModelArchetype::MultiStoryTower3D => {
                let fem = crate::core::ParametricSpatialWizard::generate_multi_story_tower(
                    "Multi-Story Tower", self.wizard_tower_stories, self.wizard_tower_story_h,
                    self.wizard_tower_bays_x, self.wizard_tower_bays_y, 6.0, 6.0
                );
                let mut list = Vec::with_capacity(fem.elements.len());
                for (idx, el) in fem.elements.iter().enumerate() {
                    let n1 = &fem.nodes[el.start_node];
                    let n2 = &fem.nodes[el.end_node];
                    let p1 = (n1.x as f32, n1.z as f32, n1.y as f32);
                    let p2 = (n2.x as f32, n2.z as f32, n2.y as f32);
                    let is_col = el.name.starts_with("COL");
                    list.push(Member3D {
                        id: idx + 1,
                        p1,
                        p2,
                        name: el.name.clone(),
                        section: if is_col { "HSS 200x200x8".to_string() } else { "WF 300x150".to_string() },
                        base_width: if is_col { 7.5 } else { 5.5 },
                        force_kn: if is_col { -850.0 } else { 180.0 },
                        capacity_kn: if is_col { 1900.0 } else { 450.0 },
                        utilization: 0.44,
                        ari: 0.93,
                        is_critical: false,
                    });
                }
                let cx = (self.wizard_tower_bays_x as f32 * 6.0) * 0.5;
                let cz = (self.wizard_tower_bays_y as f32 * 6.0) * 0.5;
                let cy = (self.wizard_tower_stories as f32 * self.wizard_tower_story_h as f32) * 0.5;
                ((cx, cy, cz), 9.0, list)
            }
            ModelArchetype::PrestressedGirderBridge3D => {
                let fem = crate::core::PrestressedConcreteEngine::generate_bridge_archetype_elements(
                    "30m PT Bridge", 30.0, 4, 2.4, 480.0
                );
                let mut list = Vec::with_capacity(fem.elements.len());
                for (idx, el) in fem.elements.iter().enumerate() {
                    let n1 = &fem.nodes[el.start_node];
                    let n2 = &fem.nodes[el.end_node];
                    // Map FEM (x, y, z) -> Viewport (x, z_up, y)
                    let p1 = (n1.x as f32, n1.z as f32, n1.y as f32);
                    let p2 = (n2.x as f32, n2.z as f32, n2.y as f32);
                    let is_tendon = el.name.starts_with("PT_Tendon");
                    let is_diaph = el.name.starts_with("Diaphragm");
                    list.push(Member3D {
                        id: idx + 1,
                        p1,
                        p2,
                        name: el.name.clone(),
                        section: if is_tendon {
                            "36x 0.6\" Low-Relaxation Tendon (1860 MPa)".to_string()
                        } else if is_diaph {
                            "RC Diaphragm 300x1000".to_string()
                        } else {
                            "AASHTO Type IV Precast Girder (1370mm)".to_string()
                        },
                        base_width: if is_tendon { 4.0 } else if is_diaph { 6.0 } else { 9.0 },
                        force_kn: if is_tendon { 4666.0 } else { -420.0 },
                        capacity_kn: if is_tendon { 7030.0 } else { 1200.0 },
                        utilization: if is_tendon { 0.66 } else { 0.35 },
                        ari: if is_tendon { 0.98 } else { 0.92 },
                        is_critical: false,
                    });
                }
                let cx = 15.0;
                let cy = 0.0;
                let cz = 0.0;
                ((cx, cy, cz), 22.0, list)
            }
            ModelArchetype::CoupledShearWallCore3D => {
                let fem = crate::core::ShearWallEngine::generate_coupled_core_archetype(
                    "12-Story Coupled Core", 12, 3.5, 6.0, 350.0, 2.0
                );
                let mut list = Vec::with_capacity(fem.elements.len());
                for (idx, el) in fem.elements.iter().enumerate() {
                    let n_start = fem.nodes.iter().find(|n| n.id == el.start_node);
                    let n_end = fem.nodes.iter().find(|n| n.id == el.end_node);
                    if let (Some(n1), Some(n2)) = (n_start, n_end) {
                        let p1 = (n1.x as f32, n1.z as f32, n1.y as f32);
                        let p2 = (n2.x as f32, n2.z as f32, n2.y as f32);
                        let is_coupling = el.name.starts_with("CouplingBeam");
                        let is_spandrel = el.name.starts_with("Spandrel");
                        list.push(Member3D {
                            id: idx + 1,
                            p1,
                            p2,
                            name: el.name.clone(),
                            section: if is_coupling {
                                "Diagonally Reinforced Coupling Beam 350x800".to_string()
                            } else if is_spandrel {
                                "RC Floor Spandrel 350x600".to_string()
                            } else {
                                "Special Structural Wall Pier 6000x350".to_string()
                            },
                            base_width: if is_coupling { 7.0 } else if is_spandrel { 5.0 } else { 8.5 },
                            force_kn: if is_coupling { 620.0 } else { -3600.0 },
                            capacity_kn: if is_coupling { 950.0 } else { 7200.0 },
                            utilization: if is_coupling { 0.65 } else { 0.50 },
                            ari: if is_coupling { 0.96 } else { 0.94 },
                            is_critical: false,
                        });
                    }
                }
                let cy = 0.0;
                let cz = (12.0 * 3.5) * 0.5;
                ((0.0, cy, cz), 26.0, list)
            }
            ModelArchetype::BaseIsolatedHospital3D => {
                let (nodes, elements) = crate::core::SeismicIsolationEngine::generate_base_isolated_hospital_archetype();
                let mut list = Vec::with_capacity(elements.len());
                for &(el_id, n1_id, n2_id, is_isolator) in &elements {
                    let (_, x1, y1, z1) = nodes[n1_id];
                    let (_, x2, y2, z2) = nodes[n2_id];
                    let p1 = (x1 as f32, z1 as f32, y1 as f32);
                    let p2 = (x2 as f32, z2 as f32, y2 as f32);
                    list.push(Member3D {
                        id: el_id + 1,
                        p1,
                        p2,
                        name: if is_isolator { format!("IsolatorBearing-{}", el_id + 1) } else { format!("FrameMember-{}", el_id + 1) },
                        section: if is_isolator { "Lead-Rubber Bearing LRB Ø850".to_string() } else { "RC Frame Column 700x700 / Beam 400x650".to_string() },
                        base_width: if is_isolator { 8.5 } else { 5.0 },
                        force_kn: if is_isolator { 4000.0 } else { 1200.0 },
                        capacity_kn: if is_isolator { 12000.0 } else { 3500.0 },
                        utilization: if is_isolator { 0.33 } else { 0.45 },
                        ari: if is_isolator { 0.99 } else { 0.95 },
                        is_critical: false,
                    });
                }
                let cx = 12.0;
                let cy = 8.0;
                let cz = (8.0 * 3.6) * 0.5;
                ((cx, cy, cz), 24.0, list)
            }
            ModelArchetype::TimeHistoryTower3D => {
                let (nodes, elements) = crate::core::TimeHistoryEngine::generate_nl_tha_tower_archetype();
                let mut list = Vec::with_capacity(elements.len());
                for &(el_id, n1_id, n2_id, is_link) in &elements {
                    let (_, x1, y1, z1) = nodes[n1_id];
                    let (_, x2, y2, z2) = nodes[n2_id];
                    let p1 = (x1 as f32, z1 as f32, y1 as f32);
                    let p2 = (x2 as f32, z2 as f32, y2 as f32);
                    list.push(Member3D {
                        id: el_id + 1,
                        p1,
                        p2,
                        name: if is_link { format!("InelasticLink-{}", el_id + 1) } else { format!("TowerMember-{}", el_id + 1) },
                        section: if is_link { "Bouc-Wen Inelastic Link".to_string() } else { "Built-Up Box Column 800x800 / W36x150 Beam".to_string() },
                        base_width: if is_link { 7.0 } else { 4.5 },
                        force_kn: if is_link { 2800.0 } else { 1500.0 },
                        capacity_kn: if is_link { 5500.0 } else { 4200.0 },
                        utilization: if is_link { 0.51 } else { 0.36 },
                        ari: if is_link { 0.98 } else { 0.94 },
                        is_critical: false,
                    });
                }
                let cx = 13.5;
                let cy = 9.0;
                let cz = (16.0 * 3.75) * 0.5;
                ((cx, cy, cz), 28.0, list)
            }
            ModelArchetype::FemaP58HospitalArchetype => {
                let (nodes, elements) = crate::core::FemaP58Engine::generate_fema_resilience_archetype();
                let mut list = Vec::with_capacity(elements.len());
                for &(el_id, n1_id, n2_id, is_structural) in &elements {
                    let (_, x1, y1, z1) = nodes[n1_id];
                    let (_, x2, y2, z2) = nodes[n2_id];
                    let p1 = (x1 as f32, z1 as f32, y1 as f32);
                    let p2 = (x2 as f32, z2 as f32, y2 as f32);
                    list.push(Member3D {
                        id: el_id + 1,
                        p1,
                        p2,
                        name: if is_structural { format!("FrameBeamCol-{}", el_id + 1) } else { format!("FacadeTie-{}", el_id + 1) },
                        section: if is_structural { "RC Column 650x650 / Beam 400x600".to_string() } else { "Silicone Glazed Curtain Wall Facade".to_string() },
                        base_width: if is_structural { 6.5 } else { 3.5 },
                        force_kn: if is_structural { 1650.0 } else { 120.0 },
                        capacity_kn: if is_structural { 4200.0 } else { 350.0 },
                        utilization: if is_structural { 0.39 } else { 0.34 },
                        ari: if is_structural { 0.96 } else { 0.99 },
                        is_critical: false,
                    });
                }
                let cx = 12.0;
                let cy = 8.0;
                let cz = (10.0 * 3.8) * 0.5;
                ((cx, cy, cz), 22.0, list)
            }
            ModelArchetype::BlastResistantBunker3D => {
                let (nodes, elements) = crate::core::BlastEngine::generate_blast_bunker_archetype();
                let mut list = Vec::with_capacity(elements.len());
                for el in &elements {
                    let n1 = nodes.iter().find(|n| n.id == el.start_node).unwrap();
                    let n2 = nodes.iter().find(|n| n.id == el.end_node).unwrap();
                    let p1 = (n1.x as f32, n1.z as f32, n1.y as f32);
                    let p2 = (n2.x as f32, n2.z as f32, n2.y as f32);
                    let is_barrier = n1.y < -5.0 || n2.y < -5.0;
                    list.push(Member3D {
                        id: el.id,
                        p1,
                        p2,
                        name: if is_barrier { format!("BlastBarrier-{}", el.id) } else { format!("BunkerWallGirder-{}", el.id) },
                        section: el.section.name.clone(),
                        base_width: if is_barrier { 4.5 } else { 7.0 },
                        force_kn: if is_barrier { 480.0 } else { 2100.0 },
                        capacity_kn: if is_barrier { 1200.0 } else { 5800.0 },
                        utilization: if is_barrier { 0.40 } else { 0.36 },
                        ari: 0.98,
                        is_critical: false,
                    });
                }
                let cx = 6.0;
                let cy = -3.5;
                let cz = 2.1;
                ((cx, cy, cz), 16.0, list)
            }
            ModelArchetype::FireCompartmentFrame3D => {
                let (nodes, elements) = crate::core::FireEngine::generate_fire_compartment_archetype();
                let mut list = Vec::with_capacity(elements.len());
                for el in &elements {
                    let n1 = nodes.iter().find(|n| n.id == el.start_node).unwrap();
                    let n2 = nodes.iter().find(|n| n.id == el.end_node).unwrap();
                    let p1 = (n1.x as f32, n1.z as f32, n1.y as f32);
                    let p2 = (n2.x as f32, n2.z as f32, n2.y as f32);
                    let is_fire_zone = n1.x <= 7.5 && n1.y <= 7.5 && n2.x <= 7.5 && n2.y <= 7.5;
                    list.push(Member3D {
                        id: el.id,
                        p1,
                        p2,
                        name: if is_fire_zone { format!("FireZone-{}", el.id) } else { format!("ColdFrame-{}", el.id) },
                        section: el.section.name.clone(),
                        base_width: if is_fire_zone { 6.0 } else { 4.5 },
                        force_kn: if is_fire_zone { 350.0 } else { 180.0 },
                        capacity_kn: if is_fire_zone { 480.0 } else { 850.0 },
                        utilization: if is_fire_zone { 0.73 } else { 0.21 },
                        ari: 0.97,
                        is_critical: is_fire_zone,
                    });
                }
                let cx = 7.5;
                let cy = 7.5;
                let cz = 1.9;
                ((cx, cy, cz), 20.0, list)
            }
            ModelArchetype::OpenBimIfcModel3D => {
                let ifc_text = crate::core::IfcModel::sample_villa_ifc4();
                let ifc_model = crate::core::IfcModel::parse(&ifc_text).unwrap();
                let mut list = Vec::new();
                let mut id_counter = 0;
                for el in &ifc_model.elements {
                    for &(p1, p2) in &el.wireframe_lines {
                        id_counter += 1;
                        let is_col = el.class_name == "IFCCOLUMN";
                        list.push(Member3D {
                            id: id_counter,
                            p1: (p1.0 as f32, p1.2 as f32, p1.1 as f32),
                            p2: (p2.0 as f32, p2.2 as f32, p2.1 as f32),
                            name: format!("{}: {}", el.class_name, el.name),
                            section: if el.object_type.is_empty() { "OpenBIM Standard".to_string() } else { el.object_type.clone() },
                            base_width: if is_col { 5.5 } else { 4.0 },
                            force_kn: if is_col { 220.0 } else { 95.0 },
                            capacity_kn: if is_col { 650.0 } else { 380.0 },
                            utilization: if is_col { 0.34 } else { 0.25 },
                            ari: 0.98,
                            is_critical: false,
                        });
                    }
                }
                let cx = 3.0;
                let cy = 3.0;
                let cz = 1.75;
                ((cx, cy, cz), 12.0, list)
            }
        }
    }

}

/// Helper function to calculate distance from a point to a line segment
fn distance_to_segment(p: Pos2, a: Pos2, b: Pos2) -> f32 {
    let ab = b - a;
    let ap = p - a;
    let ab_len_sq = ab.length_sq();
    if ab_len_sq < 1e-4 {
        return ap.length();
    }
    let t = (ap.dot(ab) / ab_len_sq).clamp(0.0, 1.0);
    let projection = a + ab * t;
    (p - projection).length()
}
