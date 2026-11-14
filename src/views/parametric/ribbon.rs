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
        _active_pillar: &mut NavigationPillar,
        universal_canvas: &mut crate::views::universal_canvas::UniversalCanvasState,
        members: &[crate::models::StructuralMember],
        workspace_settings: crate::views::universal_canvas::WorkspaceCanvasSettings,
        sidebar_toggle_inset_px: f32,
    ) {
        Frame::new()
            .fill(theme.bg_sidebar)
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .inner_margin(Margin::symmetric(14, 6))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    // Row 1: CAD Tools + Snaps + Freeze + Sync + 1-File Save + DB
                    ui.horizontal_wrapped(|ui| {
                        if sidebar_toggle_inset_px > 0.0 {
                            ui.add_space(sidebar_toggle_inset_px);
                        }
                        icons::icon_label(
                            ui,
                            AppIcon::RulerSquare,
                            14.0,
                            theme.accent_blue,
                            "2D CAD & BIM",
                            12.0,
                            theme.accent_blue,
                            true,
                        );
                        ui.separator();

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
                            let fg = if is_selected {
                                Color32::WHITE
                            } else {
                                theme.text_secondary
                            };
                            let bg = if is_selected {
                                theme.accent_blue
                            } else {
                                theme.bg_card
                            };

                            if icons::icon_button(
                                ui,
                                cad_tool_icon(tool),
                                text,
                                10.5,
                                fg,
                                bg,
                                Stroke::NONE,
                                CornerRadius::same(6),
                                Vec2::new(0.0, 24.0),
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

                        if icons::icon_button(
                            ui,
                            AppIcon::RulerSquare,
                            "ORTHO (F8)",
                            10.5,
                            if self.ortho_mode { Color32::WHITE } else { theme.text_secondary },
                            if self.ortho_mode { theme.accent_blue } else { theme.bg_card },
                            Stroke::NONE,
                            CornerRadius::same(6),
                            Vec2::new(0.0, 24.0),
                        )
                        .clicked()
                        {
                            self.ortho_mode = !self.ortho_mode;
                        }

                        if icons::icon_button(
                            ui,
                            AppIcon::MagnetSnap,
                            "OSNAP (F9)",
                            10.5,
                            if self.snap_to_grid { Color32::WHITE } else { theme.text_secondary },
                            if self.snap_to_grid { theme.accent_blue } else { theme.bg_card },
                            Stroke::NONE,
                            CornerRadius::same(6),
                            Vec2::new(0.0, 24.0),
                        )
                        .clicked()
                        {
                            self.snap_to_grid = !self.snap_to_grid;
                        }

                        // Freeze Canvas (Unified scroll guard across all views)
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
                            CornerRadius::same(6),
                            Vec2::new(0.0, 24.0),
                        )
                        .on_hover_text("Freeze canvas zoom/pan so vertical scrolling moves the page smoothly")
                        .clicked()
                        {
                            universal_canvas.is_frozen = !universal_canvas.is_frozen;
                            ui_kit::set_canvas_frozen(ui.ctx(), universal_canvas.is_frozen);
                        }

                        // Toggle Universal Canvas markup overlay inside this single Parametric canvas
                        if icons::icon_button(
                            ui,
                            AppIcon::Layers,
                            "Sketch Sync",
                            10.5,
                            if self.sketch_overlay_synced {
                                Color32::WHITE
                            } else {
                                theme.text_secondary
                            },
                            if self.sketch_overlay_synced {
                                theme.accent_blue.gamma_multiply(0.85)
                            } else {
                                theme.bg_card
                            },
                            Stroke::NONE,
                            CornerRadius::same(6),
                            Vec2::new(0.0, 24.0),
                        )
                        .on_hover_text("Show or hide synchronized 15-module Universal Canvas sketches on Floor Plan")
                        .clicked()
                        {
                            self.sketch_overlay_synced = !self.sketch_overlay_synced;
                        }

                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if icons::icon_button(
                                ui,
                                AppIcon::SaveDisk,
                                "Save 1-File",
                                10.5,
                                Color32::WHITE,
                                theme.accent_green,
                                Stroke::NONE,
                                CornerRadius::same(6),
                                Vec2::new(0.0, 24.0),
                            )
                            .on_hover_text("Save unified workspace (.eigen.json)")
                            .clicked()
                            {
                                universal_canvas.export_unified_file(
                                    &self.bim_bus.project,
                                    members,
                                    &self.variables,
                                    workspace_settings,
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
                                Vec2::new(0.0, 24.0),
                            )
                            .on_hover_text("Import unified .eigen.json workspace archive")
                            .clicked()
                            {
                                universal_canvas.import_json_open = true;
                            }

                            if icons::icon_button(
                                ui,
                                AppIcon::BoxPackage,
                                "Save Comp",
                                10.5,
                                Color32::WHITE,
                                theme.accent_blue,
                                Stroke::NONE,
                                CornerRadius::same(6),
                                Vec2::new(0.0, 24.0),
                            )
                            .clicked()
                            {
                                self.save_current_to_db(db_components);
                            }

                            let db_text = format!("DB ({})", db_components.len());
                            if icons::icon_button(
                                ui,
                                AppIcon::Database,
                                db_text,
                                10.5,
                                if self.show_db_drawer {
                                    Color32::WHITE
                                } else {
                                    theme.text_secondary
                                },
                                if self.show_db_drawer {
                                    theme.accent_blue
                                } else {
                                    theme.bg_card
                                },
                                Stroke::NONE,
                                CornerRadius::same(6),
                                Vec2::new(0.0, 24.0),
                            )
                            .clicked()
                            {
                                self.show_db_drawer = !self.show_db_drawer;
                            }
                        });
                    });

                    ui.add_space(4.0);

                    // Row 2: BIMCraft Authoring Ribbon (Wall, Beam, Column, Door, Window, Rebar, View Mode, Sheet Export)
                    ui.horizontal_wrapped(|ui| {
                        icons::icon_label(
                            ui,
                            AppIcon::Building,
                            13.0,
                            theme.accent_orange,
                            "BIMCRAFT:",
                            11.0,
                            theme.accent_orange,
                            true,
                        );

                        let bim_tools = [
                            (BimAuthoringTool::WallChain, AppIcon::Wall, "Wall [WA]"),
                            (BimAuthoringTool::StructuralBeam, AppIcon::Beam, "Beam [BM]"),
                            (BimAuthoringTool::StructuralColumn, AppIcon::Column, "Column [CL]"),
                            (BimAuthoringTool::PlaceDoor, AppIcon::Door, "Door [DR]"),
                            (BimAuthoringTool::PlaceWindow, AppIcon::Window, "Window [WN]"),
                            (BimAuthoringTool::ReinforceConcrete, AppIcon::Rebar, "Reinforce [RF]"),
                        ];
                        for (btool, icon, label) in bim_tools {
                            let active = self.bim_tool == btool;
                            if icons::icon_button(
                                ui,
                                icon,
                                label,
                                10.5,
                                if active {
                                    Color32::WHITE
                                } else {
                                    theme.text_secondary
                                },
                                if active {
                                    theme.accent_orange
                                } else {
                                    theme.bg_card
                                },
                                Stroke::NONE,
                                CornerRadius::same(5),
                                Vec2::new(0.0, 24.0),
                            )
                            .clicked()
                            {
                                self.bim_tool = btool;
                                self.pending_start_pt = None;
                            }
                        }

                        ui.separator();

                        // View Mode Switcher: Plan | Section Cut | Sheet Composer 2D | Dynamo | Rebar BBS | Elevations
                        let views = [
                            (DraftingViewMode::FloorPlanViewRange, AppIcon::MapPin, "Floor Plan (+1200mm)"),
                            (DraftingViewMode::LiveSectionCutPoche, AppIcon::SectionCut, "Section Cut"),
                            (DraftingViewMode::SheetLayoutA101, AppIcon::Document, "Sheet 2D"),
                            (DraftingViewMode::VisualNodeGraphDynamo, AppIcon::NodesGraph, "Dynamo Graph"),
                            (DraftingViewMode::RebarBbsDetailing, AppIcon::Rebar, "Rebar & BBS"),
                            (DraftingViewMode::ElevationsAndMepRiser, AppIcon::Building, "Elevations & MEP"),
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
                                CornerRadius::same(5),
                                Vec2::new(0.0, 24.0),
                            )
                            .clicked()
                            {
                                self.view_mode = vm;
                            }
                        }

                        ui.separator();

                        // Undo / Redo Buttons
                        ui.add_enabled_ui(!self.bim_bus.undo_stack.is_empty(), |ui| {
                            if icons::icon_button(
                                ui,
                                AppIcon::Undo,
                                "Undo",
                                10.5,
                                theme.text_secondary,
                                theme.bg_card,
                                Stroke::NONE,
                                CornerRadius::same(5),
                                Vec2::new(0.0, 24.0),
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
                                CornerRadius::same(5),
                                Vec2::new(0.0, 24.0),
                            )
                            .clicked()
                            {
                                if let Some(msg) = self.bim_bus.redo() {
                                    self.command_history.push(msg);
                                    self.refresh_live_ari();
                                }
                            }
                        });

                        // Export PDF / SVG / IFC4
                        if icons::icon_button(
                            ui,
                            AppIcon::Printer,
                            "Export PDF/IFC4",
                            10.5,
                            theme.accent_blue,
                            theme.bg_card,
                            Stroke::NONE,
                            CornerRadius::same(5),
                            Vec2::new(0.0, 24.0),
                        )
                        .clicked()
                        {
                            self.export_bim_deliverables();
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

                            // Paper size selector
                            for p in [SheetPaperSize::A4Landscape, SheetPaperSize::A3Landscape, SheetPaperSize::A2Landscape, SheetPaperSize::A1Landscape] {
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

                            // Scale selector
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
