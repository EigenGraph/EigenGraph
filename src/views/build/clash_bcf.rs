//! views/build/clash_bcf.rs — Spatial Clash Detection Matrix & BCF 2.1 Collaboration Hub.

use super::*;

impl BuildView {
    pub(super) fn render_clash_tab(&mut self, ui: &mut Ui, theme: &MacTheme, features: &mut FeatureRegistry) {
        use crate::models::{BcfPriority, BcfTopicRecord, BcfTopicStatus, ClashSeverity, ClashStatus};

        ui.label(
            RichText::new("⚔️ SPATIAL CLASH MATRIX & INTERFERENCE DETECTOR")
                .size(11.5)
                .strong()
                .color(theme.text_secondary),
        );
        ui.add_space(4.0);

        ui_kit::feature_slot_card(ui, theme, features, FeatureId::LiveBvhGeometricClashSolver, |ui| {
            ui.horizontal_wrapped(|ui| {
                let _ = ui.selectable_label(true, "Bvh3D AABB Tree (0.01mm)");
                let _ = ui.selectable_label(false, "Rebar vs Duct Sweep");
                let _ = ui.button("⚡ Recompute Live 3D BVH Clashes");
            });
        });
        ui.add_space(6.0);

        let disc_matches = |pair: &str, filter: &str| -> bool {
            filter.starts_with("All") || pair == filter
        };

        let hard_active = self
            .clash_items
            .iter()
            .filter(|c| {
                c.severity == ClashSeverity::Hard
                    && c.status != ClashStatus::Resolved
                    && c.is_triggered_at_tolerance(self.clash_tolerance_mm)
                    && disc_matches(&c.discipline_pair, &self.clash_filter_discipline)
            })
            .count();
        let soft_active = self
            .clash_items
            .iter()
            .filter(|c| {
                matches!(
                    c.severity,
                    ClashSeverity::Soft | ClashSeverity::CodeTolerance
                ) && c.status != ClashStatus::Resolved
                    && c.is_triggered_at_tolerance(self.clash_tolerance_mm)
                    && disc_matches(&c.discipline_pair, &self.clash_filter_discipline)
            })
            .count();
        let resolved_cnt = self
            .clash_items
            .iter()
            .filter(|c| c.status == ClashStatus::Resolved)
            .count();

        // KPI Summary Counters (wraps gracefully on narrow viewports)
        ui.horizontal_wrapped(|ui| {
            let kpis = [
                ("🔴 Hard", format!("{}", hard_active), theme.accent_red),
                ("🟡 Soft/Code", format!("{}", soft_active), theme.accent_orange),
                ("🟢 Resolved", format!("{}", resolved_cnt), theme.accent_green),
            ];
            for (label, val, col) in kpis {
                Frame::new()
                    .fill(theme.bg_card)
                    .corner_radius(CornerRadius::same(6))
                    .inner_margin(Margin::symmetric(8, 4))
                    .stroke(Stroke::new(1.0, theme.border_card))
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(format!("{}: {}", label, val))
                                .size(10.5)
                                .strong()
                                .color(col),
                        );
                    });
            }
        });
        ui.add_space(6.0);

        // Tolerance, Severity Filter & 3D Overlay Controls
        theme.card_frame().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    RichText::new("Tolerance:")
                        .size(10.5)
                        .color(theme.text_secondary),
                );
                ui.add(
                    egui::Slider::new(&mut self.clash_tolerance_mm, 5.0..=100.0)
                        .suffix(" mm")
                        .text(""),
                );
            });
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    RichText::new("Discipline:")
                        .size(10.5)
                        .color(theme.text_secondary),
                );
                egui::ComboBox::from_id_salt("clash_filter_combo")
                    .selected_text(
                        RichText::new(&self.clash_filter_discipline)
                            .size(10.5)
                            .color(theme.text_primary),
                    )
                    .show_ui(ui, |ui| {
                        let opts = [
                            "All (Arch vs Struct vs MEP)",
                            "Architecture vs Structure",
                            "Structure vs HVAC Duct",
                            "Structure vs Piping",
                            "Framing vs Rebar Clearances",
                        ];
                        for opt in opts {
                            let is_sel = self.clash_filter_discipline == opt;
                            if ui.selectable_label(is_sel, opt).clicked() {
                                self.clash_filter_discipline = opt.to_string();
                            }
                        }
                    });
            });
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                let sev_opts: [(Option<ClashSeverity>, &str); 4] = [
                    (None, "All"),
                    (Some(ClashSeverity::Hard), "🔴 Hard"),
                    (Some(ClashSeverity::Soft), "🟡 Soft"),
                    (Some(ClashSeverity::CodeTolerance), "🟠 Code"),
                ];
                for (opt, lbl) in sev_opts {
                    if ui
                        .selectable_label(self.clash_filter_severity == opt, lbl)
                        .clicked()
                    {
                        self.clash_filter_severity = opt;
                    }
                }
            });
            ui.add_space(4.0);
            ui.checkbox(
                &mut self.show_3d_clash_markers,
                "📍 Highlight Clash Boxes & Pins in 3D Viewport",
            );
            ui.add_space(6.0);
            if ui
                .add(
                    egui::Button::new(
                        RichText::new("🔄 Re-run 3D AABB Collision Solver")
                            .size(11.0)
                            .strong()
                            .color(Color32::WHITE),
                    )
                    .fill(theme.accent_blue)
                    .corner_radius(CornerRadius::same(6))
                    .min_size(Vec2::new(ui.available_width(), 26.0)),
                )
                .clicked()
            {
                let stamp = crate::core::web_bridge::unix_timestamp_label();
                self.clash_last_run_label = format!("Verified {} ({:.0} mm)", stamp, self.clash_tolerance_mm);
                self.show_3d_clash_markers = true;
                self.modify_status_msg = Some(format!(
                    "Clash solver executed at tol={:.1} mm: {} Hard, {} Soft/Code active, {} Resolved.",
                    self.clash_tolerance_mm, hard_active, soft_active, resolved_cnt
                ));
                self.bim_console_logs.push(format!(
                    "⚔️ [Clash Solver {}] Evaluated {} zones at tol={:.1}mm -> {} Hard, {} Soft/Code.",
                    stamp,
                    self.clash_items.len(),
                    self.clash_tolerance_mm,
                    hard_active,
                    soft_active
                ));
            }
            ui.label(
                RichText::new(format!("Last audit: {}", self.clash_last_run_label))
                    .size(9.5)
                    .color(theme.text_muted),
            );
        });
        ui.add_space(6.0);

        // Filtered Dynamic Clash List
        let mut focus_target: Option<(usize, String, [f32; 3])> = None;
        let mut log_bcf_for: Option<(usize, String, String, [f32; 3], ClashSeverity)> = None;
        let mut status_toggle_idx: Option<usize> = None;

        egui::ScrollArea::vertical()
            .id_salt("clash_items_scroll")
            .show(ui, |ui| {
                let mut visible_count = 0usize;
                for (idx, item) in self.clash_items.iter().enumerate() {
                    if !disc_matches(&item.discipline_pair, &self.clash_filter_discipline) {
                        continue;
                    }
                    if let Some(sev_f) = self.clash_filter_severity {
                        if item.severity != sev_f {
                            continue;
                        }
                    }
                    if item.status != ClashStatus::Resolved
                        && !item.is_triggered_at_tolerance(self.clash_tolerance_mm)
                    {
                        continue;
                    }
                    visible_count += 1;

                    let is_selected = self.selected_clash_id == Some(item.id);
                    let border_col = if is_selected {
                        theme.accent_blue
                    } else {
                        theme.border_card
                    };
                    let (sr, sg, sb) = item.severity.color_rgb();
                    let sev_col = if item.status == ClashStatus::Resolved {
                        theme.accent_green
                    } else {
                        Color32::from_rgb(sr, sg, sb)
                    };
                    let (str_r, str_g, str_b) = item.status.color_rgb();
                    let st_col = Color32::from_rgb(str_r, str_g, str_b);

                    Frame::new()
                        .fill(if is_selected {
                            theme.bg_card_hover
                        } else {
                            theme.bg_card
                        })
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(Margin::same(10))
                        .stroke(Stroke::new(if is_selected { 1.5 } else { 1.0 }, border_col))
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.label(
                                    RichText::new(&item.tag)
                                        .strong()
                                        .size(11.0)
                                        .color(theme.text_primary),
                                );
                                ui.label(
                                    RichText::new(item.severity.badge())
                                        .size(10.0)
                                        .strong()
                                        .color(sev_col),
                                );
                                Frame::new()
                                    .fill(st_col.gamma_multiply(0.18))
                                    .corner_radius(CornerRadius::same(4))
                                    .inner_margin(Margin::symmetric(5, 1))
                                    .show(ui, |ui| {
                                        ui.label(
                                            RichText::new(item.status.label())
                                                .size(9.5)
                                                .strong()
                                                .color(st_col),
                                        );
                                    });
                            });

                            ui.label(
                                RichText::new(format!("{} ∩ {}", item.element_a, item.element_b))
                                    .size(10.5)
                                    .strong()
                                    .color(theme.text_secondary),
                            );
                            ui.label(
                                RichText::new(format!(
                                    "{} • Vol {:.3} m³ • {}",
                                    item.metric_summary(),
                                    item.clash_volume_m3(),
                                    item.discipline_pair
                                ))
                                .size(9.5)
                                .color(sev_col),
                            );
                            ui.label(
                                RichText::new(&item.note)
                                    .size(9.5)
                                    .color(theme.text_muted),
                            );
                            ui.add_space(4.0);

                            ui.horizontal_wrapped(|ui| {
                                if ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new("👁️ Focus 3D")
                                                .size(9.5)
                                                .color(theme.accent_blue),
                                        )
                                        .fill(theme.bg_canvas)
                                        .corner_radius(CornerRadius::same(4)),
                                    )
                                    .clicked()
                                {
                                    focus_target =
                                        Some((item.id, item.tag.clone(), item.point_3d));
                                }

                                if ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new("💬 Log BCF")
                                                .size(9.5)
                                                .color(theme.text_primary),
                                        )
                                        .fill(theme.bg_canvas)
                                        .corner_radius(CornerRadius::same(4)),
                                    )
                                    .clicked()
                                {
                                    log_bcf_for = Some((
                                        item.id,
                                        item.tag.clone(),
                                        format!("{} ∩ {}", item.element_a, item.element_b),
                                        item.point_3d,
                                        item.severity,
                                    ));
                                }

                                if ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new(format!(
                                                "🔄 {}",
                                                item.status.next().label()
                                            ))
                                            .size(9.5)
                                            .color(theme.accent_green),
                                        )
                                        .fill(theme.bg_canvas)
                                        .corner_radius(CornerRadius::same(4)),
                                    )
                                    .clicked()
                                {
                                    status_toggle_idx = Some(idx);
                                }
                            });
                        });
                    ui.add_space(4.0);
                }

                if visible_count == 0 {
                    theme.card_frame().show(ui, |ui| {
                        ui.label(
                            RichText::new("No clashes match current discipline and tolerance filter.")
                                .size(10.5)
                                .color(theme.text_muted),
                        );
                    });
                }
            });

        if let Some((cid, tag, pt)) = focus_target {
            self.selected_clash_id = Some(cid);
            self.show_3d_clash_markers = true;
            self.cam_zoom = 2.25;
            self.cam_yaw = (pt[0].atan2(pt[2].max(0.5))).clamp(-2.8, 2.8);
            self.cam_pitch = 0.42;
            self.modify_status_msg = Some(format!(
                "Focused 3D camera on {} at ({:.1}, {:.1}, {:.1})m",
                tag, pt[0], pt[1], pt[2]
            ));
        }

        if let Some(idx) = status_toggle_idx {
            if let Some(c) = self.clash_items.get_mut(idx) {
                c.status = c.status.next();
                self.modify_status_msg =
                    Some(format!("{} marked as '{}'.", c.tag, c.status.label()));
            }
        }

        if let Some((cid, ctag, summary, pt, sev)) = log_bcf_for {
            let next_id = self.bcf_topics.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            let bcf_tag = format!("BCF-{:03}", 100 + next_id);
            let prio = match sev {
                ClashSeverity::Hard => BcfPriority::Critical,
                ClashSeverity::CodeTolerance => BcfPriority::High,
                ClashSeverity::Soft => BcfPriority::Medium,
            };
            self.bcf_topics.insert(
                0,
                BcfTopicRecord {
                    id: next_id,
                    guid: format!("8a9e4f12-21b0-4e8f-b901-bcf21{:07}", next_id),
                    tag: bcf_tag.clone(),
                    title: format!("Resolve {}: {}", ctag, summary),
                    description: format!(
                        "Auto-logged from 3D Clash Matrix at coordinates ({:.2}, {:.2}, {:.2})m.",
                        pt[0], pt[1], pt[2]
                    ),
                    priority: prio,
                    status: BcfTopicStatus::Open,
                    assignee: "BIM Coordinator".to_string(),
                    due_date: "2026-11-24".to_string(),
                    linked_clash_id: Some(cid),
                    cam_yaw: self.cam_yaw,
                    cam_pitch: self.cam_pitch,
                    cam_zoom: self.cam_zoom,
                    cam_target: pt,
                    comments: Vec::new(),
                },
            );
            self.selected_bcf_topic_id = Some(next_id);
            self.active_sidebar_tab = SidebarTab::BcfIssues;
            self.modify_status_msg = Some(format!("Created {} from {} with 3D viewpoint.", bcf_tag, ctag));
        }
    }


    pub(super) fn render_bcf_tab(&mut self, ui: &mut Ui, theme: &MacTheme) {
        use crate::models::{BcfComment, BcfPriority, BcfTopicRecord, BcfTopicStatus};

        ui.label(
            RichText::new("💬 BCF 2.1 COORDINATION ISSUES & TOPICS")
                .size(11.5)
                .strong()
                .color(theme.text_secondary),
        );
        ui.add_space(4.0);

        let open_cnt = self
            .bcf_topics
            .iter()
            .filter(|t| t.status == BcfTopicStatus::Open)
            .count();
        let inprog_cnt = self
            .bcf_topics
            .iter()
            .filter(|t| t.status == BcfTopicStatus::InProgress)
            .count();
        let res_cnt = self
            .bcf_topics
            .iter()
            .filter(|t| t.status == BcfTopicStatus::Resolved)
            .count();

        ui.horizontal_wrapped(|ui| {
            let new_btn_label = if self.show_new_bcf_form {
                "✕ Cancel Draft"
            } else {
                "➕ New Topic"
            };
            if ui
                .add(
                    egui::Button::new(RichText::new(new_btn_label).size(10.5).color(Color32::WHITE))
                        .fill(if self.show_new_bcf_form {
                            theme.accent_orange
                        } else {
                            theme.accent_blue
                        })
                        .corner_radius(CornerRadius::same(5)),
                )
                .clicked()
            {
                self.show_new_bcf_form = !self.show_new_bcf_form;
            }

            if ui
                .add(
                    egui::Button::new(
                        RichText::new("📬 Export BCF 2.1 XML")
                            .size(10.5)
                            .color(theme.text_primary),
                    )
                    .fill(theme.bg_card)
                    .corner_radius(CornerRadius::same(5)),
                )
                .clicked()
            {
                let xml = BcfTopicRecord::export_bcf_xml(
                    &self.bcf_topics,
                    &self.bim_bus.project.project_name,
                );
                let res = crate::core::DiskExporter::write_export_file(
                    "EigenGraph_BCF21_Topics.xml",
                    &xml,
                );
                let msg = match res {
                    Ok(loc) => format!(
                        "Exported {} BCF 2.1 topics & camera viewpoints ({})",
                        self.bcf_topics.len(),
                        loc
                    ),
                    Err(err) => format!("BCF 2.1 XML prepared ({} bytes): {}", xml.len(), err),
                };
                self.modify_status_msg = Some(msg.clone());
                self.bim_console_logs.push(format!("📦 [BCF 2.1] {}", msg));
            }
        });

        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new(format!(
                    "Open: {} • In Progress: {} • Resolved: {}",
                    open_cnt, inprog_cnt, res_cnt
                ))
                .size(10.0)
                .color(theme.text_muted),
            );
        });
        ui.add_space(6.0);

        if self.show_new_bcf_form {
            theme.card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("📸 CREATE BCF 2.1 TOPIC + 3D VIEWPOINT")
                        .size(10.5)
                        .strong()
                        .color(theme.accent_blue),
                );
                ui.add_space(4.0);
                ui.label(RichText::new("Title:").size(9.5).color(theme.text_muted));
                ui.add(
                    egui::TextEdit::singleline(&mut self.new_bcf_title)
                        .hint_text("e.g., Sleeve required at Grid C-2 Slab")
                        .desired_width(f32::INFINITY),
                );
                ui.add_space(3.0);
                ui.label(RichText::new("Assignee:").size(9.5).color(theme.text_muted));
                ui.add(
                    egui::TextEdit::singleline(&mut self.new_bcf_assignee)
                        .desired_width(f32::INFINITY),
                );
                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Priority:").size(9.5).color(theme.text_muted));
                    for prio in [
                        BcfPriority::Critical,
                        BcfPriority::High,
                        BcfPriority::Medium,
                        BcfPriority::Low,
                    ] {
                        if ui
                            .selectable_label(self.new_bcf_priority == prio, prio.label())
                            .clicked()
                        {
                            self.new_bcf_priority = prio;
                        }
                    }
                });
                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!(
                        "Viewpoint: Yaw {:.0}° | Pitch {:.0}° | Zoom {:.2}x",
                        self.cam_yaw.to_degrees(),
                        self.cam_pitch.to_degrees(),
                        self.cam_zoom
                    ))
                    .size(9.5)
                    .color(theme.text_secondary),
                );
                ui.add_space(4.0);
                if ui
                    .add(
                        egui::Button::new(
                            RichText::new("✅ Create Topic")
                                .size(10.5)
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .fill(theme.accent_green)
                        .corner_radius(CornerRadius::same(5))
                        .min_size(Vec2::new(ui.available_width(), 24.0)),
                    )
                    .clicked()
                {
                    let next_id = self.bcf_topics.iter().map(|t| t.id).max().unwrap_or(0) + 1;
                    let tag = format!("BCF-{:03}", 100 + next_id);
                    let title = if self.new_bcf_title.trim().is_empty() {
                        format!("Field Coordination Observation #{}", next_id)
                    } else {
                        self.new_bcf_title.trim().to_string()
                    };
                    let target = self
                        .selected_clash_id
                        .and_then(|cid| self.clash_items.iter().find(|c| c.id == cid))
                        .map(|c| c.point_3d)
                        .unwrap_or([6.0, 3.2, 4.5]);
                    self.bcf_topics.insert(
                        0,
                        BcfTopicRecord {
                            id: next_id,
                            guid: format!("8a9e4f12-21b0-4e8f-b901-bcf21{:07}", next_id),
                            tag: tag.clone(),
                            title,
                            description: "Captured directly from active 3D viewport perspective."
                                .to_string(),
                            priority: self.new_bcf_priority,
                            status: BcfTopicStatus::Open,
                            assignee: if self.new_bcf_assignee.trim().is_empty() {
                                "BIM Coordinator".to_string()
                            } else {
                                self.new_bcf_assignee.trim().to_string()
                            },
                            due_date: "2026-11-28".to_string(),
                            linked_clash_id: self.selected_clash_id,
                            cam_yaw: self.cam_yaw,
                            cam_pitch: self.cam_pitch,
                            cam_zoom: self.cam_zoom,
                            cam_target: target,
                            comments: Vec::new(),
                        },
                    );
                    self.selected_bcf_topic_id = Some(next_id);
                    self.show_new_bcf_form = false;
                    self.new_bcf_title.clear();
                    self.modify_status_msg = Some(format!(
                        "Created {} with captured 3D camera viewpoint.",
                        tag
                    ));
                    self.bim_console_logs
                        .push(format!("💬 [BCF 2.1] Created topic {} with 3D snapshot.", tag));
                }
            });
            ui.add_space(6.0);
        }

        let mut restore_cam: Option<(String, f32, f32, f32, Option<usize>)> = None;
        let mut cycle_status_idx: Option<usize> = None;
        let mut select_topic_id: Option<Option<usize>> = None;
        let mut post_comment_idx: Option<usize> = None;

        egui::ScrollArea::vertical()
            .id_salt("bcf_topics_scroll")
            .show(ui, |ui| {
                for (idx, topic) in self.bcf_topics.iter().enumerate() {
                    let is_expanded = self.selected_bcf_topic_id == Some(topic.id);
                    let (pr, pg, pb) = topic.priority.color_rgb();
                    let pcol = Color32::from_rgb(pr, pg, pb);
                    let (sr, sg, sb) = topic.status.color_rgb();
                    let scol = Color32::from_rgb(sr, sg, sb);

                    Frame::new()
                        .fill(if is_expanded {
                            theme.bg_card_hover
                        } else {
                            theme.bg_card
                        })
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(Margin::same(10))
                        .stroke(Stroke::new(
                            if is_expanded { 1.4 } else { 1.0 },
                            if is_expanded {
                                theme.accent_blue
                            } else {
                                theme.border_card
                            },
                        ))
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.label(
                                    RichText::new(&topic.tag)
                                        .strong()
                                        .size(11.0)
                                        .color(theme.accent_blue),
                                );
                                ui.label(
                                    RichText::new(topic.priority.label())
                                        .size(10.0)
                                        .strong()
                                        .color(pcol),
                                );
                                if ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new(topic.status.label())
                                                .size(9.5)
                                                .strong()
                                                .color(scol),
                                        )
                                        .fill(scol.gamma_multiply(0.18))
                                        .corner_radius(CornerRadius::same(4)),
                                    )
                                    .on_hover_text("Click to advance workflow status")
                                    .clicked()
                                {
                                    cycle_status_idx = Some(idx);
                                }
                            });

                            ui.label(
                                RichText::new(&topic.title)
                                    .size(10.5)
                                    .strong()
                                    .color(theme.text_primary),
                            );
                            ui.horizontal_wrapped(|ui| {
                                ui.label(
                                    RichText::new(format!("👤 {}", topic.assignee))
                                        .size(9.5)
                                        .color(theme.text_muted),
                                );
                                ui.label(
                                    RichText::new(format!("📅 {}", topic.due_date))
                                        .size(9.5)
                                        .color(theme.text_muted),
                                );
                            });
                            ui.add_space(4.0);

                            ui.horizontal_wrapped(|ui| {
                                if ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new("📷 Restore Viewpoint")
                                                .size(9.5)
                                                .color(theme.accent_blue),
                                        )
                                        .fill(theme.bg_canvas)
                                        .corner_radius(CornerRadius::same(4)),
                                    )
                                    .clicked()
                                {
                                    restore_cam = Some((
                                        topic.tag.clone(),
                                        topic.cam_yaw,
                                        topic.cam_pitch,
                                        topic.cam_zoom,
                                        topic.linked_clash_id,
                                    ));
                                }

                                let thread_lbl = if is_expanded {
                                    format!("🔽 Hide Thread ({})", topic.comments.len())
                                } else {
                                    format!("💬 Thread ({})", topic.comments.len())
                                };
                                if ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new(thread_lbl)
                                                .size(9.5)
                                                .color(theme.text_secondary),
                                        )
                                        .fill(theme.bg_canvas)
                                        .corner_radius(CornerRadius::same(4)),
                                    )
                                    .clicked()
                                {
                                    select_topic_id = Some(if is_expanded {
                                        None
                                    } else {
                                        Some(topic.id)
                                    });
                                }
                            });

                            if is_expanded {
                                ui.add_space(6.0);
                                ui.separator();
                                ui.add_space(4.0);
                                ui.label(
                                    RichText::new(&topic.description)
                                        .size(9.5)
                                        .color(theme.text_secondary),
                                );
                                ui.label(
                                    RichText::new(format!(
                                        "Camera: Yaw {:.0}° | Pitch {:.0}° | Zoom {:.2}x",
                                        topic.cam_yaw.to_degrees(),
                                        topic.cam_pitch.to_degrees(),
                                        topic.cam_zoom
                                    ))
                                    .size(9.0)
                                    .color(theme.text_muted),
                                );
                                ui.add_space(4.0);

                                for c in &topic.comments {
                                    Frame::new()
                                        .fill(theme.bg_canvas)
                                        .corner_radius(CornerRadius::same(5))
                                        .inner_margin(Margin::same(6))
                                        .show(ui, |ui| {
                                            ui.label(
                                                RichText::new(format!("{} • {}", c.author, c.timestamp))
                                                    .size(9.0)
                                                    .strong()
                                                    .color(theme.accent_blue),
                                            );
                                            ui.label(
                                                RichText::new(&c.comment)
                                                    .size(9.5)
                                                    .color(theme.text_primary),
                                            );
                                        });
                                    ui.add_space(3.0);
                                }

                                ui.horizontal(|ui| {
                                    let field_w = (ui.available_width() - 62.0).max(90.0);
                                    ui.add(
                                        egui::TextEdit::singleline(&mut self.bcf_comment_draft)
                                            .hint_text("Add coordination comment...")
                                            .desired_width(field_w),
                                    );
                                    if ui
                                        .add(
                                            egui::Button::new(
                                                RichText::new("Post")
                                                    .size(9.5)
                                                    .strong()
                                                    .color(Color32::WHITE),
                                            )
                                            .fill(theme.accent_blue)
                                            .corner_radius(CornerRadius::same(4)),
                                        )
                                        .clicked()
                                        && !self.bcf_comment_draft.trim().is_empty()
                                    {
                                        post_comment_idx = Some(idx);
                                    }
                                });
                            }
                        });
                    ui.add_space(4.0);
                }
            });

        if let Some((tag, yaw, pitch, zoom, clash_id)) = restore_cam {
            self.cam_yaw = yaw;
            self.cam_pitch = pitch;
            self.cam_zoom = zoom;
            if clash_id.is_some() {
                self.selected_clash_id = clash_id;
                self.show_3d_clash_markers = true;
            }
            self.modify_status_msg = Some(format!("Restored 3D viewpoint for {}", tag));
        }

        if let Some(idx) = cycle_status_idx {
            if let Some(t) = self.bcf_topics.get_mut(idx) {
                t.status = t.status.next();
                self.modify_status_msg =
                    Some(format!("{} status updated to '{}'.", t.tag, t.status.label()));
            }
        }

        if let Some(sel) = select_topic_id {
            self.selected_bcf_topic_id = sel;
        }

        if let Some(idx) = post_comment_idx {
            let reply = self.bcf_comment_draft.trim().to_string();
            if let Some(t) = self.bcf_topics.get_mut(idx) {
                let cid = t.comments.iter().map(|c| c.id).max().unwrap_or(0) + 1;
                t.comments.push(BcfComment {
                    id: cid,
                    author: "Lead Structural Engineer".to_string(),
                    timestamp: crate::core::web_bridge::unix_timestamp_label(),
                    comment: reply,
                });
                self.modify_status_msg = Some(format!("Added coordination note to {}", t.tag));
            }
            self.bcf_comment_draft.clear();
        }
    }


}
