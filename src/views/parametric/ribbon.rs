//! views/parametric/ribbon.rs — Authoring Ribbon & Vector Delivery Exports.

use super::*;

impl ParametricView {
    pub(super) fn render_drafting_ribbon(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        db_components: &mut Vec<DatabaseComponent>,
        _active_pillar: &mut NavigationPillar,
    ) {
        Frame::new()
            .fill(theme.bg_sidebar)
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .inner_margin(Margin::symmetric(14, 6))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    // Row 1: CAD Tools + Undo/Redo + Save/DB
                    ui.horizontal_wrapped(|ui| {
                        ui.label(
                            RichText::new("📐 2D CAD & BIM")
                                .size(12.5)
                                .strong()
                                .color(theme.accent_blue),
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

                            let btn = egui::Button::new(
                                RichText::new(text).size(11.0).color(if is_selected {
                                    Color32::WHITE
                                } else {
                                    theme.text_secondary
                                }),
                            )
                            .fill(if is_selected {
                                theme.accent_blue
                            } else {
                                theme.bg_card
                            })
                            .corner_radius(CornerRadius::same(6));

                            if ui.add(btn).clicked() {
                                self.active_tool = tool;
                                self.bim_tool = BimAuthoringTool::CadStandard;
                                self.pending_start_pt = None;
                                self.command_history
                                    .push(format!("CAD Tool changed to: {:?}", tool));
                            }
                        }

                        ui.separator();

                        if ui
                            .selectable_label(self.ortho_mode, "📐 ORTHO (F8)")
                            .clicked()
                        {
                            self.ortho_mode = !self.ortho_mode;
                        }
                        if ui
                            .selectable_label(self.snap_to_grid, "🧲 OSNAP (F9)")
                            .clicked()
                        {
                            self.snap_to_grid = !self.snap_to_grid;
                        }

                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let save_btn = egui::Button::new(
                                RichText::new("💾 Save Component")
                                    .size(11.0)
                                    .strong()
                                    .color(Color32::WHITE),
                            )
                            .fill(theme.accent_green)
                            .corner_radius(CornerRadius::same(6));

                            if ui.add(save_btn).clicked() {
                                self.save_current_to_db(db_components);
                            }

                            let db_text = format!("📦 Saved DB ({})", db_components.len());
                            if ui.selectable_label(self.show_db_drawer, db_text).clicked() {
                                self.show_db_drawer = !self.show_db_drawer;
                            }
                        });
                    });

                    ui.add_space(4.0);

                    // Row 2: BIMCraft Authoring Ribbon (Wall, Beam, Column, Door, Window, Rebar, View Mode, Sheet Export)
                    ui.horizontal_wrapped(|ui| {
                        ui.label(
                            RichText::new("🏛️ BIMCRAFT:")
                                .size(11.0)
                                .strong()
                                .color(theme.accent_orange),
                        );

                        let bim_tools = [
                            (BimAuthoringTool::WallChain, "🧱 Wall [WA]"),
                            (BimAuthoringTool::StructuralBeam, "📏 Beam/Run [BM]"),
                            (BimAuthoringTool::StructuralColumn, "🏛️ Column [CL]"),
                            (BimAuthoringTool::PlaceDoor, "🚪 Door [DR]"),
                            (BimAuthoringTool::PlaceWindow, "🪟 Window [WN]"),
                            (BimAuthoringTool::ReinforceConcrete, "🔩 Reinforce [RF]"),
                        ];
                        for (btool, label) in bim_tools {
                            let active = self.bim_tool == btool;
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new(label).size(11.0).color(if active {
                                            Color32::WHITE
                                        } else {
                                            theme.text_secondary
                                        }),
                                    )
                                    .fill(if active {
                                        theme.accent_orange
                                    } else {
                                        theme.bg_card
                                    })
                                    .corner_radius(CornerRadius::same(5)),
                                )
                                .clicked()
                            {
                                self.bim_tool = btool;
                                self.pending_start_pt = None;
                            }
                        }

                        ui.separator();

                        // View Mode Switcher: Plan | Section Cut | Sheet Composer 2D
                        let views = [
                            (DraftingViewMode::FloorPlanViewRange, "🗺️ Floor Plan (+1200mm)"),
                            (DraftingViewMode::LiveSectionCutPoche, "🔪 Section Cut (Poché)"),
                            (DraftingViewMode::SheetLayoutA101, "📑 Sheet Composer 2D"),
                            (DraftingViewMode::VisualNodeGraphDynamo, "⚡ Dynamo Node Graph"),
                        ];
                        for (vm, vlabel) in views {
                            if ui.selectable_label(self.view_mode == vm, vlabel).clicked() {
                                self.view_mode = vm;
                            }
                        }

                        ui.separator();

                        // Undo / Redo Buttons
                        if ui
                            .add_enabled(
                                !self.bim_bus.undo_stack.is_empty(),
                                egui::Button::new(RichText::new("⟲ Undo").size(10.5)),
                            )
                            .clicked()
                        {
                            if let Some(msg) = self.bim_bus.undo() {
                                self.command_history.push(msg);
                                self.refresh_live_ari();
                            }
                        }
                        if ui
                            .add_enabled(
                                !self.bim_bus.redo_stack.is_empty(),
                                egui::Button::new(RichText::new("⟳ Redo").size(10.5)),
                            )
                            .clicked()
                        {
                            if let Some(msg) = self.bim_bus.redo() {
                                self.command_history.push(msg);
                                self.refresh_live_ari();
                            }
                        }

                        // Export PDF / SVG / IFC4
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("🖨️ Export PDF/IFC4")
                                        .size(10.5)
                                        .color(theme.accent_blue),
                                )
                                .fill(theme.bg_card)
                                .corner_radius(CornerRadius::same(5)),
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
                            ui.label(RichText::new("📐 Sheet Config:").size(11.0).strong().color(theme.accent_blue));

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
                                if ui.add(
                                    egui::Button::new(RichText::new("📥 Print Vector PDF").size(11.0).strong().color(Color32::WHITE))
                                        .fill(theme.accent_blue)
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
                "✅ Exported '{}', '{}', and '{}' via browser downloads",
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
                "✅ Successfully exported Drawing Sheet '{}' to {}",
                pdf_filename,
                export_dir.display()
            ));
        }
    }


}
