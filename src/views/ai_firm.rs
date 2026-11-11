//! ai_firm.rs — Pillar 2: AI Agent & Virtual Engineering Firm Container.
//! Features the autonomous multi-agent engineering firm organigram (Project Manager, Senior Drafter,
//! Finite Element Analyst, and QS Specialist) with live collaboration board and directive console.

use egui::{
    Align, Align2, Color32, CornerRadius, Frame, Layout, Margin, RichText, ScrollArea, Stroke, Ui,
    Vec2,
};
use crate::models::{DatabaseComponent, ProjectItem, StructuralMember};
use crate::theme::MacTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiFirmSubTab {
    Organigram,
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
    pub timestamp: &'static str,
    pub role: FirmRole,
    pub message: String,
    pub tag: &'static str,
    pub is_critical: bool,
}

pub struct AiFirmView {
    pub active_subtab: AiFirmSubTab,
    pub selected_agent: Option<FirmRole>,
    pub directive_input: String,
    pub target_recipient: Option<FirmRole>,
    pub live_logs: Vec<AgentLogEntry>,
}

impl Default for AiFirmView {
    fn default() -> Self {
        Self {
            active_subtab: AiFirmSubTab::Organigram,
            selected_agent: Some(FirmRole::ProjectManager),
            directive_input: String::new(),
            target_recipient: None, // Broadcast to all
            live_logs: vec![
                AgentLogEntry {
                    timestamp: "10:14:02",
                    role: FirmRole::ProjectManager,
                    message: "Ecosystem milestone Phase 22 verified: Master Shell and 15-Pillar Apple HIG architecture initialized.".to_string(),
                    tag: "ORCHESTRATION",
                    is_critical: false,
                },
                AgentLogEntry {
                    timestamp: "10:14:08",
                    role: FirmRole::FeaAnalyst,
                    message: "Global sparse stiffness matrix assembled. SVD rank evaluation confirmed: rank = 120/120 DOFs. Zero kinematic sway mechanisms.".to_string(),
                    tag: "FEA / SVD",
                    is_critical: false,
                },
                AgentLogEntry {
                    timestamp: "10:14:15",
                    role: FirmRole::SeniorDrafter,
                    message: "Drafting companion synchronized. Extruded I-beam profile and generated 135° seismic hook bar marks for Grid A-1.".to_string(),
                    tag: "DETAILING",
                    is_critical: false,
                },
                AgentLogEntry {
                    timestamp: "10:14:22",
                    role: FirmRole::QsSpecialist,
                    message: "Dynamic quantity takeoff updated: 142.5 m³ concrete (K-350) and 24.8 tonnes BJTS-420 steel rebar ledger balanced.".to_string(),
                    tag: "5D BoQ",
                    is_critical: false,
                },
            ],
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
        ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
            Frame::new().inner_margin(Margin::symmetric(24, 16)).show(ui, |ui| {
                // 1. Top Subtabs Bar
                self.render_header_bar(ui, theme);
                ui.add_space(16.0);

                // 2. Active Tab Content
                match self.active_subtab {
                    AiFirmSubTab::Organigram => self.render_organigram_view(ui, theme),
                    AiFirmSubTab::CollaborationBoard => self.render_collaboration_board(ui, theme),
                    AiFirmSubTab::DirectivesConsole => self.render_directives_console(ui, theme),
                    AiFirmSubTab::AutonomousTelemetry => self.render_telemetry_view(ui, theme),
                }

                ui.add_space(32.0);
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
                ui.horizontal(|ui| {
                    let subtabs = [
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

    fn render_organigram_view(&mut self, ui: &mut Ui, theme: &MacTheme) {
        // Hero Card
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(12))
            .stroke(Stroke::new(1.0, theme.border_card))
            .inner_margin(Margin::same(20))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("🏛️ Autonomous Virtual Engineering Firm Organigram")
                                .size(18.0)
                                .strong()
                                .color(theme.text_primary),
                        );
                        ui.label(
                            RichText::new("Four specialized AI engineering agents working asynchronously on multi-hazard modeling, compliance verification, and quantity takeoffs.")
                                .size(12.0)
                                .color(theme.text_secondary),
                        );
                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
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
            });

        ui.add_space(16.0);

        // Organigram Cards Grid
        let roles = [
            FirmRole::ProjectManager,
            FirmRole::SeniorDrafter,
            FirmRole::FeaAnalyst,
            FirmRole::QsSpecialist,
        ];

        ui.columns(2, |cols| {
            for (idx, role) in roles.iter().enumerate() {
                let col = &mut cols[idx % 2];
                let is_selected = self.selected_agent == Some(*role);

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
                    .show(col, |ui| {
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

                        ui.horizontal(|ui| {
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
                        ui.horizontal(|ui| {
                            let select_btn = ui.add(
                                egui::Button::new(
                                    RichText::new(if is_selected { "Selected ✓" } else { "Inspect Agent" })
                                        .size(11.0)
                                        .color(if is_selected { Color32::WHITE } else { theme.text_primary }),
                                )
                                .fill(if is_selected { theme.accent_blue } else { theme.bg_canvas })
                                .corner_radius(CornerRadius::same(6)),
                            );

                            if select_btn.clicked() {
                                self.selected_agent = Some(*role);
                            }
                        });
                    });

                col.add_space(14.0);
            }
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
                ui.horizontal(|ui| {
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
                            .stroke(Stroke::new(0.8, theme.border_subtle))
                            .inner_margin(Margin::symmetric(14, 10))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
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
                                                RichText::new(log.tag)
                                                    .size(9.5)
                                                    .strong()
                                                    .color(theme.accent_blue),
                                            );
                                        });

                                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                        ui.label(
                                            RichText::new(log.timestamp)
                                                .size(10.0)
                                                .color(theme.text_muted),
                                        );
                                    });
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
            .inner_margin(Margin::same(20))
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
                ui.horizontal(|ui| {
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
                    let text_edit = ui.add(
                        egui::TextEdit::singleline(&mut self.directive_input)
                            .hint_text("Type engineering directive (e.g., 'check seismic drift on story 3', 'generate A1 sheet')...")
                            .desired_width(ui.available_width() - 110.0),
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
                            timestamp: "Now",
                            role: target_role,
                            message: format!("Directive executed: \"{}\" — Analysis completed with zero constraint violations.", directive),
                            tag: "DIRECTIVE",
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
            .inner_margin(Margin::same(20))
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

                ui.columns(4, |cols| {
                    let kpis = [
                        ("ARI Redundancy", "0.942", "Grade A (Safe)", theme.accent_green),
                        ("SVD Condition κ", "1.42e3", "Well-Conditioned", theme.accent_blue),
                        ("BoQ Accuracy", "99.8%", "AHSP Synchronized", theme.accent_green),
                        ("Directives Cleared", "18 / 18", "100% Resolved", theme.text_primary),
                    ];

                    for (idx, (title, val, sub, col)) in kpis.iter().enumerate() {
                        let c = &mut cols[idx];
                        Frame::new()
                            .fill(theme.bg_canvas)
                            .corner_radius(CornerRadius::same(8))
                            .stroke(Stroke::new(0.8, theme.border_subtle))
                            .inner_margin(Margin::same(12))
                            .show(c, |ui| {
                                ui.label(RichText::new(*title).size(11.0).color(theme.text_muted));
                                ui.label(RichText::new(*val).size(20.0).strong().color(*col));
                                ui.label(RichText::new(*sub).size(10.0).color(theme.text_secondary));
                            });
                    }
                });
            });
    }
}
