//! views/industrialized/dfma_robotics.rs — Modular DfMA Pipeline, QR/RFID Console, Robotics & Crane Erection.

use super::*;

impl IndustrializedView {
    pub(super) fn render_modular_dfma(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let total_units = self.prefab_units.len();
        let erected_count = self
            .prefab_units
            .iter()
            .filter(|u| u.stage == PrefabSupplyStage::ErectedAndTorqued)
            .count();
        let erected_pct = if total_units > 0 {
            (erected_count as f32 / total_units as f32) * 100.0
        } else {
            0.0
        };
        let transit_or_staged = self
            .prefab_units
            .iter()
            .filter(|u| {
                matches!(
                    u.stage,
                    PrefabSupplyStage::InTransitFleet | PrefabSupplyStage::SiteLaydownStaging
                )
            })
            .count();
        let qa_passed = self.prefab_units.iter().filter(|u| u.qa_passed).count();
        let max_swl_pct = self
            .prefab_units
            .iter()
            .map(|u| u.crane_utilization_pct())
            .fold(0.0f32, f32::max);

        let kpis = [
            (
                "ERECTION PROGRESS",
                format!("{}/{} ({:.0}%)", erected_count, total_units, erected_pct),
                "Torqued & As-Built Verified".to_string(),
                theme.accent_green,
            ),
            (
                "OFF-SITE PREFAB RATIO",
                format!("{:.1}%", self.offsite_ratio_pct),
                "Factory Volumetric & Panels".to_string(),
                theme.accent_blue,
            ),
            (
                "IN-TRANSIT & STAGING",
                format!("{} Modules", transit_or_staged),
                format!("QA Passed: {}/{} Units", qa_passed, total_units),
                theme.accent_orange,
            ),
            (
                "PEAK CRANE SWL LOAD",
                format!("{:.0}% SWL", max_swl_pct),
                "12.0T Jib Rigging Safety Guard".to_string(),
                if max_swl_pct > 85.0 {
                    theme.accent_red
                } else {
                    theme.accent_green
                },
            ),
        ];
        self.render_responsive_cards(ui, &kpis, theme);

        ui.add_space(14.0);

        // 6-Stage DfMA Supply Chain Pipeline Funnel
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(14))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new("🏭 6-STAGE DfMA SUPPLY CHAIN PIPELINE")
                            .size(12.5)
                            .strong()
                            .color(theme.text_primary),
                    );
                    ui.label(
                        RichText::new("Click any stage to filter modular prefabricated elements")
                            .size(10.0)
                            .color(theme.text_muted),
                    );
                });
                ui.add_space(8.0);

                ui.horizontal_wrapped(|ui| {
                    let all_sel = self.prefab_stage_filter.is_none();
                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new(format!("All ({})", total_units))
                                    .size(10.5)
                                    .strong()
                                    .color(if all_sel { Color32::WHITE } else { theme.text_secondary }),
                            )
                            .fill(if all_sel { theme.accent_blue } else { theme.bg_canvas })
                            .corner_radius(CornerRadius::same(6)),
                        )
                        .clicked()
                    {
                        self.prefab_stage_filter = None;
                    }

                    for stage in PrefabSupplyStage::all() {
                        let count = self.prefab_units.iter().filter(|u| u.stage == *stage).count();
                        let (r, g, b) = stage.color_rgb();
                        let st_col = Color32::from_rgb(r, g, b);
                        let is_sel = self.prefab_stage_filter == Some(*stage);
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new(format!("{} ({})", stage.label(), count))
                                        .size(10.0)
                                        .strong()
                                        .color(if is_sel { Color32::WHITE } else { st_col }),
                                )
                                .fill(if is_sel { st_col } else { theme.bg_canvas })
                                .corner_radius(CornerRadius::same(6)),
                            )
                            .clicked()
                        {
                            self.prefab_stage_filter = if is_sel { None } else { Some(*stage) };
                        }
                    }
                });

                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new("Target Prefabrication Ratio:")
                            .size(11.0)
                            .color(theme.text_secondary),
                    );
                    ui.add(egui::Slider::new(&mut self.offsite_ratio_pct, 50.0..=95.0).suffix(" %"));
                });
            });

        ui.add_space(12.0);

        // Simulated QR / RFID Scanner & Check-In Console
        let mut scan_trigger = false;
        let mut advance_selected_trigger = false;
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(14))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.label(
                    RichText::new("📡 SIMULATED QR / RFID SCANNER & SITE GATE CHECK-IN CONSOLE")
                        .size(12.0)
                        .strong()
                        .color(theme.accent_blue),
                );
                ui.add_space(6.0);

                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new("Barcode / RFID Tag:")
                            .size(11.0)
                            .color(theme.text_secondary),
                    );
                    let w = ui.available_width().min(210.0).max(120.0);
                    let edit_res = ui.add(
                        egui::TextEdit::singleline(&mut self.scanner_input)
                            .hint_text("Scan tag e.g. MOD-TR-014 or RFID-EG-88412")
                            .desired_width(w),
                    );
                    if edit_res.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        scan_trigger = true;
                    }

                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("📡 Scan & Check-In")
                                    .size(10.5)
                                    .strong()
                                    .color(Color32::WHITE),
                            )
                            .fill(theme.accent_blue)
                            .corner_radius(CornerRadius::same(6)),
                        )
                        .clicked()
                    {
                        scan_trigger = true;
                    }

                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("⏭️ Advance Stage")
                                    .size(10.5)
                                    .strong()
                                    .color(Color32::WHITE),
                            )
                            .fill(theme.accent_green)
                            .corner_radius(CornerRadius::same(6)),
                        )
                        .clicked()
                    {
                        advance_selected_trigger = true;
                    }
                });

                ui.add_space(6.0);
                let tags: Vec<String> = self.prefab_units.iter().map(|u| u.tag_id.clone()).collect();
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Quick Tap Tag:").size(10.0).color(theme.text_muted));
                    for tag in tags {
                        let is_sel = self.selected_module_tag == tag;
                        if ui
                            .selectable_label(is_sel, RichText::new(&tag).size(10.0).monospace())
                            .clicked()
                        {
                            self.scanner_input = tag.clone();
                            self.selected_module_tag = tag;
                        }
                    }
                });

                ui.add_space(6.0);
                Frame::new()
                    .fill(theme.bg_canvas)
                    .corner_radius(CornerRadius::same(6))
                    .inner_margin(Margin::symmetric(10, 6))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        for log in self.scanner_log.iter().rev().take(3) {
                            ui.label(
                                RichText::new(format!("• {}", log))
                                    .size(9.5)
                                    .monospace()
                                    .color(theme.text_secondary),
                            );
                        }
                    });
            });

        if scan_trigger {
            let query = self.scanner_input.trim().to_uppercase();
            let now = crate::core::web_bridge::unix_timestamp_label();
            if let Some(unit) = self.prefab_units.iter_mut().find(|u| {
                u.tag_id.to_uppercase() == query || u.rfid_code.to_uppercase() == query
            }) {
                unit.last_scan_timestamp = now.clone();
                self.selected_module_tag = unit.tag_id.clone();
                let msg = format!(
                    "Checked in {} ({}) @ stage '{}' [{}].",
                    unit.tag_id,
                    unit.rfid_code,
                    unit.stage.label(),
                    now
                );
                self.scanner_log.push(msg.clone());
                self.export_status = Some(msg);
            } else {
                let err = format!(
                    "RFID Lookup Miss: '{}' did not match any prefabricated module tag.",
                    self.scanner_input.trim()
                );
                self.scanner_log.push(err.clone());
                self.export_status = Some(err);
            }
        }

        if advance_selected_trigger {
            let query = if !self.scanner_input.trim().is_empty() {
                self.scanner_input.trim().to_uppercase()
            } else {
                self.selected_module_tag.to_uppercase()
            };
            let now = crate::core::web_bridge::unix_timestamp_label();
            if let Some(unit) = self.prefab_units.iter_mut().find(|u| {
                u.tag_id.to_uppercase() == query || u.rfid_code.to_uppercase() == query
            }) {
                self.selected_module_tag = unit.tag_id.clone();
                if let Some(next_stage) = unit.stage.next() {
                    unit.stage = next_stage;
                    if next_stage.step_index() >= PrefabSupplyStage::QaNdtInspection.step_index() {
                        unit.qa_passed = true;
                    }
                    unit.last_scan_timestamp = now.clone();
                    let msg = format!(
                        "{} advanced to '{}' (Crane SWL {:.0}%) [{}].",
                        unit.tag_id,
                        unit.stage.label(),
                        unit.crane_utilization_pct(),
                        now
                    );
                    self.scanner_log.push(msg.clone());
                    self.export_status = Some(msg);
                } else {
                    let msg = format!(
                        "{} is already Erected & Torqued at {}.",
                        unit.tag_id, unit.target_zone
                    );
                    self.scanner_log.push(msg.clone());
                    self.export_status = Some(msg);
                }
            }
        }

        ui.add_space(12.0);

        // Filtered Modular Prefab Unit Cards (Responsive Grid)
        let filtered_indices: Vec<usize> = self
            .prefab_units
            .iter()
            .enumerate()
            .filter_map(|(i, u)| {
                if self.prefab_stage_filter.map_or(true, |st| u.stage == st) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        let mut card_advance_idx: Option<usize> = None;
        let mut card_select_tag: Option<String> = None;

        if filtered_indices.is_empty() {
            theme.card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("No prefabricated units match the selected pipeline stage filter.")
                        .size(11.0)
                        .color(theme.text_muted),
                );
            });
        } else {
            let units = &self.prefab_units;
            let selected_tag = &self.selected_module_tag;

            ui_kit::responsive_grid_n(ui, filtered_indices.len(), 320.0, |ui, pos| {
                let u_idx = filtered_indices[pos];
                let unit = &units[u_idx];
                let (r, g, b) = unit.stage.color_rgb();
                let st_col = Color32::from_rgb(r, g, b);
                let is_sel = *selected_tag == unit.tag_id;
                let swl_pct = unit.crane_utilization_pct();
                let swl_col = if swl_pct > 85.0 {
                    theme.accent_red
                } else if swl_pct > 65.0 {
                    theme.accent_orange
                } else {
                    theme.accent_green
                };

                Frame::new()
                    .fill(theme.bg_card)
                    .corner_radius(CornerRadius::same(10))
                    .inner_margin(Margin::same(12))
                    .stroke(Stroke::new(
                        if is_sel { 1.6 } else { 1.0 },
                        if is_sel { theme.accent_blue } else { theme.border_subtle },
                    ))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.horizontal_wrapped(|ui| {
                            ui.label(
                                RichText::new(&unit.tag_id)
                                    .size(12.0)
                                    .strong()
                                    .monospace()
                                    .color(theme.accent_blue),
                            );
                            ui.label(
                                RichText::new(&unit.rfid_code)
                                    .size(9.5)
                                    .monospace()
                                    .color(theme.text_muted),
                            );
                            Frame::new()
                                .fill(st_col.gamma_multiply(0.18))
                                .corner_radius(CornerRadius::same(4))
                                .inner_margin(Margin::symmetric(6, 2))
                                .show(ui, |ui| {
                                    ui.label(
                                        RichText::new(unit.stage.label())
                                            .size(9.5)
                                            .strong()
                                            .color(st_col),
                                    );
                                });
                            ui.label(
                                RichText::new(if unit.qa_passed {
                                    "✅ QA Passed"
                                } else {
                                    "⏳ Pending QA"
                                })
                                .size(9.5)
                                .strong()
                                .color(if unit.qa_passed {
                                    theme.accent_green
                                } else {
                                    theme.accent_orange
                                }),
                            );
                        });

                        ui.add_space(4.0);
                        ui.label(
                            RichText::new(&unit.name)
                                .size(11.5)
                                .strong()
                                .color(theme.text_primary),
                        );
                        ui.label(
                            RichText::new(format!(
                                "{} • Zone: {} • Envelope: {:.1}×{:.2}×{:.1} m",
                                unit.module_type.label(),
                                unit.target_zone,
                                unit.dimensions_m[0],
                                unit.dimensions_m[1],
                                unit.dimensions_m[2]
                            ))
                            .size(9.5)
                            .color(theme.text_secondary),
                        );

                        ui.add_space(6.0);
                        ui.label(
                            RichText::new(format!(
                                "Crane Hoist Check: {:.2} t / {:.1} t Max SWL ({:.0}%)",
                                unit.mass_tons, unit.crane_swl_tons, swl_pct
                            ))
                            .size(9.5)
                            .color(swl_col),
                        );
                        ui.add(
                            egui::ProgressBar::new((swl_pct / 100.0).clamp(0.0, 1.0))
                                .desired_height(6.0)
                                .fill(swl_col),
                        );

                        ui.add_space(6.0);
                        // 6-Step Visual Pipeline Stepper
                        let step_idx = unit.stage.step_index();
                        let (bar_resp, bar_painter) = ui.allocate_painter(
                            Vec2::new(ui.available_width().max(120.0), 8.0),
                            Sense::hover(),
                        );
                        let b_rect = bar_resp.rect;
                        let seg_w = ((b_rect.width() - 10.0) / 6.0).max(8.0);
                        for s in 0..6 {
                            let sx = b_rect.left() + (s as f32) * (seg_w + 2.0);
                            let seg_rect = Rect::from_min_size(
                                egui::pos2(sx, b_rect.top()),
                                Vec2::new(seg_w, 6.0),
                            );
                            let seg_col = if s <= step_idx {
                                st_col
                            } else {
                                Color32::from_rgb(42, 48, 60)
                            };
                            bar_painter.rect_filled(seg_rect, CornerRadius::same(2), seg_col);
                        }

                        ui.add_space(4.0);
                        ui.horizontal_wrapped(|ui| {
                            ui.label(
                                RichText::new(format!("Scan: {}", unit.last_scan_timestamp))
                                    .size(9.0)
                                    .color(theme.text_muted),
                            );
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new("📡 Select")
                                            .size(9.5)
                                            .color(theme.accent_blue),
                                    )
                                    .fill(theme.bg_canvas)
                                    .corner_radius(CornerRadius::same(4)),
                                )
                                .clicked()
                            {
                                card_select_tag = Some(unit.tag_id.clone());
                            }
                            if let Some(next_st) = unit.stage.next() {
                                if ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new(format!("⏭️ {}", next_st.label()))
                                                .size(9.5)
                                                .color(theme.accent_green),
                                        )
                                        .fill(theme.bg_canvas)
                                        .corner_radius(CornerRadius::same(4)),
                                    )
                                    .clicked()
                                {
                                    card_advance_idx = Some(u_idx);
                                }
                            }
                        });
                    });
            });
        }

        if let Some(tag) = card_select_tag {
            self.scanner_input = tag.clone();
            self.selected_module_tag = tag;
        }
        if let Some(idx) = card_advance_idx {
            let now = crate::core::web_bridge::unix_timestamp_label();
            if let Some(unit) = self.prefab_units.get_mut(idx) {
                if let Some(next_stage) = unit.stage.next() {
                    unit.stage = next_stage;
                    if next_stage.step_index() >= PrefabSupplyStage::QaNdtInspection.step_index() {
                        unit.qa_passed = true;
                    }
                    unit.last_scan_timestamp = now.clone();
                    self.selected_module_tag = unit.tag_id.clone();
                    self.scanner_input = unit.tag_id.clone();
                    let msg = format!(
                        "{} moved to '{}' [{}].",
                        unit.tag_id,
                        unit.stage.label(),
                        now
                    );
                    self.scanner_log.push(msg.clone());
                    self.export_status = Some(msg);
                }
            }
        }
    }

    pub(super) fn render_robotic_automation(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let kpis = [
            (
                "ROBOTIC WELDING SPEED",
                format!("{:.1} mm/s", self.robotic_speed_mm_s),
                "6-Axis Articulated GMAW".to_string(),
                theme.accent_blue,
            ),
            (
                "DEFECT RATE",
                "< 0.04%".to_string(),
                "Ultrasonic NDT Verified".to_string(),
                theme.accent_green,
            ),
            (
                "G-CODE COMPLIANCE",
                "ISO 6983".to_string(),
                "Machine Tool Toolpaths".to_string(),
                theme.text_primary,
            ),
            (
                "PRODUCTIVITY GAIN",
                "+320%".to_string(),
                "Continuous Arc Operation".to_string(),
                theme.accent_green,
            ),
        ];
        self.render_responsive_cards(ui, &kpis, theme);

        ui.add_space(16.0);

        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.label(
                    RichText::new("🦾 Automated Structural Steel Robotic Workcell")
                        .size(13.5)
                        .strong()
                        .color(theme.text_primary),
                );
                ui.add_space(10.0);

                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new("End-Effector Feed Velocity:")
                            .size(11.5)
                            .color(theme.text_secondary),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.robotic_speed_mm_s, 5.0..=30.0)
                            .suffix(" mm/s")
                            .text(""),
                    );
                });
            });
    }

    pub(super) fn render_automated_erection(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.label(
                    RichText::new(
                        "🏗️ Autonomous Crane, Collision-Free Trajectory & LiDAR Drone Inspection",
                    )
                    .size(13.5)
                    .strong()
                    .color(theme.text_primary),
                );
                ui.add_space(10.0);

                let kpis = [
                    (
                        "CRANE TRAJECTORY CLEARANCE",
                        "2.8 m".to_string(),
                        "Dynamic Boom Collision-Free Zone".to_string(),
                        theme.accent_green,
                    ),
                    (
                        "DRONE LIDAR AS-BUILT",
                        "± 2.1 mm".to_string(),
                        "Model vs. As-Built Field Deviation".to_string(),
                        theme.accent_blue,
                    ),
                    (
                        "SVD ERECTION GUARD",
                        "Full Rank".to_string(),
                        "All Erection Stages Certified Mechanism-Free".to_string(),
                        theme.accent_green,
                    ),
                ];
                self.render_responsive_cards(ui, &kpis, theme);
            });
    }
}
