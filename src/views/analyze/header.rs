//! views/analyze/header.rs — Grouped Analysis Discipline Header, Interop Bar & Locked BIM->FEM Sync Preview.

use super::*;

impl AnalyzeView {
    pub(super) fn render_subtabs_header(&mut self, ui: &mut Ui, theme: &MacTheme) {
        theme.card_frame().show(ui, |ui| {
            // Top Row: Suite Title & Interop / Dossier Export Actions
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    RichText::new("🛡️ 8D Structural Resilience & Multi-Code Verification Suite")
                        .size(12.5)
                        .strong()
                        .color(theme.accent_blue),
                );

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let publish_text = if self.published_to_dossier {
                        "✅ Synced to MY PROJECT"
                    } else {
                        "📤 Publish to MY PROJECT Dossier"
                    };
                    let publish_bg = if self.published_to_dossier {
                        theme.accent_green
                    } else {
                        theme.accent_blue.gamma_multiply(0.85)
                    };

                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new(publish_text)
                                    .size(11.5)
                                    .strong()
                                    .color(Color32::WHITE),
                            )
                            .fill(publish_bg)
                            .corner_radius(CornerRadius::same(7)),
                        )
                        .clicked()
                    {
                        self.published_to_dossier = true;
                        self.export_status =
                            Some("Model & ARI Analytics synced to MY PROJECT Dossier".to_string());
                    }

                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("📥 SAP2000 (.s2k)")
                                    .size(11.0)
                                    .color(Color32::WHITE),
                            )
                            .fill(theme.bg_canvas)
                            .stroke(Stroke::new(1.0, theme.accent_blue))
                            .corner_radius(CornerRadius::same(7)),
                        )
                        .clicked()
                    {
                        let mut engine = crate::core::ParametricEngine::new();
                        if let Ok(model) = engine.generate_warren_truss() {
                            let ari = crate::core::AriEngine::evaluate_redundancy(&model).ok();
                            let s2k =
                                crate::core::Sap2000Exporter::export_to_s2k(&model, ari.as_ref());
                            if let Ok(path) = crate::core::DiskExporter::write_export_file(
                                "EigenGraph_SAP2000.s2k",
                                &s2k,
                            ) {
                                self.export_status =
                                    Some(format!("CSI SAP2000 .s2k saved to: {}", path));
                            }
                        }
                    }

                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("📥 DXF 3D (.dxf)")
                                    .size(11.0)
                                    .color(Color32::WHITE),
                            )
                            .fill(theme.bg_canvas)
                            .stroke(Stroke::new(1.0, theme.accent_green))
                            .corner_radius(CornerRadius::same(7)),
                        )
                        .clicked()
                    {
                        let mut engine = crate::core::ParametricEngine::new();
                        if let Ok(model) = engine.generate_warren_truss() {
                            let ari = crate::core::AriEngine::evaluate_redundancy(&model).ok();
                            let dxf =
                                crate::core::DxfExporter::export_to_dxf(&model, ari.as_ref());
                            if let Ok(path) = crate::core::DiskExporter::write_export_file(
                                "EigenGraph_3D.dxf",
                                &dxf,
                            ) {
                                self.export_status =
                                    Some(format!("AutoCAD 3D DXF saved to: {}", path));
                            }
                        }
                    }

                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("🌐 OpenBIM (.ifc)")
                                    .size(11.0)
                                    .color(Color32::WHITE),
                            )
                            .fill(theme.bg_canvas)
                            .stroke(Stroke::new(1.0, Color32::from_rgb(180, 100, 255)))
                            .corner_radius(CornerRadius::same(7)),
                        )
                        .clicked()
                    {
                        let mut engine = crate::core::ParametricEngine::new();
                        if let Ok(model) = engine.generate_warren_truss() {
                            let ari = crate::core::AriEngine::evaluate_redundancy(&model).ok();
                            let ifc =
                                crate::core::IfcExporter::export_to_ifc(&model, ari.as_ref());
                            if let Ok(path) = crate::core::DiskExporter::write_export_file(
                                "EigenGraph_OpenBIM.ifc",
                                &ifc,
                            ) {
                                self.export_status =
                                    Some(format!("OpenBIM IFC 4.3 saved to: {}", path));
                            }
                        }
                    }
                });
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            // Tier 1: Discipline Category Switcher (reduces 19 flat tabs into 4 clean engineering tracks)
            let mut active_category = self.active_subtab.category();
            let prev_category = active_category;
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(6.0, 6.0);
                ui.label(
                    RichText::new("DISCIPLINE:")
                        .size(10.5)
                        .strong()
                        .color(theme.text_muted),
                );
                for &cat in AnalyzeCategory::all() {
                    let is_active = active_category == cat;
                    let bg = if is_active {
                        theme.accent_blue.gamma_multiply(0.24)
                    } else {
                        theme.bg_canvas
                    };
                    let fg = if is_active {
                        Color32::WHITE
                    } else {
                        theme.text_secondary
                    };
                    let stroke = if is_active {
                        Stroke::new(1.2, theme.accent_blue)
                    } else {
                        Stroke::new(1.0, theme.border_subtle)
                    };
                    if ui
                        .add(
                            egui::Button::new(RichText::new(cat.label()).size(11.5).strong().color(fg))
                                .fill(bg)
                                .stroke(stroke)
                                .corner_radius(CornerRadius::same(7)),
                        )
                        .clicked()
                    {
                        active_category = cat;
                    }
                }
            });

            if active_category != prev_category {
                self.active_subtab = active_category.default_subtab();
            }

            ui.add_space(6.0);

            // Tier 2: Filtered Sub-Tabs inside the active discipline (responsive wrapping / phone horizontal scroll)
            ui_kit::subtab_bar(
                ui,
                theme,
                "analyze_filtered_subtabs",
                &mut self.active_subtab,
                active_category.subtabs(),
            );
        });
    }

    pub(super) fn render_bim_sync_gate(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(ui, theme, features, FeatureId::LiveBimToFemAutoSync, |ui| {
            ui_kit::responsive_grid_n(ui, 4, 190.0, |ui, idx| match idx {
                0 => {
                    ui.label(RichText::new("Analytical Source Bus").size(10.5).strong().color(theme.text_muted));
                    let _ = ui.selectable_label(true, "Active BimProject (3D Frame + Shells)");
                }
                1 => {
                    ui.label(RichText::new("Mesh & Diaphragm Coupling").size(10.5).strong().color(theme.text_muted));
                    let _ = ui.selectable_label(false, "Auto Rigid Diaphragm + 0.5m Mesh");
                }
                2 => {
                    ui.label(RichText::new("1D GIS Seismic Coupling").size(10.5).strong().color(theme.text_muted));
                    let _ = ui.selectable_label(false, "Live pusGen Ss/S1 -> SNI 1726");
                }
                _ => {
                    ui.label(RichText::new("Reactive Solver Trigger").size(10.5).strong().color(theme.text_muted));
                    let _ = ui.button("⚡ Compile & Solve Live BIM -> FEM");
                }
            });
        });
    }

    pub(super) fn metric_card(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        title: &str,
        value: &str,
        sub: &str,
        color: Color32,
    ) {
        ui_kit::stat_card(ui, theme, title, value, sub, color);
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
