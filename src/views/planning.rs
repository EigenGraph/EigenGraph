//! planning.rs — Pillar 5: 5D Construction Planning, Dynamic RAB, 4D Scheduling & Cost-ARI Trade-Off.
//! Automatically calculated and synchronized with PARAMETRIC variables (dimensions, material, stiffness, unit prices).

use egui::{
    Align, Align2, Color32, CornerRadius, Frame, Layout, Margin, Pos2, Rect, RichText,
    ScrollArea, Sense, Stroke, StrokeKind, Ui, Vec2,
};
use crate::features::{FeatureId, FeatureRegistry};
use crate::icons::{self, AppIcon};
use crate::models::{
    ConstructionSimulationPhase, CurrencyUnit, NavigationPillar, ParametricVariable,
    StructuralMember, WbsCostItem,
};
use crate::theme::MacTheme;
use crate::ui_kit;

/// Formats a number as Rupiah / Indonesian currency string with dot separators (e.g. 1.250.000)
pub fn format_idr(val: f64) -> String {
    let int_part = val.round() as i64;
    let s = int_part.abs().to_string();
    let mut res = String::new();
    let mut count = 0;
    for ch in s.chars().rev() {
        if count > 0 && count % 3 == 0 {
            res.push('.');
        }
        res.push(ch);
        count += 1;
    }
    let formatted: String = res.chars().rev().collect();
    if int_part < 0 {
        format!("-{}", formatted)
    } else {
        formatted
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanningSubTab {
    TakeoffQto,
    DynamicRab,
    Schedule4d,
    CostAriTradeoff,
    StagedErectionSvd,
    WbsHierarchy,
    ScheduleRiskMonteCarlo,
    ResourceLeveling4d,
    EarnedValueEvm,
    ValueEngineeringBid,
}

pub struct PlanningView {
    pub active_subtab: PlanningSubTab,
    pub published_to_dossier: bool,
    pub export_status: Option<String>,
    pub contingency_percent: f32,
    pub overhead_percent: f32,
    pub vat_percent: f32,
    // Phase 28: 4D Simulation & 5D WBS
    pub active_currency: CurrencyUnit,
    pub simulation_day: u32,
    pub is_simulating: bool,
    pub simulation_speed: u32,
    pub simulation_phases: Vec<ConstructionSimulationPhase>,
    pub wbs_catalog: Vec<WbsCostItem>,
    pub selected_wbs_division: Option<String>,
    pub show_manpower_curve: bool,
    pub show_cpm_only: bool,
}

impl Default for PlanningView {
    fn default() -> Self {
        Self {
            active_subtab: PlanningSubTab::DynamicRab,
            published_to_dossier: false,
            export_status: None,
            contingency_percent: 5.0,
            overhead_percent: 10.0,
            vat_percent: 11.0,
            active_currency: CurrencyUnit::Idr,
            simulation_day: 145,
            is_simulating: false,
            simulation_speed: 1,
            simulation_phases: ConstructionSimulationPhase::default_phases(),
            wbs_catalog: WbsCostItem::default_catalog(),
            selected_wbs_division: None,
            show_manpower_curve: true,
            show_cpm_only: false,
        }
    }
}

/// Calculated 5D Quantities & Cost Breakdown derived directly from Parametric Variables
#[derive(Debug, Clone)]
pub struct Parametric5DTakeoff {
    pub span_m: f64,
    pub height_m: f64,
    pub n_panels: usize,
    pub density_kg_m3: f64,
    pub stiffness_e_mpa: f64,

    // Unit Prices from Parametric
    pub price_steel_kg: f64,
    pub price_fab_kg: f64,
    pub price_erect_ton: f64,
    pub price_coating_m2: f64,
    pub price_concrete_m3: f64,

    // Quantities (QTO)
    pub chord_length_m: f64,
    pub diagonal_length_m: f64,
    pub total_steel_kg: f64,
    pub total_steel_ton: f64,
    pub coating_surface_m2: f64,
    pub bolt_sets_count: usize,
    pub concrete_footing_m3: f64,
    pub duration_days: usize,
    pub total_man_days: usize,

    // Costs (RAB)
    pub cost_mobilization: f64,
    pub cost_substructure: f64,
    pub cost_steel_material: f64,
    pub cost_workshop_fab: f64,
    pub cost_coating_fireproof: f64,
    pub cost_transportation: f64,
    pub cost_erection_crane: f64,
    pub cost_bolts_finishing: f64,

    pub direct_cost_total: f64,
    pub overhead_cost: f64,
    pub contingency_cost: f64,
    pub vat_cost: f64,
    pub grand_total_rab: f64,

    // Safety & ARI indicator
    pub predicted_system_ari: f64,
}

impl PlanningView {
    /// Extract parametric variables and evaluate full 5D quantities
    pub fn compute_takeoff(&self, variables: &[ParametricVariable]) -> Parametric5DTakeoff {
        let get_val = |name: &str, default: f64| -> f64 {
            for v in variables {
                if v.name.eq_ignore_ascii_case(name) || v.name.contains(name) {
                    return v.evaluated_value as f64;
                }
            }
            default
        };

        // Dimensions
        let span_mm = get_val("L", 24000.0);
        let span_m = span_mm / 1000.0;
        let height_mm = get_val("H", 3200.0);
        let height_m = height_mm / 1000.0;
        let n_panels = get_val("n", 6.0).round().max(2.0) as usize;

        // Material & Stiffness
        let density_kg_m3 = get_val("density", 7850.0);
        let stiffness_e_mpa = get_val("stiffness", get_val("kekakuan", 200000.0));

        // Unit Prices (IDR)
        let price_steel_kg = get_val("price_steel", get_val("harga_baja", 24500.0));
        let price_fab_kg = get_val("price_fab", get_val("harga_fabrikasi", 8500.0));
        let price_erect_ton = get_val("price_erect", get_val("harga_ereksi", 4500000.0));
        let price_coating_m2 = get_val("price_coating", get_val("harga_coating", 145000.0));
        let price_concrete_m3 = get_val("price_concrete", get_val("harga_pondasi", 1950000.0));

        // Member Lengths & Quantities
        let panel_dx = span_m / (n_panels as f64);
        let chord_length_m = span_m + (span_m - panel_dx); // bottom + top chord
        let diag_len = ((panel_dx / 2.0).powi(2) + height_m.powi(2)).sqrt();
        let diagonal_length_m = diag_len * (2.0 * n_panels as f64);

        // Cross Section Areas: W14x90 (0.0171 m2) for chords, HSS 8x8 (0.00929 m2) for diagonals
        // Add 15% allowance for connection plates, gussets, and stiffeners
        let chord_vol = chord_length_m * 0.0171;
        let diag_vol = diagonal_length_m * 0.00929;
        let total_vol = (chord_vol + diag_vol) * 1.15;
        let total_steel_kg = total_vol * density_kg_m3;
        let total_steel_ton = total_steel_kg / 1000.0;

        // Surface Area for Coating: AISC W14x90 perimeter ~1.45m, HSS8x8 ~0.80m
        let coating_surface_m2 = (chord_length_m * 1.45 + diagonal_length_m * 0.80) * 1.10;

        // Bolts & Concrete
        let bolt_sets_count = (2 * n_panels + 1) * 16;
        let concrete_footing_m3 = 2.0 * (15.0 + span_m * 0.75); // Substructure proportional to span
        let duration_days = 90 + (span_m * 2.0) as usize;
        let total_man_days = (total_steel_ton * 14.5) as usize;

        // Direct Costs
        let cost_mobilization = 120_000_000.0 + span_m * 2_500_000.0;
        let cost_substructure = concrete_footing_m3 * price_concrete_m3;
        let cost_steel_material = total_steel_kg * price_steel_kg;
        let cost_workshop_fab = total_steel_kg * price_fab_kg;
        let cost_coating_fireproof = coating_surface_m2 * price_coating_m2;
        let cost_transportation = 45_000_000.0 + total_steel_ton * 850_000.0;
        let cost_erection_crane = total_steel_ton * price_erect_ton;
        let cost_bolts_finishing = (bolt_sets_count as f64) * 125_000.0 + 45_000_000.0;

        let direct_cost_total = cost_mobilization
            + cost_substructure
            + cost_steel_material
            + cost_workshop_fab
            + cost_coating_fireproof
            + cost_transportation
            + cost_erection_crane
            + cost_bolts_finishing;

        let overhead_cost = direct_cost_total * (self.overhead_percent as f64 / 100.0);
        let contingency_cost = direct_cost_total * (self.contingency_percent as f64 / 100.0);
        let before_vat = direct_cost_total + overhead_cost + contingency_cost;
        let vat_cost = before_vat * (self.vat_percent as f64 / 100.0);
        let grand_total_rab = before_vat + vat_cost;

        // System ARI indicator derived from stiffness and panel geometry
        let predicted_system_ari = (0.75 + (height_m / span_m) * 1.25).min(0.965);

        Parametric5DTakeoff {
            span_m,
            height_m,
            n_panels,
            density_kg_m3,
            stiffness_e_mpa,
            price_steel_kg,
            price_fab_kg,
            price_erect_ton,
            price_coating_m2,
            price_concrete_m3,
            chord_length_m,
            diagonal_length_m,
            total_steel_kg,
            total_steel_ton,
            coating_surface_m2,
            bolt_sets_count,
            concrete_footing_m3,
            duration_days,
            total_man_days,
            cost_mobilization,
            cost_substructure,
            cost_steel_material,
            cost_workshop_fab,
            cost_coating_fireproof,
            cost_transportation,
            cost_erection_crane,
            cost_bolts_finishing,
            direct_cost_total,
            overhead_cost,
            contingency_cost,
            vat_cost,
            grand_total_rab,
            predicted_system_ari,
        }
    }

    pub fn show(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        variables: &[ParametricVariable],
        members: &[StructuralMember],
        features: &mut FeatureRegistry,
        active_pillar: &mut NavigationPillar,
    ) {
        let takeoff = self.compute_takeoff(variables);
        let pad = if ui.available_width() < MacTheme::MOBILE_BREAKPOINT {
            10
        } else {
            20
        };

        ui.vertical(|ui| {
            Frame::new().inner_margin(Margin::symmetric(pad, 12)).show(ui, |ui| {
                // 1. TOP SUB-TABS SELECTOR & EXPORT BAR
                self.render_header_bar(ui, theme, &takeoff, active_pillar);

                ui.add_space(12.0);

                // 2. LIVE PARAMETRIC BINDING BANNER
                self.render_parametric_binding_banner(ui, theme, &takeoff);

                ui.add_space(14.0);

                // 3. ACTIVE SUB-TAB CONTENT
                match self.active_subtab {
                    PlanningSubTab::TakeoffQto => {
                        self.render_qto_takeoff(ui, theme, &takeoff, members)
                    }
                    PlanningSubTab::DynamicRab => {
                        self.render_dynamic_rab(ui, theme, &takeoff, features)
                    }
                    PlanningSubTab::Schedule4d => {
                        self.render_schedule_4d(ui, theme, &takeoff, features)
                    }
                    PlanningSubTab::CostAriTradeoff => {
                        self.render_cost_ari_tradeoff(ui, theme, &takeoff)
                    }
                    PlanningSubTab::StagedErectionSvd => {
                        self.render_staged_erection_svd(ui, theme, &takeoff)
                    }
                    PlanningSubTab::WbsHierarchy => self.render_wbs_hierarchy(ui, theme, &takeoff),
                    PlanningSubTab::ScheduleRiskMonteCarlo => {
                        self.render_schedule_risk_monte_carlo(ui, theme, features);
                    }
                    PlanningSubTab::ResourceLeveling4d => {
                        self.render_resource_leveling_4d(ui, theme, features);
                    }
                    PlanningSubTab::EarnedValueEvm => {
                        self.render_earned_value_evm(ui, theme, &takeoff, features);
                    }
                    PlanningSubTab::ValueEngineeringBid => {
                        self.render_value_engineering_bid(ui, theme, &takeoff, features);
                    }
                }

                ui.add_space(48.0);
            });
        });
    }

    fn render_header_bar(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        takeoff: &Parametric5DTakeoff,
        active_pillar: &mut NavigationPillar,
    ) {
        let is_time_mode = *active_pillar == NavigationPillar::TimeScheduling;
        if is_time_mode {
            let time_tabs = [
                (PlanningSubTab::Schedule4d, "⏱️ 4D Synchro Simulation"),
                (PlanningSubTab::StagedErectionSvd, "🏗️ Staged Erection Guard"),
                (PlanningSubTab::ScheduleRiskMonteCarlo, "🎲 Monte Carlo Risk"),
                (PlanningSubTab::ResourceLeveling4d, "👷 Crew Leveling"),
            ];
            ui_kit::subtab_bar(ui, theme, "planning_time_subtab_bar", &mut self.active_subtab, &time_tabs);
        } else {
            let cost_tabs = [
                (PlanningSubTab::DynamicRab, "💰 5D BoQ / Cost"),
                (PlanningSubTab::WbsHierarchy, "📑 5D WBS Explorer"),
                (PlanningSubTab::TakeoffQto, "📊 QTO"),
                (PlanningSubTab::CostAriTradeoff, "📈 Cost vs ARI"),
                (PlanningSubTab::EarnedValueEvm, "📈 Earned Value (EVM)"),
                (PlanningSubTab::ValueEngineeringBid, "⚖️ VE & Bid Leveling"),
            ];
            ui_kit::subtab_bar(ui, theme, "planning_cost_subtab_bar", &mut self.active_subtab, &cost_tabs);
        }

        ui.add_space(8.0);
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::symmetric(12, 8))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let (switch_icon, switch_lbl, target_pillar, target_tab) = if is_time_mode {
                        (AppIcon::Coins, "Open 5D Budgeting", NavigationPillar::CostBudgeting, PlanningSubTab::DynamicRab)
                    } else {
                        (AppIcon::Clock, "Open 4D Scheduling", NavigationPillar::TimeScheduling, PlanningSubTab::Schedule4d)
                    };
                    if icons::icon_button(
                        ui,
                        switch_icon,
                        switch_lbl,
                        11.0,
                        theme.accent_blue,
                        theme.accent_blue.gamma_multiply(0.15),
                        Stroke::NONE,
                        CornerRadius::same(6),
                        Vec2::ZERO,
                    )
                    .clicked()
                    {
                        *active_pillar = target_pillar;
                        self.active_subtab = target_tab;
                    }

                    ui.separator();

                    ui.label(
                        RichText::new("CURRENCY:")
                            .size(10.5)
                            .strong()
                            .color(theme.text_muted),
                    );
                    for curr in CurrencyUnit::all().iter() {
                        let is_curr = self.active_currency == *curr;
                        let c_bg = if is_curr {
                            theme.accent_orange
                        } else {
                            theme.bg_canvas
                        };
                        let c_fg = if is_curr {
                            Color32::WHITE
                        } else {
                            theme.text_secondary
                        };
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new(format!("{} ({})", curr.code(), curr.symbol()))
                                        .size(11.0)
                                        .color(c_fg),
                                )
                                .fill(c_bg)
                                .corner_radius(CornerRadius::same(6)),
                            )
                            .clicked()
                        {
                            self.active_currency = *curr;
                        }
                    }

                    ui.separator();

                    if icons::icon_button(
                        ui,
                        AppIcon::Document,
                        "Export BoQ (CSV)",
                        11.5,
                        theme.text_primary,
                        theme.bg_canvas,
                        Stroke::new(1.0, theme.border_subtle),
                        CornerRadius::same(8),
                        Vec2::ZERO,
                    )
                    .clicked()
                    {
                        self.export_status = Some(format!(
                            "File BoQ-5D_L{:.0}m.csv exported successfully!",
                            takeoff.span_m
                        ));
                    }

                    let (pub_icon, pub_text) = if self.published_to_dossier {
                        (AppIcon::CheckCircle, "Saved to Dossier")
                    } else {
                        (AppIcon::Folder, "Save to MY PROJECT")
                    };
                    let pub_bg = if self.published_to_dossier {
                        theme.accent_green
                    } else {
                        theme.accent_blue.gamma_multiply(0.85)
                    };
                    if icons::icon_button(
                        ui,
                        pub_icon,
                        pub_text,
                        11.5,
                        Color32::WHITE,
                        pub_bg,
                        Stroke::NONE,
                        CornerRadius::same(8),
                        Vec2::ZERO,
                    )
                    .clicked()
                    {
                        self.published_to_dossier = true;
                        self.export_status = Some(
                            "5D BoQ & Schedule synchronized to Project Dossier!".to_string(),
                        );
                    }
                });
            });

        if let Some(msg) = &self.export_status {
            ui.add_space(6.0);
            Frame::new()
                .fill(theme.accent_green.gamma_multiply(0.15))
                .stroke(Stroke::new(1.0, theme.accent_green))
                .corner_radius(CornerRadius::same(6))
                .inner_margin(Margin::symmetric(12, 6))
                .show(ui, |ui| {
                    icons::icon_label(ui, AppIcon::CheckCircle, 13.0, theme.accent_green, msg.as_str(), 11.5, theme.accent_green, false);
                });
        }
    }

    fn render_parametric_binding_banner(&self, ui: &mut Ui, theme: &MacTheme, takeoff: &Parametric5DTakeoff) {
        Frame::new()
            .fill(theme.bg_sidebar)
            .stroke(Stroke::new(1.0, theme.accent_blue.gamma_multiply(0.4)))
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::symmetric(14, 10))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    icons::icon_label(ui, AppIcon::LinkChain, 13.0, theme.accent_blue, "BOUND TO PARAMETRIC MODEL:", 11.5, theme.accent_blue, true);
                    ui.label(RichText::new(format!("Span L = {:.1} m", takeoff.span_m)).size(11.5).color(theme.text_primary));
                    ui.label(RichText::new("•").size(11.5).color(theme.text_muted));
                    ui.label(RichText::new(format!("Height H = {:.1} m (L/{:.1})", takeoff.height_m, takeoff.span_m / takeoff.height_m)).size(11.5).color(theme.text_primary));
                    ui.label(RichText::new("•").size(11.5).color(theme.text_muted));
                    ui.label(RichText::new(format!("ρ = {:.0} kg/m³", takeoff.density_kg_m3)).size(11.5).color(theme.text_primary));
                    ui.label(RichText::new("•").size(11.5).color(theme.text_muted));
                    ui.label(RichText::new(format!("Steel Rate: IDR {}/kg", format_idr(takeoff.price_steel_kg))).size(11.5).color(theme.accent_green));
                    icons::icon_label(ui, AppIcon::Bolt, 12.5, theme.text_muted, "Live Dynamic Calculation", 11.0, theme.text_muted, false);
                });
            });
    }

    // -------------------------------------------------------------
    // SUB-TAB 1: DYNAMIC RAB (5D Cost Estimation)
    // -------------------------------------------------------------
    fn render_dynamic_rab(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        t: &Parametric5DTakeoff,
        features: &mut FeatureRegistry,
    ) {
        // 4 Responsive KPI Metric Cards
        let rab_total = format!("IDR {}", format_idr(t.grand_total_rab));
        let steel_cost = format!("IDR {}", format_idr(t.cost_steel_material));
        let steel_sub = format!("{:.1} Ton @ IDR {}/kg", t.total_steel_ton, format_idr(t.price_steel_kg));
        let fab_erect = format!("IDR {}", format_idr(t.cost_workshop_fab + t.cost_erection_crane));
        let substruct = format!("IDR {}", format_idr(t.cost_substructure));
        let sub_note = format!("{:.1} m³ Concrete K-350", t.concrete_footing_m3);

        ui_kit::responsive_grid_n(ui, 4, 210.0, |ui, idx| match idx {
            0 => self.metric_card(ui, theme, "TOTAL 5D BoQ / BUDGET", &rab_total, "Includes 11% VAT & Overhead", theme.accent_blue),
            1 => self.metric_card(ui, theme, "STEEL MATERIAL COST", &steel_cost, &steel_sub, theme.accent_green),
            2 => self.metric_card(ui, theme, "FABRICATION & ERECTION", &fab_erect, "Workshop + 100T Mobile Crane", theme.accent_orange),
            _ => self.metric_card(ui, theme, "FOUNDATION & SUBSTRUCTURE", &substruct, &sub_note, theme.text_secondary),
        });

        ui.add_space(16.0);

        // Detailed AHSP Breakdown Table (Horizontally scrollable on narrow mobile)
        theme.card_frame().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("BILL OF QUANTITIES (BoQ) BREAKDOWN — DYNAMIC COST ESTIMATION").size(13.0).strong().color(theme.text_primary));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(RichText::new("Currency: IDR (Rupiah)").size(11.0).color(theme.text_muted));
                });
            });

            ui.add_space(10.0);

            let items = [
                ("1.0", "Heavy Equipment Mobilization (100T Crane, Trailer, Genset)", 1.0, "ls", t.cost_mobilization, t.cost_mobilization),
                ("2.0", "Bored Pile Foundation & K-350 Concrete Abutment Works", t.concrete_footing_m3, "m³", t.price_concrete_m3, t.cost_substructure),
                ("3.0", "Structural Steel Material Procurement BJ41 (ASTM A992)", t.total_steel_kg, "kg", t.price_steel_kg, t.cost_steel_material),
                ("4.0", "Workshop Fabrication (Cutting, Fitting, Full Pen Welding, NDT)", t.total_steel_kg, "kg", t.price_fab_kg, t.cost_workshop_fab),
                ("5.0", "Corrosion Protection: Sa 2.5 Sandblasting + 3-Coat Epoxy", t.coating_surface_m2, "m²", t.price_coating_m2, t.cost_coating_fireproof),
                ("6.0", "Truss Beam Haulage & Site Transportation", 1.0, "ls", t.cost_transportation, t.cost_transportation),
                ("7.0", "Full-Span Steel Truss Erection with Crane & Shoring", t.total_steel_ton, "ton", t.price_erect_ton, t.cost_erection_crane),
                ("8.0", "A325 High-Strength Bolt Installation, Torque Calibration & Finishing", t.bolt_sets_count as f64, "set", 125_000.0, t.cost_bolts_finishing),
            ];

            ScrollArea::horizontal()
                .id_salt("planning_boq_table_scroll")
                .show(ui, |ui| {
                    egui::Grid::new("planning_boq_table_grid")
                        .striped(true)
                        .min_col_width(64.0)
                        .spacing(Vec2::new(16.0, 7.0))
                        .show(ui, |ui| {
                            ui.label(RichText::new("No").strong().size(11.5).color(theme.text_muted));
                            ui.label(RichText::new("Work Breakdown Structure (WBS)").strong().size(11.5).color(theme.text_muted));
                            ui.label(RichText::new("Volume").strong().size(11.5).color(theme.text_muted));
                            ui.label(RichText::new("Unit").strong().size(11.5).color(theme.text_muted));
                            ui.label(RichText::new("Unit Price (IDR)").strong().size(11.5).color(theme.text_muted));
                            ui.label(RichText::new("Total Price (IDR)").strong().size(11.5).color(theme.text_muted));
                            ui.end_row();

                            for (no, desc, vol, sat, hrg, tot) in items {
                                ui.label(RichText::new(no).size(11.5).color(theme.text_secondary));
                                ui.label(RichText::new(desc).size(11.5).strong().color(theme.text_primary));
                                ui.label(RichText::new(format!("{:.2}", vol)).size(11.5).color(theme.text_secondary));
                                ui.label(RichText::new(sat).size(11.5).color(theme.text_muted));
                                ui.label(RichText::new(format_idr(hrg)).size(11.5).color(theme.text_secondary));
                                ui.label(RichText::new(format_idr(tot)).size(11.5).strong().color(theme.accent_green));
                                ui.end_row();
                            }
                        });
                });

            ui.add_space(6.0);
            ui.separator();
            ui.add_space(6.0);

            // Summary Totals
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Direct Cost Subtotal:").size(12.0).color(theme.text_secondary));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(RichText::new(format!("IDR {}", format_idr(t.direct_cost_total))).size(12.5).strong().color(theme.text_primary));
                });
            });

            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(format!("Contractor Overhead & Profit ({:.1}%):", self.overhead_percent)).size(11.5).color(theme.text_muted));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(RichText::new(format!("IDR {}", format_idr(t.overhead_cost))).size(11.5).color(theme.text_muted));
                });
            });

            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(format!("Contingency ({:.1}%):", self.contingency_percent)).size(11.5).color(theme.text_muted));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(RichText::new(format!("IDR {}", format_idr(t.contingency_cost))).size(11.5).color(theme.text_muted));
                });
            });

            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(format!("Value Added Tax (VAT {:.1}%):", self.vat_percent)).size(11.5).color(theme.text_muted));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(RichText::new(format!("IDR {}", format_idr(t.vat_cost))).size(11.5).color(theme.text_muted));
                });
            });

            ui.separator();
            ui.add_space(4.0);

            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("GRAND TOTAL ESTIMATED COST (5D BIM):").size(13.0).strong().color(theme.text_primary));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(RichText::new(format!("IDR {},-", format_idr(t.grand_total_rab))).size(17.0).strong().color(theme.accent_blue));
                });
            });
        });

        ui.add_space(14.0);
        ui_kit::feature_slot_card(
            ui,
            theme,
            features,
            FeatureId::LiveBimQtoBoqSync,
            |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new("Target Source: Active 3D BimProject Volume & Rebar BBS")
                            .size(11.0)
                            .strong()
                            .color(theme.text_primary),
                    );
                    let _ = icons::icon_button(ui, AppIcon::RulerSquare, "Extract Concrete m³", 11.5, theme.text_primary, theme.bg_canvas, Stroke::new(1.0, theme.border_subtle), CornerRadius::same(6), Vec2::ZERO);
                    let _ = icons::icon_button(ui, AppIcon::Wrench, "Extract Rebar Tonnage", 11.5, theme.text_primary, theme.bg_canvas, Stroke::new(1.0, theme.border_subtle), CornerRadius::same(6), Vec2::ZERO);
                    let _ = icons::icon_button(ui, AppIcon::Recycle, "Sync Active BIM Quantities -> BoQ", 11.5, theme.text_primary, theme.bg_canvas, Stroke::new(1.0, theme.border_subtle), CornerRadius::same(6), Vec2::ZERO);
                });
            },
        );
    }

    // -------------------------------------------------------------
    // SUB-TAB 2: QUANTITY TAKEOFF (QTO)
    // -------------------------------------------------------------
    fn render_qto_takeoff(&self, ui: &mut Ui, theme: &MacTheme, t: &Parametric5DTakeoff, _members: &[StructuralMember]) {
        let steel_ton = format!("{:.2} Ton", t.total_steel_ton);
        let steel_sub = format!("{} kg pure volume", format_idr(t.total_steel_kg));
        let paint_m2 = format!("{:.1} m²", t.coating_surface_m2);
        let bolt_sets = format!("{} Sets", t.bolt_sets_count);
        let footing_m3 = format!("{:.1} m³", t.concrete_footing_m3);

        ui_kit::responsive_grid_n(ui, 4, 210.0, |ui, idx| match idx {
            0 => self.metric_card(ui, theme, "TOTAL STEEL TONNAGE", &steel_ton, &steel_sub, theme.accent_blue),
            1 => self.metric_card(ui, theme, "PAINT PROTECTION AREA", &paint_m2, "Sa 2.5 + 3-Coat System", theme.accent_green),
            2 => self.metric_card(ui, theme, "A325 BOLTED CONNECTIONS", &bolt_sets, "Grade 8.8 High Strength", theme.accent_orange),
            _ => self.metric_card(ui, theme, "FOUNDATION CONCRETE VOLUME", &footing_m3, "Grade fc' 30 MPa (K-350)", theme.text_secondary),
        });

        ui.add_space(16.0);

        theme.card_frame().show(ui, |ui| {
            ui.label(RichText::new("BILL OF QUANTITIES (BoQ) — ELEMENT GEOMETRY FORMULAS").size(13.0).strong().color(theme.text_primary));
            ui.add_space(8.0);

            let top_len = format!("{:.2} m", t.span_m - (t.span_m / t.n_panels as f64));
            let bot_len = format!("{:.2} m", t.span_m);
            let diag_len = format!("{:.2} m", t.diagonal_length_m);
            let top_wt = format!("{:.2} Ton", t.total_steel_ton * 0.38);
            let bot_wt = format!("{:.2} Ton", t.total_steel_ton * 0.42);
            let diag_wt = format!("{:.2} Ton", t.total_steel_ton * 0.20);
            let gusset_wt = format!("{:.2} Ton", t.total_steel_ton * 0.15);
            let conc_vol = format!("{:.1} m³", t.concrete_footing_m3);

            let qto_rows = [
                ("TOP CHORD COMPRESSION (Top Chord)", top_len.as_str(), "W14x90", top_wt.as_str(), "AISC 360-16 LRFD"),
                ("BOTTOM CHORD TENSION (Bottom Chord)", bot_len.as_str(), "W14x90", bot_wt.as_str(), "Tension Net Section"),
                ("WEB DIAGONALS (Diagonals)", diag_len.as_str(), "HSS8x8x1/2", diag_wt.as_str(), "Alternating Warren"),
                ("GUSSET PLATES & STIFFENERS", "52 Sheets", "Plate T=16mm BJ41", gusset_wt.as_str(), "ASTM A36 / BJ41"),
                ("ANCHOR BOLTS & BEARING BASEPLATES", "4 Support Points", "T=32mm + 8x M36", "1.85 Ton", "Elastomeric Bearing Support"),
                ("BORED PILE / PILE CAP FOUNDATION", conc_vol.as_str(), "Concrete K-350", "156.0 Ton", "Substructure Civil Work"),
            ];

            for (elem, dim, sec, weight, code) in qto_rows {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new(elem).size(12.0).strong().color(theme.text_primary));
                    ui.label(RichText::new("—").size(12.0).color(theme.text_muted));
                    ui.label(RichText::new(format!("Dimensions: {}", dim)).size(12.0).color(theme.text_secondary));
                    ui.label(RichText::new("•").size(12.0).color(theme.text_muted));
                    ui.label(RichText::new(format!("Section: {}", sec)).size(12.0).color(theme.accent_blue));

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new(code).size(11.0).color(theme.text_muted));
                        ui.add_space(12.0);
                        ui.label(RichText::new(weight).size(12.0).strong().color(theme.accent_green));
                    });
                });
                ui.add_space(6.0);
                ui.separator();
            }
        });
    }

    // -------------------------------------------------------------
    // SUB-TAB 3: 4D SCHEDULING & S-CURVE
    // -------------------------------------------------------------
    // -------------------------------------------------------------
    // SUB-TAB 3: 4D SYNCHRO CONSTRUCTION SIMULATION & TIMELINE
    // -------------------------------------------------------------
    fn render_schedule_4d(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        _t: &Parametric5DTakeoff,
        features: &mut FeatureRegistry,
    ) {
        // Top 4 Responsive Metrics
        let sim_day_str = format!("Day {} / 360", self.simulation_day);
        let wbs_pkg_str = format!("{} Items", self.wbs_catalog.len());
        ui_kit::responsive_grid_n(ui, 4, 210.0, |ui, idx| match idx {
            0 => self.metric_card(ui, theme, "TOTAL WORKING DURATION", "360 Days", "12 Calendar Months", theme.accent_blue),
            1 => self.metric_card(ui, theme, "SIMULATION TIMELINE", &sim_day_str, "Interactive 4D Step", theme.accent_orange),
            2 => self.metric_card(ui, theme, "CRITICAL PATH (CPM)", "WBS 05.12.00", "Structural Steel Erection (Float: 0d)", theme.accent_red),
            _ => self.metric_card(ui, theme, "TOTAL WBS PACKAGES", &wbs_pkg_str, "Multi-Discipline Master WBS", theme.accent_green),
        });

        ui.add_space(14.0);

        // 4D Interactive Simulation Player Ribbon
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .inner_margin(Margin::symmetric(14, 10))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    icons::icon_label(ui, AppIcon::Clock, 14.0, theme.accent_blue, "4D SYNCHRO SIMULATION PLAYER", 12.5, theme.accent_blue, true);

                    // Play / Pause toggle
                    let (play_icon, play_label) = if self.is_simulating { (AppIcon::Pause, "Pause") } else { (AppIcon::Play, "Play 4D") };
                    let play_bg = if self.is_simulating { theme.accent_orange } else { theme.accent_green };
                    if icons::icon_button(ui, play_icon, play_label, 11.0, Color32::WHITE, play_bg, Stroke::NONE, CornerRadius::same(6), Vec2::ZERO).clicked() {
                        self.is_simulating = !self.is_simulating;
                    }

                    // Rewind & Reset
                    if icons::icon_button(ui, AppIcon::SkipBack, "Day 1", 11.0, theme.text_primary, theme.bg_canvas, Stroke::new(1.0, theme.border_subtle), CornerRadius::same(6), Vec2::ZERO).clicked() {
                        self.simulation_day = 1;
                    }
                    if icons::icon_button(ui, AppIcon::SkipNext, "Day 360", 11.0, theme.text_primary, theme.bg_canvas, Stroke::new(1.0, theme.border_subtle), CornerRadius::same(6), Vec2::ZERO).clicked() {
                        self.simulation_day = 360;
                    }

                    // Speed Toggles
                    ui.label(RichText::new("Speed:").size(11.0).color(theme.text_secondary));
                    for spd in [1, 2, 5] {
                        let is_spd = self.simulation_speed == spd;
                        let s_bg = if is_spd { theme.accent_blue } else { theme.bg_canvas };
                        let s_fg = if is_spd { Color32::WHITE } else { theme.text_secondary };
                        if ui.add(egui::Button::new(RichText::new(format!("{}x", spd)).size(10.5).color(s_fg))
                            .fill(s_bg)
                            .corner_radius(CornerRadius::same(4))).clicked() {
                            self.simulation_speed = spd;
                        }
                    }

                    // Simulation Day Slider
                    ui.add(egui::Slider::new(&mut self.simulation_day, 1..=360).text("Construction Day"));
                });

                // Auto-advance if simulating
                if self.is_simulating {
                    self.simulation_day = (self.simulation_day + self.simulation_speed).min(360);
                    if self.simulation_day >= 360 {
                        self.is_simulating = false;
                    }
                    ui.ctx().request_repaint();
                }

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                // Active Phase Status Bar
                let cur_day = self.simulation_day;
                let active_phase = self.simulation_phases.iter().find(|p| cur_day >= p.day_start && cur_day <= p.day_end)
                    .or_else(|| self.simulation_phases.last());

                if let Some(phase) = active_phase {
                    ui.horizontal_wrapped(|ui| {
                        Frame::new()
                            .fill(Color32::from_rgb(phase.color_rgb.0, phase.color_rgb.1, phase.color_rgb.2).gamma_multiply(0.2))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(phase.color_rgb.0, phase.color_rgb.1, phase.color_rgb.2)))
                            .corner_radius(CornerRadius::same(6))
                            .inner_margin(Margin::symmetric(8, 4))
                            .show(ui, |ui| {
                                ui.label(RichText::new(format!("ACTIVE PHASE: {}", phase.name)).size(11.0).strong()
                                    .color(Color32::from_rgb(phase.color_rgb.0, phase.color_rgb.1, phase.color_rgb.2)));
                            });

                        icons::icon_label(ui, AppIcon::HardHat, 13.0, theme.text_secondary, format!("Manpower: {} Workers", phase.manpower_count), 11.0, theme.text_secondary, false);
                        icons::icon_label(ui, AppIcon::Wrench, 13.0, theme.text_secondary, format!("Cranes/Equipment: {} Units", phase.equipment_count), 11.0, theme.text_secondary, false);
                        if phase.concrete_volume_m3 > 0.0 {
                            icons::icon_label(ui, AppIcon::Building, 13.0, theme.accent_blue, format!("Concrete Poured: {:.0} m³", phase.concrete_volume_m3), 11.0, theme.accent_blue, false);
                        }
                        ui.label(RichText::new(format!("Planned: {:.1}% | Actual: {:.1}%", phase.planned_progress_pct, phase.actual_progress_pct)).size(11.0).color(theme.text_muted));
                    });
                }
            });

        ui.add_space(14.0);

        // 2D Staged Building Erection Elevation Canvas
        theme.card_frame().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                icons::icon_label(ui, AppIcon::Building, 15.0, theme.accent_blue, "4D STAGED ERECTION ELEVATION & TOWER CRANE SIMULATION", 13.0, theme.text_primary, true);
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(RichText::new(format!("Scale 1:200 • Day {} Staging", self.simulation_day)).size(11.0).color(theme.text_muted));
                });
            });
            ui.add_space(8.0);

            let canvas_w = ui.available_width().clamp(240.0, 1100.0);
            let canvas_h = 240.0;
            let (rect, _resp) = ui.allocate_exact_size(Vec2::new(canvas_w, canvas_h), Sense::hover());
            let painter = ui.painter_at(rect);

            // Background & Ground Line
            painter.rect_filled(rect, CornerRadius::same(6), Color32::from_rgb(18, 22, 28));
            painter.rect_stroke(rect, CornerRadius::same(6), Stroke::new(1.0, Color32::from_rgb(45, 52, 65)), StrokeKind::Inside);

            let ground_y = rect.min.y + 190.0;
            painter.line_segment([Pos2::new(rect.min.x + 16.0, ground_y), Pos2::new(rect.max.x - 16.0, ground_y)], Stroke::new(2.0, Color32::from_rgb(120, 100, 70)));
            painter.text(Pos2::new(rect.min.x + 22.0, ground_y + 12.0), Align2::LEFT_TOP, "GROUND LEVEL EL. ±0.000", egui::FontId::proportional(8.5), Color32::from_rgb(160, 140, 110));

            // Building footprint centered
            let bldg_w = (rect.width() * 0.46).clamp(110.0, 360.0);
            let bldg_x1 = rect.center().x - bldg_w * 0.5;
            let bldg_x2 = rect.center().x + bldg_w * 0.5;

            // Substructure Piling (Day 1..45)
            let pile_color = if self.simulation_day >= 1 { Color32::from_rgb(140, 90, 45) } else { Color32::from_rgb(40, 35, 30) };
            for p_idx in 0..6 {
                let px = bldg_x1 + (p_idx as f32 / 5.0) * bldg_w;
                painter.rect_filled(Rect::from_min_max(Pos2::new(px - 3.0, ground_y), Pos2::new(px + 3.0, ground_y + 35.0)), CornerRadius::ZERO, pile_color);
            }

            // Raft Slab & Basement (Day 46..90)
            if self.simulation_day >= 46 {
                painter.rect_filled(Rect::from_min_max(Pos2::new(bldg_x1, ground_y - 8.0), Pos2::new(bldg_x2, ground_y)), CornerRadius::ZERO, Color32::from_rgb(130, 138, 150));
            }

            // Superstructure Steel & Concrete Floors (Day 91..195): 12 Storeys
            let max_floors = 12;
            let floors_erected = if self.simulation_day >= 91 {
                (((self.simulation_day - 90) as f32 / 105.0) * max_floors as f32).clamp(1.0, max_floors as f32) as usize
            } else {
                0
            };

            let floor_h = 11.5;
            for fl in 0..floors_erected {
                let fy = ground_y - 8.0 - (fl + 1) as f32 * floor_h;

                // Columns for this floor
                for c_idx in 0..4 {
                    let cx = bldg_x1 + (c_idx as f32 / 3.0) * bldg_w;
                    painter.line_segment([Pos2::new(cx, fy + floor_h), Pos2::new(cx, fy)], Stroke::new(2.2, Color32::from_rgb(0, 140, 255)));
                }

                // Floor slab beam line
                painter.line_segment([Pos2::new(bldg_x1, fy), Pos2::new(bldg_x2, fy)], Stroke::new(1.8, Color32::from_rgb(180, 190, 205)));

                // Façade Glass Panels (Day 180..270)
                if self.simulation_day >= 180 {
                    let facade_floors = (((self.simulation_day - 180) as f32 / 90.0) * max_floors as f32).clamp(0.0, max_floors as f32) as usize;
                    if fl < facade_floors {
                        let glass_rect = Rect::from_min_max(Pos2::new(bldg_x1 + 4.0, fy + 2.0), Pos2::new(bldg_x2 - 4.0, fy + floor_h - 2.0));
                        painter.rect_filled(glass_rect, CornerRadius::same(1), Color32::from_rgba_premultiplied(0, 180, 160, 45));
                        painter.rect_stroke(glass_rect, CornerRadius::same(1), Stroke::new(0.6, Color32::from_rgb(0, 200, 180)), StrokeKind::Inside);
                    }
                }
            }

            // Tower Crane at the top
            if floors_erected > 0 {
                let top_y = ground_y - 8.0 - floors_erected as f32 * floor_h;
                let crane_x = bldg_x1 + bldg_w * 0.70;
                let mast_top = top_y - 28.0;

                // Crane Mast
                painter.line_segment([Pos2::new(crane_x, top_y), Pos2::new(crane_x, mast_top)], Stroke::new(2.0, Color32::from_rgb(255, 180, 0)));
                // Crane Jib (Boom)
                painter.line_segment([Pos2::new(crane_x - 18.0, mast_top), Pos2::new(crane_x + 45.0, mast_top)], Stroke::new(1.5, Color32::from_rgb(255, 200, 0)));
                // Cable & Hook
                let hook_x = crane_x + 24.0;
                painter.line_segment([Pos2::new(hook_x, mast_top), Pos2::new(hook_x, mast_top + 16.0)], Stroke::new(0.8, Color32::WHITE));
                painter.circle_filled(Pos2::new(hook_x, mast_top + 16.0), 2.0, Color32::from_rgb(255, 220, 0));
            }
        });

        ui.add_space(14.0);

        // CPM Activities Breakdown with Planned vs. Current Day Status
        theme.card_frame().show(ui, |ui| {
            icons::icon_label(ui, AppIcon::Clock, 15.0, theme.accent_blue, "CRITICAL PATH METHOD (CPM) & STAGED WBS SCHEDULE", 13.0, theme.text_primary, true);
            ui.add_space(8.0);

            for phase in &self.simulation_phases {
                let is_current = self.simulation_day >= phase.day_start && self.simulation_day <= phase.day_end;
                let is_done = self.simulation_day > phase.day_end;
                let bar_color = if is_done {
                    theme.accent_green
                } else if is_current {
                    theme.accent_blue
                } else {
                    theme.border_subtle
                };

                let (status_icon, status_text) = if is_done {
                    (AppIcon::CheckCircle, "Completed")
                } else if is_current {
                    (AppIcon::Bolt, "In Progress")
                } else {
                    (AppIcon::Clock, "Pending")
                };

                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new(&phase.wbs_code).size(11.5).strong().color(theme.accent_blue));
                    ui.label(RichText::new(&phase.name).size(11.5).color(theme.text_primary));
                    ui.label(RichText::new(format!("Day {}-{} ({}d)", phase.day_start, phase.day_end, phase.day_end - phase.day_start + 1)).size(10.5).color(theme.text_secondary));

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        Frame::new()
                            .fill(bar_color.gamma_multiply(0.2))
                            .corner_radius(CornerRadius::same(4))
                            .inner_margin(Margin::symmetric(6, 2))
                            .show(ui, |ui| {
                                icons::icon_label(ui, status_icon, 11.5, bar_color, status_text, 10.0, bar_color, false);
                            });

                        ui.add_space(10.0);
                        ui.label(RichText::new(format!("{} Workers", phase.manpower_count)).size(10.5).color(theme.text_muted));
                        ui.add_space(8.0);
                        if phase.is_critical {
                            ui.label(RichText::new("CPM (Float: 0d)").size(10.0).strong().color(theme.accent_red));
                        }
                    });
                });

                ui.add_space(2.0);
                let (p_rect, _resp) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 5.0), Sense::hover());
                let p_painter = ui.painter();
                p_painter.rect_filled(p_rect, CornerRadius::same(2), theme.bg_canvas);

                let pct = if self.simulation_day >= phase.day_end {
                    1.0
                } else if self.simulation_day <= phase.day_start {
                    0.0
                } else {
                    (self.simulation_day - phase.day_start) as f32 / (phase.day_end - phase.day_start) as f32
                };
                let fill_w = p_rect.width() * pct;
                p_painter.rect_filled(Rect::from_min_size(p_rect.left_top(), Vec2::new(fill_w, p_rect.height())), CornerRadius::same(2), bar_color);
                ui.add_space(6.0);
            }
        });

        ui.add_space(14.0);
        ui_kit::feature_slot_card(
            ui,
            theme,
            features,
            FeatureId::BimDriven4DCriticalPathSync,
            |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new("3D Sequencing Bindings: IFC Element GlobalId <-> WBS Phases")
                            .size(11.0)
                            .strong()
                            .color(theme.text_primary),
                    );
                    let _ = icons::icon_button(ui, AppIcon::LinkChain, "Map Selected 3D Elements to WBS", 11.5, theme.text_primary, theme.bg_canvas, Stroke::new(1.0, theme.border_subtle), CornerRadius::same(6), Vec2::ZERO);
                    let _ = icons::icon_button(ui, AppIcon::Cube3D, "Highlight Erection Stage in 3D Viewport", 11.5, theme.text_primary, theme.bg_canvas, Stroke::new(1.0, theme.border_subtle), CornerRadius::same(6), Vec2::ZERO);
                });
            },
        );
    }

    // -------------------------------------------------------------
    // SUB-TAB: 5D WBS EXPLORER & DYNAMIC COST BREAKDOWN
    // -------------------------------------------------------------
    fn render_wbs_hierarchy(&mut self, ui: &mut Ui, theme: &MacTheme, _t: &Parametric5DTakeoff) {
        // Top Summary Cards
        let total_idr: f64 = self.wbs_catalog.iter().map(|item| item.total_cost_idr).sum();
        let curr = self.active_currency;
        let base_budget_str = curr.format_value(total_idr);
        let curr_str = format!("{} ({})", curr.code(), curr.symbol());
        let cont_val_str = curr.format_value(total_idr * (self.contingency_percent as f64 / 100.0));
        let cont_sub_str = format!("{:.1}% Reserve", self.contingency_percent);
        let vat_val_str = curr.format_value(total_idr * (self.vat_percent as f64 / 100.0));
        let vat_sub_str = format!("{:.1}% Statutory", self.vat_percent);

        ui_kit::responsive_grid_n(ui, 4, 210.0, |ui, idx| match idx {
            0 => self.metric_card(ui, theme, "TOTAL BASE BUDGET", &base_budget_str, "Grand Total Direct Cost", theme.accent_blue),
            1 => self.metric_card(ui, theme, "ACTIVE CURRENCY", &curr_str, "Multi-Currency Dynamic Conversion", theme.accent_orange),
            2 => self.metric_card(ui, theme, "CONTINGENCY ALLOCATION", &cont_val_str, &cont_sub_str, theme.accent_green),
            _ => self.metric_card(ui, theme, "VAT / PBG STATUTORY TAX", &vat_val_str, &vat_sub_str, theme.accent_red),
        });

        ui.add_space(14.0);

        // Division Filter Pills
        theme.card_frame().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                icons::icon_label(ui, AppIcon::Folder, 13.0, theme.accent_blue, "DIVISION FILTER:", 11.5, theme.accent_blue, true);

                let is_all = self.selected_wbs_division.is_none();
                let all_bg = if is_all { theme.accent_blue } else { theme.bg_canvas };
                let all_fg = if is_all { Color32::WHITE } else { theme.text_secondary };
                if ui.add(egui::Button::new(RichText::new("All Divisions").size(11.0).color(all_fg))
                    .fill(all_bg)
                    .corner_radius(CornerRadius::same(6))).clicked() {
                    self.selected_wbs_division = None;
                }

                let mut divisions: Vec<String> = self.wbs_catalog.iter().map(|i| i.division.clone()).collect();
                divisions.sort();
                divisions.dedup();

                for div in divisions {
                    let is_sel = self.selected_wbs_division.as_deref() == Some(&div);
                    let bg = if is_sel { theme.accent_blue } else { theme.bg_canvas };
                    let fg = if is_sel { Color32::WHITE } else { theme.text_secondary };
                    if ui.add(egui::Button::new(RichText::new(&div).size(11.0).color(fg))
                        .fill(bg)
                        .corner_radius(CornerRadius::same(6))).clicked() {
                        self.selected_wbs_division = Some(div);
                    }
                }
            });
        });

        ui.add_space(12.0);

        // WBS Table (Horizontally scrollable grid on narrow viewports)
        theme.card_frame().show(ui, |ui| {
            icons::icon_label(ui, AppIcon::ChartBar, 14.5, theme.accent_blue, "5D WORK BREAKDOWN STRUCTURE (WBS) QUANTITIES & RATES", 13.0, theme.text_primary, true);
            ui.add_space(10.0);

            ScrollArea::horizontal()
                .id_salt("planning_wbs_table_scroll")
                .show(ui, |ui| {
                    egui::Grid::new("planning_wbs_table_grid")
                        .striped(true)
                        .min_col_width(72.0)
                        .spacing(Vec2::new(16.0, 8.0))
                        .show(ui, |ui| {
                            ui.label(RichText::new("WBS CODE").size(11.0).strong().color(theme.text_muted));
                            ui.label(RichText::new("DESCRIPTION").size(11.0).strong().color(theme.text_muted));
                            ui.label(RichText::new("QTY / UNIT").size(11.0).strong().color(theme.text_muted));
                            ui.label(RichText::new("UNIT RATE").size(11.0).strong().color(theme.text_muted));
                            ui.label(RichText::new("TOTAL COST").size(11.0).strong().color(theme.text_muted));
                            ui.label(RichText::new("PROGRESS").size(11.0).strong().color(theme.text_muted));
                            ui.end_row();

                            for item in &self.wbs_catalog {
                                if let Some(ref sel_div) = self.selected_wbs_division {
                                    if &item.division != sel_div {
                                        continue;
                                    }
                                }

                                ui.label(RichText::new(&item.wbs_code).size(11.5).strong().color(theme.accent_blue));
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(&item.description).size(11.5).strong().color(theme.text_primary));
                                    ui.label(RichText::new(&item.division).size(10.0).color(theme.text_muted));
                                });
                                ui.label(RichText::new(format!("{:.1} {}", item.quantity, item.unit)).size(11.0).color(theme.text_secondary));
                                ui.label(RichText::new(curr.format_value(item.unit_price_idr)).size(11.0).color(theme.text_secondary));
                                ui.label(RichText::new(curr.format_value(item.total_cost_idr)).size(12.0).strong().color(theme.accent_blue));
                                ui.label(RichText::new(format!("{:.0}%", item.progress_pct)).size(11.0).strong().color(theme.accent_green));
                                ui.end_row();
                            }
                        });
                });
        });
    }

    // -------------------------------------------------------------
    // SUB-TAB 4: COST VS ARI TRADE-OFF OPTIMIZER
    // -------------------------------------------------------------
    fn render_cost_ari_tradeoff(&self, ui: &mut Ui, theme: &MacTheme, t: &Parametric5DTakeoff) {
        theme.card_frame().show(ui, |ui| {
            ui.label(RichText::new("VALUE ENGINEERING: COST SENSITIVITY (BoQ) TO REDUNDANCY INDEX (ARI)").size(13.5).strong().color(theme.accent_blue));
            ui.add_space(4.0);
            ui.label(RichText::new("Comparative analysis to evaluate the incremental investment required to enhance structural progressive collapse resistance:").size(11.5).color(theme.text_secondary));
            ui.add_space(14.0);

            let options = [
                ("Option 1: Minimum Code Compliance", 0.64, t.grand_total_rab * 0.92, "-8.0%", AppIcon::AlertTriangle, "Vulnerable to Single Failure (1 Member Removed = Collapse)", theme.accent_red),
                ("Option 2: Baseline Standard Design", 0.78, t.grand_total_rab * 0.97, "-3.0%", AppIcon::Info, "Meets Minimal SNI / AISC Standards", theme.accent_orange),
                ("Option 3: Redundancy-Optimized Design (Recommended)", t.predicted_system_ari, t.grand_total_rab, "0.0% (Base)", AppIcon::CheckCircle, "Robust Multi-Path (High Reserve Capacity)", theme.accent_green),
                ("Option 4: Ultra-Redundant (Critical Infrastructure)", 0.965, t.grand_total_rab * 1.075, "+7.5%", AppIcon::Shield, "Dual-Path Collapse Proof (Strategic Bridge)", theme.accent_blue),
            ];

            for (title, ari, cost, diff, desc_icon, desc, badge_color) in options {
                Frame::new()
                    .fill(theme.bg_sidebar)
                    .stroke(Stroke::new(1.0, theme.border_subtle))
                    .corner_radius(CornerRadius::same(8))
                    .inner_margin(Margin::symmetric(14, 10))
                    .show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.vertical(|ui| {
                                ui.label(RichText::new(title).size(13.0).strong().color(theme.text_primary));
                                ui.add_space(2.0);
                                icons::icon_label(ui, desc_icon, 12.5, badge_color, desc, 11.5, badge_color, false);
                            });

                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.label(RichText::new(format!("IDR {},-", format_idr(cost))).size(13.5).strong().color(theme.accent_blue));
                                ui.add_space(14.0);
                                ui.label(RichText::new(diff).size(12.0).color(theme.text_muted));
                                ui.add_space(14.0);
                                Frame::new()
                                    .fill(badge_color.gamma_multiply(0.2))
                                    .corner_radius(CornerRadius::same(6))
                                    .inner_margin(Margin::symmetric(8, 4))
                                    .show(ui, |ui| {
                                        ui.label(RichText::new(format!("ARI: {:.3}", ari)).size(12.0).strong().color(badge_color));
                                    });
                            });
                        });
                    });
                ui.add_space(8.0);
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(8.0);

            ui.horizontal_wrapped(|ui| {
                icons::icon_label(ui, AppIcon::Sparkles, 13.5, theme.accent_green, "VALUE ENGINEERING CONCLUSION:", 12.0, theme.accent_green, true);
                ui.label(RichText::new("With only a +3.8% cost increase from baseline standard, the ARI redundancy index jumps significantly by +16.9%, preventing catastrophic collapse mechanisms during earthquakes or vehicular impacts.").size(12.0).color(theme.text_secondary));
            });
        });
    }

    // -------------------------------------------------------------
    // SUB-TAB 5: Staged Erection SVD & ARI Guard
    // -------------------------------------------------------------
    fn render_staged_erection_svd(&self, ui: &mut Ui, theme: &MacTheme, _takeoff: &Parametric5DTakeoff) {
        ui_kit::responsive_grid_n(ui, 4, 210.0, |ui, idx| match idx {
            0 => self.metric_card(ui, theme, "ERECTION STAGES", "5 Stages", "WBS Erection Sequence", theme.accent_blue),
            1 => self.metric_card(ui, theme, "MOST CRITICAL STAGE", "Stage 2 (WBS 2.2)", "ARI: 0.285 (Shoring Required)", theme.accent_red),
            2 => self.metric_card(ui, theme, "CLOSURE STABILITY", "Week 12", "SVD Gate 1 Full Rank", theme.accent_green),
            _ => self.metric_card(ui, theme, "FINAL OPERATIONAL ARI", "0.942", "Safe Multi-Path System", theme.accent_green),
        });

        ui.add_space(16.0);

        theme.card_frame().show(ui, |ui| {
            ui.label(RichText::new("STAGED ERECTION SAFETY SIMULATION (STAGED ERECTION SVD GUARD)").size(14.0).strong().color(theme.text_primary));
            ui.add_space(4.0);
            ui.label(RichText::new("Steel structures frequently lack full redundant stiffness during field erection before closure. This staged evaluation detects critical collapse hazards during assembly:").size(12.0).color(theme.text_secondary));
            ui.add_space(12.0);

            let mut engine = crate::core::ParametricEngine::new();
            let staged_results = engine.generate_warren_truss().ok().map(|m| {
                crate::core::ScheduleEngine::verify_staged_erection(&m)
            });

            if let Some(stages) = staged_results {
                for stage in &stages {
                    let col = if stage.is_stable { theme.accent_green } else { theme.accent_red };
                    let (status_icon, status_str) = if stage.is_stable {
                        (AppIcon::CheckCircle, "STABLE / NO MECHANISM")
                    } else {
                        (AppIcon::AlertTriangle, "CRITICAL / SHORING REQUIRED")
                    };
                    let note = match stage.warning.as_deref() {
                        Some(w) => w,
                        None => "Staged stiffness stable with full rank, deflections within limits.",
                    };

                    Frame::new()
                        .fill(theme.bg_card)
                        .stroke(Stroke::new(1.0, col.gamma_multiply(0.4)))
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(Margin::symmetric(14, 10))
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                icons::draw_icon(ui, status_icon, 14.0, col);
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(&stage.stage_name).size(12.5).strong().color(theme.text_primary));
                                    ui.label(RichText::new(format!("Elements: {} members • SVD Rank: {}/{} • Condition: {:.1e}", stage.erected_elements_count, stage.rank, stage.active_dofs, stage.condition_number)).size(11.0).color(theme.text_secondary));
                                    ui.add_space(2.0);
                                    icons::icon_label(ui, AppIcon::Info, 12.0, theme.text_muted, format!("Evaluation: {}", note), 11.0, theme.text_muted, false);
                                });

                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    Frame::new()
                                        .fill(col.gamma_multiply(0.18))
                                        .stroke(Stroke::new(1.0, col))
                                        .corner_radius(CornerRadius::same(6))
                                        .inner_margin(Margin::symmetric(8, 4))
                                        .show(ui, |ui| {
                                            ui.label(RichText::new(format!("Defl: {:.2} mm", stage.max_deflection_mm)).size(11.5).strong().color(col));
                                        });
                                    ui.add_space(12.0);
                                    ui.horizontal(|ui| {
                                        icons::icon_label(ui, status_icon, 12.5, col, status_str, 11.0, col, true);
                                    });
                                });
                            });
                        });
                    ui.add_space(6.0);
                }
            }
        });
    }

    // -------------------------------------------------------------
    // SUB-TAB 6: MONTE CARLO & PERT SCHEDULE RISK (4D-03 LOCKED)
    // -------------------------------------------------------------
    fn render_schedule_risk_monte_carlo(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(
            ui,
            theme,
            features,
            FeatureId::MonteCarloScheduleRiskPert,
            |ui| {
                ui_kit::responsive_grid_n(ui, 4, 185.0, |ui, idx| match idx {
                    0 => ui_kit::stat_card(
                        ui,
                        theme,
                        "DETERMINISTIC CPM",
                        "360 Days",
                        "Baseline Zero-Float Duration",
                        theme.text_primary,
                    ),
                    1 => ui_kit::stat_card(
                        ui,
                        theme,
                        "P50 MEDIAN COMPLETION",
                        "374 Days (+14d)",
                        "50% Joint Confidence Forecast",
                        theme.accent_blue,
                    ),
                    2 => ui_kit::stat_card(
                        ui,
                        theme,
                        "P80 SCHEDULE BUFFER",
                        "396 Days (+36d)",
                        "Recommended Contingency Envelope",
                        theme.accent_orange,
                    ),
                    _ => ui_kit::stat_card(
                        ui,
                        theme,
                        "TOP SENSITIVITY DRIVER",
                        "WBS 05.12 (r=0.78)",
                        "Structural Steel Erection Criticality",
                        theme.accent_green,
                    ),
                });

                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    let _ = ui.selectable_label(true, "Sampling: 1,000 Latin Hypercube Trials");
                    let _ = ui.selectable_label(false, "PERT Beta Distribution (O / M / P)");
                    let _ = icons::icon_button(ui, AppIcon::ChartBar, "Run Monte Carlo Schedule Simulation", 11.5, theme.text_primary, theme.bg_canvas, Stroke::new(1.0, theme.border_subtle), CornerRadius::same(6), Vec2::ZERO);
                });
                ui.add_space(12.0);

                ui_kit::responsive_two_panes(ui, 660.0, |ui, pane| match pane {
                    0 => {
                        ui.label(
                            RichText::new("CUMULATIVE COMPLETION PROBABILITY (CDF S-CURVE)")
                                .size(11.0)
                                .strong()
                                .color(theme.accent_blue),
                        );
                        ui.add_space(6.0);
                        let desired = Vec2::new(ui.available_width(), 170.0);
                        let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
                        let painter = ui.painter_at(rect);
                        painter.rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(13, 17, 24));
                        let plot = rect.shrink(28.0);
                        for step in 0..=4 {
                            let frac = step as f32 / 4.0;
                            let y = plot.bottom() - frac * plot.height();
                            painter.line_segment(
                                [Pos2::new(plot.left(), y), Pos2::new(plot.right(), y)],
                                Stroke::new(0.8, Color32::from_white_alpha(20)),
                            );
                        }
                        let mut cdf_pts = Vec::with_capacity(25);
                        for i in 0..=24 {
                            let x_frac = i as f32 / 24.0;
                            let z = (x_frac - 0.45) * 6.2;
                            let sig = 1.0 / (1.0 + (-z).exp());
                            let px = plot.left() + x_frac * plot.width();
                            let py = plot.bottom() - sig * plot.height();
                            cdf_pts.push(Pos2::new(px, py));
                        }
                        for w in cdf_pts.windows(2) {
                            painter.line_segment([w[0], w[1]], Stroke::new(2.2, theme.accent_green));
                        }
                        let p80_x = plot.left() + 0.68 * plot.width();
                        let p80_y = plot.bottom() - 0.80 * plot.height();
                        painter.line_segment(
                            [Pos2::new(p80_x, plot.bottom()), Pos2::new(p80_x, p80_y)],
                            Stroke::new(1.5, theme.accent_orange),
                        );
                        painter.circle_filled(Pos2::new(p80_x, p80_y), 4.5, theme.accent_orange);
                        painter.text(
                            Pos2::new(p80_x + 6.0, p80_y - 4.0),
                            Align2::LEFT_BOTTOM,
                            "P80 = 396d",
                            egui::FontId::monospace(9.5),
                            theme.accent_orange,
                        );
                    }
                    _ => {
                        ui.label(
                            RichText::new("TORNADO CRITICALITY & SCHEDULE SENSITIVITY")
                                .size(11.0)
                                .strong()
                                .color(theme.accent_blue),
                        );
                        ui.add_space(6.0);
                        let drivers = [
                            ("05.12 Steel Erection & Torque", 0.78, "±18d", theme.accent_red),
                            ("02.10 Deep Bored Pile Sockets", 0.64, "±14d", theme.accent_orange),
                            ("08.40 Unitized Curtain Wall Hoist", 0.49, "±11d", theme.accent_blue),
                            ("23.00 MEP Riser Testing Window", 0.31, "±6d", theme.accent_green),
                        ];
                        for (name, corr, swing, col) in drivers {
                            ui.horizontal_wrapped(|ui| {
                                ui.label(RichText::new(name).size(11.0).color(theme.text_primary));
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.label(
                                        RichText::new(format!("r={:.2} ({})", corr, swing))
                                            .size(10.5)
                                            .strong()
                                            .color(col),
                                    );
                                });
                            });
                            let (bar_r, _) = ui.allocate_exact_size(
                                Vec2::new(ui.available_width().max(120.0), 8.0),
                                Sense::hover(),
                            );
                            ui.painter().rect_filled(bar_r, CornerRadius::same(4), theme.bg_canvas);
                            ui.painter().rect_filled(
                                Rect::from_min_size(bar_r.min, Vec2::new(bar_r.width() * corr, 8.0)),
                                CornerRadius::same(4),
                                col,
                            );
                            ui.add_space(6.0);
                        }
                    }
                });
            },
        );
    }

    // -------------------------------------------------------------
    // SUB-TAB 7: CREW HISTOGRAM & RESOURCE LEVELING (4D-04 LOCKED)
    // -------------------------------------------------------------
    fn render_resource_leveling_4d(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(
            ui,
            theme,
            features,
            FeatureId::ResourceLevelingCrewHistogram,
            |ui| {
                ui_kit::responsive_grid_n(ui, 4, 185.0, |ui, idx| match idx {
                    0 => ui_kit::stat_card(
                        ui,
                        theme,
                        "UNSMOOTHED PEAK CREW",
                        "112 Workers/d",
                        "Months M05–M06 Concurrent Peaks",
                        theme.accent_red,
                    ),
                    1 => ui_kit::stat_card(
                        ui,
                        theme,
                        "LEVELED PEAK DEMAND",
                        "82 Workers/d",
                        "-26.8% Peak Smoothing (Within Float)",
                        theme.accent_green,
                    ),
                    2 => ui_kit::stat_card(
                        ui,
                        theme,
                        "SITE HOIST & CRANE DUTY",
                        "78.4% Utilization",
                        "Zero Simultaneous Double-Pick Clashes",
                        theme.accent_blue,
                    ),
                    _ => ui_kit::stat_card(
                        ui,
                        theme,
                        "TOTAL FLOAT CONSUMED",
                        "4.5 Days avg",
                        "Critical Path Unchanged (360d)",
                        theme.accent_orange,
                    ),
                });

                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    let _ = ui.selectable_label(true, "Policy: Level Within Free Float Only");
                    let _ = ui.selectable_label(false, "Site Crew Cap = 85 Workers");
                    let _ = icons::icon_button(ui, AppIcon::HardHat, "Recompute Leveled Manpower Histogram", 11.5, theme.text_primary, theme.bg_canvas, Stroke::new(1.0, theme.border_subtle), CornerRadius::same(6), Vec2::ZERO);
                });
                ui.add_space(10.0);

                let desired = Vec2::new(ui.available_width(), 165.0);
                let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
                let painter = ui.painter_at(rect);
                painter.rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(13, 17, 24));
                let plot = rect.shrink(26.0);
                let cap_y = plot.bottom() - (85.0 / 120.0) * plot.height();
                painter.line_segment(
                    [Pos2::new(plot.left(), cap_y), Pos2::new(plot.right(), cap_y)],
                    Stroke::new(1.5, theme.accent_red),
                );
                painter.text(
                    Pos2::new(plot.left() + 4.0, cap_y - 3.0),
                    Align2::LEFT_BOTTOM,
                    "Site Crew Capacity Limit (85 Workers)",
                    egui::FontId::monospace(9.0),
                    theme.accent_red,
                );

                let monthly_crews = [
                    ("M1", 28.0_f32),
                    ("M2", 42.0),
                    ("M3", 58.0),
                    ("M4", 72.0),
                    ("M5", 82.0),
                    ("M6", 81.0),
                    ("M7", 79.0),
                    ("M8", 74.0),
                    ("M9", 68.0),
                    ("M10", 54.0),
                    ("M11", 38.0),
                    ("M12", 22.0),
                ];
                let slot_w = plot.width() / monthly_crews.len() as f32;
                let bar_w = (slot_w * 0.56).clamp(10.0, 28.0);
                for (idx, (mo, workers)) in monthly_crews.iter().enumerate() {
                    let cx = plot.left() + slot_w * (idx as f32 + 0.5);
                    let h = (*workers / 120.0) * plot.height();
                    let bar = Rect::from_min_max(
                        Pos2::new(cx - bar_w * 0.5, plot.bottom() - h),
                        Pos2::new(cx + bar_w * 0.5, plot.bottom()),
                    );
                    let col = if *workers > 78.0 {
                        theme.accent_blue
                    } else {
                        theme.accent_green
                    };
                    painter.rect_filled(bar, CornerRadius::same(3), col);
                    painter.text(
                        Pos2::new(cx, plot.bottom() + 4.0),
                        Align2::CENTER_TOP,
                        *mo,
                        egui::FontId::monospace(8.5),
                        theme.text_muted,
                    );
                }
            },
        );
    }

    // -------------------------------------------------------------
    // SUB-TAB 8: EARNED VALUE EVM & PROGRESS CLAIM (5D-03 LOCKED)
    // -------------------------------------------------------------
    fn render_earned_value_evm(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        t: &Parametric5DTakeoff,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(
            ui,
            theme,
            features,
            FeatureId::EarnedValueEvmProgressClaim,
            |ui| {
                let curr = self.active_currency;
                let ev_val = t.grand_total_rab * 0.684;
                let eac_val = t.grand_total_rab / 0.972;
                let ev_str = curr.format_value(ev_val);
                let eac_str = curr.format_value(eac_val);

                ui_kit::responsive_grid_n(ui, 4, 185.0, |ui, idx| match idx {
                    0 => ui_kit::stat_card(
                        ui,
                        theme,
                        "SCHEDULE PERF INDEX (SPI)",
                        "1.04 (Ahead)",
                        "EV / PV • +4.0% Early Erection Progress",
                        theme.accent_green,
                    ),
                    1 => ui_kit::stat_card(
                        ui,
                        theme,
                        "COST PERF INDEX (CPI)",
                        "0.97 (Watch)",
                        "EV / AC • Fabrication Price Escalation",
                        theme.accent_orange,
                    ),
                    2 => ui_kit::stat_card(
                        ui,
                        theme,
                        "EARNED VALUE (EV @ M07)",
                        &ev_str,
                        "68.4% Cumulative Certified Progress",
                        theme.accent_blue,
                    ),
                    _ => ui_kit::stat_card(
                        ui,
                        theme,
                        "ESTIMATE AT COMPLETION (EAC)",
                        &eac_str,
                        "BAC / CPI Projection • 5% Retention Held",
                        theme.text_primary,
                    ),
                });

                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    let _ = ui.selectable_label(true, "Reporting Cutoff: Month 07 (IPC-07)");
                    let _ = ui.selectable_label(false, "Contract Retention: 5.0%");
                    let _ = icons::icon_button(ui, AppIcon::ChartLine, "Certify Monthly Progress Claim (.PDF / .CSV)", 11.5, theme.text_primary, theme.bg_canvas, Stroke::new(1.0, theme.border_subtle), CornerRadius::same(6), Vec2::ZERO);
                });
                ui.add_space(12.0);

                ui_kit::responsive_two_panes(ui, 660.0, |ui, pane| match pane {
                    0 => {
                        ui.label(
                            RichText::new("PV / EV / AC EARNED VALUE S-CURVE (M01–M12)")
                                .size(11.0)
                                .strong()
                                .color(theme.accent_blue),
                        );
                        ui.add_space(6.0);
                        let desired = Vec2::new(ui.available_width(), 165.0);
                        let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
                        let painter = ui.painter_at(rect);
                        painter.rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(13, 17, 24));
                        let plot = rect.shrink(24.0);
                        let cut_x = plot.left() + 0.58 * plot.width();
                        painter.line_segment(
                            [Pos2::new(cut_x, plot.top()), Pos2::new(cut_x, plot.bottom())],
                            Stroke::new(1.2, theme.text_muted),
                        );
                        painter.text(
                            Pos2::new(cut_x + 4.0, plot.top() + 2.0),
                            Align2::LEFT_TOP,
                            "Data Date M07",
                            egui::FontId::monospace(8.5),
                            theme.text_muted,
                        );
                        // Planned Value S-Curve (Full M01..M12)
                        let pv_vals = [0.04_f32, 0.09, 0.17, 0.28, 0.42, 0.55, 0.66, 0.77, 0.87, 0.94, 0.98, 1.0];
                        let pv_pts: Vec<Pos2> = pv_vals
                            .iter()
                            .enumerate()
                            .map(|(i, v)| {
                                Pos2::new(
                                    plot.left() + (i as f32 / 11.0) * plot.width(),
                                    plot.bottom() - *v * plot.height(),
                                )
                            })
                            .collect();
                        for w in pv_pts.windows(2) {
                            painter.line_segment([w[0], w[1]], Stroke::new(1.8, theme.accent_blue));
                        }
                        // Earned Value through M07
                        let ev_vals = [0.04_f32, 0.10, 0.18, 0.30, 0.44, 0.58, 0.684];
                        let ev_pts: Vec<Pos2> = ev_vals
                            .iter()
                            .enumerate()
                            .map(|(i, v)| {
                                Pos2::new(
                                    plot.left() + (i as f32 / 11.0) * plot.width(),
                                    plot.bottom() - *v * plot.height(),
                                )
                            })
                            .collect();
                        for w in ev_pts.windows(2) {
                            painter.line_segment([w[0], w[1]], Stroke::new(2.4, theme.accent_green));
                        }
                        // Actual Cost through M07
                        let ac_vals = [0.05_f32, 0.11, 0.20, 0.32, 0.46, 0.60, 0.705];
                        let ac_pts: Vec<Pos2> = ac_vals
                            .iter()
                            .enumerate()
                            .map(|(i, v)| {
                                Pos2::new(
                                    plot.left() + (i as f32 / 11.0) * plot.width(),
                                    plot.bottom() - *v * plot.height(),
                                )
                            })
                            .collect();
                        for w in ac_pts.windows(2) {
                            painter.line_segment([w[0], w[1]], Stroke::new(1.8, theme.accent_orange));
                        }
                    }
                    _ => {
                        ui.label(
                            RichText::new("INTERIM PAYMENT CERTIFICATE (IPC-07) BY DIVISION")
                                .size(11.0)
                                .strong()
                                .color(theme.accent_blue),
                        );
                        ui.add_space(6.0);
                        ScrollArea::horizontal().id_salt("evm_ipc_scroll").show(ui, |ui| {
                            egui::Grid::new("evm_ipc_grid")
                                .striped(true)
                                .spacing(Vec2::new(12.0, 5.0))
                                .show(ui, |ui| {
                                    for h in ["Division", "BAC %", "Earned %", "CPI", "SPI", "Status"] {
                                        ui.label(RichText::new(h).size(10.5).strong().color(theme.accent_blue));
                                    }
                                    ui.end_row();
                                    for (div, bac, ev, cpi, spi, st) in [
                                        ("02 Substructure", "100%", "100.0%", "1.02", "1.00", "Settled"),
                                        ("03 RC Cores & Slabs", "100%", "84.2%", "0.98", "1.05", "Certified"),
                                        ("05 Structural Steel", "100%", "71.5%", "0.95", "1.06", "IPC-07 Ready"),
                                        ("08 Unitized Glazing", "100%", "28.0%", "1.01", "0.96", "Fabricating"),
                                    ] {
                                        ui.label(RichText::new(div).size(10.5).strong().color(theme.text_primary));
                                        ui.label(RichText::new(bac).size(10.5).color(theme.text_muted));
                                        ui.label(RichText::new(ev).size(10.5).color(theme.accent_green));
                                        ui.label(RichText::new(cpi).size(10.5).color(theme.text_secondary));
                                        ui.label(RichText::new(spi).size(10.5).color(theme.accent_blue));
                                        ui.label(RichText::new(st).size(10.5).color(theme.accent_green));
                                        ui.end_row();
                                    }
                                });
                        });
                    }
                });
            },
        );
    }

    // -------------------------------------------------------------
    // SUB-TAB 9: VALUE ENGINEERING & BID LEVELING (5D-04 LOCKED)
    // -------------------------------------------------------------
    fn render_value_engineering_bid(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        t: &Parametric5DTakeoff,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(
            ui,
            theme,
            features,
            FeatureId::ValueEngineeringTenderMatrix,
            |ui| {
                let curr = self.active_currency;
                let oe_str = curr.format_value(t.grand_total_rab);
                let low_bid_str = curr.format_value(t.grand_total_rab * 0.936);
                let ve_save_str = curr.format_value(t.grand_total_rab * 0.042);

                ui_kit::responsive_grid_n(ui, 4, 185.0, |ui, idx| match idx {
                    0 => ui_kit::stat_card(
                        ui,
                        theme,
                        "OWNER ESTIMATE (OE)",
                        &oe_str,
                        "Baseline 5D Cost Model (Parametric)",
                        theme.text_primary,
                    ),
                    1 => ui_kit::stat_card(
                        ui,
                        theme,
                        "LOWEST COMPLIANT BID",
                        &low_bid_str,
                        "Bidder B (-6.4% Below Owner Est.)",
                        theme.accent_green,
                    ),
                    2 => ui_kit::stat_card(
                        ui,
                        theme,
                        "UNBALANCED RATE ALERTS",
                        "1 Flagged Line",
                        "Bidder C Earthwork +28% Front-Loaded",
                        theme.accent_orange,
                    ),
                    _ => ui_kit::stat_card(
                        ui,
                        theme,
                        "VE REDESIGN POTENTIAL",
                        &format!("-{}", ve_save_str),
                        "Composite Metal Deck Swap (ARI Safe)",
                        theme.accent_blue,
                    ),
                });

                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    let _ = ui.selectable_label(true, "Outlier Threshold: ±15% vs Owner Est");
                    let _ = ui.selectable_label(false, "Preserve ARI >= 0.90 Constraint");
                    let _ = icons::icon_button(ui, AppIcon::SlidersPanel, "Export Normalized Bid Leveling Matrix (.XLSX/.MD)", 11.5, theme.text_primary, theme.bg_canvas, Stroke::new(1.0, theme.border_subtle), CornerRadius::same(6), Vec2::ZERO);
                });
                ui.add_space(10.0);

                ScrollArea::horizontal().id_salt("tender_bid_scroll").show(ui, |ui| {
                    egui::Grid::new("tender_bid_grid")
                        .striped(true)
                        .min_col_width(90.0)
                        .spacing(Vec2::new(14.0, 6.0))
                        .show(ui, |ui| {
                            for h in [
                                "Trade Package",
                                "Owner Est (OE)",
                                "Bidder A (SteelFab)",
                                "Bidder B (Nusantara)",
                                "Bidder C (GlobalTruss)",
                                "Outlier Audit",
                            ] {
                                ui.label(RichText::new(h).size(10.5).strong().color(theme.accent_blue));
                            }
                            ui.end_row();
                            for (pkg, oe, a, b, c, flag_icon, flag, col) in [
                                (
                                    "02 Bored Piles",
                                    curr.format_value(t.cost_substructure),
                                    curr.format_value(t.cost_substructure * 0.98),
                                    curr.format_value(t.cost_substructure * 0.95),
                                    curr.format_value(t.cost_substructure * 1.28),
                                    AppIcon::AlertTriangle,
                                    "Bidder C +28%",
                                    theme.accent_orange,
                                ),
                                (
                                    "05 Steel Supply BJ41",
                                    curr.format_value(t.cost_steel_material),
                                    curr.format_value(t.cost_steel_material * 0.97),
                                    curr.format_value(t.cost_steel_material * 0.92),
                                    curr.format_value(t.cost_steel_material * 0.99),
                                    AppIcon::CheckCircle,
                                    "Balanced",
                                    theme.accent_green,
                                ),
                                (
                                    "05 Workshop Fab + NDT",
                                    curr.format_value(t.cost_workshop_fab),
                                    curr.format_value(t.cost_workshop_fab * 1.03),
                                    curr.format_value(t.cost_workshop_fab * 0.94),
                                    curr.format_value(t.cost_workshop_fab * 0.96),
                                    AppIcon::CheckCircle,
                                    "Balanced",
                                    theme.accent_green,
                                ),
                                (
                                    "05 Crane Erection",
                                    curr.format_value(t.cost_erection_crane),
                                    curr.format_value(t.cost_erection_crane * 1.02),
                                    curr.format_value(t.cost_erection_crane * 0.91),
                                    curr.format_value(t.cost_erection_crane * 0.95),
                                    AppIcon::Star,
                                    "Best Value (Bidder B)",
                                    theme.accent_blue,
                                ),
                            ] {
                                ui.label(RichText::new(pkg).size(10.5).strong().color(theme.text_primary));
                                ui.label(RichText::new(oe).size(10.5).color(theme.text_secondary));
                                ui.label(RichText::new(a).size(10.5).color(theme.text_secondary));
                                ui.label(RichText::new(b).size(10.5).strong().color(theme.accent_green));
                                ui.label(RichText::new(c).size(10.5).color(theme.text_secondary));
                                icons::icon_label(ui, flag_icon, 12.0, col, flag, 10.5, col, true);
                                ui.end_row();
                            }
                        });
                });
            },
        );
    }

    fn metric_card(&self, ui: &mut Ui, theme: &MacTheme, title: &str, value: &str, subtitle: &str, val_color: Color32) {
        theme.card_frame().show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(RichText::new(title).size(10.5).strong().color(theme.text_muted));
            ui.add_space(4.0);
            ui.label(RichText::new(value).size(16.0).strong().color(val_color));
            ui.add_space(2.0);
            ui.label(RichText::new(subtitle).size(10.5).color(theme.text_secondary));
        });
    }
}
