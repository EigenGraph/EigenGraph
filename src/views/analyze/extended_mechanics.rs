//! views/analyze/extended_mechanics.rs — Bridge Moving Loads, Retaining Slope Stability & Floor Vibration / Fatigue.

use super::*;
use egui::{FontId, Rect, StrokeKind};

impl AnalyzeView {
    pub(super) fn render_moving_load_influence(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(ui, theme, features, FeatureId::BridgeMovingLoadInfluence, |ui| {
            let cards = [
                ("LIVE LOAD MODEL", "AASHTO HL-93 + SNI 1725", "Design Truck + 9.3 kN/m Lane + 33% IM", theme.accent_blue),
                ("PEAK SPAN MOMENT (+Mmax)", "4,280 kN·m @ 0.45L", "Critical Axle Train @ x = 14.4 m", theme.accent_green),
                ("MAX PIER REACTION (R_B)", "1,195 kN", "Tandem + Uniform Lane Envelope", theme.accent_orange),
                ("PERMIT OVERLOAD RATIO", "RF = 1.42 Operating", "HS-20 / HL-93 Load Rating Safe", theme.accent_green),
            ];
            ui_kit::responsive_grid_n(ui, cards.len(), 175.0, |ui, idx| {
                let (t, v, s, c) = cards[idx];
                ui_kit::stat_card(ui, theme, t, v, s, c);
            });

            ui.add_space(10.0);
            ui.horizontal_wrapped(|ui| {
                let _ = ui.selectable_label(true, "HL-93 Truck + Lane");
                let _ = ui.selectable_label(false, "SNI 1725 BGT + BTR");
                let _ = ui.selectable_label(false, "Envelope All Spans");
                let _ = ui.button("🚂 Sweep Vehicle Axles & Trace Influence Line");
            });
            ui.add_space(10.0);

            let desired = Vec2::new(ui.available_width(), 150.0);
            let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
            let painter = ui.painter_at(rect);
            painter.rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(13, 17, 24));
            let plot = rect.shrink(28.0);
            let zero_y = plot.top() + plot.height() * 0.55;
            painter.line_segment(
                [Pos2::new(plot.left(), zero_y), Pos2::new(plot.right(), zero_y)],
                Stroke::new(1.0, theme.text_muted),
            );
            let mut pts = Vec::with_capacity(41);
            for i in 0..=40 {
                let frac = i as f32 / 40.0;
                // Two-span continuous beam mid-span influence ordinate curve
                let ord = if frac < 0.5 {
                    let s = frac * 2.0;
                    (1.0 - (2.0 * s - 1.0).powi(2)) * 0.92
                } else {
                    let s = (frac - 0.5) * 2.0;
                    -(4.0 * s * (1.0 - s)) * 0.34
                };
                pts.push(Pos2::new(
                    plot.left() + frac * plot.width(),
                    zero_y - ord * (plot.height() * 0.50),
                ));
            }
            for w in pts.windows(2) {
                painter.line_segment([w[0], w[1]], Stroke::new(2.2, theme.accent_blue));
            }
            painter.circle_filled(pts[10], 5.0, theme.accent_green);
            painter.text(
                pts[10] + Vec2::new(0.0, -8.0),
                Align2::CENTER_BOTTOM,
                "+Mmax Influence Peak",
                FontId::proportional(10.0),
                theme.accent_green,
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
                let _ = ui.button("⛰️ Search Critical Slip Surface Arc");
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
                let _ = ui.button("〰️ Compute Modal Footfall Response & S-N Curves");
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
