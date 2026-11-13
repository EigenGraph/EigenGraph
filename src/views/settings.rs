//! settings.rs — Workspace Settings, Design Standards & Centralized Feature Unlock Roadmap.

use egui::{Align, Color32, CornerRadius, Frame, Layout, Margin, ProgressBar, RichText, ScrollArea, Stroke, Ui};
use crate::core::DiskExporter;
use crate::features::{FeatureRegistry, FeatureStatus};
use crate::models::NavigationPillar;
use crate::theme::MacTheme;
use crate::ui_kit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsSubTab {
    Preferences,
    FeatureRegistry,
    About,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistryFilter {
    All,
    LockedOnly,
    PrioritizedOnly,
    ActiveOnly,
}

pub struct SettingsView {
    pub active_subtab: SettingsSubTab,
    pub unit_system: String,
    pub design_standard: String,
    pub solver_threads: usize,
    pub enable_svd_guard: bool,
    pub enable_anti_leakage: bool,
    pub gemini_api_key: String,
    pub registry_search: String,
    pub registry_filter: RegistryFilter,
    pub pillar_filter: Option<NavigationPillar>,
    pub phase_filter: Option<&'static str>,
    pub export_feedback: Option<String>,
}

impl Default for SettingsView {
    fn default() -> Self {
        Self {
            active_subtab: SettingsSubTab::FeatureRegistry,
            unit_system: "kN, mm, MPa (Structural Metric)".to_string(),
            design_standard: "SNI 1729 / AISC 360-16".to_string(),
            solver_threads: 8,
            enable_svd_guard: true,
            enable_anti_leakage: true,
            gemini_api_key: String::new(),
            registry_search: String::new(),
            registry_filter: RegistryFilter::All,
            pillar_filter: None,
            phase_filter: None,
            export_feedback: None,
        }
    }
}

impl SettingsView {
    /// Renders System Preferences & Feature Unlock Roadmap.
    /// Returns `true` if the user clicked an Inspect action to jump straight to a domain workspace.
    pub fn show(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        registry: &mut FeatureRegistry,
        active_pillar: &mut NavigationPillar,
    ) -> bool {
        let mut requested_jump = false;

        ui_kit::subtab_bar(
            ui,
            theme,
            "settings_subtabs",
            &mut self.active_subtab,
            &[
                (SettingsSubTab::FeatureRegistry, "🔐 Feature Registry & Unlock Roadmap"),
                (SettingsSubTab::Preferences, "⚙️ Preferences & Standards"),
                (SettingsSubTab::About, "ℹ️ About EigenGraph"),
            ],
        );
        ui.add_space(10.0);

        ScrollArea::vertical()
            .id_salt("settings_scroll_body")
            .auto_shrink([false, false])
            .show(ui, |ui| match self.active_subtab {
                SettingsSubTab::FeatureRegistry => {
                    requested_jump = self.render_feature_registry(ui, theme, registry, active_pillar);
                }
                SettingsSubTab::Preferences => {
                    self.render_preferences(ui, theme);
                }
                SettingsSubTab::About => {
                    self.render_about(ui, theme);
                }
            });

        requested_jump
    }

    fn render_feature_registry(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        registry: &mut FeatureRegistry,
        active_pillar: &mut NavigationPillar,
    ) -> bool {
        let mut jump_clicked = false;
        let total = registry.total_count().max(1);
        let active = registry.active_count();
        let locked = registry.locked_count();
        let prioritized = registry.prioritized_count();
        let readiness_pct = (active as f32 / total as f32) * 100.0;

        ui.label(
            RichText::new("Centralized Feature Registry & Unlock Roadmap")
                .size(18.0)
                .strong()
                .color(theme.text_primary),
        );
        ui.label(
            RichText::new(
                "Phase 31 locks all 15 workspace UI layouts across 74 capabilities. Queue locked modules below to target Phases 32–35 logic unlocks without layout churn.",
            )
            .size(12.0)
            .color(theme.text_secondary),
        );
        ui.add_space(12.0);

        // Responsive 4-card KPI strip
        ui_kit::responsive_grid_n(ui, 4, 155.0, |ui, idx| match idx {
            0 => ui_kit::stat_card(
                ui,
                theme,
                "ACTIVE MODULES",
                &format!("{active} / {total}"),
                "Operational engineering capabilities",
                theme.accent_green,
            ),
            1 => ui_kit::stat_card(
                ui,
                theme,
                "LOCKED WORKSPACE SLOTS",
                &format!("{locked}"),
                "Frozen UI layouts awaiting logic unlock",
                theme.accent_orange,
            ),
            2 => ui_kit::stat_card(
                ui,
                theme,
                "PRIORITIZED FOR UNLOCK",
                &format!("{prioritized}"),
                "Flagged for next unlock sprint",
                theme.accent_blue,
            ),
            _ => ui_kit::stat_card(
                ui,
                theme,
                "SUITE READINESS",
                &format!("{readiness_pct:.0}%"),
                "Core + preview slot coverage",
                theme.text_primary,
            ),
        });

        ui.add_space(12.0);

        // Action & filter toolbar
        theme.card_frame().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.registry_search)
                        .hint_text("Search code, title, module, or pillar...")
                        .desired_width(220.0),
                );

                for (filter, label) in [
                    (RegistryFilter::All, "All"),
                    (RegistryFilter::LockedOnly, "🔒 Locked Only"),
                    (RegistryFilter::PrioritizedOnly, "🎯 Prioritized"),
                    (RegistryFilter::ActiveOnly, "🔓 Active Only"),
                ] {
                    let selected = self.registry_filter == filter;
                    let fill = if selected {
                        theme.accent_blue
                    } else {
                        theme.bg_sidebar
                    };
                    let fg = if selected {
                        Color32::WHITE
                    } else {
                        theme.text_secondary
                    };
                    if ui
                        .add(
                            egui::Button::new(RichText::new(label).size(11.0).color(fg))
                                .fill(fill)
                                .corner_radius(CornerRadius::same(6)),
                        )
                        .clicked()
                    {
                        self.registry_filter = filter;
                    }
                }

                let selected_pillar_label = match self.pillar_filter {
                    Some(p) => p.clean_name().to_string(),
                    None => "All 15 Pillars".to_string(),
                };
                egui::ComboBox::from_id_salt("settings_registry_pillar_filter")
                    .selected_text(selected_pillar_label)
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_label(self.pillar_filter.is_none(), "All 15 Pillars")
                            .clicked()
                        {
                            self.pillar_filter = None;
                        }
                        for &p in NavigationPillar::all() {
                            let label = format!("{} {}", p.icon(), p.clean_name());
                            if ui
                                .selectable_label(self.pillar_filter == Some(p), label)
                                .clicked()
                            {
                                self.pillar_filter = Some(p);
                            }
                        }
                    });
            });

            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    RichText::new("TARGET UNLOCK PHASE:")
                        .size(10.5)
                        .strong()
                        .color(theme.text_muted),
                );
                let all_phases_sel = self.phase_filter.is_none();
                if ui
                    .add(
                        egui::Button::new(RichText::new("All Phases").size(10.5).color(
                            if all_phases_sel {
                                Color32::WHITE
                            } else {
                                theme.text_secondary
                            },
                        ))
                        .fill(if all_phases_sel {
                            theme.accent_blue
                        } else {
                            theme.bg_sidebar
                        })
                        .corner_radius(CornerRadius::same(6)),
                    )
                    .clicked()
                {
                    self.phase_filter = None;
                }
                for &phase in FeatureRegistry::distinct_target_phases() {
                    let count = registry.locked_by_phase_count(phase);
                    let selected = self.phase_filter == Some(phase);
                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new(format!("{phase} ({count})")).size(10.5).color(
                                    if selected {
                                        Color32::WHITE
                                    } else {
                                        theme.text_secondary
                                    },
                                ),
                            )
                            .fill(if selected {
                                theme.accent_blue
                            } else {
                                theme.bg_sidebar
                            })
                            .corner_radius(CornerRadius::same(6)),
                        )
                        .clicked()
                    {
                        self.phase_filter = if selected { None } else { Some(phase) };
                    }
                }
            });

            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add(
                        egui::Button::new(
                            RichText::new("⭐ Prioritize Phase 32")
                                .size(11.0)
                                .color(theme.accent_blue),
                        )
                        .fill(theme.accent_blue.gamma_multiply(0.15))
                        .corner_radius(CornerRadius::same(6)),
                    )
                    .clicked()
                {
                    registry.prioritize_phase("Phase 32");
                }

                if ui
                    .add(
                        egui::Button::new(
                            RichText::new("⭐ Prioritize Phase 33")
                                .size(11.0)
                                .color(theme.accent_blue),
                        )
                        .fill(theme.accent_blue.gamma_multiply(0.15))
                        .corner_radius(CornerRadius::same(6)),
                    )
                    .clicked()
                {
                    registry.prioritize_phase("Phase 33");
                }

                if ui
                    .add(
                        egui::Button::new(
                            RichText::new("🧹 Clear Queue Flags")
                                .size(11.0)
                                .color(theme.text_secondary),
                        )
                        .fill(theme.bg_sidebar)
                        .corner_radius(CornerRadius::same(6)),
                    )
                    .clicked()
                {
                    registry.clear_all_priorities();
                }

                if ui
                    .add(
                        egui::Button::new(
                            RichText::new("📄 Export Unlock Backlog (.md)")
                                .size(11.0)
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .fill(theme.accent_green)
                        .corner_radius(CornerRadius::same(6)),
                    )
                    .clicked()
                {
                    let md = registry.export_unlock_backlog_markdown();
                    match DiskExporter::write_export_file("EigenGraph_Unlock_Backlog.md", &md) {
                        Ok(dest) => {
                            self.export_feedback = Some(format!("Exported unlock backlog -> {dest}"));
                        }
                        Err(err) => {
                            self.export_feedback = Some(format!("Export failed: {err}"));
                        }
                    }
                }

                if ui
                    .add(
                        egui::Button::new(
                            RichText::new("📦 Export Unlock Roadmap (.json)")
                                .size(11.0)
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .fill(theme.accent_blue)
                        .corner_radius(CornerRadius::same(6)),
                    )
                    .clicked()
                {
                    let json = registry.export_unlock_roadmap_json();
                    match DiskExporter::write_export_file("EigenGraph_Unlock_Roadmap.json", &json) {
                        Ok(dest) => {
                            self.export_feedback = Some(format!("Exported structured JSON roadmap -> {dest}"));
                        }
                        Err(err) => {
                            self.export_feedback = Some(format!("JSON export failed: {err}"));
                        }
                    }
                }
            });

            if let Some(msg) = &self.export_feedback {
                ui.add_space(6.0);
                ui.label(RichText::new(msg).size(11.0).color(theme.accent_green));
            }
        });

        ui.add_space(12.0);

        // Domain Pillar Readiness Strip (Click to filter by pillar)
        ui.label(
            RichText::new("15-PILLAR READINESS COVERAGE (CLICK TO FILTER)")
                .size(11.0)
                .strong()
                .color(theme.text_muted),
        );
        ui.add_space(6.0);

        let all_pillars = NavigationPillar::all();
        ui_kit::responsive_grid_n(ui, all_pillars.len(), 210.0, |ui, idx| {
            let pillar = all_pillars[idx];
            let (p_active, p_total) = registry.pillar_counts(pillar);
            let frac = if p_total > 0 {
                p_active as f32 / p_total as f32
            } else {
                0.0
            };
            let is_selected = self.pillar_filter == Some(pillar);
            let stroke = if is_selected {
                Stroke::new(1.5, theme.accent_blue)
            } else {
                Stroke::new(1.0, theme.border_subtle)
            };

            Frame::new()
                .fill(if is_selected {
                    theme.accent_blue.gamma_multiply(0.12)
                } else {
                    theme.bg_card
                })
                .corner_radius(CornerRadius::same(8))
                .stroke(stroke)
                .inner_margin(Margin::same(10))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("{} {}", pillar.icon(), pillar.clean_name()))
                                .size(11.5)
                                .strong()
                                .color(theme.text_primary),
                        );
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(
                                RichText::new(format!("{p_active}/{p_total}"))
                                    .size(10.5)
                                    .strong()
                                    .color(theme.accent_green),
                            );
                        });
                    });
                    ui.add_space(4.0);
                    ui.add(ProgressBar::new(frac).desired_height(5.0));
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        if ui
                            .small_button(if is_selected { "Clear Filter" } else { "Filter" })
                            .clicked()
                        {
                            self.pillar_filter = if is_selected { None } else { Some(pillar) };
                        }
                        if ui.small_button("↗ Open").clicked() {
                            *active_pillar = pillar;
                            jump_clicked = true;
                        }
                    });
                });
        });

        ui.add_space(14.0);

        // Capability Entries List
        let q = self.registry_search.trim().to_lowercase();
        ui.label(
            RichText::new("CAPABILITY SLOTS & UNLOCK TARGETS")
                .size(11.0)
                .strong()
                .color(theme.text_muted),
        );
        ui.add_space(6.0);

        let mut shown_count = 0usize;
        for entry in &mut registry.entries {
            if let Some(pf) = self.pillar_filter {
                if entry.pillar != pf {
                    continue;
                }
            }
            if let Some(phase_target) = self.phase_filter {
                if !entry
                    .status
                    .target_phase()
                    .is_some_and(|p| p.eq_ignore_ascii_case(phase_target))
                {
                    continue;
                }
            }
            match self.registry_filter {
                RegistryFilter::All => {}
                RegistryFilter::LockedOnly => {
                    if entry.status.is_active() {
                        continue;
                    }
                }
                RegistryFilter::PrioritizedOnly => {
                    if entry.status.is_active() || !entry.prioritized {
                        continue;
                    }
                }
                RegistryFilter::ActiveOnly => {
                    if !entry.status.is_active() {
                        continue;
                    }
                }
            }
            if !q.is_empty() {
                let hay = format!(
                    "{} {} {} {} {}",
                    entry.code,
                    entry.title,
                    entry.description,
                    entry.module_hint,
                    entry.pillar.clean_name()
                )
                .to_lowercase();
                if !hay.contains(&q) {
                    continue;
                }
            }

            shown_count += 1;
            let border = match entry.status {
                FeatureStatus::Active => theme.border_subtle,
                FeatureStatus::Locked { .. } if entry.prioritized => {
                    theme.accent_blue.gamma_multiply(0.7)
                }
                FeatureStatus::Locked { .. } => theme.accent_orange.gamma_multiply(0.45),
            };

            Frame::new()
                .fill(theme.bg_card)
                .corner_radius(CornerRadius::same(10))
                .stroke(Stroke::new(1.0, border))
                .inner_margin(Margin::same(12))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal_wrapped(|ui| {
                        ui.label(
                            RichText::new(format!("[{}]", entry.code))
                                .size(11.5)
                                .strong()
                                .color(theme.accent_blue),
                        );
                        ui.label(
                            RichText::new(entry.title)
                                .size(12.5)
                                .strong()
                                .color(theme.text_primary),
                        );
                        ui_kit::feature_status_badge(ui, theme, entry.status);

                        Frame::new()
                            .fill(theme.bg_sidebar)
                            .corner_radius(CornerRadius::same(5))
                            .inner_margin(Margin::symmetric(6, 2))
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(format!(
                                        "{} {}",
                                        entry.pillar.icon(),
                                        entry.pillar.clean_name()
                                    ))
                                    .size(10.0)
                                    .color(theme.text_secondary),
                                );
                            });
                    });

                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(entry.description)
                            .size(11.5)
                            .color(theme.text_secondary),
                    );
                    ui.add_space(4.0);

                    ui.horizontal_wrapped(|ui| {
                        ui.label(
                            RichText::new(format!("Module: {}", entry.module_hint))
                                .size(10.5)
                                .color(theme.text_muted),
                        );

                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new("↗ Inspect Layout")
                                            .size(10.5)
                                            .color(theme.text_primary),
                                    )
                                    .fill(theme.bg_sidebar)
                                    .corner_radius(CornerRadius::same(6)),
                                )
                                .clicked()
                            {
                                *active_pillar = entry.pillar;
                                jump_clicked = true;
                            }

                            if !entry.status.is_active() {
                                let label = if entry.prioritized {
                                    "🎯 Prioritized for Unlock"
                                } else {
                                    "☆ Queue for Unlock"
                                };
                                let fill = if entry.prioritized {
                                    theme.accent_blue.gamma_multiply(0.25)
                                } else {
                                    theme.bg_sidebar
                                };
                                let fg = if entry.prioritized {
                                    theme.accent_blue
                                } else {
                                    theme.text_secondary
                                };
                                if ui
                                    .add(
                                        egui::Button::new(RichText::new(label).size(10.5).strong().color(fg))
                                            .fill(fill)
                                            .corner_radius(CornerRadius::same(6)),
                                    )
                                    .clicked()
                                {
                                    entry.prioritized = !entry.prioritized;
                                }
                            }
                        });
                    });
                });
            ui.add_space(8.0);
        }

        if shown_count == 0 {
            theme.card_frame().show(ui, |ui| {
                ui.label(
                    RichText::new("No capabilities match the current filter criteria.")
                        .size(12.0)
                        .color(theme.text_muted),
                );
            });
        }

        jump_clicked
    }

    fn render_preferences(&mut self, ui: &mut Ui, theme: &MacTheme) {
        ui.heading(
            RichText::new("⚙️ System Preferences")
                .size(20.0)
                .strong()
                .color(theme.text_primary),
        );
        ui.label(
            RichText::new("Configure engineering units, solver safeguards, and AI copilot credentials.")
                .size(12.5)
                .color(theme.text_secondary),
        );
        ui.add_space(12.0);

        ui_kit::responsive_two_columns(
            ui,
            640.0,
            |ui| {
                theme.card_frame().show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.label(
                        RichText::new("ENGINEERING UNITS & DESIGN STANDARDS")
                            .strong()
                            .size(11.5)
                            .color(theme.accent_blue),
                    );
                    ui.separator();
                    ui.add_space(8.0);

                    ui.label(RichText::new("Active Unit System:").size(12.5).color(theme.text_primary));
                    egui::ComboBox::from_id_salt("settings_unit_system")
                        .selected_text(&self.unit_system)
                        .width(ui.available_width().min(280.0))
                        .show_ui(ui, |ui| {
                            for opt in [
                                "kN, mm, MPa (Structural Metric)",
                                "kN, m, kPa (Civil / Geotech Metric)",
                                "kips, in, ksi (US Imperial)",
                            ] {
                                ui.selectable_value(&mut self.unit_system, opt.to_string(), opt);
                            }
                        });

                    ui.add_space(10.0);
                    ui.label(RichText::new("Design Standard:").size(12.5).color(theme.text_primary));
                    egui::ComboBox::from_id_salt("settings_design_standard")
                        .selected_text(&self.design_standard)
                        .width(ui.available_width().min(280.0))
                        .show_ui(ui, |ui| {
                            for opt in [
                                "SNI 1729 / AISC 360-16",
                                "SNI 2847:2019 / ACI 318-19",
                                "Eurocode 2 / Eurocode 3 / EN 1998",
                            ] {
                                ui.selectable_value(&mut self.design_standard, opt.to_string(), opt);
                            }
                        });
                });
            },
            |ui| {
                theme.card_frame().show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.label(
                        RichText::new("SOLVER & NUMERICAL SAFEGUARDS")
                            .strong()
                            .size(11.5)
                            .color(theme.accent_blue),
                    );
                    ui.separator();
                    ui.add_space(8.0);

                    ui.checkbox(
                        &mut self.enable_svd_guard,
                        RichText::new("Enable SVD Kinematic Guard (Sherman-Morrison margin check)")
                            .color(theme.text_primary),
                    );
                    ui.label(
                        RichText::new("Instantly detects mechanism formation upon N-1 member damage without pseudoinverse.")
                            .size(11.0)
                            .color(theme.text_muted),
                    );

                    ui.add_space(8.0);
                    ui.checkbox(
                        &mut self.enable_anti_leakage,
                        RichText::new("Enforce Strict Pre-Analysis Protocol (Leakage-Free)")
                            .color(theme.text_primary),
                    );
                    ui.label(
                        RichText::new("Guarantees screening upper bounds are computed 100% from intact structure facts.")
                            .size(11.0)
                            .color(theme.text_muted),
                    );

                    ui.add_space(8.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("Parallel Solver Threads:").color(theme.text_primary));
                        ui.add(egui::Slider::new(&mut self.solver_threads, 1..=32));
                    });
                });
            },
        );

        ui.add_space(12.0);
        theme.card_frame().show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                RichText::new("AI COPILOT INTEGRATION")
                    .strong()
                    .size(11.5)
                    .color(theme.accent_blue),
            );
            ui.separator();
            ui.add_space(8.0);

            ui.label(RichText::new("Google Gemini API Key:").size(12.5).color(theme.text_primary));
            ui.add(
                egui::TextEdit::singleline(&mut self.gemini_api_key)
                    .password(true)
                    .desired_width(ui.available_width().min(380.0)),
            );
            ui.label(
                RichText::new(
                    "Stored in-session for upcoming live LLM BIM tool orchestration (Feature AI-03).",
                )
                .size(11.0)
                .color(theme.text_muted),
            );
        });
    }

    fn render_about(&self, ui: &mut Ui, theme: &MacTheme) {
        theme.card_frame().show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                RichText::new("ABOUT EIGENGRAPH")
                    .strong()
                    .size(12.0)
                    .color(theme.accent_blue),
            );
            ui.separator();
            ui.add_space(8.0);

            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Version:").strong().size(13.0).color(theme.text_primary));
                ui.label(
                    RichText::new(format!("v{} — Phase 31 Full-Suite Master Workspace", env!("CARGO_PKG_VERSION")))
                        .size(13.0)
                        .color(theme.accent_green)
                        .strong(),
                );
            });

            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Runtime:").strong().size(12.0).color(theme.text_primary));
                let runtime = if cfg!(target_arch = "wasm32") {
                    "Browser (WebGL2) • WebAssembly (wasm32)"
                } else {
                    "EigenGraph Native & Web Engine"
                };
                ui.label(RichText::new(runtime).size(12.0).color(theme.text_secondary));
            });

            ui.add_space(8.0);
            ui.label(
                RichText::new("Theoretical & Computational Foundations:")
                    .strong()
                    .size(12.0)
                    .color(theme.text_primary),
            );
            ui.label(
                RichText::new(
                    "• Linear Adjusted Redundancy Index (ARI) with Sherman-Morrison O(N^2) fast update\n\
                     • Woodbury Rank-2 (N-2) simultaneous multi-element disruption screening\n\
                     • Euler elastic buckling & AISC 360-16 B1 moment amplification geometric stiffness guard\n\
                     • SVD Kinematic Mechanism Guard (m_e = 1 - h_e) for instant singularity detection\n\
                     • Planar Half-Edge Face Arrangement & Non-Recursive BSP Tree Solid CSG Subtraction\n\
                     • 2D Sheet Composer (A0–A3) with Vector PDF 1.4, DXF & SVG Blueprint Generation\n\
                     • Responsive 15-Pillar Master Workspace with Centralized Feature Unlock Registry",
                )
                .size(11.0)
                .color(theme.text_muted),
            );

            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Design Standards:").strong().size(11.5).color(theme.text_primary));
                ui.label(
                    RichText::new("AISC 360-16 • ACI 318-19 • SNI 1729:2020 • SNI 2847:2019 • SNI 1726:2019")
                        .size(11.0)
                        .color(theme.text_secondary),
                );
            });

            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Integrity & License:").strong().size(11.5).color(theme.text_primary));
                ui.label(
                    RichText::new("NIST FIPS 180-4 SHA-256 Validated • Dual MIT / Apache 2.0")
                        .size(11.0)
                        .color(theme.text_secondary),
                );
            });

            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Repository:").strong().size(11.5).color(theme.text_primary));
                ui.hyperlink_to(
                    "github.com/haridsirawan/EigenGraph",
                    "https://github.com/haridsirawan/EigenGraph",
                );
            });
        });
    }
}
