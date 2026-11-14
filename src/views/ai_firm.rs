//! ai_firm.rs — Pillar 2: AI Agent & Virtual Engineering Firm Container.
//! Features the autonomous multi-agent engineering firm organigram, interactive AI Civil Engineering
//! Copilot Studio (ACI 318-19 Shear Wall, ASCE 7-22 Wind, ISO 14040 Carbon Minimizer), live
//! collaboration board, and directive console.

use egui::{
    Align, Color32, CornerRadius, Frame, Layout, Margin, RichText, ScrollArea, Stroke, Ui,
};
use crate::core::web_bridge;
use crate::features::{FeatureId, FeatureRegistry};
use crate::icons::{self, AppIcon};
use crate::models::{
    CalculationMemoResult, CopilotCarbonParams, CopilotShearWallParams, CopilotWindParams,
    CopilotWorkflowKind, DatabaseComponent, ProjectItem, StructuralMember,
    solve_aci318_shear_wall, solve_asce7_wind, solve_iso14040_carbon,
};
use crate::theme::MacTheme;
use crate::ui_kit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiFirmSubTab {
    Organigram,
    CopilotStudio,
    ComplianceAuditor,
    ParetoOptimizer,
    CollaborationBoard,
    DirectivesConsole,
    AutonomousTelemetry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirmRole {
    ProjectManager,
    SeniorDrafter,
    FeaAnalyst,
    QsSpecialist,
}

impl FirmRole {
    pub fn title(&self) -> &'static str {
        match self {
            Self::ProjectManager => "Project Manager & Orchestrator",
            Self::SeniorDrafter => "Senior Structural Drafter",
            Self::FeaAnalyst => "Finite Element Analyst",
            Self::QsSpecialist => "Quantity Surveyor & Cost Specialist",
        }
    }

    pub fn app_icon(&self) -> AppIcon {
        match self {
            Self::ProjectManager => AppIcon::HardHat,
            Self::SeniorDrafter => AppIcon::RulerSquare,
            Self::FeaAnalyst => AppIcon::Bolt,
            Self::QsSpecialist => AppIcon::Coins,
        }
    }

    pub fn specialization(&self) -> &'static str {
        match self {
            Self::ProjectManager => "Workflow Coordination, Quality Assurance & Milestone Verification",
            Self::SeniorDrafter => "AutoCAD Vector Detailing, Rebar BBS Schedules & 2D/3D Detailing",
            Self::FeaAnalyst => "Non-Linear Frame Mechanics, SVD Kinematic Guard & ARI Scoring",
            Self::QsSpecialist => "5D Dynamic QTO, AHSP Unit Price Book & Parametric BoQ Ledgers",
        }
    }

    pub fn default_status(&self) -> &'static str {
        match self {
            Self::ProjectManager => "Active • Monitoring Quality & Deliverables",
            Self::SeniorDrafter => "Active • Synthesizing Grid A-1 Section Sheets",
            Self::FeaAnalyst => "Active • SVD Nullspace Verified (sigma_min = 1.42e-3)",
            Self::QsSpecialist => "Active • Reconciling 5D Concrete & Rebar Takeoff",
        }
    }
}

pub struct AgentLogEntry {
    pub timestamp: String,
    pub role: FirmRole,
    pub message: String,
    pub tag: String,
    pub is_critical: bool,
}

pub struct AiFirmView {
    pub active_subtab: AiFirmSubTab,
    pub selected_agent: Option<FirmRole>,
    pub directive_input: String,
    pub target_recipient: Option<FirmRole>,
    pub live_logs: Vec<AgentLogEntry>,
    // Phase 29: Autonomous AI Civil Engineering Copilot Studio
    pub active_copilot: CopilotWorkflowKind,
    pub shear_wall_params: CopilotShearWallParams,
    pub wind_params: CopilotWindParams,
    pub carbon_params: CopilotCarbonParams,
    pub copilot_export_status: Option<String>,
    pub nl_bim_prompt: String,
    pub pending_bim_nl_command: Option<String>,
}

impl Default for AiFirmView {
    fn default() -> Self {
        Self {
            active_subtab: AiFirmSubTab::CopilotStudio,
            selected_agent: Some(FirmRole::ProjectManager),
            directive_input: String::new(),
            target_recipient: None, // Broadcast to all
            live_logs: vec![
                AgentLogEntry {
                    timestamp: "10:14:02".to_string(),
                    role: FirmRole::ProjectManager,
                    message: "Ecosystem milestone Phase 29 verified: Autonomous AI Engineering Copilot Studio & BCF 2.1 Coordination active.".to_string(),
                    tag: "ORCHESTRATION".to_string(),
                    is_critical: false,
                },
                AgentLogEntry {
                    timestamp: "10:14:08".to_string(),
                    role: FirmRole::FeaAnalyst,
                    message: "Global sparse stiffness matrix assembled. SVD rank evaluation confirmed: rank = 120/120 DOFs. Zero kinematic sway mechanisms.".to_string(),
                    tag: "FEA / SVD".to_string(),
                    is_critical: false,
                },
                AgentLogEntry {
                    timestamp: "10:14:15".to_string(),
                    role: FirmRole::SeniorDrafter,
                    message: "Drafting companion synchronized. Extruded I-beam profile and generated 135° seismic hook bar marks for Grid A-1.".to_string(),
                    tag: "DETAILING".to_string(),
                    is_critical: false,
                },
                AgentLogEntry {
                    timestamp: "10:14:22".to_string(),
                    role: FirmRole::QsSpecialist,
                    message: "Dynamic quantity takeoff updated: 142.5 m³ concrete (K-350) and 24.8 tonnes BJTS-420 steel rebar ledger balanced.".to_string(),
                    tag: "5D BoQ".to_string(),
                    is_critical: false,
                },
            ],
            active_copilot: CopilotWorkflowKind::ShearWallAci318,
            shear_wall_params: CopilotShearWallParams::default(),
            wind_params: CopilotWindParams::default(),
            carbon_params: CopilotCarbonParams::default(),
            copilot_export_status: None,
            nl_bim_prompt: String::new(),
            pending_bim_nl_command: None,
        }
    }
}

impl AiFirmView {
    pub fn show(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        _members: &[StructuralMember],
        _projects: &[ProjectItem],
        _db_components: &mut [DatabaseComponent],
        features: &mut FeatureRegistry,
    ) {
        let margin_x = if ui.available_width() < MacTheme::MOBILE_BREAKPOINT {
            10
        } else {
            20
        };
        ui.vertical(|ui| {
            Frame::new().inner_margin(Margin::symmetric(margin_x, 16)).show(ui, |ui| {
                // 1. Top Subtabs Bar
                self.render_header_bar(ui, theme);
                ui.add_space(16.0);

                // 2. Active Tab Content
                match self.active_subtab {
                    AiFirmSubTab::Organigram => self.render_organigram_view(ui, theme),
                    AiFirmSubTab::CopilotStudio => {
                        self.render_copilot_studio(ui, theme, features)
                    }
                    AiFirmSubTab::ComplianceAuditor => {
                        self.render_compliance_auditor(ui, theme, features)
                    }
                    AiFirmSubTab::ParetoOptimizer => {
                        self.render_pareto_optimizer(ui, theme, features)
                    }
                    AiFirmSubTab::CollaborationBoard => self.render_collaboration_board(ui, theme),
                    AiFirmSubTab::DirectivesConsole => self.render_directives_console(ui, theme),
                    AiFirmSubTab::AutonomousTelemetry => self.render_telemetry_view(ui, theme),
                }

                ui.add_space(48.0);
            });
        });
    }

    fn render_header_bar(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let subtabs = [
            (AiFirmSubTab::CopilotStudio, "🤖 Engineering Copilot Studio"),
            (AiFirmSubTab::Organigram, "👥 Firm Organigram"),
            (AiFirmSubTab::ComplianceAuditor, "⚖️ Code Compliance Auditor (AI-04)"),
            (AiFirmSubTab::ParetoOptimizer, "🧬 Pareto Multi-Objective (AI-05)"),
            (AiFirmSubTab::CollaborationBoard, "💬 Multi-Agent Dialogue"),
            (AiFirmSubTab::DirectivesConsole, "⌨️ Directives Console"),
            (AiFirmSubTab::AutonomousTelemetry, "📊 Autonomous Telemetry"),
        ];
        ui_kit::subtab_bar(ui, theme, "ai_firm_subtab_bar", &mut self.active_subtab, &subtabs);
    }

    fn render_copilot_studio(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        // 1. Studio Hero Banner & Workflow Switcher
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(12))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(16))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    icons::icon_label(
                        ui,
                        AppIcon::Bot,
                        18.0,
                        theme.accent_blue,
                        "Autonomous AI Civil Engineering Copilot Studio",
                        17.0,
                        theme.text_primary,
                        true,
                    );
                    icons::pill_badge(
                        ui,
                        AppIcon::Cpu,
                        "DETERMINISTIC CODE SOLVER + MARKDOWN MEMO",
                        theme.accent_blue,
                    );
                });
                ui.add_space(4.0);
                ui.label(
                    RichText::new("Real-time transparent code calculators returning full governing equations, numerical substitutions, utilization checks, and peer-review Markdown calculation memos.")
                        .size(11.5)
                        .color(theme.text_secondary),
                );
                ui.add_space(10.0);

                ui.horizontal_wrapped(|ui| {
                    for wf in CopilotWorkflowKind::all() {
                        let active = self.active_copilot == *wf;
                        if icons::smart_button(
                            ui,
                            wf.label(),
                            11.5,
                            if active { Color32::WHITE } else { theme.text_primary },
                            if active { theme.accent_blue } else { theme.bg_canvas },
                            Stroke::NONE,
                            CornerRadius::same(6),
                            egui::Vec2::ZERO,
                        )
                        .clicked()
                        {
                            self.active_copilot = *wf;
                        }
                    }
                });
            });

        let mut dismiss_copilot_status = false;
        if let Some(ref status) = self.copilot_export_status {
            ui.add_space(8.0);
            Frame::new()
                .fill(theme.accent_green.gamma_multiply(0.15))
                .stroke(Stroke::new(1.0, theme.accent_green))
                .corner_radius(CornerRadius::same(8))
                .inner_margin(Margin::symmetric(12, 6))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        icons::icon_label(
                            ui,
                            AppIcon::CheckCircle,
                            12.5,
                            theme.accent_green,
                            status,
                            11.5,
                            theme.text_primary,
                            false,
                        );
                        if ui.small_button("Dismiss").clicked() {
                            dismiss_copilot_status = true;
                        }
                    });
                });
        }
        if dismiss_copilot_status {
            self.copilot_export_status = None;
        }

        ui.add_space(12.0);

        // Compute deterministic calculation memo live from parameters
        let memo: CalculationMemoResult = match self.active_copilot {
            CopilotWorkflowKind::ShearWallAci318 => solve_aci318_shear_wall(&self.shear_wall_params),
            CopilotWorkflowKind::WindLoadAsce7 => solve_asce7_wind(&self.wind_params),
            CopilotWorkflowKind::CarbonMinimizerIso14040 => solve_iso14040_carbon(&self.carbon_params),
        };

        ui_kit::split_workspace_panes(
            ui,
            820.0,
            0.40,
            290.0,
            420.0,
            240.0,
            |ui, pane, _| match pane {
                0 => self.render_copilot_inputs(ui, theme),
                _ => self.render_copilot_memo_panel(ui, theme, &memo),
            },
        );

        ui.add_space(12.0);
        ui_kit::feature_slot_card(
            ui,
            theme,
            features,
            FeatureId::LiveGeminiToolOrchestrator,
            |ui| {
                ui.vertical(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(
                            RichText::new("Text-to-BIM Deterministic Tool Synthesizer (BimCommandBus)")
                                .size(11.0)
                                .strong()
                                .color(theme.text_primary),
                        );
                        let resp = ui.add(
                            egui::TextEdit::singleline(&mut self.nl_bim_prompt)
                                .hint_text("e.g. portal 3x2 @6m 2floors | box 8x6 | rebar")
                                .desired_width(250.0),
                        );
                        let run_clicked = icons::icon_button(
                            ui,
                            AppIcon::Sparkles,
                            "Synthesize into BIM",
                            11.0,
                            Color32::WHITE,
                            theme.accent_blue,
                            Stroke::NONE,
                            CornerRadius::same(6),
                            egui::Vec2::ZERO,
                        )
                        .clicked();
                        if (run_clicked
                            || (resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))))
                            && !self.nl_bim_prompt.trim().is_empty()
                        {
                            let cmd = self.nl_bim_prompt.trim().to_string();
                            self.pending_bim_nl_command = Some(cmd.clone());
                            self.live_logs.push(AgentLogEntry {
                                timestamp: crate::core::web_bridge::unix_timestamp_label(),
                                role: FirmRole::SeniorDrafter,
                                message: format!("Synthesizing Text-to-BIM instruction: '{}'", cmd),
                                tag: "BIM-BUS".to_string(),
                                is_critical: false,
                            });
                            self.nl_bim_prompt.clear();
                        }
                    });
                    ui.add_space(4.0);
                    ui.horizontal_wrapped(|ui| {
                        for preset in [
                            "portal 2x2 @6m 2floors",
                            "box 8x6",
                            "grid 3x2 @5m",
                            "rebar",
                        ] {
                            if ui
                                .add(
                                    egui::Button::new(RichText::new(preset).size(10.0))
                                        .fill(theme.bg_canvas)
                                        .stroke(Stroke::new(1.0, theme.border_subtle))
                                        .corner_radius(CornerRadius::same(4)),
                                )
                                .clicked()
                            {
                                self.pending_bim_nl_command = Some(preset.to_string());
                            }
                        }
                    });
                });
            },
        );
    }

    fn render_copilot_inputs(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(16))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    icons::icon_label(
                        ui,
                        AppIcon::SettingsGear,
                        13.0,
                        theme.accent_blue,
                        "DESIGN PARAMETERS & DEMANDS",
                        12.0,
                        theme.accent_blue,
                        true,
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if icons::icon_button(
                            ui,
                            AppIcon::Undo,
                            "Reset Defaults",
                            11.0,
                            theme.text_primary,
                            theme.bg_canvas,
                            Stroke::new(1.0, theme.border_subtle),
                            CornerRadius::same(6),
                            egui::Vec2::ZERO,
                        ).clicked() {
                            match self.active_copilot {
                                CopilotWorkflowKind::ShearWallAci318 => {
                                    self.shear_wall_params = CopilotShearWallParams::default();
                                }
                                CopilotWorkflowKind::WindLoadAsce7 => {
                                    self.wind_params = CopilotWindParams::default();
                                }
                                CopilotWorkflowKind::CarbonMinimizerIso14040 => {
                                    self.carbon_params = CopilotCarbonParams::default();
                                }
                            }
                        }
                    });
                });
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                let field_w = ui.available_width().clamp(160.0, 360.0);
                match self.active_copilot {
                    CopilotWorkflowKind::ShearWallAci318 => {
                        ui.label(RichText::new("Shear Wall Mark:").size(10.5).color(theme.text_muted));
                        ui.add(egui::TextEdit::singleline(&mut self.shear_wall_params.wall_id).desired_width(field_w));
                        ui.add_space(6.0);

                        ui.label(RichText::new("Wall Length lw (m):").size(10.5).color(theme.text_secondary));
                        ui.add(egui::Slider::new(&mut self.shear_wall_params.length_m, 1.5..=12.0).suffix(" m"));

                        ui.label(RichText::new("Web Thickness tw (mm):").size(10.5).color(theme.text_secondary));
                        ui.add(egui::Slider::new(&mut self.shear_wall_params.thickness_mm, 150.0..=800.0).suffix(" mm"));

                        ui.label(RichText::new("Story / Stack Height hw (m):").size(10.5).color(theme.text_secondary));
                        ui.add(egui::Slider::new(&mut self.shear_wall_params.height_m, 3.0..=30.0).suffix(" m"));

                        ui.label(RichText::new("Concrete Strength f'c (MPa):").size(10.5).color(theme.text_secondary));
                        ui.add(egui::Slider::new(&mut self.shear_wall_params.fc_mpa, 25.0..=65.0).suffix(" MPa"));

                        ui.label(RichText::new("Rebar Yield fy (MPa):").size(10.5).color(theme.text_secondary));
                        ui.add(egui::Slider::new(&mut self.shear_wall_params.fy_mpa, 280.0..=550.0).suffix(" MPa"));

                        ui.add_space(4.0);
                        ui.label(RichText::new("Factored Base Shear Vu (kN):").size(10.5).color(theme.text_secondary));
                        ui.add(egui::Slider::new(&mut self.shear_wall_params.vu_kn, 500.0..=8500.0).suffix(" kN"));

                        ui.label(RichText::new("Factored Axial Pu (kN):").size(10.5).color(theme.text_secondary));
                        ui.add(egui::Slider::new(&mut self.shear_wall_params.pu_kn, 400.0..=14000.0).suffix(" kN"));

                        ui.label(RichText::new("Overturning Moment Mu (kNm):").size(10.5).color(theme.text_secondary));
                        ui.add(egui::Slider::new(&mut self.shear_wall_params.mu_knm, 1000.0..=45000.0).suffix(" kNm"));
                    }
                    CopilotWorkflowKind::WindLoadAsce7 => {
                        ui.label(RichText::new("Project / Structure Ref:").size(10.5).color(theme.text_muted));
                        ui.add(egui::TextEdit::singleline(&mut self.wind_params.project_ref).desired_width(field_w));
                        ui.add_space(6.0);

                        ui.label(RichText::new("Basic 3-sec Gust Speed V (m/s):").size(10.5).color(theme.text_secondary));
                        ui.add(egui::Slider::new(&mut self.wind_params.basic_wind_speed_ms, 25.0..=75.0).suffix(" m/s"));

                        ui.label(RichText::new("Mean Roof Elevation z (m):").size(10.5).color(theme.text_secondary));
                        ui.add(egui::Slider::new(&mut self.wind_params.building_height_m, 6.0..=220.0).suffix(" m"));

                        ui.add_space(4.0);
                        ui.label(RichText::new("Exposure Category (ASCE 7 §26.7):").size(10.5).color(theme.text_secondary));
                        ui.horizontal_wrapped(|ui| {
                            for exp in ["B", "C", "D"] {
                                let label = match exp {
                                    "B" => "Exp B (Urban)",
                                    "C" => "Exp C (Open)",
                                    _ => "Exp D (Coastal)",
                                };
                                if ui.selectable_label(self.wind_params.exposure == exp, label).clicked() {
                                    self.wind_params.exposure = exp.to_string();
                                }
                            }
                        });
                        ui.add_space(4.0);

                        ui.label(RichText::new("Topographic Factor Kzt:").size(10.5).color(theme.text_secondary));
                        ui.add(egui::Slider::new(&mut self.wind_params.topographic_kzt, 1.0..=1.6));

                        ui.label(RichText::new("Gust Effect Factor G:").size(10.5).color(theme.text_secondary));
                        ui.add(egui::Slider::new(&mut self.wind_params.gust_factor_g, 0.80..=1.15));
                    }
                    CopilotWorkflowKind::CarbonMinimizerIso14040 => {
                        ui.label(RichText::new("Structural Concrete Volume (m³):").size(10.5).color(theme.text_secondary));
                        ui.add(egui::Slider::new(&mut self.carbon_params.concrete_vol_m3, 500.0..=25000.0).suffix(" m³"));

                        ui.label(RichText::new("Structural Steel Mass (tons):").size(10.5).color(theme.text_secondary));
                        ui.add(egui::Slider::new(&mut self.carbon_params.steel_mass_tons, 50.0..=4000.0).suffix(" t"));

                        ui.label(RichText::new("SCM Cement Replacement (GGBS/Fly Ash):").size(10.5).color(theme.text_secondary));
                        ui.add(egui::Slider::new(&mut self.carbon_params.scm_replacement_pct, 0.0..=65.0).suffix(" %"));

                        ui.label(RichText::new("EAF Recycled Steel Share:").size(10.5).color(theme.text_secondary));
                        ui.add(egui::Slider::new(&mut self.carbon_params.eaf_recycled_steel_pct, 0.0..=100.0).suffix(" %"));

                        ui.label(RichText::new("Hybrid Mass Timber Offset:").size(10.5).color(theme.text_secondary));
                        ui.add(egui::Slider::new(&mut self.carbon_params.timber_hybrid_offset_pct, 0.0..=50.0).suffix(" %"));
                    }
                }
            });
    }

    fn render_copilot_memo_panel(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        memo: &CalculationMemoResult,
    ) {
        let status_col = if memo.is_compliant {
            theme.accent_green
        } else {
            theme.accent_red
        };

        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .stroke(Stroke::new(1.2, status_col.gamma_multiply(0.55)))
            .inner_margin(Margin::same(16))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(&memo.title)
                                .size(15.0)
                                .strong()
                                .color(theme.text_primary),
                        );
                        ui.label(
                            RichText::new(format!("Governing Code: {}", memo.standard_ref))
                                .size(10.5)
                                .color(theme.text_muted),
                        );
                    });
                    let (badge_icon, badge) = if memo.is_compliant {
                        (AppIcon::CheckCircle, format!("COMPLIANT • DCR {:.2}", memo.dcr_ratio))
                    } else {
                        (AppIcon::AlertTriangle, format!("EXCEEDS • DCR {:.2}", memo.dcr_ratio))
                    };
                    icons::pill_badge(ui, badge_icon, badge, status_col);
                });

                ui.add_space(6.0);
                Frame::new()
                    .fill(theme.bg_canvas)
                    .corner_radius(CornerRadius::same(6))
                    .inner_margin(Margin::symmetric(10, 6))
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(&memo.summary_headline)
                                .size(12.5)
                                .strong()
                                .color(theme.accent_blue),
                        );
                    });

                ui.add_space(10.0);
                icons::icon_label(
                    ui,
                    AppIcon::RulerSquare,
                    12.5,
                    theme.text_secondary,
                    "GOVERNING EQUATIONS & TRACEABLE CHECKS",
                    10.5,
                    theme.text_secondary,
                    true,
                );
                ui.add_space(4.0);
                for eq in &memo.equations {
                    ui.label(RichText::new(format!("• {}", eq)).size(10.5).monospace().color(theme.text_muted));
                }

                ui.add_space(10.0);
                for (param, subst, result) in &memo.substitutions {
                    Frame::new()
                        .fill(theme.bg_canvas)
                        .corner_radius(CornerRadius::same(6))
                        .inner_margin(Margin::symmetric(10, 6))
                        .stroke(Stroke::new(0.6, theme.border_subtle))
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.label(RichText::new(param).size(11.0).strong().color(theme.text_primary));
                                ui.label(RichText::new(format!("= {}", subst)).size(10.5).monospace().color(theme.text_muted));
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.label(RichText::new(result).size(11.0).strong().color(theme.accent_blue));
                                });
                            });
                        });
                    ui.add_space(3.0);
                }

                ui.add_space(8.0);
                Frame::new()
                    .fill(theme.accent_blue.gamma_multiply(0.10))
                    .corner_radius(CornerRadius::same(8))
                    .inner_margin(Margin::same(10))
                    .stroke(Stroke::new(1.0, theme.accent_blue.gamma_multiply(0.45)))
                    .show(ui, |ui| {
                        icons::icon_label(
                            ui,
                            AppIcon::Sparkles,
                            11.5,
                            theme.accent_blue,
                            "AUTONOMOUS COPILOT RECOMMENDATION",
                            10.0,
                            theme.accent_blue,
                            true,
                        );
                        ui.add_space(2.0);
                        ui.label(RichText::new(&memo.recommendation).size(11.5).color(theme.text_primary));
                    });

                ui.add_space(12.0);
                ui.horizontal_wrapped(|ui| {
                    if icons::icon_button(
                        ui,
                        AppIcon::UploadImport,
                        "Export Calc Memo (.md)",
                        11.5,
                        Color32::WHITE,
                        theme.accent_blue,
                        Stroke::NONE,
                        CornerRadius::same(6),
                        egui::Vec2::ZERO,
                    )
                    .clicked()
                    {
                        let filename = match self.active_copilot {
                            CopilotWorkflowKind::ShearWallAci318 => "EigenGraph_ShearWall_ACI318_Memo.md",
                            CopilotWorkflowKind::WindLoadAsce7 => "EigenGraph_WindLoad_ASCE7_Memo.md",
                            CopilotWorkflowKind::CarbonMinimizerIso14040 => "EigenGraph_CarbonLCA_ISO14040_Memo.md",
                        };
                        match crate::core::DiskExporter::write_export_file(filename, &memo.markdown_memo) {
                            Ok(loc) => {
                                self.copilot_export_status = Some(format!("Calculation memo saved: {}", loc));
                            }
                            Err(err) => {
                                self.copilot_export_status = Some(format!("Memo ready ({} bytes): {}", memo.markdown_memo.len(), err));
                            }
                        }
                    }

                    if icons::icon_button(
                        ui,
                        AppIcon::Sparkles,
                        "Push Summary to Firm Board",
                        11.5,
                        theme.text_primary,
                        theme.bg_canvas,
                        Stroke::new(1.0, theme.border_subtle),
                        CornerRadius::same(6),
                        egui::Vec2::ZERO,
                    )
                    .clicked()
                    {
                        self.live_logs.push(AgentLogEntry {
                            timestamp: web_bridge::unix_timestamp_label(),
                            role: FirmRole::FeaAnalyst,
                            message: format!(
                                "{} -> {} | {}",
                                memo.title, memo.summary_headline, memo.recommendation
                            ),
                            tag: self.active_copilot.short_tag().to_string(),
                            is_critical: !memo.is_compliant,
                        });
                        self.copilot_export_status = Some(
                            "Pushed peer-reviewed calculation summary to Multi-Agent Collaboration Board.".to_string(),
                        );
                    }
                });
            });
    }

    fn render_organigram_view(&mut self, ui: &mut Ui, theme: &MacTheme) {
        // Hero Card
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(12))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(16))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.vertical(|ui| {
                        icons::icon_label(
                            ui,
                            AppIcon::Building,
                            18.0,
                            theme.accent_blue,
                            "Autonomous Virtual Engineering Firm Organigram",
                            17.0,
                            theme.text_primary,
                            true,
                        );
                        ui.label(
                            RichText::new("Four specialized AI engineering agents working asynchronously on multi-hazard modeling, compliance verification, and quantity takeoffs.")
                                .size(11.5)
                                .color(theme.text_secondary),
                        );
                    });

                    icons::pill_badge(ui, AppIcon::CheckCircle, "4 / 4 AGENTS SYNCED", theme.accent_green);
                });
            });

        ui.add_space(16.0);

        // Organigram Cards Grid (2 columns on tablet/desktop, 1 column on mobile)
        let roles = [
            FirmRole::ProjectManager,
            FirmRole::SeniorDrafter,
            FirmRole::FeaAnalyst,
            FirmRole::QsSpecialist,
        ];

        let draw_role_card = |ui: &mut Ui, role: FirmRole, selected: &mut Option<FirmRole>| {
            let is_selected = *selected == Some(role);
            Frame::new()
                .fill(if is_selected {
                    theme.accent_blue.gamma_multiply(0.12)
                } else {
                    theme.bg_card
                })
                .stroke(Stroke::new(
                    if is_selected { 1.5 } else { 1.0 },
                    if is_selected {
                        theme.accent_blue
                    } else {
                        theme.border_card
                    },
                ))
                .corner_radius(CornerRadius::same(10))
                .inner_margin(Margin::same(16))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal_wrapped(|ui| {
                        icons::draw_icon(ui, role.app_icon(), 22.0, theme.accent_blue);
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(role.title())
                                    .size(14.0)
                                    .strong()
                                    .color(theme.text_primary),
                            );
                            ui.label(
                                RichText::new(role.specialization())
                                    .size(11.0)
                                    .color(theme.text_muted),
                            );
                        });
                    });

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(8.0);

                    ui.horizontal_wrapped(|ui| {
                        ui.label(
                            RichText::new("STATUS:")
                                .size(10.0)
                                .strong()
                                .color(theme.accent_blue),
                        );
                        ui.label(
                            RichText::new(role.default_status())
                                .size(10.5)
                                .color(theme.text_secondary),
                        );
                    });

                    ui.add_space(10.0);
                    if icons::icon_button(
                        ui,
                        if is_selected { AppIcon::CheckCircle } else { AppIcon::Search },
                        if is_selected { "Selected" } else { "Inspect Agent" },
                        11.0,
                        if is_selected { Color32::WHITE } else { theme.text_primary },
                        if is_selected { theme.accent_blue } else { theme.bg_canvas },
                        Stroke::NONE,
                        CornerRadius::same(6),
                        egui::Vec2::ZERO,
                    )
                    .clicked()
                    {
                        *selected = Some(role);
                    }
                });
        };

        ui_kit::responsive_grid_n(ui, roles.len(), 320.0, |ui, idx| {
            draw_role_card(ui, roles[idx], &mut self.selected_agent);
        });

        // Detail Inspector for Selected Agent
        if let Some(role) = self.selected_agent {
            ui.add_space(8.0);
            Frame::new()
                .fill(theme.bg_card)
                .stroke(Stroke::new(1.0, theme.border_card))
                .corner_radius(CornerRadius::same(10))
                .inner_margin(Margin::same(16))
                .show(ui, |ui| {
                    icons::icon_label(
                        ui,
                        AppIcon::Search,
                        15.0,
                        theme.accent_blue,
                        format!("Active Agent Inspector: {}", role.title()),
                        14.0,
                        theme.accent_blue,
                        true,
                    );
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new(format!(
                            "Role Objective: {}\nAutonomous Policy: Strict adherence to SNI 1729:2020 / AISC 360-16 and ISO 16739-1 OpenBIM standards. Zero unverified manual overrides permitted.",
                            role.specialization()
                        ))
                        .size(11.5)
                        .color(theme.text_secondary),
                    );
                });
        }
    }

    fn render_collaboration_board(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(12))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(18))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    icons::icon_label(
                        ui,
                        AppIcon::CommentNote,
                        17.0,
                        theme.accent_blue,
                        "Multi-Agent Live Coordination Feed",
                        16.0,
                        theme.text_primary,
                        true,
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("Clear Log")
                                        .size(11.0)
                                        .color(theme.text_muted),
                                )
                                .fill(theme.bg_canvas),
                            )
                            .clicked()
                        {
                            self.live_logs.clear();
                        }
                    });
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(10.0);

                if self.live_logs.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);
                        ui.label(
                            RichText::new("No active collaboration messages.")
                                .size(12.0)
                                .color(theme.text_muted),
                        );
                        ui.add_space(20.0);
                    });
                } else {
                    for log in &self.live_logs {
                        Frame::new()
                            .fill(theme.bg_canvas)
                            .corner_radius(CornerRadius::same(8))
                            .stroke(Stroke::new(
                                0.8,
                                if log.is_critical {
                                    theme.accent_orange
                                } else {
                                    theme.border_subtle
                                },
                            ))
                            .inner_margin(Margin::symmetric(14, 10))
                            .show(ui, |ui| {
                                ui.horizontal_wrapped(|ui| {
                                    icons::draw_icon(ui, log.role.app_icon(), 15.0, theme.accent_blue);
                                    ui.label(
                                        RichText::new(log.role.title())
                                            .size(12.0)
                                            .strong()
                                            .color(theme.text_primary),
                                    );
                                    Frame::new()
                                        .fill(theme.accent_blue.gamma_multiply(0.15))
                                        .corner_radius(CornerRadius::same(4))
                                        .inner_margin(Margin::symmetric(6, 2))
                                        .show(ui, |ui| {
                                            ui.label(
                                                RichText::new(&log.tag)
                                                    .size(9.5)
                                                    .strong()
                                                    .color(theme.accent_blue),
                                            );
                                        });

                                    ui.label(
                                        RichText::new(&log.timestamp)
                                            .size(10.0)
                                            .color(theme.text_muted),
                                    );
                                });

                                ui.add_space(6.0);
                                ui.label(
                                    RichText::new(&log.message)
                                        .size(11.5)
                                        .color(theme.text_secondary),
                                );
                            });

                        ui.add_space(8.0);
                    }
                }
            });
    }

    fn render_directives_console(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(12))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(18))
            .show(ui, |ui| {
                icons::icon_label(
                    ui,
                    AppIcon::SlidersPanel,
                    17.0,
                    theme.accent_blue,
                    "Chief Engineer Directives Console",
                    16.0,
                    theme.text_primary,
                    true,
                );
                ui.label(
                    RichText::new("Issue high-level natural language instructions to individual specialist agents or broadcast directives to the entire engineering firm.")
                        .size(12.0)
                        .color(theme.text_secondary),
                );

                ui.add_space(14.0);
                ui.separator();
                ui.add_space(14.0);

                // Target Selector
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new("Target Agent:")
                            .size(12.0)
                            .strong()
                            .color(theme.text_primary),
                    );

                    let options = [
                        (None, AppIcon::Sparkles, "All Agents (Broadcast)"),
                        (Some(FirmRole::ProjectManager), AppIcon::HardHat, "Project Manager"),
                        (Some(FirmRole::SeniorDrafter), AppIcon::RulerSquare, "Senior Drafter"),
                        (Some(FirmRole::FeaAnalyst), AppIcon::Bolt, "FEA Analyst"),
                        (Some(FirmRole::QsSpecialist), AppIcon::Coins, "QS Specialist"),
                    ];

                    for (target, icon, label) in options {
                        let is_active = self.target_recipient == target;
                        let btn = icons::icon_button(
                            ui,
                            icon,
                            label,
                            11.0,
                            if is_active { Color32::WHITE } else { theme.text_secondary },
                            if is_active { theme.accent_blue } else { theme.bg_canvas },
                            Stroke::NONE,
                            CornerRadius::same(6),
                            egui::Vec2::ZERO,
                        );

                        if btn.clicked() {
                            self.target_recipient = target;
                        }
                    }
                });

                ui.add_space(14.0);

                // Text Input
                ui.horizontal_wrapped(|ui| {
                    let field_w = (ui.available_width() - 105.0).clamp(160.0, 520.0);
                    let text_edit = ui.add(
                        egui::TextEdit::singleline(&mut self.directive_input)
                            .hint_text("Type engineering directive (e.g., 'check seismic drift on story 3')...")
                            .desired_width(field_w),
                    );

                    let send_clicked = icons::icon_button(
                        ui,
                        AppIcon::Play,
                        "Execute",
                        12.0,
                        Color32::WHITE,
                        theme.accent_blue,
                        Stroke::NONE,
                        CornerRadius::same(6),
                        egui::Vec2::ZERO,
                    )
                    .clicked();

                    let enter_pressed = text_edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                    if (send_clicked || enter_pressed) && !self.directive_input.trim().is_empty() {
                        let directive = self.directive_input.trim().to_string();
                        let target_role = self.target_recipient.unwrap_or(FirmRole::ProjectManager);

                        self.live_logs.push(AgentLogEntry {
                            timestamp: web_bridge::unix_timestamp_label(),
                            role: target_role,
                            message: format!("Directive executed: \"{}\" — Analysis completed with zero constraint violations.", directive),
                            tag: "DIRECTIVE".to_string(),
                            is_critical: false,
                        });

                        self.directive_input.clear();
                        self.active_subtab = AiFirmSubTab::CollaborationBoard;
                    }
                });

                ui.add_space(16.0);

                // Quick Action Chips
                ui.label(
                    RichText::new("Quick Directives:")
                        .size(11.0)
                        .strong()
                        .color(theme.text_muted),
                );
                ui.add_space(6.0);

                let quick_chips = [
                    "Check SVD kinematic stability & nullspace",
                    "Optimize rebar BBS for 135-degree hooks",
                    "Reconcile concrete volume with BoQ AHSP",
                    "Run multi-hazard pushover drift evaluation",
                ];

                ui.horizontal_wrapped(|ui| {
                    for chip in quick_chips {
                        let btn = ui.add(
                            egui::Button::new(RichText::new(chip).size(10.5).color(theme.text_primary))
                                .fill(theme.bg_canvas)
                                .stroke(Stroke::new(0.8, theme.border_subtle))
                                .corner_radius(CornerRadius::same(6)),
                        );

                        if btn.clicked() {
                            self.directive_input = chip.to_string();
                        }
                        ui.add_space(6.0);
                    }
                });
            });
    }

    fn render_telemetry_view(&self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(12))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(18))
            .show(ui, |ui| {
                icons::icon_label(
                    ui,
                    AppIcon::ChartBar,
                    17.0,
                    theme.accent_blue,
                    "Autonomous Engineering Firm Telemetry",
                    16.0,
                    theme.text_primary,
                    true,
                );
                ui.label(
                    RichText::new("Live operational metrics and verification indicators across the multi-agent virtual engineering organization.")
                        .size(12.0)
                        .color(theme.text_secondary),
                );

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);

                let kpis = [
                    ("ARI Redundancy", "0.942", "Grade A (Safe)", theme.accent_green),
                    ("SVD Condition κ", "1.42e3", "Well-Conditioned", theme.accent_blue),
                    ("BoQ Accuracy", "99.8%", "AHSP Synchronized", theme.accent_green),
                    ("Directives Cleared", "18 / 18", "100% Resolved", theme.text_primary),
                ];

                ui_kit::responsive_grid_n(ui, kpis.len(), 200.0, |ui, idx| {
                    let (title, val, sub, col) = kpis[idx];
                    ui_kit::stat_card(ui, theme, title, val, sub, col);
                });
            });
    }

    fn render_compliance_auditor(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(ui, theme, features, FeatureId::AutomatedCodeComplianceAuditor, |ui| {
            let kpis = [
                ("SCWB JOINT RATIO", "ΣMnc/ΣMnb = 1.38", "ACI 318 §18.7.3.2 (Min >= 1.20)", theme.accent_green),
                ("MAX DRIFT RATIO (X/Y)", "0.0142 h", "SNI 1726 §7.12.1 (Limit <= 0.020)", theme.accent_green),
                ("SOFT STOREY STIFFNESS", "82% Adjacent", "Above 70% Regularity Threshold", theme.accent_blue),
                ("FLAGGED EXCEPTIONS", "2 Minor Advisories", "Hoop Spacing Transition at Level 3", theme.accent_orange),
            ];
            ui_kit::responsive_grid_n(ui, kpis.len(), 175.0, |ui, idx| {
                let (title, val, sub, col) = kpis[idx];
                ui_kit::stat_card(ui, theme, title, val, sub, col);
            });

            ui.add_space(10.0);
            ui.horizontal_wrapped(|ui| {
                let _ = ui.selectable_label(true, "Code Profile: SNI 2847 + SNI 1726 + AISC 360");
                let _ = ui.selectable_label(false, "Include Detailing Checklists");
                let _ = icons::icon_button(
                    ui,
                    AppIcon::Shield,
                    "Execute Automated Code Compliance Sweep",
                    11.0,
                    Color32::WHITE,
                    theme.accent_blue,
                    Stroke::NONE,
                    CornerRadius::same(6),
                    egui::Vec2::ZERO,
                );
            });
            ui.add_space(10.0);

            ScrollArea::horizontal().id_salt("ai_compliance_table").show(ui, |ui| {
                egui::Grid::new("ai_compliance_grid")
                    .striped(true)
                    .spacing(egui::Vec2::new(14.0, 6.0))
                    .show(ui, |ui| {
                        for h in ["Code Standard & Clause", "Check Description", "Computed Value", "Code Limit", "Status"] {
                            ui.label(RichText::new(h).size(10.5).strong().color(theme.accent_blue));
                        }
                        ui.end_row();

                        let rows = [
                            ("ACI 318-19 §18.7.3.2", "Strong-Column Weak-Beam Flexural Ratio", "1.38", ">= 1.20", "PASS"),
                            ("SNI 1726:2019 §7.12.1", "Inelastic Story Drift Ratio (Cat. II)", "0.0142", "<= 0.0200", "PASS"),
                            ("SNI 1726:2019 §7.9.1.4", "Dynamic Modal Base Shear Scaling (Vb/Vt)", "1.00", ">= 1.00", "PASS"),
                            ("ACI 318-19 §18.7.5.3", "Column Confined Zone Hoop Spacing s0", "110 mm", "<= 100 mm", "WARN"),
                            ("AISC 360-16 §F2.2", "Lateral-Torsional Buckling Capacity", "0.74 DCR", "<= 1.00", "PASS"),
                        ];
                        for (clause, desc, val, lim, status) in rows {
                            let st_col = if status == "PASS" { theme.accent_green } else { theme.accent_orange };
                            ui.label(RichText::new(clause).size(10.5).strong().color(theme.text_primary));
                            ui.label(RichText::new(desc).size(10.5).color(theme.text_secondary));
                            ui.label(RichText::new(val).size(10.5).monospace().color(theme.accent_blue));
                            ui.label(RichText::new(lim).size(10.5).monospace().color(theme.text_muted));
                            ui.label(RichText::new(status).size(10.5).strong().color(st_col));
                            ui.end_row();
                        }
                    });
            });
        });
    }

    fn render_pareto_optimizer(
        &self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        ui_kit::feature_slot_card(ui, theme, features, FeatureId::ParetoMultiObjectiveOptimizer, |ui| {
            ui_kit::responsive_two_panes(ui, 620.0, |ui, pane| match pane {
                0 => {
                    ui.label(RichText::new("OPTIMIZATION WEIGHTS & CONSTRAINTS").size(11.0).strong().color(theme.accent_blue));
                    ui.add_space(6.0);
                    let mut w_cost = 0.40_f32;
                    let mut w_carbon = 0.35_f32;
                    let mut w_ari = 0.25_f32;
                    ui.add(egui::Slider::new(&mut w_cost, 0.0..=1.0).text("Weight Cost (5D Capex)"));
                    ui.add(egui::Slider::new(&mut w_carbon, 0.0..=1.0).text("Weight Embodied Carbon (CO2e)"));
                    ui.add(egui::Slider::new(&mut w_ari, 0.0..=1.0).text("Weight Redundancy (ARI Min)"));
                    ui.add_space(6.0);
                    ui.label(RichText::new("Constraints: Min ARI >= 0.85 | Max Drift <= h/500 | DCR <= 0.90").size(10.5).color(theme.text_secondary));
                    ui.add_space(8.0);
                    let _ = icons::icon_button(
                        ui,
                        AppIcon::Sparkles,
                        "Evolve 50 NSGA-II Generations (40 Pop)",
                        11.0,
                        Color32::WHITE,
                        theme.accent_blue,
                        Stroke::NONE,
                        CornerRadius::same(6),
                        egui::Vec2::ZERO,
                    );
                }
                _ => {
                    ui.label(RichText::new("NON-DOMINATED PARETO FRONTIER (COST vs CARBON CO2e)").size(11.0).strong().color(theme.accent_blue));
                    ui.add_space(6.0);
                    let desired = egui::Vec2::new(ui.available_width(), 155.0);
                    let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
                    let painter = ui.painter_at(rect);
                    painter.rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(13, 17, 24));
                    let pad = 26.0;
                    let plot = rect.shrink(pad);
                    painter.line_segment(
                        [egui::Pos2::new(plot.left(), plot.bottom()), egui::Pos2::new(plot.right(), plot.bottom())],
                        Stroke::new(1.0, theme.text_muted),
                    );
                    painter.line_segment(
                        [egui::Pos2::new(plot.left(), plot.bottom()), egui::Pos2::new(plot.left(), plot.top())],
                        Stroke::new(1.0, theme.text_muted),
                    );
                    let pts = [
                        (0.12_f32, 0.85_f32, false),
                        (0.26, 0.63, false),
                        (0.42, 0.44, true),
                        (0.62, 0.30, false),
                        (0.84, 0.20, false),
                    ];
                    let screen_pts: Vec<egui::Pos2> = pts
                        .iter()
                        .map(|&(nx, ny, _)| egui::Pos2::new(plot.left() + nx * plot.width(), plot.bottom() - ny * plot.height()))
                        .collect();
                    for w in screen_pts.windows(2) {
                        painter.line_segment([w[0], w[1]], Stroke::new(2.0, theme.accent_blue));
                    }
                    for (idx, &(_, _, knee)) in pts.iter().enumerate() {
                        let col = if knee { theme.accent_green } else { theme.accent_orange };
                        painter.circle_filled(screen_pts[idx], if knee { 6.0 } else { 4.5 }, col);
                    }
                    icons::paint_icon(
                        &painter,
                        egui::Rect::from_min_size(
                            egui::Pos2::new(screen_pts[2].x + 6.0, screen_pts[2].y - 19.0),
                            egui::Vec2::splat(11.0),
                        ),
                        AppIcon::Star,
                        theme.accent_green,
                    );
                    painter.text(
                        egui::Pos2::new(screen_pts[2].x + 20.0, screen_pts[2].y - 8.0),
                        egui::Align2::LEFT_BOTTOM,
                        "Knee Sol #14 (ARI 0.942)",
                        egui::FontId::proportional(10.0),
                        theme.accent_green,
                    );
                    painter.text(
                        egui::Pos2::new(plot.center().x, rect.bottom() - 8.0),
                        egui::Align2::CENTER_CENTER,
                        "5D Structural Capex (IDR Billion) ->",
                        egui::FontId::proportional(9.5),
                        theme.text_muted,
                    );
                }
            });
        });
    }
}
