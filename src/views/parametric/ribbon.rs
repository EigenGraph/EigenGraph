//! views/parametric/ribbon.rs — Authoring Ribbon & Vector Delivery Exports.

use super::*;

fn cad_tool_icon(tool: CadTool) -> AppIcon {
    match tool {
        CadTool::Select => AppIcon::SelectArrow,
        CadTool::Line => AppIcon::LineSeg,
        CadTool::Polyline => AppIcon::NodesGraph,
        CadTool::Arc => AppIcon::ArcCurve,
        CadTool::Circle => AppIcon::CircleShape,
        CadTool::Rectangle => AppIcon::RectShape,
        CadTool::Dimension => AppIcon::DimensionRuler,
        CadTool::Trim => AppIcon::Scissors,
        CadTool::Offset => AppIcon::Layers,
    }
}

impl ParametricView {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn render_drafting_ribbon(
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
        let _avail_w = ui.available_width();
        Frame::new()
            .fill(theme.bg_sidebar)
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .inner_margin(Margin::symmetric(10, 8))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.vertical(|ui| {
                    // Row 1: Universal Canvas HUD Toolbar (Module Pill, Views, Snaps, Freeze, Floating Panel Toggles, 1-File Sync)
                    ui.horizontal_wrapped(|ui| {
                        if sidebar_toggle_inset_px > 0.0 {
                            ui.add_space(sidebar_toggle_inset_px);
                        }
                        icons::pill_badge(
                            ui,
                            AppIcon::RulerSquare,
                            "Parametric • 2D",
                            theme.accent_blue,
                        );
                        ui.separator();

                        // View Mode Switcher
                        let views = [
                            (DraftingViewMode::FloorPlanViewRange, AppIcon::MapPin, "Plan"),
                            (DraftingViewMode::LiveSectionCutPoche, AppIcon::SectionCut, "Section"),
                            (DraftingViewMode::SheetLayoutA101, AppIcon::Document, "Sheet 2D"),
                            (DraftingViewMode::VisualNodeGraphDynamo, AppIcon::NodesGraph, "Dynamo"),
                            (DraftingViewMode::RebarBbsDetailing, AppIcon::Rebar, "Rebar"),
                            (DraftingViewMode::ElevationsAndMepRiser, AppIcon::Building, "Elev/MEP"),
                        ];
                        for (vm, icon, vlabel) in views {
                            let selected = self.view_mode == vm;
                            if icons::icon_button(
                                ui,
                                icon,
                                vlabel,
                                10.5,
                                if selected { Color32::WHITE } else { theme.text_secondary },
                                if selected { theme.accent_blue } else { theme.bg_card },
                                Stroke::NONE,
                                CornerRadius::same(6),
                                Vec2::new(0.0, 26.0),
                            )
                            .clicked()
                            {
                                self.view_mode = vm;
                            }
                        }

                        ui.separator();

                        // Undo / Redo / Grid / Ortho / OSNAP / Fit
                        ui.add_enabled_ui(!self.bim_bus.undo_stack.is_empty(), |ui| {
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
                            .clicked()
                            {
                                if let Some(msg) = self.bim_bus.undo() {
                                    self.command_history.push(msg);
                                    self.refresh_live_ari();
                                }
                            }
                        });
                        ui.add_enabled_ui(!self.bim_bus.redo_stack.is_empty(), |ui| {
                            if icons::icon_button(
                                ui,
                                AppIcon::Redo,
                                "Redo",
                                10.5,
                                theme.text_secondary,
                                theme.bg_card,
                                Stroke::NONE,
                                CornerRadius::same(6),
                                Vec2::new(0.0, 26.0),
                            )
                            .clicked()
                            {
                                if let Some(msg) = self.bim_bus.redo() {
                                    self.command_history.push(msg);
                                    self.refresh_live_ari();
                                }
                            }
                        });

                        if icons::icon_button(
                            ui,
                            AppIcon::Grid,
                            "Grid",
                            10.5,
                            if universal_canvas.show_grid { Color32::WHITE } else { theme.text_secondary },
                            if universal_canvas.show_grid { theme.accent_blue.gamma_multiply(0.75) } else { theme.bg_card },
                            Stroke::NONE,
                            CornerRadius::same(6),
                            Vec2::new(0.0, 26.0),
                        )
                        .clicked()
                        {
                            universal_canvas.show_grid = !universal_canvas.show_grid;
                        }

                        if icons::icon_button(
                            ui,
                            AppIcon::RulerSquare,
                            "ORTHO",
                            10.5,
                            if self.ortho_mode { Color32::WHITE } else { theme.text_secondary },
                            if self.ortho_mode { theme.accent_blue } else { theme.bg_card },
                            Stroke::NONE,
                            CornerRadius::same(6),
                            Vec2::new(0.0, 26.0),
                        )
                        .on_hover_text("Toggle Ortho F8 constraint")
                        .clicked()
                        {
                            self.ortho_mode = !self.ortho_mode;
                        }

                        if icons::icon_button(
                            ui,
                            AppIcon::MagnetSnap,
                            "OSNAP",
                            10.5,
                            if self.snap_to_grid { Color32::WHITE } else { theme.text_secondary },
                            if self.snap_to_grid { theme.accent_blue } else { theme.bg_card },
                            Stroke::NONE,
                            CornerRadius::same(6),
                            Vec2::new(0.0, 26.0),
                        )
                        .on_hover_text("Toggle Revit Object Snaps (SE, SM, SI, SC, SP, SG)")
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
                            universal_canvas.fit_world_or_site(!self.bim_bus.project.is_empty());
                            self.canvas_pan = Vec2::ZERO;
                            self.canvas_zoom = 1.0;
                        }

                        ui.separator();

                        // Shared World Map Basemap, Globe Reset, Ground 1:200 Zoom, Set Origin
                        universal_canvas.render_world_map_toolbar_controls(ui, theme, 26.0);

                        ui.separator();

                        // Shared Freeze Canvas button
                        UniversalCanvasState::render_freeze_button(
                            ui,
                            theme,
                            &mut universal_canvas.is_frozen,
                            26.0,
                        );

                        // Floating HUD Overlays inside Canvas
                        // Phase 36: Top Toolbar Dock Panel Toggles (no canvas-blocking floating HUD)
                        if icons::icon_button(
                            ui,
                            AppIcon::Folder,
                            "Project & Levels",
                            10.5,
                            if self.floating_project_open { Color32::WHITE } else { theme.text_secondary },
                            if self.floating_project_open { theme.accent_green } else { theme.bg_card },
                            Stroke::NONE,
                            CornerRadius::same(6),
                            Vec2::new(0.0, 26.0),
                        )
                        .on_hover_text("Toggle docked Project Browser & Storey Levels strip inside top toolbar")
                        .clicked()
                        {
                            self.floating_project_open = !self.floating_project_open;
                        }

                        if icons::icon_button(
                            ui,
                            AppIcon::SlidersPanel,
                            "Properties",
                            10.5,
                            if self.floating_properties_open { Color32::WHITE } else { theme.text_secondary },
                            if self.floating_properties_open { theme.accent_green } else { theme.bg_card },
                            Stroke::NONE,
                            CornerRadius::same(6),
                            Vec2::new(0.0, 26.0),
                        )
                        .on_hover_text("Toggle docked Properties, Driving Parameters & Canvas Sync inside top toolbar")
                        .clicked()
                        {
                            self.floating_properties_open = !self.floating_properties_open;
                        }

                        if icons::icon_button(
                            ui,
                            AppIcon::Cpu,
                            "CLI Log",
                            10.5,
                            if self.show_cli_float { Color32::WHITE } else { theme.text_secondary },
                            if self.show_cli_float { theme.accent_blue.gamma_multiply(0.85) } else { theme.bg_card },
                            Stroke::NONE,
                            CornerRadius::same(6),
                            Vec2::new(0.0, 26.0),
                        )
                        .on_hover_text("Expand or collapse CAD Command Line history inside top toolbar")
                        .clicked()
                        {
                            self.show_cli_float = !self.show_cli_float;
                        }

                        if icons::icon_button(
                            ui,
                            AppIcon::Pencil,
                            "Ink Sketch",
                            10.5,
                            if self.sketch_draw_mode { Color32::WHITE } else { theme.text_secondary },
                            if self.sketch_draw_mode { theme.accent_orange } else { theme.bg_card },
                            Stroke::NONE,
                            CornerRadius::same(6),
                            Vec2::new(0.0, 26.0),
                        )
                        .on_hover_text("Toggle Universal Canvas ink sketch & markup drawing mode")
                        .clicked()
                        {
                            self.sketch_draw_mode = !self.sketch_draw_mode;
                            if self.sketch_draw_mode {
                                self.sketch_overlay_synced = true;
                                if universal_canvas.active_tool == crate::views::universal_canvas::UniversalCanvasTool::SelectPan {
                                    universal_canvas.active_tool = crate::views::universal_canvas::UniversalCanvasTool::Freehand;
                                }
                            }
                        }

                        // Sync & Single-File Actions
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
                        .on_hover_text("Save unified workspace (.eigen.json)")
                        .clicked()
                        {
                            universal_canvas.export_unified_file(
                                &self.bim_bus.project,
                                members,
                                &self.variables,
                                workspace_settings.clone(),
                            );
                            self.save_notification = universal_canvas.status_message.clone();
                        }

                        if icons::icon_button(
                            ui,
                            AppIcon::UploadImport,
                            "Import",
                            10.5,
                            theme.text_secondary,
                            theme.bg_card,
                            Stroke::NONE,
                            CornerRadius::same(6),
                            Vec2::new(0.0, 26.0),
                        )
                        .on_hover_text("Import unified .eigen.json workspace archive")
                        .clicked()
                        {
                            universal_canvas.import_json_open = true;
                        }

                        let db_text = format!("DB ({})", db_components.len());
                        if icons::icon_button(
                            ui,
                            AppIcon::Database,
                            db_text,
                            10.5,
                            if self.show_db_drawer { Color32::WHITE } else { theme.text_secondary },
                            if self.show_db_drawer { theme.accent_blue } else { theme.bg_card },
                            Stroke::NONE,
                            CornerRadius::same(6),
                            Vec2::new(0.0, 26.0),
                        )
                        .on_hover_text("Open Parametric Component Catalog")
                        .clicked()
                        {
                            self.show_db_drawer = !self.show_db_drawer;
                        }

                        if icons::icon_button(
                            ui,
                            AppIcon::Printer,
                            "PDF/IFC4",
                            10.5,
                            theme.accent_blue,
                            theme.bg_card,
                            Stroke::NONE,
                            CornerRadius::same(6),
                            Vec2::new(0.0, 26.0),
                        )
                        .on_hover_text("Export Vector PDF, SVG & IFC4 Deliverables")
                        .clicked()
                        {
                            self.export_bim_deliverables();
                        }
                    });

                    ui.add_space(4.0);

                    // Row 2: Active Drafting Tools (Universal Ink Sketch Mode OR CAD + BIMCraft Authoring)
                    ui.horizontal_wrapped(|ui| {
                        if self.sketch_draw_mode {
                            icons::icon_label(
                                ui,
                                AppIcon::Pencil,
                                13.0,
                                theme.accent_orange,
                                "UNIVERSAL SKETCH:",
                                10.5,
                                theme.accent_orange,
                                true,
                            );
                            universal_canvas.render_sketch_tools_bar(ui, theme, 24.0, true);
                            ui.separator();
                            universal_canvas.render_color_swatches(ui, theme);
                            ui.separator();
                            if icons::icon_button(
                                ui,
                                AppIcon::Undo,
                                "Undo Ink",
                                10.0,
                                theme.text_secondary,
                                theme.bg_card,
                                Stroke::NONE,
                                CornerRadius::same(5),
                                Vec2::new(0.0, 24.0),
                            )
                            .clicked()
                            {
                                universal_canvas.undo();
                            }
                            if icons::icon_button(
                                ui,
                                AppIcon::Layers,
                                if self.sketch_overlay_synced { "Overlay ON" } else { "Overlay OFF" },
                                10.0,
                                if self.sketch_overlay_synced { Color32::WHITE } else { theme.text_secondary },
                                if self.sketch_overlay_synced { theme.accent_blue } else { theme.bg_card },
                                Stroke::NONE,
                                CornerRadius::same(5),
                                Vec2::new(0.0, 24.0),
                            )
                            .clicked()
                            {
                                self.sketch_overlay_synced = !self.sketch_overlay_synced;
                            }
                        } else {
                            let tools = [
                                CadTool::Select,
                                CadTool::Line,
                                CadTool::Polyline,
                                CadTool::Arc,
                                CadTool::Circle,
                                CadTool::Rectangle,
                                CadTool::Dimension,
                                CadTool::Trim,
                                CadTool::Offset,
                            ];

                            for tool in tools {
                                let (name, shortcut) = tool.label_and_shortcut();
                                let is_selected = self.bim_tool == BimAuthoringTool::CadStandard
                                    && self.active_tool == tool;
                                let text = format!("{} [{}]", name, shortcut);
                                if icons::icon_button(
                                    ui,
                                    cad_tool_icon(tool),
                                    text,
                                    10.0,
                                    if is_selected { Color32::WHITE } else { theme.text_secondary },
                                    if is_selected { theme.accent_blue } else { theme.bg_card },
                                    Stroke::NONE,
                                    CornerRadius::same(5),
                                    Vec2::new(0.0, 23.0),
                                )
                                .clicked()
                                {
                                    self.active_tool = tool;
                                    self.bim_tool = BimAuthoringTool::CadStandard;
                                    self.pending_start_pt = None;
                                    self.command_history
                                        .push(format!("CAD Tool changed to: {:?}", tool));
                                }
                            }

                            ui.separator();

                            let bim_tools = [
                                (BimAuthoringTool::WallChain, AppIcon::Wall, "Wall [WA]"),
                                (BimAuthoringTool::StructuralBeam, AppIcon::Beam, "Beam [BM]"),
                                (BimAuthoringTool::StructuralColumn, AppIcon::Column, "Col [CL]"),
                                (BimAuthoringTool::PlaceDoor, AppIcon::Door, "Door [DR]"),
                                (BimAuthoringTool::PlaceWindow, AppIcon::Window, "Win [WN]"),
                                (BimAuthoringTool::ReinforceConcrete, AppIcon::Rebar, "Rebar [RF]"),
                            ];
                            for (btool, icon, label) in bim_tools {
                                let active = self.bim_tool == btool;
                                if icons::icon_button(
                                    ui,
                                    icon,
                                    label,
                                    10.0,
                                    if active { Color32::WHITE } else { theme.text_secondary },
                                    if active { theme.accent_orange } else { theme.bg_card },
                                    Stroke::NONE,
                                    CornerRadius::same(5),
                                    Vec2::new(0.0, 23.0),
                                )
                                .clicked()
                                {
                                    self.bim_tool = btool;
                                    self.pending_start_pt = None;
                                }
                            }
                        }
                    });

                    // Sheet Composer 2D Control Bar
                    if self.view_mode == DraftingViewMode::SheetLayoutA101 {
                        ui.add_space(4.0);
                        ui.horizontal_wrapped(|ui| {
                            icons::icon_label(
                                ui,
                                AppIcon::Document,
                                13.0,
                                theme.accent_blue,
                                "Sheet Config:",
                                11.0,
                                theme.accent_blue,
                                true,
                            );

                            for p in [
                                SheetPaperSize::A4Landscape,
                                SheetPaperSize::A3Landscape,
                                SheetPaperSize::A2Landscape,
                                SheetPaperSize::A1Landscape,
                            ] {
                                let label = match p {
                                    SheetPaperSize::A4Landscape => "A4",
                                    SheetPaperSize::A3Landscape => "A3",
                                    SheetPaperSize::A2Landscape => "A2",
                                    SheetPaperSize::A1Landscape => "A1",
                                };
                                if ui.selectable_label(self.sheet_paper_size == p, label).clicked() {
                                    self.sheet_paper_size = p;
                                }
                            }

                            ui.separator();

                            for s in [SheetScale::Scale1_50, SheetScale::Scale1_100, SheetScale::Scale1_200] {
                                if ui.selectable_label(self.sheet_scale == s, s.label()).clicked() {
                                    self.sheet_scale = s;
                                }
                            }

                            ui.separator();

                            ui.label(RichText::new("No:").size(10.5).color(theme.text_secondary));
                            ui.add(egui::TextEdit::singleline(&mut self.sheet_number).desired_width(45.0));

                            ui.label(RichText::new("Title:").size(10.5).color(theme.text_secondary));
                            ui.add(egui::TextEdit::singleline(&mut self.sheet_title).desired_width(170.0));

                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                if icons::icon_button(
                                    ui,
                                    AppIcon::Printer,
                                    "Print Vector PDF",
                                    11.0,
                                    Color32::WHITE,
                                    theme.accent_blue,
                                    Stroke::NONE,
                                    CornerRadius::same(5),
                                    Vec2::new(0.0, 24.0),
                                )
                                .on_hover_text("Export official scaled drawing sheet to vector PDF")
                                .clicked()
                                {
                                    self.export_bim_deliverables();
                                }
                            });
                        });
                    }

                    // Row 2B: Organized Levels Quick Bar + Inline CAD Command Line inside Top Toolbar
                    ui.add_space(4.0);
                    ui.horizontal_wrapped(|ui| {
                        icons::icon_label(
                            ui,
                            AppIcon::Layers,
                            12.0,
                            theme.accent_blue,
                            "Level:",
                            10.5,
                            theme.text_secondary,
                            true,
                        );
                        let active_lvl_text = self
                            .bim_bus
                            .project
                            .levels
                            .get(self.active_level_idx)
                            .map(|l| format!("{} ({:+.1} m)", l.name, l.elevation_mm * 1e-3))
                            .unwrap_or_else(|| "Level 1 (+0.0 m)".to_string());
                        egui::ComboBox::from_id_salt("param_top_level_combo")
                            .selected_text(RichText::new(active_lvl_text).size(10.5).color(theme.text_primary))
                            .show_ui(ui, |ui| {
                                for (idx, lvl) in self.bim_bus.project.levels.iter().enumerate() {
                                    let label = format!("{} ({:+.1} m)", lvl.name, lvl.elevation_mm * 1e-3);
                                    if ui.selectable_label(self.active_level_idx == idx, label).clicked() {
                                        self.active_level_idx = idx;
                                    }
                                }
                            });
                        if ui
                            .add(
                                egui::Button::new(RichText::new("+ Lvl").size(10.0).color(theme.accent_green))
                                    .fill(theme.bg_card)
                                    .corner_radius(CornerRadius::same(5))
                                    .min_size(Vec2::new(0.0, 22.0)),
                            )
                            .on_hover_text("Add a new storey level (+3.5 m)")
                            .clicked()
                        {
                            let next_num = self.bim_bus.project.levels.len() + 1;
                            let next_elev = self
                                .bim_bus
                                .project
                                .levels
                                .last()
                                .map(|l| l.elevation_mm + 3500.0)
                                .unwrap_or(0.0);
                            let _ = self.bim_bus.execute(BimCommand::AddLevel {
                                name: format!("Level {}", next_num),
                                elevation_mm: next_elev,
                            });
                            self.active_level_idx = self.bim_bus.project.levels.len().saturating_sub(1);
                            self.refresh_live_ari();
                        }

                        ui.separator();

                        icons::icon_label(
                            ui,
                            AppIcon::Cpu,
                            12.0,
                            theme.accent_blue,
                            "CAD Cmd:",
                            10.5,
                            theme.accent_blue,
                            true,
                        );
                        let cli_resp = ui.add(
                            egui::TextEdit::singleline(&mut self.command_line_input)
                                .desired_width(210.0)
                                .hint_text("WA, BM, CL, DR, WN, RM, RF, UNDO..."),
                        );
                        if ((cli_resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                            || ui
                                .add(
                                    egui::Button::new(RichText::new("Run").size(10.5).color(Color32::WHITE))
                                        .fill(theme.accent_blue)
                                        .corner_radius(CornerRadius::same(5))
                                        .min_size(Vec2::new(0.0, 22.0)),
                                )
                                .clicked())
                            && !self.command_line_input.trim().is_empty()
                        {
                            let raw = self.command_line_input.trim().to_string();
                            self.command_history.push(format!("Command: {raw}"));
                            self.execute_cad_command(&raw);
                            self.command_line_input.clear();
                            cli_resp.request_focus();
                        }
                        if let Some(last_log) = self.command_history.last() {
                            ui.label(
                                RichText::new(last_log)
                                    .size(10.0)
                                    .color(theme.text_muted),
                            );
                        }
                    });

                    // Row 3: Expandable Top-Toolbar Docked Inspectors (Project/Levels, Properties/Variables/Canvas, CLI Log)
                    self.render_top_toolbar_inspector_dock(
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
            });
    }

    pub(super) fn export_bim_deliverables(&mut self) {
        let safe_title = self.sheet_title.replace([' ', '/', '\\'], "_");
        let pdf_filename = format!("Sheet_{}_{}.pdf", self.sheet_number, safe_title);
        let svg_filename = format!("Sheet_{}_{}.svg", self.sheet_number, safe_title);
        let ifc_filename = format!(
            "{}.ifc",
            self.bim_bus.project.project_name.replace([' ', '/', '\\'], "_")
        );

        let pdf_bytes = self
            .bim_bus
            .project
            .export_sheet_pdf(&self.sheet_number, &self.sheet_title);
        let svg_str = self
            .bim_bus
            .project
            .export_sheet_svg(&self.sheet_number, &self.sheet_title);
        let ifc_str = self.bim_bus.project.export_ifc4_spatial();

        #[cfg(target_arch = "wasm32")]
        {
            let _ = crate::core::web_bridge::trigger_browser_download(
                &pdf_filename,
                &pdf_bytes,
                "application/pdf",
            );
            let _ = crate::core::web_bridge::trigger_browser_download(
                &svg_filename,
                svg_str.as_bytes(),
                "image/svg+xml;charset=utf-8",
            );
            let _ = crate::core::web_bridge::trigger_browser_download(
                &ifc_filename,
                ifc_str.as_bytes(),
                "application/x-step;charset=utf-8",
            );

            self.save_notification = Some(format!(
                "Exported '{}', '{}', and '{}' via browser downloads",
                pdf_filename, svg_filename, ifc_filename
            ));
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let export_dir = crate::core::InternalVault::vault_root_dir().join("Exports");
            let _ = std::fs::create_dir_all(&export_dir);
            let pdf_path = export_dir.join(&pdf_filename);
            let svg_path = export_dir.join(&svg_filename);
            let ifc_path = export_dir.join(&ifc_filename);

            let _ = std::fs::write(&pdf_path, pdf_bytes);
            let _ = std::fs::write(&svg_path, svg_str);
            let _ = std::fs::write(&ifc_path, ifc_str);

            self.save_notification = Some(format!(
                "Successfully exported Drawing Sheet '{}' to {}",
                pdf_filename,
                export_dir.display()
            ));
        }
    }


}
