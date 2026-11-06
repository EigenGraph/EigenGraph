//! research.rs — Pillar 1D: Research & Concept.
//! The foundational stage involving initial ideas, feasibility studies, regulatory checks, surveying data, and project guidelines.

use egui::{Align, Color32, CornerRadius, Frame, Layout, Margin, RichText, ScrollArea, Stroke, Ui, Vec2};
use crate::models::NavigationPillar;
use crate::theme::MacTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResearchSubTab {
    ConceptFeasibility,
    RegulatoryCodes,
    SurveyingSite,
    AriFormulations,
}

pub struct ResearchConceptView {
    pub active_subtab: ResearchSubTab,
    pub selected_truss_family: String,
    pub target_design_span_m: f32,
}

impl Default for ResearchConceptView {
    fn default() -> Self {
        Self {
            active_subtab: ResearchSubTab::ConceptFeasibility,
            selected_truss_family: "Warren with Verticals (Family S1)".to_string(),
            target_design_span_m: 24.0,
        }
    }
}

impl ResearchConceptView {
    pub fn show(&mut self, ui: &mut Ui, theme: &MacTheme, active_pillar: &mut NavigationPillar) {
        ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
            Frame::new().inner_margin(Margin::symmetric(20, 12)).show(ui, |ui| {
                // 1. TOP SUBTABS SELECTOR
                self.render_header_bar(ui, theme);

                ui.add_space(16.0);

                // 2. ACTIVE SUB-TAB CONTENT
                match self.active_subtab {
                    ResearchSubTab::ConceptFeasibility => self.render_concept_feasibility(ui, theme, active_pillar),
                    ResearchSubTab::RegulatoryCodes => self.render_regulatory_codes(ui, theme),
                    ResearchSubTab::SurveyingSite => self.render_surveying_site(ui, theme),
                    ResearchSubTab::AriFormulations => self.render_ari_formulations(ui, theme),
                }

                ui.add_space(24.0);
            });
        });
    }

    fn render_header_bar(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(6))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                let tabs = [
                    (ResearchSubTab::ConceptFeasibility, "💡 Feasibility Study & Initial Concept"),
                    (ResearchSubTab::RegulatoryCodes, "📜 Regulations & Design Standards (SNI/AISC)"),
                    (ResearchSubTab::SurveyingSite, "🗺️ Site Survey & Geotechnical Data"),
                    (ResearchSubTab::AriFormulations, "🔬 Official ARI Formulations (EigenGraph)"),
                ];

                ui.columns(4, |cols| {
                    for (idx, (tab, label)) in tabs.into_iter().enumerate() {
                        let is_active = self.active_subtab == tab;
                        let bg = if is_active { theme.accent_blue } else { Color32::TRANSPARENT };
                        let fg = if is_active { Color32::WHITE } else { theme.text_secondary };

                        let w = cols[idx].available_width();
                        if cols[idx].add(egui::Button::new(RichText::new(label).size(12.0).strong().color(fg))
                            .fill(bg)
                            .corner_radius(CornerRadius::same(7))
                            .min_size(Vec2::new(w, 30.0))).clicked() {
                            self.active_subtab = tab;
                        }
                    }
                });
            });
    }

    fn render_concept_feasibility(&mut self, ui: &mut Ui, theme: &MacTheme, active_pillar: &mut NavigationPillar) {
        // Hero Card
        Frame::new()
            .fill(theme.accent_blue.gamma_multiply(0.12))
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.accent_blue.gamma_multiply(0.4)))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("💡 INITIAL CONCEPTUAL STAGE (1D BIM)").size(11.0).strong().color(theme.accent_blue));
                        ui.add_space(4.0);
                        ui.label(RichText::new("Technical Feasibility Study, Regulatory Constraints & Project Guidelines").size(18.0).strong().color(theme.text_primary));
                        ui.add_space(4.0);
                        ui.label(RichText::new("Early stage integration of design ideas, river clear span limitations, setting ARI redundancy target ≥ 0.85, and budget readiness.").size(12.0).color(theme.text_secondary));
                    });
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.add(egui::Button::new(RichText::new("Start 2D Drafting ➔").size(12.0).strong().color(Color32::WHITE))
                            .fill(theme.accent_blue)
                            .corner_radius(CornerRadius::same(6))
                            .min_size(Vec2::new(140.0, 32.0))).clicked() {
                            *active_pillar = NavigationPillar::DraftingDrawings;
                        }
                    });
                });
            });

        ui.add_space(16.0);

        // KPI Cards Row
        ui.columns(4, |cols| {
            self.render_stat_card(&mut cols[0], theme, "TECHNICAL FEASIBILITY", "96.5% FEASIBLE", "24m Clear Span Without Central Pier", theme.accent_green);
            self.render_stat_card(&mut cols[1], theme, "REDUNDANCY TARGET", "ARI ≥ 0.85", "Grade A Criteria: Free of Single Point of Failure", theme.accent_blue);
            self.render_stat_card(&mut cols[2], theme, "COST FEASIBILITY", "IDR 3.84 B", "Max Budget Cap: IDR 4.20 B (Safe)", theme.text_primary);
            self.render_stat_card(&mut cols[3], theme, "SCHEDULE TIMELINE", "140 Days", "6 SVD-Verified Erection Stages", theme.text_primary);
        });

        ui.add_space(16.0);

        // Comparative Feasibility Matrix
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.label(RichText::new("📊 Structural Initial Concept Alternative Evaluation Matrix").size(13.5).strong().color(theme.text_primary));
                ui.add_space(10.0);

                let rows = [
                    ("Option 1: Red Steel Warren Bowstring Truss (Selected)", "Span 24.0m, Height 3.2m", "ARI = 0.94", "IDR 3.84 B", "⭐ Optimum: Highest ARI score, controlled cost, iconic aesthetics", true),
                    ("Option 2: Twin Composite I-Girder System", "Span 24.0m, 4 Girders", "ARI = 0.62", "IDR 3.65 B", "⚠️ Vulnerable: Large midspan deflection, low redundancy", false),
                    ("Option 3: Cantilever Pratt Truss", "Span 24.0m, Sub-Truss 12m", "ARI = 0.74", "IDR 4.05 B", "🟡 Fair: Efficient erection but higher material cost", false),
                ];

                for (name, spec, ari, cost, notes, is_best) in rows {
                    let fill = if is_best { theme.accent_blue.gamma_multiply(0.10) } else { theme.bg_canvas };
                    let stroke = if is_best { Stroke::new(1.5, theme.accent_blue) } else { Stroke::new(1.0, theme.border_subtle) };

                    Frame::new()
                        .fill(fill)
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(Margin::same(12))
                        .stroke(stroke)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new(name).size(12.5).strong().color(if is_best { theme.accent_blue } else { theme.text_primary }));
                                        if is_best {
                                            ui.label(RichText::new("⭐ SELECTED RECOMMENDATION").size(10.0).strong().color(theme.accent_green));
                                        }
                                    });
                                    ui.add_space(2.0);
                                    ui.label(RichText::new(format!("{} | Note: {}", spec, notes)).size(11.0).color(theme.text_secondary));
                                });
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.vertical(|ui| {
                                        ui.label(RichText::new(ari).size(13.0).strong().color(if is_best { theme.accent_green } else { theme.text_primary }));
                                        ui.label(RichText::new(cost).size(11.0).color(theme.text_muted));
                                    });
                                });
                            });
                        });
                    ui.add_space(8.0);
                }
            });
    }

    fn render_regulatory_codes(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.label(RichText::new("📜 Regulatory Compliance & National / International Design Standards Verification").size(13.5).strong().color(theme.text_primary));
                ui.add_space(10.0);

                let codes = [
                    ("SNI 1729:2020", "Specification for Structural Steel Buildings (AISC 360-16 LRFD Adoption)", "Resistance factors: ϕ_t = 0.90, ϕ_c = 0.90, ϕ_b = 0.90. Verified elastic safe.", "🟢 Compliant"),
                    ("SNI 2833:2016", "Bridge Design for Earthquake Loads", "Critical bridge importance category, importance factor I = 1.20.", "🟢 Compliant"),
                    ("AISC 360-16 Section H1", "Interaction of Flexure and Axial Force for Members under Combined Stresses", "P-M-V interaction evaluation equations H1-1a and H1-1b. Maximum DCR 0.528.", "🟢 Compliant"),
                    ("DoD UFC 4-023-03", "Design of Buildings to Resist Progressive Collapse (Alternate Path Method)", "N-1 and N-2 key member removal disruption test. Full rank SVD K_ff.", "🟢 Compliant"),
                    ("PUPR Ministerial Regulation No. 10/2021", "Construction Safety Management System Guidelines (SMKK)", "Integration of OSH risk matrix and SVD Kinematic Mechanism Guard during design-build phase.", "🟢 Compliant"),
                ];

                for (code, title, details, status) in codes {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(RichText::new(code).size(12.5).strong().color(theme.accent_blue));
                            ui.label(RichText::new(title).size(11.5).strong().color(theme.text_primary));
                            ui.label(RichText::new(details).size(11.0).color(theme.text_secondary));
                        });
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(RichText::new(status).size(11.5).strong().color(theme.accent_green));
                        });
                    });
                    ui.separator();
                }
            });
    }

    fn render_surveying_site(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.label(RichText::new("🗺️ Topographic Survey, Site Conditions & Foundation Geotechnical Data").size(13.5).strong().color(theme.text_primary));
                ui.add_space(10.0);

                ui.columns(4, |cols| {
                    self.render_stat_card(&mut cols[0], theme, "SOIL BEARING CAPACITY (qa)", "285.0 kPa", "Hard Clay N-SPT > 35", theme.text_primary);
                    self.render_stat_card(&mut cols[1], theme, "BASIC SEISMIC ZONE", "Site Class D", "Ss = 0.85g, S1 = 0.42g (Java Region)", theme.accent_blue);
                    self.render_stat_card(&mut cols[2], theme, "DESIGN WIND SPEED", "32.0 m/s", "Dynamic Pressure q = 0.627 kN/m²", theme.text_primary);
                    self.render_stat_card(&mut cols[3], theme, "SITE CORROSION CATEGORY", "C5-High Marine", "Sa 2.5 Polyurethane Epoxy Paint Protection", theme.accent_green);
                });
            });
    }

    fn render_ari_formulations(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(16))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("🔬 Official Theoretical Foundation: Adjusted Redundancy Index (EigenGraph 2026)").size(13.5).strong().color(theme.text_primary));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new("Lead Engineer: Harids Irawan, S.T., M.T. (EigenGraph Systems)").size(11.0).color(theme.accent_blue));
                    });
                });
                ui.add_space(10.0);

                let formulas = [
                    ("1. Hyperbolic Saturation Normalization (Equations 2.5a–2.5b)", "f(k) = k / (k + s_k)   and   g(R) = R / (R + s_R)", "Prevents distortion of unbounded stiffness and capacity scales."),
                    ("2. Member Edge Mechanical Weight (Equation 2.7)", "w_e^M = α_k · f(k_e) + α_R · g(R_e)", "Combines contributions of elastic stiffness and ultimate cross-section capacity."),
                    ("3. Multigraph & Edge-Disjoint K-Paths (Menger's Theorem, Equation 2.9)", "S(G^M) = (1/K) · Σ w_e^M", "Traces topological redundancy of alternative load transfer paths."),
                    ("4. Multiplicative Adjusted Redundancy Index Core (Equation 2.17)", "ARI_e = RR_e × (1.0 - U_e^*)", "Integrates topological redundancy ratio with remaining elastic reserve of the member."),
                    ("5. SVD Kinematic Mechanism Guard (Gate 1, Equation 4.1)", "K_ff = U · Σ · V^T   ==>   deteksi σ_min = 0 (Nullspace = mechanism)", "Guarantees the structure is free from unstable kinematic collapse mechanisms."),
                ];

                for (title, eq, desc) in formulas {
                    Frame::new()
                        .fill(theme.bg_canvas)
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(Margin::same(12))
                        .stroke(Stroke::new(1.0, theme.border_subtle))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.label(RichText::new(title).size(12.0).strong().color(theme.text_primary));
                            ui.add_space(4.0);
                            ui.label(RichText::new(eq).size(13.0).strong().color(theme.accent_blue));
                            ui.add_space(2.0);
                            ui.label(RichText::new(desc).size(11.0).color(theme.text_secondary));
                        });
                    ui.add_space(8.0);
                }
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
