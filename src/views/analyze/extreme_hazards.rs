//! views/analyze/extreme_hazards.rs — Extreme Hazard Assessments (FEMA P-58, UFC 3-340 Blast & AISC/EC3 Structural Fire).

use super::*;

impl AnalyzeView {
    pub(super) fn render_fema_p58_assessment(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let input = crate::core::FemaP58Input {
            project_name: self.fema_project_name.clone(),
            building_replacement_value_usd: self.fema_asset_value_usd,
            total_floor_area_m2: self.fema_floor_area_m2,
            num_stories: self.fema_num_stories,
            story_height_m: self.fema_story_height_m,
            site_pga_g: self.fema_site_pga_g,
            spectral_accel_sa1_g: self.fema_spectral_sa1_g,
            max_laborers_per_story: self.fema_workers_per_floor,
            daily_business_revenue_usd: self.fema_daily_revenue_usd,
            target_redi_tier: self.fema_target_redi_tier,
        };

        let rep = crate::core::FemaP58Engine::evaluate(&input);

        // Header & Quick Export Banner
        theme.card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    icons::icon_label(ui, AppIcon::Building, 14.0, theme.accent_blue, "FEMA P-58 Seismic Performance & REDi™ Resilience Engine", 13.0, theme.accent_blue, true);
                    ui.label(RichText::new("FEMA P-58-1 / ATC-58 · Arup REDi™ Rating System · ASCE 7-22 / ASCE 41-17 · ISO 14040 Embodied Carbon").size(10.5).color(theme.text_secondary));
                });

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if icons::icon_button(
                        ui,
                        AppIcon::SaveDisk,
                        "Export FEMA P-58 Dossier (.md)",
                        11.0,
                        Color32::WHITE,
                        theme.bg_canvas,
                        Stroke::new(1.0, theme.accent_blue),
                        CornerRadius::same(7),
                        Vec2::ZERO,
                    ).clicked() {
                        let dossier = crate::core::FemaP58Engine::generate_markdown_dossier(&input, &rep);
                        if let Ok(path) = crate::core::DiskExporter::write_export_file("EigenGraph_FEMA_P58_Seismic_Loss_Dossier.md", &dossier) {
                            self.export_status = Some(format!("FEMA P-58 Calculation Dossier saved to: {}", path));
                        }
                    }
                });
            });
        });

        ui.add_space(10.0);

        // 4 KPI Summary Cards
        ui_kit::responsive_grid_n(ui, 4, 210.0, |ui, idx| match idx {
    0 => {
        // Card 1: Direct Repair Loss
            let cost_col = if rep.repair_cost_ratio <= 0.10 { theme.accent_green } else { theme.accent_orange };
            theme.card_frame().show(ui, |ui| {
                    ui.set_width(ui.available_width());
ui.label(RichText::new("DIRECT REPAIR LOSS").size(10.0).strong().color(theme.text_secondary));
                ui.label(RichText::new(format!("${:.0}", rep.total_repair_cost_usd)).size(16.5).strong().color(cost_col));
                ui.label(RichText::new(format!("{:.1}% Asset | EAL: {:.2}%/yr", rep.repair_cost_ratio * 100.0, rep.expected_annual_loss_ratio * 100.0)).size(10.0).color(theme.text_muted));
            });
    }
    1 => {
        // Card 2: Facility Downtime & BI
            let dt_col = if rep.downtime_days <= 30.0 { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
            theme.card_frame().show(ui, |ui| {
                    ui.set_width(ui.available_width());
ui.label(RichText::new("RECOVERY DOWNTIME").size(10.0).strong().color(theme.text_secondary));
                ui.label(RichText::new(format!("{:.0} Days", rep.downtime_days)).size(16.5).strong().color(dt_col));
                ui.label(RichText::new(format!("BI Loss: ${:.0}", rep.business_interruption_loss_usd)).size(10.0).color(theme.text_muted));
            });
    }
    2 => {
        // Card 3: Embodied Carbon Footprint
            theme.card_frame().show(ui, |ui| {
                    ui.set_width(ui.available_width());
ui.label(RichText::new("EMBODIED CARBON").size(10.0).strong().color(theme.text_secondary));
                ui.label(RichText::new(format!("{:.1} kg/m²", rep.embodied_carbon_per_m2)).size(16.5).strong().color(Color32::from_rgb(20, 184, 166)));
                ui.label(RichText::new(format!("Total: {:.0} kg CO2e", rep.total_embodied_carbon_kg_co2e)).size(10.0).color(theme.text_muted));
            });
    }
    _ => {
        // Card 4: REDi Resilience Tier
            let redi_col = match rep.achieved_redi_tier {
                crate::core::REDiResilienceTier::Platinum => Color32::from_rgb(220, 220, 240),
                crate::core::REDiResilienceTier::Gold => Color32::from_rgb(245, 158, 11),
                crate::core::REDiResilienceTier::Silver => Color32::from_rgb(156, 163, 175),
                crate::core::REDiResilienceTier::StandardCode => Color32::from_rgb(239, 68, 68),
            };
            theme.card_frame().show(ui, |ui| {
                    ui.set_width(ui.available_width());
ui.label(RichText::new("REDi™ RATING").size(10.0).strong().color(theme.text_secondary));
                ui.label(RichText::new(format!("{:?}", rep.achieved_redi_tier)).size(16.5).strong().color(redi_col));
                let (r_ic, r_txt, r_col) = if rep.is_target_redi_achieved {
                    (AppIcon::CheckCircle, "Target Tier Achieved", theme.accent_green)
                } else {
                    (AppIcon::AlertTriangle, "Below Target Tier", theme.accent_orange)
                };
                icons::icon_label(ui, r_ic, 11.0, r_col, r_txt, 10.0, theme.text_muted, false);
            });
    }
});

        ui.add_space(10.0);

        // Parameters Control Card (3 columns)
        theme.card_frame().show(ui, |ui| {
            ui_kit::responsive_grid_n(ui, 3, 240.0, |ui, col_idx| match col_idx {
                0 => {
                    // Column 1: Asset Valuation & Geometry
                            ui.vertical(|ui| {
                                ui.label(RichText::new("1. Asset Valuation & Floor Area").size(11.0).strong().color(theme.text_primary));
                                ui.horizontal(|ui| {
                                    ui.label("Asset Value ($):");
                                    ui.add(egui::Slider::new(&mut self.fema_asset_value_usd, 10_000_000.0..=250_000_000.0).prefix("$"));
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Floor Area (m²):");
                                    ui.add(egui::Slider::new(&mut self.fema_floor_area_m2, 5_000.0..=60_000.0).suffix(" m²"));
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Stories (N):");
                                    ui.add(egui::Slider::new(&mut self.fema_num_stories, 3..=25));
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Story Height:");
                                    ui.add(egui::Slider::new(&mut self.fema_story_height_m, 3.0..=5.0).suffix(" m"));
                                });
                            });
                }
                1 => {
                    // Column 2: Seismic Hazard & Business Revenue
                            ui.vertical(|ui| {
                                ui.label(RichText::new("2. Seismic Demand & Business Exposure").size(11.0).strong().color(theme.text_primary));
                                ui.horizontal(|ui| {
                                    ui.label("Site PGA (g):");
                                    ui.add(egui::Slider::new(&mut self.fema_site_pga_g, 0.15..=1.00).suffix(" g"));
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Sa(T1) (g):");
                                    ui.add(egui::Slider::new(&mut self.fema_spectral_sa1_g, 0.20..=1.80).suffix(" g"));
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Daily Revenue ($):");
                                    ui.add(egui::Slider::new(&mut self.fema_daily_revenue_usd, 20_000.0..=300_000.0).prefix("$"));
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Demolition Risk:");
                                    ui.label(RichText::new(format!("{:.1}% (Residual Drift)", rep.demolition_probability * 100.0)).color(theme.accent_orange).strong());
                                });
                            });
                }
                _ => {
                    // Column 3: REDi Target & Labor Allocation
                            ui.vertical(|ui| {
                                ui.label(RichText::new("3. REDi™ Resilience & Labor Constraints").size(11.0).strong().color(theme.text_primary));
                                ui.horizontal(|ui| {
                                    ui.label("Target Tier:");
                                    egui::ComboBox::from_id_salt("fema_redi_cb")
                                        .selected_text(format!("{:?}", self.fema_target_redi_tier))
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(&mut self.fema_target_redi_tier, crate::core::REDiResilienceTier::Platinum, "Platinum (≤ 7 Days)");
                                            ui.selectable_value(&mut self.fema_target_redi_tier, crate::core::REDiResilienceTier::Gold, "Gold (≤ 30 Days)");
                                            ui.selectable_value(&mut self.fema_target_redi_tier, crate::core::REDiResilienceTier::Silver, "Silver (≤ 60 Days)");
                                            ui.selectable_value(&mut self.fema_target_redi_tier, crate::core::REDiResilienceTier::StandardCode, "Baseline Code (> 180 Days)");
                                        });
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Workers / Story:");
                                    ui.add(egui::Slider::new(&mut self.fema_workers_per_floor, 4..=24));
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Total Labor Days:");
                                    ui.label(RichText::new(format!("{:.0} person-days", rep.total_repair_labor_days)).color(theme.accent_blue).strong());
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Collapse Risk:");
                                    ui.label(RichText::new(format!("{:.2}% (MCE)", rep.collapse_probability * 100.0)).color(theme.accent_green).strong());
                                });
                            });
                }
            });
        });

        ui.add_space(10.0);

        // 2D Vector CAD Canvas: Fragility Curves Plot + Story-by-Story Loss Distribution
        theme.card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                icons::icon_label(ui, AppIcon::RulerSquare, 13.5, theme.accent_blue, "2D Vector CAD Component Fragility Curves & Story Loss Breakdown", 12.5, theme.accent_blue, true);
                ui.add_space(20.0);
                ui.label(RichText::new(format!("Direct Loss: ${:.0} | Total Exposure: ${:.0}", rep.total_repair_cost_usd, rep.total_economic_loss_usd)).size(11.0).color(theme.accent_green));
            });

            ui.add_space(6.0);

            let (canvas_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 270.0), Sense::hover());
            let painter = ui.painter_at(canvas_rect);

            painter.rect_filled(canvas_rect, 6.0, Color32::from_rgb(18, 22, 30));
            painter.rect_stroke(canvas_rect, 6.0, Stroke::new(1.0, Color32::from_rgb(45, 55, 75)), egui::StrokeKind::Inside);

            let mid_x = canvas_rect.min.x + canvas_rect.width() * 0.52;

            // Divider line between Fragility Curves and Story Losses
            painter.line_segment(
                [Pos2::new(mid_x, canvas_rect.min.y + 10.0), Pos2::new(mid_x, canvas_rect.max.y - 10.0)],
                Stroke::new(1.0, Color32::from_rgb(45, 55, 75)),
            );

            // Left Side: Lognormal Component Fragility Curves
            let frag_left = canvas_rect.min.x + 35.0;
            let frag_right = mid_x - 20.0;
            let frag_w = (frag_right - frag_left).max(10.0);
            let frag_bot = canvas_rect.max.y - 30.0;
            let frag_top = canvas_rect.min.y + 35.0;
            let frag_h = (frag_bot - frag_top).max(10.0);

            // Axes
            painter.line_segment([Pos2::new(frag_left, frag_bot), Pos2::new(frag_right, frag_bot)], Stroke::new(1.2, Color32::from_rgb(70, 80, 100)));
            painter.line_segment([Pos2::new(frag_left, frag_bot), Pos2::new(frag_left, frag_top)], Stroke::new(1.2, Color32::from_rgb(70, 80, 100)));

            // Horizontal Probability Grid Lines (0.25, 0.50, 0.75, 1.0)
            for p_step in 1..=4 {
                let py = frag_bot - (p_step as f32 / 4.0) * frag_h;
                painter.line_segment([Pos2::new(frag_left, py), Pos2::new(frag_right, py)], Stroke::new(0.5, Color32::from_rgb(35, 45, 60)));
                painter.text(Pos2::new(frag_left - 4.0, py), Align2::RIGHT_CENTER, format!("{:.0}%", p_step * 25), egui::FontId::proportional(8.5), Color32::from_rgb(130, 140, 160));
            }

            // Draw component fragility curves
            let colors = [
                Color32::from_rgb(56, 189, 248),  // RC Frame (cyan)
                Color32::from_rgb(245, 158, 11),  // Partitions (amber)
                Color32::from_rgb(168, 85, 247),  // Curtain Wall (purple)
                Color32::from_rgb(16, 185, 129),  // Ceilings (emerald)
                Color32::from_rgb(239, 68, 68),   // MEP Piping (red)
            ];

            for (c_idx, (_, pts)) in rep.fragility_sample_curves.iter().enumerate() {
                let col = colors[c_idx % colors.len()];
                if pts.len() >= 2 {
                    let mut screen_pts = Vec::with_capacity(pts.len());
                    for (i, &(_, p)) in pts.iter().enumerate() {
                        let px = frag_left + (i as f32 / (pts.len() - 1) as f32) * frag_w;
                        let py = frag_bot - (p as f32) * frag_h;
                        screen_pts.push(Pos2::new(px, py));
                    }
                    for w in screen_pts.windows(2) {
                        painter.line_segment([w[0], w[1]], Stroke::new(1.8, col));
                    }
                }
            }

            painter.text(
                Pos2::new(frag_left, canvas_rect.min.y + 12.0),
                Align2::LEFT_TOP,
                "FEMA P-58 Component Fragilities: P(DS ≥ ds | EDP)",
                egui::FontId::proportional(11.0),
                Color32::from_rgb(56, 189, 248),
            );

            // Right Side: Story-by-Story Loss Distribution
            let story_left = mid_x + 35.0;
            let story_right = canvas_rect.max.x - 25.0;
            let story_w = (story_right - story_left).max(10.0);
            let story_bot = canvas_rect.max.y - 30.0;
            let story_top = canvas_rect.min.y + 35.0;
            let story_h = (story_bot - story_top).max(10.0);

            // Vertical axis (Stories)
            painter.line_segment([Pos2::new(story_left, story_bot), Pos2::new(story_left, story_top)], Stroke::new(1.2, Color32::from_rgb(70, 80, 100)));
            painter.line_segment([Pos2::new(story_left, story_bot), Pos2::new(story_right, story_bot)], Stroke::new(1.2, Color32::from_rgb(70, 80, 100)));

            let n_stories = rep.story_losses.len();
            if n_stories > 0 {
                let bar_gap = story_h / (n_stories as f32);
                let bar_h = (bar_gap * 0.70).clamp(4.0, 18.0);
                let max_story_cost = rep.story_losses.iter().map(|s| s.repair_cost_usd).fold(1.0f64, |a, b| a.max(b));

                for (idx, s) in rep.story_losses.iter().enumerate() {
                    let py = story_bot - (idx as f32 + 0.5) * bar_gap;
                    let bar_len = ((s.repair_cost_usd / max_story_cost) as f32 * story_w).clamp(4.0, story_w);

                    // Horizontal story loss bar
                    let bar_rect = egui::Rect::from_min_max(
                        Pos2::new(story_left, py - bar_h * 0.5),
                        Pos2::new(story_left + bar_len, py + bar_h * 0.5),
                    );
                    painter.rect_filled(bar_rect, CornerRadius::same(3), Color32::from_rgba_unmultiplied(245, 158, 11, 200));

                    // Story label
                    painter.text(
                        Pos2::new(story_left - 4.0, py),
                        Align2::RIGHT_CENTER,
                        format!("L{:02}", s.story_number),
                        egui::FontId::proportional(8.5),
                        Color32::from_rgb(160, 170, 190),
                    );

                    // Value label on bar
                    painter.text(
                        Pos2::new(story_left + bar_len + 4.0, py),
                        Align2::LEFT_CENTER,
                        format!("${:.0} ({:.1}d)", s.repair_cost_usd, s.repair_labor_person_days),
                        egui::FontId::proportional(8.5),
                        Color32::from_rgb(220, 225, 235),
                    );
                }
            }

            painter.text(
                Pos2::new(story_left, canvas_rect.min.y + 12.0),
                Align2::LEFT_TOP,
                "Story Repair Cost Breakdown & Labor Days",
                egui::FontId::proportional(11.0),
                Color32::from_rgb(245, 158, 11),
            );
        });

        ui.add_space(10.0);

        // Regulatory & REDi Compliance Audit Table
        theme.card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                icons::icon_label(ui, AppIcon::Clipboard, 13.5, theme.accent_blue, "FEMA P-58 / REDi™ Performance Compliance Audit", 12.5, theme.accent_blue, true);
                ui.add_space(20.0);
                let (ov_ic, overall_status, status_col) = if rep.is_target_redi_achieved {
                    (AppIcon::CheckCircle, "TARGET REDi™ RESILIENCE TIER ACHIEVED", theme.accent_green)
                } else {
                    (AppIcon::AlertTriangle, "TARGET RESILIENCE TIER DEFICIT", Color32::from_rgb(239, 68, 68))
                };
                icons::pill_badge(ui, ov_ic, overall_status, status_col);
            });

            ui.add_space(6.0);

            // Table Header
            ui.horizontal(|ui| {
                ui.label(RichText::new("Performance Metric").size(10.5).strong().color(theme.text_secondary));
                ui.add_space(220.0);
                ui.label(RichText::new("Code / Rating Reference").size(10.5).strong().color(theme.text_secondary));
                ui.add_space(150.0);
                ui.label(RichText::new("Demand").size(10.5).strong().color(theme.text_secondary));
                ui.add_space(45.0);
                ui.label(RichText::new("Limit").size(10.5).strong().color(theme.text_secondary));
                ui.add_space(45.0);
                ui.label(RichText::new("D/C Ratio").size(10.5).strong().color(theme.text_secondary));
                ui.add_space(30.0);
                ui.label(RichText::new("Status").size(10.5).strong().color(theme.text_secondary));
            });
            ui.separator();

            for item in &rep.compliance_audit {
                let row_col = if item.is_passed { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&item.metric_name).size(10.5).color(theme.accent_blue));
                    ui.add_space(10.0);
                    ui.label(RichText::new(&item.code_reference).size(10.5).color(theme.text_primary));
                    ui.add_space(10.0);
                    ui.label(RichText::new(format!("{:.1} {}", item.demand_val, item.unit)).size(10.5).color(theme.text_primary));
                    ui.add_space(15.0);
                    ui.label(RichText::new(format!("{:.1} {}", item.permissible_limit, item.unit)).size(10.5).color(theme.accent_green));
                    ui.add_space(15.0);
                    ui.label(RichText::new(format!("{:.3}", item.utilization_ratio)).size(10.5).strong().color(row_col));
                    ui.add_space(20.0);
                    let (r_ic, r_txt) = if item.is_passed { (AppIcon::CheckCircle, "PASS") } else { (AppIcon::AlertTriangle, "FAIL") };
                    icons::icon_label(ui, r_ic, 11.0, row_col, r_txt, 10.5, row_col, true);
                });
                ui.separator();
            }
        });
    }

    pub(super) fn render_blast_resistant_design(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let input = crate::core::BlastAnalysisInput {
            project_name: self.blast_project_name.clone(),
            explosive_mass_kg: self.blast_charge_kg as f64,
            explosive_type: self.blast_explosive_type,
            burst_type: self.blast_burst_type,
            standoff_distance_m: self.blast_standoff_m as f64,
            component_kind: self.blast_component_kind,
            support_condition: self.blast_support_condition,
            span_length_m: self.blast_span_length_m as f64,
            tributary_width_m: self.blast_tributary_width_m as f64,
            thickness_m: self.blast_thickness_m as f64,
            f_c_mpa: self.blast_fc_mpa as f64,
            f_y_mpa: self.blast_fy_mpa as f64,
            rebar_ratio_percent: self.blast_rebar_percent as f64,
            damping_ratio: 0.02,
            target_damage_level: self.blast_target_damage,
        };

        let rep = crate::core::BlastEngine::run_assessment(&input);

        // 4 Executive KPI Cards
        ui_kit::responsive_grid_n(ui, 4, 210.0, |ui, idx| match idx {
    0 => {
        // Card 1: Threat Intensity
            theme.card_frame().show(ui, |ui| {
                    ui.set_width(ui.available_width());
                icons::icon_label(ui, AppIcon::Flame, 12.0, Color32::from_rgb(239, 68, 68), "Blast Threat Intensity", 11.0, theme.text_secondary, true);
                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!("{:.1} kg TNT", rep.blast_wave.effective_tnt_mass_kg))
                        .size(19.0)
                        .strong()
                        .color(Color32::from_rgb(239, 68, 68)),
                );
                ui.label(
                    RichText::new(format!(
                        "Standoff R = {:.1} m | Z = {:.2} m/kg⅓\nPso = {:.1} kPa ({:.2} bar)",
                        rep.blast_wave.standoff_distance_m,
                        rep.blast_wave.scaled_distance_z,
                        rep.blast_wave.peak_incident_overpressure_kpa,
                        rep.blast_wave.peak_incident_overpressure_kpa / 100.0
                    ))
                    .size(10.5)
                    .color(theme.text_secondary),
                );
            });
    }
    1 => {
        // Card 2: Reflected Impulse & Duration
            theme.card_frame().show(ui, |ui| {
                    ui.set_width(ui.available_width());
                icons::icon_label(ui, AppIcon::Bolt, 12.0, Color32::from_rgb(245, 158, 11), "Reflected Shock & Impulse", 11.0, theme.text_secondary, true);
                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!("{:.1} kPa", rep.blast_wave.peak_reflected_overpressure_kpa))
                        .size(19.0)
                        .strong()
                        .color(Color32::from_rgb(245, 158, 11)),
                );
                ui.label(
                    RichText::new(format!(
                        "Reflected ir = {:.1} kPa·ms\nDuration td = {:.2} ms | Us = {:.0} m/s",
                        rep.blast_wave.reflected_impulse_kpa_ms,
                        rep.blast_wave.positive_phase_duration_ms,
                        rep.blast_wave.shock_velocity_m_s
                    ))
                    .size(10.5)
                    .color(theme.text_secondary),
                );
            });
    }
    2 => {
        // Card 3: Dynamic Structural Deformation
            theme.card_frame().show(ui, |ui| {
                    ui.set_width(ui.available_width());
                icons::icon_label(ui, AppIcon::RulerSquare, 12.0, theme.accent_blue, "Dynamic SDOF Demands", 11.0, theme.text_secondary, true);
                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!("μ = {:.2}", rep.dynamic_ductility_ratio))
                        .size(19.0)
                        .strong()
                        .color(theme.accent_blue),
                );
                ui.label(
                    RichText::new(format!(
                        "Deflection ymax = {:.1} mm (ye = {:.1} mm)\nSupport Rotation θ = {:.2}°",
                        rep.max_displacement_mm,
                        rep.sdof_system.yield_displacement_mm,
                        rep.support_rotation_deg
                    ))
                    .size(10.5)
                    .color(theme.text_secondary),
                );
            });
    }
    _ => {
        // Card 4: UFC Damage Level & Compliance
            let (r, g, b) = rep.evaluated_damage_level.color_rgb();
            let status_badge_col = Color32::from_rgb(r, g, b);
            theme.card_frame().show(ui, |ui| {
                    ui.set_width(ui.available_width());
                icons::icon_label(ui, AppIcon::Shield, 12.0, status_badge_col, "UFC 3-340-02 Damage Tier", 11.0, theme.text_secondary, true);
                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!("{:?}", rep.evaluated_damage_level))
                        .size(19.0)
                        .strong()
                        .color(status_badge_col),
                );
                let (ach_ic, achieved_str, ach_col) = if rep.is_target_damage_achieved {
                    (AppIcon::CheckCircle, "Target Limit Satisfied", theme.accent_green)
                } else {
                    (AppIcon::AlertTriangle, "Target Limit Deficit", Color32::from_rgb(239, 68, 68))
                };
                icons::icon_label(ui, ach_ic, 11.0, ach_col, achieved_str, 10.5, ach_col, true);
                ui.label(
                    RichText::new(format!("Regime: {}", rep.sdof_system.response_regime.label()))
                        .size(10.5)
                        .color(theme.text_secondary),
                );
            });
    }
});

        ui.add_space(10.0);

        // Parameter Controls Panel (3 Columns)
        theme.card_frame().show(ui, |ui| {
            icons::icon_label(ui, AppIcon::SettingsGear, 13.5, theme.accent_blue, "Blast Threat, Structural Geometry & Antiterrorism Design Parameters", 12.5, theme.accent_blue, true);
            ui.add_space(8.0);

            ui_kit::responsive_grid_n(ui, 3, 240.0, |ui, col_idx| match col_idx {
                0 => {
                    // Column 1: Threat Characterization
                            ui.vertical(|ui| {
                                ui.label(RichText::new("1. Threat & Standoff Definition").strong().size(11.0).color(theme.accent_blue));
                                ui.add_space(4.0);

                                ui.label(RichText::new("Explosive Charge Mass (kg):").size(10.5));
                                ui.add(egui::Slider::new(&mut self.blast_charge_kg, 10.0..=1000.0).suffix(" kg"));

                                ui.add_space(4.0);
                                ui.label(RichText::new("Explosive Type:").size(10.5));
                                egui::ComboBox::from_id_salt("blast_exp_combo")
                                    .selected_text(self.blast_explosive_type.label())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.blast_explosive_type, crate::core::ExplosiveType::Tnt, "TNT (Trinitrotoluene)");
                                        ui.selectable_value(&mut self.blast_explosive_type, crate::core::ExplosiveType::Anfo, "ANFO (Ammonium Nitrate/Fuel Oil)");
                                        ui.selectable_value(&mut self.blast_explosive_type, crate::core::ExplosiveType::C4, "Composition C-4 (Military RDX)");
                                        ui.selectable_value(&mut self.blast_explosive_type, crate::core::ExplosiveType::Petn, "PETN (High Brisance)");
                                        ui.selectable_value(&mut self.blast_explosive_type, crate::core::ExplosiveType::Dynamite, "Commercial Dynamite");
                                    });

                                ui.add_space(4.0);
                                ui.label(RichText::new("Detonation Burst Type:").size(10.5));
                                egui::ComboBox::from_id_salt("blast_burst_combo")
                                    .selected_text(self.blast_burst_type.label())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.blast_burst_type, crate::core::BurstType::SurfaceBurst, "Surface Burst (Ground Reflection 1.8x)");
                                        ui.selectable_value(&mut self.blast_burst_type, crate::core::BurstType::FreeAirBurst, "Free-Air Burst (Spherical 1.0x)");
                                    });

                                ui.add_space(4.0);
                                ui.label(RichText::new("Standoff Distance R (m):").size(10.5));
                                ui.add(egui::Slider::new(&mut self.blast_standoff_m, 3.0..=60.0).suffix(" m"));
                            });
                }
                1 => {
                    // Column 2: Structural Component Geometry
                            ui.vertical(|ui| {
                                ui.label(RichText::new("2. Component Archetype & Dimensions").strong().size(11.0).color(theme.accent_blue));
                                ui.add_space(4.0);

                                ui.label(RichText::new("Component Archetype:").size(10.5));
                                egui::ComboBox::from_id_salt("blast_comp_combo")
                                    .selected_text(self.blast_component_kind.label())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.blast_component_kind, crate::core::BlastComponentKind::RcWall, "RC Hardened Blast Wall");
                                        ui.selectable_value(&mut self.blast_component_kind, crate::core::BlastComponentKind::RcSlab, "RC Roof / Floor Slab Diaphragm");
                                        ui.selectable_value(&mut self.blast_component_kind, crate::core::BlastComponentKind::SteelColumn, "Structural Steel W-Column");
                                        ui.selectable_value(&mut self.blast_component_kind, crate::core::BlastComponentKind::SteelBarrier, "Steel Plate Blast Barrier Gate");
                                        ui.selectable_value(&mut self.blast_component_kind, crate::core::BlastComponentKind::BlastGlazing, "Laminated Anti-Shatter Glazing");
                                    });

                                ui.add_space(4.0);
                                ui.label(RichText::new("Support Boundary Condition:").size(10.5));
                                egui::ComboBox::from_id_salt("blast_supp_combo")
                                    .selected_text(self.blast_support_condition.label())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.blast_support_condition, crate::core::SupportCondition::FixedFixed, "Fixed - Fixed (Clamped)");
                                        ui.selectable_value(&mut self.blast_support_condition, crate::core::SupportCondition::SimplySupported, "Simply Supported (Pinned - Pinned)");
                                        ui.selectable_value(&mut self.blast_support_condition, crate::core::SupportCondition::ProppedCantilever, "Propped Cantilever (Fixed - Pinned)");
                                        ui.selectable_value(&mut self.blast_support_condition, crate::core::SupportCondition::Cantilever, "Cantilever (Fixed - Free)");
                                    });

                                ui.add_space(4.0);
                                ui.label(RichText::new("Clear Span Length L (m):").size(10.5));
                                ui.add(egui::Slider::new(&mut self.blast_span_length_m, 2.0..=8.0).suffix(" m"));

                                ui.add_space(4.0);
                                ui.label(RichText::new("Tributary Width B (m):").size(10.5));
                                ui.add(egui::Slider::new(&mut self.blast_tributary_width_m, 0.5..=3.0).suffix(" m"));

                                ui.add_space(4.0);
                                ui.label(RichText::new("Component Thickness h (m):").size(10.5));
                                ui.add(egui::Slider::new(&mut self.blast_thickness_m, 0.15..=0.80).suffix(" m"));
                            });
                }
                _ => {
                    // Column 3: Materials & Antiterrorism Target
                            ui.vertical(|ui| {
                                ui.label(RichText::new("3. Material Capacities & Target Damage").strong().size(11.0).color(theme.accent_blue));
                                ui.add_space(4.0);

                                ui.label(RichText::new("Concrete f'c (MPa):").size(10.5));
                                ui.add(egui::Slider::new(&mut self.blast_fc_mpa, 25.0..=80.0).suffix(" MPa"));

                                ui.add_space(4.0);
                                ui.label(RichText::new("Steel Rebar / Profile fy (MPa):").size(10.5));
                                ui.add(egui::Slider::new(&mut self.blast_fy_mpa, 280.0..=550.0).suffix(" MPa"));

                                ui.add_space(4.0);
                                ui.label(RichText::new("Rebar Flexural Ratio ρ (%):").size(10.5));
                                ui.add(egui::Slider::new(&mut self.blast_rebar_percent, 0.40..=2.50).suffix(" %"));

                                ui.add_space(4.0);
                                ui.label(RichText::new("Target PDC/UFC Damage Level:").size(10.5));
                                egui::ComboBox::from_id_salt("blast_target_combo")
                                    .selected_text(self.blast_target_damage.label())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.blast_target_damage, crate::core::UfcDamageLevel::Superficial, "B1: Superficial (Elastic, Reusable)");
                                        ui.selectable_value(&mut self.blast_target_damage, crate::core::UfcDamageLevel::Low, "B2: Low Damage (Minor Yielding, Occupiable)");
                                        ui.selectable_value(&mut self.blast_target_damage, crate::core::UfcDamageLevel::Medium, "B3: Medium Damage (Moderate Plasticity)");
                                        ui.selectable_value(&mut self.blast_target_damage, crate::core::UfcDamageLevel::High, "B4: High Damage (Life Safety)");
                                    });

                                ui.add_space(6.0);
                                ui.label(
                                    RichText::new(format!(
                                        "SDOF Mass Me = {:.1} kg | Tn = {:.1} ms\nUltimate Ru = {:.1} kN | ye = {:.2} mm",
                                        rep.sdof_system.equivalent_mass_kg,
                                        rep.sdof_system.natural_period_ms,
                                        rep.sdof_system.ultimate_resistance_kn,
                                        rep.sdof_system.yield_displacement_mm
                                    ))
                                    .size(10.0)
                                    .color(theme.text_secondary),
                                );
                            });
                }
            });
        });

        ui.add_space(10.0);

        // 2D Vector CAD Canvas: Blast Waveform P(t) (Left) & SDOF Response y(t) with P-I Contour (Right)
        theme.card_frame().show(ui, |ui| {
            icons::icon_label(ui, AppIcon::ChartLine, 13.5, theme.accent_blue, "Dynamic Waveform & SDOF Elastoplastic Response Trajectory (UFC 3-340-02)", 12.5, theme.accent_blue, true);
            ui.add_space(6.0);

            let (canvas_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 260.0), Sense::hover());
            let painter = ui.painter_at(canvas_rect);

            painter.rect_filled(canvas_rect, CornerRadius::same(6), Color32::from_rgb(18, 22, 28));
            painter.rect_stroke(canvas_rect, CornerRadius::same(6), Stroke::new(1.0, theme.border_subtle), egui::StrokeKind::Inside);

            let half_w = canvas_rect.width() * 0.5 - 10.0;
            let left_rect = egui::Rect::from_min_size(canvas_rect.min + Vec2::new(5.0, 5.0), Vec2::new(half_w, canvas_rect.height() - 10.0));
            let right_rect = egui::Rect::from_min_size(Pos2::new(canvas_rect.min.x + half_w + 15.0, canvas_rect.min.y + 5.0), Vec2::new(half_w, canvas_rect.height() - 10.0));

            // Grid lines on both canvases
            for r in &[left_rect, right_rect] {
                for i in 1..5 {
                    let y_line = r.min.y + (r.height() / 5.0) * (i as f32);
                    painter.line_segment([Pos2::new(r.min.x, y_line), Pos2::new(r.max.x, y_line)], Stroke::new(0.5, Color32::from_rgb(35, 42, 54)));
                }
            }

            // === LEFT CANVAS: Friedlander Blast Waveform P(t) ===
            let td = rep.blast_wave.positive_phase_duration_ms as f32;
            let pr = rep.blast_wave.peak_reflected_overpressure_kpa as f32;
            let b = rep.blast_wave.friedlander_decay_coeff as f32;
            let t_window = (td * 2.5).max(30.0);

            // Shaded positive impulse area
            let mut wave_pts = Vec::new();
            let steps = 60;
            for s in 0..=steps {
                let t = (s as f32 / steps as f32) * td;
                let tau = (t / td).clamp(0.0, 1.0);
                let p = pr * (1.0 - tau) * (-b * tau).exp();
                let sx = left_rect.min.x + 35.0 + (t / t_window) * (left_rect.width() - 50.0);
                let sy = left_rect.max.y - 30.0 - (p / pr.max(1.0)) * (left_rect.height() - 60.0);
                wave_pts.push(Pos2::new(sx, sy));
            }

            // Draw shaded positive phase impulse
            if wave_pts.len() >= 2 {
                let mut poly_pts = wave_pts.clone();
                let baseline_y = left_rect.max.y - 30.0;
                poly_pts.push(Pos2::new(wave_pts.last().unwrap().x, baseline_y));
                poly_pts.push(Pos2::new(wave_pts.first().unwrap().x, baseline_y));
                painter.add(egui::Shape::convex_polygon(
                    poly_pts,
                    Color32::from_rgba_unmultiplied(239, 68, 68, 45),
                    Stroke::NONE,
                ));

                for w in 0..(wave_pts.len() - 1) {
                    painter.line_segment([wave_pts[w], wave_pts[w + 1]], Stroke::new(2.2, Color32::from_rgb(239, 68, 68)));
                }
            }

            // Arrival time line
            let arrival_x = left_rect.min.x + 35.0;
            let base_y = left_rect.max.y - 30.0;
            painter.line_segment([Pos2::new(arrival_x, left_rect.min.y + 20.0), Pos2::new(arrival_x, base_y)], Stroke::new(1.0, theme.accent_blue));

            painter.text(
                Pos2::new(left_rect.min.x + 10.0, left_rect.min.y + 10.0),
                Align2::LEFT_TOP,
                "Normally Reflected Blast Waveform P(t) [Friedlander Profile]",
                egui::FontId::proportional(11.0),
                Color32::from_rgb(239, 68, 68),
            );
            painter.text(
                Pos2::new(left_rect.min.x + 40.0, left_rect.min.y + 35.0),
                Align2::LEFT_TOP,
                format!("Pr = {:.1} kPa | td = {:.2} ms | ir = {:.1} kPa·ms", pr, td, rep.blast_wave.reflected_impulse_kpa_ms),
                egui::FontId::proportional(10.0),
                Color32::from_rgb(245, 158, 11),
            );

            // === RIGHT CANVAS: SDOF Dynamic Displacement History y(t) ===
            let samples = &rep.time_history_samples;
            if !samples.is_empty() {
                let max_disp_canvas = (rep.max_displacement_mm as f32 * 1.30).max(rep.sdof_system.yield_displacement_mm as f32 * 1.5).max(1.0);
                let t_end_samples = samples.last().map(|s| s.time_ms as f32).unwrap_or(50.0);

                let ye_y = right_rect.max.y - 30.0 - (rep.sdof_system.yield_displacement_mm as f32 / max_disp_canvas) * (right_rect.height() - 60.0);
                painter.line_segment(
                    [Pos2::new(right_rect.min.x + 35.0, ye_y), Pos2::new(right_rect.max.x - 15.0, ye_y)],
                    Stroke::new(1.0, Color32::from_rgb(16, 185, 129)),
                );
                painter.text(
                    Pos2::new(right_rect.max.x - 18.0, ye_y - 2.0),
                    Align2::RIGHT_BOTTOM,
                    format!("Yield ye = {:.1} mm", rep.sdof_system.yield_displacement_mm),
                    egui::FontId::proportional(9.5),
                    Color32::from_rgb(16, 185, 129),
                );

                let mut disp_pts = Vec::new();
                for pt in samples {
                    let sx = right_rect.min.x + 35.0 + (pt.time_ms as f32 / t_end_samples) * (right_rect.width() - 50.0);
                    let sy = right_rect.max.y - 30.0 - (pt.displacement_mm as f32 / max_disp_canvas) * (right_rect.height() - 60.0);
                    disp_pts.push(Pos2::new(sx, sy));
                }

                for w in 0..(disp_pts.len().saturating_sub(1)) {
                    painter.line_segment([disp_pts[w], disp_pts[w + 1]], Stroke::new(2.2, theme.accent_blue));
                }

                // Peak displacement marker
                let peak_x = right_rect.min.x + 35.0 + (rep.time_at_max_disp_ms as f32 / t_end_samples) * (right_rect.width() - 50.0);
                let peak_y = right_rect.max.y - 30.0 - (rep.max_displacement_mm as f32 / max_disp_canvas) * (right_rect.height() - 60.0);
                painter.circle_filled(Pos2::new(peak_x, peak_y), 4.5, Color32::from_rgb(249, 115, 22));
                painter.text(
                    Pos2::new(peak_x + 6.0, peak_y - 8.0),
                    Align2::LEFT_BOTTOM,
                    format!("ymax = {:.1} mm (μ={:.2})", rep.max_displacement_mm, rep.dynamic_ductility_ratio),
                    egui::FontId::proportional(10.0),
                    Color32::from_rgb(249, 115, 22),
                );
            }

            painter.text(
                Pos2::new(right_rect.min.x + 10.0, right_rect.min.y + 10.0),
                Align2::LEFT_TOP,
                "Dynamic Elastoplastic Displacement History y(t) & Resistance",
                egui::FontId::proportional(11.0),
                theme.accent_blue,
            );
            painter.text(
                Pos2::new(right_rect.min.x + 35.0, right_rect.min.y + 35.0),
                Align2::LEFT_TOP,
                format!("Support Rotation θ = {:.2}° | Perm Offset = {:.1} mm", rep.support_rotation_deg, rep.permanent_displacement_mm),
                egui::FontId::proportional(10.0),
                theme.text_secondary,
            );
        });

        ui.add_space(10.0);

        // Regulatory & UFC 3-340-02 / ASCE 59-11 Compliance Audit Table
        theme.card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                icons::icon_label(ui, AppIcon::Clipboard, 13.5, theme.accent_blue, "UFC 3-340-02 & ASCE 59-11 Antiterrorism Compliance Audit", 12.5, theme.accent_blue, true);
                ui.add_space(20.0);
                let (ov_ic, overall_status, status_col) = if rep.is_target_damage_achieved {
                    (AppIcon::CheckCircle, "TARGET DAMAGE LEVEL ACHIEVED", theme.accent_green)
                } else {
                    (AppIcon::AlertTriangle, "TARGET DAMAGE DEFICIT", Color32::from_rgb(239, 68, 68))
                };
                icons::pill_badge(ui, ov_ic, overall_status, status_col);
            });

            ui.add_space(6.0);

            // Table Header
            ui.horizontal(|ui| {
                ui.label(RichText::new("Performance Check").size(10.5).strong().color(theme.text_secondary));
                ui.add_space(220.0);
                ui.label(RichText::new("Code / Standard Reference").size(10.5).strong().color(theme.text_secondary));
                ui.add_space(150.0);
                ui.label(RichText::new("Demand").size(10.5).strong().color(theme.text_secondary));
                ui.add_space(45.0);
                ui.label(RichText::new("Limit").size(10.5).strong().color(theme.text_secondary));
                ui.add_space(45.0);
                ui.label(RichText::new("D/C Ratio").size(10.5).strong().color(theme.text_secondary));
                ui.add_space(30.0);
                ui.label(RichText::new("Status").size(10.5).strong().color(theme.text_secondary));
            });
            ui.separator();

            for item in &rep.compliance_audit {
                let row_col = if item.is_passed { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&item.parameter_name).size(10.5).color(theme.accent_blue));
                    ui.add_space(10.0);
                    ui.label(RichText::new(&item.code_reference).size(10.5).color(theme.text_primary));
                    ui.add_space(10.0);
                    ui.label(RichText::new(format!("{:.2} {}", item.demand_val, item.unit)).size(10.5).color(theme.text_primary));
                    ui.add_space(15.0);
                    ui.label(RichText::new(format!("{:.2} {}", item.permissible_limit, item.unit)).size(10.5).color(theme.accent_green));
                    ui.add_space(15.0);
                    ui.label(RichText::new(format!("{:.3}", item.utilization_ratio)).size(10.5).strong().color(row_col));
                    ui.add_space(20.0);
                    let (r_ic, r_txt) = if item.is_passed { (AppIcon::CheckCircle, "PASS") } else { (AppIcon::AlertTriangle, "FAIL") };
                    icons::icon_label(ui, r_ic, 11.0, row_col, r_txt, 10.5, row_col, true);
                });
                ui.separator();
            }
        });
    }

    pub(super) fn render_fire_engineering(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let input = crate::core::FireAnalysisInput {
            project_name: self.fire_project_name.clone(),
            fire_curve: self.fire_curve_kind,
            member_kind: self.fire_member_kind,
            protection_kind: self.fire_protection_kind,
            protection_thickness_mm: self.fire_protection_thickness_mm as f64,
            fire_duration_minutes: self.fire_duration_minutes as f64,
            target_frr_minutes: self.fire_target_frr_minutes as f64,
            clear_span_m: self.fire_span_m as f64,
            section_depth_mm: self.fire_depth_mm as f64,
            section_width_mm: self.fire_width_mm as f64,
            web_thickness_mm: self.fire_web_thk_mm as f64,
            flange_thickness_mm: self.fire_flange_thk_mm as f64,
            concrete_cover_mm: self.fire_cover_mm as f64,
            aggregate_kind: self.fire_aggregate_kind,
            steel_fy_mpa: self.fire_steel_fy_mpa as f64,
            concrete_fc_mpa: self.fire_concrete_fc_mpa as f64,
            service_moment_demand_knm: self.fire_moment_demand_knm as f64,
            service_axial_demand_kn: self.fire_axial_demand_kn as f64,
            thermal_restraint: self.fire_thermal_restraint,
        };

        let rep = crate::core::FireEngine::run_assessment(&input);

        // 4 Executive KPI Cards
        ui_kit::responsive_grid_n(ui, 4, 210.0, |ui, idx| match idx {
    0 => {
        // Card 1: Fire Exposure & Gas Temp
            theme.card_frame().show(ui, |ui| {
                    ui.set_width(ui.available_width());
                icons::icon_label(ui, AppIcon::Flame, 12.0, Color32::from_rgb(239, 68, 68), "Fire Exposure & Temperatures", 11.0, theme.text_secondary, true);
                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!("{:.0} °C", rep.final_member_temp_c))
                        .size(19.0)
                        .strong()
                        .color(Color32::from_rgb(239, 68, 68)),
                );
                ui.label(
                    RichText::new(format!(
                        "Gas Tg = {:.0} °C | Am/V = {:.1} m⁻¹\nCurve: {}",
                        rep.peak_gas_temp_c,
                        rep.section_factor_m_inv,
                        rep.input.fire_curve.label()
                    ))
                    .size(10.5)
                    .color(theme.text_secondary),
                );
            });
    }
    1 => {
        // Card 2: Fire Resistance Rating (FRR)
            let frr_col = if rep.is_target_frr_achieved { theme.accent_green } else { Color32::from_rgb(245, 158, 11) };
            theme.card_frame().show(ui, |ui| {
                    ui.set_width(ui.available_width());
                icons::icon_label(ui, AppIcon::Shield, 12.0, frr_col, "Fire Endurance Rating", 11.0, theme.text_secondary, true);
                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!("R{:.0} ({:.0} min)", rep.fire_endurance_rating_minutes, rep.fire_endurance_rating_minutes))
                        .size(19.0)
                        .strong()
                        .color(frr_col),
                );
                let (ach_ic, achieved_str, ach_col) = if rep.is_target_frr_achieved {
                    (AppIcon::CheckCircle, "Target FRR Satisfied", theme.accent_green)
                } else {
                    (AppIcon::AlertTriangle, "Target FRR Deficit", Color32::from_rgb(239, 68, 68))
                };
                icons::icon_label(ui, ach_ic, 11.0, ach_col, achieved_str, 10.5, ach_col, true);
                ui.label(
                    RichText::new(format!(
                        "Target: R{:.0} | Tcrit = {:.0} °C",
                        rep.input.target_frr_minutes,
                        rep.critical_temperature_c
                    ))
                    .size(10.5)
                    .color(theme.text_secondary),
                );
            });
    }
    2 => {
        // Card 3: Degraded Elevated Capacity
            theme.card_frame().show(ui, |ui| {
                    ui.set_width(ui.available_width());
                icons::icon_label(ui, AppIcon::ChartLine, 12.0, theme.accent_blue, "Elevated Structural Capacity", 11.0, theme.text_secondary, true);
                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!("D/C = {:.2}", rep.max_utilization_ratio))
                        .size(19.0)
                        .strong()
                        .color(if rep.max_utilization_ratio <= 1.0 { theme.accent_blue } else { Color32::from_rgb(239, 68, 68) }),
                );
                ui.label(
                    RichText::new(format!(
                        "Cold: {:.0} | Elevated: {:.0}\nRetained: {:.1}%",
                        rep.ambient_capacity,
                        rep.residual_capacity_at_duration,
                        (rep.residual_capacity_at_duration / rep.ambient_capacity.max(0.1)) * 100.0
                    ))
                    .size(10.5)
                    .color(theme.text_secondary),
                );
            });
    }
    _ => {
        // Card 4: Passive Fire Insulation Sizing
            theme.card_frame().show(ui, |ui| {
                    ui.set_width(ui.available_width());
                icons::icon_label(ui, AppIcon::Wall, 12.0, Color32::from_rgb(168, 85, 247), "Passive Protection Sizing", 11.0, theme.text_secondary, true);
                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!("{:.1} mm", rep.input.protection_thickness_mm))
                        .size(19.0)
                        .strong()
                        .color(Color32::from_rgb(168, 85, 247)),
                );
                ui.label(
                    RichText::new(format!(
                        "Req for Target: {:.1} mm\nSystem: {}",
                        rep.required_insulation_thickness_mm,
                        rep.input.protection_kind.label()
                    ))
                    .size(10.5)
                    .color(theme.text_secondary),
                );
            });
    }
});

        ui.add_space(10.0);

        // Parameter Controls Panel (3 Columns)
        theme.card_frame().show(ui, |ui| {
            icons::icon_label(ui, AppIcon::SettingsGear, 13.5, theme.accent_blue, "Fire Exposure, Member Section Geometry & Passive Protection Controls", 12.5, theme.accent_blue, true);
            ui.add_space(8.0);

            ui_kit::responsive_grid_n(ui, 3, 240.0, |ui, col_idx| match col_idx {
                0 => {
                    // Column 1: Fire Exposure & Targets
                            ui.vertical(|ui| {
                                ui.label(RichText::new("1. Fire Curve & Target Endurance").strong().size(11.0).color(theme.accent_blue));
                                ui.add_space(4.0);

                                ui.label(RichText::new("Design Fire Exposure Curve:").size(10.5));
                                egui::ComboBox::from_id_salt("fire_curve_combo")
                                    .selected_text(self.fire_curve_kind.label())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.fire_curve_kind, crate::core::FireCurveKind::Iso834Standard, "ISO 834 / EN 1991-1-2 Standard Fire");
                                        ui.selectable_value(&mut self.fire_curve_kind, crate::core::FireCurveKind::AstmE119, "ASTM E119 / UL 263 Standard Fire");
                                        ui.selectable_value(&mut self.fire_curve_kind, crate::core::FireCurveKind::EurocodeHydrocarbon, "Eurocode Hydrocarbon (HC) Rapid Fire");
                                        ui.selectable_value(&mut self.fire_curve_kind, crate::core::FireCurveKind::ExternalFireCurve, "External Exposure Fire Curve");
                                        ui.selectable_value(&mut self.fire_curve_kind, crate::core::FireCurveKind::ParametricNaturalFire, "Parametric Natural Fire with Decay");
                                    });

                                ui.add_space(4.0);
                                ui.label(RichText::new("Fire Duration Time (min):").size(10.5));
                                ui.add(egui::Slider::new(&mut self.fire_duration_minutes, 15.0..=240.0).suffix(" min"));

                                ui.add_space(4.0);
                                ui.label(RichText::new("Target Fire Rating FRR (min):").size(10.5));
                                ui.add(egui::Slider::new(&mut self.fire_target_frr_minutes, 30.0..=180.0).suffix(" min"));

                                ui.add_space(4.0);
                                ui.label(RichText::new("Thermal Restraint Boundary:").size(10.5));
                                egui::ComboBox::from_id_salt("fire_restr_combo")
                                    .selected_text(self.fire_thermal_restraint.label())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.fire_thermal_restraint, crate::core::ThermalRestraintKind::Unrestrained, "Unrestrained (Free Expansion)");
                                        ui.selectable_value(&mut self.fire_thermal_restraint, crate::core::ThermalRestraintKind::ModeratelyRestrained, "Moderately Restrained (Multi-Bay Frame)");
                                        ui.selectable_value(&mut self.fire_thermal_restraint, crate::core::ThermalRestraintKind::RigidlyRestrained, "Rigidly Restrained (Stiff Shaft / Bunker)");
                                    });
                            });
                }
                1 => {
                    // Column 2: Member Archetype & Section Dimensions
                            ui.vertical(|ui| {
                                ui.label(RichText::new("2. Member Archetype & Section Dimensions").strong().size(11.0).color(theme.accent_blue));
                                ui.add_space(4.0);

                                ui.label(RichText::new("Member Archetype:").size(10.5));
                                egui::ComboBox::from_id_salt("fire_member_combo")
                                    .selected_text(self.fire_member_kind.label())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.fire_member_kind, crate::core::FireMemberKind::SteelBeam, "Structural Steel W-Beam (Flexure)");
                                        ui.selectable_value(&mut self.fire_member_kind, crate::core::FireMemberKind::SteelColumn, "Structural Steel W-Column (Compression)");
                                        ui.selectable_value(&mut self.fire_member_kind, crate::core::FireMemberKind::RcBeam, "RC Rectangular Beam (Flexure)");
                                        ui.selectable_value(&mut self.fire_member_kind, crate::core::FireMemberKind::RcColumn, "RC Square Column (Biaxial Compression)");
                                    });

                                ui.add_space(4.0);
                                ui.label(RichText::new("Clear Span Length L (m):").size(10.5));
                                ui.add(egui::Slider::new(&mut self.fire_span_m, 3.0..=15.0).suffix(" m"));

                                ui.add_space(4.0);
                                ui.label(RichText::new("Section Depth D (mm):").size(10.5));
                                ui.add(egui::Slider::new(&mut self.fire_depth_mm, 200.0..=900.0).suffix(" mm"));

                                ui.add_space(4.0);
                                ui.label(RichText::new("Section Width B (mm):").size(10.5));
                                ui.add(egui::Slider::new(&mut self.fire_width_mm, 150.0..=500.0).suffix(" mm"));

                                if matches!(self.fire_member_kind, crate::core::FireMemberKind::RcBeam | crate::core::FireMemberKind::RcColumn) {
                                    ui.add_space(4.0);
                                    ui.label(RichText::new("Clear Concrete Cover (mm):").size(10.5));
                                    ui.add(egui::Slider::new(&mut self.fire_cover_mm, 20.0..=70.0).suffix(" mm"));
                                } else {
                                    ui.add_space(4.0);
                                    ui.label(RichText::new("Flange Thickness tf (mm):").size(10.5));
                                    ui.add(egui::Slider::new(&mut self.fire_flange_thk_mm, 8.0..=35.0).suffix(" mm"));
                                }
                            });
                }
                _ => {
                    // Column 3: Material Strengths & Passive Protection
                            ui.vertical(|ui| {
                                ui.label(RichText::new("3. Protection & Material Demands").strong().size(11.0).color(theme.accent_blue));
                                ui.add_space(4.0);

                                ui.label(RichText::new("Passive Protection System:").size(10.5));
                                egui::ComboBox::from_id_salt("fire_prot_combo")
                                    .selected_text(self.fire_protection_kind.label())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.fire_protection_kind, crate::core::FireProtectionKind::SprayedMineralFiber, "Sprayed Fire Resistive (SFRM / Gypsum)");
                                        ui.selectable_value(&mut self.fire_protection_kind, crate::core::FireProtectionKind::IntumescentCoating, "Thin-Film Solvent Intumescent Coating");
                                        ui.selectable_value(&mut self.fire_protection_kind, crate::core::FireProtectionKind::BoardEncasement, "Calcium Silicate Board Encasement");
                                        ui.selectable_value(&mut self.fire_protection_kind, crate::core::FireProtectionKind::ConcreteEncasement, "Solid Structural Concrete Encasement");
                                        ui.selectable_value(&mut self.fire_protection_kind, crate::core::FireProtectionKind::Unprotected, "Unprotected Bare Structural Steel");
                                    });

                                ui.add_space(4.0);
                                ui.label(RichText::new("Insulation Thickness dp (mm):").size(10.5));
                                ui.add(egui::Slider::new(&mut self.fire_protection_thickness_mm, 5.0..=50.0).suffix(" mm"));

                                ui.add_space(4.0);
                                ui.label(RichText::new("Steel Yield Strength fy (MPa):").size(10.5));
                                ui.add(egui::Slider::new(&mut self.fire_steel_fy_mpa, 235.0..=460.0).suffix(" MPa"));

                                ui.add_space(4.0);
                                ui.label(RichText::new("Applied Moment / Axial Service Load:").size(10.5));
                                ui.add(egui::Slider::new(&mut self.fire_moment_demand_knm, 50.0..=600.0).suffix(" kNm"));
                            });
                }
            });
        });

        ui.add_space(10.0);

        // 2D Vector CAD Canvas: Temperature Histories (Left) & Capacity Degradation (Right)
        theme.card_frame().show(ui, |ui| {
            icons::icon_label(ui, AppIcon::ChartLine, 13.5, theme.accent_blue, "Transient Heat Conduction & Elevated Temperature Structural Degradation", 12.5, theme.accent_blue, true);
            ui.add_space(6.0);

            let (canvas_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 260.0), Sense::hover());
            let painter = ui.painter_at(canvas_rect);

            painter.rect_filled(canvas_rect, CornerRadius::same(6), Color32::from_rgb(18, 22, 28));
            painter.rect_stroke(canvas_rect, CornerRadius::same(6), Stroke::new(1.0, theme.border_subtle), egui::StrokeKind::Inside);

            let half_w = canvas_rect.width() * 0.5 - 10.0;
            let left_rect = egui::Rect::from_min_size(canvas_rect.min + Vec2::new(5.0, 5.0), Vec2::new(half_w, canvas_rect.height() - 10.0));
            let right_rect = egui::Rect::from_min_size(Pos2::new(canvas_rect.min.x + half_w + 15.0, canvas_rect.min.y + 5.0), Vec2::new(half_w, canvas_rect.height() - 10.0));

            // Grid lines on both canvases
            for r in &[left_rect, right_rect] {
                for i in 1..5 {
                    let y_line = r.min.y + (r.height() / 5.0) * (i as f32);
                    painter.line_segment([Pos2::new(r.min.x, y_line), Pos2::new(r.max.x, y_line)], Stroke::new(0.5, Color32::from_rgb(35, 42, 54)));
                }
            }

            // === LEFT CANVAS: Gas Temp Tg(t) and Member Core Temp Ts(t) ===
            let samples = &rep.time_history_samples;
            if !samples.is_empty() {
                let max_temp_canvas = 1200.0_f32;
                let t_end = rep.input.fire_duration_minutes as f32;

                // Critical steel temperature horizontal line (Tcrit = 550 C)
                let crit_y = left_rect.max.y - 30.0 - (rep.critical_temperature_c as f32 / max_temp_canvas) * (left_rect.height() - 60.0);
                painter.line_segment(
                    [Pos2::new(left_rect.min.x + 35.0, crit_y), Pos2::new(left_rect.max.x - 15.0, crit_y)],
                    Stroke::new(1.0, Color32::from_rgb(239, 68, 68)),
                );
                painter.text(
                    Pos2::new(left_rect.max.x - 18.0, crit_y - 2.0),
                    Align2::RIGHT_BOTTOM,
                    format!("Critical Tcrit = {:.0} °C", rep.critical_temperature_c),
                    egui::FontId::proportional(9.5),
                    Color32::from_rgb(239, 68, 68),
                );

                // Gas Temp curve
                let mut tg_pts = Vec::new();
                let mut ts_pts = Vec::new();
                for pt in samples {
                    let sx = left_rect.min.x + 35.0 + (pt.time_minutes as f32 / t_end) * (left_rect.width() - 50.0);
                    let sy_g = left_rect.max.y - 30.0 - (pt.gas_temp_c as f32 / max_temp_canvas) * (left_rect.height() - 60.0);
                    let sy_s = left_rect.max.y - 30.0 - (pt.member_temp_c as f32 / max_temp_canvas) * (left_rect.height() - 60.0);
                    tg_pts.push(Pos2::new(sx, sy_g));
                    ts_pts.push(Pos2::new(sx, sy_s));
                }

                for w in 0..(tg_pts.len().saturating_sub(1)) {
                    painter.line_segment([tg_pts[w], tg_pts[w + 1]], Stroke::new(1.8, Color32::from_rgb(245, 158, 11)));
                }
                for w in 0..(ts_pts.len().saturating_sub(1)) {
                    painter.line_segment([ts_pts[w], ts_pts[w + 1]], Stroke::new(2.4, Color32::from_rgb(239, 68, 68)));
                }

                // Final temperature marker
                if let Some(last_pt) = ts_pts.last() {
                    painter.circle_filled(*last_pt, 4.5, Color32::from_rgb(239, 68, 68));
                    painter.text(
                        Pos2::new(last_pt.x - 5.0, last_pt.y - 8.0),
                        Align2::RIGHT_BOTTOM,
                        format!("{:.0} °C", rep.final_member_temp_c),
                        egui::FontId::proportional(10.0),
                        Color32::from_rgb(239, 68, 68),
                    );
                }
            }

            painter.text(
                Pos2::new(left_rect.min.x + 10.0, left_rect.min.y + 10.0),
                Align2::LEFT_TOP,
                "Gas Temp Tg(t) [Amber] vs Member Core Temp Ts(t) [Red]",
                egui::FontId::proportional(11.0),
                Color32::from_rgb(245, 158, 11),
            );

            // === RIGHT CANVAS: Capacity Degradation & Demand D/C ===
            if !samples.is_empty() {
                let max_cap_canvas = (rep.ambient_capacity as f32 * 1.15).max(10.0);
                let t_end = rep.input.fire_duration_minutes as f32;

                // Demand horizontal line
                let applied_demand = rep.input.service_moment_demand_knm as f32;
                let demand_y = right_rect.max.y - 30.0 - (applied_demand / max_cap_canvas) * (right_rect.height() - 60.0);
                painter.line_segment(
                    [Pos2::new(right_rect.min.x + 35.0, demand_y), Pos2::new(right_rect.max.x - 15.0, demand_y)],
                    Stroke::new(1.0, Color32::from_rgb(245, 158, 11)),
                );
                painter.text(
                    Pos2::new(right_rect.max.x - 18.0, demand_y - 2.0),
                    Align2::RIGHT_BOTTOM,
                    format!("Demand = {:.0} kNm", applied_demand),
                    egui::FontId::proportional(9.5),
                    Color32::from_rgb(245, 158, 11),
                );

                let mut cap_pts = Vec::new();
                for pt in samples {
                    let sx = right_rect.min.x + 35.0 + (pt.time_minutes as f32 / t_end) * (right_rect.width() - 50.0);
                    let sy = right_rect.max.y - 30.0 - (pt.elevated_capacity as f32 / max_cap_canvas) * (right_rect.height() - 60.0);
                    cap_pts.push(Pos2::new(sx, sy));
                }

                for w in 0..(cap_pts.len().saturating_sub(1)) {
                    painter.line_segment([cap_pts[w], cap_pts[w + 1]], Stroke::new(2.4, theme.accent_blue));
                }

                // Fire endurance failure marker (if failed)
                let fail_x = right_rect.min.x + 35.0 + (rep.fire_endurance_rating_minutes as f32 / t_end) * (right_rect.width() - 50.0);
                painter.line_segment(
                    [Pos2::new(fail_x, right_rect.min.y + 20.0), Pos2::new(fail_x, right_rect.max.y - 30.0)],
                    Stroke::new(1.2, Color32::from_rgb(168, 85, 247)),
                );
                painter.text(
                    Pos2::new(fail_x + 4.0, right_rect.min.y + 25.0),
                    Align2::LEFT_TOP,
                    format!("FRR = R{:.0}", rep.fire_endurance_rating_minutes),
                    egui::FontId::proportional(10.0),
                    Color32::from_rgb(168, 85, 247),
                );
            }

            painter.text(
                Pos2::new(right_rect.min.x + 10.0, right_rect.min.y + 10.0),
                Align2::LEFT_TOP,
                "Elevated Plastic Capacity Mn,T(t) & Fire Endurance Rating",
                egui::FontId::proportional(11.0),
                theme.accent_blue,
            );
        });

        ui.add_space(10.0);

        // Regulatory & Code Compliance Audit Table (AISC 360-16 / Eurocode 2 & 3)
        theme.card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                icons::icon_label(ui, AppIcon::Clipboard, 13.5, theme.accent_blue, "AISC 360-16 & Eurocode 1/2/3 Fire Compliance Audit", 12.5, theme.accent_blue, true);
                ui.add_space(20.0);
                let (ov_ic, overall_status, status_col) = if rep.is_target_frr_achieved {
                    (AppIcon::CheckCircle, "TARGET FIRE ENDURANCE ACHIEVED", theme.accent_green)
                } else {
                    (AppIcon::AlertTriangle, "TARGET FIRE ENDURANCE DEFICIT", Color32::from_rgb(239, 68, 68))
                };
                icons::pill_badge(ui, ov_ic, overall_status, status_col);
            });

            ui.add_space(6.0);

            // Table Header
            ui.horizontal(|ui| {
                ui.label(RichText::new("Performance Check").size(10.5).strong().color(theme.text_secondary));
                ui.add_space(220.0);
                ui.label(RichText::new("Governing Standard").size(10.5).strong().color(theme.text_secondary));
                ui.add_space(150.0);
                ui.label(RichText::new("Demand").size(10.5).strong().color(theme.text_secondary));
                ui.add_space(45.0);
                ui.label(RichText::new("Limit").size(10.5).strong().color(theme.text_secondary));
                ui.add_space(45.0);
                ui.label(RichText::new("D/C Ratio").size(10.5).strong().color(theme.text_secondary));
                ui.add_space(30.0);
                ui.label(RichText::new("Status").size(10.5).strong().color(theme.text_secondary));
            });
            ui.separator();

            for item in &rep.compliance_audit {
                let row_col = if item.is_passed { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&item.parameter_name).size(10.5).color(theme.accent_blue));
                    ui.add_space(10.0);
                    ui.label(RichText::new(&item.code_reference).size(10.5).color(theme.text_primary));
                    ui.add_space(10.0);
                    ui.label(RichText::new(format!("{:.1} {}", item.demand_val, item.unit)).size(10.5).color(theme.text_primary));
                    ui.add_space(15.0);
                    ui.label(RichText::new(format!("{:.1} {}", item.permissible_limit, item.unit)).size(10.5).color(theme.accent_green));
                    ui.add_space(15.0);
                    ui.label(RichText::new(format!("{:.3}", item.utilization_ratio)).size(10.5).strong().color(row_col));
                    ui.add_space(20.0);
                    let (r_ic, r_txt) = if item.is_passed { (AppIcon::CheckCircle, "PASS") } else { (AppIcon::AlertTriangle, "FAIL") };
                    icons::icon_label(ui, r_ic, 11.0, row_col, r_txt, 10.5, row_col, true);
                });
                ui.separator();
            }
        });
    }
}
