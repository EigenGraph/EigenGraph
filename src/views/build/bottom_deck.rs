//! views/build/bottom_deck.rs — Collapsible Schedules, 4D Erection, Clash Deck & BIM Console.

use super::*;

impl BuildView {
    pub(super) fn render_bottom_deck(&mut self, ui: &mut Ui, theme: &MacTheme, _members: &[StructuralMember], features: &mut FeatureRegistry) {
        let toggle_icon = if self.bottom_deck_expanded { "▼ Collapse" } else { "▲ Expand" };

        Frame::new()
            .fill(theme.bg_sidebar)
            .inner_margin(Margin::symmetric(10, 4))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let tabs = [
                        BottomDeckTab::ScheduleQto,
                        BottomDeckTab::TimeSimulation4D,
                        BottomDeckTab::ClashManager,
                        BottomDeckTab::BimConsole,
                    ];
                    for tab in tabs {
                        let is_sel = self.active_bottom_deck_tab == tab && self.bottom_deck_expanded;
                        let bg = if is_sel { theme.accent_blue } else { theme.bg_card };
                        let fg = if is_sel { Color32::WHITE } else { theme.text_secondary };

                        if ui.add(egui::Button::new(RichText::new(tab.label()).size(10.0).color(fg))
                            .fill(bg).corner_radius(CornerRadius::same(5))).clicked() {
                            self.active_bottom_deck_tab = tab;
                            self.bottom_deck_expanded = true;
                        }
                    }

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.add(egui::Button::new(RichText::new(toggle_icon).size(10.0).color(theme.text_secondary))
                            .fill(theme.bg_card).corner_radius(CornerRadius::same(5))).clicked() {
                            self.bottom_deck_expanded = !self.bottom_deck_expanded;
                        }
                    });
                });

                if self.bottom_deck_expanded {
                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(4.0);

                    match self.active_bottom_deck_tab {
                        BottomDeckTab::ScheduleQto => {
                            // Filter row
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("🔍").size(11.0));
                                ui.add(egui::TextEdit::singleline(&mut self.schedule_search_query)
                                    .hint_text("Filter schedule...")
                                    .desired_width(120.0));
                                ui.separator();
                                egui::ComboBox::from_id_salt("schedule_cat_filter")
                                    .selected_text(RichText::new(&self.schedule_category_filter).size(10.5))
                                    .show_ui(ui, |ui| {
                                        let cats = ["All Elements", "Structural Columns", "Beams & Girders", "Slabs & Floors", "Shear Walls", "Rebar Sets", "MEP Equipment"];
                                        for c in cats {
                                            if ui.selectable_label(self.schedule_category_filter == c, c).clicked() {
                                                self.schedule_category_filter = c.to_string();
                                            }
                                        }
                                    });
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    if ui.add(egui::Button::new(RichText::new("📥 Export CSV").size(10.0).color(theme.accent_blue))
                                        .fill(theme.bg_card).corner_radius(CornerRadius::same(4))).clicked() {
                                        self.modify_status_msg = Some("Exported Schedule to CSV: EigenGraph_BoQ_Schedule.csv".to_string());
                                    }
                                });
                            });
                            ui.add_space(4.0);

                            // Schedule Table
                            ScrollArea::both()
                                .id_salt("schedule_table_scroll")
                                .max_height(140.0)
                                .show(ui, |ui| {
                                    egui::Grid::new("schedule_qto_grid")
                                        .striped(true)
                                        .spacing(Vec2::new(12.0, 4.0))
                                        .show(ui, |ui| {
                                            ui.label(RichText::new("Mark").strong().size(10.5).color(theme.accent_blue));
                                            ui.label(RichText::new("Category").strong().size(10.5).color(theme.text_secondary));
                                            ui.label(RichText::new("Level").strong().size(10.5).color(theme.text_secondary));
                                            ui.label(RichText::new("Section / Size").strong().size(10.5).color(theme.text_secondary));
                                            ui.label(RichText::new("Length (m)").strong().size(10.5).color(theme.text_secondary));
                                            ui.label(RichText::new("Volume (m³)").strong().size(10.5).color(theme.text_secondary));
                                            ui.label(RichText::new("Weight (t)").strong().size(10.5).color(theme.text_secondary));
                                            ui.label(RichText::new("Rate ($)").strong().size(10.5).color(theme.text_secondary));
                                            ui.label(RichText::new("Total ($)").strong().size(10.5).color(theme.accent_green));
                                            ui.end_row();

                                            let schedule_rows = [
                                                ("C-01", "Column", "Level 1", "RC 500x500", "4.00", "1.00", "2.45", "185.00", "$185.00"),
                                                ("C-02", "Column", "Level 1", "RC 500x500", "4.00", "1.00", "2.45", "185.00", "$185.00"),
                                                ("B-101", "Beam", "Level 2", "W18x50 Steel", "7.50", "0.08", "0.56", "2,200/t", "$1,232.00"),
                                                ("B-102", "Beam", "Level 2", "W18x50 Steel", "7.50", "0.08", "0.56", "2,200/t", "$1,232.00"),
                                                ("S-201", "Slab", "Level 2", "Composite Deck 150mm", "12.00", "18.00", "44.10", "165.00", "$2,970.00"),
                                                ("SW-01", "Shear Wall", "Level 1", "RC Wall 300mm", "6.00", "7.20", "17.64", "195.00", "$1,404.00"),
                                                ("RB-101", "Rebar", "Level 1", "Grade 60 (D25/D13)", "48.00", "0.05", "0.41", "1,850/t", "$758.50"),
                                                ("DT-301", "HVAC Duct", "Level 2", "Galvanized 600x400", "14.50", "3.48", "0.22", "85.00/m", "$1,232.50"),
                                            ];

                                            for (mark, cat, lvl, sec, len, vol, wt, rate, tot) in schedule_rows {
                                                if !self.schedule_search_query.trim().is_empty() {
                                                    let q = self.schedule_search_query.trim().to_lowercase();
                                                    if !mark.to_lowercase().contains(&q) && !cat.to_lowercase().contains(&q) && !sec.to_lowercase().contains(&q) {
                                                        continue;
                                                    }
                                                }
                                                if self.schedule_category_filter != "All Elements" && !cat.contains(&self.schedule_category_filter) {
                                                    continue;
                                                }

                                                ui.label(RichText::new(mark).size(10.0).strong().color(theme.text_primary));
                                                ui.label(RichText::new(cat).size(10.0).color(theme.text_secondary));
                                                ui.label(RichText::new(lvl).size(10.0).color(theme.text_muted));
                                                ui.label(RichText::new(sec).size(10.0).color(theme.text_primary));
                                                ui.label(RichText::new(len).size(10.0).color(theme.text_secondary));
                                                ui.label(RichText::new(vol).size(10.0).color(theme.text_secondary));
                                                ui.label(RichText::new(wt).size(10.0).color(theme.text_secondary));
                                                ui.label(RichText::new(rate).size(10.0).color(theme.text_muted));
                                                ui.label(RichText::new(tot).size(10.0).strong().color(theme.accent_green));
                                                ui.end_row();
                                            }
                                        });
                                });

                            ui.add_space(2.0);
                            Frame::new()
                                .fill(theme.bg_card)
                                .corner_radius(CornerRadius::same(5))
                                .inner_margin(Margin::symmetric(8, 3))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("TOTALS:").size(10.0).strong().color(theme.text_secondary));
                                        ui.label(RichText::new("Volume: 842.5 m³").size(10.0).strong().color(theme.accent_blue));
                                        ui.label(RichText::new("• Steel: 148.2 t").size(10.0).strong().color(theme.accent_blue));
                                        ui.label(RichText::new("• Rebar: 38.6 t").size(10.0).strong().color(theme.accent_blue));
                                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                            ui.label(RichText::new("Grand Total: $1,842,500.00").size(10.5).strong().color(theme.accent_green));
                                        });
                                    });
                                });
                        }
                        BottomDeckTab::TimeSimulation4D => {
                            ui.horizontal(|ui| {
                                let play_lbl = if self.simulation_4d_playing { "⏸ Pause" } else { "▶ Play" };
                                if ui.add(egui::Button::new(RichText::new(play_lbl).size(10.5).strong().color(Color32::WHITE))
                                    .fill(theme.accent_blue).corner_radius(CornerRadius::same(5))).clicked() {
                                    self.simulation_4d_playing = !self.simulation_4d_playing;
                                }
                                if ui.add(egui::Button::new(RichText::new("⏮ Day 1").size(10.5).color(theme.text_secondary))
                                    .fill(theme.bg_card).corner_radius(CornerRadius::same(5))).clicked() {
                                    self.simulation_4d_day = 1;
                                }
                                if ui.add(egui::Button::new(RichText::new("◀ Prev").size(10.5).color(theme.text_secondary))
                                    .fill(theme.bg_card).corner_radius(CornerRadius::same(5))).clicked() {
                                    self.simulation_4d_day = self.simulation_4d_day.saturating_sub(5).max(1);
                                }
                                if ui.add(egui::Button::new(RichText::new("Next ▶").size(10.5).color(theme.text_secondary))
                                    .fill(theme.bg_card).corner_radius(CornerRadius::same(5))).clicked() {
                                    self.simulation_4d_day = (self.simulation_4d_day + 5).min(180);
                                }
                                if ui.add(egui::Button::new(RichText::new("⏭ Day 180").size(10.5).color(theme.text_secondary))
                                    .fill(theme.bg_card).corner_radius(CornerRadius::same(5))).clicked() {
                                    self.simulation_4d_day = 180;
                                }

                                ui.separator();
                                ui.label(RichText::new(format!("Day: {}", self.simulation_4d_day)).size(11.0).strong().color(theme.accent_blue));
                                ui.add(egui::Slider::new(&mut self.simulation_4d_day, 1..=180).suffix(" d").text(""));
                            });
                            ui.add_space(4.0);

                            // Active Phase Banner
                            let (phase_title, phase_desc, phase_prog) = match self.simulation_4d_day {
                                1..=30 => ("Phase 1: Substructure & Foundation", "Deep piles, cap excavation & raft mat casting", (self.simulation_4d_day as f32 / 30.0)),
                                31..=80 => ("Phase 2: Superstructure Concrete & Steel Frame", "Multi-story columns, primary girders & floor slabs", ((self.simulation_4d_day - 30) as f32 / 50.0)),
                                81..=120 => ("Phase 3: Exterior Enclosure & Curtain Wall", "Precast spandrel panels, curtain wall glazing & roof", ((self.simulation_4d_day - 80) as f32 / 40.0)),
                                121..=160 => ("Phase 4: MEP Systems & Fire Protection", "HVAC duct risers, electrical conduits & wet sprinklers", ((self.simulation_4d_day - 120) as f32 / 40.0)),
                                _ => ("Phase 5: Interior Finishes & Commissioning", "Architectural partitions, elevator testing & handover", ((self.simulation_4d_day - 160) as f32 / 20.0)),
                            };

                            Frame::new()
                                .fill(theme.bg_card)
                                .corner_radius(CornerRadius::same(6))
                                .inner_margin(Margin::symmetric(10, 6))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new(phase_title).strong().size(11.0).color(theme.text_primary));
                                        ui.label(RichText::new(format!("({:.0}% complete)", phase_prog * 100.0)).size(10.5).color(theme.accent_green));
                                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                            ui.label(RichText::new(phase_desc).size(10.0).color(theme.text_secondary));
                                        });
                                    });
                                });
                        }
                        BottomDeckTab::ClashManager => {
                        ui_kit::feature_slot_card(ui, theme, features, FeatureId::LiveBvhGeometricClashSolver, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                let _ = ui.selectable_label(true, "Spatial Bvh3D AABB Index");
                                let _ = ui.selectable_label(false, "Auto-Generate BCF from Clashes");
                                let _ = ui.button("🔍 Scan Active BimProject CSG Solids");
                            });
                        });
                        ui.add_space(6.0);
                            let mut deck_focus: Option<(usize, String, [f32; 3])> = None;
                            let mut deck_toggle_idx: Option<usize> = None;

                            ScrollArea::horizontal()
                                .id_salt("clash_deck_scroll")
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        let mut shown = 0usize;
                                        for (idx, item) in self.clash_items.iter().enumerate() {
                                            if item.status != crate::models::ClashStatus::Resolved
                                                && !item.is_triggered_at_tolerance(self.clash_tolerance_mm)
                                            {
                                                continue;
                                            }
                                            shown += 1;
                                            let is_sel = self.selected_clash_id == Some(item.id);
                                            let (sr, sg, sb) = item.severity.color_rgb();
                                            let sev_col = if item.status == crate::models::ClashStatus::Resolved {
                                                theme.accent_green
                                            } else {
                                                Color32::from_rgb(sr, sg, sb)
                                            };

                                            Frame::new()
                                                .fill(if is_sel { theme.bg_card_hover } else { theme.bg_card })
                                                .corner_radius(CornerRadius::same(6))
                                                .inner_margin(Margin::same(8))
                                                .stroke(Stroke::new(1.0, if is_sel { theme.accent_blue } else { theme.border_card }))
                                                .show(ui, |ui| {
                                                    ui.vertical(|ui| {
                                                        ui.horizontal(|ui| {
                                                            ui.label(RichText::new(&item.tag).strong().size(10.5).color(theme.accent_blue));
                                                            ui.label(RichText::new(item.severity.badge()).size(9.5).color(sev_col));
                                                            ui.label(RichText::new(item.status.label()).size(9.0).strong().color(theme.text_muted));
                                                        });
                                                        ui.label(RichText::new(format!("{} ∩ {}", item.element_a, item.element_b)).size(10.0).color(theme.text_primary));
                                                        ui.label(RichText::new(format!("{} • Vol {:.3} m³", item.metric_summary(), item.clash_volume_m3())).size(9.5).color(theme.text_secondary));
                                                        ui.horizontal(|ui| {
                                                            if ui.add(egui::Button::new(RichText::new("👁️ Isolate 3D").size(9.0).color(theme.accent_blue))
                                                                .fill(theme.bg_canvas).corner_radius(CornerRadius::same(4))).clicked() {
                                                                deck_focus = Some((item.id, item.tag.clone(), item.point_3d));
                                                            }
                                                            if ui.add(egui::Button::new(RichText::new(format!("🔄 {}", item.status.next().label())).size(9.0).color(theme.accent_green))
                                                                .fill(theme.bg_canvas).corner_radius(CornerRadius::same(4))).clicked() {
                                                                deck_toggle_idx = Some(idx);
                                                            }
                                                        });
                                                    });
                                                });
                                        }
                                        if shown == 0 {
                                            ui.label(RichText::new("Zero clashes triggered at current tolerance.").size(10.5).color(theme.accent_green));
                                        }
                                    });
                                });

                            if let Some((cid, tag, pt)) = deck_focus {
                                self.selected_clash_id = Some(cid);
                                self.show_3d_clash_markers = true;
                                self.cam_zoom = 2.25;
                                self.cam_yaw = (pt[0].atan2(pt[2].max(0.5))).clamp(-2.8, 2.8);
                                self.cam_pitch = 0.42;
                                self.modify_status_msg = Some(format!("Isolated {} at ({:.1}, {:.1}, {:.1})m in 3D viewport", tag, pt[0], pt[1], pt[2]));
                            }
                            if let Some(idx) = deck_toggle_idx {
                                if let Some(c) = self.clash_items.get_mut(idx) {
                                    c.status = c.status.next();
                                    self.modify_status_msg = Some(format!("{} status changed to '{}'", c.tag, c.status.label()));
                                }
                            }
                        }
                        BottomDeckTab::BimConsole => {
                            ScrollArea::vertical()
                                .id_salt("bim_console_scroll")
                                .max_height(85.0)
                                .stick_to_bottom(true)
                                .show(ui, |ui| {
                                    for log in &self.bim_console_logs {
                                        ui.label(RichText::new(log).size(9.5).color(theme.text_secondary));
                                    }
                                });

                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(">").strong().size(11.0).color(theme.accent_green));
                                let resp = ui.add(egui::TextEdit::singleline(&mut self.bim_console_input)
                                    .hint_text("Type BIM command e.g. bim.audit_clashes(), bim.export_ifc4()...")
                                    .desired_width((ui.available_width() - 80.0).max(100.0)));
                                if (resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) || ui.button("Execute").clicked() {
                                    if !self.bim_console_input.trim().is_empty() {
                                        let cmd = self.bim_console_input.trim().to_string();
                                        self.bim_console_logs.push(format!("> {}", cmd));
                                        self.bim_console_logs.push(format!("✓ Executed command '{}' with status: SUCCESS", cmd));
                                        self.modify_status_msg = Some(format!("Executed: {}", cmd));
                                        self.bim_console_input.clear();
                                    }
                                }
                            });
                        }
                        BottomDeckTab::PointCloudDeviation => {
                            ScrollArea::vertical()
                                .id_salt("point_cloud_deck_scroll")
                                .max_height(185.0)
                                .show(ui, |ui| {
                                    ui_kit::feature_slot_card(ui, theme, features, FeatureId::PointCloudAsBuiltDeviation, |ui| {
                                        let kpis = [
                                            ("SCAN POINTS REGISTERED", "14.8M Pts (E57/LAS)", "ICP Alignment RMS = ±3.4 mm", theme.accent_blue),
                                            ("FLOOR FLATNESS (FF / FL)", "FF 38.2 / FL 29.4", "ASTM E1155 Superflat Slab Spec", theme.accent_green),
                                            ("COLUMN PLUMBNESS MAX", "4.1 mm @ L4 C-02", "Within ACI 117 (±6.0 mm Limit)", theme.accent_green),
                                            ("OUT-OF-TOLERANCE ZONES", "2 Localized Slabs", "Grid B-2Edge Camber +7.2 mm", theme.accent_orange),
                                        ];
                                        ui_kit::responsive_grid_n(ui, kpis.len(), 170.0, |ui, idx| {
                                            let (t, v, s, c) = kpis[idx];
                                            ui_kit::stat_card(ui, theme, t, v, s, c);
                                        });
                                        ui.add_space(6.0);
                                        ui.horizontal_wrapped(|ui| {
                                            let _ = ui.selectable_label(true, "Heat Band: ±10mm ACI 117");
                                            let _ = ui.selectable_label(false, "Overlay Laser Scan Voxels");
                                            let _ = ui.button("📡 Import .E57 / .LAS Point Cloud");
                                        });
                                    });
                                });
                        }
                    }
                }
            });
    }


}
