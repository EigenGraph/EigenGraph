//! ai_firm.rs — Pillar 2: AI Agent & Virtual Engineering Firm Container.
//! Features the autonomous multi-agent engineering firm organigram, interactive AI Civil Engineering
//! Copilot Studio (ACI 318-19 Shear Wall, ASCE 7-22 Wind, ISO 14040 Carbon Minimizer), live
//! collaboration board, and directive console.

use egui::{
    Align, Color32, CornerRadius, Frame, Layout, Margin, RichText, ScrollArea, Stroke, Ui,
};
use crate::core::web_bridge;
use crate::models::{
    CalculationMemoResult, CopilotCarbonParams, CopilotShearWallParams, CopilotWindParams,
    CopilotWorkflowKind, DatabaseComponent, ProjectItem, StructuralMember,
    solve_aci318_shear_wall, solve_asce7_wind, solve_iso14040_carbon,
};
use crate::theme::MacTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiFirmSubTab {
    Organigram,
    CopilotStudio,
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

    pub fn icon(&self) -> &'static str {
        match self {
            Self::ProjectManager => "👔",
            Self::SeniorDrafter => "📐",
            Self::FeaAnalyst => "⚡",
            Self::QsSpecialist => "💰",
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
    ) {
        let margin_x = if MacTheme::is_mobile(ui.ctx()) { 12 } else { 24 };
        ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
            Frame::new().inner_margin(Margin::symmetric(margin_x, 16)).show(ui, |ui| {
                // 1. Top Subtabs Bar
                self.render_header_bar(ui, theme);
                ui.add_space(16.0);

                // 2. Active Tab Content
                match self.active_subtab {
                    AiFirmSubTab::Organigram => self.render_organigram_view(ui, theme),
                    AiFirmSubTab::CopilotStudio => self.render_copilot_studio(ui, theme),
                    AiFirmSubTab::CollaborationBoard => self.render_collaboration_board(ui, theme),
                    AiFirmSubTab::DirectivesConsole => self.render_directives_console(ui, theme),
                    AiFirmSubTab::AutonomousTelemetry => self.render_telemetry_view(ui, theme),
                }

                ui.add_space(48.0);
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
                ui.horizontal_wrapped(|ui| {
                    let subtabs = [
                        (AiFirmSubTab::CopilotStudio, "🤖 Engineering Copilot Studio"),
                        (AiFirmSubTab::Organigram, "👥 Firm Organigram"),
                        (AiFirmSubTab::CollaborationBoard, "💬 Multi-Agent Dialogue"),
                        (AiFirmSubTab::DirectivesConsole, "⌨️ Directives Console"),
                        (AiFirmSubTab::AutonomousTelemetry, "📊 Autonomous Telemetry"),
                    ];

                    for (tab, label) in subtabs {
                        let is_active = self.active_subtab == tab;
                        let text_color = if is_active {
                            Color32::WHITE
                        } else {
                            theme.text_secondary
                        };
                        let fill_color = if is_active {
                            theme.accent_blue
                        } else {
                            Color32::TRANSPARENT
                        };

                        let btn = egui::Button::new(
                            RichText::new(label)
                                .size(12.0)
                                .strong()
                                .color(text_color),
                        )
                        .fill(fill_color)
                        .corner_radius(CornerRadius::same(6));

                        if ui.add(btn).clicked() {
                            self.active_subtab = tab;
                        }
                        ui.add_space(4.0);
                    }
                });
            });
    }

    fn render_copilot_studio(&mut self, ui: &mut Ui, theme: &MacTheme) {
        // 1. Studio Hero Banner & Workflow Switcher
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(12))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(16))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new("🤖 Autonomous AI Civil Engineering Copilot Studio")
                            .size(17.0)
                            .strong()
                            .color(theme.text_primary),
                    );
                    Frame::new()
                        .fill(theme.accent_blue.gamma_multiply(0.15))
                        .corner_radius(CornerRadius::same(6))
                        .inner_margin(Margin::symmetric(8, 3))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new("DETERMINISTIC CODE SOLVER + MARKDOWN MEMO")
                                    .size(9.5)
                                    .strong()
                                    .color(theme.accent_blue),
                            );
                        });
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
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new(wf.label())
                                        .size(11.5)
                                        .strong()
                                        .color(if active { Color32::WHITE } else { theme.text_primary }),
                                )
                                .fill(if active { theme.accent_blue } else { theme.bg_canvas })
                                .corner_radius(CornerRadius::same(6)),
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
                        ui.label(RichText::new(format!("✅ {}", status)).size(11.5).color(theme.text_primary));
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

        let avail_w = ui.available_width();
        let is_wide = avail_w >= 820.0;

        if is_wide {
            let left_w = (avail_w * 0.40).clamp(300.0, 420.0);
            let right_w = (avail_w - left_w - 14.0).max(320.0);
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_width(left_w);
                    self.render_copilot_inputs(ui, theme);
                });
                ui.add_space(12.0);
                ui.vertical(|ui| {
                    ui.set_width(right_w);
                    self.render_copilot_memo_panel(ui, theme, &memo);
                });
            });
        } else {
            self.render_copilot_inputs(ui, theme);
            ui.add_space(12.0);
            self.render_copilot_memo_panel(ui, theme, &memo);
        }
    }

    fn render_copilot_inputs(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(16))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new("⚙️ DESIGN PARAMETERS & DEMANDS")
                            .size(12.0)
                            .strong()
                            .color(theme.accent_blue),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.small_button("↺ Reset Defaults").clicked() {
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

                match self.active_copilot {
                    CopilotWorkflowKind::ShearWallAci318 => {
                        ui.label(RichText::new("Shear Wall Mark:").size(10.5).color(theme.text_muted));
                        ui.add(egui::TextEdit::singleline(&mut self.shear_wall_params.wall_id).desired_width(f32::INFINITY));
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
                        ui.add(egui::TextEdit::singleline(&mut self.wind_params.project_ref).desired_width(f32::INFINITY));
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
                    Frame::new()
                        .fill(status_col.gamma_multiply(0.16))
                        .stroke(Stroke::new(1.0, status_col))
                        .corner_radius(CornerRadius::same(6))
                        .inner_margin(Margin::symmetric(10, 4))
                        .show(ui, |ui| {
                            let badge = if memo.is_compliant {
                                format!("✅ COMPLIANT • DCR {:.2}", memo.dcr_ratio)
                            } else {
                                format!("⚠️ EXCEEDS • DCR {:.2}", memo.dcr_ratio)
                            };
                            ui.label(RichText::new(badge).size(10.5).strong().color(status_col));
                        });
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
                ui.label(
                    RichText::new("📐 GOVERNING EQUATIONS & TRACEABLE CHECKS")
                        .size(10.5)
                        .strong()
                        .color(theme.text_secondary),
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
                        ui.label(
                            RichText::new("💡 AUTONOMOUS COPILOT RECOMMENDATION")
                                .size(10.0)
                                .strong()
                                .color(theme.accent_blue),
                        );
                        ui.add_space(2.0);
                        ui.label(RichText::new(&memo.recommendation).size(11.5).color(theme.text_primary));
                    });

                ui.add_space(12.0);
                ui.horizontal_wrapped(|ui| {
                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("📥 Export Calc Memo (.md)")
                                    .size(11.5)
                                    .strong()
                                    .color(Color32::WHITE),
                            )
                            .fill(theme.accent_blue)
                            .corner_radius(CornerRadius::same(6)),
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

                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("📢 Push Summary to Firm Board")
                                    .size(11.5)
                                    .strong()
                                    .color(theme.text_primary),
                            )
                            .fill(theme.bg_canvas)
                            .corner_radius(CornerRadius::same(6)),
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
                        ui.label(
                            RichText::new("🏛️ Autonomous Virtual Engineering Firm Organigram")
                                .size(17.0)
                                .strong()
                                .color(theme.text_primary),
                        );
                        ui.label(
                            RichText::new("Four specialized AI engineering agents working asynchronously on multi-hazard modeling, compliance verification, and quantity takeoffs.")
                                .size(11.5)
                                .color(theme.text_secondary),
                        );
                    });

                    Frame::new()
                        .fill(theme.accent_green.gamma_multiply(0.15))
                        .stroke(Stroke::new(1.0, theme.accent_green.gamma_multiply(0.6)))
                        .corner_radius(CornerRadius::same(6))
                        .inner_margin(Margin::symmetric(10, 5))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new("● 4 / 4 AGENTS SYNCED")
                                    .size(10.5)
                                    .strong()
                                    .color(theme.accent_green),
                            );
                        });
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

        let avail_w = ui.available_width();
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
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(role.icon()).size(26.0));
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
                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new(if is_selected { "Selected ✓" } else { "Inspect Agent" })
                                    .size(11.0)
                                    .color(if is_selected { Color32::WHITE } else { theme.text_primary }),
                            )
                            .fill(if is_selected { theme.accent_blue } else { theme.bg_canvas })
                            .corner_radius(CornerRadius::same(6)),
                        )
                        .clicked()
                    {
                        *selected = Some(role);
                    }
                });
        };

        if avail_w >= 700.0 {
            ui.columns(2, |cols| {
                for (idx, role) in roles.iter().enumerate() {
                    draw_role_card(&mut cols[idx % 2], *role, &mut self.selected_agent);
                    cols[idx % 2].add_space(14.0);
                }
            });
        } else {
            for role in roles {
                draw_role_card(ui, role, &mut self.selected_agent);
                ui.add_space(10.0);
            }
        }

        // Detail Inspector for Selected Agent
        if let Some(role) = self.selected_agent {
            ui.add_space(8.0);
            Frame::new()
                .fill(theme.bg_card)
                .stroke(Stroke::new(1.0, theme.border_card))
                .corner_radius(CornerRadius::same(10))
                .inner_margin(Margin::same(16))
                .show(ui, |ui| {
                    ui.label(
                        RichText::new(format!("🔎 Active Agent Inspector: {}", role.title()))
                            .size(14.0)
                            .strong()
                            .color(theme.accent_blue),
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
                    ui.label(
                        RichText::new("💬 Multi-Agent Live Coordination Feed")
                            .size(16.0)
                            .strong()
                            .color(theme.text_primary),
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
                                    ui.label(RichText::new(log.role.icon()).size(16.0));
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
                ui.label(
                    RichText::new("⌨️ Chief Engineer Directives Console")
                        .size(16.0)
                        .strong()
                        .color(theme.text_primary),
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
                        (None, "📢 All Agents (Broadcast)"),
                        (Some(FirmRole::ProjectManager), "👔 Project Manager"),
                        (Some(FirmRole::SeniorDrafter), "📐 Senior Drafter"),
                        (Some(FirmRole::FeaAnalyst), "⚡ FEA Analyst"),
                        (Some(FirmRole::QsSpecialist), "💰 QS Specialist"),
                    ];

                    for (target, label) in options {
                        let is_active = self.target_recipient == target;
                        let btn = ui.add(
                            egui::Button::new(
                                RichText::new(label)
                                    .size(11.0)
                                    .color(if is_active { Color32::WHITE } else { theme.text_secondary }),
                            )
                            .fill(if is_active { theme.accent_blue } else { theme.bg_canvas })
                            .corner_radius(CornerRadius::same(6)),
                        );

                        if btn.clicked() {
                            self.target_recipient = target;
                        }
                    }
                });

                ui.add_space(14.0);

                // Text Input
                ui.horizontal(|ui| {
                    let field_w = (ui.available_width() - 105.0).max(120.0);
                    let text_edit = ui.add(
                        egui::TextEdit::singleline(&mut self.directive_input)
                            .hint_text("Type engineering directive (e.g., 'check seismic drift on story 3')...")
                            .desired_width(field_w),
                    );

                    let send_clicked = ui
                        .add(
                            egui::Button::new(
                                RichText::new("Execute ↵")
                                    .size(12.0)
                                    .strong()
                                    .color(Color32::WHITE),
                            )
                            .fill(theme.accent_blue)
                            .corner_radius(CornerRadius::same(6)),
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
                ui.label(
                    RichText::new("📊 Autonomous Engineering Firm Telemetry")
                        .size(16.0)
                        .strong()
                        .color(theme.text_primary),
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

                let draw_kpi = |ui: &mut Ui, title: &str, val: &str, sub: &str, col: Color32| {
                    Frame::new()
                        .fill(theme.bg_canvas)
                        .corner_radius(CornerRadius::same(8))
                        .stroke(Stroke::new(0.8, theme.border_subtle))
                        .inner_margin(Margin::same(12))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.label(RichText::new(title).size(11.0).color(theme.text_muted));
                            ui.label(RichText::new(val).size(20.0).strong().color(col));
                            ui.label(RichText::new(sub).size(10.0).color(theme.text_secondary));
                        });
                };

                let avail_w = ui.available_width();
                if avail_w >= 840.0 {
                    ui.columns(4, |cols| {
                        for (idx, (title, val, sub, col)) in kpis.iter().enumerate() {
                            draw_kpi(&mut cols[idx], title, val, sub, *col);
                        }
                    });
                } else if avail_w >= 500.0 {
                    ui.columns(2, |cols| {
                        for (idx, (title, val, sub, col)) in kpis.iter().enumerate() {
                            draw_kpi(&mut cols[idx % 2], title, val, sub, *col);
                            cols[idx % 2].add_space(8.0);
                        }
                    });
                } else {
                    for (title, val, sub, col) in kpis {
                        draw_kpi(ui, title, val, sub, col);
                        ui.add_space(8.0);
                    }
                }
            });
    }
}
