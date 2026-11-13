//! views/analyze/member_design.rs — Code Member Verification Suite (Concrete P-M, Steel Connections, Wind, Composite, PT, Shear Walls).

use super::*;

impl AnalyzeView {
    pub(super) fn render_concrete_pm_interaction(&mut self, ui: &mut Ui, theme: &MacTheme) {
        use crate::core::concrete::ConcreteEngine;

        // 1. Build Section and Evaluate P-M Interaction Diagram
        let total_bars = self.concrete_bars_per_face_x * 2 + self.concrete_bars_intermediate_y * 2;
        let col_sec = ConcreteEngine::create_rectangular_column(
            &format!("Col_{:.0}x{:.0}", self.concrete_col_width_mm, self.concrete_col_height_mm),
            self.concrete_col_width_mm as f64,
            self.concrete_col_height_mm as f64,
            self.concrete_fc_mpa as f64,
            self.concrete_fy_mpa as f64,
            self.concrete_cover_mm as f64,
            self.concrete_bars_per_face_x,
            self.concrete_bars_intermediate_y,
            self.concrete_bar_dia_mm as f64,
        );

        let pm_report = ConcreteEngine::evaluate_pm_diagram(
            &col_sec,
            Some(self.concrete_demand_pu_kn as f64),
            Some(self.concrete_demand_mu_knm as f64),
        );

        let biaxial_rep = ConcreteEngine::evaluate_biaxial_pm(
            &col_sec,
            self.concrete_demand_pu_kn as f64,
            self.concrete_demand_mu_knm as f64,
            self.concrete_demand_muy_knm as f64,
        );

        let _dcr = pm_report.demand_check.as_ref().map(|d| d.dcr).unwrap_or(0.0);
        let is_safe = biaxial_rep.is_safe;

        // 2. Top Metric Cards
        ui_kit::responsive_grid_n(ui, 4, 190.0, |ui, col_idx| match col_idx {
            0 => {
                self.metric_card(
                        ui,
                        theme,
                        "MAX AXIAL LIMIT (φPn,max)",
                        &format!("{:.0} kN", pm_report.cardinal.phi_pn_max_kn),
                        "SNI 2847:2019 Section 22.4.2 (Tied)",
                        theme.accent_blue,
                    );
            }
            1 => {
                self.metric_card(
                        ui,
                        theme,
                        "BALANCED POINT (φPb, φMb)",
                        &format!("{:.0} kN, {:.0} kNm", pm_report.cardinal.balanced.phi_pn_kn, pm_report.cardinal.balanced.phi_mn_knm),
                        "Simultaneous Tensile Yield (εt = εy)",
                        theme.accent_orange,
                    );
            }
            2 => {
                self.metric_card(
                        ui,
                        theme,
                        "REBAR RATIO (ρ)",
                        &format!("{:.2}%", pm_report.rebar_ratio_pct),
                        &format!("{} Bars D{:.0} (Ast = {:.0} mm²)", total_bars, self.concrete_bar_dia_mm, pm_report.ast_mm2),
                        if pm_report.rebar_ratio_pct >= 1.0 && pm_report.rebar_ratio_pct <= 4.0 { theme.accent_green } else { Color32::from_rgb(245, 158, 11) },
                    );
            }
            _ => {
                self.metric_card(
                        ui,
                        theme,
                        "BIAXIAL AUDIT (DCR)",
                        &format!("{:.3} (Biax)", biaxial_rep.biaxial_dcr),
                        if is_safe { "🟢 SAFE (Biaxial Adequate)" } else { "🔴 OVERSTRESSED (Exceeds Envelope)" },
                        if is_safe { theme.accent_green } else { Color32::from_rgb(239, 68, 68) },
                    );
            }
        });

        ui.add_space(16.0);

        // 3. Section Preset Buttons & Parameter Controls
        theme.card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("🏛️ Standard Section Presets (SNI 2847 / ACI 318):").size(12.5).strong().color(theme.text_primary));
                ui.add_space(8.0);

                if ui.button(RichText::new("Building Column 400x400 (8 D22, fc' 30)").size(11.0)).clicked() {
                    self.concrete_col_width_mm = 400.0;
                    self.concrete_col_height_mm = 400.0;
                    self.concrete_fc_mpa = 30.0;
                    self.concrete_fy_mpa = 420.0;
                    self.concrete_bars_per_face_x = 3;
                    self.concrete_bars_intermediate_y = 1;
                    self.concrete_bar_dia_mm = 22.0;
                }
                if ui.button(RichText::new("Interior Column 600x600 (12 D25, fc' 35)").size(11.0)).clicked() {
                    self.concrete_col_width_mm = 600.0;
                    self.concrete_col_height_mm = 600.0;
                    self.concrete_fc_mpa = 35.0;
                    self.concrete_fy_mpa = 420.0;
                    self.concrete_bars_per_face_x = 4;
                    self.concrete_bars_intermediate_y = 2;
                    self.concrete_bar_dia_mm = 25.0;
                }
                if ui.button(RichText::new("Tower Column 800x800 (16 D32, fc' 40)").size(11.0)).clicked() {
                    self.concrete_col_width_mm = 800.0;
                    self.concrete_col_height_mm = 800.0;
                    self.concrete_fc_mpa = 40.0;
                    self.concrete_fy_mpa = 420.0;
                    self.concrete_bars_per_face_x = 5;
                    self.concrete_bars_intermediate_y = 3;
                    self.concrete_bar_dia_mm = 32.0;
                }
                if ui.button(RichText::new("Blade Column 250x600 (10 D19, fc' 25)").size(11.0)).clicked() {
                    self.concrete_col_width_mm = 250.0;
                    self.concrete_col_height_mm = 600.0;
                    self.concrete_fc_mpa = 25.0;
                    self.concrete_fy_mpa = 420.0;
                    self.concrete_bars_per_face_x = 2;
                    self.concrete_bars_intermediate_y = 3;
                    self.concrete_bar_dia_mm = 19.0;
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(8.0);

            // Parameter Sliders
            ui_kit::responsive_grid_n(ui, 3, 240.0, |ui, col_idx| match col_idx {
                0 => {
                    ui.label(RichText::new("📐 Dimensions & Concrete Cover:").size(11.5).strong().color(theme.text_secondary));
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Width b:").size(11.0));
                                    ui.add(egui::Slider::new(&mut self.concrete_col_width_mm, 200.0..=1200.0).suffix(" mm").step_by(50.0));
                                });
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Height h:").size(11.0));
                                    ui.add(egui::Slider::new(&mut self.concrete_col_height_mm, 200.0..=1200.0).suffix(" mm").step_by(50.0));
                                });
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Cover:").size(11.0));
                                    ui.add(egui::Slider::new(&mut self.concrete_cover_mm, 25.0..=75.0).suffix(" mm").step_by(5.0));
                                });
                }
                1 => {
                    ui.label(RichText::new("🔩 Material Grades (SNI / ACI):").size(11.5).strong().color(theme.text_secondary));
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Concrete Grade f'c:").size(11.0));
                                    egui::ComboBox::from_id_salt("fc_select")
                                        .selected_text(format!("{:.0} MPa", self.concrete_fc_mpa))
                                        .show_ui(ui, |ui| {
                                            for &fc in &[20.0, 25.0, 30.0, 35.0, 40.0, 50.0] {
                                                ui.selectable_value(&mut self.concrete_fc_mpa, fc, format!("{:.0} MPa", fc));
                                            }
                                        });
                                });
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Steel Grade fy:").size(11.0));
                                    egui::ComboBox::from_id_salt("fy_select")
                                        .selected_text(format!("BJTS {:.0}", self.concrete_fy_mpa))
                                        .show_ui(ui, |ui| {
                                            for &fy in &[280.0, 420.0, 520.0] {
                                                let label = if fy > 400.0 { format!("Grade {:.0} (Deformed)", fy) } else { format!("Grade {:.0} (Plain)", fy) };
                                                ui.selectable_value(&mut self.concrete_fy_mpa, fy, label);
                                            }
                                        });
                                });
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Rebar Diameter:").size(11.0));
                                    egui::ComboBox::from_id_salt("dia_select")
                                        .selected_text(format!("D{:.0}", self.concrete_bar_dia_mm))
                                        .show_ui(ui, |ui| {
                                            for &dia in &[13.0, 16.0, 19.0, 22.0, 25.0, 29.0, 32.0] {
                                                ui.selectable_value(&mut self.concrete_bar_dia_mm, dia, format!("D{:.0}", dia));
                                            }
                                        });
                                });
                }
                _ => {
                    ui.label(RichText::new("⚡ Factored Design Demand:").size(11.5).strong().color(theme.text_secondary));
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Axial Load Pu:").size(11.0));
                                    ui.add(egui::Slider::new(&mut self.concrete_demand_pu_kn, -500.0..=5000.0).suffix(" kN").step_by(50.0));
                                });
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Moment Mux:").size(11.0));
                                    ui.add(egui::Slider::new(&mut self.concrete_demand_mu_knm, 0.0..=800.0).suffix(" kNm").step_by(10.0));
                                });
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Moment Muy:").size(11.0));
                                    ui.add(egui::Slider::new(&mut self.concrete_demand_muy_knm, 0.0..=800.0).suffix(" kNm").step_by(10.0));
                                });
                                ui.horizontal(|ui| {
                                    let status_label = if is_safe { "🟢 SAFE" } else { "🔴 UNSAFE" };
                                    let status_col = if is_safe { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
                                    ui.label(RichText::new(format!("Biax DCR: {:.3} (α={:.2}) • {}", biaxial_rep.biaxial_dcr, biaxial_rep.alpha_exponent, status_label)).size(11.0).strong().color(status_col));
                                });
                }
            });
        });

        ui.add_space(16.0);

        // 4. Interactive Dual Canvas: P-M Diagram (Left) + Rebar Section View (Right)
        theme.card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("📊 P-M Interaction Diagram & Column Section Visualizer").size(13.5).strong().color(theme.text_primary));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(RichText::new(format!("Standard: SNI 2847:2019 / ACI 318-19 • β₁ = {:.3}", pm_report.beta1)).size(11.0).color(theme.accent_blue));
                });
            });
            ui.add_space(10.0);

            let canvas_height = 360.0;
            ui_kit::split_workspace_panes(ui, 760.0, 0.64, 320.0, 760.0, canvas_height, |ui, pane_idx, pane_w| {
                let eff_w = pane_w.unwrap_or_else(|| ui.available_width()).max(220.0);
                if pane_idx == 0 {
                    let pm_width = eff_w;
                // LEFT CANVAS: P-M Interaction Curves
                let (pm_rect, _) = ui.allocate_exact_size(Vec2::new(pm_width, canvas_height), egui::Sense::hover());
                ui.painter().rect_filled(pm_rect, CornerRadius::same(8), theme.bg_canvas);
                ui.painter().rect_stroke(pm_rect, CornerRadius::same(8), Stroke::new(1.0, theme.border_subtle), egui::StrokeKind::Inside);

                let m_max = pm_report.nominal_curve.iter().map(|(m, _)| *m).fold(100.0_f64, f64::max) * 1.15;
                let p_max = pm_report.cardinal.p0_kn * 1.08;
                let p_min = pm_report.cardinal.pnt_kn * 1.15; // negative tension

                let margin_left = 65.0;
                let margin_bottom = 35.0;
                let plot_w = pm_width - margin_left - 20.0;
                let plot_h = canvas_height - margin_bottom - 20.0;

                let map_pt = |m: f64, p: f64| -> egui::Pos2 {
                    let norm_x = (m / m_max).clamp(0.0, 1.2) as f32;
                    let norm_y = ((p - p_min) / (p_max - p_min)).clamp(0.0, 1.2) as f32;
                    egui::Pos2::new(
                        pm_rect.min.x + margin_left + norm_x * plot_w,
                        pm_rect.max.y - margin_bottom - norm_y * plot_h,
                    )
                };

                // Grid & Zero-Lines
                let zero_pt = map_pt(0.0, 0.0);
                // Horizontal zero-P axis
                ui.painter().line_segment(
                    [egui::Pos2::new(pm_rect.min.x + margin_left, zero_pt.y), egui::Pos2::new(pm_rect.max.x - 15.0, zero_pt.y)],
                    Stroke::new(1.0, Color32::from_rgb(70, 70, 80)),
                );
                // Vertical zero-M axis
                ui.painter().line_segment(
                    [egui::Pos2::new(pm_rect.min.x + margin_left, pm_rect.min.y + 15.0), egui::Pos2::new(pm_rect.min.x + margin_left, pm_rect.max.y - margin_bottom)],
                    Stroke::new(1.2, Color32::from_rgb(100, 100, 110)),
                );

                // Axis Labels
                ui.painter().text(
                    egui::Pos2::new(pm_rect.min.x + margin_left + 10.0, pm_rect.min.y + 16.0),
                    egui::Align2::LEFT_TOP,
                    "P (kN) [Axial]",
                    egui::FontId::proportional(11.0),
                    theme.text_secondary,
                );
                ui.painter().text(
                    egui::Pos2::new(pm_rect.max.x - 20.0, zero_pt.y + 14.0),
                    egui::Align2::RIGHT_TOP,
                    "M (kNm) [Flexure]",
                    egui::FontId::proportional(11.0),
                    theme.text_secondary,
                );

                // 1. Draw Nominal Envelope Curve (Pn - Mn) - Dashed Blue
                for w in pm_report.nominal_curve.windows(2) {
                    let p1 = map_pt(w[0].0, w[0].1);
                    let p2 = map_pt(w[1].0, w[1].1);
                    ui.painter().line_segment([p1, p2], Stroke::new(1.4, Color32::from_rgba_premultiplied(14, 165, 233, 160)));
                }

                // 2. Draw Factored Design Envelope Curve (phi*Pn - phi*Mn) - Solid Bright Emerald Green
                for w in pm_report.design_curve.windows(2) {
                    let p1 = map_pt(w[0].0, w[0].1);
                    let p2 = map_pt(w[1].0, w[1].1);
                    ui.painter().line_segment([p1, p2], Stroke::new(2.5, theme.accent_green));
                }

                // 3. Draw Upper Compression Cutoff (phi*Pn,max) - Amber Dashed
                let cutoff_y = map_pt(0.0, pm_report.cardinal.phi_pn_max_kn).y;
                ui.painter().line_segment(
                    [egui::Pos2::new(pm_rect.min.x + margin_left, cutoff_y), egui::Pos2::new(pm_rect.min.x + margin_left + plot_w * 0.45, cutoff_y)],
                    Stroke::new(1.6, Color32::from_rgb(245, 158, 11)),
                );
                ui.painter().text(
                    egui::Pos2::new(pm_rect.min.x + margin_left + 8.0, cutoff_y - 12.0),
                    egui::Align2::LEFT_BOTTOM,
                    format!("φPn,max = {:.0} kN", pm_report.cardinal.phi_pn_max_kn),
                    egui::FontId::proportional(10.0),
                    Color32::from_rgb(245, 158, 11),
                );

                // 4. Mark Balanced Point
                let bal_pt = map_pt(pm_report.cardinal.balanced.phi_mn_knm, pm_report.cardinal.balanced.phi_pn_kn);
                ui.painter().circle_filled(bal_pt, 4.5, theme.accent_orange);
                ui.painter().text(
                    bal_pt + Vec2::new(8.0, -4.0),
                    egui::Align2::LEFT_BOTTOM,
                    format!("Balanced ({:.0}, {:.0})", pm_report.cardinal.balanced.phi_mn_knm, pm_report.cardinal.balanced.phi_pn_kn),
                    egui::FontId::proportional(10.0),
                    theme.accent_orange,
                );

                // 5. Mark Pure Moment Point
                let m0_pt = map_pt(pm_report.cardinal.pure_flexure.phi_mn_knm, 0.0);
                ui.painter().circle_filled(m0_pt, 4.0, theme.accent_blue);
                ui.painter().text(
                    m0_pt + Vec2::new(0.0, 12.0),
                    egui::Align2::CENTER_TOP,
                    format!("φM0 = {:.0}", pm_report.cardinal.pure_flexure.phi_mn_knm),
                    egui::FontId::proportional(9.5),
                    theme.accent_blue,
                );

                // 6. Draw Factored Demand Point (Pu, Mu)
                let demand_pt = map_pt(self.concrete_demand_mu_knm as f64, self.concrete_demand_pu_kn as f64);
                let pt_color = if is_safe { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };

                // Glowing halo
                ui.painter().circle_filled(demand_pt, 8.0, pt_color.gamma_multiply(0.25));
                ui.painter().circle_filled(demand_pt, 4.5, pt_color);
                ui.painter().circle_stroke(demand_pt, 4.5, Stroke::new(1.5, Color32::WHITE));

                ui.painter().text(
                    demand_pt + Vec2::new(10.0, -8.0),
                    egui::Align2::LEFT_BOTTOM,
                    format!("Demand (Mu: {:.0}, Pu: {:.0})", self.concrete_demand_mu_knm, self.concrete_demand_pu_kn),
                    egui::FontId::proportional(11.0),
                    pt_color,
                );

                // Legend at top-right
                let leg_x = pm_rect.max.x - 170.0;
                let leg_y = pm_rect.min.y + 16.0;
                ui.painter().line_segment([egui::Pos2::new(leg_x, leg_y), egui::Pos2::new(leg_x + 22.0, leg_y)], Stroke::new(1.5, Color32::from_rgb(14, 165, 233)));
                ui.painter().text(egui::Pos2::new(leg_x + 28.0, leg_y), egui::Align2::LEFT_CENTER, "Nominal Pn-Mn", egui::FontId::proportional(10.0), theme.text_muted);

                ui.painter().line_segment([egui::Pos2::new(leg_x, leg_y + 16.0), egui::Pos2::new(leg_x + 22.0, leg_y + 16.0)], Stroke::new(2.5, theme.accent_green));
                ui.painter().text(egui::Pos2::new(leg_x + 28.0, leg_y + 16.0), egui::Align2::LEFT_CENTER, "Design Factored φPn-φMn", egui::FontId::proportional(10.0), theme.accent_green);

                } else {
                    let sec_width = eff_w;
                // RIGHT CANVAS: Column Cross-Section Visualizer
                let (sec_rect, _) = ui.allocate_exact_size(Vec2::new(sec_width, canvas_height), egui::Sense::hover());
                ui.painter().rect_filled(sec_rect, CornerRadius::same(8), theme.bg_canvas);
                ui.painter().rect_stroke(sec_rect, CornerRadius::same(8), Stroke::new(1.0, theme.border_subtle), egui::StrokeKind::Inside);

                let sec_center = sec_rect.center();
                let max_dim = self.concrete_col_width_mm.max(self.concrete_col_height_mm);
                let scale = (sec_width.min(canvas_height) * 0.65) / max_dim;

                let draw_w = self.concrete_col_width_mm * scale;
                let draw_h = self.concrete_col_height_mm * scale;

                let col_min = sec_center - Vec2::new(draw_w * 0.5, draw_h * 0.5);
                let col_max = sec_center + Vec2::new(draw_w * 0.5, draw_h * 0.5);
                let col_box = egui::Rect::from_min_max(col_min, col_max);

                // Concrete outer box
                ui.painter().rect_filled(col_box, CornerRadius::same(3), Color32::from_rgb(45, 47, 54));
                ui.painter().rect_stroke(col_box, CornerRadius::same(3), Stroke::new(1.5, Color32::from_rgb(180, 185, 195)), egui::StrokeKind::Inside);

                // Stirrup / Tie box (Cover offset)
                let cover_px = self.concrete_cover_mm * scale;
                let tie_box = col_box.shrink(cover_px);
                ui.painter().rect_stroke(tie_box, CornerRadius::same(2), Stroke::new(1.2, Color32::from_rgb(14, 165, 233)), egui::StrokeKind::Inside);

                // Draw Rebars inside
                let r_px = (self.concrete_bar_dia_mm * scale * 0.5).max(3.0);
                for layer in &col_sec.rebar_layers {
                    let y_px = col_min.y + (layer.depth_d_mm as f32) * scale;

                    if layer.bar_count == 2 {
                        // Side bars on left and right
                        let x1 = tie_box.min.x + r_px + 2.0;
                        let x2 = tie_box.max.x - r_px - 2.0;
                        for &x in &[x1, x2] {
                            ui.painter().circle_filled(egui::Pos2::new(x, y_px), r_px, Color32::from_rgb(52, 211, 153));
                            ui.painter().circle_stroke(egui::Pos2::new(x, y_px), r_px, Stroke::new(1.0, Color32::WHITE));
                        }
                    } else if layer.bar_count > 1 {
                        let x_start = tie_box.min.x + r_px + 2.0;
                        let x_end = tie_box.max.x - r_px - 2.0;
                        let dx = (x_end - x_start) / (layer.bar_count - 1) as f32;
                        for i in 0..layer.bar_count {
                            let x = x_start + dx * i as f32;
                            ui.painter().circle_filled(egui::Pos2::new(x, y_px), r_px, Color32::from_rgb(52, 211, 153));
                            ui.painter().circle_stroke(egui::Pos2::new(x, y_px), r_px, Stroke::new(1.0, Color32::WHITE));
                        }
                    }
                }

                // Section Dimension Callouts
                ui.painter().text(
                    egui::Pos2::new(col_box.center().x, col_box.max.y + 12.0),
                    egui::Align2::CENTER_TOP,
                    format!("b = {:.0} mm", self.concrete_col_width_mm),
                    egui::FontId::proportional(11.0),
                    theme.text_secondary,
                );
                ui.painter().text(
                    egui::Pos2::new(col_box.min.x - 12.0, col_box.center().y),
                    egui::Align2::RIGHT_CENTER,
                    format!("h = {:.0} mm", self.concrete_col_height_mm),
                    egui::FontId::proportional(11.0),
                    theme.text_secondary,
                );
                ui.painter().text(
                    egui::Pos2::new(sec_rect.center().x, sec_rect.min.y + 12.0),
                    egui::Align2::CENTER_TOP,
                    format!("Section: {} D{:.0} • Cover: {:.0}mm", total_bars, self.concrete_bar_dia_mm, self.concrete_cover_mm),
                    egui::FontId::proportional(10.5),
                    theme.accent_blue,
                );
                }
            });
        });

        ui.add_space(16.0);

        // 5. Technical Cardinal Values Verification Table per SNI 2847:2019
        theme.card_frame().show(ui, |ui| {
            ui.label(RichText::new("📋 Column Capacity Cardinal Points Verification Table (SNI 2847 / ACI 318):").size(12.5).strong().color(theme.text_primary));
            ui.add_space(8.0);

            let cardinals: [(&str, String, String, String, String); 5] = [
                ("1. Pure Compression (P0)", format!("{:.1} kN", pm_report.cardinal.p0_kn), "-".to_string(), "0.65".to_string(), "Absolute Limit P0 = 0.85 f'c (Ag - Ast) + fy Ast".to_string()),
                ("2. Factored Axial Limit (φPn,max)", format!("{:.1} kN", pm_report.cardinal.pn_max_kn), "-".to_string(), "0.65".to_string(), "φPn,max = 0.65 × 0.80 P0 (Section 22.4.2.1)".to_string()),
                ("3. Balanced Point", format!("{:.1} kN", pm_report.cardinal.balanced.pn_kn), format!("{:.1} kNm", pm_report.cardinal.balanced.mn_knm), format!("{:.3}", pm_report.cardinal.balanced.phi), format!("εt = {:.4} = εy (Simultaneous yield)", pm_report.cardinal.balanced.eps_t)),
                ("4. Pure Flexure (Pn = 0)", "0.0 kN".to_string(), format!("{:.1} kNm", pm_report.cardinal.pure_flexure.mn_knm), format!("{:.2}", pm_report.cardinal.pure_flexure.phi), format!("φM0 = {:.1} kNm (Tension-controlled)", pm_report.cardinal.pure_flexure.phi_mn_knm)),
                ("5. Pure Tension (Pnt)", format!("{:.1} kN", pm_report.cardinal.pnt_kn), "0.0 kNm".to_string(), "0.90".to_string(), format!("φPnt = 0.90 × (-fy Ast) = {:.1} kN", pm_report.cardinal.phi_pnt_kn)),
            ];

            ScrollArea::horizontal().id_salt("concrete_pm_cardinal_scroll").show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Cardinal Limit State").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(140.0);
                    ui.label(RichText::new("Axial Pn").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(45.0);
                    ui.label(RichText::new("Moment Mn").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(45.0);
                    ui.label(RichText::new("Factor φ").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(35.0);
                    ui.label(RichText::new("SNI / ACI Code Notes & Technical Details").size(11.0).strong().color(theme.text_secondary));
                });
                ui.separator();

                for (state_label, pn, mn, phi, code_note) in cardinals {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(state_label).size(11.5).strong().color(theme.accent_blue));
                        ui.add_space(30.0);
                        ui.label(RichText::new(pn).size(11.5).color(theme.text_primary));
                        ui.add_space(45.0);
                        ui.label(RichText::new(mn).size(11.5).color(theme.text_primary));
                        ui.add_space(45.0);
                        ui.label(RichText::new(phi).size(11.5).strong().color(theme.accent_green));
                        ui.add_space(45.0);
                        ui.label(RichText::new(code_note).size(11.0).color(theme.text_muted));
                    });
                    ui.separator();
                }
            });
        });

        ui.add_space(16.0);

        // 6. 3D Biaxial P-Mx-My Interaction Surface & PCA Load Contour (SNI 2847:2019 / Bresler)
        theme.card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("📐 3D Biaxial P-Mx-My Interaction Surface & PCA Load Contour Envelope (SNI 2847:2019 / ACI 318-19):").size(12.5).strong().color(theme.text_primary));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let bresler_lbl = format!("Bresler Reciprocal φPn: {:.1} kN • α = {:.3}", biaxial_rep.bresler_phi_pn_kn, biaxial_rep.alpha_exponent);
                    ui.label(RichText::new(bresler_lbl).size(11.0).color(theme.accent_blue));
                });
            });
            ui.add_space(8.0);

            // Live vector slice of Mx - My contour at design axial Pu
            let desired = Vec2::new(ui.available_width(), 160.0);
            let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
            let painter = ui.painter_at(rect);
            painter.rect_filled(rect, CornerRadius::same(6), Color32::from_rgb(12, 16, 23));

            let center_x = rect.center().x;
            let center_y = rect.center().y;
            let max_m = (biaxial_rep.phi_mnx_knm.max(biaxial_rep.phi_mny_knm) * 1.25).max(50.0);
            let radius_px = (rect.height() * 0.42).min(rect.width() * 0.28);

            // Draw axis lines
            painter.line_segment(
                [Pos2::new(center_x - radius_px * 1.15, center_y), Pos2::new(center_x + radius_px * 1.15, center_y)],
                Stroke::new(1.0, Color32::from_rgb(60, 65, 75)),
            );
            painter.line_segment(
                [Pos2::new(center_x, center_y - radius_px * 1.15), Pos2::new(center_x, center_y + radius_px * 1.15)],
                Stroke::new(1.0, Color32::from_rgb(60, 65, 75)),
            );

            // Draw PCA Contour Polygon
            let mut contour_pts = Vec::with_capacity(biaxial_rep.contour_envelope.len());
            for &(mx, my) in &biaxial_rep.contour_envelope {
                let px = center_x + (mx as f32 / max_m as f32) * radius_px;
                let py = center_y - (my as f32 / max_m as f32) * radius_px;
                contour_pts.push(Pos2::new(px, py));
            }
            if contour_pts.len() > 2 {
                for w in contour_pts.windows(2) {
                    painter.line_segment([w[0], w[1]], Stroke::new(2.0, theme.accent_blue));
                }
            }

            // Draw Demand Point (Mux, Muy)
            let dem_x = center_x + (self.concrete_demand_mu_knm / max_m as f32) * radius_px;
            let dem_y = center_y - (self.concrete_demand_muy_knm / max_m as f32) * radius_px;
            let dem_pos = Pos2::new(dem_x, dem_y);
            let dem_col = if biaxial_rep.is_safe { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };

            painter.circle_filled(dem_pos, 7.0, dem_col.gamma_multiply(0.3));
            painter.circle_filled(dem_pos, 4.0, dem_col);
            painter.circle_stroke(dem_pos, 4.0, Stroke::new(1.5, Color32::WHITE));

            painter.text(
                dem_pos + Vec2::new(8.0, -8.0),
                Align2::LEFT_BOTTOM,
                format!("Demand (Mux: {:.0}, Muy: {:.0})", self.concrete_demand_mu_knm, self.concrete_demand_muy_knm),
                egui::FontId::proportional(10.5),
                dem_col,
            );

            // Labels for φMnx and φMny
            painter.text(
                Pos2::new(center_x + (biaxial_rep.phi_mnx_knm as f32 / max_m as f32) * radius_px + 6.0, center_y),
                Align2::LEFT_CENTER,
                format!("φMnx = {:.1} kNm", biaxial_rep.phi_mnx_knm),
                egui::FontId::proportional(9.5),
                theme.text_secondary,
            );
            painter.text(
                Pos2::new(center_x, center_y - (biaxial_rep.phi_mny_knm as f32 / max_m as f32) * radius_px - 8.0),
                Align2::CENTER_BOTTOM,
                format!("φMny = {:.1} kNm", biaxial_rep.phi_mny_knm),
                egui::FontId::proportional(9.5),
                theme.text_secondary,
            );

            // Information summary footer
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(format!("Biaxial Demand-Capacity Ratio (DCR): {:.3}", biaxial_rep.biaxial_dcr)).size(11.5).strong().color(dem_col));
                ui.label(RichText::new("•").size(11.0).color(theme.text_muted));
                ui.label(RichText::new(format!("PCA Load Contour Formula: (Mux/φMnx)^{{α}} + (Muy/φMny)^{{α}} ≤ 1.0 (α = {:.2})", biaxial_rep.alpha_exponent)).size(11.0).color(theme.text_secondary));
                ui.label(RichText::new("•").size(11.0).color(theme.text_muted));
                ui.label(RichText::new(format!("Bresler Reciprocal φPn = {:.1} kN (Pu = {:.1} kN)", biaxial_rep.bresler_phi_pn_kn, biaxial_rep.pu_kn)).size(11.0).color(theme.accent_blue));
            });
        });
    }

    pub(super) fn render_steel_connection(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let rep = match self.conn_type {
            crate::core::ConnectionType::ShearTabSinglePlate => {
                crate::core::ConnectionEngine::evaluate_shear_tab(
                    "Beam-to-Girder Shear Tab Connection",
                    self.conn_bolt_grade,
                    self.conn_bolt_dia_mm as f64,
                    self.conn_bolt_rows,
                    self.conn_plate_thick_mm as f64,
                    110.0,
                    250.0,
                    400.0,
                    8.0,
                    self.conn_demand_vu_kn as f64,
                    0.0,
                )
            }
            _ => {
                crate::core::ConnectionEngine::evaluate_extended_end_plate(
                    "Beam-to-Column 4E Extended End-Plate Moment Connection",
                    self.conn_bolt_grade,
                    self.conn_bolt_dia_mm as f64,
                    self.conn_plate_thick_mm as f64,
                    220.0,
                    345.0,
                    450.0,
                    550.0,
                    200.0,
                    16.0,
                    10.0,
                    350.0,
                    19.0,
                    12.0,
                    self.conn_demand_mu_knm as f64,
                    self.conn_demand_vu_kn as f64,
                )
            }
        };

        // 1. Top Summary Metric Cards (4 Columns)
        ui_kit::responsive_grid_n(ui, 4, 190.0, |ui, col_idx| match col_idx {
            0 => {
                let status_color = if rep.is_compliant { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
                    self.metric_card(
                        ui,
                        theme,
                        "GOVERNING DCR",
                        &format!("{:.3}", rep.governing_dcr),
                        if rep.is_compliant { "AISC 360-16 Compliant" } else { "Overstress Detected" },
                        status_color,
                    );
            }
            1 => {
                self.metric_card(
                        ui,
                        theme,
                        "CRITICAL LIMIT STATE",
                        &rep.governing_limit_state,
                        &format!("Applied Vu: {:.0} kN • Mu: {:.0} kNm", rep.applied_shear_vu_kn, rep.applied_moment_mu_knm),
                        theme.accent_blue,
                    );
            }
            2 => {
                self.metric_card(
                        ui,
                        theme,
                        "FASTENER CONFIGURATION",
                        &format!("{}x D{:.0}mm", rep.bolt_count_n, rep.bolt_diameter_mm),
                        rep.bolt_grade.label(),
                        theme.accent_orange,
                    );
            }
            _ => {
                let pz_summary = if let Some(ref pz) = rep.panel_zone {
                        if pz.doubler_plate_required {
                            format!("Doubler Req (t = {:.0}mm)", pz.recommended_doubler_thickness_mm)
                        } else {
                            format!("Panel Zone DCR: {:.3} (OK)", pz.panel_zone_dcr)
                        }
                    } else {
                        format!("Plate t = {:.0} mm (Fy = {:.0})", rep.plate_thickness_mm, rep.plate_fy_mpa)
                    };
                    self.metric_card(
                        ui,
                        theme,
                        "PANEL ZONE / PLATE",
                        &pz_summary,
                        "AISC 360-16 Section J10.6",
                        theme.accent_green,
                    );
            }
        });

        ui.add_space(12.0);

        // 2. Control Toolbar
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(8))
            .inner_margin(Margin::same(10))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Typology:").size(11.0).strong().color(theme.text_secondary));
                    let types = [
                        (crate::core::ConnectionType::ExtendedEndPlate4E, "4E Moment End-Plate"),
                        (crate::core::ConnectionType::ShearTabSinglePlate, "Shear Tab (Pinned)"),
                    ];
                    for (ct, lbl) in types {
                        let is_sel = self.conn_type == ct;
                        let btn = egui::Button::new(RichText::new(lbl).size(11.0).color(if is_sel { Color32::WHITE } else { theme.text_secondary }))
                            .fill(if is_sel { theme.accent_blue } else { theme.bg_canvas });
                        if ui.add(btn).clicked() {
                            self.conn_type = ct;
                        }
                    }

                    ui.add_space(12.0);
                    ui.label(RichText::new("Bolt Grade:").size(11.0).strong().color(theme.text_secondary));
                    let grades = [
                        (crate::core::BoltGrade::A325N, "A325-N"),
                        (crate::core::BoltGrade::A325X, "A325-X"),
                        (crate::core::BoltGrade::A490X, "A490-X"),
                        (crate::core::BoltGrade::Grade10_9, "Gr 10.9"),
                    ];
                    for (bg, lbl) in grades {
                        let is_sel = self.conn_bolt_grade == bg;
                        let btn = egui::Button::new(RichText::new(lbl).size(11.0).color(if is_sel { Color32::WHITE } else { theme.text_secondary }))
                            .fill(if is_sel { theme.accent_blue } else { theme.bg_canvas });
                        if ui.add(btn).clicked() {
                            self.conn_bolt_grade = bg;
                        }
                    }

                    ui.add_space(12.0);
                    ui.label(RichText::new("Bolt Dia (mm):").size(11.0).color(theme.text_secondary));
                    ui.add(egui::DragValue::new(&mut self.conn_bolt_dia_mm).speed(2.0).range(16.0..=36.0));

                    ui.add_space(8.0);
                    ui.label(RichText::new("Plate t (mm):").size(11.0).color(theme.text_secondary));
                    ui.add(egui::DragValue::new(&mut self.conn_plate_thick_mm).speed(1.0).range(6.0..=45.0));

                    ui.add_space(8.0);
                    ui.label(RichText::new("Vu (kN):").size(11.0).color(theme.text_secondary));
                    ui.add(egui::DragValue::new(&mut self.conn_demand_vu_kn).speed(5.0).range(10.0..=800.0));

                    if self.conn_type != crate::core::ConnectionType::ShearTabSinglePlate {
                        ui.add_space(8.0);
                        ui.label(RichText::new("Mu (kNm):").size(11.0).color(theme.text_secondary));
                        ui.add(egui::DragValue::new(&mut self.conn_demand_mu_knm).speed(10.0).range(10.0..=1200.0));
                    }
                });
            });

        ui.add_space(12.0);

        // 3. DUAL VECTOR VISUALIZER: Left = 2D Connection Detailing Blueprint; Right = Limit States DCR Breakdown
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("📐 Connection 2D Detailing Blueprint & AISC 360-16 Limit State Breakdown").size(13.5).strong().color(theme.text_primary));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let verdict_tag = if rep.is_compliant { "✅ Connection Approved (All DCR ≤ 1.00)" } else { "⚠️ Reinforcement / Plate Upgrade Required" };
                        let verdict_col = if rep.is_compliant { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
                        ui.label(RichText::new(verdict_tag).size(11.5).strong().color(verdict_col));
                    });
                });

                ui.add_space(10.0);

                ui_kit::responsive_two_panes(ui, 720.0, |ui, pane_idx| match pane_idx {
                    0 => {
                        // LEFT: 2D Elevation Detailing
                                    ui.label(RichText::new("1. Elevation Detailing Blueprint (CAD Vector Projection)").size(11.0).strong().color(theme.text_secondary));
                                    let desired_left = Vec2::new(ui.available_width(), 220.0);
                                    let (rect_l, _) = ui.allocate_exact_size(desired_left, Sense::hover());
                                    let painter_l = ui.painter_at(rect_l);
                    }
                    _ => {
                        painter_l.rect_filled(rect_l, CornerRadius::same(6), Color32::from_rgb(12, 16, 23));

                                    let center_l = rect_l.center();

                                    if rep.connection_type == crate::core::ConnectionType::ExtendedEndPlate4E {
                                        // Extended End-Plate Drawing
                                        let col_w_px = 65.0;
                                        let col_h_px = rect_l.height() * 0.88;
                                        let col_x = rect_l.left() + 50.0;
                                        let col_rect = egui::Rect::from_min_size(Pos2::new(col_x, center_l.y - col_h_px * 0.5), Vec2::new(col_w_px, col_h_px));
                                        painter_l.rect_filled(col_rect, CornerRadius::same(2), Color32::from_rgb(45, 55, 72));
                                        painter_l.rect_stroke(col_rect, CornerRadius::same(2), Stroke::new(1.2, Color32::from_rgb(148, 163, 184)), egui::StrokeKind::Inside);
                                        painter_l.text(col_rect.center(), egui::Align2::CENTER_CENTER, "Column W14", egui::FontId::proportional(9.0), Color32::from_rgb(203, 213, 225));

                                        // End Plate
                                        let plate_w_px = (self.conn_plate_thick_mm * 1.5).clamp(12.0, 30.0);
                                        let plate_h_px = col_h_px * 0.78;
                                        let plate_x = col_rect.right();
                                        let plate_rect = egui::Rect::from_min_size(Pos2::new(plate_x, center_l.y - plate_h_px * 0.5), Vec2::new(plate_w_px, plate_h_px));
                                        painter_l.rect_filled(plate_rect, CornerRadius::same(2), Color32::from_rgb(30, 41, 59));
                                        painter_l.rect_stroke(plate_rect, CornerRadius::same(2), Stroke::new(1.5, theme.accent_blue), egui::StrokeKind::Inside);

                                        // Beam Profile
                                        let beam_h_px = plate_h_px * 0.72;
                                        let beam_w_px = rect_l.right() - plate_rect.right() - 20.0;
                                        let beam_top = center_l.y - beam_h_px * 0.5;
                                        let tf_px = 9.0;

                                        // Top Flange
                                        painter_l.rect_filled(egui::Rect::from_min_size(Pos2::new(plate_rect.right(), beam_top), Vec2::new(beam_w_px, tf_px)), CornerRadius::same(1), Color32::from_rgb(51, 65, 85));
                                        // Bottom Flange
                                        painter_l.rect_filled(egui::Rect::from_min_size(Pos2::new(plate_rect.right(), beam_top + beam_h_px - tf_px), Vec2::new(beam_w_px, tf_px)), CornerRadius::same(1), Color32::from_rgb(51, 65, 85));
                                        // Web
                                        painter_l.rect_filled(egui::Rect::from_min_size(Pos2::new(plate_rect.right(), beam_top + tf_px), Vec2::new(beam_w_px, beam_h_px - tf_px * 2.0)), CornerRadius::same(0), Color32::from_rgb(30, 41, 59));
                                        painter_l.text(Pos2::new(plate_rect.right() + beam_w_px * 0.45, center_l.y), egui::Align2::CENTER_CENTER, "Beam W24", egui::FontId::proportional(9.5), Color32::from_rgb(226, 232, 240));

                                        // 8 Bolts: 4 Tension (top), 4 Compression (bottom)
                                        let r_bolt = (self.conn_bolt_dia_mm * 0.22).clamp(3.5, 6.5);
                                        let bolt_x = col_rect.right() + plate_w_px * 0.5;

                                        let tension_y = [beam_top - 18.0, beam_top + tf_px + 14.0];
                                        for &by in &tension_y {
                                            painter_l.circle_filled(Pos2::new(bolt_x, by), r_bolt, Color32::from_rgb(239, 68, 68));
                                            painter_l.circle_stroke(Pos2::new(bolt_x, by), r_bolt, Stroke::new(1.0, Color32::WHITE));
                                        }
                                        painter_l.text(Pos2::new(bolt_x + 12.0, beam_top - 18.0), egui::Align2::LEFT_CENTER, "Tension Bolts", egui::FontId::proportional(8.5), Color32::from_rgb(239, 68, 68));

                                        let comp_y = [beam_top + beam_h_px - tf_px - 14.0, beam_top + beam_h_px + 18.0];
                                        for &by in &comp_y {
                                            painter_l.circle_filled(Pos2::new(bolt_x, by), r_bolt, theme.accent_blue);
                                            painter_l.circle_stroke(Pos2::new(bolt_x, by), r_bolt, Stroke::new(1.0, Color32::WHITE));
                                        }
                                        painter_l.text(Pos2::new(bolt_x + 12.0, beam_top + beam_h_px + 18.0), egui::Align2::LEFT_CENTER, "Shear Bolts", egui::FontId::proportional(8.5), theme.accent_blue);

                                        // Welds (Beads)
                                        painter_l.line_segment([Pos2::new(plate_rect.right(), beam_top - 2.0), Pos2::new(plate_rect.right() + 14.0, beam_top - 2.0)], Stroke::new(2.5, Color32::from_rgb(245, 158, 11)));
                                        painter_l.line_segment([Pos2::new(plate_rect.right(), beam_top + beam_h_px + 2.0), Pos2::new(plate_rect.right() + 14.0, beam_top + beam_h_px + 2.0)], Stroke::new(2.5, Color32::from_rgb(245, 158, 11)));
                                    } else {
                                        // Single-Plate Shear Tab Drawing
                                        let sup_w_px = 60.0;
                                        let sup_rect = egui::Rect::from_min_size(Pos2::new(rect_l.left() + 45.0, rect_l.top() + 20.0), Vec2::new(sup_w_px, rect_l.height() - 40.0));
                                        painter_l.rect_filled(sup_rect, CornerRadius::same(2), Color32::from_rgb(45, 55, 72));
                                        painter_l.rect_stroke(sup_rect, CornerRadius::same(2), Stroke::new(1.2, Color32::from_rgb(148, 163, 184)), egui::StrokeKind::Inside);
                                        painter_l.text(sup_rect.center(), egui::Align2::CENTER_CENTER, "Support", egui::FontId::proportional(9.0), Color32::from_rgb(203, 213, 225));

                                        // Shear Tab Plate
                                        let tab_w = 65.0;
                                        let tab_h = ((rep.bolt_count_n as f32 - 1.0) * 35.0 + 50.0).clamp(70.0, rect_l.height() - 50.0);
                                        let tab_rect = egui::Rect::from_center_size(Pos2::new(sup_rect.right() + tab_w * 0.5, center_l.y), Vec2::new(tab_w, tab_h));
                                        painter_l.rect_filled(tab_rect, CornerRadius::same(2), Color32::from_rgb(30, 41, 59));
                                        painter_l.rect_stroke(tab_rect, CornerRadius::same(2), Stroke::new(1.4, theme.accent_blue), egui::StrokeKind::Inside);

                                        // Beam Web
                                        let beam_w = rect_l.right() - tab_rect.left() - 25.0;
                                        let beam_h = tab_h + 40.0;
                                        let beam_rect = egui::Rect::from_min_size(Pos2::new(tab_rect.left() + 15.0, center_l.y - beam_h * 0.5), Vec2::new(beam_w, beam_h));
                                        painter_l.rect_stroke(beam_rect, CornerRadius::same(2), Stroke::new(1.0, Color32::from_rgb(100, 116, 139)), egui::StrokeKind::Inside);

                                        // Double Fillet Welds along tab left
                                        painter_l.line_segment([Pos2::new(sup_rect.right(), tab_rect.top()), Pos2::new(sup_rect.right(), tab_rect.bottom())], Stroke::new(3.0, Color32::from_rgb(245, 158, 11)));

                                        // Vertical Bolt Line
                                        let r_b = (self.conn_bolt_dia_mm * 0.22).clamp(4.0, 7.0);
                                        let bx = tab_rect.left() + tab_w * 0.65;
                                        let pitch = if rep.bolt_count_n > 1 { (tab_h - 40.0) / (rep.bolt_count_n as f32 - 1.0) } else { 0.0 };
                                        for i in 0..rep.bolt_count_n {
                                            let by = tab_rect.top() + 20.0 + (i as f32) * pitch;
                                            painter_l.circle_filled(Pos2::new(bx, by), r_b, Color32::from_rgb(52, 211, 153));
                                            painter_l.circle_stroke(Pos2::new(bx, by), r_b, Stroke::new(1.0, Color32::WHITE));
                                        }
                                    }

                                    // RIGHT: Limit State DCR Breakdown Bar Chart
                                    ui.label(RichText::new("2. AISC 360-16 Limit State DCR Utilization Bars").size(11.0).strong().color(theme.text_secondary));
                                    let desired_right = Vec2::new(ui.available_width(), 220.0);
                                    let (rect_r, _) = ui.allocate_exact_size(desired_right, Sense::hover());
                                    let painter_r = ui.painter_at(rect_r);
                                    painter_r.rect_filled(rect_r, CornerRadius::same(6), Color32::from_rgb(12, 16, 23));

                                    let bar_h = 16.0;
                                    let bar_gap = 6.0;
                                    let max_plot_w = (rect_r.width() - 175.0).max(80.0);

                                    for (idx, ls) in rep.limit_states.iter().enumerate().take(7) {
                                        let y_pos = rect_r.top() + 12.0 + (idx as f32) * (bar_h + bar_gap);

                                        // Limit state name
                                        let short_name = if ls.limit_state_name.len() > 22 {
                                            format!("{}...", &ls.limit_state_name[..20])
                                        } else {
                                            ls.limit_state_name.clone()
                                        };
                                        painter_r.text(Pos2::new(rect_r.left() + 8.0, y_pos + bar_h * 0.5), egui::Align2::LEFT_CENTER, short_name, egui::FontId::proportional(9.0), Color32::from_rgb(203, 213, 225));

                                        // Bar background
                                        let bar_x = rect_r.left() + 130.0;
                                        let bar_bg = egui::Rect::from_min_size(Pos2::new(bar_x, y_pos), Vec2::new(max_plot_w, bar_h));
                                        painter_r.rect_filled(bar_bg, CornerRadius::same(3), Color32::from_rgb(30, 41, 59));

                                        // Active fill bar
                                        let fill_w = ((ls.demand_capacity_ratio as f32 / 1.0) * max_plot_w).clamp(2.0, max_plot_w * 1.15);
                                        let bar_color = if ls.demand_capacity_ratio <= 0.75 {
                                            Color32::from_rgb(16, 185, 129)
                                        } else if ls.demand_capacity_ratio <= 1.00 {
                                            Color32::from_rgb(245, 158, 11)
                                        } else {
                                            Color32::from_rgb(239, 68, 68)
                                        };
                                        let fill_rect = egui::Rect::from_min_size(Pos2::new(bar_x, y_pos), Vec2::new(fill_w, bar_h));
                                        painter_r.rect_filled(fill_rect, CornerRadius::same(3), bar_color);

                                        // DCR label
                                        painter_r.text(Pos2::new(bar_x + fill_w + 6.0, y_pos + bar_h * 0.5), egui::Align2::LEFT_CENTER, format!("{:.3}", ls.demand_capacity_ratio), egui::FontId::proportional(9.0), bar_color);
                                    }
                    }
                });
            });

        ui.add_space(14.0);

        // 4. Limit States Verification Matrix Table
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(14))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("📋 AISC 360-16 / SNI 1729:2020 Connection Limit States Verification Table:").size(12.5).strong().color(theme.text_primary));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new("Standard: ANSI/AISC 360-16 Specification for Structural Steel Buildings").size(11.0).color(theme.text_muted));
                    });
                });
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Limit State Description").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(130.0);
                    ui.label(RichText::new("Code Ref").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(50.0);
                    ui.label(RichText::new("Nominal Rn").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(45.0);
                    ui.label(RichText::new("Factor φ").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(40.0);
                    ui.label(RichText::new("Design φRn").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(45.0);
                    ui.label(RichText::new("Demand").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(45.0);
                    ui.label(RichText::new("DCR").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(45.0);
                    ui.label(RichText::new("Status").size(11.0).strong().color(theme.text_secondary));
                });
                ui.separator();

                for ls in &rep.limit_states {
                    let dcr_col = if ls.demand_capacity_ratio <= 0.75 {
                        theme.accent_green
                    } else if ls.demand_capacity_ratio <= 1.00 {
                        theme.accent_orange
                    } else {
                        Color32::from_rgb(239, 68, 68)
                    };

                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&ls.limit_state_name).size(11.0).strong().color(theme.accent_blue));
                        ui.add_space(20.0);
                        ui.label(RichText::new(&ls.standard_code_ref).size(10.5).color(theme.text_muted));
                        ui.add_space(30.0);
                        ui.label(RichText::new(format!("{:7.1}", ls.nominal_capacity)).size(11.0).color(theme.text_primary));
                        ui.add_space(45.0);
                        ui.label(RichText::new(format!("{:.2}", ls.phi_factor)).size(11.0).color(theme.text_secondary));
                        ui.add_space(45.0);
                        ui.label(RichText::new(format!("{:7.1}", ls.design_capacity)).size(11.0).color(theme.accent_green));
                        ui.add_space(40.0);
                        ui.label(RichText::new(format!("{:7.1}", ls.factored_demand)).size(11.0).color(theme.text_primary));
                        ui.add_space(40.0);
                        ui.label(RichText::new(format!("{:.3}", ls.demand_capacity_ratio)).size(11.0).strong().color(dcr_col));
                        ui.add_space(45.0);
                        ui.label(RichText::new(if ls.is_safe { "🟢 PASS" } else { "🔴 FAIL" }).size(11.0).strong().color(dcr_col));
                    });
                    ui.separator();
                }
            });

        if let Some(ref pz) = rep.panel_zone {
            ui.add_space(14.0);
            Frame::new()
                .fill(Color32::from_rgb(17, 24, 39))
                .corner_radius(CornerRadius::same(8))
                .inner_margin(Margin::same(12))
                .stroke(Stroke::new(1.0, Color32::from_rgb(59, 130, 246)))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("🔬 Column Panel Zone Shear & Stiffener Audit (AISC 360-16 Section J10):").size(12.5).strong().color(Color32::WHITE));
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let badge = if pz.doubler_plate_required { "⚠️ DOUBLER PLATE REQUIRED" } else { "🟢 WEB THICKNESS ADEQUATE" };
                            let b_color = if pz.doubler_plate_required { Color32::from_rgb(245, 158, 11) } else { theme.accent_green };
                            ui.label(RichText::new(badge).size(11.0).strong().color(b_color));
                        });
                    });
                    ui.add_space(6.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(format!("Panel Zone Shear Vpz: {:.1} kN", pz.applied_shear_vpz_kn)).size(11.0).color(theme.text_secondary));
                        ui.add_space(14.0);
                        ui.label(RichText::new(format!("Shear Capacity φVn: {:.1} kN", pz.design_shear_capacity_phi_vn_kn)).size(11.0).strong().color(theme.accent_blue));
                        ui.add_space(14.0);
                        ui.label(RichText::new(format!("Web Yielding φRn: {:.1} kN", pz.web_local_yielding_phi_rn_kn)).size(11.0).color(theme.text_secondary));
                        ui.add_space(14.0);
                        ui.label(RichText::new(format!("Web Crippling φRn: {:.1} kN", pz.web_crippling_phi_rn_kn)).size(11.0).color(theme.text_secondary));
                        ui.add_space(14.0);
                        ui.label(RichText::new(if pz.continuity_plates_required { "Continuity Plates: REQUIRED" } else { "Continuity Plates: None" }).size(11.0).strong().color(if pz.continuity_plates_required { Color32::from_rgb(245, 158, 11) } else { theme.accent_green }));
                    });
                });
        }
    }

    pub(super) fn render_wind_engineering(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let rep = crate::core::WindEngine::run_wind_analysis(
            "EigenGraph Aerodynamic MWFRS Assessment",
            self.wind_v_ms as f64,
            self.wind_exposure,
            self.wind_risk,
            self.wind_enclosure,
            self.wind_building_height_m as f64,
            self.wind_building_width_m as f64,
            self.wind_building_length_m as f64,
            self.wind_num_stories,
            self.wind_t1_s as f64,
            450.0, // Reference seismic base shear
        );

        // 1. Metric Cards Header
        ui_kit::responsive_grid_n(ui, 4, 190.0, |ui, col_idx| match col_idx {
            0 => {
                self.metric_card(
                        ui,
                        theme,
                        "BASIC WIND SPEED (V)",
                        &format!("{:.1} m/s", rep.basic_wind_speed_v_ms),
                        &format!("{:.0} mph • ASCE 7-22", rep.basic_wind_speed_mph),
                        theme.accent_blue,
                    );
            }
            1 => {
                self.metric_card(
                        ui,
                        theme,
                        "MEAN ROOF PRESSURE (qh)",
                        &format!("{:.0} Pa", rep.qh_mean_roof_velocity_pressure_pa),
                        &format!("{:.2} kPa • qz(h)", rep.qh_mean_roof_velocity_pressure_pa / 1000.0),
                        theme.accent_orange,
                    );
            }
            2 => {
                self.metric_card(
                        ui,
                        theme,
                        "WIND BASE SHEAR (Vw)",
                        &format!("{:.1} kN", rep.total_base_shear_wind_kn),
                        &format!("OTM: {:.0} kN·m", rep.total_overturning_moment_wind_knm),
                        theme.accent_green,
                    );
            }
            _ => {
                let comfort_col = if rep.comfort_audit.is_comfort_acceptable {
                        theme.accent_green
                    } else {
                        Color32::from_rgb(239, 68, 68)
                    };
                    let comfort_tag = if rep.comfort_audit.is_comfort_acceptable {
                        "ISO 10137 OK (≤ 20 mg)"
                    } else {
                        "EXCEEDS LIMIT (> 20 mg)"
                    };
                    self.metric_card(
                        ui,
                        theme,
                        "HUMAN COMFORT ACCEL",
                        &format!("{:.2} mg", rep.comfort_audit.resultant_acceleration_mg),
                        comfort_tag,
                        comfort_col,
                    );
            }
        });

        ui.add_space(12.0);

        // 2. Control Toolbar
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(8))
            .inner_margin(Margin::same(10))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Wind Speed (V):").size(11.0).strong().color(theme.text_secondary));
                    ui.add(egui::DragValue::new(&mut self.wind_v_ms).speed(0.5).range(20.0..=85.0).suffix(" m/s"));

                    ui.add_space(10.0);
                    ui.label(RichText::new("Exposure:").size(11.0).strong().color(theme.text_secondary));
                    let exposures = [
                        (crate::core::ExposureCategory::ExposureB, "Exposure B (Suburban)"),
                        (crate::core::ExposureCategory::ExposureC, "Exposure C (Open)"),
                        (crate::core::ExposureCategory::ExposureD, "Exposure D (Coastal)"),
                    ];
                    for (exp, lbl) in exposures {
                        let is_sel = self.wind_exposure == exp;
                        let btn = egui::Button::new(RichText::new(lbl).size(11.0).color(if is_sel { Color32::WHITE } else { theme.text_secondary }))
                            .fill(if is_sel { theme.accent_blue } else { theme.bg_canvas });
                        if ui.add(btn).clicked() {
                            self.wind_exposure = exp;
                        }
                    }

                    ui.add_space(10.0);
                    ui.label(RichText::new("Enclosure:").size(11.0).strong().color(theme.text_secondary));
                    let enclosures = [
                        (crate::core::EnclosureClassification::Enclosed, "Enclosed"),
                        (crate::core::EnclosureClassification::PartiallyEnclosed, "Partially Enclosed"),
                        (crate::core::EnclosureClassification::Open, "Open"),
                    ];
                    for (enc, lbl) in enclosures {
                        let is_sel = self.wind_enclosure == enc;
                        let btn = egui::Button::new(RichText::new(lbl).size(11.0).color(if is_sel { Color32::WHITE } else { theme.text_secondary }))
                            .fill(if is_sel { theme.accent_blue } else { theme.bg_canvas });
                        if ui.add(btn).clicked() {
                            self.wind_enclosure = enc;
                        }
                    }
                });

                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Dimensions (H x B x L):").size(11.0).strong().color(theme.text_secondary));
                    ui.add(egui::DragValue::new(&mut self.wind_building_height_m).speed(1.0).range(6.0..=250.0).suffix(" m H"));
                    ui.add(egui::DragValue::new(&mut self.wind_building_width_m).speed(1.0).range(6.0..=120.0).suffix(" m B"));
                    ui.add(egui::DragValue::new(&mut self.wind_building_length_m).speed(1.0).range(6.0..=120.0).suffix(" m L"));

                    ui.add_space(10.0);
                    ui.label(RichText::new("Stories:").size(11.0).strong().color(theme.text_secondary));
                    ui.add(egui::DragValue::new(&mut self.wind_num_stories).speed(1.0).range(2..=80));

                    ui.add_space(10.0);
                    ui.label(RichText::new("Period T1:").size(11.0).strong().color(theme.text_secondary));
                    ui.add(egui::DragValue::new(&mut self.wind_t1_s).speed(0.05).range(0.1..=8.0).suffix(" s"));
                });
            });

        ui.add_space(12.0);

        // 3. DUAL VECTOR VISUALIZER
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("🌪️ ASCE 7-22 Directional MWFRS Pressure Elevation & Dynamic Comfort Analysis").size(13.5).strong().color(theme.text_primary));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let gov_text = if rep.wind_governs_lateral_system {
                            "⚠️ WIND GOVERNS LATERAL SYSTEM (Vw > Vs)"
                        } else {
                            "🟢 SEISMIC GOVERNS LATERAL SYSTEM (Vs > Vw)"
                        };
                        let gov_col = if rep.wind_governs_lateral_system {
                            Color32::from_rgb(245, 158, 11)
                        } else {
                            theme.accent_green
                        };
                        ui.label(RichText::new(gov_text).size(11.5).strong().color(gov_col));
                    });
                });

                ui.add_space(10.0);

                ui_kit::responsive_two_panes(ui, 720.0, |ui, pane_idx| match pane_idx {
                    0 => {
                        // LEFT: Elevation Velocity Pressure Profile
                                    ui.label(RichText::new("1. Elevation Velocity Pressure & Windward Net Vector Profile qz(z)").size(11.0).strong().color(theme.text_secondary));
                                    let desired_left = Vec2::new(ui.available_width(), 220.0);
                                    let (rect_l, _) = ui.allocate_exact_size(desired_left, Sense::hover());
                                    let painter_l = ui.painter_at(rect_l);
                    }
                    _ => {
                        painter_l.rect_filled(rect_l, CornerRadius::same(6), Color32::from_rgb(12, 16, 23));

                                    // Building silhouette
                                    let b_width_px = 75.0;
                                    let b_height_px = rect_l.height() * 0.82;
                                    let b_left = rect_l.left() + 110.0;
                                    let b_bottom = rect_l.bottom() - 15.0;
                                    let b_top = b_bottom - b_height_px;
                                    let b_rect = egui::Rect::from_min_max(Pos2::new(b_left, b_top), Pos2::new(b_left + b_width_px, b_bottom));

                                    // Draw building
                                    painter_l.rect_filled(b_rect, CornerRadius::same(3), Color32::from_rgb(30, 41, 59));
                                    painter_l.rect_stroke(b_rect, CornerRadius::same(3), Stroke::new(1.2, Color32::from_rgb(100, 116, 139)), egui::StrokeKind::Inside);

                                    // Ground line
                                    painter_l.line_segment(
                                        [Pos2::new(rect_l.left() + 10.0, b_bottom), Pos2::new(rect_l.right() - 10.0, b_bottom)],
                                        Stroke::new(2.0, Color32::from_rgb(71, 85, 105)),
                                    );
                                    painter_l.text(Pos2::new(rect_l.left() + 15.0, b_bottom + 8.0), egui::Align2::LEFT_TOP, "Ground El. ±0.00", egui::FontId::proportional(9.0), Color32::from_rgb(148, 163, 184));

                                    // Floor lines & pressure arrows
                                    let n_vis = rep.story_levels.len().min(12);
                                    let max_p = rep.story_levels.iter().map(|s| s.net_mwfrs_pressure_pa).fold(1.0f64, |a, b| a.max(b));
                                    for (idx, story) in rep.story_levels.iter().enumerate().take(n_vis) {
                                        let frac = (idx + 1) as f32 / n_vis as f32;
                                        let floor_y = b_bottom - frac * b_height_px;

                                        // Floor separator in building
                                        painter_l.line_segment(
                                            [Pos2::new(b_left, floor_y), Pos2::new(b_left + b_width_px, floor_y)],
                                            Stroke::new(0.8, Color32::from_rgb(51, 65, 85)),
                                        );

                                        // Windward arrow coming from left
                                        let arrow_len = ((story.net_mwfrs_pressure_pa / max_p) * 75.0).clamp(12.0, 85.0) as f32;
                                        let arrow_start = Pos2::new(b_left - arrow_len, floor_y);
                                        let arrow_end = Pos2::new(b_left - 2.0, floor_y);

                                        painter_l.line_segment([arrow_start, arrow_end], Stroke::new(1.5, Color32::from_rgb(56, 189, 248)));
                                        // Arrow head
                                        painter_l.line_segment([Pos2::new(arrow_end.x - 4.0, arrow_end.y - 3.0), arrow_end], Stroke::new(1.5, Color32::from_rgb(56, 189, 248)));
                                        painter_l.line_segment([Pos2::new(arrow_end.x - 4.0, arrow_end.y + 3.0), arrow_end], Stroke::new(1.5, Color32::from_rgb(56, 189, 248)));
                                    }

                                    // Height and Roof label
                                    painter_l.text(Pos2::new(b_left + b_width_px * 0.5, b_top - 8.0), egui::Align2::CENTER_BOTTOM, format!("Roof H = {:.0}m (qh = {:.0} Pa)", rep.building_height_h_m, rep.qh_mean_roof_velocity_pressure_pa), egui::FontId::proportional(9.5), Color32::from_rgb(226, 232, 240));

                                    // RIGHT: Dynamic Comfort Acceleration & Shear Envelope Comparison
                                    ui.label(RichText::new("2. Human Comfort Dynamic Acceleration & Wind vs Seismic Envelope").size(11.0).strong().color(theme.text_secondary));
                                    let desired_right = Vec2::new(ui.available_width(), 220.0);
                                    let (rect_r, _) = ui.allocate_exact_size(desired_right, Sense::hover());
                                    let painter_r = ui.painter_at(rect_r);
                                    painter_r.rect_filled(rect_r, CornerRadius::same(6), Color32::from_rgb(12, 16, 23));

                                    // Draw ISO 10137 Acceleration Bar Chart
                                    let chart_left = rect_r.left() + 16.0;
                                    let chart_top = rect_r.top() + 24.0;
                                    let bar_w = rect_r.width() - 32.0;

                                    let acc = &rep.comfort_audit;
                                    let max_acc_scale = 35.0f32; // mg scale

                                    // Along-wind Bar
                                    let y1 = chart_top + 10.0;
                                    painter_r.text(Pos2::new(chart_left, y1 - 2.0), egui::Align2::LEFT_BOTTOM, format!("Along-Wind Acceleration: {:.2} mg", acc.peak_acceleration_along_wind_mg), egui::FontId::proportional(10.0), Color32::from_rgb(148, 163, 184));
                                    let w1 = ((acc.peak_acceleration_along_wind_mg as f32 / max_acc_scale) * bar_w).clamp(4.0, bar_w);
                                    painter_r.rect_filled(egui::Rect::from_min_size(Pos2::new(chart_left, y1), Vec2::new(bar_w, 12.0)), CornerRadius::same(3), Color32::from_rgb(30, 41, 59));
                                    painter_r.rect_filled(egui::Rect::from_min_size(Pos2::new(chart_left, y1), Vec2::new(w1, 12.0)), CornerRadius::same(3), Color32::from_rgb(59, 130, 246));

                                    // Cross-wind Vortex Shedding Bar
                                    let y2 = y1 + 32.0;
                                    painter_r.text(Pos2::new(chart_left, y2 - 2.0), egui::Align2::LEFT_BOTTOM, format!("Cross-Wind (Vortex Shedding): {:.2} mg", acc.peak_acceleration_cross_wind_mg), egui::FontId::proportional(10.0), Color32::from_rgb(148, 163, 184));
                                    let w2 = ((acc.peak_acceleration_cross_wind_mg as f32 / max_acc_scale) * bar_w).clamp(4.0, bar_w);
                                    painter_r.rect_filled(egui::Rect::from_min_size(Pos2::new(chart_left, y2), Vec2::new(bar_w, 12.0)), CornerRadius::same(3), Color32::from_rgb(30, 41, 59));
                                    painter_r.rect_filled(egui::Rect::from_min_size(Pos2::new(chart_left, y2), Vec2::new(w2, 12.0)), CornerRadius::same(3), Color32::from_rgb(168, 85, 247));

                                    // Resultant Bar vs Limit Line
                                    let y3 = y2 + 32.0;
                                    let res_color = if acc.is_comfort_acceptable { Color32::from_rgb(34, 197, 94) } else { Color32::from_rgb(239, 68, 68) };
                                    painter_r.text(Pos2::new(chart_left, y3 - 2.0), egui::Align2::LEFT_BOTTOM, format!("Resultant Accel: {:.2} mg (ISO 10137 limit: {:.1} mg)", acc.resultant_acceleration_mg, acc.occupant_comfort_threshold_mg), egui::FontId::proportional(10.0), res_color);
                                    let w3 = ((acc.resultant_acceleration_mg as f32 / max_acc_scale) * bar_w).clamp(4.0, bar_w);
                                    painter_r.rect_filled(egui::Rect::from_min_size(Pos2::new(chart_left, y3), Vec2::new(bar_w, 12.0)), CornerRadius::same(3), Color32::from_rgb(30, 41, 59));
                                    painter_r.rect_filled(egui::Rect::from_min_size(Pos2::new(chart_left, y3), Vec2::new(w3, 12.0)), CornerRadius::same(3), res_color);

                                    // Limit threshold vertical dotted line
                                    let x_lim = chart_left + (acc.occupant_comfort_threshold_mg as f32 / max_acc_scale) * bar_w;
                                    painter_r.line_segment([Pos2::new(x_lim, chart_top), Pos2::new(x_lim, y3 + 14.0)], Stroke::new(1.5, Color32::from_rgb(245, 158, 11)));

                                    // Base Shear Comparison: Wind vs Seismic
                                    let y4 = y3 + 42.0;
                                    painter_r.text(Pos2::new(chart_left, y4 - 2.0), egui::Align2::LEFT_BOTTOM, format!("Base Shear Comparison: Wind Vw = {:.0} kN vs Seismic Vs = {:.0} kN", rep.total_base_shear_wind_kn, rep.seismic_base_shear_ref_kn), egui::FontId::proportional(10.0), Color32::from_rgb(226, 232, 240));

                                    let max_v = (rep.total_base_shear_wind_kn.max(rep.seismic_base_shear_ref_kn) * 1.25) as f32;
                                    let w_wind = ((rep.total_base_shear_wind_kn as f32 / max_v) * bar_w).clamp(4.0, bar_w);
                                    let w_seis = ((rep.seismic_base_shear_ref_kn as f32 / max_v) * bar_w).clamp(4.0, bar_w);

                                    // Dual bar
                                    painter_r.rect_filled(egui::Rect::from_min_size(Pos2::new(chart_left, y4), Vec2::new(w_wind, 9.0)), CornerRadius::same(2), Color32::from_rgb(56, 189, 248));
                                    painter_r.rect_filled(egui::Rect::from_min_size(Pos2::new(chart_left, y4 + 11.0), Vec2::new(w_seis, 9.0)), CornerRadius::same(2), Color32::from_rgb(249, 115, 22));

                                    painter_r.text(Pos2::new(chart_left + w_wind + 6.0, y4 + 4.0), egui::Align2::LEFT_CENTER, "Wind (Vw)", egui::FontId::proportional(8.5), Color32::from_rgb(56, 189, 248));
                                    painter_r.text(Pos2::new(chart_left + w_seis + 6.0, y4 + 15.0), egui::Align2::LEFT_CENTER, "Seismic (Vs)", egui::FontId::proportional(8.5), Color32::from_rgb(249, 115, 22));
                    }
                });
            });

        ui.add_space(14.0);

        // 4. Story Wind Loading Schedule Table
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(8))
            .inner_margin(Margin::same(12))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("📊 Story-by-Story ASCE 7-22 Directional MWFRS Wind Force Schedule").size(12.5).strong().color(theme.text_primary));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new(format!("Total Overturning Moment: {:.0} kN·m", rep.total_overturning_moment_wind_knm)).size(11.0).strong().color(theme.accent_orange));
                    });
                });

                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Story #").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(30.0);
                    ui.label(RichText::new("Elev z (m)").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(35.0);
                    ui.label(RichText::new("Kz Coef").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(40.0);
                    ui.label(RichText::new("qz (Pa)").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(45.0);
                    ui.label(RichText::new("Windward p (Pa)").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(35.0);
                    ui.label(RichText::new("Leeward p (Pa)").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(35.0);
                    ui.label(RichText::new("Net MWFRS (Pa)").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(35.0);
                    ui.label(RichText::new("Story Force (kN)").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(35.0);
                    ui.label(RichText::new("Shear Vx (kN)").size(11.0).strong().color(theme.text_secondary));
                });
                ui.separator();

                for story in rep.story_levels.iter().rev() {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("Level {:02}", story.story_num)).size(11.0).strong().color(theme.accent_blue));
                        ui.add_space(30.0);
                        ui.label(RichText::new(format!("{:5.1}", story.height_z_m)).size(11.0).color(theme.text_primary));
                        ui.add_space(55.0);
                        ui.label(RichText::new(format!("{:.3}", story.kz_exposure_coef)).size(11.0).color(theme.text_secondary));
                        ui.add_space(50.0);
                        ui.label(RichText::new(format!("{:6.0}", story.qz_velocity_pressure_pa)).size(11.0).color(theme.text_primary));
                        ui.add_space(50.0);
                        ui.label(RichText::new(format!("{:6.0}", story.windward_pressure_pa)).size(11.0).color(theme.accent_blue));
                        ui.add_space(60.0);
                        ui.label(RichText::new(format!("{:6.0}", story.leeward_pressure_pa)).size(11.0).color(theme.text_muted));
                        ui.add_space(65.0);
                        ui.label(RichText::new(format!("{:6.0}", story.net_mwfrs_pressure_pa)).size(11.0).strong().color(theme.text_primary));
                        ui.add_space(65.0);
                        ui.label(RichText::new(format!("{:6.1}", story.tributary_force_fx_kn)).size(11.0).strong().color(theme.accent_orange));
                        ui.add_space(60.0);
                        ui.label(RichText::new(format!("{:6.1}", story.cumulative_shear_vx_kn)).size(11.0).strong().color(theme.accent_green));
                    });
                    ui.separator();
                }
            });
    }

    pub(super) fn render_composite_design(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let slab = crate::core::CompositeSlabDeck {
            deck_rib_height_hr_mm: self.comp_deck_hr_mm as f64,
            deck_rib_width_wr_mm: 150.0,
            slab_thickness_above_deck_tc_mm: self.comp_slab_tc_mm as f64,
            total_slab_thickness_h_mm: (self.comp_deck_hr_mm + self.comp_slab_tc_mm) as f64,
            concrete_fc_mpa: self.comp_concrete_fc_mpa as f64,
            concrete_density_kg_m3: 2400.0,
            deck_orientation: crate::core::DeckOrientation::Perpendicular,
            studs_per_rib: 1,
        };
        let stud = crate::core::ShearStud {
            diameter_ds_mm: self.comp_stud_dia_mm as f64,
            length_hs_mm: self.comp_deck_hr_mm as f64 + 40.0,
            tensile_strength_fu_mpa: 450.0,
        };
        let rep = crate::core::CompositeBeamEngine::evaluate_composite_beam(
            "Composite Floor Framing B12",
            self.comp_span_l_m as f64,
            self.comp_beam_spacing_s_m as f64,
            "W18x35",
            self.comp_steel_depth_mm as f64,
            self.comp_steel_bf_mm as f64,
            self.comp_steel_tf_mm as f64,
            self.comp_steel_tw_mm as f64,
            6645.0,
            self.comp_steel_fy_mpa as f64,
            21200.0,
            1100.0,
            &slab,
            &stud,
            self.comp_studs_count,
            self.comp_unshored,
            self.comp_demand_mu_knm as f64,
            self.comp_live_load_kn_m as f64,
        );

        // 1. Metric Cards Header
        ui_kit::responsive_grid_n(ui, 4, 190.0, |ui, col_idx| match col_idx {
            0 => {
                let flex_col = if rep.flexure_dcr <= 1.0 { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
                    self.metric_card(
                        ui,
                        theme,
                        "FLEXURAL STRENGTH (φMn)",
                        &format!("{:.1} kN·m", rep.design_flexural_strength_phi_mn_knm),
                        &format!("DCR: {:.3} • Mu = {:.1} kNm", rep.flexure_dcr, rep.applied_factored_moment_mu_knm),
                        flex_col,
                    );
            }
            1 => {
                let comp_col = if rep.is_composite_action_adequate { theme.accent_blue } else { Color32::from_rgb(245, 158, 11) };
                    let comp_tag = if rep.is_full_composite { "Full Composite (100%)" } else { "Partial Composite (≥25%)" };
                    self.metric_card(
                        ui,
                        theme,
                        "COMPOSITE ACTION",
                        &format!("{:.1}%", rep.degree_of_composite_action_pct),
                        comp_tag,
                        comp_col,
                    );
            }
            2 => {
                self.metric_card(
                        ui,
                        theme,
                        "SHEAR STUDS (Qn)",
                        &format!("{:.1} kN/stud", rep.stud_single_shear_capacity_qn_kn),
                        &format!("Total: {} Studs • D{:.0}mm", rep.total_studs_provided, self.comp_stud_dia_mm),
                        theme.accent_orange,
                    );
            }
            _ => {
                let def_col = if rep.is_deflection_ll_ok { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
                    self.metric_card(
                        ui,
                        theme,
                        "DEFLECTION (Δ_LL)",
                        &format!("{:.1} mm", rep.live_load_deflection_delta_ll_mm),
                        &format!("Limit L/360 = {:.1} mm", rep.live_load_deflection_limit_mm),
                        def_col,
                    );
            }
        });

        ui.add_space(12.0);

        // 2. Control Toolbar
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(8))
            .inner_margin(Margin::same(10))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Span (L):").size(11.0).strong().color(theme.text_secondary));
                    ui.add(egui::DragValue::new(&mut self.comp_span_l_m).speed(0.1).range(4.0..=24.0).suffix(" m"));

                    ui.add_space(10.0);
                    ui.label(RichText::new("Spacing (S):").size(11.0).strong().color(theme.text_secondary));
                    ui.add(egui::DragValue::new(&mut self.comp_beam_spacing_s_m).speed(0.1).range(1.5..=6.0).suffix(" m"));

                    ui.add_space(10.0);
                    ui.label(RichText::new("Steel Depth:").size(11.0).strong().color(theme.text_secondary));
                    ui.add(egui::DragValue::new(&mut self.comp_steel_depth_mm).speed(5.0).range(200.0..=900.0).suffix(" mm"));

                    ui.add_space(10.0);
                    ui.label(RichText::new("Flange Width:").size(11.0).strong().color(theme.text_secondary));
                    ui.add(egui::DragValue::new(&mut self.comp_steel_bf_mm).speed(5.0).range(100.0..=400.0).suffix(" mm"));

                    ui.add_space(10.0);
                    ui.label(RichText::new("Fy:").size(11.0).strong().color(theme.text_secondary));
                    ui.add(egui::DragValue::new(&mut self.comp_steel_fy_mpa).speed(5.0).range(240.0..=500.0).suffix(" MPa"));
                });

                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Slab Thickness (tc):").size(11.0).strong().color(theme.text_secondary));
                    ui.add(egui::DragValue::new(&mut self.comp_slab_tc_mm).speed(2.0).range(50.0..=200.0).suffix(" mm"));

                    ui.add_space(10.0);
                    ui.label(RichText::new("Deck Rib Height (hr):").size(11.0).strong().color(theme.text_secondary));
                    ui.add(egui::DragValue::new(&mut self.comp_deck_hr_mm).speed(2.0).range(38.0..=100.0).suffix(" mm"));

                    ui.add_space(10.0);
                    ui.label(RichText::new("Concrete f'c:").size(11.0).strong().color(theme.text_secondary));
                    ui.add(egui::DragValue::new(&mut self.comp_concrete_fc_mpa).speed(1.0).range(20.0..=60.0).suffix(" MPa"));

                    ui.add_space(10.0);
                    ui.label(RichText::new("Studs Count:").size(11.0).strong().color(theme.text_secondary));
                    ui.add(egui::DragValue::new(&mut self.comp_studs_count).speed(2.0).range(10..=120));

                    ui.add_space(10.0);
                    ui.checkbox(&mut self.comp_unshored, "Unshored Construction");

                    ui.add_space(10.0);
                    ui.label(RichText::new("Demand Mu:").size(11.0).strong().color(theme.text_secondary));
                    ui.add(egui::DragValue::new(&mut self.comp_demand_mu_knm).speed(10.0).range(50.0..=2000.0).suffix(" kNm"));
                });
            });

        ui.add_space(12.0);

        // 3. DUAL VECTOR VISUALIZER
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("🥞 AISC 360-16 / SNI 1729:2020 Composite Section Blueprint & Stress State").size(13.5).strong().color(theme.text_primary));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let tag = if rep.is_overall_compliant { "🟢 COMPOSITE BEAM APPROVED" } else { "⚠️ SECTION UPGRADE REQUIRED" };
                        let tag_col = if rep.is_overall_compliant { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
                        ui.label(RichText::new(tag).size(11.5).strong().color(tag_col));
                    });
                });

                ui.add_space(10.0);

                ui_kit::responsive_two_panes(ui, 720.0, |ui, pane_idx| match pane_idx {
                    0 => {
                        // LEFT: 2D Section Blueprint
                                    ui.label(RichText::new("1. Cross-Section Detailing (Concrete Slab, Steel Deck Ribs & Studs)").size(11.0).strong().color(theme.text_secondary));
                                    let desired_left = Vec2::new(ui.available_width(), 220.0);
                                    let (rect_l, _) = ui.allocate_exact_size(desired_left, Sense::hover());
                                    let painter_l = ui.painter_at(rect_l);
                    }
                    _ => {
                        painter_l.rect_filled(rect_l, CornerRadius::same(6), Color32::from_rgb(12, 16, 23));

                                    let center_x = rect_l.center().x;
                                    let top_y = rect_l.top() + 20.0;

                                    // Concrete slab
                                    let slab_w_px = 240.0;
                                    let tc_px = (self.comp_slab_tc_mm * 0.35).clamp(18.0, 35.0);
                                    let hr_px = (self.comp_deck_hr_mm * 0.35).clamp(16.0, 30.0);

                                    let slab_rect = egui::Rect::from_min_max(
                                        Pos2::new(center_x - slab_w_px * 0.5, top_y),
                                        Pos2::new(center_x + slab_w_px * 0.5, top_y + tc_px),
                                    );
                                    painter_l.rect_filled(slab_rect, CornerRadius::same(2), Color32::from_rgb(71, 85, 105));
                                    painter_l.rect_stroke(slab_rect, CornerRadius::same(2), Stroke::new(1.0, Color32::from_rgb(148, 163, 184)), egui::StrokeKind::Inside);
                                    painter_l.text(slab_rect.center(), egui::Align2::CENTER_CENTER, format!("Concrete Slab tc = {:.0}mm (f'c = {:.0})", self.comp_slab_tc_mm, self.comp_concrete_fc_mpa), egui::FontId::proportional(9.0), Color32::from_rgb(226, 232, 240));

                                    // Metal deck flutes
                                    let flute_w = 40.0;
                                    let flute_top = top_y + tc_px;
                                    let flute_bot = flute_top + hr_px;
                                    for i in -2..=2 {
                                        let fx = center_x + i as f32 * 50.0;
                                        painter_l.rect_filled(
                                            egui::Rect::from_min_max(Pos2::new(fx - flute_w * 0.5, flute_top), Pos2::new(fx + flute_w * 0.5, flute_bot)),
                                            CornerRadius::same(1),
                                            Color32::from_rgb(51, 65, 85),
                                        );
                                    }

                                    // Shear stud in center flute
                                    let stud_top = flute_top - 12.0;
                                    let stud_rect = egui::Rect::from_min_max(Pos2::new(center_x - 4.0, stud_top), Pos2::new(center_x + 4.0, flute_bot));
                                    painter_l.rect_filled(stud_rect, CornerRadius::same(1), Color32::from_rgb(245, 158, 11));
                                    // Stud head
                                    painter_l.rect_filled(
                                        egui::Rect::from_min_max(Pos2::new(center_x - 9.0, stud_top - 4.0), Pos2::new(center_x + 9.0, stud_top)),
                                        CornerRadius::same(1),
                                        Color32::from_rgb(245, 158, 11),
                                    );

                                    // Steel Beam (W-shape)
                                    let beam_top = flute_bot;
                                    let d_px = (self.comp_steel_depth_mm * 0.22).clamp(55.0, 100.0);
                                    let bf_px = (self.comp_steel_bf_mm * 0.22).clamp(35.0, 75.0);
                                    let tf_px = 6.0;
                                    let tw_px = 4.0;

                                    // Top Flange
                                    painter_l.rect_filled(
                                        egui::Rect::from_min_max(Pos2::new(center_x - bf_px * 0.5, beam_top), Pos2::new(center_x + bf_px * 0.5, beam_top + tf_px)),
                                        CornerRadius::same(1),
                                        Color32::from_rgb(56, 189, 248),
                                    );
                                    // Web
                                    painter_l.rect_filled(
                                        egui::Rect::from_min_max(Pos2::new(center_x - tw_px * 0.5, beam_top + tf_px), Pos2::new(center_x + tw_px * 0.5, beam_top + d_px - tf_px)),
                                        CornerRadius::same(0),
                                        Color32::from_rgb(30, 41, 59),
                                    );
                                    painter_l.rect_stroke(
                                        egui::Rect::from_min_max(Pos2::new(center_x - tw_px * 0.5, beam_top + tf_px), Pos2::new(center_x + tw_px * 0.5, beam_top + d_px - tf_px)),
                                        CornerRadius::same(0),
                                        Stroke::new(1.0, Color32::from_rgb(56, 189, 248)),
                                        egui::StrokeKind::Inside,
                                    );
                                    // Bottom Flange
                                    painter_l.rect_filled(
                                        egui::Rect::from_min_max(Pos2::new(center_x - bf_px * 0.5, beam_top + d_px - tf_px), Pos2::new(center_x + bf_px * 0.5, beam_top + d_px)),
                                        CornerRadius::same(1),
                                        Color32::from_rgb(56, 189, 248),
                                    );

                                    // Plastic Neutral Axis (PNA) Reticle Line
                                    let pna_ratio = (rep.pna_depth_from_slab_top_mm / (self.comp_slab_tc_mm as f64 + self.comp_deck_hr_mm as f64 + self.comp_steel_depth_mm as f64)).clamp(0.05, 0.95);
                                    let pna_y = top_y + pna_ratio as f32 * (tc_px + hr_px + d_px);

                                    painter_l.line_segment(
                                        [Pos2::new(rect_l.left() + 15.0, pna_y), Pos2::new(rect_l.right() - 15.0, pna_y)],
                                        Stroke::new(1.5, Color32::from_rgb(239, 68, 68)),
                                    );
                                    painter_l.text(Pos2::new(rect_l.right() - 20.0, pna_y - 2.0), egui::Align2::RIGHT_BOTTOM, format!("PNA: {:.1}mm ({})", rep.pna_depth_from_slab_top_mm, match rep.pna_location { PnaLocation::InConcreteSlab => "Slab", PnaLocation::InSteelFlange => "Flange", PnaLocation::InSteelWeb => "Web" }), egui::FontId::proportional(9.0), Color32::from_rgb(239, 68, 68));

                                    // RIGHT: Plastic Stress Distribution & Deflections
                                    ui.label(RichText::new("2. Plastic Stress Distribution & Serviceability Deflection").size(11.0).strong().color(theme.text_secondary));
                                    let desired_right = Vec2::new(ui.available_width(), 220.0);
                                    let (rect_r, _) = ui.allocate_exact_size(desired_right, Sense::hover());
                                    let painter_r = ui.painter_at(rect_r);
                                    painter_r.rect_filled(rect_r, CornerRadius::same(6), Color32::from_rgb(12, 16, 23));

                                    let chart_left = rect_r.left() + 16.0;
                                    let chart_top = rect_r.top() + 20.0;
                                    let bar_w = rect_r.width() - 32.0;

                                    // Stress Block Diagram: C in Concrete, T in Steel
                                    painter_r.text(Pos2::new(chart_left, chart_top), egui::Align2::LEFT_TOP, "Plastic Internal Force Equilibrium:", egui::FontId::proportional(10.5), Color32::from_rgb(226, 232, 240));

                                    let y_c = chart_top + 22.0;
                                    painter_r.text(Pos2::new(chart_left, y_c), egui::Align2::LEFT_TOP, format!("Concrete Compressive Force (Cc): {:.1} kN", rep.total_shear_transfer_v_kn), egui::FontId::proportional(9.5), Color32::from_rgb(56, 189, 248));
                                    let w_c = ((rep.total_shear_transfer_v_kn / rep.max_steel_tensile_t_max_kn.max(1.0)) * bar_w as f64).clamp(10.0, bar_w as f64) as f32;
                                    painter_r.rect_filled(egui::Rect::from_min_size(Pos2::new(chart_left, y_c + 14.0), Vec2::new(w_c, 10.0)), CornerRadius::same(2), Color32::from_rgb(56, 189, 248));

                                    let y_t = y_c + 32.0;
                                    painter_r.text(Pos2::new(chart_left, y_t), egui::Align2::LEFT_TOP, format!("Steel Tensile Yield Force (Ts): {:.1} kN", rep.max_steel_tensile_t_max_kn), egui::FontId::proportional(9.5), Color32::from_rgb(249, 115, 22));
                                    painter_r.rect_filled(egui::Rect::from_min_size(Pos2::new(chart_left, y_t + 14.0), Vec2::new(bar_w, 10.0)), CornerRadius::same(2), Color32::from_rgb(249, 115, 22));

                                    // Deflection bars
                                    let y_def = y_t + 36.0;
                                    painter_r.text(Pos2::new(chart_left, y_def), egui::Align2::LEFT_TOP, format!("Live Load Deflection: {:.2} mm (Limit: {:.1} mm)", rep.live_load_deflection_delta_ll_mm, rep.live_load_deflection_limit_mm), egui::FontId::proportional(9.5), if rep.is_deflection_ll_ok { Color32::from_rgb(34, 197, 94) } else { Color32::from_rgb(239, 68, 68) });
                                    let def_ratio = (rep.live_load_deflection_delta_ll_mm / rep.live_load_deflection_limit_mm.max(1.0)).clamp(0.05, 1.5) as f32;
                                    let w_def = (def_ratio * bar_w * 0.7).clamp(8.0, bar_w);
                                    painter_r.rect_filled(egui::Rect::from_min_size(Pos2::new(chart_left, y_def + 14.0), Vec2::new(bar_w, 9.0)), CornerRadius::same(2), Color32::from_rgb(30, 41, 59));
                                    painter_r.rect_filled(egui::Rect::from_min_size(Pos2::new(chart_left, y_def + 14.0), Vec2::new(w_def, 9.0)), CornerRadius::same(2), if rep.is_deflection_ll_ok { Color32::from_rgb(34, 197, 94) } else { Color32::from_rgb(239, 68, 68) });

                                    let y_stage = y_def + 30.0;
                                    let stage_text = if rep.is_unshored_construction {
                                        format!("Construction Stage (Unshored Wet Concrete DCR: {:.3} - {})", rep.construction_stage_dcr, if rep.is_construction_stage_safe { "PASS" } else { "FAIL" })
                                    } else {
                                        "Construction Stage: Shored (Temporary shores provided)".to_string()
                                    };
                                    painter_r.text(Pos2::new(chart_left, y_stage), egui::Align2::LEFT_TOP, stage_text, egui::FontId::proportional(9.5), if rep.is_construction_stage_safe { Color32::from_rgb(148, 163, 184) } else { Color32::from_rgb(245, 158, 11) });
                    }
                });
            });

        ui.add_space(14.0);

        // 4. Limit States Verification Table
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(8))
            .inner_margin(Margin::same(12))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("📊 AISC 360-16 / SNI 1729:2020 Limit States Verification Table").size(12.5).strong().color(theme.text_primary));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new(format!("Effective Inertia Ieff: {:.0} cm⁴", rep.lower_bound_effective_inertia_ieff_cm4)).size(11.0).strong().color(theme.accent_blue));
                    });
                });

                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Limit State Description").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(140.0);
                    ui.label(RichText::new("Code Ref").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(70.0);
                    ui.label(RichText::new("Nominal").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(40.0);
                    ui.label(RichText::new("φ").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(35.0);
                    ui.label(RichText::new("Design").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(45.0);
                    ui.label(RichText::new("Demand").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(40.0);
                    ui.label(RichText::new("DCR").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(40.0);
                    ui.label(RichText::new("Status").size(11.0).strong().color(theme.text_secondary));
                });
                ui.separator();

                for ls in &rep.limit_states {
                    let dcr_col = if ls.demand_capacity_ratio <= 0.75 {
                        theme.accent_green
                    } else if ls.demand_capacity_ratio <= 1.00 {
                        theme.accent_orange
                    } else {
                        Color32::from_rgb(239, 68, 68)
                    };

                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&ls.limit_state_name).size(11.0).strong().color(theme.accent_blue));
                        ui.add_space(20.0);
                        ui.label(RichText::new(&ls.standard_code_ref).size(10.5).color(theme.text_muted));
                        ui.add_space(30.0);
                        ui.label(RichText::new(format!("{:7.1}", ls.nominal_capacity)).size(11.0).color(theme.text_primary));
                        ui.add_space(40.0);
                        ui.label(RichText::new(format!("{:.2}", ls.phi_factor)).size(11.0).color(theme.text_secondary));
                        ui.add_space(35.0);
                        ui.label(RichText::new(format!("{:7.1}", ls.design_capacity)).size(11.0).color(theme.accent_green));
                        ui.add_space(40.0);
                        ui.label(RichText::new(format!("{:7.1}", ls.factored_demand)).size(11.0).color(theme.text_primary));
                        ui.add_space(40.0);
                        ui.label(RichText::new(format!("{:.3}", ls.demand_capacity_ratio)).size(11.0).strong().color(dcr_col));
                        ui.add_space(40.0);
                        ui.label(RichText::new(if ls.is_safe { "🟢 PASS" } else { "🔴 FAIL" }).size(11.0).strong().color(dcr_col));
                    });
                    ui.separator();
                }
            });
    }

    pub(super) fn render_prestressed_design(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let rep = crate::core::PrestressedConcreteEngine::evaluate_girder(
            "EigenGraph Viaduct",
            "Girder G-01",
            self.pt_span_length_m as f64,
            self.pt_girder_spacing_m as f64,
            self.pt_archetype,
            self.pt_method,
            self.pt_profile,
            crate::core::StrandGrade::Grade270LowRelaxation,
            self.pt_strand_dia_mm as f64,
            self.pt_num_strands,
            0.75,
            self.pt_eccentricity_mid_mm as f64,
            self.pt_eccentricity_end_mm as f64,
            self.pt_fc_mpa as f64,
            self.pt_fci_mpa as f64,
            self.pt_sdl_kn_m as f64,
            self.pt_live_load_kn_m as f64,
        );

        // 1. Metric Cards Header
        ui_kit::responsive_grid_n(ui, 4, 190.0, |ui, col_idx| match col_idx {
            0 => {
                let status_col = if rep.is_code_compliant { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
                    let status_text = if rep.is_code_compliant { "🟢 PASS (Code Compliant)" } else { "🔴 FAIL (DCR Exceeded)" };
                    self.metric_card(
                        ui,
                        theme,
                        "OVERALL COMPLIANCE",
                        status_text,
                        &format!("Gov. DCR: {:.3} • ACI 318-19 / SNI", rep.governing_dcr),
                        status_col,
                    );
            }
            1 => {
                let loss_col = if rep.losses.loss_percentage <= 25.0 { theme.accent_green } else { theme.accent_orange };
                    self.metric_card(
                        ui,
                        theme,
                        "EFFECTIVE PRESTRESS (Peff)",
                        &format!("{:.1} kN", rep.losses.p_effective_kn),
                        &format!("Loss: {:.1}% ({:.1} MPa) • fse: {:.1} MPa", rep.losses.loss_percentage, rep.losses.total_loss_mpa, rep.losses.f_se_mpa),
                        loss_col,
                    );
            }
            2 => {
                let flex_col = if rep.flexure_dcr <= 1.0 { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
                    self.metric_card(
                        ui,
                        theme,
                        "FLEXURAL STRENGTH (φMn)",
                        &format!("{:.1} kN·m", rep.phi_mn_knm),
                        &format!("DCR: {:.3} • Mu: {:.1} kNm • 1.2Mcr: {:.1}", rep.flexure_dcr, rep.m_ultimate_knm, rep.min_flexure_req_knm),
                        flex_col,
                    );
            }
            _ => {
                let class_col = match rep.aci_class {
                        crate::core::AciTensionClass::ClassU => theme.accent_blue,
                        crate::core::AciTensionClass::ClassT => theme.accent_orange,
                        crate::core::AciTensionClass::ClassC => Color32::from_rgb(239, 68, 68),
                    };
                    self.metric_card(
                        ui,
                        theme,
                        "SERVICE CLASS & CAMBER",
                        match rep.aci_class {
                            crate::core::AciTensionClass::ClassU => "Class U (Uncracked)",
                            crate::core::AciTensionClass::ClassT => "Class T (Transition)",
                            crate::core::AciTensionClass::ClassC => "Class C (Cracked)",
                        },
                        &format!("Initial: +{:.1}mm • Long-term: {:.1}mm", rep.net_camber_transfer_mm, rep.long_term_camber_mm),
                        class_col,
                    );
            }
        });

        ui.add_space(16.0);

        // 2. Main Two-Column Layout
        ui_kit::responsive_two_panes(ui, 720.0, |ui, pane_idx| match pane_idx {
            0 => {
                // LEFT COLUMN: PARAMETER CONTROLS
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("⚙️").size(15.0));
                            ui.label(RichText::new("Prestressed Girder & Tendon Studio").size(13.5).strong().color(theme.text_primary));
                        });
                        ui.separator();
                        ui.add_space(4.0);

                        ui.label(RichText::new("GIRDER GEOMETRY & ARCHETYPE").size(11.0).strong().color(theme.accent_blue));
                        ui.horizontal(|ui| {
                            ui.label("Archetype: ");
                            egui::ComboBox::from_id_salt("pt_arch_combo")
                                .selected_text(match self.pt_archetype {
                                    crate::core::GirderSectionArchetype::AashtoTypeIV => "AASHTO Type IV (1370mm)",
                                    crate::core::GirderSectionArchetype::PrecastIGirder => "Bulb-T / I-Girder (1600mm)",
                                    crate::core::GirderSectionArchetype::BoxGirder => "Box Girder (1500mm)",
                                    crate::core::GirderSectionArchetype::RectangularBeam => "Rectangular (450x1200mm)",
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.pt_archetype, crate::core::GirderSectionArchetype::AashtoTypeIV, "AASHTO Type IV (1370mm)");
                                    ui.selectable_value(&mut self.pt_archetype, crate::core::GirderSectionArchetype::PrecastIGirder, "Bulb-T / I-Girder (1600mm)");
                                    ui.selectable_value(&mut self.pt_archetype, crate::core::GirderSectionArchetype::BoxGirder, "Box Girder (1500mm)");
                                    ui.selectable_value(&mut self.pt_archetype, crate::core::GirderSectionArchetype::RectangularBeam, "Rectangular (450x1200mm)");
                                });
                        });

                        ui.add(egui::Slider::new(&mut self.pt_span_length_m, 15.0..=45.0).text("Span Length L (m)").step_by(0.5));
                        ui.add(egui::Slider::new(&mut self.pt_girder_spacing_m, 1.5..=3.6).text("Girder Spacing S (m)").step_by(0.1));

                        ui.add_space(6.0);
                        ui.separator();
                        ui.label(RichText::new("PRESTRESSING TENDON PROFILE").size(11.0).strong().color(theme.accent_blue));

                        ui.horizontal(|ui| {
                            ui.label("Profile Kind: ");
                            egui::ComboBox::from_id_salt("pt_profile_combo")
                                .selected_text(match self.pt_profile {
                                    crate::core::TendonProfileKind::Parabolic => "Parabolic Draped",
                                    crate::core::TendonProfileKind::HarpedTwoPoint => "Harped (2-Point)",
                                    crate::core::TendonProfileKind::StraightEccentric => "Straight Constant",
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.pt_profile, crate::core::TendonProfileKind::Parabolic, "Parabolic Draped");
                                    ui.selectable_value(&mut self.pt_profile, crate::core::TendonProfileKind::HarpedTwoPoint, "Harped (2-Point)");
                                    ui.selectable_value(&mut self.pt_profile, crate::core::TendonProfileKind::StraightEccentric, "Straight Constant");
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.label("Method: ");
                            egui::ComboBox::from_id_salt("pt_method_combo")
                                .selected_text(match self.pt_method {
                                    crate::core::PrestressingMethod::PostTensioned => "Post-Tensioned (Grouted Duct)",
                                    crate::core::PrestressingMethod::PreTensioned => "Pre-Tensioned (Precast Yard)",
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.pt_method, crate::core::PrestressingMethod::PostTensioned, "Post-Tensioned (Grouted Duct)");
                                    ui.selectable_value(&mut self.pt_method, crate::core::PrestressingMethod::PreTensioned, "Pre-Tensioned (Precast Yard)");
                                });
                        });

                        ui.add(egui::Slider::new(&mut self.pt_num_strands, 12..=54).text("Strands Count Ns").step_by(2.0));
                        ui.add(egui::Slider::new(&mut self.pt_eccentricity_mid_mm, 200.0..=600.0).text("Midspan Drape e_mid (mm)").step_by(10.0));
                        ui.add(egui::Slider::new(&mut self.pt_eccentricity_end_mm, 0.0..=250.0).text("End Drape e_end (mm)").step_by(10.0));

                        ui.add_space(6.0);
                        ui.separator();
                        ui.label(RichText::new("CONCRETE & SERVICE LOADS").size(11.0).strong().color(theme.accent_blue));

                        ui.add(egui::Slider::new(&mut self.pt_fc_mpa, 35.0..=70.0).text("28-day Concrete f'c (MPa)").step_by(1.0));
                        ui.add(egui::Slider::new(&mut self.pt_fci_mpa, 25.0..=50.0).text("Release Concrete f'ci (MPa)").step_by(1.0));
                        ui.add(egui::Slider::new(&mut self.pt_sdl_kn_m, 1.0..=10.0).text("Superimposed DL w_sdl (kN/m)").step_by(0.5));
                        ui.add(egui::Slider::new(&mut self.pt_live_load_kn_m, 5.0..=30.0).text("Service Live Load w_LL (kN/m)").step_by(1.0));

                        ui.add_space(8.0);
                        if ui.add(egui::Button::new(RichText::new("📄 Export Prestressed Dossier (.md)").size(11.5).strong().color(Color32::WHITE))
                            .fill(theme.accent_blue)
                            .corner_radius(CornerRadius::same(6))).clicked() {
                            let dossier = rep.generate_markdown_dossier();
                            if let Ok(path) = crate::core::DiskExporter::write_export_file("EigenGraph_Prestressed_Concrete_Dossier.md", &dossier) {
                                self.export_status = Some(format!("Prestressed Concrete Dossier saved to: {}", path));
                            }
                        }
                    });
            }
            _ => {
                // RIGHT COLUMN: 2D VECTOR TENDON PROFILE & LOSSES
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("📐").size(15.0));
                            ui.label(RichText::new("Tendon Elevation & Neutral Axis Profile").size(13.5).strong().color(theme.text_primary));
                        });
                        ui.separator();

                        // 2D Vector Canvas for Tendon Profile
                        let (response, painter) = ui.allocate_painter(Vec2::new(ui.available_width(), 160.0), Sense::hover());
                        let rect = response.rect;

                        // Canvas background
                        painter.rect_filled(rect, CornerRadius::same(6), theme.bg_card);
                        painter.rect_stroke(rect, CornerRadius::same(6), Stroke::new(1.0, theme.border_subtle), egui::StrokeKind::Inside);

                        let margin_x = 35.0;
                        let margin_y = 20.0;
                        let draw_w = rect.width() - 2.0 * margin_x;
                        let draw_h = rect.height() - 2.0 * margin_y;

                        // Draw Girder Bounding Box
                        let top_left = Pos2::new(rect.min.x + margin_x, rect.min.y + margin_y);
                        let bot_right = Pos2::new(rect.min.x + margin_x + draw_w, rect.min.y + margin_y + draw_h);
                        let girder_rect = egui::Rect::from_min_max(top_left, bot_right);
                        painter.rect_filled(girder_rect, CornerRadius::ZERO, theme.bg_canvas);
                        painter.rect_stroke(girder_rect, CornerRadius::ZERO, Stroke::new(1.5, theme.text_muted), egui::StrokeKind::Inside);

                        // Draw Neutral Axis (dashed centerline)
                        let y_na = top_left.y + draw_h * (rep.y_top_mm as f32 / rep.depth_h_mm as f32);
                        let na_stroke = Stroke::new(1.0, theme.text_secondary);
                        let mut x_dash = top_left.x;
                        while x_dash < bot_right.x {
                            painter.line_segment([Pos2::new(x_dash, y_na), Pos2::new((x_dash + 6.0).min(bot_right.x), y_na)], na_stroke);
                            x_dash += 12.0;
                        }
                        painter.text(Pos2::new(top_left.x + 4.0, y_na - 10.0), Align2::LEFT_TOP, "N.A.", egui::FontId::proportional(9.0), theme.text_secondary);

                        // Draw Tendon Profile Curve
                        let mut tendon_pts = Vec::with_capacity(rep.tendon_profile_points.len());
                        for pt in &rep.tendon_profile_points {
                            let px = top_left.x + (pt.x_m as f32 / self.pt_span_length_m) * draw_w;
                            // Distance below neutral axis
                            let py = y_na + (pt.e_mm as f32 / rep.y_bot_mm as f32) * (draw_h * (rep.y_bot_mm as f32 / rep.depth_h_mm as f32) * 0.85);
                            tendon_pts.push(Pos2::new(px, py));
                        }

                        if tendon_pts.len() >= 2 {
                            for i in 0..(tendon_pts.len() - 1) {
                                painter.line_segment([tendon_pts[i], tendon_pts[i + 1]], Stroke::new(2.5, theme.accent_blue));
                            }
                            // Anchorage plates at ends
                            let p_start = tendon_pts[0];
                            let p_end = *tendon_pts.last().unwrap();
                            painter.rect_filled(egui::Rect::from_center_size(p_start, Vec2::new(6.0, 14.0)), CornerRadius::ZERO, theme.accent_green);
                            painter.rect_filled(egui::Rect::from_center_size(p_end, Vec2::new(6.0, 14.0)), CornerRadius::ZERO, theme.accent_green);
                        }

                        // Dimension callouts
                        painter.text(Pos2::new(rect.center().x, bot_right.y + 4.0), Align2::CENTER_TOP, format!("Span L = {:.1} m (Drape e_mid = {:.0} mm)", self.pt_span_length_m, self.pt_eccentricity_mid_mm), egui::FontId::proportional(10.0), theme.accent_blue);

                        ui.add_space(8.0);
                        ui.separator();
                        ui.label(RichText::new("PRESTRESS LOSSES BREAKDOWN (ACI / AASHTO)").size(11.0).strong().color(theme.accent_blue));

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("• Friction:").size(10.5).color(theme.text_secondary));
                            ui.label(RichText::new(format!("{:.1} MPa", rep.losses.friction_loss_mpa)).size(10.5).strong().color(theme.text_primary));
                            ui.add_space(8.0);
                            ui.label(RichText::new("• Anchorage:").size(10.5).color(theme.text_secondary));
                            ui.label(RichText::new(format!("{:.1} MPa", rep.losses.anchorage_slip_mpa)).size(10.5).strong().color(theme.text_primary));
                            ui.add_space(8.0);
                            ui.label(RichText::new("• Elastic Short.:").size(10.5).color(theme.text_secondary));
                            ui.label(RichText::new(format!("{:.1} MPa", rep.losses.elastic_shortening_mpa)).size(10.5).strong().color(theme.text_primary));
                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("• Creep:").size(10.5).color(theme.text_secondary));
                            ui.label(RichText::new(format!("{:.1} MPa", rep.losses.concrete_creep_mpa)).size(10.5).strong().color(theme.text_primary));
                            ui.add_space(8.0);
                            ui.label(RichText::new("• Shrinkage:").size(10.5).color(theme.text_secondary));
                            ui.label(RichText::new(format!("{:.1} MPa", rep.losses.concrete_shrinkage_mpa)).size(10.5).strong().color(theme.text_primary));
                            ui.add_space(8.0);
                            ui.label(RichText::new("• Relaxation:").size(10.5).color(theme.text_secondary));
                            ui.label(RichText::new(format!("{:.1} MPa", rep.losses.steel_relaxation_mpa)).size(10.5).strong().color(theme.text_primary));
                        });
                    });
            }
        });

        ui.add_space(16.0);

        // 3. Concrete Fiber Stresses & Limit States Audit Table
        theme.card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("📋").size(15.0));
                ui.label(RichText::new("Concrete Fiber Stresses & Code Compliance Audit (ACI 318-19 / SNI 2847:2019)").size(13.0).strong().color(theme.text_primary));
            });
            ui.separator();
            ui.add_space(4.0);

            // Table Header
            ui.horizontal(|ui| {
                ui.label(RichText::new("Verification Stage / Limit State").size(11.0).strong().color(theme.text_secondary));
                ui.add_space(80.0);
                ui.label(RichText::new("Standard Ref").size(11.0).strong().color(theme.text_secondary));
                ui.add_space(50.0);
                ui.label(RichText::new("Top Fiber").size(11.0).strong().color(theme.text_secondary));
                ui.add_space(35.0);
                ui.label(RichText::new("Bottom Fiber").size(11.0).strong().color(theme.text_secondary));
                ui.add_space(35.0);
                ui.label(RichText::new("Allowable Limits").size(11.0).strong().color(theme.text_secondary));
                ui.add_space(35.0);
                ui.label(RichText::new("DCR").size(11.0).strong().color(theme.text_secondary));
                ui.add_space(35.0);
                ui.label(RichText::new("Status").size(11.0).strong().color(theme.text_secondary));
            });
            ui.separator();

            // Row 1: Transfer Stage Stresses
            let t_dcr = rep.transfer_stress.top_dcr.max(rep.transfer_stress.bot_dcr);
            let t_col = if t_dcr <= 1.0 { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
            ui.horizontal(|ui| {
                ui.label(RichText::new("1. Transfer Stage (Pi + Msw)").size(11.0).strong().color(theme.accent_blue));
                ui.add_space(85.0);
                ui.label(RichText::new("ACI 318-19 §24.5.3").size(10.5).color(theme.text_muted));
                ui.add_space(50.0);
                ui.label(RichText::new(format!("{:6.2} MPa", rep.transfer_stress.sigma_top_mpa)).size(11.0).color(theme.text_primary));
                ui.add_space(40.0);
                ui.label(RichText::new(format!("{:6.2} MPa", rep.transfer_stress.sigma_bot_mpa)).size(11.0).color(theme.text_primary));
                ui.add_space(30.0);
                ui.label(RichText::new(format!("+{:.2} / -{:.1} MPa", rep.transfer_stress.allowable_tens_mpa, rep.transfer_stress.allowable_comp_mpa.abs())).size(10.5).color(theme.text_secondary));
                ui.add_space(30.0);
                ui.label(RichText::new(format!("{:.3}", t_dcr)).size(11.0).strong().color(t_col));
                ui.add_space(35.0);
                ui.label(RichText::new(if t_dcr <= 1.0 { "🟢 PASS" } else { "🔴 FAIL" }).size(11.0).strong().color(t_col));
            });
            ui.separator();

            // Row 2: Service Stage Stresses
            let s_dcr = rep.service_stress.top_dcr.max(rep.service_stress.bot_dcr);
            let s_col = if s_dcr <= 1.0 { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
            ui.horizontal(|ui| {
                ui.label(RichText::new("2. Service Stage (Peff + Mserv)").size(11.0).strong().color(theme.accent_blue));
                ui.add_space(72.0);
                ui.label(RichText::new("ACI 318-19 §24.5.4").size(10.5).color(theme.text_muted));
                ui.add_space(50.0);
                ui.label(RichText::new(format!("{:6.2} MPa", rep.service_stress.sigma_top_mpa)).size(11.0).color(theme.text_primary));
                ui.add_space(40.0);
                ui.label(RichText::new(format!("{:6.2} MPa", rep.service_stress.sigma_bot_mpa)).size(11.0).color(theme.text_primary));
                ui.add_space(30.0);
                ui.label(RichText::new(format!("+{:.2} / -{:.1} MPa", rep.service_stress.allowable_tens_mpa, rep.service_stress.allowable_comp_mpa.abs())).size(10.5).color(theme.text_secondary));
                ui.add_space(30.0);
                ui.label(RichText::new(format!("{:.3}", s_dcr)).size(11.0).strong().color(s_col));
                ui.add_space(35.0);
                ui.label(RichText::new(if s_dcr <= 1.0 { "🟢 PASS" } else { "🔴 FAIL" }).size(11.0).strong().color(s_col));
            });
            ui.separator();

            // Row 3: Ultimate Flexural Strength
            let flex_col = if rep.flexure_dcr <= 1.0 { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
            ui.horizontal(|ui| {
                ui.label(RichText::new("3. Ultimate Flexure (φMn ≥ Mu)").size(11.0).strong().color(theme.accent_blue));
                ui.add_space(76.0);
                ui.label(RichText::new("ACI 318-19 §22.2").size(10.5).color(theme.text_muted));
                ui.add_space(60.0);
                ui.label(RichText::new(format!("Mu = {:6.1} kNm", rep.m_ultimate_knm)).size(11.0).color(theme.text_primary));
                ui.add_space(20.0);
                ui.label(RichText::new(format!("φMn = {:6.1} kNm", rep.phi_mn_knm)).size(11.0).color(theme.accent_green));
                ui.add_space(25.0);
                ui.label(RichText::new(format!("fps = {:.0} MPa", rep.fps_mpa)).size(10.5).color(theme.text_secondary));
                ui.add_space(40.0);
                ui.label(RichText::new(format!("{:.3}", rep.flexure_dcr)).size(11.0).strong().color(flex_col));
                ui.add_space(35.0);
                ui.label(RichText::new(if rep.flexure_dcr <= 1.0 { "🟢 PASS" } else { "🔴 FAIL" }).size(11.0).strong().color(flex_col));
            });
            ui.separator();

            // Row 4: Minimum Flexure (1.2 Mcr)
            let mcr_dcr = rep.min_flexure_req_knm / rep.phi_mn_knm;
            let mcr_col = if mcr_dcr <= 1.0 { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
            ui.horizontal(|ui| {
                ui.label(RichText::new("4. Cracking Safety (φMn ≥ 1.2Mcr)").size(11.0).strong().color(theme.accent_blue));
                ui.add_space(60.0);
                ui.label(RichText::new("ACI 318-19 §18.8.2").size(10.5).color(theme.text_muted));
                ui.add_space(50.0);
                ui.label(RichText::new(format!("1.2Mcr = {:5.1} kNm", rep.min_flexure_req_knm)).size(11.0).color(theme.text_primary));
                ui.add_space(20.0);
                ui.label(RichText::new(format!("Mcr = {:6.1} kNm", rep.m_cracking_knm)).size(11.0).color(theme.text_secondary));
                ui.add_space(30.0);
                ui.label(RichText::new(format!("fr = {:.2} MPa", 0.62 * (self.pt_fc_mpa as f64).sqrt())).size(10.5).color(theme.text_secondary));
                ui.add_space(40.0);
                ui.label(RichText::new(format!("{:.3}", mcr_dcr)).size(11.0).strong().color(mcr_col));
                ui.add_space(35.0);
                ui.label(RichText::new(if mcr_dcr <= 1.0 { "🟢 PASS" } else { "🔴 FAIL" }).size(11.0).strong().color(mcr_col));
            });
            ui.separator();

            // Row 5: Concrete Shear Resistance
            let shear_col = if rep.shear_dcr <= 1.0 { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
            ui.horizontal(|ui| {
                ui.label(RichText::new("5. Concrete Shear (φVn ≥ Vu)").size(11.0).strong().color(theme.accent_blue));
                ui.add_space(80.0);
                ui.label(RichText::new("ACI 318-19 §22.5").size(10.5).color(theme.text_muted));
                ui.add_space(60.0);
                ui.label(RichText::new(format!("Vu = {:6.1} kN", rep.v_ultimate_kn)).size(11.0).color(theme.text_primary));
                ui.add_space(25.0);
                ui.label(RichText::new(format!("φVn = {:6.1} kN", rep.phi_vn_kn)).size(11.0).color(theme.accent_green));
                ui.add_space(25.0);
                ui.label(RichText::new(format!("Vc = {:.0} kN, Vs = {:.0} kN", rep.v_c_kn, rep.v_s_kn)).size(10.5).color(theme.text_secondary));
                ui.add_space(20.0);
                ui.label(RichText::new(format!("{:.3}", rep.shear_dcr)).size(11.0).strong().color(shear_col));
                ui.add_space(35.0);
                ui.label(RichText::new(if rep.shear_dcr <= 1.0 { "🟢 PASS" } else { "🔴 FAIL" }).size(11.0).strong().color(shear_col));
            });
        });
    }

    pub(super) fn render_shear_wall_design(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let rep = crate::core::ShearWallEngine::evaluate_wall(
            "SW-Core-01",
            self.sw_length_lw_m as f64,
            self.sw_height_hw_m as f64,
            self.sw_thickness_tw_mm as f64,
            self.sw_story_height_hu_m as f64,
            self.sw_fc_mpa as f64,
            self.sw_fy_mpa as f64,
            self.sw_fyt_mpa as f64,
            self.sw_demand_pu_kn as f64,
            self.sw_demand_mu_knm as f64,
            self.sw_demand_vu_kn as f64,
            self.sw_drift_ratio as f64,
            self.sw_cb_clear_span_m as f64,
            self.sw_cb_depth_m as f64,
            self.sw_cb_width_mm as f64,
            self.sw_cb_demand_vu_kn as f64,
            self.sw_cb_bar_dia_mm as f64,
            self.sw_cb_bars_per_group,
        );

        // 1. Executive Metric Cards Header
        ui_kit::responsive_grid_n(ui, 4, 190.0, |ui, col_idx| match col_idx {
            0 => {
                let status_col = if rep.is_fully_compliant { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
                    let status_text = if rep.is_fully_compliant { "🟢 PASS (Code Compliant)" } else { "🔴 FAIL (DCR Exceeded)" };
                    self.metric_card(
                        ui,
                        theme,
                        "OVERALL COMPLIANCE",
                        status_text,
                        &format!("Gov. DCR: {:.3} • ACI 318-19 / SNI 2847", rep.governing_dcr),
                        status_col,
                    );
            }
            1 => {
                let sbe_col = if rep.boundary_element.is_sbe_required {
                        if rep.boundary_element.is_confinement_adequate { theme.accent_blue } else { Color32::from_rgb(239, 68, 68) }
                    } else {
                        theme.accent_green
                    };
                    let sbe_status = if rep.boundary_element.is_sbe_required { "⚠️ MANDATORY SBE" } else { "✅ NO SBE NEEDED" };
                    self.metric_card(
                        ui,
                        theme,
                        "BOUNDARY ELEMENT (SBE)",
                        sbe_status,
                        &format!("c: {:.0}mm • clim: {:.0}mm • bE: {:.0}mm", rep.boundary_element.neutral_axis_c_mm, rep.boundary_element.c_limit_mm, rep.boundary_element.provided_length_be_mm),
                        sbe_col,
                    );
            }
            2 => {
                let shear_col = if rep.in_plane_shear.is_shear_adequate { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
                    self.metric_card(
                        ui,
                        theme,
                        "IN-PLANE SHEAR (φVn)",
                        &format!("{:.1} kN", rep.in_plane_shear.phi_vn_kn),
                        &format!("DCR: {:.3} • Vu: {:.1} kN • αc: {:.2}", rep.in_plane_shear.shear_dcr, rep.in_plane_shear.factored_shear_vu_kn, rep.in_plane_shear.alpha_c),
                        shear_col,
                    );
            }
            _ => {
                let cb_col = if rep.coupling_beam.is_coupling_adequate { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
                    self.metric_card(
                        ui,
                        theme,
                        "COUPLING BEAM (φVn)",
                        &format!("{:.1} kN", rep.coupling_beam.phi_vn_kn),
                        &format!("DCR: {:.3} • α: {:.1}° • ln/h: {:.2}", rep.coupling_beam.coupling_dcr, rep.coupling_beam.diagonal_angle_deg, rep.coupling_beam.aspect_ratio_ln_over_h),
                        cb_col,
                    );
            }
        });

        ui.add_space(14.0);

        // 2. Dual Panel Studio: Parameters on Left, 2D Vector Canvas on Right
        ui_kit::responsive_two_panes(ui, 720.0, |ui, pane_idx| match pane_idx {
            0 => {
                // LEFT COLUMN: Parameter Controls Studio
                    theme.card_frame().show(ui, |ui| {
                        ui.label(RichText::new("🏢 WALL PIER & COUPLING PARAMETERS").size(12.5).strong().color(theme.accent_blue));
                        ui.add_space(6.0);

                        // Section A: Wall Geometry
                        ui.label(RichText::new("Wall Pier Dimensions (ACI 318-19 §18.10)").size(11.0).strong().color(theme.text_primary));
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Length lw:").size(10.5).color(theme.text_secondary));
                            ui.add(egui::Slider::new(&mut self.sw_length_lw_m, 3.0..=12.0).suffix(" m"));
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Height hw:").size(10.5).color(theme.text_secondary));
                            ui.add(egui::Slider::new(&mut self.sw_height_hw_m, 10.0..=80.0).suffix(" m"));
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Thickness tw:").size(10.5).color(theme.text_secondary));
                            ui.add(egui::Slider::new(&mut self.sw_thickness_tw_mm, 200.0..=600.0).suffix(" mm"));
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Story hu:").size(10.5).color(theme.text_secondary));
                            ui.add(egui::Slider::new(&mut self.sw_story_height_hu_m, 2.8..=5.0).suffix(" m"));
                        });

                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(6.0);

                        // Section B: Materials & Applied Demands
                        ui.label(RichText::new("Materials & Factored Demands").size(11.0).strong().color(theme.text_primary));
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("f'c:").size(10.5).color(theme.text_secondary));
                            ui.add(egui::Slider::new(&mut self.sw_fc_mpa, 25.0..=65.0).suffix(" MPa"));
                            ui.label(RichText::new("fy:").size(10.5).color(theme.text_secondary));
                            ui.add(egui::Slider::new(&mut self.sw_fy_mpa, 400.0..=500.0).suffix(" MPa"));
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Pu:").size(10.5).color(theme.text_secondary));
                            ui.add(egui::Slider::new(&mut self.sw_demand_pu_kn, 1000.0..=12000.0).suffix(" kN"));
                            ui.label(RichText::new("Mu:").size(10.5).color(theme.text_secondary));
                            ui.add(egui::Slider::new(&mut self.sw_demand_mu_knm, 2000.0..=35000.0).suffix(" kNm"));
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Vu:").size(10.5).color(theme.text_secondary));
                            ui.add(egui::Slider::new(&mut self.sw_demand_vu_kn, 500.0..=6000.0).suffix(" kN"));
                            ui.label(RichText::new("Drift δu/hw:").size(10.5).color(theme.text_secondary));
                            ui.add(egui::Slider::new(&mut self.sw_drift_ratio, 0.005..=0.025));
                        });

                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(6.0);

                        // Section C: Diagonally Reinforced Coupling Beam
                        ui.label(RichText::new("Coupling Beam Parameters (§18.10.7)").size(11.0).strong().color(theme.text_primary));
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Clear Span ln:").size(10.5).color(theme.text_secondary));
                            ui.add(egui::Slider::new(&mut self.sw_cb_clear_span_m, 1.0..=3.5).suffix(" m"));
                            ui.label(RichText::new("Depth h:").size(10.5).color(theme.text_secondary));
                            ui.add(egui::Slider::new(&mut self.sw_cb_depth_m, 0.6..=1.5).suffix(" m"));
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Demand Vu,cb:").size(10.5).color(theme.text_secondary));
                            ui.add(egui::Slider::new(&mut self.sw_cb_demand_vu_kn, 200.0..=1500.0).suffix(" kN"));
                            ui.label(RichText::new("Diag Bar Ø:").size(10.5).color(theme.text_secondary));
                            ui.add(egui::Slider::new(&mut self.sw_cb_bar_dia_mm, 19.0..=32.0).suffix(" mm"));
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Bars per Group:").size(10.5).color(theme.text_secondary));
                            ui.add(egui::Slider::new(&mut self.sw_cb_bars_per_group, 2..=8));
                        });
                    });
            }
            _ => {
                // RIGHT COLUMN: 2D Vector CAD Canvas Detailer
                    theme.card_frame().show(ui, |ui| {
                        ui.label(RichText::new("📐 2D VECTOR REBAR & BOUNDARY BLUEPRINT").size(12.5).strong().color(theme.accent_blue));
                        ui.add_space(4.0);

                        let canvas_size = Vec2::new(ui.available_width(), 320.0);
                        let (response, painter) = ui.allocate_painter(canvas_size, Sense::hover());
                        let rect = response.rect;

                        // Background
                        painter.rect_filled(rect, CornerRadius::same(6), theme.bg_canvas);
                        painter.rect_stroke(rect, CornerRadius::same(6), Stroke::new(1.0, theme.border_subtle), egui::StrokeKind::Inside);

                        // 1. Draw Wall Pier Horizontal Cross-Section (Top Half)
                        let wall_scale = (rect.width() - 80.0) / (self.sw_length_lw_m * 1000.0);
                        let section_w = (self.sw_length_lw_m * 1000.0) * wall_scale;
                        let section_h = (self.sw_thickness_tw_mm * 1.5) * wall_scale;
                        let sec_x0 = rect.left() + (rect.width() - section_w) * 0.5;
                        let sec_y0 = rect.top() + 35.0;

                        // Concrete Web
                        let web_rect = egui::Rect::from_min_size(Pos2::new(sec_x0, sec_y0), Vec2::new(section_w, section_h));
                        painter.rect_filled(web_rect, CornerRadius::same(2), Color32::from_rgb(45, 55, 72));
                        painter.rect_stroke(web_rect, CornerRadius::same(2), Stroke::new(1.5, theme.border_subtle), egui::StrokeKind::Inside);

                        // Special Boundary Elements (Left & Right Ends)
                        let be_pixel_w = (rep.boundary_element.provided_length_be_mm * wall_scale as f64) as f32;
                        let be_left_rect = egui::Rect::from_min_size(Pos2::new(sec_x0, sec_y0), Vec2::new(be_pixel_w, section_h));
                        let be_right_rect = egui::Rect::from_min_size(Pos2::new(sec_x0 + section_w - be_pixel_w, sec_y0), Vec2::new(be_pixel_w, section_h));

                        let sbe_fill = if rep.boundary_element.is_sbe_required {
                            Color32::from_rgba_unmultiplied(59, 130, 246, 60)
                        } else {
                            Color32::from_rgba_unmultiplied(16, 185, 129, 40)
                        };
                        let sbe_stroke = if rep.boundary_element.is_sbe_required {
                            Stroke::new(1.5, theme.accent_blue)
                        } else {
                            Stroke::new(1.0, theme.accent_green)
                        };

                        painter.rect_filled(be_left_rect, CornerRadius::ZERO, sbe_fill);
                        painter.rect_stroke(be_left_rect, CornerRadius::ZERO, sbe_stroke, egui::StrokeKind::Inside);
                        painter.rect_filled(be_right_rect, CornerRadius::ZERO, sbe_fill);
                        painter.rect_stroke(be_right_rect, CornerRadius::ZERO, sbe_stroke, egui::StrokeKind::Inside);

                        // Boundary Element Longitudinal Rebar Dots
                        let dot_r = 3.0;
                        for i in 0..4 {
                            let frac = (i as f32 + 0.5) / 4.0;
                            // Left Boundary
                            painter.circle_filled(Pos2::new(sec_x0 + frac * be_pixel_w, sec_y0 + 4.0), dot_r, Color32::from_rgb(251, 191, 36));
                            painter.circle_filled(Pos2::new(sec_x0 + frac * be_pixel_w, sec_y0 + section_h - 4.0), dot_r, Color32::from_rgb(251, 191, 36));
                            // Right Boundary
                            painter.circle_filled(Pos2::new(sec_x0 + section_w - be_pixel_w + frac * be_pixel_w, sec_y0 + 4.0), dot_r, Color32::from_rgb(251, 191, 36));
                            painter.circle_filled(Pos2::new(sec_x0 + section_w - be_pixel_w + frac * be_pixel_w, sec_y0 + section_h - 4.0), dot_r, Color32::from_rgb(251, 191, 36));
                        }

                        // Neutral Axis Depth Line
                        let c_pixel = (rep.boundary_element.neutral_axis_c_mm * wall_scale as f64) as f32;
                        let na_x = sec_x0 + c_pixel;
                        painter.line_segment(
                            [Pos2::new(na_x, sec_y0 - 6.0), Pos2::new(na_x, sec_y0 + section_h + 6.0)],
                            Stroke::new(1.5, Color32::from_rgb(239, 68, 68)),
                        );
                        painter.text(
                            Pos2::new(na_x, sec_y0 - 10.0),
                            Align2::CENTER_BOTTOM,
                            format!("NA c = {:.0}mm", rep.boundary_element.neutral_axis_c_mm),
                            egui::FontId::proportional(9.0),
                            Color32::from_rgb(239, 68, 68),
                        );

                        // Labels for Wall Pier Section
                        painter.text(
                            Pos2::new(sec_x0 + be_pixel_w * 0.5, sec_y0 + section_h + 12.0),
                            Align2::CENTER_TOP,
                            format!("SBE bE = {:.0}mm", rep.boundary_element.provided_length_be_mm),
                            egui::FontId::proportional(9.0),
                            theme.accent_blue,
                        );
                        painter.text(
                            Pos2::new(sec_x0 + section_w * 0.5, sec_y0 + section_h * 0.5),
                            Align2::CENTER_CENTER,
                            format!("Double Curtain Web Rebar (tw = {:.0}mm)", self.sw_thickness_tw_mm),
                            egui::FontId::proportional(9.0),
                            theme.text_muted,
                        );

                        // 2. Draw Diagonally Reinforced Coupling Beam Elevation (Bottom Half)
                        let cb_y0 = rect.top() + 140.0;
                        let cb_h = 100.0;
                        let pier_w = 60.0;
                        let cb_span = (rect.width() - 2.0 * pier_w - 60.0).min(200.0);
                        let pier_left_x = rect.left() + (rect.width() - (2.0 * pier_w + cb_span)) * 0.5;
                        let beam_left_x = pier_left_x + pier_w;
                        let beam_right_x = beam_left_x + cb_span;
                        let pier_right_x = beam_right_x + pier_w;

                        // Wall Piers Left & Right
                        let p_left_rect = egui::Rect::from_min_max(Pos2::new(pier_left_x, cb_y0 - 20.0), Pos2::new(beam_left_x, cb_y0 + cb_h + 20.0));
                        let p_right_rect = egui::Rect::from_min_max(Pos2::new(beam_right_x, cb_y0 - 20.0), Pos2::new(pier_right_x, cb_y0 + cb_h + 20.0));
                        painter.rect_filled(p_left_rect, CornerRadius::ZERO, Color32::from_rgb(30, 41, 59));
                        painter.rect_stroke(p_left_rect, CornerRadius::ZERO, Stroke::new(1.0, theme.border_subtle), egui::StrokeKind::Inside);
                        painter.rect_filled(p_right_rect, CornerRadius::ZERO, Color32::from_rgb(30, 41, 59));
                        painter.rect_stroke(p_right_rect, CornerRadius::ZERO, Stroke::new(1.0, theme.border_subtle), egui::StrokeKind::Inside);

                        // Coupling Beam Body
                        let beam_rect = egui::Rect::from_min_max(Pos2::new(beam_left_x, cb_y0), Pos2::new(beam_right_x, cb_y0 + cb_h));
                        painter.rect_filled(beam_rect, CornerRadius::ZERO, Color32::from_rgb(51, 65, 85));
                        painter.rect_stroke(beam_rect, CornerRadius::ZERO, Stroke::new(1.5, theme.accent_blue), egui::StrokeKind::Inside);

                        // Diagonally Placed Bar Bundles (X-Pattern)
                        painter.line_segment(
                            [Pos2::new(beam_left_x - 15.0, cb_y0 + cb_h - 10.0), Pos2::new(beam_right_x + 15.0, cb_y0 + 10.0)],
                            Stroke::new(3.0, Color32::from_rgb(245, 158, 11)),
                        );
                        painter.line_segment(
                            [Pos2::new(beam_left_x - 15.0, cb_y0 + 10.0), Pos2::new(beam_right_x + 15.0, cb_y0 + cb_h - 10.0)],
                            Stroke::new(3.0, Color32::from_rgb(245, 158, 11)),
                        );

                        // Diagonal Hoops Tick Marks
                        for i in 1..6 {
                            let t = i as f32 / 7.0;
                            let p1 = Pos2::new(beam_left_x + t * cb_span, cb_y0 + cb_h - 10.0 - t * (cb_h - 20.0));
                            let p2 = Pos2::new(beam_left_x + t * cb_span, cb_y0 + 10.0 + t * (cb_h - 20.0));
                            painter.circle_filled(p1, 2.0, Color32::WHITE);
                            painter.circle_filled(p2, 2.0, Color32::WHITE);
                        }

                        // Coupling Beam Annotations
                        painter.text(
                            Pos2::new((beam_left_x + beam_right_x) * 0.5, cb_y0 + cb_h + 10.0),
                            Align2::CENTER_TOP,
                            format!("Clear Span ln = {:.1}m (ln/h = {:.2})", self.sw_cb_clear_span_m, rep.coupling_beam.aspect_ratio_ln_over_h),
                            egui::FontId::proportional(9.5),
                            theme.text_primary,
                        );
                        painter.text(
                            Pos2::new((beam_left_x + beam_right_x) * 0.5, cb_y0 - 8.0),
                            Align2::CENTER_BOTTOM,
                            format!("Diagonal Reinforcement: {}x Ø{:.0}mm (α = {:.1}°)", self.sw_cb_bars_per_group, self.sw_cb_bar_dia_mm, rep.coupling_beam.diagonal_angle_deg),
                            egui::FontId::proportional(9.5),
                            Color32::from_rgb(245, 158, 11),
                        );
                    });
            }
        });

        ui.add_space(14.0);

        // 3. Full Regulatory Code Compliance Audit Table
        theme.card_frame().show(ui, |ui| {
            ui.label(RichText::new("📋 ACI 318-19 CHAPTER 18 / SNI 2847:2019 PASAL 18 REGULATORY AUDIT").size(12.0).strong().color(theme.accent_blue));
            ui.add_space(4.0);

            // Table Header
            ui.horizontal(|ui| {
                ui.label(RichText::new("Code Check & Limit State").size(11.0).strong().color(theme.text_primary));
                ui.add_space(140.0);
                ui.label(RichText::new("Clause Citation").size(10.5).color(theme.text_muted));
                ui.add_space(60.0);
                ui.label(RichText::new("Factored Demand").size(10.5).color(theme.text_secondary));
                ui.add_space(35.0);
                ui.label(RichText::new("Design Capacity").size(10.5).color(theme.accent_green));
                ui.add_space(35.0);
                ui.label(RichText::new("Parameters & Notes").size(10.5).color(theme.text_secondary));
                ui.add_space(35.0);
                ui.label(RichText::new("DCR").size(11.0).strong().color(theme.text_primary));
                ui.add_space(25.0);
                ui.label(RichText::new("Status").size(11.0).strong().color(theme.text_primary));
            });
            ui.separator();

            // Row 1: In-Plane Web Shear Resistance
            let shear_col = if rep.in_plane_shear.is_shear_adequate { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
            ui.horizontal(|ui| {
                ui.label(RichText::new("1. In-Plane Web Shear (φVn ≥ Vu)").size(11.0).strong().color(theme.accent_blue));
                ui.add_space(55.0);
                ui.label(RichText::new("ACI 318-19 §18.10.4").size(10.5).color(theme.text_muted));
                ui.add_space(45.0);
                ui.label(RichText::new(format!("Vu = {:6.1} kN", rep.in_plane_shear.factored_shear_vu_kn)).size(11.0).color(theme.text_primary));
                ui.add_space(25.0);
                ui.label(RichText::new(format!("φVn = {:6.1} kN", rep.in_plane_shear.phi_vn_kn)).size(11.0).color(theme.accent_green));
                ui.add_space(25.0);
                ui.label(RichText::new(format!("αc = {:.2}, Vc = {:.0}kN, Vs = {:.0}kN", rep.in_plane_shear.alpha_c, rep.in_plane_shear.concrete_shear_vc_kn, rep.in_plane_shear.steel_shear_vs_kn)).size(10.5).color(theme.text_secondary));
                ui.add_space(15.0);
                ui.label(RichText::new(format!("{:.3}", rep.in_plane_shear.shear_dcr)).size(11.0).strong().color(shear_col));
                ui.add_space(30.0);
                ui.label(RichText::new(if rep.in_plane_shear.is_shear_adequate { "🟢 PASS" } else { "🔴 FAIL" }).size(11.0).strong().color(shear_col));
            });
            ui.separator();

            // Row 2: Displacement-Based Boundary Element Trigger
            let sbe_stat_col = if rep.boundary_element.is_geometry_adequate { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
            ui.horizontal(|ui| {
                ui.label(RichText::new("2. SBE Displacement Trigger").size(11.0).strong().color(theme.accent_blue));
                ui.add_space(80.0);
                ui.label(RichText::new("ACI 318-19 §18.10.6.2").size(10.5).color(theme.text_muted));
                ui.add_space(35.0);
                ui.label(RichText::new(format!("c = {:5.0} mm", rep.boundary_element.neutral_axis_c_mm)).size(11.0).color(theme.text_primary));
                ui.add_space(30.0);
                ui.label(RichText::new(format!("clim = {:5.0} mm", rep.boundary_element.c_limit_mm)).size(11.0).color(theme.accent_green));
                ui.add_space(30.0);
                ui.label(RichText::new(if rep.boundary_element.is_sbe_required { "Mandatory SBE Triggered" } else { "Boundary Element Not Required" }).size(10.5).color(theme.text_secondary));
                ui.add_space(20.0);
                ui.label(RichText::new(format!("{:.3}", rep.boundary_element.neutral_axis_c_mm / rep.boundary_element.c_limit_mm)).size(11.0).strong().color(sbe_stat_col));
                ui.add_space(30.0);
                ui.label(RichText::new(if rep.boundary_element.is_geometry_adequate { "🟢 PASS" } else { "🔴 FAIL" }).size(11.0).strong().color(sbe_stat_col));
            });
            ui.separator();

            // Row 3: Special Boundary Element Confinement
            let conf_col = if rep.boundary_element.is_confinement_adequate { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
            ui.horizontal(|ui| {
                ui.label(RichText::new("3. SBE Confinement (Ash/s)").size(11.0).strong().color(theme.accent_blue));
                ui.add_space(85.0);
                ui.label(RichText::new("ACI Table 18.10.6.4(f)").size(10.5).color(theme.text_muted));
                ui.add_space(25.0);
                ui.label(RichText::new(format!("Req = {:4.2} mm²/mm", rep.boundary_element.req_ash_over_s_mm2_per_mm)).size(11.0).color(theme.text_primary));
                ui.add_space(15.0);
                ui.label(RichText::new(format!("Prov = {:4.2} mm²/mm", rep.boundary_element.provided_ash_over_s_mm2_per_mm)).size(11.0).color(theme.accent_green));
                ui.add_space(15.0);
                ui.label(RichText::new(format!("s = {:.0}mm (max {:.0}mm), bE = {:.0}mm", rep.boundary_element.provided_tie_spacing_s_mm, rep.boundary_element.max_tie_spacing_s_mm, rep.boundary_element.provided_length_be_mm)).size(10.5).color(theme.text_secondary));
                ui.add_space(15.0);
                ui.label(RichText::new(format!("{:.3}", rep.boundary_element.dcr_confinement)).size(11.0).strong().color(conf_col));
                ui.add_space(30.0);
                ui.label(RichText::new(if rep.boundary_element.is_confinement_adequate { "🟢 PASS" } else { "🔴 FAIL" }).size(11.0).strong().color(conf_col));
            });
            ui.separator();

            // Row 4: Wall Pier Flexural Capacity
            let flex_col = if rep.is_flexure_adequate { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
            ui.horizontal(|ui| {
                ui.label(RichText::new("4. Wall Pier Flexure (φMn ≥ Mu)").size(11.0).strong().color(theme.accent_blue));
                ui.add_space(60.0);
                ui.label(RichText::new("ACI 318-19 §22.2").size(10.5).color(theme.text_muted));
                ui.add_space(60.0);
                ui.label(RichText::new(format!("Mu = {:6.1} kNm", rep.demands.mu_knm)).size(11.0).color(theme.text_primary));
                ui.add_space(20.0);
                ui.label(RichText::new(format!("φMn = {:6.1} kNm", rep.flexure_phi_mn_knm)).size(11.0).color(theme.accent_green));
                ui.add_space(25.0);
                ui.label(RichText::new(format!("Pu = {:.0} kN, As,sbe = {:.0} mm²", rep.demands.pu_kn, rep.boundary_element.sbe_rebar_area_mm2)).size(10.5).color(theme.text_secondary));
                ui.add_space(20.0);
                ui.label(RichText::new(format!("{:.3}", rep.flexure_dcr)).size(11.0).strong().color(flex_col));
                ui.add_space(30.0);
                ui.label(RichText::new(if rep.is_flexure_adequate { "🟢 PASS" } else { "🔴 FAIL" }).size(11.0).strong().color(flex_col));
            });
            ui.separator();

            // Row 5: Diagonally Reinforced Coupling Beam
            let cb_stat_col = if rep.coupling_beam.is_coupling_adequate { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
            ui.horizontal(|ui| {
                ui.label(RichText::new("5. Diag. Coupling Beam (φVn ≥ Vu)").size(11.0).strong().color(theme.accent_blue));
                ui.add_space(45.0);
                ui.label(RichText::new("ACI 318-19 §18.10.7").size(10.5).color(theme.text_muted));
                ui.add_space(45.0);
                ui.label(RichText::new(format!("Vu = {:6.1} kN", rep.coupling_beam.factored_shear_vu_kn)).size(11.0).color(theme.text_primary));
                ui.add_space(25.0);
                ui.label(RichText::new(format!("φVn = {:6.1} kN", rep.coupling_beam.phi_vn_kn)).size(11.0).color(theme.accent_green));
                ui.add_space(25.0);
                ui.label(RichText::new(format!("Vn = 2·Avd·fy·sinα (α = {:.1}°)", rep.coupling_beam.diagonal_angle_deg)).size(10.5).color(theme.text_secondary));
                ui.add_space(30.0);
                ui.label(RichText::new(format!("{:.3}", rep.coupling_beam.coupling_dcr)).size(11.0).strong().color(cb_stat_col));
                ui.add_space(30.0);
                ui.label(RichText::new(if rep.coupling_beam.is_coupling_adequate { "🟢 PASS" } else { "🔴 FAIL" }).size(11.0).strong().color(cb_stat_col));
            });
        });
    }


}
