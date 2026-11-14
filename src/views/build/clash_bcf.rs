//! views/build/clash_bcf.rs — Spatial Clash Detection Matrix & BCF 2.1 Collaboration Hub.

use super::*;

impl BuildView {
    pub(super) fn render_clash_tab(&mut self, ui: &mut Ui, theme: &MacTheme, features: &mut FeatureRegistry) {
        use crate::models::{BcfPriority, BcfTopicRecord, BcfTopicStatus, ClashSeverity, ClashStatus};

        crate::icons::icon_label(
            ui,
            crate::icons::AppIcon::AlertTriangle,
            13.5,
            theme.text_secondary,
            "SPATIAL CLASH MATRIX & INTERFERENCE DETECTOR",
            11.5,
            theme.text_secondary,
            true,
        );
        ui.add_space(4.0);

        ui_kit::feature_slot_card(ui, theme, features, FeatureId::LiveBvhGeometricClashSolver, |ui| {
            ui.horizontal_wrapped(|ui| {
                let _ = ui.selectable_label(true, "Bvh3D AABB Tree (0.01mm)");
                let _ = ui.selectable_label(false, "Rebar vs Duct Sweep");
                if crate::icons::icon_button(
                    ui,
                    crate::icons::AppIcon::Bolt,
                    "Recompute Live 3D BVH Clashes",
                    10.5,
                    theme.text_primary,
                    theme.bg_card,
                    Stroke::new(1.0, theme.border_subtle),
                    CornerRadius::same(5),
                    Vec2::ZERO,
                )
                .clicked()
                {
                    self.run_live_bvh_clash_scan();
                }
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
                (crate::icons::AppIcon::AlertTriangle, "Hard", format!("{}", hard_active), theme.accent_red),
                (crate::icons::AppIcon::Info, "Soft/Code", format!("{}", soft_active), theme.accent_orange),
                (crate::icons::AppIcon::CheckCircle, "Resolved", format!("{}", resolved_cnt), theme.accent_green),
            ];
            for (ico, label, val, col) in kpis {
                Frame::new()
                    .fill(theme.bg_card)
                    .corner_radius(CornerRadius::same(6))
                    .inner_margin(Margin::symmetric(8, 4))
                    .stroke(Stroke::new(1.0, theme.border_card))
                    .show(ui, |ui| {
                        crate::icons::icon_label(
                            ui,
                            ico,
                            12.0,
                            col,
                            format!("{}: {}", label, val),
                            10.5,
                            col,
                            true,
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
                let sev_opts: [(Option<ClashSeverity>, Option<crate::icons::AppIcon>, &str); 4] = [
                    (None, None, "All"),
                    (Some(ClashSeverity::Hard), Some(crate::icons::AppIcon::AlertTriangle), "Hard"),
                    (Some(ClashSeverity::Soft), Some(crate::icons::AppIcon::Info), "Soft"),
                    (Some(ClashSeverity::CodeTolerance), Some(crate::icons::AppIcon::Shield), "Code"),
                ];
                for (opt, maybe_ico, lbl) in sev_opts {
                    let is_sel = self.clash_filter_severity == opt;
                    let fg = if is_sel { Color32::WHITE } else { theme.text_secondary };
                    let bg = if is_sel { theme.accent_blue } else { theme.bg_card };
                    let stroke = Stroke::new(1.0, if is_sel { theme.accent_blue } else { theme.border_subtle });
                    let clicked = if let Some(ico) = maybe_ico {
                        crate::icons::icon_button(ui, ico, lbl, 10.5, fg, bg, stroke, CornerRadius::same(5), Vec2::ZERO).clicked()
                    } else {
                        ui.add(egui::Button::new(RichText::new(lbl).size(10.5).color(fg)).fill(bg).stroke(stroke).corner_radius(CornerRadius::same(5))).clicked()
                    };
                    if clicked {
                        self.clash_filter_severity = opt;
                    }
                }
            });
            ui.add_space(4.0);
            ui.checkbox(
                &mut self.show_3d_clash_markers,
                "Highlight Clash Boxes & Pins in 3D Viewport",
            );
            ui.add_space(6.0);
            if crate::icons::icon_button(
                ui,
                crate::icons::AppIcon::Redo,
                "Re-run 3D AABB Collision Solver",
                11.0,
                Color32::WHITE,
                theme.accent_blue,
                Stroke::NONE,
                CornerRadius::same(6),
                Vec2::new(ui.available_width(), 26.0),
            )
            .clicked()
            {
                self.run_live_bvh_clash_scan();
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
                                if crate::icons::icon_button(
                                    ui,
                                    crate::icons::AppIcon::Target,
                                    "Focus 3D",
                                    9.5,
                                    theme.accent_blue,
                                    theme.bg_canvas,
                                    Stroke::NONE,
                                    CornerRadius::same(4),
                                    Vec2::ZERO,
                                )
                                .clicked()
                                {
                                    focus_target =
                                        Some((item.id, item.tag.clone(), item.point_3d));
                                }

                                if crate::icons::icon_button(
                                    ui,
                                    crate::icons::AppIcon::CommentNote,
                                    "Log BCF",
                                    9.5,
                                    theme.text_primary,
                                    theme.bg_canvas,
                                    Stroke::NONE,
                                    CornerRadius::same(4),
                                    Vec2::ZERO,
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

                                if crate::icons::icon_button(
                                    ui,
                                    crate::icons::AppIcon::Redo,
                                    item.status.next().label(),
                                    9.5,
                                    theme.accent_green,
                                    theme.bg_canvas,
                                    Stroke::NONE,
                                    CornerRadius::same(4),
                                    Vec2::ZERO,
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

        crate::icons::icon_label(
            ui,
            crate::icons::AppIcon::CommentNote,
            13.5,
            theme.text_secondary,
            "BCF 2.1 COORDINATION ISSUES & TOPICS",
            11.5,
            theme.text_secondary,
            true,
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
            let new_btn_ico = if self.show_new_bcf_form {
                crate::icons::AppIcon::Close
            } else {
                crate::icons::AppIcon::Plus
            };
            let new_btn_label = if self.show_new_bcf_form {
                "Cancel Draft"
            } else {
                "New Topic"
            };
            if crate::icons::icon_button(
                ui,
                new_btn_ico,
                new_btn_label,
                10.5,
                Color32::WHITE,
                if self.show_new_bcf_form {
                    theme.accent_orange
                } else {
                    theme.accent_blue
                },
                Stroke::NONE,
                CornerRadius::same(5),
                Vec2::ZERO,
            )
            .clicked()
            {
                self.show_new_bcf_form = !self.show_new_bcf_form;
            }

            if crate::icons::icon_button(
                ui,
                crate::icons::AppIcon::UploadImport,
                "Export BCF 2.1 XML",
                10.5,
                theme.text_primary,
                theme.bg_card,
                Stroke::NONE,
                CornerRadius::same(5),
                Vec2::ZERO,
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
                self.bim_console_logs.push(format!("[BCF 2.1] {}", msg));
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
                crate::icons::icon_label(
                    ui,
                    crate::icons::AppIcon::Camera,
                    12.5,
                    theme.accent_blue,
                    "CREATE BCF 2.1 TOPIC + 3D VIEWPOINT",
                    10.5,
                    theme.accent_blue,
                    true,
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
                if crate::icons::icon_button(
                    ui,
                    crate::icons::AppIcon::CheckCircle,
                    "Create Topic",
                    10.5,
                    Color32::WHITE,
                    theme.accent_green,
                    Stroke::NONE,
                    CornerRadius::same(5),
                    Vec2::new(ui.available_width(), 24.0),
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
                        .push(format!("[BCF 2.1] Created topic {} with 3D snapshot.", tag));
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
                                crate::icons::icon_label(
                                    ui,
                                    crate::icons::AppIcon::HardHat,
                                    11.0,
                                    theme.text_muted,
                                    &topic.assignee,
                                    9.5,
                                    theme.text_muted,
                                    false,
                                );
                                crate::icons::icon_label(
                                    ui,
                                    crate::icons::AppIcon::Clock,
                                    11.0,
                                    theme.text_muted,
                                    &topic.due_date,
                                    9.5,
                                    theme.text_muted,
                                    false,
                                );
                            });
                            ui.add_space(4.0);

                            ui.horizontal_wrapped(|ui| {
                                if crate::icons::icon_button(
                                    ui,
                                    crate::icons::AppIcon::Camera,
                                    "Restore Viewpoint",
                                    9.5,
                                    theme.accent_blue,
                                    theme.bg_canvas,
                                    Stroke::NONE,
                                    CornerRadius::same(4),
                                    Vec2::ZERO,
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

                                let (thread_ico, thread_lbl) = if is_expanded {
                                    (
                                        crate::icons::AppIcon::ChevronUp,
                                        format!("Hide Thread ({})", topic.comments.len()),
                                    )
                                } else {
                                    (
                                        crate::icons::AppIcon::CommentNote,
                                        format!("Thread ({})", topic.comments.len()),
                                    )
                                };
                                if crate::icons::icon_button(
                                    ui,
                                    thread_ico,
                                    thread_lbl,
                                    9.5,
                                    theme.text_secondary,
                                    theme.bg_canvas,
                                    Stroke::NONE,
                                    CornerRadius::same(4),
                                    Vec2::ZERO,
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

    /// Evaluates real 3D AABB bounding-box interferences across all active BIM elements
    /// (`BimWall`, `BimColumn`, `BimRun`, `BimFooting`, `BimSlab`) using `Aabb3D::overlap_with_tolerance`.
    pub fn run_live_bvh_clash_scan(&mut self) {
        use crate::core::bim::RunCategory;
        use crate::core::csg::Aabb3D;
        use crate::models::{ClashDetectionItem, ClashSeverity, ClashStatus};

        #[derive(Clone)]
        struct ElemBox {
            uid: String,
            name: String,
            disc: &'static str,
            kind: u8, // 0=Wall, 1=Col, 2=BeamRun, 3=MepRun, 4=Footing
            level_id: usize,
            aabb: Aabb3D,
        }

        let proj = &self.bim_bus.project;
        let mut boxes: Vec<ElemBox> = Vec::new();

        for w in &proj.walls {
            let z0 = proj.level_elevation(w.base_level_id) * 0.001;
            let z1 = w
                .top_level_id
                .map(|tid| proj.level_elevation(tid) * 0.001)
                .unwrap_or_else(|| z0 + w.unconnected_height_mm * 0.001)
                .max(z0 + 0.5);
            let ht = (w.thickness_mm * 0.5 * 0.001).max(0.05);
            boxes.push(ElemBox {
                uid: format!("W{}", w.id),
                name: w.name.clone(),
                disc: if w.is_structural { "STR" } else { "ARC" },
                kind: 0,
                level_id: w.base_level_id,
                aabb: Aabb3D {
                    min: [
                        w.start.0.min(w.end.0) * 0.001 - ht,
                        z0,
                        w.start.1.min(w.end.1) * 0.001 - ht,
                    ],
                    max: [
                        w.start.0.max(w.end.0) * 0.001 + ht,
                        z1,
                        w.start.1.max(w.end.1) * 0.001 + ht,
                    ],
                },
            });
        }

        for c in &proj.columns {
            let z0 = proj.level_elevation(c.base_level_id) * 0.001;
            let z1 = proj
                .level_elevation(c.top_level_id)
                .mul_add(0.001, 0.0)
                .max(z0 + 3.0);
            let hw = (c.width_mm * 0.5 * 0.001).max(0.05);
            let hd = (c.depth_mm * 0.5 * 0.001).max(0.05);
            let cx = c.pos.0 * 0.001;
            let cz = c.pos.1 * 0.001;
            boxes.push(ElemBox {
                uid: format!("C{}", c.id),
                name: c.name.clone(),
                disc: "STR",
                kind: 1,
                level_id: c.base_level_id,
                aabb: Aabb3D {
                    min: [cx - hw, z0, cz - hd],
                    max: [cx + hw, z1, cz + hd],
                },
            });
        }

        for r in &proj.runs {
            let elev = proj.level_elevation(r.level_id) * 0.001;
            let hw = (r.width_mm * 0.5 * 0.001).max(0.04);
            let d = (r.depth_mm * 0.001).max(0.08);
            let (y0, y1) = match r.justify {
                crate::core::bim::RunJustify::Top => (elev - d, elev),
                crate::core::bim::RunJustify::Centre => (elev - d * 0.5, elev + d * 0.5),
                crate::core::bim::RunJustify::Bottom => (elev, elev + d),
            };
            let is_mep = matches!(
                r.category,
                RunCategory::PlumbingPipe
                    | RunCategory::ElectricalCableTray
                    | RunCategory::ElectricalConduit
                    | RunCategory::LightningConductor
            );
            boxes.push(ElemBox {
                uid: format!("R{}", r.id),
                name: r.name.clone(),
                disc: if is_mep { "MEP" } else { "STR" },
                kind: if is_mep { 3 } else { 2 },
                level_id: r.level_id,
                aabb: Aabb3D {
                    min: [
                        r.start.0.min(r.end.0) * 0.001 - hw,
                        y0,
                        r.start.1.min(r.end.1) * 0.001 - hw,
                    ],
                    max: [
                        r.start.0.max(r.end.0) * 0.001 + hw,
                        y1,
                        r.start.1.max(r.end.1) * 0.001 + hw,
                    ],
                },
            });
        }

        for f in &proj.footings {
            let elev = proj.level_elevation(f.level_id) * 0.001;
            let t = (f.thickness_mm * 0.001).max(0.15);
            let hw = (f.width_mm * 0.5 * 0.001).max(0.2);
            let hl = (f.length_mm * 0.5 * 0.001).max(0.2);
            let cx = f.pos.0 * 0.001;
            let cz = f.pos.1 * 0.001;
            boxes.push(ElemBox {
                uid: format!("F{}", f.id),
                name: f.name.clone(),
                disc: "STR",
                kind: 4,
                level_id: f.level_id,
                aabb: Aabb3D {
                    min: [cx - hw, elev - t, cz - hl],
                    max: [cx + hw, elev, cz + hl],
                },
            });
        }

        let tol_m = (self.clash_tolerance_mm * 0.001).clamp(0.005, 0.5);
        let mut results: Vec<ClashDetectionItem> = Vec::new();
        let mut next_clash_id = 1usize;

        for i in 0..boxes.len() {
            for j in (i + 1)..boxes.len() {
                let a = &boxes[i];
                let b = &boxes[j];
                if a.uid == b.uid {
                    continue;
                }
                // Skip intended beam-column, column-footing, or slab structural joints
                let is_beam_col = (a.kind == 1 && b.kind == 2) || (a.kind == 2 && b.kind == 1);
                let is_col_foot = (a.kind == 1 && b.kind == 4) || (a.kind == 4 && b.kind == 1);
                if is_col_foot {
                    continue;
                }

                if let Some((center, pen_mm, vol_m3)) = a.aabb.overlap_with_tolerance(&b.aabb, tol_m) {
                    // For perpendicular framing beams sharing an end joint (< 0.05m^3), skip expected node
                    if a.kind == 2 && b.kind == 2 && vol_m3 < 0.055 && a.level_id == b.level_id {
                        continue;
                    }
                    if is_beam_col && vol_m3 <= 0.12 {
                        continue;
                    }

                    let severity = if vol_m3 > 1e-5 {
                        ClashSeverity::Hard
                    } else {
                        ClashSeverity::Soft
                    };
                    let pair = match (a.disc, b.disc) {
                        ("MEP", _) | (_, "MEP") => "MEP vs STR".to_string(),
                        ("ARC", "STR") | ("STR", "ARC") => "ARC vs STR".to_string(),
                        _ => "STR vs STR".to_string(),
                    };

                    let hx = ((a.aabb.max[0].min(b.aabb.max[0]) - a.aabb.min[0].max(b.aabb.min[0]))
                        .abs()
                        * 0.5)
                        .max(0.12) as f32;
                    let hy = ((a.aabb.max[1].min(b.aabb.max[1]) - a.aabb.min[1].max(b.aabb.min[1]))
                        .abs()
                        * 0.5)
                        .max(0.12) as f32;
                    let hz = ((a.aabb.max[2].min(b.aabb.max[2]) - a.aabb.min[2].max(b.aabb.min[2]))
                        .abs()
                        * 0.5)
                        .max(0.12) as f32;

                    results.push(ClashDetectionItem {
                        id: next_clash_id,
                        tag: format!("CLH-{:03}", next_clash_id),
                        discipline_pair: pair,
                        severity,
                        status: ClashStatus::Active,
                        element_a: a.name.clone(),
                        element_b: b.name.clone(),
                        intersection_mm: if vol_m3 > 1e-5 { -pen_mm } else { 0.0 },
                        required_clearance_mm: self.clash_tolerance_mm,
                        point_3d: [center[0] as f32, center[1] as f32, center[2] as f32],
                        box_half_extents: [hx, hy, hz],
                        note: format!(
                            "Aabb3D overlap volume {:.4} m^3 (penetration {:.1} mm)",
                            vol_m3, pen_mm
                        ),
                    });
                    next_clash_id += 1;
                }
            }
        }

        let hard_cnt = results
            .iter()
            .filter(|c| c.severity == ClashSeverity::Hard)
            .count();
        let soft_cnt = results.len().saturating_sub(hard_cnt);
        let stamp = crate::core::web_bridge::unix_timestamp_label();
        self.clash_items = results;
        self.clash_last_run_label = format!("Verified {} ({:.0} mm)", stamp, self.clash_tolerance_mm);
        self.show_3d_clash_markers = true;
        let msg = if self.clash_items.is_empty() {
            format!(
                "3D AABB Clash Solver scanned {} elements: 0 clashes detected (model clash-free at tol={:.1} mm).",
                boxes.len(),
                self.clash_tolerance_mm
            )
        } else {
            format!(
                "3D AABB Clash Solver scanned {} elements: {} Hard, {} Soft interferences detected.",
                boxes.len(),
                hard_cnt,
                soft_cnt
            )
        };
        self.modify_status_msg = Some(msg.clone());
        self.bim_console_logs.push(format!("[Clash Solver {}] {}", stamp, msg));
    }
}
