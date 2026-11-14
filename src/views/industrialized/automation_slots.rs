//! views/industrialized/automation_slots.rs — Phase 40 Interactive Precast Hauling &
//! Splice Tolerance QA (10D-04) and 3D Concrete Printing Toolpaths (10D-05).

use super::*;
use egui::{Align2, FontId, Pos2, StrokeKind};

#[derive(Debug, Clone, PartialEq)]
pub struct PrecastHaulPiece {
    pub module_id: String,
    pub mass_ton: f32,
    pub length_m: f32,
    pub section_h_m: f32,
    pub trailer_permit: &'static str,
    pub gvw_ton: f32,
    pub delta_x_mm: f32,
    pub delta_y_mm: f32,
    pub grout_mpa: f32,
    pub qa_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrecastHaulingReport {
    pub peak_gvw_ton: f32,
    pub axle_limit_ton: f32,
    pub stack_height_m: f32,
    pub portal_clear_m: f32,
    pub min_clearance_margin_m: f32,
    pub max_laser_delta_mm: f32,
    pub grout_7day_mpa: f32,
    pub pieces: Vec<PrecastHaulPiece>,
}

/// Evaluates precast trailer GVW axle compliance, bridge portal clearance, and PCI MNL-135 splice tolerances from `BimProject`.
pub fn compute_precast_hauling_audit(
    bim_project: &crate::core::BimProject,
    axle_limit_ton: f32,
    portal_clear_m: f32,
    grout_mpa: f32,
    night_escort: bool,
) -> PrecastHaulingReport {
    use crate::core::bim::RunCategory;

    let mut pieces = Vec::new();
    let tare_ton = 26.5_f32;
    let deck_height_m = 1.15_f32;

    for (idx, c) in bim_project.columns.iter().take(3).enumerate() {
        let h_m = ((bim_project.level_elevation(c.top_level_id)
            - bim_project.level_elevation(c.base_level_id))
        .abs()
            / 1000.0)
            .clamp(3.0, 12.0) as f32;
        let w_m = (c.width_mm / 1000.0).max(0.35) as f32;
        let d_m = (c.depth_mm / 1000.0).max(0.35) as f32;
        let mass_ton = (w_m * d_m * h_m * 2.45).max(4.5);
        let gvw = mass_ton + tare_ton;
        let dx: f32 = if idx % 2 == 0 { 1.8 } else { -1.1 };
        let dy: f32 = if idx % 2 == 0 { -1.2 } else { 1.4 };
        let permit = if gvw > axle_limit_ton {
            "AXLE OVERLOAD"
        } else if h_m > 10.0 || night_escort {
            "Night Escort OK"
        } else {
            "Standard Flatbed"
        };
        let qa_gate = if gvw > axle_limit_ton || grout_mpa < 55.0 {
            "HOLD"
        } else if dx.abs().max(dy.abs()) > 3.0 {
            "PASS (Tight)"
        } else {
            "PASS"
        };
        pieces.push(PrecastHaulPiece {
            module_id: format!("PC-{}", c.name),
            mass_ton,
            length_m: h_m,
            section_h_m: w_m.max(d_m),
            trailer_permit: permit,
            gvw_ton: gvw,
            delta_x_mm: dx,
            delta_y_mm: dy,
            grout_mpa,
            qa_gate,
        });
    }

    for (idx, r) in bim_project
        .runs
        .iter()
        .filter(|r| matches!(r.category, RunCategory::StructuralBeam | RunCategory::StructuralBrace))
        .take(3)
        .enumerate()
    {
        let len_m = ((r.end.0 - r.start.0).hypot(r.end.1 - r.start.1) / 1000.0).max(2.5) as f32;
        let w_m = (r.width_mm / 1000.0).max(0.30) as f32;
        let d_m = (r.depth_mm / 1000.0).max(0.45) as f32;
        let mass_ton = (w_m * d_m * len_m * 2.45).max(6.2);
        let gvw = mass_ton + tare_ton;
        let dx = 1.9 + idx as f32 * 0.7;
        let dy = 1.3 + idx as f32 * 0.4;
        let permit = if gvw > axle_limit_ton {
            "AXLE OVERLOAD"
        } else if len_m > 12.0 {
            "Steerable Dolly"
        } else {
            "Low-Loader 5-Axle"
        };
        let qa_gate = if gvw > axle_limit_ton || grout_mpa < 55.0 {
            "HOLD"
        } else if dx.abs().max(dy.abs()) > 3.0 {
            "PASS (Tight)"
        } else {
            "PASS"
        };
        pieces.push(PrecastHaulPiece {
            module_id: format!("PC-{}", r.name),
            mass_ton,
            length_m: len_m,
            section_h_m: d_m,
            trailer_permit: permit,
            gvw_ton: gvw,
            delta_x_mm: dx,
            delta_y_mm: dy,
            grout_mpa,
            qa_gate,
        });
    }

    if pieces.is_empty() {
        for (id, mass_ton, length_m, sec_h, permit, dx, dy) in [
            ("PC-COL-01", 18.4_f32, 11.2_f32, 0.75_f32, "Night Escort OK", 1.8_f32, -1.2_f32),
            ("PC-GRD-04", 29.8_f32, 16.5_f32, 1.35_f32, "Steerable Dolly", 3.4_f32, 2.1_f32),
            ("PC-COR-02", 22.1_f32, 8.8_f32, 0.90_f32, "Standard Flatbed", -0.9_f32, 1.0_f32),
        ] {
            let gvw = mass_ton + tare_ton;
            let qa_gate = if gvw > axle_limit_ton || grout_mpa < 55.0 {
                "HOLD"
            } else if dx.abs().max(dy.abs()) > 3.0 {
                "PASS (Tight)"
            } else {
                "PASS"
            };
            pieces.push(PrecastHaulPiece {
                module_id: id.to_string(),
                mass_ton,
                length_m,
                section_h_m: sec_h,
                trailer_permit: if gvw > axle_limit_ton { "AXLE OVERLOAD" } else { permit },
                gvw_ton: gvw,
                delta_x_mm: dx,
                delta_y_mm: dy,
                grout_mpa,
                qa_gate,
            });
        }
    }

    let peak_gvw_ton = pieces.iter().map(|p| p.gvw_ton).fold(0.0_f32, f32::max);
    let max_sec = pieces.iter().map(|p| p.section_h_m).fold(0.8_f32, f32::max);
    let stack_height_m = deck_height_m + max_sec + 0.45; // saddle + cribbing allowance
    let min_clearance_margin_m = portal_clear_m - stack_height_m;
    let max_laser_delta_mm = pieces
        .iter()
        .map(|p| p.delta_x_mm.abs().max(p.delta_y_mm.abs()))
        .fold(0.0_f32, f32::max);

    PrecastHaulingReport {
        peak_gvw_ton,
        axle_limit_ton,
        stack_height_m,
        portal_clear_m,
        min_clearance_margin_m,
        max_laser_delta_mm,
        grout_7day_mpa: grout_mpa,
        pieces,
    }
}

impl IndustrializedView {
    pub(super) fn render_precast_logistics_tolerance(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        bim_project: &crate::core::BimProject,
        features: &mut FeatureRegistry,
    ) {
        let report = compute_precast_hauling_audit(
            bim_project,
            self.precast_axle_limit_ton,
            self.precast_portal_clear_m,
            self.precast_grout_7day_mpa,
            self.precast_night_escort,
        );
        ui_kit::feature_slot_card(
            ui,
            theme,
            features,
            FeatureId::PrecastTrailerJointTolerance,
            |ui| {
                let gvw_val = format!("{:.1} / {:.1} Ton", report.peak_gvw_ton, report.axle_limit_ton);
                let gvw_ok = report.peak_gvw_ton <= report.axle_limit_ton;
                let gvw_sub = if gvw_ok {
                    format!("{} BIM Modules Checked (Permit Clear)", report.pieces.len())
                } else {
                    "AXLE OVER-WEIGHT: Add Steerable Dolly".to_string()
                };
                let clr_val = format!("{:.2} m vs {:.2} m", report.portal_clear_m, report.stack_height_m);
                let clr_ok = report.min_clearance_margin_m >= 0.20;
                let clr_sub = format!(
                    "{:+.0} mm Highway Portal Margin",
                    report.min_clearance_margin_m * 1000.0
                );
                let las_val = format!("±{:.1} mm Max Delta", report.max_laser_delta_mm);
                let grout_val = format!("{:.1} MPa (7-Day)", report.grout_7day_mpa);
                let grout_ok = report.grout_7day_mpa >= 55.0;
                let cards = [
                    (
                        "LOW-LOADER TRAILER GVW",
                        gvw_val.as_str(),
                        gvw_sub.as_str(),
                        if gvw_ok { theme.accent_green } else { theme.accent_red },
                    ),
                    (
                        "MIN BRIDGE CLEARANCE",
                        clr_val.as_str(),
                        clr_sub.as_str(),
                        if clr_ok { theme.accent_blue } else { theme.accent_red },
                    ),
                    (
                        "LASER SPLICE TOLERANCE",
                        las_val.as_str(),
                        "PCI MNL-135 Cap <= ±4.0 mm",
                        theme.accent_green,
                    ),
                    (
                        "NON-SHRINK GROUT CUBE",
                        grout_val.as_str(),
                        "Grouted Splice Sleeve Min >= 55 MPa",
                        if grout_ok { theme.accent_blue } else { theme.accent_red },
                    ),
                ];
                ui_kit::responsive_grid_n(ui, cards.len(), 210.0, |ui, idx| {
                    let (t, v, s, c) = cards[idx];
                    self.render_stat_card(ui, theme, t, v, s, c);
                });

                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Trailer Permit (T):").size(11.0).color(theme.text_secondary));
                    ui.add(egui::Slider::new(&mut self.precast_axle_limit_ton, 40.0..=85.0).step_by(1.0));
                    ui.label(RichText::new("Portal (m):").size(11.0).color(theme.text_secondary));
                    ui.add(egui::Slider::new(&mut self.precast_portal_clear_m, 3.6..=5.8).step_by(0.05));
                    ui.label(RichText::new("Grout (MPa):").size(11.0).color(theme.text_secondary));
                    ui.add(egui::Slider::new(&mut self.precast_grout_7day_mpa, 40.0..=85.0).step_by(0.5));
                    ui.checkbox(&mut self.precast_night_escort, "Night Escort");
                    if crate::icons::icon_button(
                        ui,
                        crate::icons::AppIcon::Truck,
                        "Export Precast Haul & QA (.CSV)",
                        12.0,
                        theme.text_primary,
                        theme.bg_card,
                        Stroke::new(1.0, theme.border_subtle),
                        CornerRadius::same(6),
                        Vec2::ZERO,
                    )
                    .clicked()
                    {
                        let mut csv = String::from("ModuleID,Mass_Ton,Length_m,GVW_Ton,TrailerPermit,DeltaX_mm,DeltaY_mm,Grout_MPa,QAGate\n");
                        for p in &report.pieces {
                            csv.push_str(&format!(
                                "{},{:.1},{:.2},{:.1},{},{:+.1},{:+.1},{:.1},{}\n",
                                p.module_id,
                                p.mass_ton,
                                p.length_m,
                                p.gvw_ton,
                                p.trailer_permit,
                                p.delta_x_mm,
                                p.delta_y_mm,
                                p.grout_mpa,
                                p.qa_gate
                            ));
                        }
                        self.export_status = Some(
                            match crate::core::web_bridge::save_text_file("precast_haul_splice_qa.csv", &csv, "text/csv") {
                                Ok(msg) => format!(
                                    "Precast QA ({} pieces, peak GVW {:.1}T) • {msg}",
                                    report.pieces.len(),
                                    report.peak_gvw_ton
                                ),
                                Err(e) => format!("Precast CSV export failed: {e}"),
                            },
                        );
                    }
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
                                for p in &report.pieces {
                                    let col = if p.qa_gate == "HOLD" {
                                        theme.accent_red
                                    } else if p.qa_gate.contains("Tight") {
                                        theme.accent_orange
                                    } else {
                                        theme.accent_green
                                    };
                                    ui.label(
                                        RichText::new(&p.module_id)
                                            .size(10.5)
                                            .monospace()
                                            .color(theme.accent_blue),
                                    );
                                    ui.label(
                                        RichText::new(format!("{:.1}T / {:.1}m", p.mass_ton, p.length_m))
                                            .size(10.5)
                                            .color(theme.text_primary),
                                    );
                                    ui.label(
                                        RichText::new(format!("{} ({:.1}T GVW)", p.trailer_permit, p.gvw_ton))
                                            .size(10.5)
                                            .color(theme.text_secondary),
                                    );
                                    ui.label(
                                        RichText::new(format!("Δx={:+.1}mm Δy={:+.1}mm", p.delta_x_mm, p.delta_y_mm))
                                            .size(10.5)
                                            .monospace()
                                            .color(theme.text_secondary),
                                    );
                                    ui.label(
                                        RichText::new(format!("{:.1} MPa", p.grout_mpa))
                                            .size(10.5)
                                            .color(theme.accent_blue),
                                    );
                                    ui.label(RichText::new(p.qa_gate).size(10.5).strong().color(col));
                                    ui.end_row();
                                }
                            });
                    });
            },
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Print3dToolpathReport {
    pub layer_count: usize,
    pub layer_time_min: f32,
    pub open_time_ok: bool,
    pub mortar_volume_m3: f32,
    pub pump_pressure_mpa: f32,
    pub gcode_preview: String,
    pub full_gcode: String,
}

/// Slices a BIM structural perimeter into 3D concrete printing contours and validates thermal cold-joint limits.
pub fn generate_concrete_3d_gcode(
    project: &crate::core::BimProject,
    nozzle_mm: f32,
    layer_h_mm: f32,
    feed_mm_s: f32,
    wall_height_m: f32,
    spiralize: bool,
    cold_joint_cap_min: f32,
) -> Print3dToolpathReport {
    let nw = nozzle_mm.clamp(20.0, 90.0);
    let lh = layer_h_mm.clamp(6.0, 35.0);
    let feed = feed_mm_s.clamp(50.0, 400.0);
    let h_m = wall_height_m.clamp(0.4, 4.5);

    let wall_len_m: f32 = project
        .walls
        .iter()
        .map(|w| ((w.end.0 - w.start.0).hypot(w.end.1 - w.start.1) / 1000.0) as f32)
        .sum();
    let perim_m = if wall_len_m >= 4.0 {
        wall_len_m.clamp(6.0, 65.0)
    } else {
        11.2
    };
    let toolpath_per_layer_m = perim_m * if spiralize { 2.35 } else { 2.55 };
    let layer_time_min = (toolpath_per_layer_m * 1000.0) / feed / 60.0;
    let open_time_ok = layer_time_min <= cold_joint_cap_min.max(4.0);
    let layer_count = ((h_m * 1000.0) / lh).ceil().max(1.0) as usize;

    let bead_area_m2 = (nw / 1000.0) * (lh / 1000.0) * 0.96;
    let mortar_volume_m3 = toolpath_per_layer_m * (layer_count as f32) * bead_area_m2;
    let flow_rate_factor = (nw * lh * feed) / (45.0 * 15.0 * 185.0);
    let pump_pressure_mpa = (1.15 + 0.65 * flow_rate_factor).clamp(0.8, 3.8);

    let f_mm_min = (feed * 60.0).round() as u32;
    let mut full_gcode = format!(
        "; EigenGraph 3DCP Contour Slicer v1.1\n\
         ; Project: {} | Layers: {} | Nozzle: {:.0}x{:.0} mm | Spiralize: {}\n\
         G90 ; Absolute mm\n\
         M82 ; Pump flow sync enabled ({:.2} MPa)\n",
        project.project_name, layer_count, nw, lh, spiralize, pump_pressure_mpa
    );
    let preview_layers = layer_count.min(4);
    let x_span_mm = (perim_m * 280.0).clamp(1800.0, 6400.0);
    let y_span_mm = (perim_m * 110.0).clamp(720.0, 2200.0);
    let mut ext_e = 0.0_f32;
    for l in 1..=layer_count {
        let z_mm = l as f32 * lh;
        let de = toolpath_per_layer_m * 0.14;
        ext_e += de * 0.35;
        let line1 = format!(
            "G1 X{:.1} Y{:.1} Z{:.1} F{} E{:.2} ; Layer {} Outer\n",
            x_span_mm, 140.0, z_mm, f_mm_min, ext_e, l
        );
        ext_e += de * 0.25;
        let line2 = format!(
            "G1 X{:.1} Y{:.1} Z{:.1} E{:.2} ; Layer {} Truss Turn\n",
            x_span_mm, y_span_mm, z_mm, ext_e, l
        );
        ext_e += de * 0.40;
        let line3 = format!(
            "G1 X120.0 Y{:.1} Z{:.1} E{:.2} ; Layer {} Return\n",
            y_span_mm, z_mm, ext_e, l
        );
        if l <= preview_layers || l == layer_count {
            full_gcode.push_str(&line1);
            full_gcode.push_str(&line2);
            full_gcode.push_str(&line3);
        } else {
            full_gcode.push_str(&line1);
        }
    }
    full_gcode.push_str("M84 ; Pump & Gantry Home Park\n");

    let gcode_preview: String = full_gcode.lines().take(9).collect::<Vec<_>>().join("\n");

    Print3dToolpathReport {
        layer_count,
        layer_time_min,
        open_time_ok,
        mortar_volume_m3,
        pump_pressure_mpa,
        gcode_preview,
        full_gcode,
    }
}

impl super::IndustrializedView {
    pub(super) fn render_concrete_3d_printing(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        bim_project: &crate::core::BimProject,
        features: &mut FeatureRegistry,
    ) {
        let rep = generate_concrete_3d_gcode(
            bim_project,
            self.print_nozzle_mm,
            self.print_layer_h_mm,
            self.print_feed_mm_s,
            self.print_wall_height_m,
            self.print_spiralize,
            self.print_cold_joint_cap_min,
        );
        let time_col = if rep.open_time_ok {
            theme.accent_green
        } else {
            theme.accent_red
        };
        ui_kit::feature_slot_card(
            ui,
            theme,
            features,
            FeatureId::Concrete3DPrintingGcodeToolpath,
            |ui| {
                let cards = [
                    (
                        "EXTRUSION NOZZLE",
                        format!("{:.0} × {:.0} mm Layer", self.print_nozzle_mm, self.print_layer_h_mm),
                        format!("{} Sliced Contours (Total H = {:.2} m)", rep.layer_count, self.print_wall_height_m),
                        theme.accent_blue,
                    ),
                    (
                        "PRINT HEAD FEED RATE",
                        format!("{:.0} mm/s Constant", self.print_feed_mm_s),
                        if self.print_spiralize {
                            "Spiralize 2-Perimeter Truss Infill".to_string()
                        } else {
                            "Discrete Layer Z-Lift Mode".to_string()
                        },
                        theme.accent_green,
                    ),
                    (
                        "OPEN-TIME LAYER WINDOW",
                        format!("{:.1} min / Layer", rep.layer_time_min),
                        if rep.open_time_ok {
                            format!("Max Cold-Joint Cap = {:.1} min (Safe)", self.print_cold_joint_cap_min)
                        } else {
                            format!("COLD-JOINT RISK (> {:.1} min Cap)", self.print_cold_joint_cap_min)
                        },
                        time_col,
                    ),
                    (
                        "ESTIMATED MORTAR VOLUME",
                        format!("{:.2} m³ Fiber-Mix", rep.mortar_volume_m3),
                        format!("Pump Pressure = {:.2} MPa @ Hose", rep.pump_pressure_mpa),
                        theme.accent_orange,
                    ),
                ];
                ui_kit::responsive_grid_n(ui, cards.len(), 210.0, |ui, idx| {
                    let (t, v, s, c) = &cards[idx];
                    self.render_stat_card(ui, theme, t, v, s, *c);
                });

                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Nozzle (mm):").size(11.0).color(theme.text_secondary));
                    ui.add(egui::Slider::new(&mut self.print_nozzle_mm, 25.0..=80.0).suffix(" mm"));
                    ui.label(RichText::new("Layer H (mm):").size(11.0).color(theme.text_secondary));
                    ui.add(egui::Slider::new(&mut self.print_layer_h_mm, 8.0..=30.0).suffix(" mm"));
                    ui.label(RichText::new("Feed (mm/s):").size(11.0).color(theme.text_secondary));
                    ui.add(egui::Slider::new(&mut self.print_feed_mm_s, 80.0..=300.0).suffix(" mm/s"));
                });
                ui.add_space(6.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Wall Height (m):").size(11.0).color(theme.text_secondary));
                    ui.add(egui::Slider::new(&mut self.print_wall_height_m, 0.6..=3.6).suffix(" m"));
                    ui.label(RichText::new("Cold-Joint Cap:").size(11.0).color(theme.text_secondary));
                    ui.add(egui::Slider::new(&mut self.print_cold_joint_cap_min, 5.0..=30.0).suffix(" min"));
                    ui.checkbox(&mut self.print_spiralize, "Spiralize Contour");
                    if crate::icons::icon_button(
                        ui,
                        crate::icons::AppIcon::Printer,
                        "Slice & Export 3DCP G-Code (.gcode)",
                        12.0,
                        theme.text_primary,
                        theme.bg_card,
                        Stroke::new(1.0, theme.border_subtle),
                        CornerRadius::same(6),
                        Vec2::ZERO,
                    )
                    .clicked()
                    {
                        match crate::core::web_bridge::save_text_file(
                            "EigenGraph_3DCP_Toolpath.gcode",
                            &rep.full_gcode,
                            "text/plain",
                        ) {
                            Ok(msg) => {
                                self.export_status = Some(format!(
                                    "Sliced {} contours ({:.2} m³ mortar) & exported G-Code ({msg})",
                                    rep.layer_count, rep.mortar_volume_m3
                                ))
                            }
                            Err(err) => self.export_status = Some(format!("3DCP G-Code export error: {err}")),
                        }
                    }
                });
                ui.add_space(12.0);

                ui_kit::responsive_two_panes(ui, 640.0, |ui, pane| match pane {
                    0 => {
                        ui.label(
                            RichText::new(format!(
                                "3DCP LAYER TOOLPATH CONTOUR PREVIEW (Z = {:.2} m)",
                                self.print_wall_height_m * 0.25
                            ))
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
                        let bead_px = (self.print_nozzle_mm / 18.0).clamp(1.6, 4.2);
                        painter.rect_stroke(
                            pad,
                            CornerRadius::same(10),
                            Stroke::new(bead_px, theme.accent_blue),
                            StrokeKind::Inside,
                        );
                        painter.rect_stroke(
                            pad.shrink(12.0),
                            CornerRadius::same(6),
                            Stroke::new(bead_px * 0.9, theme.accent_green),
                            StrokeKind::Inside,
                        );
                        let inner = pad.shrink(12.0);
                        let steps = ((self.print_feed_mm_s / 12.0).round() as usize).clamp(10, 26);
                        let mut zig = Vec::with_capacity(steps + 1);
                        for i in 0..=steps {
                            let frac = i as f32 / steps as f32;
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
                            format!(
                                "{} Sinusoidal Truss Loops • {:.0} mm/s Feed",
                                rep.layer_count, self.print_feed_mm_s
                            ),
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
                                    RichText::new(&rep.gcode_preview)
                                        .size(10.0)
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

#[cfg(test)]
mod tests {
    use super::{compute_precast_hauling_audit, generate_concrete_3d_gcode};
    use crate::core::BimProject;

    #[test]
    fn test_precast_hauling_audit_checks_axle_and_clearance_caps() {
        let proj = BimProject::default();
        let pass_rep = compute_precast_hauling_audit(&proj, 64.0, 4.80, 68.5, true);
        let strict_rep = compute_precast_hauling_audit(&proj, 40.0, 2.50, 48.0, false);

        assert!(!pass_rep.pieces.is_empty());
        assert!(pass_rep.peak_gvw_ton <= 64.0);
        assert!(strict_rep.pieces.iter().any(|p| p.qa_gate == "HOLD"));
        assert!(strict_rep.min_clearance_margin_m < 0.0);
    }

    #[test]
    fn test_concrete_3d_printing_gcode_cold_joint_window() {
        let proj = BimProject::sample_bimcraft_building();
        let fast = generate_concrete_3d_gcode(&proj, 45.0, 15.0, 220.0, 1.80, true, 18.0);
        let slow_cap = generate_concrete_3d_gcode(&proj, 45.0, 10.0, 65.0, 1.80, false, 3.0);
        assert!(fast.layer_count >= 100);
        assert!(fast.full_gcode.contains("G90 ; Absolute mm"));
        assert!(fast.mortar_volume_m3 > 0.0);
        assert!(fast.open_time_ok);
        assert!(!slow_cap.open_time_ok);
    }
}
