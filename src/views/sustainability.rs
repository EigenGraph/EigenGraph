//! sustainability.rs — Pillar 6: 6D BIM Sustainability & Energy.
//! Analysis of energy efficiency, carbon footprint, environmental impact, and green building certification requirements.

use egui::{
    Align, Align2, Color32, CornerRadius, Frame, Layout, Margin, Pos2, RichText,
    ScrollArea, Sense, Stroke, Ui, Vec2,
};
use crate::features::{FeatureId, FeatureRegistry};
use crate::models::{GreenBuildingCredit, LcaStageEmission, StructuralMember};
use crate::theme::MacTheme;
use crate::ui_kit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SustainabilitySubTab {
    EmbodiedCarbon,
    CarbonAriTradeoff,
    GreenCertification,
    EnergyLifecycle,
    SolarOttvDaylight,
    CircularDfdPassport,
}

pub struct SustainabilityView {
    pub active_subtab: SustainabilitySubTab,
    pub steel_production_route: SteelProductionRoute,
    pub target_rating: GreenRatingTarget,
    // Phase 28: Whole-Life LCA & Multi-Standard Green Scorecards
    pub lca_stages: Vec<LcaStageEmission>,
    pub green_credits: Vec<GreenBuildingCredit>,
    pub selected_credit_standard: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SteelProductionRoute {
    ElectricArcFurnace, // EAF: ~1.15 t CO2e/t (High recycled scrap)
    BlastFurnaceBof,    // BF-BOF: ~2.35 t CO2e/t (Virgin ore)
    HydrogenDri,        // H2-DRI: ~0.45 t CO2e/t (Green steel)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GreenRatingTarget {
    GreenshipPlatinum,
    GreenshipGold,
    LeedPlatinum,
    LeedGold,
}

impl Default for SustainabilityView {
    fn default() -> Self {
        Self {
            active_subtab: SustainabilitySubTab::EmbodiedCarbon,
            steel_production_route: SteelProductionRoute::ElectricArcFurnace,
            target_rating: GreenRatingTarget::GreenshipPlatinum,
            lca_stages: LcaStageEmission::default_stages(),
            green_credits: GreenBuildingCredit::default_credits(),
            selected_credit_standard: None,
        }
    }
}

impl SustainabilityView {
    pub fn show(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        members: &[StructuralMember],
        features: &mut FeatureRegistry,
    ) {
        let is_mobile = ui.available_width() < MacTheme::MOBILE_BREAKPOINT;
        let pad = if is_mobile { 12 } else { 20 };

        ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
            Frame::new().inner_margin(Margin::symmetric(pad, 12)).show(ui, |ui| {
                // 1. TOP SUBTABS SELECTOR
                self.render_header_bar(ui, theme);

                ui.add_space(16.0);

                // 2. ACTIVE SUB-TAB CONTENT
                match self.active_subtab {
                    SustainabilitySubTab::EmbodiedCarbon => {
                        self.render_embodied_carbon(ui, theme, members, features)
                    }
                    SustainabilitySubTab::CarbonAriTradeoff => {
                        self.render_carbon_ari_tradeoff(ui, theme)
                    }
                    SustainabilitySubTab::GreenCertification => {
                        self.render_green_certification(ui, theme)
                    }
                    SustainabilitySubTab::EnergyLifecycle => {
                        self.render_energy_lifecycle(ui, theme)
                    }
                    SustainabilitySubTab::SolarOttvDaylight => {
                        self.render_solar_ottv_daylight(ui, theme, features);
                    }
                    SustainabilitySubTab::CircularDfdPassport => {
                        self.render_circular_dfd_passport(ui, theme, features);
                    }
                }

                ui.add_space(24.0);
            });
        });
    }

    fn render_header_bar(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let tabs = [
            (SustainabilitySubTab::EmbodiedCarbon, "🌱 Embodied Carbon (CO₂e)"),
            (SustainabilitySubTab::CarbonAriTradeoff, "📈 Carbon vs ARI Trade-Off"),
            (SustainabilitySubTab::GreenCertification, "🏅 Green Certification (LEED/Greenship)"),
            (SustainabilitySubTab::EnergyLifecycle, "⚡ Thermal & Lifecycle Durability"),
            (SustainabilitySubTab::SolarOttvDaylight, "☀️ Solar OTTV & Daylight"),
            (SustainabilitySubTab::CircularDfdPassport, "♻️ Circular Passport (DfD)"),
        ];
        ui_kit::subtab_bar(ui, theme, "sust_subtab_bar", &mut self.active_subtab, &tabs);
    }

    fn render_embodied_carbon(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        _members: &[StructuralMember],
        features: &mut FeatureRegistry,
    ) {
        let mut engine = crate::core::ParametricEngine::new();
        let lca = engine.generate_warren_truss().ok().map(|m| {
            let route = match self.steel_production_route {
                SteelProductionRoute::ElectricArcFurnace => crate::core::CarbonSteelRoute::EafRecycled,
                SteelProductionRoute::BlastFurnaceBof => crate::core::CarbonSteelRoute::BfBofStandard,
                SteelProductionRoute::HydrogenDri => crate::core::CarbonSteelRoute::GreenHydrogenDri,
            };
            crate::core::CarbonEngine::calculate_lca(&m, route, Some(65.0), Some(0.942))
        });

        let (total_co2, total_steel_tons, reduction_pct, _tier_str, eco_eff) = match lca.as_ref() {
            Some(r) => (
                r.total_embodied_carbon_ton,
                r.steel_tonnage,
                r.carbon_reduction_pct,
                r.greenship_leed_tier.as_str(),
                r.eco_redundancy_efficiency,
            ),
            None => (48.5 * 1.15, 48.5, 35.0, "LEED Silver", 0.017),
        };

        let _factor = match self.steel_production_route {
            SteelProductionRoute::ElectricArcFurnace => 1.15,
            SteelProductionRoute::BlastFurnaceBof => 2.35,
            SteelProductionRoute::HydrogenDri => 0.45,
        };
        let _steel_co2 = total_steel_tons * _factor;
        let _concrete_co2 = 72.0 * 0.28;
        let _coating_co2 = 340.0 * 0.008;
        let _transport_co2 = total_steel_tons * 0.045;

        // Responsive Metric Cards Row
        let cards = [
            (
                "TOTAL EMBODIED CARBON",
                format!("{:.1} t CO₂e", total_co2),
                format!("Steel Mass: {:.1} Tons (A1–A5)", total_steel_tons),
                theme.accent_green,
            ),
            (
                "CARBON INTENSITY",
                format!("{:.1} kg/m²", (total_co2 * 1000.0) / 288.0),
                "24m Span x 12m Floor".to_string(),
                theme.accent_blue,
            ),
            (
                "EMISSIONS SAVINGS",
                format!("-{:.1}%", reduction_pct),
                "vs. Baseline BF-BOF".to_string(),
                if reduction_pct > 30.0 {
                    theme.accent_green
                } else {
                    theme.accent_orange
                },
            ),
            (
                "ECO-REDUNDANCY (η)",
                format!("{:.4}", eco_eff),
                "ARI / Total tCO₂e".to_string(),
                theme.accent_blue,
            ),
        ];
        ui_kit::responsive_grid_n(ui, cards.len(), 200.0, |ui, idx| {
            let (title, val, sub, col) = &cards[idx];
            self.render_stat_card(ui, theme, title, val, sub, *col);
        });

        ui.add_space(12.0);

        // Phase 32 Locked Slot: Live BIM Material Embodied Carbon Sync
        ui_kit::feature_slot_card(ui, theme, features, FeatureId::BimMaterialLcaAutoSync, |ui| {
            ui.horizontal_wrapped(|ui| {
                let _ = ui.selectable_label(true, "Source: Active BimProject Steel + Slab Volumes");
                let _ = ui.selectable_label(false, "EPD Factor Database (A1-A3)");
                let _ = ui.button("♻️ Recompute Live Model CO₂e Footprint");
            });
        });

        ui.add_space(14.0);

        // Production Route Selector
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(14))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new("🏭 Steel Production Route:")
                            .size(12.0)
                            .strong()
                            .color(theme.text_primary),
                    );
                    ui.selectable_value(
                        &mut self.steel_production_route,
                        SteelProductionRoute::ElectricArcFurnace,
                        "♻️ EAF Electric Arc (1.15 t CO₂e/t)",
                    );
                    ui.selectable_value(
                        &mut self.steel_production_route,
                        SteelProductionRoute::BlastFurnaceBof,
                        "⚙️ Conventional BF-BOF (2.35 t CO₂e/t)",
                    );
                    ui.selectable_value(
                        &mut self.steel_production_route,
                        SteelProductionRoute::HydrogenDri,
                        "🌱 Green Hydrogen DRI (0.45 t CO₂e/t)",
                    );
                });
            });

        ui.add_space(16.0);

        // ISO 14040 / EN 15978 Whole-Life Carbon Assessment (WBLCA) Stages
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(14))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new("🌍 WHOLE BUILDING LIFE CYCLE ASSESSMENT (EN 15978 / ISO 14040)")
                            .size(13.0)
                            .strong()
                            .color(theme.text_primary),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(
                            RichText::new("RIBA 2030 Climate Challenge Benchmark: Tier A (< 300 kg CO₂e/m²)")
                                .size(11.0)
                                .color(theme.accent_green),
                        );
                    });
                });
                ui.add_space(10.0);

                for stage in &self.lca_stages {
                    let col = Color32::from_rgb(stage.color_rgb.0, stage.color_rgb.1, stage.color_rgb.2);
                    ui.horizontal_wrapped(|ui| {
                        Frame::new()
                            .fill(col.gamma_multiply(0.2))
                            .stroke(Stroke::new(1.0, col))
                            .corner_radius(CornerRadius::same(4))
                            .inner_margin(Margin::symmetric(6, 2))
                            .show(ui, |ui| {
                                ui.label(RichText::new(&stage.stage_code).size(11.0).strong().color(col));
                            });

                        ui.add_space(8.0);
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(&stage.stage_name)
                                    .size(11.5)
                                    .strong()
                                    .color(theme.text_primary),
                            );
                            ui.label(
                                RichText::new(&stage.description)
                                    .size(10.5)
                                    .color(theme.text_secondary),
                            );
                        });

                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(
                                RichText::new(format!("{:.1}%", stage.pct_of_total))
                                    .size(11.5)
                                    .strong()
                                    .color(col),
                            );
                            ui.add_space(12.0);
                            let sign = if stage.emissions_t_co2e < 0.0 { "" } else { "+" };
                            ui.label(
                                RichText::new(format!("{}{:.1} t CO₂e", sign, stage.emissions_t_co2e))
                                    .size(12.0)
                                    .color(theme.text_primary),
                            );
                        });
                    });

                    ui.add_space(2.0);
                    let (b_rect, _resp) = ui.allocate_exact_size(
                        Vec2::new(ui.available_width(), 4.0),
                        egui::Sense::hover(),
                    );
                    let b_painter = ui.painter();
                    b_painter.rect_filled(b_rect, CornerRadius::same(2), theme.bg_canvas);
                    let fill_w =
                        b_rect.width() * (stage.pct_of_total.abs() / 100.0).clamp(0.02, 1.0);
                    b_painter.rect_filled(
                        egui::Rect::from_min_size(b_rect.left_top(), Vec2::new(fill_w, b_rect.height())),
                        CornerRadius::same(2),
                        col,
                    );

                    ui.add_space(6.0);
                }
            });
    }

    fn render_carbon_ari_tradeoff(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.label(
                    RichText::new(
                        "📈 Pareto Frontier: Structural Redundancy (ARI) vs. Embodied Carbon (CO₂e)",
                    )
                    .size(13.5)
                    .strong()
                    .color(theme.text_primary),
                );
                ui.add_space(6.0);
                ui.label(
                    RichText::new("Finding the optimum trade-off where progressive collapse resilience increases significantly without disproportionate carbon expenditure.")
                        .size(11.5)
                        .color(theme.text_secondary),
                );
                ui.add_space(14.0);

                let scenarios = [
                    ("Vulnerable Design (Non-Redundant)", "0.58", "38.2 Tons", "43.9 t CO₂e", "🔴 Catastrophic Collapse Risk (Reconstruction = +120 t CO₂e)", false),
                    ("Code-Baseline Standard Design", "0.72", "44.0 Tons", "50.6 t CO₂e", "🟡 Meets Minimum Code, Lacks Secondary Redundancy", false),
                    ("EigenGraph Optimal Design (Recommended)", "0.94", "48.5 Tons", "55.8 t CO₂e", "🟢 SWEET SPOT: ARI +30.5%, Carbon Only +5.2 t CO₂e (+10.2%)", true),
                    ("Over-Conservative Design", "0.98", "62.0 Tons", "71.3 t CO₂e", "⚪ Inefficient: +27.8% Carbon Surcharge for Negligible ARI Gain", false),
                ];

                for (name, ari, steel, co2, desc, is_opt) in scenarios {
                    let fill = if is_opt {
                        theme.accent_blue.gamma_multiply(0.12)
                    } else {
                        theme.bg_canvas
                    };
                    let stroke = if is_opt {
                        Stroke::new(1.5, theme.accent_blue)
                    } else {
                        Stroke::new(1.0, theme.border_subtle)
                    };

                    Frame::new()
                        .fill(fill)
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(Margin::same(12))
                        .stroke(stroke)
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.vertical(|ui| {
                                    ui.horizontal_wrapped(|ui| {
                                        ui.label(RichText::new(name).size(12.5).strong().color(if is_opt { theme.accent_blue } else { theme.text_primary }));
                                        if is_opt {
                                            ui.label(RichText::new("⭐ BEST ECO-EFFICIENCY").size(10.0).strong().color(theme.accent_green));
                                        }
                                    });
                                    ui.add_space(2.0);
                                    ui.label(RichText::new(desc).size(11.0).color(theme.text_secondary));
                                });

                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.vertical(|ui| {
                                        ui.label(RichText::new(format!("ARI: {}", ari)).size(13.0).strong().color(if is_opt { theme.accent_green } else { theme.text_primary }));
                                        ui.label(RichText::new(format!("{} • {}", steel, co2)).size(11.0).color(theme.text_muted));
                                    });
                                });
                            });
                        });
                    ui.add_space(8.0);
                }
            });
    }

    fn render_green_certification(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let cert_cards = [
            ("GREENSHIP (GBCI)", "PLATINUM", "86 / 100 Credit Points", theme.accent_green),
            ("LEED v4.1 BD+C", "PLATINUM", "84 / 110 Credit Points", theme.accent_blue),
            ("BREEAM INTERNATIONAL", "OUTSTANDING", "88.4% Rating Score", theme.accent_green),
        ];
        ui_kit::responsive_grid_n(ui, cert_cards.len(), 200.0, |ui, idx| {
            let (title, val, sub, col) = cert_cards[idx];
            self.render_stat_card(ui, theme, title, val, sub, col);
        });

        ui.add_space(14.0);

        // Standard Filter Pills
        theme.card_frame().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    RichText::new("🏅 RATING SYSTEM:")
                        .size(11.5)
                        .strong()
                        .color(theme.accent_blue),
                );

                let is_all = self.selected_credit_standard.is_none();
                let all_bg = if is_all { theme.accent_blue } else { theme.bg_canvas };
                let all_fg = if is_all { Color32::WHITE } else { theme.text_secondary };
                if ui
                    .add(
                        egui::Button::new(RichText::new("All Systems").size(11.0).color(all_fg))
                            .fill(all_bg)
                            .corner_radius(CornerRadius::same(6)),
                    )
                    .clicked()
                {
                    self.selected_credit_standard = None;
                }

                let standards = ["GREENSHIP (GBCI)", "LEED v4.1 BD+C", "BREEAM International"];
                for std in standards {
                    let is_sel = self.selected_credit_standard.as_deref() == Some(std);
                    let bg = if is_sel { theme.accent_blue } else { theme.bg_canvas };
                    let fg = if is_sel { Color32::WHITE } else { theme.text_secondary };
                    if ui
                        .add(
                            egui::Button::new(RichText::new(std).size(11.0).color(fg))
                                .fill(bg)
                                .corner_radius(CornerRadius::same(6)),
                        )
                        .clicked()
                    {
                        self.selected_credit_standard = Some(std.to_string());
                    }
                }
            });
        });

        ui.add_space(12.0);

        // Credit Checklist Table
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.label(
                    RichText::new("📋 Green Certification Verification Checklist")
                        .size(13.0)
                        .strong()
                        .color(theme.text_primary),
                );
                ui.add_space(10.0);

                for credit in &self.green_credits {
                    if let Some(ref sel_std) = self.selected_credit_standard {
                        if &credit.standard != sel_std {
                            continue;
                        }
                    }

                    ui.horizontal_wrapped(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(&credit.credit_title)
                                    .size(12.0)
                                    .strong()
                                    .color(theme.text_primary),
                            );
                            ui.label(
                                RichText::new(format!("{} • {}", credit.standard, credit.category))
                                    .size(10.5)
                                    .color(theme.text_muted),
                            );
                        });

                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let (badge_bg, badge_fg) = if credit.status == "Compliant" {
                                (theme.accent_green.gamma_multiply(0.2), theme.accent_green)
                            } else {
                                (theme.accent_orange.gamma_multiply(0.2), theme.accent_orange)
                            };

                            Frame::new()
                                .fill(badge_bg)
                                .corner_radius(CornerRadius::same(4))
                                .inner_margin(Margin::symmetric(6, 2))
                                .show(ui, |ui| {
                                    ui.label(
                                        RichText::new(&credit.status)
                                            .size(10.5)
                                            .strong()
                                            .color(badge_fg),
                                    );
                                });

                            ui.add_space(12.0);
                            ui.label(
                                RichText::new(format!(
                                    "{}/{} Pts",
                                    credit.points_achieved, credit.points_max
                                ))
                                .size(12.0)
                                .strong()
                                .color(theme.accent_blue),
                            );
                        });
                    });
                    ui.separator();
                }
            });
    }

    fn render_energy_lifecycle(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.label(
                    RichText::new("⚡ Thermal Efficiency & Life Cycle Durability Assessment")
                        .size(13.5)
                        .strong()
                        .color(theme.text_primary),
                );
                ui.add_space(10.0);

                let energy_cards = [
                    ("DESIGN SERVICE LIFE", "100 Years", "SNI Bridge Serviceability Class A", theme.text_primary),
                    ("COATING DURABILITY", "25+ Years", "C5-High Marine Grade Polyurethane", theme.accent_blue),
                    ("THERMAL EXPANSION", "±14.2 mm", "Elastomeric Bearing Expansion Allowance", theme.accent_green),
                ];
                ui_kit::responsive_grid_n(ui, energy_cards.len(), 200.0, |ui, idx| {
                    let (title, val, sub, col) = energy_cards[idx];
                    self.render_stat_card(ui, theme, title, val, sub, col);
                });
            });
    }

    fn render_solar_ottv_daylight(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(ui, theme, features, FeatureId::SolarOttvThermalDaylight, |ui| {
            let cards = [
                ("OVERALL OTTV (SNI 6389)", "31.4 W/m² (PASS)", "Max Allowed Cap = 35.0 W/m²", theme.accent_green),
                ("SOLAR HEAT GAIN (SHGC)", "0.32 Double Low-E", "WWR = 42% w/ External Louvers", theme.accent_blue),
                ("DAYLIGHT AUTONOMY (sDA)", "68.5% Floor Area", "Target >= 55% @ 300 lux Occupied", theme.accent_green),
                ("COOLING LOAD REDUCTION", "-184 kW Peak Chiller", "Annual HVAC Savings ≈ 240 MWh/yr", theme.accent_orange),
            ];
            ui_kit::responsive_grid_n(ui, cards.len(), 185.0, |ui, idx| {
                let (t, v, s, c) = cards[idx];
                self.render_stat_card(ui, theme, t, v, s, c);
            });

            ui.add_space(10.0);
            ui.horizontal_wrapped(|ui| {
                let _ = ui.selectable_label(true, "Climate Zone: Tropical Marine (-6.2° S)");
                let _ = ui.selectable_label(false, "Include 600mm Horizontal Sunshades");
                let _ = ui.button("☀️ Compute N/E/S/W Facade OTTV & Glare Audit");
            });
            ui.add_space(12.0);

            ui_kit::responsive_two_panes(ui, 640.0, |ui, pane| match pane {
                0 => {
                    ui.label(
                        RichText::new("FACADE ORIENTATION SOLAR FLUX COMPASS (W/m²)")
                            .size(11.0)
                            .strong()
                            .color(theme.accent_blue),
                    );
                    ui.add_space(6.0);
                    let desired = Vec2::new(ui.available_width(), 165.0);
                    let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
                    let painter = ui.painter_at(rect);
                    painter.rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(13, 17, 24));
                    let c = rect.center();
                    let r_max = (rect.width().min(rect.height()) * 0.36).max(28.0);
                    painter.circle_stroke(c, r_max, Stroke::new(1.0, theme.border_subtle));
                    painter.circle_stroke(c, r_max * 0.65, Stroke::new(0.8, theme.accent_green.gamma_multiply(0.5)));

                    let dirs = [
                        ("N: 24.8", 0.0_f32, -1.0_f32, 0.56_f32, theme.accent_green),
                        ("E: 33.9", 1.0, 0.0, 0.85, theme.accent_orange),
                        ("S: 27.1", 0.0, 1.0, 0.62, theme.accent_blue),
                        ("W: 34.6", -1.0, 0.0, 0.88, theme.accent_red),
                    ];
                    let poly: Vec<Pos2> = dirs
                        .iter()
                        .map(|(_, dx, dy, frac, _)| c + Vec2::new(*dx * r_max * *frac, *dy * r_max * *frac))
                        .collect();
                    for i in 0..poly.len() {
                        painter.line_segment(
                            [poly[i], poly[(i + 1) % poly.len()]],
                            Stroke::new(2.0, theme.accent_orange),
                        );
                    }
                    for (lbl, dx, dy, _, col) in dirs {
                        painter.text(
                            c + Vec2::new(dx * (r_max + 22.0), dy * (r_max + 12.0)),
                            Align2::CENTER_CENTER,
                            lbl,
                            egui::FontId::monospace(9.5),
                            col,
                        );
                    }
                }
                _ => {
                    ui.label(
                        RichText::new("FACADE ORIENTATION THERMAL BREAKDOWN")
                            .size(11.0)
                            .strong()
                            .color(theme.accent_blue),
                    );
                    ui.add_space(6.0);
                    ScrollArea::horizontal().id_salt("ottv_facade_scroll").show(ui, |ui| {
                        egui::Grid::new("ottv_facade_grid")
                            .striped(true)
                            .spacing(Vec2::new(12.0, 5.0))
                            .show(ui, |ui| {
                                for h in ["Orientation", "WWR", "SHGC", "OTTV (W/m²)", "SNI 6389"] {
                                    ui.label(RichText::new(h).size(10.5).strong().color(theme.accent_blue));
                                }
                                ui.end_row();
                                for (dir, wwr, shgc, ottv, st, col) in [
                                    ("North Facade", "38%", "0.34", "24.8", "PASS", theme.accent_green),
                                    ("East Facade", "44%", "0.32", "33.9", "PASS (Marginal)", theme.accent_orange),
                                    ("South Facade", "40%", "0.34", "27.1", "PASS", theme.accent_green),
                                    ("West Facade", "46%", "0.28 + Fin", "34.6", "PASS (<35.0)", theme.accent_orange),
                                ] {
                                    ui.label(RichText::new(dir).size(10.5).strong().color(theme.text_primary));
                                    ui.label(RichText::new(wwr).size(10.5).color(theme.text_secondary));
                                    ui.label(RichText::new(shgc).size(10.5).color(theme.text_secondary));
                                    ui.label(RichText::new(ottv).size(10.5).strong().color(col));
                                    ui.label(RichText::new(st).size(10.5).color(col));
                                    ui.end_row();
                                }
                            });
                    });
                }
            });
        });
    }

    fn render_circular_dfd_passport(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(ui, theme, features, FeatureId::CircularMaterialPassportDfd, |ui| {
            let cards = [
                ("REVERSIBLE BOLTED STEEL", "76.4% by Mass", "A325 Bolted End-Plates (Reuse Ready)", theme.accent_green),
                ("EN 15978 MODULE D OFFSET", "-14.8 t CO₂e", "Circular Salvage & Recycling Credit", theme.accent_blue),
                ("MATERIAL CIRCULARITY (MCI)", "0.68 Index", "Target >= 0.60 Urban Mining Tier", theme.accent_green),
                ("PASSPORT INVENTORY TAGS", "142 Tagged Batches", "Mill Heat # & EPD Serials Archived", theme.text_primary),
            ];
            ui_kit::responsive_grid_n(ui, cards.len(), 185.0, |ui, idx| {
                let (t, v, s, c) = cards[idx];
                self.render_stat_card(ui, theme, t, v, s, c);
            });

            ui.add_space(10.0);
            ui.horizontal_wrapped(|ui| {
                let _ = ui.selectable_label(true, "Include Bolted Deconstruction Pathways");
                let _ = ui.selectable_label(false, "EU Madaster / ISO 20887 Schema");
                let _ = ui.button("♻️ Export Circular Material Passport (.JSON / QR)");
            });
            ui.add_space(10.0);

            ScrollArea::horizontal().id_salt("dfd_passport_scroll").show(ui, |ui| {
                egui::Grid::new("dfd_passport_grid")
                    .striped(true)
                    .min_col_width(90.0)
                    .spacing(Vec2::new(14.0, 6.0))
                    .show(ui, |ui| {
                        for h in ["Batch ID", "Structural Material", "Mass (Ton)", "Joint Type", "Reuse Pathway", "Module D Credit"] {
                            ui.label(RichText::new(h).size(10.5).strong().color(theme.accent_blue));
                        }
                        ui.end_row();
                        for (id, mat, mass, jnt, path, cr) in [
                            ("MAT-ST-01", "W14x90 ASTM A992 (EAF)", "38.2 T", "Bolted Splice", "100% Direct Reuse", "-11.4 t CO₂e"),
                            ("MAT-ST-02", "HSS 8x8x1/2 Diagonals", "10.3 T", "Pinned Gusset", "92% Direct Reuse", "-2.9 t CO₂e"),
                            ("MAT-RC-01", "Pile Cap Concrete K-350", "168.0 T", "Cast In-Situ", "Crushed Aggregate", "-0.5 t CO₂e"),
                        ] {
                            ui.label(RichText::new(id).size(10.5).monospace().color(theme.accent_blue));
                            ui.label(RichText::new(mat).size(10.5).strong().color(theme.text_primary));
                            ui.label(RichText::new(mass).size(10.5).color(theme.text_secondary));
                            ui.label(RichText::new(jnt).size(10.5).color(theme.accent_green));
                            ui.label(RichText::new(path).size(10.5).color(theme.text_secondary));
                            ui.label(RichText::new(cr).size(10.5).strong().color(theme.accent_green));
                            ui.end_row();
                        }
                    });
            });
        });
    }

    fn render_stat_card(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        title: &str,
        val: &str,
        subtitle: &str,
        val_color: Color32,
    ) {
        ui_kit::stat_card(ui, theme, title, val, subtitle, val_color);
    }
}
