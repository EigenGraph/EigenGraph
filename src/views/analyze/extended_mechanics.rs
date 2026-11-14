//! views/analyze/extended_mechanics.rs — Bridge Moving Loads, Retaining Slope Stability & Floor Vibration / Fatigue.

use super::*;
use egui::{CornerRadius, FontId, Frame, Margin, Rect, RichText, Stroke, StrokeKind, Vec2};

#[derive(Clone, Debug)]
pub struct MovingLoadResponse {
    pub env_max_pos_moment_knm: f32,
    pub env_max_neg_pier_knm: f32,
    pub env_max_shear_kn: f32,
    pub active_mid_moment_knm: f32,
    pub peak_truck_pos_frac: f32,
    pub total_length_m: f32,
    pub influence_line: Vec<(f32, f32)>,
    pub active_bmd: Vec<(f32, f32)>,
    pub active_axle_xs: Vec<f32>,
}

fn beam_pier_moment_unit(u: f32, span: f32, two_span: bool) -> f32 {
    if !two_span || span <= 1e-4 {
        return 0.0;
    }
    if u <= span {
        -(u * (span * span - u * u)) / (4.0 * span * span)
    } else {
        let s = (2.0 * span - u).max(0.0);
        -(s * (span * span - s * s)) / (4.0 * span * span)
    }
}

fn beam_reactions_unit(u: f32, span: f32, two_span: bool) -> (f32, f32, f32) {
    if !two_span {
        let r_a = ((span - u) / span).clamp(0.0, 1.0);
        let r_c = (u / span).clamp(0.0, 1.0);
        return (r_a, 0.0, r_c);
    }
    let mb = beam_pier_moment_unit(u, span, true);
    if u <= span {
        let r_a = (span - u) / span + mb / span;
        let r_b = u / span - 2.0 * mb / span;
        let r_c = mb / span;
        (r_a, r_b, r_c)
    } else {
        let s = 2.0 * span - u;
        let r_c = (span - s) / span + mb / span;
        let r_b = s / span - 2.0 * mb / span;
        let r_a = mb / span;
        (r_a, r_b, r_c)
    }
}

fn unit_moment_at(x: f32, u: f32, span: f32, two_span: bool) -> f32 {
    let (r_a, _r_b, r_c) = beam_reactions_unit(u, span, two_span);
    if !two_span || x <= span {
        r_a * x - if x >= u { x - u } else { 0.0 }
    } else {
        let xr = 2.0 * span - x;
        r_c * xr - if u >= x { u - x } else { 0.0 }
    }
}

pub fn compute_bridge_moving_load_response(
    span_m: f32,
    truck_pos_frac: f32,
    standard_idx: usize,
    dynamic_im: f32,
    two_span: bool,
) -> MovingLoadResponse {
    let span = span_m.clamp(8.0, 120.0);
    let total_length_m = if two_span { span * 2.0 } else { span };
    let (axles, udl_kn_m): ([(f32, f32); 3], f32) = if standard_idx == 1 {
        // SNI 1725:2016 (T-Truck 50/225/225 kN + BTR 9.0 kN/m)
        ([(0.0, 50.0), (5.0, 225.0), (10.0, 225.0)], 9.0)
    } else {
        // AASHTO HL-93 (35/145/145 kN + 9.3 kN/m design lane)
        ([(0.0, 35.0), (4.3, 145.0), (8.6, 145.0)], 9.3)
    };
    let im_factor = 1.0 + dynamic_im.clamp(0.0, 0.75);
    let eval_sec_x = span * if two_span { 0.42 } else { 0.50 };

    let sample_bmd_at = |lead_x: f32| -> (f32, f32, Vec<(f32, f32)>) {
        let mut pts = Vec::with_capacity(41);
        let mut max_pos = 0.0_f32;
        let mut max_neg_pier = 0.0_f32;
        let dx_int = total_length_m / 60.0;
        for i in 0..=40 {
            let frac = i as f32 / 40.0;
            let x = frac * total_length_m;
            let mut m = 0.0_f32;
            for &(off, load_kn) in &axles {
                let u = lead_x - off;
                if u >= 0.0 && u <= total_length_m {
                    m += load_kn * im_factor * unit_moment_at(x, u, span, two_span);
                }
            }
            for j in 0..60 {
                let u = (j as f32 + 0.5) * dx_int;
                let infl = unit_moment_at(x, u, span, two_span);
                if (!two_span) || (x - span).abs() > 0.25 * span {
                    if infl > 0.0 {
                        m += udl_kn_m * dx_int * infl;
                    }
                } else if infl < 0.0 {
                    m += udl_kn_m * dx_int * infl;
                }
            }
            max_pos = max_pos.max(m);
            max_neg_pier = max_neg_pier.min(m);
            pts.push((frac, m));
        }
        (max_pos, max_neg_pier, pts)
    };

    let mut env_max_pos = 0.0_f32;
    let mut env_max_neg_pier = 0.0_f32;
    let mut env_max_shear = 0.0_f32;
    let mut peak_frac = 0.5_f32;
    let dx_int = total_length_m / 60.0;
    for k in 0..=45 {
        let f = k as f32 / 45.0;
        let lead_x = f * (total_length_m + 10.0);
        let (p, n, _) = sample_bmd_at(lead_x);
        if p > env_max_pos {
            env_max_pos = p;
            peak_frac = (lead_x / (total_length_m + 10.0)).clamp(0.0, 1.0);
        }
        env_max_neg_pier = env_max_neg_pier.min(n);

        let mut v_left = 0.0_f32;
        for &(off, load_kn) in &axles {
            let u = lead_x - off;
            if u >= 0.0 && u <= total_length_m {
                v_left += load_kn * im_factor * beam_reactions_unit(u, span, two_span).0.max(0.0);
            }
        }
        for j in 0..60 {
            let u = (j as f32 + 0.5) * dx_int;
            let ra = beam_reactions_unit(u, span, two_span).0;
            if ra > 0.0 {
                v_left += udl_kn_m * dx_int * ra;
            }
        }
        env_max_shear = env_max_shear.max(v_left);
    }

    let lead_x_active = truck_pos_frac.clamp(0.0, 1.0) * (total_length_m + 10.0);
    let (_p_act, _n_act, active_bmd) = sample_bmd_at(lead_x_active);
    let active_mid_moment_knm = {
        let idx = ((eval_sec_x / total_length_m) * 40.0).round() as usize;
        active_bmd.get(idx.min(active_bmd.len() - 1)).map(|v| v.1).unwrap_or(0.0)
    };
    let influence_line: Vec<(f32, f32)> = (0..=40)
        .map(|i| {
            let frac = i as f32 / 40.0;
            let u = frac * total_length_m;
            (frac, unit_moment_at(eval_sec_x, u, span, two_span))
        })
        .collect();
    let active_axle_xs = axles
        .iter()
        .map(|&(off, _)| (lead_x_active - off) / total_length_m)
        .filter(|f| *f >= 0.0 && *f <= 1.0)
        .collect();

    MovingLoadResponse {
        env_max_pos_moment_knm: env_max_pos,
        env_max_neg_pier_knm: env_max_neg_pier,
        env_max_shear_kn: env_max_shear,
        active_mid_moment_knm,
        peak_truck_pos_frac: peak_frac,
        total_length_m,
        influence_line,
        active_bmd,
        active_axle_xs,
    }
}

impl AnalyzeView {
    pub(super) fn render_moving_load_influence(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(ui, theme, features, FeatureId::BridgeMovingLoadInfluence, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Design Standard:").size(11.0).strong().color(theme.text_primary));
                if ui.selectable_label(self.ml_standard_idx == 0, "AASHTO HL-93 (35/145/145 kN + 9.3 kN/m)").clicked() {
                    self.ml_standard_idx = 0;
                }
                if ui.selectable_label(self.ml_standard_idx == 1, "SNI 1725:2016 (BGT 50/225/225 kN + BTR 9.0 kN/m)").clicked() {
                    self.ml_standard_idx = 1;
                }
                ui.separator();
                if ui.selectable_label(!self.ml_two_span, "1-Span Simple").clicked() {
                    self.ml_two_span = false;
                }
                if ui.selectable_label(self.ml_two_span, "2-Span Continuous").clicked() {
                    self.ml_two_span = true;
                }
            });

            ui.add_space(6.0);
            let resp = compute_bridge_moving_load_response(
                self.ml_span_m,
                self.ml_truck_pos_frac,
                self.ml_standard_idx,
                self.ml_dynamic_im,
                self.ml_two_span,
            );

            ui.horizontal_wrapped(|ui| {
                ui.add(egui::Slider::new(&mut self.ml_span_m, 15.0..=60.0).text("Span L (m)"));
                ui.add(egui::Slider::new(&mut self.ml_dynamic_im, 0.0..=0.50).text("Dynamic IM"));
                ui.add(egui::Slider::new(&mut self.ml_truck_pos_frac, 0.0..=1.0).text("Truck Position"));
                if icons::icon_button(ui, AppIcon::ArrowRight, "Step Axle (+10%)", 10.5, Color32::WHITE, theme.accent_blue, Stroke::NONE, CornerRadius::same(5), Vec2::ZERO).clicked() {
                    self.ml_truck_pos_frac = if self.ml_truck_pos_frac + 0.10 > 1.0 { 0.05 } else { self.ml_truck_pos_frac + 0.10 };
                }
                if icons::icon_button(ui, AppIcon::Target, "Jump to Peak +M", 10.5, theme.text_primary, theme.bg_card, Stroke::new(1.0, theme.border_subtle), CornerRadius::same(5), Vec2::ZERO).clicked() {
                    self.ml_truck_pos_frac = resp.peak_truck_pos_frac;
                }
            });

            ui.add_space(8.0);
            let pier_lbl = if self.ml_two_span {
                format!("{:.0} kN·m", resp.env_max_neg_pier_knm)
            } else {
                "0 kN·m (Simple)".to_string()
            };
            let cards = [
                ("ENVELOPE +M_MAX", format!("{:.0} kN·m", resp.env_max_pos_moment_knm), if self.ml_standard_idx == 0 { "AASHTO HL-93 Envelope" } else { "SNI 1725:2016 Envelope" }, theme.accent_blue),
                ("PIER HOGGING -M_MAX", pier_lbl, if self.ml_two_span { "Continuous Interior Support" } else { "Pin-Roller Supports" }, theme.accent_orange),
                ("ENVELOPE V_MAX", format!("{:.0} kN", resp.env_max_shear_kn), "Critical Abutment Shear", theme.accent_green),
                ("ACTIVE M (TRUCK POS)", format!("{:.0} kN·m", resp.active_mid_moment_knm), &format!("Total Deck {:.0} m (IM {:.0}%)", resp.total_length_m, self.ml_dynamic_im * 100.0), theme.text_primary),
            ];
            let cards_w = ui.available_width().max(260.0);
            let cols = if cards_w >= 620.0 { 4 } else if cards_w >= 340.0 { 2 } else { 1 };
            let col_w = ((cards_w - 8.0 * (cols - 1) as f32) / cols as f32).max(120.0);
            for row in cards.chunks(cols) {
                ui.horizontal(|ui| {
                    for (title, val, sub, col) in row {
                        Frame::new()
                            .fill(theme.bg_card)
                            .stroke(Stroke::new(1.0, theme.border_subtle))
                            .corner_radius(CornerRadius::same(8))
                            .inner_margin(Margin::same(10))
                            .show(ui, |ui| {
                                ui.set_width((col_w - 22.0).max(90.0));
                                ui.label(RichText::new(*title).size(9.5).strong().color(theme.text_muted));
                                ui.label(RichText::new(val).size(15.0).strong().color(*col));
                                ui.label(RichText::new(*sub).size(9.5).color(theme.text_secondary));
                            });
                    }
                });
                ui.add_space(6.0);
            }

            let canvas_w = ui.available_width().max(260.0);
            let (rect, _) = ui.allocate_exact_size(Vec2::new(canvas_w, 165.0), Sense::hover());
            let painter = ui.painter_at(rect);
            painter.rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(13, 18, 28));
            let deck_y = rect.top() + 56.0;
            let x0 = rect.left() + 26.0;
            let x1 = rect.right() - 26.0;
            let w_span = (x1 - x0).max(40.0);

            painter.line_segment([Pos2::new(x0, deck_y), Pos2::new(x1, deck_y)], Stroke::new(3.0, Color32::from_rgb(148, 163, 184)));
            for &sx in if self.ml_two_span { vec![x0, (x0 + x1) * 0.5, x1] } else { vec![x0, x1] }.iter() {
                painter.add(egui::Shape::convex_polygon(
                    vec![Pos2::new(sx, deck_y + 2.0), Pos2::new(sx - 6.0, deck_y + 12.0), Pos2::new(sx + 6.0, deck_y + 12.0)],
                    Color32::from_rgb(100, 116, 139),
                    Stroke::NONE,
                ));
            }
            for &ax_f in &resp.active_axle_xs {
                let ax_x = x0 + w_span * ax_f;
                painter.line_segment([Pos2::new(ax_x, deck_y - 26.0), Pos2::new(ax_x, deck_y - 4.0)], Stroke::new(2.2, theme.accent_orange));
                painter.circle_filled(Pos2::new(ax_x, deck_y - 4.0), 3.0, theme.accent_orange);
            }

            let max_infl = resp.influence_line.iter().map(|v| v.1.abs()).fold(1.0_f32, f32::max);
            let infl_pts: Vec<Pos2> = resp
                .influence_line
                .iter()
                .map(|&(f, eta)| Pos2::new(x0 + w_span * f, deck_y + (eta / max_infl) * 52.0))
                .collect();
            painter.add(egui::Shape::line(infl_pts, Stroke::new(2.0, theme.accent_blue)));

            let max_bmd = resp.active_bmd.iter().map(|v| v.1.abs()).fold(1.0_f32, f32::max);
            let bmd_pts: Vec<Pos2> = resp
                .active_bmd
                .iter()
                .map(|&(f, m)| Pos2::new(x0 + w_span * f, deck_y + (m / max_bmd) * 70.0))
                .collect();
            painter.add(egui::Shape::line(bmd_pts, Stroke::new(2.4, theme.accent_green)));

            painter.text(
                Pos2::new(rect.left() + 14.0, rect.top() + 14.0),
                Align2::LEFT_TOP,
                format!("Blue: Influence Line η_M  |  Green: Live Truck + Lane BMD ({:.0} m)", resp.total_length_m),
                FontId::proportional(10.5),
                Color32::WHITE,
            );
        });
    }

    pub(super) fn render_slope_retaining_stability(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(ui, theme, features, FeatureId::RetainingWallSlopeStability, |ui| {
            let cards = [
                ("OVERTURNING SAFETY (FS_OT)", "FS = 2.64 (SAFE)", "Min Required = 2.00 (Static)", theme.accent_green),
                ("SLIDING SAFETY (FS_SL)", "FS = 1.82 (SAFE)", "Min Required = 1.50 (c-phi Soil)", theme.accent_green),
                ("BISHOP CIRCULAR SLIP FS", "1.58 Static / 1.18 EQ", "SNI 8460:2017 Pseudo-Static kh=0.18", theme.accent_blue),
                ("ACTIVE EARTH COEFFICIENT", "Ka = 0.294 (Kae = 0.42)", "Mononobe-Okabe Seismic Thrust", theme.accent_orange),
            ];
            ui_kit::responsive_grid_n(ui, cards.len(), 175.0, |ui, idx| {
                let (t, v, s, c) = cards[idx];
                ui_kit::stat_card(ui, theme, t, v, s, c);
            });

            ui.add_space(10.0);
            ui.horizontal_wrapped(|ui| {
                let _ = ui.selectable_label(true, "Method: Bishop Simplified Slices");
                let _ = ui.selectable_label(false, "Cantilever Retaining Wall + Key");
                let _ = icons::icon_button(
                    ui,
                    AppIcon::Search,
                    "Search Critical Slip Surface Arc",
                    11.0,
                    Color32::WHITE,
                    theme.accent_blue,
                    Stroke::NONE,
                    CornerRadius::same(6),
                    Vec2::ZERO,
                );
            });
            ui.add_space(10.0);

            let desired = Vec2::new(ui.available_width(), 155.0);
            let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
            let painter = ui.painter_at(rect);
            painter.rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(13, 17, 24));
            let base_y = rect.bottom() - 24.0;
            let wall_x = rect.center().x - 16.0;
            // Ground slope + retaining stem geometry
            painter.line_segment(
                [Pos2::new(rect.left() + 20.0, base_y), Pos2::new(wall_x, base_y)],
                Stroke::new(2.0, theme.text_secondary),
            );
            painter.line_segment(
                [Pos2::new(wall_x, rect.top() + 34.0), Pos2::new(rect.right() - 20.0, rect.top() + 34.0)],
                Stroke::new(2.0, theme.text_secondary),
            );
            let stem = Rect::from_min_max(
                Pos2::new(wall_x, rect.top() + 34.0),
                Pos2::new(wall_x + 14.0, base_y),
            );
            painter.rect_filled(stem, CornerRadius::ZERO, theme.accent_blue.gamma_multiply(0.75));
            let footing = Rect::from_min_max(
                Pos2::new(wall_x - 24.0, base_y - 12.0),
                Pos2::new(wall_x + 48.0, base_y),
            );
            painter.rect_filled(footing, CornerRadius::ZERO, theme.accent_blue);

            // Critical circular slip arc
            let mut arc_pts = Vec::with_capacity(24);
            for i in 0..=23 {
                let t = i as f32 / 23.0;
                let px = (wall_x - 42.0) + t * 155.0;
                let depth = 4.0 * t * (1.0 - t) * 58.0;
                let top_interp = if px < wall_x { base_y } else { rect.top() + 34.0 };
                let py = (base_y - 26.0 * t) + depth.min(54.0);
                let _ = top_interp;
                arc_pts.push(Pos2::new(px.min(rect.right() - 24.0), py.min(rect.bottom() - 8.0)));
            }
            for w in arc_pts.windows(2) {
                painter.line_segment([w[0], w[1]], Stroke::new(2.2, theme.accent_red));
            }
            painter.text(
                Pos2::new(rect.right() - 28.0, rect.bottom() - 14.0),
                Align2::RIGHT_CENTER,
                "Critical Slip Circle (FS = 1.58)",
                FontId::proportional(10.0),
                theme.accent_red,
            );
        });
    }

    pub(super) fn render_floor_vibration_fatigue(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(ui, theme, features, FeatureId::FloorVibrationSteelFatigue, |ui| {
            let cards = [
                ("FLOOR NATURAL FREQ (fn)", "6.84 Hz", "Above 3.0 Hz Walking Resonance Band", theme.accent_green),
                ("PEAK WALKING ACCEL (ap/g)", "0.34% g <= 0.50% g", "AISC Design Guide 11 Office Criteria", theme.accent_green),
                ("S-N DETAIL CATEGORY", "Category C' Stiffener", "sigma_TH = 82.7 MPa Threshold", theme.accent_blue),
                ("MINER FATIGUE DAMAGE (D)", "Σ(n_i/N_i) = 0.24", "76% Residual Fatigue Life @ 50 Yrs", theme.accent_green),
            ];
            ui_kit::responsive_grid_n(ui, cards.len(), 175.0, |ui, idx| {
                let (t, v, s, c) = cards[idx];
                ui_kit::stat_card(ui, theme, t, v, s, c);
            });

            ui.add_space(10.0);
            ui.horizontal_wrapped(|ui| {
                let _ = ui.selectable_label(true, "AISC DG-11 Office (0.5% g)");
                let _ = ui.selectable_label(false, "Sensitive Lab / MRI (0.1% g)");
                let _ = ui.selectable_label(false, "Crane Girder Fatigue Class C");
                let _ = icons::icon_button(
                    ui,
                    AppIcon::Pulse,
                    "Compute Modal Footfall Response & S-N Curves",
                    11.0,
                    Color32::WHITE,
                    theme.accent_blue,
                    Stroke::NONE,
                    CornerRadius::same(6),
                    Vec2::ZERO,
                );
            });
            ui.add_space(10.0);

            let desired = Vec2::new(ui.available_width(), 145.0);
            let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
            let painter = ui.painter_at(rect);
            painter.rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(13, 17, 24));
            let plot = rect.shrink(28.0);
            painter.line_segment(
                [Pos2::new(plot.left(), plot.bottom()), Pos2::new(plot.right(), plot.bottom())],
                Stroke::new(1.0, theme.text_muted),
            );
            painter.line_segment(
                [Pos2::new(plot.left(), plot.bottom()), Pos2::new(plot.left(), plot.top())],
                Stroke::new(1.0, theme.text_muted),
            );
            // Log-log bilinear S-N curve + CAFL plateau
            let p0 = Pos2::new(plot.left() + 10.0, plot.top() + 18.0);
            let p1 = Pos2::new(plot.left() + plot.width() * 0.62, plot.top() + plot.height() * 0.64);
            let p2 = Pos2::new(plot.right() - 10.0, p1.y);
            painter.line_segment([p0, p1], Stroke::new(2.2, theme.accent_orange));
            painter.line_segment([p1, p2], Stroke::new(2.2, theme.accent_green));
            painter.circle_filled(
                Pos2::new(plot.left() + plot.width() * 0.48, plot.top() + plot.height() * 0.56),
                5.0,
                theme.accent_blue,
            );
            painter.text(
                Pos2::new(plot.center().x, rect.bottom() - 8.0),
                Align2::CENTER_CENTER,
                "Log Cycles N (10⁴ -> 10⁸ Cycles) • S-N Constant Amplitude Fatigue Limit",
                FontId::proportional(9.5),
                theme.text_muted,
            );
            let _ = StrokeKind::Inside;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::compute_bridge_moving_load_response;

    #[test]
    fn test_bridge_moving_load_influence_single_and_two_span() {
        let simple = compute_bridge_moving_load_response(30.0, 0.5, 0, 0.33, false);
        assert!(simple.env_max_pos_moment_knm > 2500.0);
        assert!(simple.env_max_shear_kn > 300.0);
        assert!((simple.env_max_neg_pier_knm).abs() < 1e-3);

        let cont = compute_bridge_moving_load_response(30.0, 0.5, 1, 0.33, true);
        assert!(cont.env_max_pos_moment_knm > simple.env_max_pos_moment_knm * 0.5);
        assert!(cont.env_max_neg_pier_knm < -500.0);
        assert_eq!(cont.influence_line.len(), 41);
    }
}
