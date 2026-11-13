//! app.rs — Core application struct implementing eframe::App.

use eframe::App;
use egui::{
    Align, Align2, Area, CentralPanel, Color32, CornerRadius, Frame, Id, Layout, Margin, Order,
    Panel, Pos2, Rect, RichText, ScrollArea, Stroke, StrokeKind, Vec2,
};

use crate::features::FeatureRegistry;
use crate::models::{DatabaseComponent, NavigationPillar, ProjectItem, StructuralMember};
use crate::theme::MacTheme;
use crate::views::{
    settings::SettingsSubTab, AiAssistantView, AiFirmView, AnalyzeView, BuildView, FacilityView,
    IndustrializedView, LeanView, MyProjectView, ParametricView, PlanningView, PredictionView,
    ResearchConceptView, ResultsView, SafetyView, SettingsView, SustainabilityView,
};

pub struct EigenGraphApp {
    pub theme: MacTheme,
    pub active_pillar: NavigationPillar,
    pub search_query: String,
    pub show_settings: bool,
    pub sidebar_open: bool,
    layout_initialized: bool,
    last_compact_layout: bool,

    // Centralized Feature Gate & Phase 31+ Unlock Roadmap Registry
    pub feature_registry: FeatureRegistry,

    // 15-Pillar Master Shell High-Fidelity Views
    pub my_project_view: MyProjectView,         // Pillar 1: 📁 My Library (Obsidian-Style Vault)
    pub ai_firm_view: AiFirmView,               // Pillar 2: 🤖 AI Agent (Virtual Engineering Firm)
    pub research_view: ResearchConceptView,     // Pillar 3: 💡 Research (GIS, CRS & DEM TIN Mesh)
    pub parametric_view: ParametricView,        // Pillar 4: 📐 Parametric (2D CAD & 3D Companion)
    pub build_view: BuildView,                  // Pillar 5: 🏗️ Geometry (Clean BIM Spatial Assembly)
    pub analyze_view: AnalyzeView,              // Pillar 6: ⚡ Analysis (3D Geo-Structural Solver)
    pub planning_view: PlanningView,            // Pillars 7 & 8: ⏱️ Time (4D) & 💰 Cost (5D Heatmap)
    pub sustainability_view: SustainabilityView,// Pillar 9: 🌱 Sustainability (6D Solar Heliodon)
    pub facility_view: FacilityView,            // Pillar 10: 🏢 Facility Management (7D Digital Twin)
    pub safety_view: SafetyView,                // Pillar 11: 🛡️ Safety & Health (8D Crane & Fall Risk)
    pub lean_view: LeanView,                    // Pillar 12: ♻️ Lean Construction (9D Staging)
    pub industrialized_view: IndustrializedView,// Pillar 13: 🤖 Industrialized & Automation (10D DfMA)
    pub prediction_view: PredictionView,        // Pillar 14: 🔮 Prediction (AI Aging & Drift)
    pub results_view: ResultsView,              // Pillar 15: 📑 Report (Living Docs, Sheets, Slides)
    pub settings_view: SettingsView,
    pub ai_assistant_view: AiAssistantView,

    // Domain Data & Parametric Component Database
    pub projects: Vec<ProjectItem>,
    pub db_components: Vec<DatabaseComponent>,
    pub members: Vec<StructuralMember>,
}

impl Default for EigenGraphApp {
    fn default() -> Self {
        Self {
            theme: MacTheme::default(),
            active_pillar: NavigationPillar::MyLibrary,
            search_query: String::new(),
            show_settings: false,
            sidebar_open: true,
            layout_initialized: false,
            last_compact_layout: false,

            feature_registry: FeatureRegistry::default(),

            my_project_view: MyProjectView::default(),
            ai_firm_view: AiFirmView::default(),
            research_view: ResearchConceptView::default(),
            parametric_view: ParametricView::default(),
            build_view: BuildView::default(),
            analyze_view: AnalyzeView::default(),
            planning_view: PlanningView::default(),
            sustainability_view: SustainabilityView::default(),
            facility_view: FacilityView::default(),
            safety_view: SafetyView::default(),
            lean_view: LeanView::default(),
            industrialized_view: IndustrializedView::default(),
            prediction_view: PredictionView::default(),
            results_view: ResultsView::default(),
            settings_view: SettingsView::default(),
            ai_assistant_view: AiAssistantView::default(),

            projects: crate::core::InternalVault::default_nvw_dcr_lod_projects(),
            db_components: crate::core::InternalVault::load_or_init_parametric_components(),
            members: crate::core::InternalVault::default_nvw_dcr_lod_members(),
        }
    }
}

const ALL_PILLARS: [NavigationPillar; 15] = [
    NavigationPillar::MyLibrary,
    NavigationPillar::AiAgent,
    NavigationPillar::ResearchConcept,
    NavigationPillar::ParametricDED,
    NavigationPillar::GeometryAssembly,
    NavigationPillar::AnalysisSimulation,
    NavigationPillar::TimeScheduling,
    NavigationPillar::CostBudgeting,
    NavigationPillar::SustainabilityEnergy,
    NavigationPillar::FacilityManagement,
    NavigationPillar::SafetyHealth,
    NavigationPillar::LeanConstruction,
    NavigationPillar::IndustrializedAutomation,
    NavigationPillar::Prediction,
    NavigationPillar::ReportPublishing,
];

impl EigenGraphApp {
    /// Renders sidebar navigation contents identically for both the docked panel and mobile/tablet drawer.
    fn render_sidebar_contents(&mut self, ui: &mut egui::Ui, is_overlay: bool, is_mobile: bool) {
        ui.vertical(|ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.add_space(2.0);
                ui.label(
                    RichText::new("⚛️ EigenGraph")
                        .size(13.0)
                        .strong()
                        .color(self.theme.text_primary),
                );

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let close_icon = if is_overlay { "✕" } else { "◀" };
                    let close_hint = if is_overlay {
                        "Close navigation drawer"
                    } else {
                        "Collapse sidebar"
                    };
                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new(close_icon)
                                    .size(12.0)
                                    .color(self.theme.text_secondary),
                            )
                            .fill(self.theme.bg_card)
                            .corner_radius(CornerRadius::same(6))
                            .min_size(Vec2::new(26.0, 24.0)),
                        )
                        .on_hover_text(close_hint)
                        .clicked()
                    {
                        self.sidebar_open = false;
                    }

                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                            .size(9.5)
                            .strong()
                            .color(self.theme.text_muted),
                    );
                });
            });
            ui.add_space(10.0);

            // Search Pill Bar
            Frame::new()
                .fill(self.theme.bg_card)
                .corner_radius(CornerRadius::same(8))
                .inner_margin(Margin::symmetric(10, 6))
                .stroke(Stroke::new(1.0, self.theme.border_card))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.search_query)
                                .hint_text("🔍 Search modules...")
                                .desired_width((ui.available_width() - 4.0).max(80.0))
                                .frame(Frame::NONE),
                        );
                    });
                });
            ui.add_space(10.0);

            let bottom_section_height = 52.0;
            let nav_area_height = (ui.available_height() - bottom_section_height).max(100.0);
            let nav_item_h = if is_overlay || is_mobile { 38.0 } else { 32.0 };

            ui.allocate_ui_with_layout(
                Vec2::new(ui.available_width(), nav_area_height),
                Layout::top_down(Align::Min),
                |ui| {
                    ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            for pillar in ALL_PILLARS {
                                if !self.search_query.trim().is_empty() {
                                    let q = self.search_query.trim().to_lowercase();
                                    let name = pillar.clean_name().to_lowercase();
                                    if !name.contains(&q) {
                                        continue;
                                    }
                                }

                                let is_active = self.active_pillar == pillar;
                                let (rect, response) = ui.allocate_exact_size(
                                    Vec2::new(ui.available_width(), nav_item_h),
                                    egui::Sense::click(),
                                );

                                if response.clicked() {
                                    self.active_pillar = pillar;
                                    if is_overlay {
                                        self.sidebar_open = false;
                                    }
                                }

                                let is_hovered = response.hovered();
                                let fill_color = if is_active {
                                    self.theme.accent_blue.gamma_multiply(0.22)
                                } else if is_hovered {
                                    self.theme.bg_card_hover
                                } else {
                                    Color32::TRANSPARENT
                                };

                                if fill_color != Color32::TRANSPARENT {
                                    ui.painter().rect_filled(
                                        rect,
                                        CornerRadius::same(8),
                                        fill_color,
                                    );
                                }

                                if is_active {
                                    ui.painter().rect_stroke(
                                        rect,
                                        CornerRadius::same(8),
                                        Stroke::new(
                                            1.0,
                                            self.theme.accent_blue.gamma_multiply(0.55),
                                        ),
                                        StrokeKind::Inside,
                                    );
                                    let accent_bar = Rect::from_min_max(
                                        Pos2::new(rect.left() + 2.0, rect.top() + 6.0),
                                        Pos2::new(rect.left() + 5.5, rect.bottom() - 6.0),
                                    );
                                    ui.painter().rect_filled(
                                        accent_bar,
                                        CornerRadius::same(2),
                                        self.theme.accent_blue,
                                    );
                                }

                                let text_color = if is_active || is_hovered {
                                    Color32::WHITE
                                } else {
                                    self.theme.text_secondary
                                };

                                let item_label =
                                    format!("{}   {}", pillar.icon(), pillar.clean_name());
                                ui.painter().text(
                                    rect.left_center() + Vec2::new(12.0, 0.0),
                                    Align2::LEFT_CENTER,
                                    item_label,
                                    egui::FontId::proportional(12.5),
                                    text_color,
                                );

                                let (p_act, p_tot) = self.feature_registry.pillar_counts(pillar);
                                let badge_color = if p_act == p_tot {
                                    self.theme.accent_green
                                } else {
                                    self.theme.text_muted
                                };
                                ui.painter().text(
                                    rect.right_center() - Vec2::new(8.0, 0.0),
                                    Align2::RIGHT_CENTER,
                                    format!("{p_act}/{p_tot}"),
                                    egui::FontId::monospace(9.5),
                                    badge_color,
                                );

                                ui.add_space(2.0);
                            }
                        });
                },
            );

            // Bottom Pinned Dock: Workspace Settings
            ui.separator();
            ui.add_space(6.0);

            if ui
                .add(
                    egui::Button::new(
                        RichText::new("⚙️ Settings")
                            .size(11.5)
                            .color(self.theme.text_secondary),
                    )
                    .fill(self.theme.bg_card)
                    .corner_radius(CornerRadius::same(7))
                    .min_size(Vec2::new(ui.available_width(), 32.0)),
                )
                .on_hover_text("Workspace Settings")
                .clicked()
            {
                self.show_settings = !self.show_settings;
                if is_overlay {
                    self.sidebar_open = false;
                }
            }
        });
    }

    /// Renders compact status, feature roadmap gate, and build version pills inside the workspace header.
    fn render_header_badges(&mut self, ui: &mut egui::Ui) {
        let active_features = self.feature_registry.active_count();
        let locked_features = self.feature_registry.locked_count();
        let total_features = self.feature_registry.total_count();
        let roadmap_label = format!(
            "🔓 {active_features}/{total_features} Active • 🔒 {locked_features} Slots • 📋 Roadmap"
        );

        if ui
            .add(
                egui::Button::new(
                    RichText::new(roadmap_label)
                        .size(10.5)
                        .strong()
                        .color(self.theme.accent_blue),
                )
                .fill(self.theme.accent_blue.gamma_multiply(0.14))
                .stroke(Stroke::new(
                    1.0,
                    self.theme.accent_blue.gamma_multiply(0.55),
                ))
                .corner_radius(CornerRadius::same(7)),
            )
            .on_hover_text("Open Feature Unlock Roadmap & Workspace Settings")
            .clicked()
        {
            self.settings_view.active_subtab = SettingsSubTab::FeatureRegistry;
            self.show_settings = true;
        }

        Frame::new()
            .fill(self.theme.accent_green.gamma_multiply(0.14))
            .stroke(Stroke::new(1.0, self.theme.accent_green.gamma_multiply(0.6)))
            .corner_radius(CornerRadius::same(7))
            .inner_margin(Margin::symmetric(8, 4))
            .show(ui, |ui| {
                ui.label(
                    RichText::new("🟢 ARI 0.942 • SVD Verified")
                        .size(10.5)
                        .strong()
                        .color(self.theme.accent_green),
                );
            });

        Frame::new()
            .fill(self.theme.bg_card)
            .stroke(Stroke::new(1.0, self.theme.border_card))
            .corner_radius(CornerRadius::same(7))
            .inner_margin(Margin::symmetric(8, 4))
            .show(ui, |ui| {
                ui.label(
                    RichText::new(format!("v{} • Phase 31", env!("CARGO_PKG_VERSION")))
                        .size(10.5)
                        .strong()
                        .color(self.theme.accent_blue),
                );
            });
    }

    /// Renders the quick stage ribbon with horizontal scrolling on phones and wrapping on wider screens.
    fn render_workflow_ribbon(&mut self, ui: &mut egui::Ui, is_mobile: bool) {
        let quick_stages = [
            (NavigationPillar::MyLibrary, "📁 Vault"),
            (NavigationPillar::ResearchConcept, "🗺️ 1D GIS & TIN"),
            (NavigationPillar::ParametricDED, "📐 2D Parametric"),
            (NavigationPillar::GeometryAssembly, "🏗️ 3D BIM"),
            (NavigationPillar::AnalysisSimulation, "⚡ FEA Solver"),
            (NavigationPillar::TimeScheduling, "⏱️ 4D Schedule"),
            (NavigationPillar::CostBudgeting, "💰 5D Cost"),
            (NavigationPillar::IndustrializedAutomation, "🤖 10D OpenBIM"),
            (NavigationPillar::ReportPublishing, "📑 Report"),
        ];

        let mut render_chips = |ui: &mut egui::Ui| {
            ui.label(
                RichText::new("WORKFLOW:")
                    .size(10.0)
                    .strong()
                    .color(self.theme.text_muted),
            );
            for (p, label) in quick_stages {
                let active = self.active_pillar == p;
                let bg = if active {
                    self.theme.accent_blue
                } else {
                    self.theme.bg_card
                };
                let fg = if active {
                    Color32::WHITE
                } else {
                    self.theme.text_secondary
                };
                if ui
                    .add(
                        egui::Button::new(RichText::new(label).size(10.5).color(fg))
                            .fill(bg)
                            .corner_radius(CornerRadius::same(6))
                            .min_size(Vec2::new(0.0, if is_mobile { 28.0 } else { 24.0 })),
                    )
                    .clicked()
                {
                    self.active_pillar = p;
                }
            }
        };

        if is_mobile {
            ScrollArea::horizontal()
                .id_salt("mobile_workflow_ribbon")
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        render_chips(ui);
                    });
                });
        } else {
            ui.horizontal_wrapped(|ui| {
                render_chips(ui);
            });
        }
    }
}

impl App for EigenGraphApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.theme.apply_to_ctx(&ctx);

        // Keyboard shortcut (Cmd+,) toggles Settings modal
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::Comma)) {
            self.show_settings = !self.show_settings;
        }

        let screen_rect = ctx.content_rect();
        let is_mobile = MacTheme::is_mobile(&ctx);
        let is_compact = MacTheme::is_compact(&ctx);

        // Initialize or adapt sidebar drawer automatically when crossing mobile/tablet threshold
        if !self.layout_initialized {
            self.sidebar_open = !is_compact;
            self.layout_initialized = true;
            self.last_compact_layout = is_compact;
            ctx.request_repaint();
        } else if is_compact != self.last_compact_layout {
            self.sidebar_open = !is_compact;
            self.last_compact_layout = is_compact;
            ctx.request_repaint();
        }

        // 1. DOCKED SIDEBAR ON DESKTOP (>= 960px WHEN OPEN)
        let mut docked_sidebar_width = 0.0;
        if !is_compact && self.sidebar_open {
            let panel_resp = Panel::left("unified_sidebar")
                .resizable(true)
                .default_size(246.0)
                .size_range(200.0..=320.0)
                .frame(
                    Frame::new()
                        .fill(self.theme.bg_sidebar)
                        .stroke(Stroke::new(1.0, self.theme.border_subtle))
                        .inner_margin(Margin::symmetric(12, 10)),
                )
                .show(ui, |ui| {
                    self.render_sidebar_contents(ui, false, is_mobile);
                });
            docked_sidebar_width = panel_resp.response.rect.width();
        }

        // 2. CENTRAL PANEL (Responsive Workspace Layout for Phone, Tablet & Desktop)
        CentralPanel::default()
            .frame(Frame::new().fill(self.theme.bg_canvas))
            .show(ui, |ui| {
                let header_margin = if is_mobile {
                    Margin::symmetric(12, 10)
                } else {
                    Margin::symmetric(20, 12)
                };
                Frame::new()
                    .fill(self.theme.bg_sidebar)
                    .stroke(Stroke::new(1.0, self.theme.border_subtle))
                    .inner_margin(header_margin)
                    .show(ui, |ui| {
                        let avail_w = ui.available_width();
                        let toggle_icon = if self.sidebar_open { "✕" } else { "☰" };
                        let toggle_tip = if self.sidebar_open {
                            "Hide navigation menu"
                        } else {
                            "Open navigation menu"
                        };

                        if avail_w >= 680.0 {
                            ui.horizontal(|ui| {
                                if ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new(toggle_icon)
                                                .size(15.0)
                                                .strong()
                                                .color(self.theme.text_primary),
                                        )
                                        .fill(self.theme.bg_card)
                                        .stroke(Stroke::new(1.0, self.theme.border_card))
                                        .corner_radius(CornerRadius::same(8))
                                        .min_size(Vec2::new(34.0, 34.0)),
                                    )
                                    .on_hover_text(toggle_tip)
                                    .clicked()
                                {
                                    self.sidebar_open = !self.sidebar_open;
                                }

                                ui.add_space(6.0);
                                ui.vertical(|ui| {
                                    ui.label(
                                        RichText::new(format!(
                                            "{}  {}",
                                            self.active_pillar.icon(),
                                            self.active_pillar.clean_name()
                                        ))
                                        .size(20.0)
                                        .strong()
                                        .color(self.theme.text_primary),
                                    );
                                    ui.label(
                                        RichText::new(self.active_pillar.subtitle())
                                            .size(11.5)
                                            .color(self.theme.text_secondary),
                                    );
                                });

                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    self.render_header_badges(ui);
                                });
                            });
                        } else {
                            ui.horizontal(|ui| {
                                if ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new(toggle_icon)
                                                .size(16.0)
                                                .strong()
                                                .color(self.theme.text_primary),
                                        )
                                        .fill(self.theme.bg_card)
                                        .stroke(Stroke::new(1.0, self.theme.border_card))
                                        .corner_radius(CornerRadius::same(8))
                                        .min_size(Vec2::new(36.0, 36.0)),
                                    )
                                    .on_hover_text(toggle_tip)
                                    .clicked()
                                {
                                    self.sidebar_open = !self.sidebar_open;
                                }

                                ui.add_space(6.0);
                                ui.vertical(|ui| {
                                    ui.label(
                                        RichText::new(format!(
                                            "{}  {}",
                                            self.active_pillar.icon(),
                                            self.active_pillar.clean_name()
                                        ))
                                        .size(if is_mobile { 17.0 } else { 19.0 })
                                        .strong()
                                        .color(self.theme.text_primary),
                                    );
                                    ui.label(
                                        RichText::new(self.active_pillar.subtitle())
                                            .size(11.0)
                                            .color(self.theme.text_secondary),
                                    );
                                });
                            });

                            ui.add_space(6.0);
                            ui.horizontal_wrapped(|ui| {
                                self.render_header_badges(ui);
                            });
                        }

                        ui.add_space(8.0);
                        self.render_workflow_ribbon(ui, is_mobile);
                    });

                ui.add_space(6.0);

                match self.active_pillar {
                    NavigationPillar::MyLibrary => {
                        self.my_project_view.show(
                            ui,
                            &self.theme,
                            &self.projects,
                            &mut self.active_pillar,
                            &mut self.feature_registry,
                        );
                    }
                    NavigationPillar::AiAgent => {
                        self.ai_firm_view.show(
                            ui,
                            &self.theme,
                            &self.members,
                            &self.projects,
                            &mut self.db_components,
                            &mut self.feature_registry,
                        );
                    }
                    NavigationPillar::ResearchConcept => {
                        self.research_view.show(
                            ui,
                            &self.theme,
                            &mut self.active_pillar,
                            &mut self.feature_registry,
                        );
                    }
                    NavigationPillar::ParametricDED => {
                        self.parametric_view.show(
                            ui,
                            &self.theme,
                            &mut self.db_components,
                            &mut self.active_pillar,
                            &mut self.feature_registry,
                        );
                    }
                    NavigationPillar::GeometryAssembly => {
                        self.build_view.show(
                            ui,
                            &self.theme,
                            &self.members,
                            &self.db_components,
                            &mut self.feature_registry,
                        );
                    }
                    NavigationPillar::AnalysisSimulation => {
                        self.analyze_view.show(
                            ui,
                            &self.theme,
                            &self.members,
                            &mut self.feature_registry,
                        );
                    }
                    NavigationPillar::TimeScheduling => {
                        if !matches!(
                            self.planning_view.active_subtab,
                            crate::views::planning::PlanningSubTab::Schedule4d
                                | crate::views::planning::PlanningSubTab::StagedErectionSvd
                                | crate::views::planning::PlanningSubTab::ScheduleRiskMonteCarlo
                                | crate::views::planning::PlanningSubTab::ResourceLeveling4d
                        ) {
                            self.planning_view.active_subtab =
                                crate::views::planning::PlanningSubTab::Schedule4d;
                        }
                        self.planning_view.show(
                            ui,
                            &self.theme,
                            &self.parametric_view.variables,
                            &self.members,
                            &mut self.feature_registry,
                            &mut self.active_pillar,
                        );
                    }
                    NavigationPillar::CostBudgeting => {
                        if !matches!(
                            self.planning_view.active_subtab,
                            crate::views::planning::PlanningSubTab::DynamicRab
                                | crate::views::planning::PlanningSubTab::TakeoffQto
                                | crate::views::planning::PlanningSubTab::CostAriTradeoff
                                | crate::views::planning::PlanningSubTab::WbsHierarchy
                                | crate::views::planning::PlanningSubTab::EarnedValueEvm
                                | crate::views::planning::PlanningSubTab::ValueEngineeringBid
                        ) {
                            self.planning_view.active_subtab =
                                crate::views::planning::PlanningSubTab::DynamicRab;
                        }
                        self.planning_view.show(
                            ui,
                            &self.theme,
                            &self.parametric_view.variables,
                            &self.members,
                            &mut self.feature_registry,
                            &mut self.active_pillar,
                        );
                    }
                    NavigationPillar::SustainabilityEnergy => {
                        self.sustainability_view.show(
                            ui,
                            &self.theme,
                            &self.members,
                            &mut self.feature_registry,
                        );
                    }
                    NavigationPillar::FacilityManagement => {
                        self.facility_view.show(
                            ui,
                            &self.theme,
                            &self.members,
                            &mut self.feature_registry,
                        );
                    }
                    NavigationPillar::SafetyHealth => {
                        self.safety_view.show(
                            ui,
                            &self.theme,
                            &self.members,
                            &mut self.feature_registry,
                        );
                    }
                    NavigationPillar::LeanConstruction => {
                        self.lean_view.show(
                            ui,
                            &self.theme,
                            &self.members,
                            &mut self.feature_registry,
                        );
                    }
                    NavigationPillar::IndustrializedAutomation => {
                        self.industrialized_view.show(
                            ui,
                            &self.theme,
                            &self.members,
                            &mut self.feature_registry,
                        );
                    }
                    NavigationPillar::Prediction => {
                        self.prediction_view.show(
                            ui,
                            &self.theme,
                            &self.members,
                            &mut self.feature_registry,
                        );
                    }
                    NavigationPillar::ReportPublishing => {
                        self.results_view.show(
                            ui,
                            &self.theme,
                            &self.members,
                            &mut self.feature_registry,
                        );
                    }
                }
            });

        // 3. FLOATING DYNAMIC ISLAND AI COPILOT
        self.ai_assistant_view.show(
            &ctx,
            &self.theme,
            self.active_pillar,
            &self.members,
            &self.projects,
            &mut self.db_components,
            docked_sidebar_width * 0.5,
        );

        // 4. RESPONSIVE NAVIGATION DRAWER OVERLAY (MOBILE & NARROW TABLET < 960px)
        if is_compact && self.sidebar_open {
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.sidebar_open = false;
            } else {
                let drawer_w = (screen_rect.width() * 0.84).clamp(240.0, 300.0);
                let drawer_h = screen_rect.height().max(260.0);
                let backdrop_w = (screen_rect.width() - drawer_w).max(0.0);

                if backdrop_w > 1.0 {
                    Area::new(Id::new("mobile_sidebar_backdrop"))
                        .order(Order::Foreground)
                        .fixed_pos(Pos2::new(screen_rect.left() + drawer_w, screen_rect.top()))
                        .show(&ctx, |ui| {
                            let (rect, resp) = ui.allocate_exact_size(
                                Vec2::new(backdrop_w, drawer_h),
                                egui::Sense::click(),
                            );
                            ui.painter().rect_filled(
                                rect,
                                CornerRadius::ZERO,
                                Color32::from_black_alpha(150),
                            );
                            if resp.clicked() {
                                self.sidebar_open = false;
                            }
                        });
                }

                Area::new(Id::new("mobile_nav_drawer"))
                    .order(Order::Foreground)
                    .fixed_pos(screen_rect.min)
                    .show(&ctx, |ui| {
                        Frame::new()
                            .fill(self.theme.bg_sidebar)
                            .stroke(Stroke::new(1.0, self.theme.border_card))
                            .inner_margin(Margin::symmetric(12, 10))
                            .show(ui, |ui| {
                                let inner_w = (drawer_w - 24.0).max(180.0);
                                let inner_h = (drawer_h - 20.0).max(200.0);
                                ui.set_min_size(Vec2::new(inner_w, inner_h));
                                ui.set_max_size(Vec2::new(inner_w, inner_h));
                                self.render_sidebar_contents(ui, true, is_mobile);
                            });
                    });
            }
        }

        // 5. SETTINGS & FEATURE ROADMAP MODAL (Responsive to Mobile, Tablet & Desktop)
        if self.show_settings {
            let mut open = self.show_settings;
            let mut close_after_jump = false;
            egui::Window::new("⚙️ Workspace Settings & Feature Roadmap")
                .open(&mut open)
                .collapsible(false)
                .resizable(true)
                .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 0.0))
                .default_size(MacTheme::clamp_modal_size(&ctx, Vec2::new(820.0, 650.0)))
                .max_size(MacTheme::modal_max_size(&ctx))
                .frame(
                    Frame::new()
                        .fill(self.theme.bg_card)
                        .stroke(Stroke::new(1.0, self.theme.border_subtle))
                        .corner_radius(CornerRadius::same(12))
                        .inner_margin(Margin::same(if is_mobile { 12 } else { 18 })),
                )
                .show(&ctx, |ui| {
                    if self.settings_view.show(
                        ui,
                        &self.theme,
                        &mut self.feature_registry,
                        &mut self.active_pillar,
                    ) {
                        close_after_jump = true;
                    }
                });
            self.show_settings = open && !close_after_jump;
        }

        // 8. SCREENSHOT HOOK (AUTOMATED QA & PREVIEWS)
        #[cfg(not(target_arch = "wasm32"))]
        if let Ok(path) = std::env::var("EIGENGRAPH_SCREENSHOT_PATH") {
            let mut screenshot_saved = false;
            ui.ctx().input(|i| {
                for event in &i.raw.events {
                    if let egui::Event::Screenshot { image, .. } = event {
                        let width = image.size[0] as u32;
                        let height = image.size[1] as u32;
                        let mut bytes = Vec::with_capacity((width * height * 4) as usize);
                        for p in &image.pixels {
                            bytes.push(p.r());
                            bytes.push(p.g());
                            bytes.push(p.b());
                            bytes.push(p.a());
                        }
                        let _ = image::save_buffer(
                            &path,
                            &bytes,
                            width,
                            height,
                            image::ColorType::Rgba8,
                        );
                        screenshot_saved = true;
                    }
                }
            });

            if screenshot_saved {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            } else {
                static FRAME_COUNT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                let count = FRAME_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if count >= 3 {
                    ui.ctx()
                        .send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
                }
                ui.ctx().request_repaint();
            }
        }
    }
}
