//! views/industrialized/automation_slots.rs — Phase 31 Locked Preview Slots for
//! Precast Hauling & Splice Tolerance (10D-04) and 3D Concrete Printing Toolpaths (10D-05).

use super::*;
use egui::{Align2, FontId, Pos2, StrokeKind};

impl IndustrializedView {
    pub(super) fn render_precast_logistics_tolerance(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(
            ui,
            theme,
            features,
            FeatureId::PrecastTrailerJointTolerance,
            |ui| {
                let cards = [
                    (
                        "LOW-LOADER TRAILER AXLE",
                        "58.2 / 64.0 Ton",
                        "5-Axle Over-Dimension Permit Clear".to_string(),
                        theme.accent_green,
                    ),
                    (
                        "MIN BRIDGE CLEARANCE",
                        "4.82 m vs 4.55 m",
                        "+270 mm Highway Portal Margin".to_string(),
                        theme.accent_blue,
                    ),
                    (
                        "LASER SPLICE TOLERANCE",
                        "±2.4 mm Recorded",
                        "PCI MNL-135 Cap <= ±4.0 mm".to_string(),
                        theme.accent_green,
                    ),
                    (
                        "NON-SHRINK GROUT CUBE",
                        "68.5 MPa (7-Day)",
                        "Grouted Splice Sleeve Full Pullout Verified".to_string(),
                        theme.accent_blue,
                    ),
                ];
                self.render_responsive_cards(ui, &cards, theme);

                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    let _ = ui.selectable_label(true, "Corridor: Precast Yard -> Gate B-2");
                    let _ = ui.selectable_label(false, "Laser Tracker Millimeter QA");
                    let _ = ui.button("🚛 Run Highway Envelope & Joint Fit-Up Check");
                });
                ui.add_space(10.0);

                ScrollArea::horizontal()
                    .id_salt("precast_tolerance_scroll")
                    .show(ui, |ui| {
                        egui::Grid::new("precast_tolerance_grid")
                            .striped(true)
                            .min_col_width(90.0)
                            .spacing(Vec2::new(14.0, 6.0))
                            .show(ui, |ui| {
                                for h in [
                                    "Module ID",
                                    "Mass / Length",
                                    "Trailer Permit",
                                    "Delta X / Y",
                                    "Grout Strength",
                                    "QA Gate",
                                ] {
                                    ui.label(
                                        RichText::new(h).size(10.5).strong().color(theme.accent_blue),
                                    );
                                }
                                ui.end_row();
                                for (id, wt, perm, dxy, gr, st, col) in [
                                    (
                                        "PC-COL-01",
                                        "18.4T / 11.2m",
                                        "Night Escort OK",
                                        "Δx=+1.8mm Δy=-1.2mm",
                                        "71.0 MPa",
                                        "PASS",
                                        theme.accent_green,
                                    ),
                                    (
                                        "PC-GRD-04",
                                        "29.8T / 16.5m",
                                        "Steerable Dolly",
                                        "Δx=+3.4mm Δy=+2.1mm",
                                        "68.5 MPa",
                                        "PASS (Tight)",
                                        theme.accent_orange,
                                    ),
                                    (
                                        "PC-COR-02",
                                        "22.1T / 8.8m",
                                        "Standard Flatbed",
                                        "Δx=-0.9mm Δy=+1.0mm",
                                        "74.2 MPa",
                                        "PASS",
                                        theme.accent_green,
                                    ),
                                ] {
                                    ui.label(
                                        RichText::new(id)
                                            .size(10.5)
                                            .monospace()
                                            .color(theme.accent_blue),
                                    );
                                    ui.label(
                                        RichText::new(wt).size(10.5).color(theme.text_primary),
                                    );
                                    ui.label(
                                        RichText::new(perm)
                                            .size(10.5)
                                            .color(theme.text_secondary),
                                    );
                                    ui.label(
                                        RichText::new(dxy)
                                            .size(10.5)
                                            .monospace()
                                            .color(theme.text_secondary),
                                    );
                                    ui.label(
                                        RichText::new(gr)
                                            .size(10.5)
                                            .color(theme.accent_blue),
                                    );
                                    ui.label(RichText::new(st).size(10.5).strong().color(col));
                                    ui.end_row();
                                }
                            });
                    });
            },
        );
    }

    pub(super) fn render_concrete_3d_printing(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(
            ui,
            theme,
            features,
            FeatureId::Concrete3DPrintingGcodeToolpath,
            |ui| {
                let cards = [
                    (
                        "EXTRUSION NOZZLE",
                        "45 × 15 mm Layer",
                        "118 Sliced Contours (Total H = 1.77 m)".to_string(),
                        theme.accent_blue,
                    ),
                    (
                        "PRINT HEAD FEED RATE",
                        "185 mm/s Constant",
                        "Stepover 2-Perimeter Truss Infill".to_string(),
                        theme.accent_green,
                    ),
                    (
                        "OPEN-TIME LAYER WINDOW",
                        "11.4 min / Layer",
                        "Max Cold-Joint Cap = 18.0 min (Safe)".to_string(),
                        theme.accent_green,
                    ),
                    (
                        "ESTIMATED MORTAR VOLUME",
                        "6.42 m³ Fiber-Mix",
                        "Pump Pressure = 1.8 MPa @ Hose".to_string(),
                        theme.accent_orange,
                    ),
                ];
                self.render_responsive_cards(ui, &cards, theme);

                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    let _ = ui.selectable_label(true, "Slicer: Continuous Spiralize Contour");
                    let _ = ui.selectable_label(false, "6-Axis Gantry Kinematic Limits");
                    let _ = ui.button("🖨️ Slice Selected Element & Export 3DCP G-Code (.gcode)");
                });
                ui.add_space(12.0);

                ui_kit::responsive_two_panes(ui, 640.0, |ui, pane| match pane {
                    0 => {
                        ui.label(
                            RichText::new("3DCP LAYER TOOLPATH CONTOUR PREVIEW (Z = 0.45 m)")
                                .size(11.0)
                                .strong()
                                .color(theme.accent_blue),
                        );
                        ui.add_space(6.0);
                        let desired = Vec2::new(ui.available_width(), 160.0);
                        let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
                        let painter = ui.painter_at(rect);
                        painter.rect_filled(
                            rect,
                            CornerRadius::same(8),
                            Color32::from_rgb(13, 17, 24),
                        );
                        let pad = rect.shrink(24.0);
                        // Outer & Inner shell loops
                        painter.rect_stroke(
                            pad,
                            CornerRadius::same(10),
                            Stroke::new(2.2, theme.accent_blue),
                            StrokeKind::Inside,
                        );
                        painter.rect_stroke(
                            pad.shrink(12.0),
                            CornerRadius::same(6),
                            Stroke::new(2.0, theme.accent_green),
                            StrokeKind::Inside,
                        );
                        // Sinusoidal internal truss infill
                        let inner = pad.shrink(12.0);
                        let mut zig = Vec::with_capacity(17);
                        for i in 0..=16 {
                            let frac = i as f32 / 16.0;
                            let px = inner.left() + frac * inner.width();
                            let py = if i % 2 == 0 {
                                inner.top() + 3.0
                            } else {
                                inner.bottom() - 3.0
                            };
                            zig.push(Pos2::new(px, py));
                        }
                        for w in zig.windows(2) {
                            painter.line_segment([w[0], w[1]], Stroke::new(1.6, theme.accent_orange));
                        }
                        painter.text(
                            Pos2::new(rect.center().x, rect.bottom() - 10.0),
                            Align2::CENTER_CENTER,
                            "Zero-Travel Sinusoidal Lattice Infill (f'c,28 = 55 MPa)",
                            FontId::monospace(9.0),
                            theme.text_secondary,
                        );
                    }
                    _ => {
                        ui.label(
                            RichText::new("EXTRUDED G-CODE & PUMP STREAM SNIPPET")
                                .size(11.0)
                                .strong()
                                .color(theme.accent_blue),
                        );
                        ui.add_space(6.0);
                        Frame::new()
                            .fill(Color32::from_rgb(13, 17, 24))
                            .corner_radius(CornerRadius::same(8))
                            .inner_margin(Margin::same(10))
                            .show(ui, |ui| {
                                ui.set_width(ui.available_width());
                                ui.label(
                                    RichText::new(
                                        "; EigenGraph 3DCP Contour Slicer v1.0\n\
                                         G90 ; Absolute mm\n\
                                         M82 ; Pump flow sync enabled (1.8 MPa)\n\
                                         G1 X120.0 Y140.0 Z450.0 F11100 E12.40\n\
                                         G1 X2850.0 Y140.0 Z450.0 E118.65 ; Outer Bead\n\
                                         G1 X2850.0 Y920.0 Z450.0 E152.10 ; Sinusoid Turn\n\
                                         G1 X120.0 Y920.0 Z465.0 F11100 E264.80 ; Z+15mm",
                                    )
                                    .size(10.5)
                                    .monospace()
                                    .color(theme.accent_green),
                                );
                            });
                    }
                });
            },
        );
    }
}
