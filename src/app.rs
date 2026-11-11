//! app.rs — Core application struct implementing eframe::App.

use eframe::App;
use egui::{
    Align, Align2, CentralPanel, Color32, CornerRadius, Frame, Layout, Margin, Panel, Pos2, Rect,
    RichText, Stroke, StrokeKind, Vec2,
};

use crate::models::{DatabaseComponent, NavigationPillar, ProjectItem, StructuralMember};
use crate::theme::MacTheme;
use crate::views::{
    AiAssistantView, AiFirmView, AnalyzeView, BuildView, FacilityView, IndustrializedView,
    LeanView, MyProjectView, ParametricView, PlanningView, PredictionView, ResearchConceptView,
    ResultsView, SafetyView, SettingsView, SustainabilityView,
};

pub struct EigenGraphApp {
    pub theme: MacTheme,
    pub active_pillar: NavigationPillar,
    pub search_query: String,
    pub show_settings: bool,

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
    pub updater: crate::updater::AppUpdater,

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
            updater: crate::updater::AppUpdater::default(),

            projects: crate::core::InternalVault::default_nvw_dcr_lod_projects(),
            db_components: crate::core::InternalVault::load_or_init_parametric_components(),
            members: crate::core::InternalVault::default_nvw_dcr_lod_members(),
        }
    }
}

impl App for EigenGraphApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.theme.apply_to_ctx(&ctx);

        // macOS Command+, shortcut for System Preferences
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::Comma)) {
            self.show_settings = !self.show_settings;
        }

        let pillars = [
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

        // 1. UNIFIED FULL-HEIGHT LEFT SIDEBAR (Modernized Pro Dark macOS Architecture)
        Panel::left("unified_sidebar")
            .resizable(false)
            .exact_size(246.0)
            .frame(
                Frame::new()
                    .fill(self.theme.bg_sidebar)
                    .stroke(Stroke::new(1.0, self.theme.border_subtle))
                    .inner_margin(Margin::symmetric(12, 10)),
            )
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    #[cfg(target_os = "macos")]
                    {
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.add_space(2.0);
                            ui.label(RichText::new("🔴  🟡  🟢").size(9.5));
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.label(
                                    RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                                        .size(9.5)
                                        .strong()
                                        .color(self.theme.text_muted),
                                );
                            });
                        });
                        ui.add_space(12.0);
                    }
                    #[cfg(not(target_os = "macos"))]
                    {
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.add_space(2.0);
                            ui.label(RichText::new("🔴  🟡  🟢").size(9.5));
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.label(
                                    RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                                        .size(9.5)
                                        .strong()
                                        .color(self.theme.text_muted),
                                );
                            });
                        });
                        ui.add_space(12.0);
                    }

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
                                        .hint_text("🔍 Search 15 BIM Pillars...")
                                        .desired_width(ui.available_width() - 4.0)
                                        .frame(Frame::NONE),
                                );
                            });
                        });
                    ui.add_space(10.0);

                    let bottom_section_height = 112.0;
                    let nav_area_height =
                        (ui.available_height() - bottom_section_height).max(100.0);

                    ui.allocate_ui_with_layout(
                        Vec2::new(ui.available_width(), nav_area_height),
                        Layout::top_down(Align::Min),
                        |ui| {
                            egui::ScrollArea::vertical()
                                .auto_shrink([false; 2])
                                .show(ui, |ui| {
                                    for pillar in pillars {
                                        if !self.search_query.trim().is_empty() {
                                            let q = self.search_query.trim().to_lowercase();
                                            let name = pillar.clean_name().to_lowercase();
                                            if !name.contains(&q) {
                                                continue;
                                            }
                                        }

                                        let is_active = self.active_pillar == pillar;
                                        let (rect, response) = ui.allocate_exact_size(
                                            Vec2::new(ui.available_width(), 32.0),
                                            egui::Sense::click(),
                                        );

                                        if response.clicked() {
                                            self.active_pillar = pillar;
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

                                        ui.add_space(2.0);
                                    }
                                });
                        },
                    );

                    // Bottom Pinned Dock: 1-Click Update & User Profile
                    ui.separator();
                    ui.add_space(6.0);

                    ui.columns(2, |cols| {
                        let w0 = cols[0].available_width();
                        if cols[0]
                            .add(
                                egui::Button::new(
                                    RichText::new("🔄 Update App")
                                        .size(11.0)
                                        .strong()
                                        .color(Color32::WHITE),
                                )
                                .fill(self.theme.accent_blue)
                                .corner_radius(CornerRadius::same(7))
                                .min_size(Vec2::new(w0, 28.0)),
                            )
                            .on_hover_text("Check GitHub Releases & install 1-Click macOS Update")
                            .clicked()
                        {
                            self.updater.check_for_updates();
                        }

                        let w1 = cols[1].available_width();
                        if cols[1]
                            .add(
                                egui::Button::new(
                                    RichText::new("⚙️ Settings")
                                        .size(11.0)
                                        .color(self.theme.text_secondary),
                                )
                                .fill(self.theme.bg_card)
                                .corner_radius(CornerRadius::same(7))
                                .min_size(Vec2::new(w1, 28.0)),
                            )
                            .on_hover_text("System Preferences (⌘,)")
                            .clicked()
                        {
                            self.show_settings = !self.show_settings;
                        }
                    });

                    ui.add_space(6.0);

                    let (prof_rect, prof_resp) = ui.allocate_exact_size(
                        Vec2::new(ui.available_width(), 38.0),
                        egui::Sense::click(),
                    );
                    let prof_hovered = prof_resp.hovered();
                    let prof_fill = if self.show_settings {
                        self.theme.accent_blue.gamma_multiply(0.18)
                    } else if prof_hovered {
                        self.theme.bg_card_hover
                    } else {
                        self.theme.bg_card
                    };
                    ui.painter()
                        .rect_filled(prof_rect, CornerRadius::same(8), prof_fill);

                    let avatar_pos = prof_rect.left_center() + Vec2::new(16.0, 0.0);
                    ui.painter()
                        .circle_filled(avatar_pos, 11.5, self.theme.accent_blue);
                    ui.painter().text(
                        avatar_pos,
                        Align2::CENTER_CENTER,
                        "HI",
                        egui::FontId::proportional(9.5),
                        Color32::WHITE,
                    );

                    ui.painter().text(
                        prof_rect.left_center() + Vec2::new(34.0, -5.0),
                        Align2::LEFT_CENTER,
                        "Harids Irawan",
                        egui::FontId::proportional(12.0),
                        self.theme.text_primary,
                    );
                    ui.painter().text(
                        prof_rect.left_center() + Vec2::new(34.0, 7.5),
                        Align2::LEFT_CENTER,
                        "Lead Structural Engineer",
                        egui::FontId::proportional(9.5),
                        self.theme.text_muted,
                    );

                    if prof_resp.on_hover_text("System Preferences (⌘,)").clicked() {
                        self.show_settings = !self.show_settings;
                    }
                });
            });

        // 2. CENTRAL PANEL (Modern Re-Ordered Workspace Layout)
        CentralPanel::default()
            .frame(Frame::new().fill(self.theme.bg_canvas))
            .show(ui, |ui| {
                // Top Workspace Header & Quick Action Bar
                Frame::new()
                    .fill(self.theme.bg_sidebar)
                    .stroke(Stroke::new(1.0, self.theme.border_subtle))
                    .inner_margin(Margin::symmetric(22, 12))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new(format!(
                                        "{}  {}",
                                        self.active_pillar.icon(),
                                        self.active_pillar.clean_name()
                                    ))
                                    .size(21.0)
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
                                if ui
                                    .add(
                                        egui::Button::new(
                                            RichText::new("🔄 Update")
                                                .size(11.0)
                                                .strong()
                                                .color(Color32::WHITE),
                                        )
                                        .fill(self.theme.accent_blue)
                                        .corner_radius(CornerRadius::same(7))
                                        .min_size(Vec2::new(82.0, 26.0)),
                                    )
                                    .on_hover_text("Check for software updates and install in-place")
                                    .clicked()
                                {
                                    self.updater.check_for_updates();
                                }

                                ui.add_space(6.0);

                                Frame::new()
                                    .fill(self.theme.bg_card)
                                    .stroke(Stroke::new(1.0, self.theme.border_card))
                                    .corner_radius(CornerRadius::same(7))
                                    .inner_margin(Margin::symmetric(10, 4))
                                    .show(ui, |ui| {
                                        ui.label(
                                            RichText::new(format!(
                                                "v{} • Phase 25",
                                                env!("CARGO_PKG_VERSION")
                                            ))
                                            .size(11.0)
                                            .strong()
                                            .color(self.theme.accent_blue),
                                        );
                                    });

                                ui.add_space(6.0);

                                Frame::new()
                                    .fill(self.theme.accent_green.gamma_multiply(0.14))
                                    .stroke(Stroke::new(
                                        1.0,
                                        self.theme.accent_green.gamma_multiply(0.6),
                                    ))
                                    .corner_radius(CornerRadius::same(7))
                                    .inner_margin(Margin::symmetric(10, 4))
                                    .show(ui, |ui| {
                                        ui.label(
                                            RichText::new("🟢 ARI 0.942 • SVD Verified")
                                                .size(11.0)
                                                .strong()
                                                .color(self.theme.accent_green),
                                        );
                                    });
                            });
                        });

                        ui.add_space(8.0);

                        // Re-ordered Quick Workflow Pipeline Ribbon
                        ui.horizontal_wrapped(|ui| {
                            ui.label(
                                RichText::new("WORKFLOW:")
                                    .size(10.0)
                                    .strong()
                                    .color(self.theme.text_muted),
                            );
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
                                            .corner_radius(CornerRadius::same(6)),
                                    )
                                    .clicked()
                                {
                                    self.active_pillar = p;
                                }
                            }
                        });
                    });

                ui.add_space(6.0);

                match self.active_pillar {
                    NavigationPillar::MyLibrary => {
                        self.my_project_view.show(
                            ui,
                            &self.theme,
                            &self.projects,
                            &mut self.active_pillar,
                        );
                    }
                    NavigationPillar::AiAgent => {
                        self.ai_firm_view.show(
                            ui,
                            &self.theme,
                            &self.members,
                            &self.projects,
                            &mut self.db_components,
                        );
                    }
                    NavigationPillar::ResearchConcept => {
                        self.research_view
                            .show(ui, &self.theme, &mut self.active_pillar);
                    }
                    NavigationPillar::ParametricDED => {
                        self.parametric_view.show(
                            ui,
                            &self.theme,
                            &mut self.db_components,
                            &mut self.active_pillar,
                        );
                    }
                    NavigationPillar::GeometryAssembly => {
                        self.build_view
                            .show(ui, &self.theme, &self.members, &self.db_components);
                    }
                    NavigationPillar::AnalysisSimulation => {
                        self.analyze_view.show(ui, &self.theme, &self.members);
                    }
                    NavigationPillar::TimeScheduling => {
                        if self.planning_view.active_subtab
                            != crate::views::planning::PlanningSubTab::Schedule4d
                            && self.planning_view.active_subtab
                                != crate::views::planning::PlanningSubTab::StagedErectionSvd
                        {
                            self.planning_view.active_subtab =
                                crate::views::planning::PlanningSubTab::Schedule4d;
                        }
                        self.planning_view.show(
                            ui,
                            &self.theme,
                            &self.parametric_view.variables,
                            &self.members,
                        );
                    }
                    NavigationPillar::CostBudgeting => {
                        if self.planning_view.active_subtab
                            != crate::views::planning::PlanningSubTab::DynamicRab
                            && self.planning_view.active_subtab
                                != crate::views::planning::PlanningSubTab::TakeoffQto
                            && self.planning_view.active_subtab
                                != crate::views::planning::PlanningSubTab::CostAriTradeoff
                        {
                            self.planning_view.active_subtab =
                                crate::views::planning::PlanningSubTab::DynamicRab;
                        }
                        self.planning_view.show(
                            ui,
                            &self.theme,
                            &self.parametric_view.variables,
                            &self.members,
                        );
                    }
                    NavigationPillar::SustainabilityEnergy => {
                        self.sustainability_view
                            .show(ui, &self.theme, &self.members);
                    }
                    NavigationPillar::FacilityManagement => {
                        self.facility_view.show(ui, &self.theme, &self.members);
                    }
                    NavigationPillar::SafetyHealth => {
                        self.safety_view.show(ui, &self.theme, &self.members);
                    }
                    NavigationPillar::LeanConstruction => {
                        self.lean_view.show(ui, &self.theme, &self.members);
                    }
                    NavigationPillar::IndustrializedAutomation => {
                        self.industrialized_view
                            .show(ui, &self.theme, &self.members);
                    }
                    NavigationPillar::Prediction => {
                        self.prediction_view.show(ui, &self.theme, &self.members);
                    }
                    NavigationPillar::ReportPublishing => {
                        self.results_view.show(ui, &self.theme, &self.members);
                    }
                }
            });

        // 5. GLOBAL FLOATING DYNAMIC ISLAND AI COPILOT
        let ctx = ui.ctx().clone();
        self.ai_assistant_view.show(
            &ctx,
            &self.theme,
            self.active_pillar,
            &self.members,
            &self.projects,
            &mut self.db_components,
        );

        // 6. MACOS SOFTWARE UPDATE MODAL
        self.updater.render_modal(ui.ctx(), &self.theme);

        // 7. MACOS SYSTEM PREFERENCES MODAL (Cmd+,)
        if self.show_settings {
            let mut open = self.show_settings;
            egui::Window::new("⚙️ System Preferences")
                .open(&mut open)
                .collapsible(false)
                .resizable(true)
                .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 0.0))
                .default_size(Vec2::new(540.0, 580.0))
                .frame(
                    Frame::new()
                        .fill(self.theme.bg_card)
                        .stroke(Stroke::new(1.0, self.theme.border_subtle))
                        .corner_radius(CornerRadius::same(12))
                        .inner_margin(Margin::same(20)),
                )
                .show(ui.ctx(), |ui| {
                    self.settings_view.show(ui, &self.theme, &mut self.updater);
                });
            self.show_settings = open;
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
