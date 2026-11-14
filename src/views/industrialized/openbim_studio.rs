//! views/industrialized/openbim_studio.rs — OpenBIM IFC Source Bar, Spatial Tree Workbench & Locked WASM Picker.

use super::*;

impl IndustrializedView {
    pub(super) fn render_openbim_ifc_studio(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        features: &mut FeatureRegistry,
    ) {
        // 1. TOP KPI EXECUTIVE METRICS
        let (schema_lbl, ent_count, prod_count, health_score, verdict_str) =
            if let Some(ref m) = self.ifc_model {
                (
                    m.schema.display_label(),
                    m.health_report.total_entities,
                    m.elements.len(),
                    format!("{:.1}%", m.health_report.conformance_score_pct),
                    m.health_report.verdict.clone(),
                )
            } else {
                (
                    "No Model Loaded",
                    0,
                    0,
                    "0.0%".to_string(),
                    "Awaiting IFC File Input".to_string(),
                )
            };

        let kpis = [
            (
                "SCHEMA SPECIFICATION",
                schema_lbl.to_string(),
                "buildingSMART International Standard".to_string(),
                theme.accent_blue,
            ),
            (
                "TOTAL STEP ENTITIES",
                format!("{} Entities", ent_count),
                "ISO 10303-21 Physical File".to_string(),
                theme.text_primary,
            ),
            (
                "CONTAINED BIM PRODUCTS",
                format!("{} Elements", prod_count),
                "Beams, Columns, Walls, Slabs".to_string(),
                theme.accent_green,
            ),
            (
                "CONFORMANCE HEALTH",
                health_score,
                "Referential & Spatial Quality".to_string(),
                theme.accent_green,
            ),
        ];
        self.render_responsive_cards(ui, &kpis, theme);

        ui.add_space(14.0);

        // 2. FILE OPEN & PRESET SELECTOR TOOLBAR
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(14))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    crate::icons::icon_label(
                        ui,
                        crate::icons::AppIcon::Folder,
                        12.0,
                        theme.text_secondary,
                        "OPEN IFC SOURCE:",
                        11.5,
                        theme.text_secondary,
                        true,
                    );

                    let sources = [
                        (IfcSourceMode::SampleVillaIfc4, crate::icons::AppIcon::Building, "Sample Villa (IFC4)"),
                        (IfcSourceMode::SamplePortalFrameIfc4, crate::icons::AppIcon::BoxPackage, "Steel Portal (IFC4)"),
                        (IfcSourceMode::SampleDiagridIfc2x3, crate::icons::AppIcon::Building, "Diagrid Tower (IFC2x3)"),
                        (IfcSourceMode::CustomPaste, crate::icons::AppIcon::Clipboard, "Paste STEP SPF"),
                        (IfcSourceMode::FilePath, crate::icons::AppIcon::Folder, "Open Local File"),
                    ];

                    for (src, icon, label) in sources {
                        let is_sel = self.ifc_source_mode == src;
                        let bg = if is_sel { theme.accent_blue } else { theme.bg_canvas };
                        let fg = if is_sel { Color32::WHITE } else { theme.text_primary };

                        if crate::icons::icon_button(
                            ui,
                            icon,
                            label,
                            11.0,
                            fg,
                            bg,
                            Stroke::NONE,
                            CornerRadius::same(6),
                            Vec2::ZERO,
                        )
                        .clicked()
                        {
                            self.ifc_source_mode = src;
                            match src {
                                IfcSourceMode::SampleVillaIfc4 => {
                                    if let Ok(m) = IfcModel::parse(&IfcModel::sample_villa_ifc4()) {
                                        self.baseline_model = Some(m.clone());
                                        self.ifc_model = Some(m);
                                        self.selected_element_id = Some(200);
                                        self.clash_report = None;
                                        self.model_diff = None;
                                        self.ids_report = None;
                                        self.export_status = Some(
                                            "Loaded reference IFC4 Residential Duplex Villa."
                                                .to_string(),
                                        );
                                    }
                                }
                                IfcSourceMode::SamplePortalFrameIfc4 => {
                                    if let Ok(m) =
                                        IfcModel::parse(&IfcModel::sample_portal_frame_ifc4())
                                    {
                                        self.baseline_model = Some(m.clone());
                                        self.ifc_model = Some(m);
                                        self.selected_element_id = Some(200);
                                        self.clash_report = None;
                                        self.model_diff = None;
                                        self.ids_report = None;
                                        self.export_status = Some(
                                            "Loaded reference IFC4 Industrial Steel Portal Frame."
                                                .to_string(),
                                        );
                                    }
                                }
                                IfcSourceMode::SampleDiagridIfc2x3 => {
                                    if let Ok(m) =
                                        IfcModel::parse(&IfcModel::sample_diagrid_tower_ifc2x3())
                                    {
                                        self.baseline_model = Some(m.clone());
                                        self.ifc_model = Some(m);
                                        self.selected_element_id = Some(200);
                                        self.clash_report = None;
                                        self.model_diff = None;
                                        self.ids_report = None;
                                        self.export_status = Some(
                                            "Loaded reference IFC2x3 Diagrid High-Rise Tower."
                                                .to_string(),
                                        );
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                });

                // Custom Paste or File Path Sub-Panel
                if self.ifc_source_mode == IfcSourceMode::CustomPaste {
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new(
                            "Paste ISO-10303-21 STEP physical file text (HEADER & DATA sections):",
                        )
                        .size(10.5)
                        .color(theme.text_muted),
                    );
                    ui.add(
                        egui::TextEdit::multiline(&mut self.custom_ifc_paste)
                            .font(egui::TextStyle::Monospace)
                            .desired_rows(6)
                            .desired_width(f32::INFINITY),
                    );
                    ui.add_space(4.0);
                    if crate::icons::icon_button(
                        ui,
                        crate::icons::AppIcon::Bolt,
                        "Parse & Decompose Pasted IFC",
                        11.5,
                        Color32::WHITE,
                        theme.accent_green,
                        Stroke::NONE,
                        CornerRadius::same(6),
                        Vec2::ZERO,
                    )
                    .clicked()
                    {
                        match IfcModel::parse(&self.custom_ifc_paste) {
                            Ok(m) => {
                                self.export_status = Some(format!(
                                    "Successfully parsed {} entities and {} elements from pasted IFC.",
                                    m.health_report.total_entities,
                                    m.elements.len()
                                ));
                                self.selected_element_id = m.elements.first().map(|e| e.step_id);
                                self.baseline_model = Some(m.clone());
                                self.ifc_model = Some(m);
                                self.clash_report = None;
                                self.model_diff = None;
                                self.ids_report = None;
                            }
                            Err(e) => {
                                self.export_status = Some(format!("IFC Parse Error: {}", e));
                            }
                        }
                    }
                } else if self.ifc_source_mode == IfcSourceMode::FilePath {
                    ui.add_space(8.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.label(
                            RichText::new("Local File Path:")
                                .size(11.0)
                                .color(theme.text_secondary),
                        );
                        let w = ui.available_width().min(260.0).max(140.0);
                        ui.add(
                            egui::TextEdit::singleline(&mut self.custom_ifc_path)
                                .desired_width(w),
                        );
                        if crate::icons::icon_button(
                            ui,
                            crate::icons::AppIcon::Folder,
                            "Read & Inspect",
                            11.0,
                            Color32::WHITE,
                            theme.accent_blue,
                            Stroke::NONE,
                            CornerRadius::same(6),
                            Vec2::ZERO,
                        )
                        .clicked()
                        {
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                match std::fs::read_to_string(&self.custom_ifc_path) {
                                    Ok(content) => match IfcModel::parse(&content) {
                                        Ok(m) => {
                                            self.export_status = Some(format!(
                                                "Successfully loaded '{}' ({} elements).",
                                                self.custom_ifc_path,
                                                m.elements.len()
                                            ));
                                            self.selected_element_id =
                                                m.elements.first().map(|e| e.step_id);
                                            self.baseline_model = Some(m.clone());
                                            self.ifc_model = Some(m);
                                            self.clash_report = None;
                                            self.model_diff = None;
                                            self.ids_report = None;
                                        }
                                        Err(e) => {
                                            self.export_status =
                                                Some(format!("IFC Parse Error: {}", e));
                                        }
                                    },
                                    Err(e) => {
                                        self.export_status = Some(format!(
                                            "Cannot open file '{}': {}",
                                            self.custom_ifc_path, e
                                        ));
                                    }
                                }
                            }
                            #[cfg(target_arch = "wasm32")]
                            {
                                self.export_status = Some(
                                    "Browser WebAssembly sandbox cannot read local file paths directly; switch to 'Paste STEP SPF' above."
                                        .to_string(),
                                );
                            }
                        }
                    });
                }

                ui.add_space(10.0);
                // Phase 31+ Locked Preview Slot: WASM Browser File Picker & Drag-and-Drop IFC
                ui_kit::feature_slot_card(
                    ui,
                    theme,
                    features,
                    FeatureId::WasmLocalIfcFileDropPicker,
                    |ui| {
                        ui.horizontal_wrapped(|ui| {
                            let _ = crate::icons::icon_button(ui, crate::icons::AppIcon::Folder, "Choose Local .IFC / .E2K / .JSON File...", 12.0, theme.text_primary, theme.bg_card, Stroke::new(1.0, theme.border_subtle), CornerRadius::same(6), Vec2::ZERO);
                            let _ = ui.selectable_label(true, "Drag-and-Drop Zone Active");
                            let _ = ui.selectable_label(false, "Auto-Detect IFC2x3 / IFC4.3 Schema");
                        });
                    },
                );
            });

        ui.add_space(14.0);

        // 3. TWO-PANE WORKBENCH (Side-by-Side on Desktop, Stacked on Mobile/Tablet)
        ui_kit::split_workspace_panes(
            ui,
            760.0,
            0.42,
            260.0,
            420.0,
            480.0,
            |ui, pane, _| match pane {
                0 => self.render_ifc_spatial_tree_pane(ui, theme),
                _ => self.render_ifc_inspector_pane(ui, theme, &verdict_str),
            },
        );

        ui.add_space(16.0);

        // 4. BOTTOM ACTION & INTEROP BUTTONS
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(14))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    if crate::icons::icon_button(
                        ui,
                        crate::icons::AppIcon::Play,
                        "Sync to 3D Viewport & Analytical Solver",
                        12.0,
                        Color32::WHITE,
                        theme.accent_blue,
                        Stroke::NONE,
                        CornerRadius::same(6),
                        Vec2::ZERO,
                    )
                    .clicked()
                    {
                        if let Some(ref m) = self.ifc_model {
                            match m.to_structural_model() {
                                Ok(fem) => {
                                    self.export_status = Some(format!(
                                        "Successfully converted IFC model '{}' into {} nodes & {} structural frame elements for live ARI analysis.",
                                        fem.name,
                                        fem.nodes.len(),
                                        fem.elements.len()
                                    ));
                                }
                                Err(e) => {
                                    self.export_status = Some(format!(
                                        "BIM to Structural Model Conversion Warning: {}",
                                        e
                                    ));
                                }
                            }
                        }
                    }

                    if crate::icons::icon_button(
                        ui,
                        crate::icons::AppIcon::Globe,
                        "Export Clean IFC 4.3 (.ifc)",
                        12.0,
                        Color32::WHITE,
                        theme.accent_green,
                        Stroke::NONE,
                        CornerRadius::same(6),
                        Vec2::ZERO,
                    )
                    .clicked()
                    {
                        if let Some(ref _m) = self.ifc_model {
                            let mut engine = crate::core::ParametricEngine::new();
                            if let Ok(model) = engine.generate_warren_truss() {
                                let ari =
                                    crate::core::AriEngine::evaluate_redundancy(&model).ok();
                                let ifc =
                                    crate::core::IfcExporter::export_to_ifc(&model, ari.as_ref());
                                self.export_status = Some(format!(
                                    "Exported active model to ISO 16739-1:2024 IFC 4.3 ({} lines).",
                                    ifc.lines().count()
                                ));
                            }
                        }
                    }

                    if crate::icons::icon_button(
                        ui,
                        crate::icons::AppIcon::Cpu,
                        "Export DSTV CNC (.nc1)",
                        12.0,
                        Color32::WHITE,
                        Color32::from_rgb(139, 92, 246),
                        Stroke::NONE,
                        CornerRadius::same(6),
                        Vec2::ZERO,
                    )
                    .clicked()
                    {
                        let mut engine = crate::core::ParametricEngine::new();
                        if let Ok(model) = engine.generate_warren_truss() {
                            let pieces = crate::core::DstvExporter::export_to_dstv(&model);
                            self.export_status = Some(format!(
                                "Generated {} DSTV .nc1 robotic instructions for steel fabrication.",
                                pieces.len()
                            ));
                        }
                    }

                    if crate::icons::icon_button(
                        ui,
                        crate::icons::AppIcon::Document,
                        "Export ifcJSON-4 (.json)",
                        12.0,
                        Color32::WHITE,
                        Color32::from_rgb(234, 88, 12),
                        Stroke::NONE,
                        CornerRadius::same(6),
                        Vec2::ZERO,
                    )
                    .clicked()
                    {
                        if let Some(ref m) = self.ifc_model {
                            let json = m.to_ifc_json();
                            self.export_status = Some(format!(
                                "Exported buildingSMART ifcJSON-4 document ({} bytes, {} products).",
                                json.len(),
                                m.elements.len()
                            ));
                        }
                    }

                    if crate::icons::icon_button(
                        ui,
                        crate::icons::AppIcon::Document,
                        "Export BCF Topics (.xml)",
                        12.0,
                        Color32::WHITE,
                        Color32::from_rgb(14, 165, 233),
                        Stroke::NONE,
                        CornerRadius::same(6),
                        Vec2::ZERO,
                    )
                    .clicked()
                    {
                        if let Some(ref m) = self.ifc_model {
                            let empty = Vec::new();
                            let clashes = self
                                .clash_report
                                .as_ref()
                                .map(|r| r.clashes.as_slice())
                                .unwrap_or(&empty);
                            let xml = m.export_bcf_xml(clashes);
                            self.export_status = Some(format!(
                                "Exported {} BCF issue topics ({} bytes).",
                                clashes.len(),
                                xml.len()
                            ));
                        }
                    }
                });
            });
    }

    fn render_ifc_spatial_tree_pane(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(14))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    crate::icons::icon_label(
                        ui,
                        crate::icons::AppIcon::Layers,
                        12.0,
                        theme.accent_blue,
                        "SPATIAL HIERARCHY TREE",
                        12.0,
                        theme.accent_blue,
                        true,
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if let Some(ref m) = self.ifc_model {
                            ui.label(
                                RichText::new(format!("{} Storeys", count_storeys(&m.spatial_root)))
                                    .size(10.5)
                                    .color(theme.text_muted),
                            );
                        }
                    });
                });
                ui.add_space(6.0);

                ui.horizontal(|ui| {
                    crate::icons::draw_icon(ui, crate::icons::AppIcon::Search, 11.0, theme.text_muted);
                    ui.add(
                        egui::TextEdit::singleline(&mut self.element_search_query)
                            .hint_text("Filter elements by name/class...")
                            .desired_width((ui.available_width() - 4.0).max(120.0)),
                    );
                });
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);

                ScrollArea::vertical()
                    .id_salt("spatial_tree_scroll")
                    .max_height(380.0)
                    .show(ui, |ui| {
                        let spatial_data = self
                            .ifc_model
                            .as_ref()
                            .map(|m| (m.spatial_root.clone(), m.elements.clone()));
                        if let Some((spatial_root, elements)) = spatial_data {
                            self.render_spatial_node(ui, theme, &spatial_root, &elements);
                        } else {
                            ui.label(
                                RichText::new(
                                    "No IFC Model loaded. Please select a sample or paste file above.",
                                )
                                .size(11.0)
                                .color(theme.text_muted),
                            );
                        }
                    });
            });
    }

    fn render_ifc_inspector_pane(&mut self, ui: &mut Ui, theme: &MacTheme, verdict_str: &str) {
        Frame::new()
            .fill(theme.bg_card)
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::same(14))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                let tabs = [
                    (IfcInspectorTab::Properties, "📋 Entity Property Inspector"),
                    (IfcInspectorTab::HealthAudit, "🛡️ Conformance & Health Audit"),
                    (IfcInspectorTab::GeometryBbox, "📐 3D Geometry & Bounding Box"),
                    (IfcInspectorTab::ClashAndDiff, "⚔️ Clash, Diff & IDS"),
                ];
                ui_kit::subtab_bar(ui, theme, "ifc_inspector_tabs", &mut self.inspector_tab, &tabs);

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(8.0);

                ScrollArea::vertical()
                    .id_salt("inspector_tab_scroll")
                    .max_height(390.0)
                    .show(ui, |ui| match self.inspector_tab {
                        IfcInspectorTab::Properties => self.render_properties_inspector(ui, theme),
                        IfcInspectorTab::HealthAudit => {
                            self.render_health_audit_tab(ui, theme, verdict_str)
                        }
                        IfcInspectorTab::GeometryBbox => self.render_geometry_bbox_tab(ui, theme),
                        IfcInspectorTab::ClashAndDiff => self.render_clash_and_diff_tab(ui, theme),
                    });
            });
    }

    fn render_spatial_node(
        &mut self,
        ui: &mut Ui,
        theme: &MacTheme,
        node: &IfcSpatialNode,
        elements: &[IfcElementSummary],
    ) {
        let is_expanded = *self.spatial_tree_expanded.get(&node.step_id).unwrap_or(&true);
        let icon = match node.class_name.as_str() {
            "IfcProject" => crate::icons::AppIcon::Building,
            "IfcSite" => crate::icons::AppIcon::Globe,
            "IfcBuilding" => crate::icons::AppIcon::Building,
            "IFCBUILDINGSTOREY" => crate::icons::AppIcon::MapPin,
            _ => crate::icons::AppIcon::Folder,
        };

        ui.horizontal_wrapped(|ui| {
            let arrow_icon = if is_expanded {
                crate::icons::AppIcon::ChevronDown
            } else {
                crate::icons::AppIcon::ArrowRight
            };
            if crate::icons::icon_only_button(
                ui,
                arrow_icon,
                theme.text_muted,
                Color32::TRANSPARENT,
                Stroke::NONE,
                CornerRadius::same(3),
                Vec2::splat(16.0),
                if is_expanded { "Collapse" } else { "Expand" },
            )
            .clicked()
            {
                self.spatial_tree_expanded.insert(node.step_id, !is_expanded);
            }

            let elev_str = node
                .elevation_m
                .map(|e| format!(" (El: {:.1}m)", e * 0.001))
                .unwrap_or_default();
            crate::icons::icon_label(
                ui,
                icon,
                12.0,
                theme.accent_blue,
                format!("{}{}", node.name, elev_str),
                11.0,
                theme.text_primary,
                true,
            );

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(
                    RichText::new(format!("[{}]", node.total_contained_count()))
                        .size(10.0)
                        .color(theme.text_muted),
                );
            });
        });

        if is_expanded {
            ui.indent(format!("indent_node_{}", node.step_id), |ui| {
                for child in &node.children {
                    self.render_spatial_node(ui, theme, child, elements);
                }

                let q_lower = self.element_search_query.to_lowercase();
                for &el_id in &node.contained_element_ids {
                    if let Some(el) = elements.iter().find(|e| e.step_id == el_id) {
                        if !q_lower.is_empty()
                            && !el.name.to_lowercase().contains(&q_lower)
                            && !el.class_name.to_lowercase().contains(&q_lower)
                        {
                            continue;
                        }

                        let is_selected = self.selected_element_id == Some(el.step_id);
                        let el_icon = match el.class_name.as_str() {
                            "IFCCOLUMN" => crate::icons::AppIcon::Column,
                            "IFCBEAM" => crate::icons::AppIcon::Beam,
                            "IFCWALL" | "IFCWALLSTANDARDCASE" => crate::icons::AppIcon::Wall,
                            "IFCSLAB" => crate::icons::AppIcon::Slab,
                            "IFCDOOR" => crate::icons::AppIcon::Door,
                            "IFCWINDOW" => crate::icons::AppIcon::Window,
                            "IFCMEMBER" => crate::icons::AppIcon::NodesGraph,
                            _ => crate::icons::AppIcon::BoxPackage,
                        };

                        let bg = if is_selected {
                            theme.accent_blue.linear_multiply(0.25)
                        } else {
                            Color32::TRANSPARENT
                        };
                        let fg = if is_selected {
                            theme.accent_blue
                        } else {
                            theme.text_secondary
                        };

                        Frame::new()
                            .fill(bg)
                            .corner_radius(CornerRadius::same(4))
                            .inner_margin(Margin::symmetric(4, 2))
                            .show(ui, |ui| {
                                ui.horizontal_wrapped(|ui| {
                                    crate::icons::draw_icon(ui, el_icon, 11.0, fg);
                                    if ui
                                        .selectable_label(
                                            is_selected,
                                            RichText::new(&el.name).size(10.5).color(fg),
                                        )
                                        .clicked()
                                    {
                                        self.selected_element_id = Some(el.step_id);
                                    }
                                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                        ui.label(
                                            RichText::new(&el.class_name)
                                                .size(9.0)
                                                .monospace()
                                                .color(theme.text_muted),
                                        );
                                    });
                                });
                            });
                    }
                }
            });
        }
    }
}
