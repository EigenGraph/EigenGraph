//! lean.rs — Pillar 9: 9D BIM Lean Construction.
//! The optimization of the construction process to minimize material waste, streamline workflow, and maximize production efficiency.
//! Features Dynamic 1D Cutting Stock Optimizer (CSO), Bar Bending Schedule (BBS), and Interactive Last Planner System (LPS).

use egui::{
    Align, Align2, Color32, CornerRadius, Frame, Layout, Margin, Pos2, Rect, RichText, ScrollArea,
    Stroke, StrokeKind, Ui, Vec2,
};
use crate::core::cso::{CuttingStockOptimizer, CsoReport};
use crate::features::{FeatureId, FeatureRegistry};
use crate::models::{LpsConstraintCategory, LpsLookaheadTask, LpsWeeklyPpcRecord, StructuralMember};
use crate::theme::MacTheme;
use crate::ui_kit;
use crate::views::planning::format_idr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeanSubTab {
    CuttingStock,
    LastPlanner,
    ValueStream,
    JitDelivery,
}

pub struct LeanView {
    pub active_subtab: LeanSubTab,
    pub standard_bar_length_m: f32,
    pub saw_kerf_mm: f32,
    pub min_reusable_offcut_m: f32,
    pub steel_price_idr_per_kg: f32,
    pub selected_profile: String,
    pub export_status: Option<String>,
    // Phase 29: Stateful Last Planner System (LPS)
    pub lps_history: Vec<LpsWeeklyPpcRecord>,
    pub lps_tasks: Vec<LpsLookaheadTask>,
    pub lps_week_filter: String,
    pub lps_new_task_name: String,
    pub lps_new_task_week: String,
}

impl Default for LeanView {
    fn default() -> Self {
        Self {
            active_subtab: LeanSubTab::CuttingStock,
            standard_bar_length_m: 12.0,
            saw_kerf_mm: 5.0,
            min_reusable_offcut_m: 1.5,
            steel_price_idr_per_kg: 17500.0,
            selected_profile: "All Profiles".to_string(),
            export_status: None,
            lps_history: LpsWeeklyPpcRecord::default_history(),
            lps_tasks: LpsLookaheadTask::default_tasks(),
            lps_week_filter: "All Weeks".to_string(),
            lps_new_task_name: String::new(),
            lps_new_task_week: "W-06".to_string(),
        }
    }
}

impl LeanView {
    pub fn show(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        members: &[StructuralMember],
        features: &mut FeatureRegistry,
    ) {
        let margin_x = if MacTheme::is_mobile(ui.ctx()) { 12 } else { 20 };
        ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
            Frame::new().inner_margin(Margin::symmetric(margin_x, 12)).show(ui, |ui| {
                // Toast notification if export triggered
                let mut dismiss_toast = false;
                if let Some(ref status) = self.export_status {
                    Frame::new()
                        .fill(theme.bg_card)
                        .corner_radius(CornerRadius::same(8))
                        .stroke(Stroke::new(1.0, theme.accent_green))
                        .inner_margin(Margin::same(10))
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.label(RichText::new("✅").size(14.0));
                                ui.label(RichText::new(status).size(12.0).color(theme.text_primary));
                                if ui.button(RichText::new("Dismiss").size(11.0)).clicked() {
                                    dismiss_toast = true;
                                }
                            });
                        });
                    ui.add_space(8.0);
                }
                if dismiss_toast {
                    self.export_status = None;
                }

                // 1. TOP SUBTABS SELECTOR
                self.render_header_bar(ui, theme);

                ui.add_space(16.0);

                // 2. ACTIVE SUB-TAB CONTENT
                match self.active_subtab {
                    LeanSubTab::CuttingStock => self.render_cutting_stock(ui, theme, members, features),
                    LeanSubTab::LastPlanner => self.render_last_planner(ui, theme),
                    LeanSubTab::ValueStream => self.render_value_stream(ui, theme),
                    LeanSubTab::JitDelivery => self.render_jit_delivery(ui, theme),
                }

                ui.add_space(64.0);
            });
        });
    }

    fn render_header_bar(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let (_, _, live_ppc) = LpsLookaheadTask::compute_live_ppc(&self.lps_tasks);
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(6))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                let lps_label = format!("📋 Last Planner System (PPC {:.1}%)", live_ppc);
                let tabs = [
                    (LeanSubTab::CuttingStock, "✂️ 1D/2D Cutting Stock Optimizer".to_string()),
                    (LeanSubTab::LastPlanner, lps_label),
                    (LeanSubTab::ValueStream, "🌊 Value Stream & 7 Wastes".to_string()),
                    (LeanSubTab::JitDelivery, "🚚 Just-In-Time (JIT) Delivery Buffer".to_string()),
                ];

                ui.horizontal_wrapped(|ui| {
                    for (tab, label) in tabs {
                        let is_active = self.active_subtab == tab;
                        let bg = if is_active { theme.accent_blue } else { Color32::TRANSPARENT };
                        let fg = if is_active { Color32::WHITE } else { theme.text_secondary };

                        if ui
                            .add(
                                egui::Button::new(RichText::new(label).size(12.0).strong().color(fg))
                                    .fill(bg)
                                    .corner_radius(CornerRadius::same(7))
                                    .min_size(Vec2::new(0.0, 30.0)),
                            )
                            .clicked()
                        {
                            self.active_subtab = tab;
                        }
                    }
                });
            });
    }

    fn render_responsive_cards(&self, ui: &mut Ui, theme: &MacTheme, cards: &[(&str, String, String, Color32)]) {
        ui_kit::responsive_grid_n(ui, cards.len(), 200.0, |ui, idx| {
            let (title, val, sub, col) = &cards[idx];
            self.render_stat_card(ui, theme, title, val, sub, *col);
        });
    }

    fn render_cutting_stock(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        members: &[StructuralMember],
        features: &mut FeatureRegistry,
    ) {
        // 1. Dynamic Extraction from active model members
        let items = CuttingStockOptimizer::extract_items_from_members(members);

        // Get unique profiles list
        let mut profiles = vec!["All Profiles".to_string()];
        for it in &items {
            if !profiles.contains(&it.profile) && !it.profile.is_empty() {
                profiles.push(it.profile.clone());
            }
        }

        let filter_opt = if self.selected_profile == "All Profiles" {
            None
        } else {
            Some(self.selected_profile.as_str())
        };

        // 2. Execute Dynamic 1D Optimization
        let report: CsoReport = CuttingStockOptimizer::optimize_1d(
            &items,
            filter_opt,
            self.standard_bar_length_m as f64,
            self.saw_kerf_mm as f64,
            self.min_reusable_offcut_m as f64,
            self.steel_price_idr_per_kg as f64,
        );

        // 3. Top Master Stat Cards
        let yield_color = if report.material_yield_pct >= 95.0 {
            theme.accent_green
        } else if report.material_yield_pct >= 88.0 {
            theme.accent_blue
        } else {
            Color32::from_rgb(239, 68, 68)
        };

        let cards = [
            (
                "MATERIAL YIELD EFFICIENCY",
                format!("{:.1}%", report.material_yield_pct),
                format!("Scrap Waste: {:.1}% ({:.2}m)", report.scrap_waste_pct, report.net_scrap_waste_m),
                yield_color,
            ),
            (
                "ESTIMATED COST SAVINGS",
                format!("Rp {}", format_idr(report.cost_savings_idr)),
                format!("Avoided Scrap: {:.1} kg Steel", (report.cost_savings_idr / self.steel_price_idr_per_kg as f64).max(0.0)),
                theme.accent_blue,
            ),
            (
                "MASTER BAR STOCK LENGTH",
                format!("{:.2} m", report.stock_length_m),
                format!("Kerf: {:.1} mm • Min Offcut: {:.1} m", report.kerf_mm, report.min_reusable_m),
                theme.text_primary,
            ),
            (
                "TOTAL MASTER BARS REQUIRED",
                format!("{} Bars", report.master_bars_used),
                format!("To Cut {} Structural Elements", report.total_pieces_demanded),
                theme.text_primary,
            ),
        ];
        self.render_responsive_cards(ui, theme, &cards);

        ui.add_space(12.0);

        ui_kit::feature_slot_card(ui, theme, features, FeatureId::LiveRebarBbsCuttingSync, |ui| {
            ui.horizontal_wrapped(|ui| {
                let _ = ui.selectable_label(true, "Source: Active BimProject Bar Marks (D16-D25)");
                let _ = ui.selectable_label(false, "12m Stock Splice Coupler Pairing");
                let _ = ui.button("✂️ Optimize BIM Rebar Schedule (1D BFD)");
            });
        });

        ui.add_space(14.0);

        // 4. Parameter Control & Filter Bar
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(14))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("⚙️ Cutting Optimization Parameters").size(13.0).strong().color(theme.text_primary));
                    ui.add_space(12.0);

                    // Profile selector
                    ui.label(RichText::new("Profile Filter:").size(11.5).color(theme.text_secondary));
                    egui::ComboBox::from_id_salt("cso_profile_filter")
                        .selected_text(RichText::new(&self.selected_profile).size(11.5))
                        .width(170.0)
                        .show_ui(ui, |ui| {
                            for p in &profiles {
                                ui.selectable_value(&mut self.selected_profile, p.clone(), p);
                            }
                        });

                    ui.add_space(8.0);
                    ui.label(RichText::new("Stock Bar Length:").size(11.5).color(theme.text_secondary));
                    ui.add(egui::Slider::new(&mut self.standard_bar_length_m, 6.0..=14.0).suffix(" m").step_by(0.5));

                    ui.add_space(8.0);
                    ui.label(RichText::new("Saw Kerf:").size(11.5).color(theme.text_secondary));
                    ui.add(egui::Slider::new(&mut self.saw_kerf_mm, 0.0..=10.0).suffix(" mm").step_by(0.5));

                    ui.add_space(8.0);
                    ui.label(RichText::new("Min Reusable:").size(11.5).color(theme.text_secondary));
                    ui.add(egui::Slider::new(&mut self.min_reusable_offcut_m, 0.5..=3.0).suffix(" m").step_by(0.25));
                });
            });

        ui.add_space(16.0);

        // 5. Visual Dynamic Cutting Pattern Canvas
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("✂️ Optimized Steel Bar Cutting Patterns (Dynamic BFD)").size(13.5).strong().color(theme.text_primary));
                    ui.label(RichText::new(format!("Algorithm: Best-Fit Decreasing • Total Cut Pieces: {} pcs", report.total_pieces_demanded))
                        .size(11.0).color(theme.accent_blue));
                });
                ui.add_space(12.0);

                let bar_width = (ui.available_width() - 10.0).max(240.0);
                let bar_height = 24.0;
                let colors = [
                    Color32::from_rgb(16, 185, 129),  // emerald
                    Color32::from_rgb(14, 165, 233),  // sky
                    Color32::from_rgb(99, 102, 241),  // indigo
                    Color32::from_rgb(245, 158, 11),  // amber
                    Color32::from_rgb(168, 85, 247),  // purple
                    Color32::from_rgb(236, 72, 153),  // pink
                ];

                // Render each master bar dynamically
                for (bar_idx, bar) in report.bars.iter().enumerate() {
                    ui.vertical(|ui| {
                        ui.horizontal_wrapped(|ui| {
                            let status_badge = if bar.remainder_m < 0.05 {
                                "⭐ 100% Efficiency (Zero Waste)"
                            } else if bar.is_reusable_offcut {
                                "♻️ Reusable Offcut (Gusset/Stiffener Ready)"
                            } else {
                                "🟡 Net Scrap Waste"
                            };

                            let badge_color = if bar.remainder_m < 0.05 {
                                theme.accent_green
                            } else if bar.is_reusable_offcut {
                                theme.accent_blue
                            } else {
                                Color32::from_rgb(245, 158, 11)
                            };

                            ui.label(RichText::new(format!(
                                "Master Bar #{:02} • {} ({:.2}m)",
                                bar.bar_index, bar.profile, bar.stock_length_m
                            )).size(12.5).strong().color(theme.text_primary));

                            ui.label(RichText::new(format!("Yield: {:.1}%", bar.yield_pct)).size(11.0).color(theme.text_muted));
                            ui.label(RichText::new(status_badge).size(11.5).strong().color(badge_color));
                        });
                        ui.add_space(4.0);

                        // Visual Segmented Bar Diagram
                        let (rect, _) = ui.allocate_exact_size(Vec2::new(bar_width, bar_height), egui::Sense::hover());

                        let total_stock_m = bar.stock_length_m as f32;

                        for (cut_idx, cut) in bar.cuts.iter().enumerate() {
                            let seg_x1 = rect.min.x + (cut.start_m as f32 / total_stock_m) * bar_width;
                            let seg_x2 = rect.min.x + (cut.end_m as f32 / total_stock_m) * bar_width;
                            let seg_w = (seg_x2 - seg_x1 - 1.5).max(1.0);

                            let seg_rect = egui::Rect::from_min_max(
                                egui::Pos2::new(seg_x1, rect.min.y),
                                egui::Pos2::new((seg_x1 + seg_w).min(rect.max.x), rect.max.y),
                            );

                            let color = colors[(bar_idx + cut_idx) % colors.len()];
                            ui.painter().rect_filled(seg_rect, CornerRadius::same(4), color);

                            if seg_w > 45.0 {
                                let label_text = format!("{} ({:.2}m)", cut.label, cut.length_m);
                                ui.painter().text(
                                    seg_rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    label_text,
                                    egui::FontId::proportional(10.0),
                                    Color32::WHITE,
                                );
                            }
                        }

                        // Offcut/Scrap at the end
                        if bar.remainder_m > 0.01 {
                            let rem_x1 = rect.min.x + (bar.used_length_m as f32 / total_stock_m) * bar_width;
                            let rem_rect = egui::Rect::from_min_max(
                                egui::Pos2::new(rem_x1.min(rect.max.x), rect.min.y),
                                egui::Pos2::new(rect.max.x, rect.max.y),
                            );
                            let rem_color = if bar.is_reusable_offcut {
                                Color32::from_rgba_premultiplied(14, 165, 233, 100)
                            } else {
                                Color32::from_rgba_premultiplied(239, 68, 68, 120)
                            };
                            ui.painter().rect_filled(rem_rect, CornerRadius::same(4), rem_color);

                            if rem_rect.width() > 50.0 {
                                let rem_text = if bar.is_reusable_offcut {
                                    format!("Offcut {:.2}m", bar.remainder_m)
                                } else {
                                    format!("Scrap {:.2}m", bar.remainder_m)
                                };
                                ui.painter().text(
                                    rem_rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    rem_text,
                                    egui::FontId::proportional(9.5),
                                    Color32::WHITE,
                                );
                            }
                        }

                        ui.add_space(4.0);
                        let sub_info = if bar.is_reusable_offcut {
                            format!("Remainder: {:.2}m (Meets reusable threshold ≥ {:.1}m for stiffeners/plates)", bar.remainder_m, report.min_reusable_m)
                        } else if bar.remainder_m > 0.01 {
                            format!("Net Scrap Offcut: {:.2}m (Kerf loss: {:.1}mm)", bar.remainder_m, bar.kerf_loss_m * 1e3)
                        } else {
                            "Fully utilized with zero waste".to_string()
                        };
                        ui.label(RichText::new(sub_info).size(10.5).color(theme.text_muted));
                        ui.add_space(8.0);
                    });
                    ui.separator();
                }

                ui.add_space(10.0);

                // 6. Real Action Export Buttons
                ui.horizontal_wrapped(|ui| {
                    if ui.add(egui::Button::new(RichText::new("📥 Export Bar Bending Schedule (BBS) .CSV").size(12.0).strong().color(Color32::WHITE))
                        .fill(theme.accent_blue)
                        .corner_radius(CornerRadius::same(6))).clicked() {
                        let bbs = CuttingStockOptimizer::generate_sample_bbs(members.len());
                        let csv = CuttingStockOptimizer::export_bbs_csv(&bbs);
                        if let Ok(path) = crate::core::DiskExporter::write_export_file("Bar_Bending_Schedule_SNI2847.csv", &csv) {
                            self.export_status = Some(format!("BBS exported to: {}", path));
                        }
                    }

                    if ui.add(egui::Button::new(RichText::new("📄 Export Cutting Plan .CSV").size(12.0).strong().color(Color32::WHITE))
                        .fill(Color32::from_rgb(16, 185, 129))
                        .corner_radius(CornerRadius::same(6))).clicked() {
                        let csv = CuttingStockOptimizer::export_cutting_plan_csv(&report);
                        if let Ok(path) = crate::core::DiskExporter::write_export_file("Cutting_Plan_Optimized.csv", &csv) {
                            self.export_status = Some(format!("Cutting plan exported to: {}", path));
                        }
                    }

                    ui.label(RichText::new("Standard format interoperable with Voortman CAM, Tekla FabTrol, and CNC Bar Benders.")
                        .size(11.0).color(theme.text_secondary));
                });
            });
    }

    fn render_last_planner(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let (committed, completed, live_ppc) = LpsLookaheadTask::compute_live_ppc(&self.lps_tasks);
        let sound_count = self.lps_tasks.iter().filter(|t| t.is_sound).count();
        let sound_pct = if self.lps_tasks.is_empty() {
            0.0
        } else {
            (sound_count as f32 / self.lps_tasks.len() as f32) * 100.0
        };
        let open_holds = self.lps_tasks.iter().filter(|t| !t.is_sound).count();
        let ppc_col = if live_ppc >= 80.0 {
            theme.accent_green
        } else if live_ppc >= 65.0 {
            theme.accent_orange
        } else {
            theme.accent_red
        };

        let cards = [
            (
                "LIVE WEEKLY PPC RELIABILITY",
                format!("{:.1}%", live_ppc),
                format!("Benchmark Target >= 80.0% ({}/{})", completed, committed),
                ppc_col,
            ),
            (
                "6-WEEK MAKE-READY SOUNDNESS",
                format!("{:.0}%", sound_pct),
                format!("{}/{} Activities Constraint-Free", sound_count, self.lps_tasks.len()),
                theme.accent_blue,
            ),
            (
                "ACTIVE CONSTRAINT HOLDS",
                format!("{} Holds", open_holds),
                "Resolve RFI / Material / Crane blockers".to_string(),
                if open_holds == 0 { theme.accent_green } else { theme.accent_orange },
            ),
            (
                "COMMITTED WWP TASKS",
                format!("{} Promises", committed),
                format!("{} Uncommitted Backlog items", self.lps_tasks.len().saturating_sub(committed as usize)),
                theme.text_primary,
            ),
        ];
        self.render_responsive_cards(ui, theme, &cards);
        ui.add_space(14.0);

        // Interactive 6-Week Lookahead Board & Constraint Log
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new("📋 6-Week Make-Ready Lookahead & Weekly Work Plan (WWP)")
                            .size(14.0)
                            .strong()
                            .color(theme.text_primary),
                    );
                    Frame::new()
                        .fill(ppc_col.gamma_multiply(0.16))
                        .corner_radius(CornerRadius::same(5))
                        .inner_margin(Margin::symmetric(8, 2))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(format!("LIVE PPC: {:.1}%", live_ppc))
                                    .size(10.5)
                                    .strong()
                                    .color(ppc_col),
                            );
                        });
                });
                ui.add_space(8.0);

                // Week filter pills
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Week Filter:").size(11.0).color(theme.text_secondary));
                    for wk in ["All Weeks", "W-01", "W-02", "W-03", "W-04", "W-05", "W-06"] {
                        if ui.selectable_label(self.lps_week_filter == wk, wk).clicked() {
                            self.lps_week_filter = wk.to_string();
                        }
                    }
                });

                ui.add_space(8.0);

                // Quick Commitment Creator Bar
                ui.horizontal_wrapped(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.lps_new_task_name)
                            .hint_text("Add new lookahead commitment (e.g., Curtain wall gasket torque)...")
                            .desired_width((ui.available_width() * 0.45).clamp(160.0, 360.0)),
                    );
                    egui::ComboBox::from_id_salt("lps_new_wk_combo")
                        .selected_text(RichText::new(&self.lps_new_task_week).size(11.0))
                        .width(75.0)
                        .show_ui(ui, |ui| {
                            for wk in ["W-04", "W-05", "W-06"] {
                                ui.selectable_value(&mut self.lps_new_task_week, wk.to_string(), wk);
                            }
                        });
                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("➕ Add Work Package")
                                    .size(11.0)
                                    .strong()
                                    .color(Color32::WHITE),
                            )
                            .fill(theme.accent_blue)
                            .corner_radius(CornerRadius::same(6)),
                        )
                        .clicked()
                        && !self.lps_new_task_name.trim().is_empty()
                    {
                        let next_id = self.lps_tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
                        self.lps_tasks.push(LpsLookaheadTask {
                            id: next_id,
                            wbs_code: format!("05.{:02}.{:02}", next_id, next_id * 2),
                            week_bucket: self.lps_new_task_week.clone(),
                            activity_name: self.lps_new_task_name.trim().to_string(),
                            trade_crew: "Field Erection Crew".to_string(),
                            constraint_category: LpsConstraintCategory::ConstraintFree,
                            constraint_note: "Pre-screened constraint-free".to_string(),
                            is_sound: true,
                            committed_in_wwp: true,
                            completed_on_time: false,
                            variance_reason: "Pending Execution".to_string(),
                        });
                        self.lps_new_task_name.clear();
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(8.0);

                // Task List
                let mut visible_rows = 0usize;
                for task in &mut self.lps_tasks {
                    if self.lps_week_filter != "All Weeks" && task.week_bucket != self.lps_week_filter {
                        continue;
                    }
                    visible_rows += 1;

                    let row_col = if task.completed_on_time && task.committed_in_wwp {
                        theme.accent_green
                    } else if !task.is_sound {
                        theme.accent_orange
                    } else {
                        theme.accent_blue
                    };

                    Frame::new()
                        .fill(theme.bg_canvas)
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(Margin::symmetric(12, 8))
                        .stroke(Stroke::new(0.8, theme.border_subtle))
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                Frame::new()
                                    .fill(theme.accent_blue.gamma_multiply(0.16))
                                    .corner_radius(CornerRadius::same(4))
                                    .inner_margin(Margin::symmetric(6, 2))
                                    .show(ui, |ui| {
                                        ui.label(
                                            RichText::new(&task.week_bucket)
                                                .size(10.0)
                                                .strong()
                                                .color(theme.accent_blue),
                                        );
                                    });
                                ui.label(
                                    RichText::new(&task.wbs_code)
                                        .size(10.0)
                                        .monospace()
                                        .color(theme.text_muted),
                                );
                                ui.label(
                                    RichText::new(&task.activity_name)
                                        .size(12.0)
                                        .strong()
                                        .color(theme.text_primary),
                                );
                                ui.label(
                                    RichText::new(format!("• {}", task.trade_crew))
                                        .size(10.5)
                                        .color(theme.text_secondary),
                                );
                            });

                            ui.add_space(4.0);
                            ui.horizontal_wrapped(|ui| {
                                ui.label(
                                    RichText::new(format!(
                                        "{} — {}",
                                        task.constraint_category.label(),
                                        task.constraint_note
                                    ))
                                    .size(10.5)
                                    .color(row_col),
                                );

                                let sound_lbl = if task.is_sound {
                                    "🔓 Sound Ready"
                                } else {
                                    "🔒 Clear Constraint"
                                };
                                if ui
                                    .add(
                                        egui::Button::new(RichText::new(sound_lbl).size(10.0).color(if task.is_sound {
                                            theme.accent_green
                                        } else {
                                            theme.accent_orange
                                        }))
                                        .fill(theme.bg_card)
                                        .corner_radius(CornerRadius::same(4)),
                                    )
                                    .clicked()
                                {
                                    task.is_sound = !task.is_sound;
                                    if task.is_sound {
                                        task.constraint_category = LpsConstraintCategory::ConstraintFree;
                                        task.constraint_note = "Constraint cleared by site engineer".to_string();
                                    } else {
                                        task.constraint_category = LpsConstraintCategory::MaterialSupply;
                                        task.completed_on_time = false;
                                        if task.variance_reason == "None" {
                                            task.variance_reason = "Material / Access Hold".to_string();
                                        }
                                    }
                                }

                                if ui
                                    .checkbox(&mut task.committed_in_wwp, "Committed (WWP)")
                                    .changed()
                                    && !task.committed_in_wwp
                                {
                                    task.completed_on_time = false;
                                }

                                if ui
                                    .add_enabled(
                                        task.committed_in_wwp,
                                        egui::Checkbox::new(&mut task.completed_on_time, "Completed On-Time"),
                                    )
                                    .changed()
                                    && task.completed_on_time
                                {
                                    task.is_sound = true;
                                    task.variance_reason = "None".to_string();
                                }
                            });
                        });
                    ui.add_space(4.0);
                }

                if visible_rows == 0 {
                    ui.label(
                        RichText::new("No lookahead activities scheduled in selected week filter.")
                            .size(11.0)
                            .color(theme.text_muted),
                    );
                }
            });

        ui.add_space(16.0);

        // PPC Trend Chart + Variance Root-Cause Pareto (Responsive Split)
        let draw_ppc_chart = |ui: &mut Ui| {
            Frame::new()
                .fill(theme.bg_card)
                .corner_radius(CornerRadius::same(10))
                .inner_margin(Margin::same(16))
                .stroke(Stroke::new(1.0, theme.border_subtle))
                .show(ui, |ui| {
                    ui.label(
                        RichText::new("📈 Weekly PPC Trend vs 80% Benchmark")
                            .size(13.0)
                            .strong()
                            .color(theme.text_primary),
                    );
                    ui.label(
                        RichText::new("Historical weekly reliability progression including live WWP simulation")
                            .size(10.5)
                            .color(theme.text_muted),
                    );
                    ui.add_space(10.0);

                    let chart_w = (ui.available_width() - 4.0).max(200.0);
                    let chart_h = 165.0;
                    let (rect, _) = ui.allocate_exact_size(Vec2::new(chart_w, chart_h), egui::Sense::hover());
                    let p = ui.painter();
                    p.rect_filled(rect, CornerRadius::same(6), Color32::from_rgb(16, 20, 28));
                    p.rect_stroke(rect, CornerRadius::same(6), Stroke::new(0.8, theme.border_subtle), StrokeKind::Inside);

                    let pad_x = 26.0;
                    let pad_top = 18.0;
                    let pad_bot = 24.0;
                    let plot_h = (rect.height() - pad_top - pad_bot).max(40.0);
                    let baseline_y = rect.bottom() - pad_bot;

                    // 80% Benchmark Line
                    let target_y = baseline_y - plot_h * 0.80;
                    p.line_segment(
                        [Pos2::new(rect.left() + pad_x, target_y), Pos2::new(rect.right() - 10.0, target_y)],
                        Stroke::new(1.2, theme.accent_orange),
                    );
                    p.text(
                        Pos2::new(rect.left() + pad_x + 4.0, target_y - 10.0),
                        Align2::LEFT_TOP,
                        "80% Target",
                        egui::FontId::monospace(9.0),
                        theme.accent_orange,
                    );

                    // Bars for W-01..W-05 + Live WWP
                    let mut series: Vec<(String, f32, bool)> = self
                        .lps_history
                        .iter()
                        .map(|h| (h.week_label.clone(), h.ppc_pct, false))
                        .collect();
                    series.push(("Live".to_string(), live_ppc, true));

                    let slot_w = (rect.width() - pad_x - 16.0) / (series.len().max(1) as f32);
                    let bar_w = (slot_w * 0.56).clamp(14.0, 38.0);

                    for (idx, (lbl, pct, is_live)) in series.iter().enumerate() {
                        let cx = rect.left() + pad_x + slot_w * (idx as f32 + 0.5);
                        let frac = (*pct / 100.0).clamp(0.0, 1.0);
                        let bh = plot_h * frac;
                        let brect = Rect::from_min_max(
                            Pos2::new(cx - bar_w * 0.5, baseline_y - bh),
                            Pos2::new(cx + bar_w * 0.5, baseline_y),
                        );
                        let bcol = if *is_live {
                            if *pct >= 80.0 { theme.accent_green } else { theme.accent_red }
                        } else if *pct >= 80.0 {
                            theme.accent_blue
                        } else {
                            theme.accent_orange
                        };
                        p.rect_filled(brect, CornerRadius::same(4), bcol);
                        p.text(
                            Pos2::new(cx, brect.top() - 11.0),
                            Align2::CENTER_TOP,
                            format!("{:.0}%", pct),
                            egui::FontId::monospace(9.0),
                            Color32::WHITE,
                        );
                        p.text(
                            Pos2::new(cx, baseline_y + 5.0),
                            Align2::CENTER_TOP,
                            lbl,
                            egui::FontId::monospace(9.0),
                            theme.text_muted,
                        );
                    }
                });
        };

        let draw_pareto_chart = |ui: &mut Ui| {
            let pareto = LpsLookaheadTask::compute_variance_pareto(&self.lps_tasks);
            Frame::new()
                .fill(theme.bg_card)
                .corner_radius(CornerRadius::same(10))
                .inner_margin(Margin::same(16))
                .stroke(Stroke::new(1.0, theme.border_subtle))
                .show(ui, |ui| {
                    ui.label(
                        RichText::new("📊 Variance Root-Cause Pareto (Non-Completions)")
                            .size(13.0)
                            .strong()
                            .color(theme.text_primary),
                    );
                    ui.label(
                        RichText::new("Primary blockers preventing 100% Weekly Work Plan reliability")
                            .size(10.5)
                            .color(theme.text_muted),
                    );
                    ui.add_space(10.0);

                    if pareto.is_empty() {
                        Frame::new()
                            .fill(theme.accent_green.gamma_multiply(0.14))
                            .corner_radius(CornerRadius::same(8))
                            .inner_margin(Margin::same(14))
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new("🎉 100% Reliability! Zero open variance reasons in current Weekly Work Plan.")
                                        .size(11.5)
                                        .strong()
                                        .color(theme.accent_green),
                                );
                            });
                    } else {
                        let total_var: u32 = pareto.iter().map(|(_, c)| *c).sum();
                        for (reason, count) in pareto {
                            let share = (count as f32 / total_var.max(1) as f32).clamp(0.0, 1.0);
                            ui.horizontal_wrapped(|ui| {
                                ui.label(RichText::new(&reason).size(11.0).strong().color(theme.text_primary));
                                ui.label(
                                    RichText::new(format!("• {} holds ({:.0}%)", count, share * 100.0))
                                        .size(10.5)
                                        .strong()
                                        .color(theme.accent_orange),
                                );
                            });
                            let bw = (ui.available_width() - 4.0).max(120.0);
                            let (bar_rect, _) = ui.allocate_exact_size(Vec2::new(bw, 10.0), egui::Sense::hover());
                            ui.painter().rect_filled(bar_rect, CornerRadius::same(4), theme.bg_canvas);
                            let fill_rect = Rect::from_min_size(bar_rect.min, Vec2::new(bw * share, 10.0));
                            ui.painter().rect_filled(fill_rect, CornerRadius::same(4), theme.accent_orange);
                            ui.add_space(6.0);
                        }
                    }
                });
        };

        ui_kit::responsive_two_columns(ui, 800.0, draw_ppc_chart, draw_pareto_chart);
    }

    fn render_value_stream(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.label(RichText::new("🌊 Value Stream Mapping: Eliminating 7 Construction Wastes").size(13.5).strong().color(theme.text_primary));
                ui.add_space(10.0);

                let wastes = [
                    ("1. Overproduction", "Fabricate modules strictly aligned with 4D BIM erection sequence", "Eliminates onsite laydown congestion (-100%)"),
                    ("2. Waiting Time", "Crane mobilized Just-In-Time with modular arrivals", "Crane rental cost reduced 35%"),
                    ("3. Transportation", "Direct transport routes from fabrication yard to site", "Reduces haul distance and diesel emissions"),
                    ("4. Overprocessing", "Standardized HTB bolted splices without field welding", "Factory-controlled joint quality verified"),
                    ("5. Inventory", "Laydown buffer restricted to maximum 2 days work", "Clear jobsite footprint for equipment staging"),
                    ("6. Motion", "Laydown organized within crane boom slewing radius", "Pick-and-place cycle duration reduced 25%"),
                    ("7. Defects / Rework", "SVD Kinematic Guard simulation prevents erection instabilities", "Zero mechanism collapse or rework disassembly"),
                ];

                for (name, action, result) in wastes {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(name).size(12.0).strong().color(theme.accent_blue));
                        ui.label(RichText::new(format!("— {} ({})", action, result)).size(11.5).color(theme.text_secondary));
                    });
                    ui.add_space(4.0);
                }
            });
    }

    fn render_jit_delivery(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.label(RichText::new("🚚 Just-In-Time (JIT) Delivery Buffer Scheduling").size(13.5).strong().color(theme.text_primary));
                ui.add_space(10.0);

                let cards = [
                    ("MAX ONSITE STAGING BUFFER", "Max 2 Days".to_string(), "Prevents Jobsite Congestion".to_string(), theme.accent_green),
                    ("DIRECT RIGGING FROM TRUCK", "70% Components".to_string(), "Rigged Directly from Trailer Bed to Position".to_string(), theme.accent_blue),
                    ("DOUBLE HANDLING REDUCTION", "-80%".to_string(), "Mitigates Structural Impact Risks".to_string(), theme.text_primary),
                ];
                self.render_responsive_cards(ui, theme, &cards);
            });
    }

    fn render_stat_card(&self, ui: &mut Ui, theme: &MacTheme, title: &str, val: &str, subtitle: &str, val_color: Color32) {
        Frame::new()
            .fill(theme.bg_canvas)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(14))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(RichText::new(title).size(10.0).strong().color(theme.text_muted));
                ui.add_space(4.0);
                ui.label(RichText::new(val).size(18.0).strong().color(val_color));
                ui.add_space(4.0);
                ui.label(RichText::new(subtitle).size(10.5).color(theme.text_secondary));
            });
    }
}
