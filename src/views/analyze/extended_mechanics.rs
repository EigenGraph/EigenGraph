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
}

#[derive(Clone, Debug)]
pub struct RetainingSlopeResult {
    pub ka: f32,
    pub kae: f32,
    pub fs_overturning: f32,
    pub fs_sliding: f32,
    pub bishop_fs_static: f32,
    pub bishop_fs_seismic: f32,
    pub q_toe_kpa: f32,
}

/// Computes Rankine / Mononobe-Okabe cantilever retaining wall stability & Bishop Simplified circular slip FS.
pub fn compute_retaining_wall_bishop_stability(
    wall_h_m: f32,
    base_b_m: f32,
    phi_deg: f32,
    cohesion_kpa: f32,
    gamma_knm3: f32,
    kh: f32,
    has_shear_key: bool,
    surcharge_kpa: f32,
) -> RetainingSlopeResult {
    let h = wall_h_m.clamp(1.5, 18.0);
    let b = base_b_m.clamp(1.0, 14.0);
    let phi = phi_deg.clamp(15.0, 45.0).to_radians();
    let ka = ((std::f32::consts::FRAC_PI_4 - 0.5 * phi).tan()).powi(2);

    // Mononobe-Okabe dynamic active earth pressure coefficient (kv = 0, i = 0, beta = 90)
    let theta = (kh.clamp(0.0, 0.45)).atan();
    let denom_mo = if phi > theta {
        let rt = (phi.sin() * (phi - theta).sin() / (theta.cos())).max(0.0).sqrt();
        theta.cos() * (1.0 + rt).powi(2)
    } else {
        theta.cos()
    };
    let kae = (phi.cos().powi(2) / denom_mo.max(0.15)).clamp(ka, 1.25);

    // Thrusts per meter width (static + surcharge)
    let pa_soil = 0.5 * ka * gamma_knm3 * h * h;
    let pa_sur = ka * surcharge_kpa * h;
    let pa_dyn_incr = 0.5 * (kae - ka) * gamma_knm3 * h * h;

    // Wall concrete geometry + soil on heel
    let t_stem = (0.30 + h * 0.06).clamp(0.30, 0.85);
    let t_base = (0.35 + h * 0.05).clamp(0.35, 0.90);
    let heel_w = (b - 0.30 * b - t_stem).max(0.35 * b);
    let w_conc = (t_stem * (h - t_base) + b * t_base) * 24.0;
    let w_soil = heel_w * (h - t_base) * gamma_knm3 + heel_w * surcharge_kpa;
    let total_v = (w_conc + w_soil).max(1.0);

    let m_res = w_conc * (0.35 * b) + w_soil * (b - 0.5 * heel_w);
    let m_ot = pa_soil * (h / 3.0) + pa_sur * (h / 2.0);
    let fs_overturning = (m_res / m_ot.max(1e-3)).clamp(0.2, 12.0);

    let mu_base = (0.67 * phi).tan();
    let key_passive = if has_shear_key {
        0.5 * (1.0 / ka.max(0.15)) * gamma_knm3 * 0.90 * 0.90
    } else {
        0.0
    };
    let res_slide = total_v * mu_base + cohesion_kpa * 0.5 * b + key_passive;
    let fs_sliding = (res_slide / (pa_soil + pa_sur + 0.5 * pa_dyn_incr).max(1e-3)).clamp(0.2, 10.0);

    let e_m = ((m_res - m_ot) / total_v - 0.5 * b).abs().min(0.45 * b);
    let q_toe_kpa = (total_v / b) * (1.0 + 6.0 * e_m / b);

    // Bishop Simplified iterative method-of-slices along critical circle behind wall toe
    let mut fs_static = 1.60_f32;
    let mut fs_seismic = 1.25_f32;
    let n_slices = 10usize;
    let slice_b = (1.6 * h) / (n_slices as f32);
    for _pass in 0..8 {
        let mut num_s = 0.0_f32;
        let mut den_s = 0.0_f32;
        let mut num_eq = 0.0_f32;
        let mut den_eq = 0.0_f32;
        for i in 0..n_slices {
            let frac = (i as f32 + 0.5) / n_slices as f32;
            let sin_a = (-0.20 + 0.85 * frac).clamp(-0.7, 0.78);
            let cos_a = (1.0 - sin_a * sin_a).sqrt();
            let h_i = (4.0 * frac * (1.0 - frac) * h * 0.82).max(0.35);
            let w_i = h_i * slice_b * gamma_knm3 + slice_b * surcharge_kpa;
            let m_a_s = (cos_a + sin_a * phi.tan() / fs_static.max(0.3)).max(0.25);
            let m_a_eq = (cos_a + sin_a * phi.tan() / fs_seismic.max(0.3)).max(0.25);
            let res_i = cohesion_kpa * slice_b + w_i * phi.tan();
            num_s += res_i / m_a_s;
            den_s += (w_i * sin_a).max(0.1);
            num_eq += res_i / m_a_eq;
            den_eq += (w_i * sin_a + w_i * kh * 0.55).max(0.1);
        }
        fs_static = (num_s / den_s.max(0.1)).clamp(0.4, 5.5);
        fs_seismic = (num_eq / den_eq.max(0.1)).clamp(0.3, 4.5);
    }

    RetainingSlopeResult {
        ka,
        kae,
        fs_overturning,
        fs_sliding,
        bishop_fs_static: fs_static,
        bishop_fs_seismic: fs_seismic,
        q_toe_kpa,
    }
}

impl AnalyzeView {
    pub(super) fn render_slope_retaining_stability(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(ui, theme, features, FeatureId::RetainingWallSlopeStability, |ui| {
            let res = compute_retaining_wall_bishop_stability(
                self.rw_wall_height_m,
                self.rw_base_width_m,
                self.rw_backfill_phi_deg,
                self.rw_cohesion_kpa,
                self.rw_unit_weight_knm3,
                self.rw_kh_seismic,
                self.rw_shear_key,
                self.rw_surcharge_kpa,
            );
            let ot_txt = format!(
                "FS = {:.2} ({})",
                res.fs_overturning,
                if res.fs_overturning >= 2.0 { "SAFE" } else { "CHECK" }
            );
            let sl_txt = format!(
                "FS = {:.2} ({})",
                res.fs_sliding,
                if res.fs_sliding >= 1.5 { "SAFE" } else { "KEY REQ" }
            );
            let bp_txt = format!("{:.2} Static / {:.2} EQ", res.bishop_fs_static, res.bishop_fs_seismic);
            let ka_txt = format!("Ka = {:.3} (Kae = {:.3})", res.ka, res.kae);
            let cards = [
                (
                    "OVERTURNING SAFETY (FS_OT)",
                    ot_txt.as_str(),
                    "Min Required = 2.00 (Static)",
                    if res.fs_overturning >= 2.0 { theme.accent_green } else { theme.accent_red },
                ),
                (
                    "SLIDING SAFETY (FS_SL)",
                    sl_txt.as_str(),
                    if self.rw_shear_key { "Shear Key Passive Engaged" } else { "Base Friction Only" },
                    if res.fs_sliding >= 1.5 { theme.accent_green } else { theme.accent_orange },
                ),
                (
                    "BISHOP CIRCULAR SLIP FS",
                    bp_txt.as_str(),
                    "SNI 8460:2017 Pseudo-Static Slices",
                    if res.bishop_fs_seismic >= 1.10 { theme.accent_blue } else { theme.accent_orange },
                ),
                (
                    "ACTIVE & TOE BEARING",
                    ka_txt.as_str(),
                    "Mononobe-Okabe Seismic Thrust",
                    theme.accent_orange,
                ),
            ];
            ui_kit::responsive_grid_n(ui, cards.len(), 175.0, |ui, idx| {
                let (t, v, s, c) = cards[idx];
                ui_kit::stat_card(ui, theme, t, v, s, c);
            });

            ui.add_space(10.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("H (m):").size(10.5).color(theme.text_muted));
                ui.add(egui::DragValue::new(&mut self.rw_wall_height_m).range(2.0..=14.0).speed(0.1).max_decimals(1));
                ui.label(RichText::new("Base B (m):").size(10.5).color(theme.text_muted));
                ui.add(egui::DragValue::new(&mut self.rw_base_width_m).range(1.5..=10.0).speed(0.1).max_decimals(1));
                ui.label(RichText::new("φ (°):").size(10.5).color(theme.text_muted));
                ui.add(egui::DragValue::new(&mut self.rw_backfill_phi_deg).range(18.0..=42.0).speed(0.5).max_decimals(0));
                ui.label(RichText::new("c' (kPa):").size(10.5).color(theme.text_muted));
                ui.add(egui::DragValue::new(&mut self.rw_cohesion_kpa).range(0.0..=60.0).speed(1.0).max_decimals(0));
                ui.label(RichText::new("kh (g):").size(10.5).color(theme.text_muted));
                ui.add(egui::DragValue::new(&mut self.rw_kh_seismic).range(0.0..=0.40).speed(0.01).max_decimals(2));
                if ui.selectable_label(self.rw_shear_key, "Base Shear Key").clicked() {
                    self.rw_shear_key = !self.rw_shear_key;
                }
                if icons::icon_button(
                    ui,
                    AppIcon::Search,
                    "Sweep Critical Slip Circle",
                    10.5,
                    Color32::WHITE,
                    theme.accent_blue,
                    Stroke::NONE,
                    CornerRadius::same(6),
                    Vec2::ZERO,
                )
                .clicked()
                {
                    self.export_status = Some(format!(
                        "Bishop Slip FS = {:.2} (Seismic {:.2}), q_toe = {:.1} kPa",
                        res.bishop_fs_static, res.bishop_fs_seismic, res.q_toe_kpa
                    ));
                }
            });
            ui.add_space(10.0);

            let desired = Vec2::new(ui.available_width(), 165.0);
            let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
            let painter = ui.painter_at(rect);
            painter.rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(13, 17, 24));
            let base_y = rect.bottom() - 26.0;
            let wall_x = rect.center().x - 16.0;
            let wall_h_px = (self.rw_wall_height_m * 15.0).clamp(48.0, rect.height() - 48.0);
            let base_w_px = (self.rw_base_width_m * 16.0).clamp(48.0, 150.0);
            let top_y = base_y - wall_h_px;
            // Ground slope + retaining stem geometry
            painter.line_segment(
                [Pos2::new(rect.left() + 20.0, base_y), Pos2::new(wall_x, base_y)],
                Stroke::new(2.0, theme.text_secondary),
            );
            painter.line_segment(
                [Pos2::new(wall_x, top_y), Pos2::new(rect.right() - 20.0, top_y)],
                Stroke::new(2.0, theme.text_secondary),
            );
            let stem = Rect::from_min_max(
                Pos2::new(wall_x, top_y),
                Pos2::new(wall_x + 14.0, base_y),
            );
            painter.rect_filled(stem, CornerRadius::ZERO, theme.accent_blue.gamma_multiply(0.75));
            let footing = Rect::from_min_max(
                Pos2::new(wall_x - base_w_px * 0.30, base_y - 12.0),
                Pos2::new(wall_x + base_w_px * 0.70, base_y),
            );
            painter.rect_filled(footing, CornerRadius::ZERO, theme.accent_blue);
            if self.rw_shear_key {
                let key = Rect::from_min_max(
                    Pos2::new(wall_x, base_y),
                    Pos2::new(wall_x + 12.0, base_y + 12.0),
                );
                painter.rect_filled(key, CornerRadius::ZERO, theme.accent_green);
            }

            // Critical circular slip arc
            let mut arc_pts = Vec::with_capacity(24);
            for i in 0..=23 {
                let t = i as f32 / 23.0;
                let px = (wall_x - 42.0) + t * (base_w_px + 96.0);
                let depth = 4.0 * t * (1.0 - t) * 56.0;
                let py = (base_y - wall_h_px * 0.30 * t) + depth.min(54.0);
                arc_pts.push(Pos2::new(px.min(rect.right() - 24.0), py.min(rect.bottom() - 8.0)));
            }
            for w in arc_pts.windows(2) {
                painter.line_segment([w[0], w[1]], Stroke::new(2.2, theme.accent_red));
            }
            painter.text(
                Pos2::new(rect.right() - 20.0, rect.bottom() - 12.0),
                Align2::RIGHT_CENTER,
                format!(
                    "Bishop Slip Circle (FS = {:.2} Static / {:.2} EQ) • q_toe = {:.0} kPa",
                    res.bishop_fs_static, res.bishop_fs_seismic, res.q_toe_kpa
                ),
                FontId::proportional(10.0),
                theme.accent_red,
            );
        });
    }
}

#[derive(Clone, Debug)]
pub struct FloorVibrationFatigueReport {
    pub fn_hz: f32,
    pub ap_over_g_pct: f32,
    pub accel_limit_pct: f32,
    pub pass_vibration: bool,
    pub detail_label: &'static str,
    pub cafl_mpa: f32,
    pub n_allowable_millions: f32,
    pub miner_damage: f32,
    pub residual_life_pct: f32,
}

/// Computes AISC Design Guide 11 composite floor footfall vibration and EN 1993-1-9 / AISC 360 Miner steel fatigue.
pub fn compute_floor_vibration_and_fatigue(
    span_m: f32,
    spacing_m: f32,
    slab_mm: f32,
    damping_beta: f32,
    occupancy_idx: usize,
    fatigue_cat_idx: usize,
    stress_range_mpa: f32,
    cycles_millions: f32,
) -> FloorVibrationFatigueReport {
    let l = span_m.clamp(3.0, 24.0);
    let s = spacing_m.clamp(1.5, 6.0);
    let t = slab_mm.clamp(80.0, 300.0);
    let beta = damping_beta.clamp(0.01, 0.08);

    // Effective beam+slab stiffness scaling for WF composite bay
    let slab_kpa = 2.40 * (t / 100.0) + 1.50; // DL + sustained walking LL
    let w_eff_kn = slab_kpa * l * s * 1.45;
    let delta_mm = 1.65 * (l / 9.0).powi(3) * (130.0 / t).powf(0.45);
    let fn_hz = (0.18 * (9810.0 / delta_mm.max(0.25)).sqrt()).clamp(2.2, 18.0);

    let p0 = match occupancy_idx {
        1 => 0.29, // Sensitive Lab (quiet step)
        2 => 0.41, // Footbridge / Crane aisle
        _ => 0.29, // Office / Residential
    };
    let accel_limit_pct = match occupancy_idx {
        1 => 0.15,
        2 => 1.50,
        _ => 0.50,
    };
    let ap_over_g_pct = ((p0 * (-0.35 * fn_hz).exp()) / (beta * w_eff_kn.max(20.0)) * 100.0)
        .clamp(0.02, 5.0);
    let pass_vibration = ap_over_g_pct <= accel_limit_pct;

    let (detail_label, cafl_mpa, c_const) = match fatigue_cat_idx {
        0 => ("Cat B (Welded Plate Girder)", 110.0_f32, 3.93e12_f32),
        1 => ("Cat C' (Transverse Stiffener)", 82.7_f32, 1.44e12_f32),
        _ => ("Cat D/E (Cover Plate End)", 48.3_f32, 7.21e11_f32),
    };
    let dsig = stress_range_mpa.clamp(10.0, 350.0);
    let n_cap = if dsig < cafl_mpa * 0.55 {
        200.0e6
    } else if dsig < cafl_mpa {
        // EN 1993-1-9 Haibach slope m2 = 5 below CAFL
        (2.0e6 * (cafl_mpa / dsig).powf(5.0)).clamp(1.0e5, 2.0e8)
    } else {
        (c_const / dsig.powi(3)).clamp(1.0e3, 2.0e8)
    };
    let n_allowable_millions = n_cap / 1.0e6;
    let miner_damage = (cycles_millions.max(0.01) / n_allowable_millions.max(1e-3)).clamp(0.001, 5.0);
    let residual_life_pct = ((1.0 - miner_damage).max(0.0)) * 100.0;

    FloorVibrationFatigueReport {
        fn_hz,
        ap_over_g_pct,
        accel_limit_pct,
        pass_vibration,
        detail_label,
        cafl_mpa,
        n_allowable_millions,
        miner_damage,
        residual_life_pct,
    }
}

impl super::AnalyzeView {
    pub(super) fn render_floor_vibration_fatigue(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        let rep = compute_floor_vibration_and_fatigue(
            self.vib_span_m,
            self.vib_spacing_m,
            self.vib_slab_mm,
            self.vib_damping_beta,
            self.vib_occupancy_idx,
            self.fatigue_cat_idx,
            self.fatigue_stress_range_mpa,
            self.fatigue_cycles_millions,
        );
        ui_kit::feature_slot_card(ui, theme, features, FeatureId::FloorVibrationSteelFatigue, |ui| {
            let vib_col = if rep.pass_vibration { theme.accent_green } else { theme.accent_red };
            let miner_col = if rep.miner_damage <= 1.0 { theme.accent_green } else { theme.accent_red };
            let cards = [
                ("FLOOR NATURAL FREQ (fn)", format!("{:.2} Hz", rep.fn_hz), (if rep.fn_hz >= 4.0 { "Above 3.0 Hz Walking Resonance" } else { "Low-Frequency Sway Band (<4Hz)" }).to_string(), vib_col),
                ("PEAK WALKING ACCEL (ap/g)", format!("{:.2}% g <= {:.2}% g", rep.ap_over_g_pct, rep.accel_limit_pct), (if rep.pass_vibration { "PASS AISC DG-11" } else { "EXCEEDS DG-11 LIMIT" }).to_string(), vib_col),
                ("S-N DETAIL CATEGORY", rep.detail_label.to_string(), format!("CAFL sigma_TH = {:.1} MPa • Ncap {:.2}M", rep.cafl_mpa, rep.n_allowable_millions), theme.accent_blue),
                ("MINER FATIGUE DAMAGE (D)", format!("Sum(n_i/N_i) = {:.3}", rep.miner_damage), format!("{:.0}% Residual Fatigue Life", rep.residual_life_pct), miner_col),
            ];
            ui_kit::responsive_grid_n(ui, cards.len(), 175.0, |ui, idx| {
                let (t, v, s, c) = &cards[idx];
                ui_kit::stat_card(ui, theme, *t, v, s, *c);
            });

            ui.add_space(10.0);
            ui.horizontal_wrapped(|ui| {
                ui.selectable_value(&mut self.vib_occupancy_idx, 0, "AISC DG-11 Office (0.50% g)");
                ui.selectable_value(&mut self.vib_occupancy_idx, 1, "Sensitive Lab / MRI (0.15% g)");
                ui.selectable_value(&mut self.vib_occupancy_idx, 2, "Crane / Indoor Walkway (1.50% g)");
                ui.separator();
                ui.selectable_value(&mut self.fatigue_cat_idx, 0, "Cat B (110 MPa)");
                ui.selectable_value(&mut self.fatigue_cat_idx, 1, "Cat C' (82.7 MPa)");
                ui.selectable_value(&mut self.fatigue_cat_idx, 2, "Cat D/E (48.3 MPa)");
            });
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                ui.add(egui::Slider::new(&mut self.vib_span_m, 4.0..=20.0).text("Bay Span (m)"));
                ui.add(egui::Slider::new(&mut self.vib_slab_mm, 90.0..=250.0).text("Slab (mm)"));
                ui.add(egui::Slider::new(&mut self.fatigue_stress_range_mpa, 20.0..=220.0).text("Cyclic dSigma (MPa)"));
                ui.add(egui::Slider::new(&mut self.fatigue_cycles_millions, 0.2..=25.0).text("Design Cycles (M)"));
            });
            ui.add_space(8.0);

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
            // Log-log bilinear S-N curve + CAFL plateau + operating point
            let cafl_frac = (1.0_f32 - (rep.cafl_mpa / 300.0_f32)).clamp(0.35, 0.80);
            let p0 = Pos2::new(plot.left() + 10.0, plot.top() + 16.0);
            let p1 = Pos2::new(plot.left() + plot.width() * 0.62, plot.top() + plot.height() * cafl_frac);
            let p2 = Pos2::new(plot.right() - 10.0, p1.y);
            painter.line_segment([p0, p1], Stroke::new(2.2, theme.accent_orange));
            painter.line_segment([p1, p2], Stroke::new(2.2, theme.accent_green));

            let op_x = ((self.fatigue_cycles_millions.max(0.01).log10() + 1.0) / 3.0).clamp(0.08, 0.92);
            let op_y = (self.fatigue_stress_range_mpa / 240.0).clamp(0.10, 0.92);
            let op_pt = Pos2::new(plot.left() + op_x * plot.width(), plot.bottom() - op_y * plot.height());
            painter.circle_filled(op_pt, 5.5, miner_col);
            painter.text(
                op_pt + Vec2::new(8.0, -4.0),
                Align2::LEFT_BOTTOM,
                format!("{:.0} MPa @ {:.1}M (D={:.2})", self.fatigue_stress_range_mpa, self.fatigue_cycles_millions, rep.miner_damage),
                FontId::proportional(10.0),
                miner_col,
            );
            painter.text(
                Pos2::new(plot.center().x, rect.bottom() - 8.0),
                Align2::CENTER_CENTER,
                format!("Log Cycles N (10^4 -> 10^8) • {} (CAFL {:.1} MPa)", rep.detail_label, rep.cafl_mpa),
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

    #[test]
    fn test_retaining_wall_and_bishop_circular_slip_solver() {
        let with_key = super::compute_retaining_wall_bishop_stability(
            5.5, 3.8, 32.0, 12.0, 18.5, 0.18, true, 15.0,
        );
        let no_key = super::compute_retaining_wall_bishop_stability(
            5.5, 3.8, 32.0, 12.0, 18.5, 0.18, false, 15.0,
        );
        assert!(with_key.ka < with_key.kae);
        assert!(with_key.fs_sliding > no_key.fs_sliding);
        assert!(with_key.bishop_fs_static > with_key.bishop_fs_seismic);
    }

    #[test]
    fn test_aisc_dg11_vibration_and_miner_fatigue() {
        let ok = super::compute_floor_vibration_and_fatigue(
            8.0, 3.0, 140.0, 0.03, 0, 1, 55.0, 1.5,
        );
        assert!(ok.fn_hz > 5.0);
        assert!(ok.pass_vibration);
        assert!(ok.miner_damage < 1.0);

        let severe = super::compute_floor_vibration_and_fatigue(
            14.0, 4.0, 110.0, 0.015, 1, 2, 165.0, 12.0,
        );
        assert!(severe.miner_damage > 1.0);
        assert!(severe.ap_over_g_pct > ok.ap_over_g_pct);
    }
}
