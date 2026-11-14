//! views/parametric/inspector.rs — Parametric Variables, Levels, Rooms & Bidirectional Sync Preview.

use super::*;

impl ParametricView {
    pub(super) fn render_variables_panel(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        active_pillar: &mut NavigationPillar,
        features: &mut FeatureRegistry,
        panel_width: Option<f32>,
    ) {
        theme.sidebar_frame().show(ui, |ui| {
            if let Some(w) = panel_width {
                ui.set_width(w);
            } else {
                ui.set_width(ui.available_width());
            }
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(
                    RichText::new("BIM LEVELS, ROOMS & DRIVING PARAMETERS")
                        .size(11.5)
                        .strong()
                        .color(theme.text_secondary),
                );
                ui.add_space(6.0);

                // Active Storey Level Selector
                theme.card_frame().show(ui, |ui| {
                    icons::icon_label(
                        ui,
                        AppIcon::Building,
                        13.0,
                        theme.accent_blue,
                        "ACTIVE STOREY LEVEL & VIEW RANGE",
                        10.5,
                        theme.accent_blue,
                        true,
                    );
                    ui.add_space(4.0);
                    for (idx, lvl) in self.bim_bus.project.levels.iter().enumerate() {
                        let is_sel = self.active_level_idx == idx;
                        if ui
                            .selectable_label(
                                is_sel,
                                format!("{} ({:+.0} mm)", lvl.name, lvl.elevation_mm),
                            )
                            .clicked()
                        {
                            self.active_level_idx = idx;
                        }
                    }
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(format!(
                            "Cut Plane: +{:.0}mm | Rooms: {} | Rebar: {:.1} kg",
                            self.bim_bus.project.view_range.cut_plane_mm,
                            self.bim_bus.project.rooms.len(),
                            self.bim_bus.project.total_rebar_weight_kg()
                        ))
                        .size(10.0)
                        .color(theme.accent_green),
                    );
                });

                ui.add_space(6.0);

                // Enclosed Planar Rooms Schedule Summary
                theme.card_frame().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        icons::icon_label(
                            ui,
                            AppIcon::Layers,
                            12.0,
                            theme.accent_orange,
                            "AUTO-BOUNDED ROOMS (HALF-EDGE)",
                            10.0,
                            theme.accent_orange,
                            true,
                        );
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if icons::icon_button(
                                ui,
                                AppIcon::Redo,
                                "Recalc",
                                10.0,
                                theme.text_primary,
                                theme.bg_sidebar,
                                Stroke::new(1.0, theme.border_subtle),
                                CornerRadius::same(5),
                                Vec2::new(0.0, 20.0),
                            )
                            .clicked()
                            {
                                let lvl_id = self
                                    .bim_bus
                                    .project
                                    .levels
                                    .get(self.active_level_idx)
                                    .map(|l| l.id)
                                    .unwrap_or(1);
                                let _ = self
                                    .bim_bus
                                    .execute(BimCommand::AutoDetectRooms { level_id: lvl_id });
                            }
                        });
                    });
                    ui.add_space(4.0);
                    for rm in &self.bim_bus.project.rooms {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!("{} {}", rm.number, rm.name))
                                    .size(10.5)
                                    .color(theme.text_primary),
                            );
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.label(
                                    RichText::new(format!(
                                        "{:.1} m² ({:.1}m)",
                                        rm.area_m2, rm.perimeter_m
                                    ))
                                    .size(10.5)
                                    .strong()
                                    .color(theme.accent_green),
                                );
                            });
                        });
                    }
                });

                ui.add_space(6.0);

                // Component Details & Driving Variables
                ui.label(
                    RichText::new("Component Name:")
                        .size(11.0)
                        .color(theme.text_muted),
                );
                ui.text_edit_singleline(&mut self.component_name);

                ui.add_space(4.0);
                ui.label(
                    RichText::new("Driving Dimensions & Parameters")
                        .size(11.5)
                        .strong()
                        .color(theme.text_primary),
                );
                ui.add_space(4.0);

                for var in &mut self.variables {
                    theme.card_frame().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(&var.name)
                                    .size(11.5)
                                    .strong()
                                    .color(theme.accent_blue),
                            );
                            ui.label(RichText::new("=").size(11.5).color(theme.text_secondary));
                            if ui
                                .add(
                                    egui::TextEdit::singleline(&mut var.expression)
                                        .desired_width(70.0),
                                )
                                .changed()
                            {
                                if let Ok(val) = var.expression.parse::<f64>() {
                                    var.evaluated_value = val;
                                }
                            }
                            ui.label(RichText::new(&var.unit).size(10.0).color(theme.text_muted));
                        });
                    });
                    ui.add_space(2.0);
                }

                ui.add_space(6.0);
                if ui.button("+ Add Variable Parameter").clicked() {
                    self.variables.push(ParametricVariable {
                        name: format!("var_{}", self.variables.len() + 1),
                        expression: "1000".to_string(),
                        evaluated_value: 1000.0,
                        unit: "mm".to_string(),
                    });
                }

                ui.add_space(8.0);
                if icons::icon_button(
                    ui,
                    AppIcon::ChartBar,
                    "View Results in Cost & Budgeting (5D BoQ)",
                    11.0,
                    Color32::WHITE,
                    theme.accent_blue,
                    Stroke::NONE,
                    CornerRadius::same(6),
                    Vec2::new(ui.available_width(), 28.0),
                )
                .clicked()
                {
                    *active_pillar = NavigationPillar::CostBudgeting;
                }

                ui.add_space(4.0);
                if icons::icon_button(
                    ui,
                    AppIcon::Leaf,
                    "Check Carbon in Sustainability (6D)",
                    11.0,
                    theme.text_primary,
                    theme.bg_card,
                    Stroke::NONE,
                    CornerRadius::same(6),
                    Vec2::new(ui.available_width(), 28.0),
                )
                .clicked()
                {
                    *active_pillar = NavigationPillar::SustainabilityEnergy;
                }

                ui.add_space(8.0);
                ui_kit::feature_slot_card(ui, theme, features, FeatureId::ParametricBidirectionalSync, |ui| {
                    ui.vertical(|ui| {
                        let _ = ui.selectable_label(true, "Source: ParametricView.bim_bus -> BuildView");
                        let _ = ui.selectable_label(false, "Auto-Sync Dynamo Bakes & 2D Walls");
                        let _ = icons::icon_button(
                            ui,
                            AppIcon::Redo,
                            "Push 2D Scene -> 3D BIM Bus",
                            10.5,
                            theme.text_primary,
                            theme.bg_sidebar,
                            Stroke::new(1.0, theme.border_subtle),
                            CornerRadius::same(5),
                            Vec2::new(0.0, 24.0),
                        );
                    });
                });

                ui.add_space(8.0);
                Frame::new()
                    .fill(Color32::from_rgb(18, 22, 28))
                    .corner_radius(CornerRadius::same(8))
                    .stroke(Stroke::new(1.0, theme.border_card))
                    .inner_margin(Margin::same(10))
                    .show(ui, |ui| {
                        icons::icon_label(
                            ui,
                            AppIcon::Bolt,
                            12.0,
                            theme.accent_blue,
                            "LIVE BIM -> FEM/ARI SOLVER",
                            10.5,
                            theme.accent_blue,
                            true,
                        );
                        ui.label(
                            RichText::new(format!(
                                "Global ARI: {:.3} (Safe/Redundant)",
                                self.live_ari_score
                            ))
                            .size(11.0)
                            .color(theme.accent_green),
                        );
                        ui.label(
                            RichText::new(format!(
                                "SVD Singularity Margin: {:.1}%",
                                self.live_svd_margin * 100.0
                            ))
                            .size(10.0)
                            .color(theme.text_secondary),
                        );
                    });
            });
        });
    }


}
