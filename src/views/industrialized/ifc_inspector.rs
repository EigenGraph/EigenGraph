//! views/industrialized/ifc_inspector.rs — Entity Property Editor, Conformance Audit, 3D BBox Mutator & Clash/Diff/IDS.

use super::*;

impl IndustrializedView {
    pub(super) fn render_properties_inspector(&mut self, ui: &mut Ui, theme: &MacTheme) {
        enum InspectorAction {
            ToggleEdit,
            Undo,
            Redo,
            UpdateAttributes { id: usize, name: String, desc: String, obj_type: String },
            SetMaterial { id: usize, mat: String },
            ReassignStorey { id: usize, target_storey_id: usize },
            SetProperty { id: usize, pset: String, prop: String, val: String },
            RemoveProperty { id: usize, pset: String, prop: String },
            AddPset { id: usize, pset: String, props: Vec<(String, String)> },
            DeletePset { id: usize, pset: String },
            ExportMutatedStep,
        }

        let mut action: Option<InspectorAction> = None;

        // 1. Transaction & Mode Control Bar
        ui.horizontal_wrapped(|ui| {
            let (edit_icon, edit_txt) = if self.is_editing {
                (crate::icons::AppIcon::SlidersPanel, "View Mode")
            } else {
                (crate::icons::AppIcon::Pencil, "Edit Entity")
            };
            let edit_bg = if self.is_editing { theme.accent_blue } else { theme.bg_canvas };
            let edit_fg = if self.is_editing { Color32::WHITE } else { theme.text_primary };

            if crate::icons::icon_button(
                ui,
                edit_icon,
                edit_txt,
                11.0,
                edit_fg,
                edit_bg,
                Stroke::NONE,
                CornerRadius::same(6),
                Vec2::ZERO,
            ).clicked() {
                action = Some(InspectorAction::ToggleEdit);
            }

            let can_undo = self.edit_session.can_undo();
            let undo_lbl = if let Some(desc) = self.edit_session.undo_description() {
                format!("Undo ({})", desc)
            } else {
                "Undo".to_string()
            };
            ui.add_enabled_ui(can_undo, |ui| {
                if crate::icons::icon_button(
                    ui,
                    crate::icons::AppIcon::Undo,
                    undo_lbl,
                    10.5,
                    if can_undo { theme.text_primary } else { theme.text_muted },
                    theme.bg_canvas,
                    Stroke::NONE,
                    CornerRadius::same(6),
                    Vec2::ZERO,
                ).clicked() {
                    action = Some(InspectorAction::Undo);
                }
            });

            let can_redo = self.edit_session.can_redo();
            let redo_lbl = if let Some(desc) = self.edit_session.redo_description() {
                format!("Redo ({})", desc)
            } else {
                "Redo".to_string()
            };
            ui.add_enabled_ui(can_redo, |ui| {
                if crate::icons::icon_button(
                    ui,
                    crate::icons::AppIcon::Redo,
                    redo_lbl,
                    10.5,
                    if can_redo { theme.text_primary } else { theme.text_muted },
                    theme.bg_canvas,
                    Stroke::NONE,
                    CornerRadius::same(6),
                    Vec2::ZERO,
                ).clicked() {
                    action = Some(InspectorAction::Redo);
                }
            });

            if self.edit_session.is_dirty {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.horizontal(|ui| {
                        crate::icons::icon_label(ui, crate::icons::AppIcon::Info, 11.0, theme.accent_orange, "MODIFIED", 10.0, theme.accent_orange, true);
                    });
                });
            }
        });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(6.0);

        let el_opt = if let Some(ref m) = self.ifc_model {
            m.elements.iter().find(|e| Some(e.step_id) == self.selected_element_id).cloned()
        } else {
            None
        };

        let storeys = if let Some(ref m) = self.ifc_model {
            m.list_storeys()
        } else {
            Vec::new()
        };

        if let Some(el) = el_opt {
            if !self.is_editing {
                // READ-ONLY INSPECTION VIEW
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("TAG:").size(11.0).strong().color(theme.text_muted));
                    ui.label(RichText::new(format!("#{}", el.step_id)).size(11.0).monospace().color(theme.accent_blue));
                    ui.label(RichText::new("CLASS:").size(11.0).strong().color(theme.text_muted));
                    ui.label(RichText::new(&el.class_name).size(11.0).strong().color(theme.accent_green));
                    ui.label(RichText::new("STOREY:").size(11.0).strong().color(theme.text_muted));
                    ui.label(RichText::new(&el.storey_name).size(11.0).color(theme.text_primary));
                });
                ui.add_space(4.0);

                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("NAME:").size(11.0).strong().color(theme.text_muted));
                    ui.label(RichText::new(&el.name).size(12.0).strong().color(theme.text_primary));
                });

                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("GUID:").size(10.0).strong().color(theme.text_muted));
                    ui.label(RichText::new(&el.global_id).size(10.0).monospace().color(theme.text_secondary));
                });

                if let Some(ref mat) = el.material_name {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("MATERIAL:").size(11.0).strong().color(theme.text_muted));
                        ui.label(RichText::new(mat).size(11.0).strong().color(theme.accent_orange));
                    });
                }

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(8.0);

                // Property Sets (Psets)
                crate::icons::icon_label(ui, crate::icons::AppIcon::Document, 12.0, theme.accent_blue, format!("PROPERTY SETS ({} Sets Assigned)", el.property_sets.len()), 11.5, theme.accent_blue, true);
                ui.add_space(4.0);

                if el.property_sets.is_empty() {
                    ui.label(RichText::new("No explicit IfcPropertySet attached to this entity.").size(10.5).color(theme.text_muted));
                } else {
                    for pset in &el.property_sets {
                        Frame::new()
                            .fill(theme.bg_canvas)
                            .corner_radius(CornerRadius::same(6))
                            .inner_margin(Margin::same(8))
                            .show(ui, |ui| {
                                ui.label(RichText::new(&pset.name).size(11.0).strong().color(theme.text_primary));
                                ui.add_space(4.0);
                                for (p_name, p_val) in &pset.properties {
                                    ui.horizontal_wrapped(|ui| {
                                        ui.label(RichText::new(format!("• {}:", p_name)).size(10.5).color(theme.text_secondary));
                                        ui.label(RichText::new(p_val).size(10.5).strong().color(theme.accent_blue));
                                    });
                                }
                            });
                        ui.add_space(4.0);
                    }
                }

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);

                // Element Quantities (Qto)
                crate::icons::icon_label(ui, crate::icons::AppIcon::ChartBar, 12.0, theme.accent_green, "QUANTITIES TAKEOFF (Qto)", 11.5, theme.accent_green, true);
                ui.add_space(4.0);
                if el.quantities.is_empty() {
                    let (dx, dy, dz) = el.bbox.dimensions();
                    let approx_vol = el.bbox.volume();
                    ui.label(RichText::new(format!("BBox Dimensions: {:.2} x {:.2} x {:.2} m • Volume: {:.3} m³", dx, dy, dz, approx_vol)).size(10.5).color(theme.text_secondary));
                } else {
                    for (q_name, q_val) in &el.quantities {
                        ui.horizontal_wrapped(|ui| {
                            ui.label(RichText::new(format!("• {}:", q_name)).size(10.5).color(theme.text_secondary));
                            ui.label(RichText::new(format!("{:.3}", q_val)).size(10.5).strong().color(theme.accent_green));
                        });
                    }
                }
            } else {
                // INTERACTIVE EDITING MODE
                crate::icons::icon_label(ui, crate::icons::AppIcon::Pencil, 12.0, theme.accent_blue, "EDIT ELEMENT ATTRIBUTES", 11.5, theme.accent_blue, true);
                ui.add_space(6.0);

                // 1. Identity attributes
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Name:").size(11.0).color(theme.text_secondary));
                    ui.add(egui::TextEdit::singleline(&mut self.edit_name).desired_width(220.0));
                });
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Description:").size(11.0).color(theme.text_secondary));
                    ui.add(egui::TextEdit::singleline(&mut self.edit_desc).desired_width(220.0));
                });
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("ObjectType:").size(11.0).color(theme.text_secondary));
                    ui.add(egui::TextEdit::singleline(&mut self.edit_obj_type).desired_width(220.0));
                });

                if crate::icons::icon_button(ui, crate::icons::AppIcon::SaveDisk, "Apply Attribute Changes", 10.5, Color32::WHITE, theme.accent_blue, Stroke::NONE, CornerRadius::same(5), Vec2::ZERO).clicked() {
                    action = Some(InspectorAction::UpdateAttributes {
                        id: el.step_id,
                        name: self.edit_name.clone(),
                        desc: self.edit_desc.clone(),
                        obj_type: self.edit_obj_type.clone(),
                    });
                }

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);

                // 2. Material Reassignment
                crate::icons::icon_label(ui, crate::icons::AppIcon::Layers, 12.0, theme.accent_orange, "MATERIAL REASSIGNMENT", 11.5, theme.accent_orange, true);
                ui.horizontal_wrapped(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.edit_material).hint_text("e.g. Steel S460ML / C30/37").desired_width(200.0));
                    if ui.add(egui::Button::new(RichText::new("Assign Material").size(10.5).strong().color(Color32::WHITE))
                        .fill(theme.accent_orange)
                        .corner_radius(CornerRadius::same(5))).clicked() {
                        action = Some(InspectorAction::SetMaterial {
                            id: el.step_id,
                            mat: self.edit_material.clone(),
                        });
                    }
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);

                // 3. Storey Reassignment
                crate::icons::icon_label(ui, crate::icons::AppIcon::Building, 12.0, theme.accent_green, "SPATIAL STOREY REASSIGNMENT", 11.5, theme.accent_green, true);
                ui.horizontal_wrapped(|ui| {
                    for (sid, sname) in &storeys {
                        let is_cur = *sname == el.storey_name;
                        let bg = if is_cur { theme.accent_green } else { theme.bg_canvas };
                        let fg = if is_cur { Color32::WHITE } else { theme.text_primary };
                        if ui.add(egui::Button::new(RichText::new(sname).size(10.0).color(fg))
                            .fill(bg)
                            .corner_radius(CornerRadius::same(4))).clicked() && !is_cur {
                            action = Some(InspectorAction::ReassignStorey {
                                id: el.step_id,
                                target_storey_id: *sid,
                            });
                        }
                    }
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);

                // 4. Property Sets Mutation
                crate::icons::icon_label(ui, crate::icons::AppIcon::Document, 12.0, theme.accent_blue, "PROPERTY SETS MUTATION", 11.5, theme.accent_blue, true);
                for pset in &el.property_sets {
                    Frame::new()
                        .fill(theme.bg_canvas)
                        .corner_radius(CornerRadius::same(6))
                        .inner_margin(Margin::same(8))
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.label(RichText::new(&pset.name).size(11.0).strong().color(theme.text_primary));
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    if crate::icons::icon_button(ui, crate::icons::AppIcon::Trash, "Delete Pset", 9.5, Color32::from_rgb(239, 68, 68), theme.bg_card, Stroke::NONE, CornerRadius::same(4), Vec2::ZERO).clicked() {
                                        action = Some(InspectorAction::DeletePset {
                                            id: el.step_id,
                                            pset: pset.name.clone(),
                                        });
                                    }
                                });
                            });
                            ui.add_space(4.0);

                            for (p_name, p_val) in &pset.properties {
                                ui.horizontal_wrapped(|ui| {
                                    ui.label(RichText::new(format!("• {}:", p_name)).size(10.5).color(theme.text_secondary));
                                    ui.label(RichText::new(p_val).size(10.5).strong().color(theme.accent_blue));
                                    if crate::icons::icon_only_button(ui, crate::icons::AppIcon::Close, theme.text_muted, theme.bg_canvas, Stroke::NONE, CornerRadius::same(3), Vec2::splat(18.0), "Remove property").clicked() {
                                        action = Some(InspectorAction::RemoveProperty {
                                            id: el.step_id,
                                            pset: pset.name.clone(),
                                            prop: p_name.clone(),
                                        });
                                    }
                                });
                            }
                        });
                    ui.add_space(4.0);
                }

                // Add Property to Pset Row
                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.new_prop_name).hint_text("Property Name").desired_width(110.0));
                    ui.add(egui::TextEdit::singleline(&mut self.new_prop_val).hint_text("Value").desired_width(110.0));
                    let target_pset = el.property_sets.first().map(|p| p.name.clone()).unwrap_or_else(|| "Pset_CustomAudit".to_string());
                    if crate::icons::icon_button(ui, crate::icons::AppIcon::Plus, "Set Property", 10.0, Color32::WHITE, theme.accent_blue, Stroke::NONE, CornerRadius::same(4), Vec2::ZERO).clicked() {
                        if !self.new_prop_name.trim().is_empty() {
                            action = Some(InspectorAction::SetProperty {
                                id: el.step_id,
                                pset: target_pset,
                                prop: self.new_prop_name.trim().to_string(),
                                val: self.new_prop_val.trim().to_string(),
                            });
                            self.new_prop_name.clear();
                            self.new_prop_val.clear();
                        }
                    }
                });

                // Add New Pset Form
                ui.add_space(6.0);
                ui.horizontal_wrapped(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.new_pset_name).hint_text("New Pset Name").desired_width(160.0));
                    if crate::icons::icon_button(ui, crate::icons::AppIcon::Plus, "Create Pset", 10.0, Color32::WHITE, theme.accent_green, Stroke::NONE, CornerRadius::same(4), Vec2::ZERO).clicked() {
                        if !self.new_pset_name.trim().is_empty() {
                            action = Some(InspectorAction::AddPset {
                                id: el.step_id,
                                pset: self.new_pset_name.trim().to_string(),
                                props: vec![("CreatedBy".to_string(), "EigenGraph_v0.1.8".to_string())],
                            });
                        }
                    }
                });

                // Export mutated STEP
                ui.add_space(10.0);
                if crate::icons::icon_button(ui, crate::icons::AppIcon::SaveDisk, "Export & Serialize Mutated STEP Physical File", 11.0, Color32::WHITE, Color32::from_rgb(139, 92, 246), Stroke::NONE, CornerRadius::same(5), Vec2::ZERO).clicked() {
                    action = Some(InspectorAction::ExportMutatedStep);
                }
            }
        } else {
            ui.label(RichText::new("Select an element in the Spatial Tree on the left to inspect properties.").size(11.0).color(theme.text_muted));
        }

        // Execute Dispatched Action
        if let Some(act) = action {
            match act {
                InspectorAction::ToggleEdit => {
                    self.is_editing = !self.is_editing;
                    if self.is_editing {
                        if let Some(ref m) = self.ifc_model {
                            if let Some(el) = m.elements.iter().find(|e| Some(e.step_id) == self.selected_element_id) {
                                self.edit_name = el.name.clone();
                                self.edit_desc = el.description.clone();
                                self.edit_obj_type = el.object_type.clone();
                                self.edit_material = el.material_name.clone().unwrap_or_default();
                            }
                        }
                    }
                }
                InspectorAction::Undo => {
                    if let Some(ref mut m) = self.ifc_model {
                        match self.edit_session.undo(m) {
                            Ok(Some(msg)) => {
                                self.export_status = Some(format!("Reverted: {}", msg));
                            }
                            Ok(None) => {}
                            Err(e) => {
                                self.export_status = Some(format!("Undo Error: {}", e));
                            }
                        }
                    }
                }
                InspectorAction::Redo => {
                    if let Some(ref mut m) = self.ifc_model {
                        match self.edit_session.redo(m) {
                            Ok(Some(msg)) => {
                                self.export_status = Some(format!("Reapplied: {}", msg));
                            }
                            Ok(None) => {}
                            Err(e) => {
                                self.export_status = Some(format!("Redo Error: {}", e));
                            }
                        }
                    }
                }
                InspectorAction::UpdateAttributes { id, name, desc, obj_type } => {
                    if let Some(ref mut m) = self.ifc_model {
                        if let Some(el) = m.elements.iter().find(|e| e.step_id == id) {
                            let cmd = IfcEditCommand::UpdateElementAttributes {
                                element_id: id,
                                old_name: el.name.clone(),
                                new_name: name,
                                old_description: el.description.clone(),
                                new_description: desc,
                                old_object_type: el.object_type.clone(),
                                new_object_type: obj_type,
                            };
                            if let Err(e) = self.edit_session.execute(m, cmd) {
                                self.export_status = Some(format!("Mutation Error: {}", e));
                            } else {
                                self.export_status = Some(format!("Updated identity attributes for element #{}.", id));
                            }
                        }
                    }
                }
                InspectorAction::SetMaterial { id, mat } => {
                    if let Some(ref mut m) = self.ifc_model {
                        if let Some(el) = m.elements.iter().find(|e| e.step_id == id) {
                            let cmd = IfcEditCommand::SetMaterial {
                                element_id: id,
                                old_material: el.material_name.clone(),
                                new_material: mat.clone(),
                            };
                            if let Err(e) = self.edit_session.execute(m, cmd) {
                                self.export_status = Some(format!("Material Error: {}", e));
                            } else {
                                self.export_status = Some(format!("Set element #{} material to '{}'.", id, mat));
                            }
                        }
                    }
                }
                InspectorAction::ReassignStorey { id, target_storey_id } => {
                    if let Some(ref mut m) = self.ifc_model {
                        let storeys = m.list_storeys();
                        let current_sid = storeys.iter().find(|(_, name)| {
                            m.elements.iter().find(|e| e.step_id == id).map(|e| e.storey_name.as_str()) == Some(name)
                        }).map(|(sid, _)| *sid).unwrap_or(0);

                        let cmd = IfcEditCommand::ReassignStorey {
                            element_id: id,
                            old_storey_id: current_sid,
                            new_storey_id: target_storey_id,
                        };
                        if let Err(e) = self.edit_session.execute(m, cmd) {
                            self.export_status = Some(format!("Storey Reassignment Error: {}", e));
                        } else {
                            self.export_status = Some(format!("Reassigned element #{} to Storey #{}.", id, target_storey_id));
                        }
                    }
                }
                InspectorAction::SetProperty { id, pset, prop, val } => {
                    if let Some(ref mut m) = self.ifc_model {
                        let old_val = m.elements.iter().find(|e| e.step_id == id).and_then(|e| {
                            e.property_sets.iter().find(|p| p.name == pset).and_then(|p| {
                                p.properties.iter().find(|(k, _)| k == &prop).map(|(_, v)| v.clone())
                            })
                        });
                        let cmd = IfcEditCommand::SetProperty {
                            element_id: id,
                            pset_name: pset.clone(),
                            prop_name: prop.clone(),
                            old_value: old_val,
                            new_value: val.clone(),
                        };
                        if let Err(e) = self.edit_session.execute(m, cmd) {
                            self.export_status = Some(format!("Property Mutation Error: {}", e));
                        } else {
                            self.export_status = Some(format!("Set #{}.{}.{} = '{}'.", id, pset, prop, val));
                        }
                    }
                }
                InspectorAction::RemoveProperty { id, pset, prop } => {
                    if let Some(ref mut m) = self.ifc_model {
                        let old_val = m.elements.iter().find(|e| e.step_id == id).and_then(|e| {
                            e.property_sets.iter().find(|p| p.name == pset).and_then(|p| {
                                p.properties.iter().find(|(k, _)| k == &prop).map(|(_, v)| v.clone())
                            })
                        }).unwrap_or_default();
                        let cmd = IfcEditCommand::RemoveProperty {
                            element_id: id,
                            pset_name: pset.clone(),
                            prop_name: prop.clone(),
                            removed_value: old_val,
                        };
                        if let Err(e) = self.edit_session.execute(m, cmd) {
                            self.export_status = Some(format!("Property Removal Error: {}", e));
                        } else {
                            self.export_status = Some(format!("Removed #{}.{}.{}.", id, pset, prop));
                        }
                    }
                }
                InspectorAction::AddPset { id, pset, props } => {
                    if let Some(ref mut m) = self.ifc_model {
                        let cmd = IfcEditCommand::AddPropertySet {
                            element_id: id,
                            pset_name: pset.clone(),
                            properties: props,
                        };
                        if let Err(e) = self.edit_session.execute(m, cmd) {
                            self.export_status = Some(format!("Pset Add Error: {}", e));
                        } else {
                            self.export_status = Some(format!("Added Property Set '{}' to #{}.", pset, id));
                        }
                    }
                }
                InspectorAction::DeletePset { id, pset } => {
                    if let Some(ref mut m) = self.ifc_model {
                        let old_props = m.elements.iter().find(|e| e.step_id == id).and_then(|e| {
                            e.property_sets.iter().find(|p| p.name == pset).map(|p| p.properties.clone())
                        }).unwrap_or_default();
                        let cmd = IfcEditCommand::DeletePropertySet {
                            element_id: id,
                            pset_name: pset.clone(),
                            deleted_properties: old_props,
                        };
                        if let Err(e) = self.edit_session.execute(m, cmd) {
                            self.export_status = Some(format!("Pset Delete Error: {}", e));
                        } else {
                            self.export_status = Some(format!("Deleted Property Set '{}' from #{}.", pset, id));
                        }
                    }
                }
                InspectorAction::ExportMutatedStep => {
                    if let Some(ref m) = self.ifc_model {
                        let step_content = m.to_step_string();
                        self.export_status = Some(format!("Serialized mutated model: {} bytes, {} entities.", step_content.len(), m.raw_step.entities.len()));
                    }
                }
            }
        }
    }

    pub(super) fn render_health_audit_tab(&self, ui: &mut Ui, theme: &MacTheme, verdict: &str) {
        if let Some(ref m) = self.ifc_model {
            let audit = &m.health_report;

            // Verdict Header
            Frame::new()
                .fill(theme.bg_canvas)
                .corner_radius(CornerRadius::same(8))
                .inner_margin(Margin::same(10))
                .stroke(Stroke::new(1.0, theme.border_subtle))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("VERDICT:").size(11.0).strong().color(theme.text_muted));
                        let v_col = if audit.conformance_score_pct >= 90.0 { theme.accent_green } else { theme.accent_orange };
                        ui.label(RichText::new(verdict).size(11.5).strong().color(v_col));
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(RichText::new(format!("Score: {:.1}%", audit.conformance_score_pct)).size(12.0).strong().color(v_col));
                        });
                    });
                });

            ui.add_space(8.0);

            // Audit Check Table
            ui.label(RichText::new("QUALITY ASSURANCE AUDIT CHECKS").size(11.5).strong().color(theme.text_secondary));
            ui.add_space(4.0);

            for check in &audit.checks {
                Frame::new()
                    .fill(theme.bg_canvas)
                    .corner_radius(CornerRadius::same(6))
                    .inner_margin(Margin::symmetric(10, 6))
                    .stroke(Stroke::new(1.0, theme.border_subtle))
                    .show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            let (badge_bg, badge_txt) = match check.status {
                                IfcAuditStatus::Pass => (Color32::from_rgb(16, 185, 129), "PASS"),
                                IfcAuditStatus::Warning => (Color32::from_rgb(245, 158, 11), "WARN"),
                                IfcAuditStatus::Fail => (Color32::from_rgb(239, 68, 68), "FAIL"),
                            };

                            Frame::new().fill(badge_bg).corner_radius(CornerRadius::same(4)).inner_margin(Margin::symmetric(6, 2)).show(ui, |ui| {
                                ui.label(RichText::new(badge_txt).size(9.5).strong().color(Color32::WHITE));
                            });

                            ui.label(RichText::new(&check.check_name).size(11.0).strong().color(theme.text_primary));
                            ui.label(RichText::new(format!("({})", check.category)).size(10.0).color(theme.text_muted));
                        });
                        ui.add_space(2.0);
                        ui.label(RichText::new(&check.detail).size(10.5).color(theme.text_secondary));
                    });
                ui.add_space(4.0);
            }
        } else {
            ui.label(RichText::new("No IFC Model loaded to audit.").size(11.0).color(theme.text_muted));
        }
    }

    pub(super) fn render_geometry_bbox_tab(&mut self, ui: &mut Ui, theme: &MacTheme) {
        enum GeomAction {
            Undo,
            Redo,
            ToggleSynthModal,
            ModifyExtrusion { element_id: usize, old_depth: f64, new_depth: f64 },
            Translate { element_id: usize, translation: [f64; 3] },
            ModifyProfile { element_id: usize, old_dims: [f64; 2], new_dims: [f64; 2] },
            DeleteElement { element_id: usize },
            SynthesizeElement { spec: IfcElementCreationSpec },
        }

        let mut action: Option<GeomAction> = None;

        // 1. Transaction & Topology Action Bar
        ui.horizontal_wrapped(|ui| {
            let can_undo = self.edit_session.can_undo();
            let undo_lbl = if let Some(desc) = self.edit_session.undo_description() {
                format!("Undo ({})", desc)
            } else {
                "Undo".to_string()
            };
            ui.add_enabled_ui(can_undo, |ui| {
                if crate::icons::icon_button(
                    ui,
                    crate::icons::AppIcon::Undo,
                    undo_lbl,
                    10.5,
                    if can_undo { theme.text_primary } else { theme.text_muted },
                    theme.bg_canvas,
                    Stroke::NONE,
                    CornerRadius::same(6),
                    Vec2::ZERO,
                ).clicked() {
                    action = Some(GeomAction::Undo);
                }
            });

            let can_redo = self.edit_session.can_redo();
            let redo_lbl = if let Some(desc) = self.edit_session.redo_description() {
                format!("Redo ({})", desc)
            } else {
                "Redo".to_string()
            };
            ui.add_enabled_ui(can_redo, |ui| {
                if crate::icons::icon_button(
                    ui,
                    crate::icons::AppIcon::Redo,
                    redo_lbl,
                    10.5,
                    if can_redo { theme.text_primary } else { theme.text_muted },
                    theme.bg_canvas,
                    Stroke::NONE,
                    CornerRadius::same(6),
                    Vec2::ZERO,
                ).clicked() {
                    action = Some(GeomAction::Redo);
                }
            });

            let synth_btn_bg = if self.show_synth_modal { theme.accent_green } else { theme.bg_canvas };
            let synth_btn_fg = if self.show_synth_modal { Color32::WHITE } else { theme.text_primary };
            if crate::icons::icon_button(
                ui,
                crate::icons::AppIcon::Plus,
                "Synthesize Element",
                10.5,
                synth_btn_fg,
                synth_btn_bg,
                Stroke::NONE,
                CornerRadius::same(6),
                Vec2::ZERO,
            ).clicked() {
                action = Some(GeomAction::ToggleSynthModal);
            }

            if self.edit_session.is_dirty {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.horizontal(|ui| {
                        crate::icons::icon_label(ui, crate::icons::AppIcon::Info, 11.0, theme.accent_orange, "MODIFIED", 10.0, theme.accent_orange, true);
                    });
                });
            }
        });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(6.0);

        let storeys = if let Some(ref m) = self.ifc_model {
            m.list_storeys()
        } else {
            Vec::new()
        };

        // 2. Element Synthesis Panel (If active)
        if self.show_synth_modal {
            Frame::new()
                .fill(theme.bg_canvas)
                .corner_radius(CornerRadius::same(8))
                .inner_margin(Margin::same(12))
                .stroke(Stroke::new(1.0, theme.accent_green))
                .show(ui, |ui| {
                    crate::icons::icon_label(ui, crate::icons::AppIcon::Sparkles, 12.0, theme.accent_green, "SYNTHESIZE STRUCTURAL ELEMENT (ISO 10303-21 / ISO 16739)", 11.5, theme.accent_green, true);
                    ui.add_space(6.0);

                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("Entity Class:").size(11.0).color(theme.text_secondary));
                        for class_candidate in &["IFCCOLUMN", "IFCBEAM", "IFCMEMBER"] {
                            let is_cur = self.synth_class == *class_candidate;
                            let bg = if is_cur { theme.accent_green } else { theme.bg_card };
                            let fg = if is_cur { Color32::WHITE } else { theme.text_primary };
                            if ui.add(egui::Button::new(RichText::new(*class_candidate).size(10.0).color(fg))
                                .fill(bg)
                                .corner_radius(CornerRadius::same(4))).clicked() {
                                self.synth_class = class_candidate.to_string();
                            }
                        }
                    });

                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("Name:").size(11.0).color(theme.text_secondary));
                        ui.add(egui::TextEdit::singleline(&mut self.synth_name).desired_width(180.0));
                        ui.label(RichText::new("Material:").size(11.0).color(theme.text_secondary));
                        ui.add(egui::TextEdit::singleline(&mut self.synth_mat).desired_width(160.0));
                    });

                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("Description:").size(11.0).color(theme.text_secondary));
                        let desc_w = ui.available_width().min(380.0).max(160.0);
                        ui.add(egui::TextEdit::singleline(&mut self.synth_desc).desired_width(desc_w));
                    });

                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("Length/Extrusion (m):").size(11.0).color(theme.text_secondary));
                        ui.add(egui::DragValue::new(&mut self.synth_length).speed(0.1).range(0.1..=100.0));
                        ui.label(RichText::new("Profile W x H (m):").size(11.0).color(theme.text_secondary));
                        ui.add(egui::DragValue::new(&mut self.synth_prof_w).speed(0.05).range(0.05..=10.0));
                        ui.label(RichText::new("x").size(11.0).color(theme.text_muted));
                        ui.add(egui::DragValue::new(&mut self.synth_prof_h).speed(0.05).range(0.05..=10.0));
                    });

                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("Target Storey:").size(11.0).color(theme.text_secondary));
                        if storeys.is_empty() {
                            ui.label(RichText::new("No storey available").size(10.5).color(theme.text_muted));
                        } else {
                            if self.selected_target_storey == 0 {
                                self.selected_target_storey = storeys[0].0;
                            }
                            for (sid, sname) in &storeys {
                                let is_cur = self.selected_target_storey == *sid;
                                let bg = if is_cur { theme.accent_blue } else { theme.bg_card };
                                let fg = if is_cur { Color32::WHITE } else { theme.text_primary };
                                if ui.add(egui::Button::new(RichText::new(sname).size(10.0).color(fg))
                                    .fill(bg)
                                    .corner_radius(CornerRadius::same(4))).clicked() {
                                    self.selected_target_storey = *sid;
                                }
                            }
                        }
                    });

                    ui.add_space(4.0);
                    if crate::icons::icon_button(ui, crate::icons::AppIcon::Play, "Synthesize Structural Entity into STEP Graph", 11.0, Color32::WHITE, theme.accent_green, Stroke::NONE, CornerRadius::same(6), Vec2::ZERO).clicked() {
                        let target_sid = if self.selected_target_storey != 0 {
                            self.selected_target_storey
                        } else {
                            storeys.first().map(|(sid, _)| *sid).unwrap_or(0)
                        };
                        let dir = if self.synth_class == "IFCBEAM" {
                            (1.0, 0.0, 0.0)
                        } else {
                            (0.0, 0.0, 1.0)
                        };
                        let spec = IfcElementCreationSpec {
                            class_name: self.synth_class.clone(),
                            name: self.synth_name.clone(),
                            description: self.synth_desc.clone(),
                            storey_id: target_sid,
                            material_name: self.synth_mat.clone(),
                            origin: (0.0, 0.0, 0.0),
                            direction: dir,
                            profile_dims: (self.synth_prof_w as f64, self.synth_prof_h as f64),
                            length: self.synth_length as f64,
                        };
                        action = Some(GeomAction::SynthesizeElement { spec });
                    }
                });
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(8.0);
        }

        // 3. Selected Element Geometry & Swept Solid Inspector
        let el_opt = if let Some(ref m) = self.ifc_model {
            m.elements.iter().find(|e| Some(e.step_id) == self.selected_element_id).cloned()
        } else {
            None
        };

        if let Some(el) = el_opt {
            let (dx, dy, dz) = el.bbox.dimensions();
            let (cx, cy, cz) = el.centroid;

            crate::icons::icon_label(ui, crate::icons::AppIcon::RulerSquare, 12.0, theme.accent_blue, format!("3D BOUNDING BOX — {} (#{}: {})", el.name, el.step_id, el.class_name), 12.0, theme.accent_blue, true);
            ui.add_space(8.0);

            let bbox_cards = [
                ("LENGTH (X)", format!("{:.3} m", dx), "Span Dimension"),
                ("WIDTH (Y)", format!("{:.3} m", dy), "Cross-Section Breadth"),
                ("HEIGHT (Z)", format!("{:.3} m", dz), "Elevation Depth"),
            ];
            ui_kit::responsive_grid_n(ui, bbox_cards.len(), 170.0, |ui, idx| {
                let (title, val, sub) = &bbox_cards[idx];
                self.render_stat_card(ui, theme, title, val, sub, theme.text_primary);
            });

            ui.add_space(10.0);

            Frame::new()
                .fill(theme.bg_canvas)
                .corner_radius(CornerRadius::same(8))
                .inner_margin(Margin::same(12))
                .stroke(Stroke::new(1.0, theme.border_subtle))
                .show(ui, |ui| {
                    ui.label(RichText::new("SPATIAL EXTENTS (WORLD COORDINATES)").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(4.0);
                    ui.label(RichText::new(format!("• Centroid (Xc, Yc, Zc): ({:.3}, {:.3}, {:.3}) m", cx, cy, cz)).size(10.5).color(theme.text_primary));
                    ui.label(RichText::new(format!("• X-Range: [{:.3}, {:.3}] m", el.bbox.min_x, el.bbox.max_x)).size(10.5).color(theme.text_secondary));
                    ui.label(RichText::new(format!("• Y-Range: [{:.3}, {:.3}] m", el.bbox.min_y, el.bbox.max_y)).size(10.5).color(theme.text_secondary));
                    ui.label(RichText::new(format!("• Z-Range: [{:.3}, {:.3}] m", el.bbox.min_z, el.bbox.max_z)).size(10.5).color(theme.text_secondary));
                    ui.label(RichText::new(format!("• Enclosing Bounding Volume: {:.4} m³", el.bbox.volume())).size(10.5).strong().color(theme.accent_green));
                    ui.label(RichText::new(format!("• Parsed Wireframe Segments: {} lines", el.wireframe_lines.len())).size(10.5).color(theme.accent_blue));
                });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(8.0);

            // 4. Interactive Geometric Topology & Extrusion Mutations
            crate::icons::icon_label(ui, crate::icons::AppIcon::Wrench, 12.0, theme.accent_orange, "GEOMETRIC TOPOLOGY & SWEPT SOLID MUTATION", 11.5, theme.accent_orange, true);
            ui.add_space(6.0);

            // Section A: Extrusion Depth
            Frame::new()
                .fill(theme.bg_canvas)
                .corner_radius(CornerRadius::same(8))
                .inner_margin(Margin::same(10))
                .stroke(Stroke::new(1.0, theme.border_subtle))
                .show(ui, |ui| {
                    crate::icons::icon_label(ui, crate::icons::AppIcon::DimensionRuler, 11.5, theme.accent_blue, "Swept Solid Extrusion Depth (IfcExtrudedAreaSolid.Depth)", 11.0, theme.text_primary, true);
                    ui.add_space(4.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("New Depth (m):").size(10.5).color(theme.text_secondary));
                        ui.add(egui::DragValue::new(&mut self.geom_depth_input).speed(0.1).range(0.1..=100.0));
                        let orig_depth = el.quantities.get("Length").copied().unwrap_or_else(|| {
                            let (x, y, z) = el.bbox.dimensions();
                            x.max(y).max(z)
                        });
                        if ui.add(egui::Button::new(RichText::new("Apply Extrusion Depth").size(10.5).strong().color(Color32::WHITE))
                            .fill(theme.accent_blue)
                            .corner_radius(CornerRadius::same(4))).clicked() {
                            action = Some(GeomAction::ModifyExtrusion {
                                element_id: el.step_id,
                                old_depth: orig_depth,
                                new_depth: self.geom_depth_input as f64,
                            });
                        }
                    });
                });

            ui.add_space(6.0);

            // Section B: 3D Spatial Translation
            Frame::new()
                .fill(theme.bg_canvas)
                .corner_radius(CornerRadius::same(8))
                .inner_margin(Margin::same(10))
                .stroke(Stroke::new(1.0, theme.border_subtle))
                .show(ui, |ui| {
                    crate::icons::icon_label(ui, crate::icons::AppIcon::Compass, 11.5, theme.accent_blue, "3D Spatial Translation (IfcCartesianPoint [dX, dY, dZ])", 11.0, theme.text_primary, true);
                    ui.add_space(4.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("dX:").size(10.5).color(theme.text_secondary));
                        ui.add(egui::DragValue::new(&mut self.geom_trans_x).speed(0.1));
                        ui.label(RichText::new("dY:").size(10.5).color(theme.text_secondary));
                        ui.add(egui::DragValue::new(&mut self.geom_trans_y).speed(0.1));
                        ui.label(RichText::new("dZ:").size(10.5).color(theme.text_secondary));
                        ui.add(egui::DragValue::new(&mut self.geom_trans_z).speed(0.1));

                        if ui.add(egui::Button::new(RichText::new("Translate Placement").size(10.5).strong().color(Color32::WHITE))
                            .fill(theme.accent_blue)
                            .corner_radius(CornerRadius::same(4))).clicked() {
                            action = Some(GeomAction::Translate {
                                element_id: el.step_id,
                                translation: [self.geom_trans_x as f64, self.geom_trans_y as f64, self.geom_trans_z as f64],
                            });
                        }
                    });
                });

            ui.add_space(6.0);

            // Section C: Profile Cross-Section Dimensions
            Frame::new()
                .fill(theme.bg_canvas)
                .corner_radius(CornerRadius::same(8))
                .inner_margin(Margin::same(10))
                .stroke(Stroke::new(1.0, theme.border_subtle))
                .show(ui, |ui| {
                    crate::icons::icon_label(ui, crate::icons::AppIcon::RulerSquare, 11.5, theme.accent_blue, "Parametric Profile Dimensions (IfcRectangleProfileDef [XDim, YDim])", 11.0, theme.text_primary, true);
                    ui.add_space(4.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("Width X (m):").size(10.5).color(theme.text_secondary));
                        ui.add(egui::DragValue::new(&mut self.geom_prof_w).speed(0.05).range(0.05..=10.0));
                        ui.label(RichText::new("Depth Y (m):").size(10.5).color(theme.text_secondary));
                        ui.add(egui::DragValue::new(&mut self.geom_prof_h).speed(0.05).range(0.05..=10.0));

                        let (x, y, _) = el.bbox.dimensions();
                        if ui.add(egui::Button::new(RichText::new("Resize Profile").size(10.5).strong().color(Color32::WHITE))
                            .fill(theme.accent_blue)
                            .corner_radius(CornerRadius::same(4))).clicked() {
                            action = Some(GeomAction::ModifyProfile {
                                element_id: el.step_id,
                                old_dims: [x, y],
                                new_dims: [self.geom_prof_w as f64, self.geom_prof_h as f64],
                            });
                        }
                    });
                });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(6.0);

            // Section D: Safe Element Deletion
            ui.horizontal_wrapped(|ui| {
                if crate::icons::icon_button(ui, crate::icons::AppIcon::Trash, format!("Delete Structural Element #{}", el.step_id), 11.0, Color32::WHITE, Color32::from_rgb(239, 68, 68), Stroke::NONE, CornerRadius::same(5), Vec2::ZERO).clicked() {
                    action = Some(GeomAction::DeleteElement { element_id: el.step_id });
                }
            });

        } else {
            ui.label(RichText::new("Select an element from the spatial hierarchy to inspect and mutate geometry, or synthesize a new element above.").size(11.0).color(theme.text_muted));
        }

        // 5. Execute Dispatched Actions
        if let Some(act) = action {
            match act {
                GeomAction::Undo => {
                    if let Some(ref mut m) = self.ifc_model {
                        if let Err(e) = self.edit_session.undo(m) {
                            self.export_status = Some(format!("Undo Error: {}", e));
                        } else {
                            self.export_status = Some("Undid last geometric command.".to_string());
                        }
                    }
                }
                GeomAction::Redo => {
                    if let Some(ref mut m) = self.ifc_model {
                        if let Err(e) = self.edit_session.redo(m) {
                            self.export_status = Some(format!("Redo Error: {}", e));
                        } else {
                            self.export_status = Some("Redid geometric command.".to_string());
                        }
                    }
                }
                GeomAction::ToggleSynthModal => {
                    self.show_synth_modal = !self.show_synth_modal;
                }
                GeomAction::ModifyExtrusion { element_id, old_depth, new_depth } => {
                    if let Some(ref mut m) = self.ifc_model {
                        let cmd = IfcEditCommand::ModifyExtrusionDepth {
                            element_id,
                            old_depth,
                            new_depth,
                        };
                        if let Err(e) = self.edit_session.execute(m, cmd) {
                            self.export_status = Some(format!("Extrusion Error: {}", e));
                        } else {
                            self.export_status = Some(format!("Modified extrusion depth of #{} to {:.3} m.", element_id, new_depth));
                        }
                    }
                }
                GeomAction::Translate { element_id, translation } => {
                    if let Some(ref mut m) = self.ifc_model {
                        let cmd = IfcEditCommand::TranslateElement {
                            element_id,
                            delta: (translation[0], translation[1], translation[2]),
                        };
                        if let Err(e) = self.edit_session.execute(m, cmd) {
                            self.export_status = Some(format!("Translation Error: {}", e));
                        } else {
                            self.geom_trans_x = 0.0;
                            self.geom_trans_y = 0.0;
                            self.geom_trans_z = 0.0;
                            self.export_status = Some(format!("Translated placement of #{} by [{:.2}, {:.2}, {:.2}] m.", element_id, translation[0], translation[1], translation[2]));
                        }
                    }
                }
                GeomAction::ModifyProfile { element_id, old_dims, new_dims } => {
                    if let Some(ref mut m) = self.ifc_model {
                        let cmd = IfcEditCommand::ModifyProfileDimensions {
                            element_id,
                            old_dims: (old_dims[0], old_dims[1]),
                            new_dims: (new_dims[0], new_dims[1]),
                        };
                        if let Err(e) = self.edit_session.execute(m, cmd) {
                            self.export_status = Some(format!("Profile Dimension Error: {}", e));
                        } else {
                            self.export_status = Some(format!("Modified profile of #{} to {:.3} x {:.3} m.", element_id, new_dims[0], new_dims[1]));
                        }
                    }
                }
                GeomAction::DeleteElement { element_id } => {
                    if let Some(ref mut m) = self.ifc_model {
                        let cmd = IfcEditCommand::DeleteElementById { element_id };
                        if let Err(e) = self.edit_session.execute(m, cmd) {
                            self.export_status = Some(format!("Element Deletion Error: {}", e));
                        } else {
                            if self.selected_element_id == Some(element_id) {
                                self.selected_element_id = None;
                            }
                            self.export_status = Some(format!("Safely deleted structural element #{} from model.", element_id));
                        }
                    }
                }
                GeomAction::SynthesizeElement { spec } => {
                    if let Some(ref mut m) = self.ifc_model {
                        let elem_name = spec.name.clone();
                        let cmd = IfcEditCommand::CreateElement { element_id: 0, spec };
                        if let Err(e) = self.edit_session.execute(m, cmd) {
                            self.export_status = Some(format!("Element Synthesis Error: {}", e));
                        } else {
                            self.selected_element_id = m.elements.last().map(|e| e.step_id);
                            self.show_synth_modal = false;
                            self.export_status = Some(format!("Synthesized new element '{}' into IFC model graph.", elem_name));
                        }
                    }
                }
            }
        }
    }

    pub(super) fn render_clash_and_diff_tab(&mut self, ui: &mut Ui, theme: &MacTheme) {
        let mut run_clash = false;
        let mut run_diff = false;
        let mut run_ids = false;
        let mut export_bcf = false;
        let mut export_json = false;
        let mut select_id: Option<usize> = None;

        // 1. Sub-Tab Selector for Clash, Diff, and IDS
        ui.horizontal_wrapped(|ui| {
            let tabs = [
                (0, crate::icons::AppIcon::AlertTriangle, "Spatial Clash Matrix"),
                (1, crate::icons::AppIcon::Layers, "BIM Differential (Model Diff)"),
                (2, crate::icons::AppIcon::Shield, "buildingSMART IDS & BCF"),
            ];

            for (idx, icon, label) in tabs {
                let is_active = self.diff_subtab == idx;
                let bg = if is_active { theme.accent_blue } else { theme.bg_canvas };
                let fg = if is_active { Color32::WHITE } else { theme.text_secondary };

                if crate::icons::icon_button(ui, icon, label, 10.5, fg, bg, Stroke::NONE, CornerRadius::same(6), Vec2::ZERO).clicked() {
                    self.diff_subtab = idx;
                }
            }
        });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(6.0);

        // 2. Render Subtab Content
        match self.diff_subtab {
            0 => {
                // SPATIAL CLASH MATRIX
                crate::icons::icon_label(ui, crate::icons::AppIcon::AlertTriangle, 12.0, theme.accent_blue, "3D SPATIAL CLASH DETECTION & CLEARANCE GUARD", 12.0, theme.accent_blue, true);
                ui.add_space(4.0);
                ui.label(RichText::new("Automated Axis-Aligned Bounding Box (AABB) volumetric intersection and proximity clearance evaluation.").size(10.5).color(theme.text_secondary));
                ui.add_space(8.0);

                Frame::new()
                    .fill(theme.bg_canvas)
                    .corner_radius(CornerRadius::same(8))
                    .inner_margin(Margin::same(10))
                    .stroke(Stroke::new(1.0, theme.border_subtle))
                    .show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.label(RichText::new("Clash Tolerance (mm):").size(10.5).color(theme.text_secondary));
                            ui.add(egui::DragValue::new(&mut self.clash_tolerance_mm).speed(1.0).range(1.0..=100.0));

                            ui.label(RichText::new("Clearance Buffer (mm):").size(10.5).color(theme.text_secondary));
                            ui.add(egui::DragValue::new(&mut self.clearance_buffer_mm).speed(5.0).range(10.0..=500.0));

                            if crate::icons::icon_button(ui, crate::icons::AppIcon::Bolt, "Run Clash Detection", 11.0, Color32::WHITE, theme.accent_blue, Stroke::NONE, CornerRadius::same(5), Vec2::ZERO).clicked() {
                                run_clash = true;
                            }
                        });
                    });

                ui.add_space(10.0);

                if let Some(ref cr) = self.clash_report {
                    let hard_col = if cr.hard_clashes == 0 { theme.accent_green } else { Color32::from_rgb(239, 68, 68) };
                    let clear_col = if cr.clearance_violations == 0 { theme.accent_green } else { theme.accent_orange };
                    let clash_cards = [
                        ("PAIRS EVALUATED", format!("{}", cr.total_evaluated_pairs), "Element Permutations", theme.text_primary),
                        ("HARD CLASHES", format!("{}", cr.hard_clashes), "Physical Intersections", hard_col),
                        ("CLEARANCE VIOLATIONS", format!("{}", cr.clearance_violations), "Proximity Violations", clear_col),
                        ("DUPLICATE ENTITIES", format!("{}", cr.duplicate_entities), "Overlapping Copies", Color32::from_rgb(168, 85, 247)),
                    ];
                    ui_kit::responsive_grid_n(ui, clash_cards.len(), 170.0, |ui, idx| {
                        let (title, val, sub, col) = &clash_cards[idx];
                        self.render_stat_card(ui, theme, title, val, sub, *col);
                    });

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(8.0);

                    if cr.clashes.is_empty() {
                        Frame::new()
                            .fill(Color32::from_rgb(16, 185, 129).linear_multiply(0.15))
                            .corner_radius(CornerRadius::same(6))
                            .inner_margin(Margin::same(10))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(16, 185, 129)))
                            .show(ui, |ui| {
                                crate::icons::icon_label(ui, crate::icons::AppIcon::CheckCircle, 12.0, theme.accent_green, "ZERO CLASHES: All structural framing elements satisfy spatial clearances and geometric separation.", 11.0, theme.accent_green, true);
                            });
                    } else {
                        ui.label(RichText::new(format!("DETECTED SPATIAL INCIDENTS ({} items)", cr.clashes.len())).size(11.0).strong().color(theme.text_secondary));
                        ui.add_space(4.0);

                        for clash in &cr.clashes {
                            let (badge_bg, badge_lbl) = match clash.severity {
                                IfcClashSeverity::Critical => (Color32::from_rgb(239, 68, 68), "CRITICAL HARD"),
                                IfcClashSeverity::Major => (Color32::from_rgb(249, 115, 22), "MAJOR HARD"),
                                IfcClashSeverity::Minor => (Color32::from_rgb(234, 179, 8), "CLEARANCE"),
                                IfcClashSeverity::Duplicate => (Color32::from_rgb(168, 85, 247), "DUPLICATE"),
                            };

                            Frame::new()
                                .fill(theme.bg_canvas)
                                .corner_radius(CornerRadius::same(6))
                                .inner_margin(Margin::same(8))
                                .stroke(Stroke::new(1.0, theme.border_subtle))
                                .show(ui, |ui| {
                                    ui.horizontal_wrapped(|ui| {
                                        Frame::new().fill(badge_bg).corner_radius(CornerRadius::same(4)).inner_margin(Margin::symmetric(6, 2)).show(ui, |ui| {
                                            ui.label(RichText::new(badge_lbl).size(9.0).strong().color(Color32::WHITE));
                                        });

                                        ui.label(RichText::new(format!("#{} ({}) vs #{} ({})", clash.id_a, clash.name_a, clash.id_b, clash.name_b)).size(10.5).strong().color(theme.text_primary));

                                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                            if ui.add(egui::Button::new(RichText::new(format!("Select #{}", clash.id_a)).size(9.5).color(theme.accent_blue))
                                                .fill(theme.bg_card)
                                                .corner_radius(CornerRadius::same(4))).clicked() {
                                                select_id = Some(clash.id_a);
                                            }
                                        });
                                    });
                                    ui.add_space(2.0);
                                    ui.label(RichText::new(&clash.description).size(10.0).color(theme.text_secondary));
                                });
                            ui.add_space(4.0);
                        }
                    }
                } else {
                    ui.label(RichText::new("Click 'Run Clash Detection' above to evaluate 3D spatial intersections.").size(10.5).color(theme.text_muted));
                }
            }
            1 => {
                // MODEL DIFFERENTIAL (BIM DIFF)
                crate::icons::icon_label(ui, crate::icons::AppIcon::Layers, 12.0, theme.accent_blue, "BIM MODEL DIFFERENTIAL & REVISION TRACKING", 12.0, theme.accent_blue, true);
                ui.add_space(4.0);
                ui.label(RichText::new("Deterministic comparison between initial baseline model and active mutated session model.").size(10.5).color(theme.text_secondary));
                ui.add_space(8.0);

                if crate::icons::icon_button(ui, crate::icons::AppIcon::Layers, "Compute Model Differential", 11.0, Color32::WHITE, theme.accent_blue, Stroke::NONE, CornerRadius::same(6), Vec2::ZERO).clicked() {
                    run_diff = true;
                }

                ui.add_space(10.0);

                if let Some(ref diff) = self.model_diff {
                    let diff_cards = [
                        ("ADDED ELEMENTS", format!("+{}", diff.added_count), "Newly Synthesized", theme.accent_green),
                        ("DELETED ELEMENTS", format!("-{}", diff.deleted_count), "Safely Excised", Color32::from_rgb(239, 68, 68)),
                        ("MODIFIED ELEMENTS", format!("*{}", diff.modified_count), "Attributes / Geometry", theme.accent_orange),
                        ("UNCHANGED", format!("={}", diff.unchanged_count), "Identical Geometry", theme.text_secondary),
                    ];
                    ui_kit::responsive_grid_n(ui, diff_cards.len(), 170.0, |ui, idx| {
                        let (title, val, sub, col) = &diff_cards[idx];
                        self.render_stat_card(ui, theme, title, val, sub, *col);
                    });

                    ui.add_space(8.0);

                    // SHA-256 Revision Hash Seal
                    Frame::new()
                        .fill(theme.bg_canvas)
                        .corner_radius(CornerRadius::same(6))
                        .inner_margin(Margin::same(8))
                        .stroke(Stroke::new(1.0, theme.border_subtle))
                        .show(ui, |ui| {
                            crate::icons::icon_label(ui, crate::icons::AppIcon::Lock, 10.5, theme.text_muted, "CRYPTOGRAPHIC REVISION HASH SEAL (FIPS 180-4 SHA-256):", 9.5, theme.text_muted, true);
                            ui.label(RichText::new(&diff.revision_hash).size(10.0).monospace().color(theme.accent_blue));
                        });

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(8.0);

                    ui.label(RichText::new("REVISION MANIFEST ITEMS").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(4.0);

                    for d in &diff.element_diffs {
                        if d.diff_kind == IfcDiffKind::Unchanged {
                            continue;
                        }

                        let (badge_bg, badge_lbl) = match d.diff_kind {
                            IfcDiffKind::Added => (Color32::from_rgb(16, 185, 129), "+ ADDED"),
                            IfcDiffKind::Deleted => (Color32::from_rgb(239, 68, 68), "- DELETED"),
                            IfcDiffKind::Modified => (Color32::from_rgb(249, 115, 22), "* MODIFIED"),
                            IfcDiffKind::Unchanged => (theme.text_muted, "= UNCHANGED"),
                        };

                        Frame::new()
                            .fill(theme.bg_canvas)
                            .corner_radius(CornerRadius::same(6))
                            .inner_margin(Margin::same(8))
                            .stroke(Stroke::new(1.0, theme.border_subtle))
                            .show(ui, |ui| {
                                ui.horizontal_wrapped(|ui| {
                                    Frame::new().fill(badge_bg).corner_radius(CornerRadius::same(4)).inner_margin(Margin::symmetric(6, 2)).show(ui, |ui| {
                                        ui.label(RichText::new(badge_lbl).size(9.0).strong().color(Color32::WHITE));
                                    });

                                    ui.label(RichText::new(format!("#{} — {} ({})", d.step_id, d.name, d.class_name)).size(10.5).strong().color(theme.text_primary));

                                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                        if ui.add(egui::Button::new(RichText::new(format!("Select #{}", d.step_id)).size(9.5).color(theme.accent_blue))
                                            .fill(theme.bg_card)
                                            .corner_radius(CornerRadius::same(4))).clicked() {
                                            select_id = Some(d.step_id);
                                        }
                                    });
                                });
                                ui.add_space(3.0);
                                for detail in &d.details {
                                    ui.label(RichText::new(format!("• {}", detail)).size(10.0).color(theme.text_secondary));
                                }
                            });
                        ui.add_space(4.0);
                    }
                } else {
                    ui.label(RichText::new("Click 'Compute Model Differential' above to compare active changes.").size(10.5).color(theme.text_muted));
                }
            }
            2 => {
                // BUILDINGSMART IDS & BCF
                crate::icons::icon_label(ui, crate::icons::AppIcon::Shield, 12.0, theme.accent_blue, "BUILDINGSMART IDS CONFORMANCE & BCF ISSUE EXPORTER", 12.0, theme.accent_blue, true);
                ui.add_space(4.0);
                ui.label(RichText::new("Information Delivery Specification (IDS) validation rules and BIM Collaboration Format (BCF 2.1) export.").size(10.5).color(theme.text_secondary));
                ui.add_space(8.0);

                ui.horizontal_wrapped(|ui| {
                    if crate::icons::icon_button(ui, crate::icons::AppIcon::Shield, "Validate IDS Specification Rules", 11.0, Color32::WHITE, theme.accent_blue, Stroke::NONE, CornerRadius::same(6), Vec2::ZERO).clicked() {
                        run_ids = true;
                    }

                    if crate::icons::icon_button(ui, crate::icons::AppIcon::Document, "Export BCF Topics XML", 11.0, Color32::WHITE, Color32::from_rgb(14, 165, 233), Stroke::NONE, CornerRadius::same(6), Vec2::ZERO).clicked() {
                        export_bcf = true;
                    }

                    if crate::icons::icon_button(ui, crate::icons::AppIcon::Document, "Export ifcJSON-4", 11.0, Color32::WHITE, Color32::from_rgb(234, 88, 12), Stroke::NONE, CornerRadius::same(6), Vec2::ZERO).clicked() {
                        export_json = true;
                    }
                });

                ui.add_space(10.0);

                if let Some(ref ids) = self.ids_report {
                    // Header verdict
                    Frame::new()
                        .fill(theme.bg_canvas)
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(Margin::same(10))
                        .stroke(Stroke::new(1.0, theme.border_subtle))
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.label(RichText::new("IDS COMPLIANCE:").size(11.0).strong().color(theme.text_muted));
                                let v_col = if ids.compliance_score_pct >= 90.0 { theme.accent_green } else { theme.accent_orange };
                                ui.label(RichText::new(format!("{:.1}% Compliant", ids.compliance_score_pct)).size(12.0).strong().color(v_col));
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.label(RichText::new(&ids.specification_title).size(10.0).color(theme.text_muted));
                                });
                            });
                        });

                    ui.add_space(8.0);
                    ui.label(RichText::new("INFORMATION DELIVERY SPECIFICATION RULES").size(11.0).strong().color(theme.text_secondary));
                    ui.add_space(4.0);

                    for rule in &ids.rules {
                        let (badge_bg, badge_lbl) = match rule.status {
                            IfcAuditStatus::Pass => (Color32::from_rgb(16, 185, 129), "PASS"),
                            IfcAuditStatus::Warning => (Color32::from_rgb(245, 158, 11), "WARN"),
                            IfcAuditStatus::Fail => (Color32::from_rgb(239, 68, 68), "FAIL"),
                        };

                        Frame::new()
                            .fill(theme.bg_canvas)
                            .corner_radius(CornerRadius::same(6))
                            .inner_margin(Margin::same(8))
                            .stroke(Stroke::new(1.0, theme.border_subtle))
                            .show(ui, |ui| {
                                ui.horizontal_wrapped(|ui| {
                                    Frame::new().fill(badge_bg).corner_radius(CornerRadius::same(4)).inner_margin(Margin::symmetric(6, 2)).show(ui, |ui| {
                                        ui.label(RichText::new(badge_lbl).size(9.0).strong().color(Color32::WHITE));
                                    });

                                    ui.label(RichText::new(&rule.rule_id).size(10.5).strong().color(theme.text_primary));
                                    ui.label(RichText::new(format!("({})", rule.target_class)).size(9.5).color(theme.text_muted));

                                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                        ui.label(RichText::new(format!("{}/{} Pass", rule.pass_count, rule.total_count)).size(10.0).strong().color(badge_bg));
                                    });
                                });
                                ui.add_space(2.0);
                                ui.label(RichText::new(&rule.description).size(10.0).color(theme.text_secondary));
                            });
                        ui.add_space(4.0);
                    }
                } else {
                    ui.label(RichText::new("Click 'Validate IDS Specification Rules' above to audit model requirements.").size(10.5).color(theme.text_muted));
                }
            }
            _ => {}
        }

        // 3. Post-UI Actions
        if let Some(id) = select_id {
            self.selected_element_id = Some(id);
        }

        if run_clash {
            if let Some(ref m) = self.ifc_model {
                let report = m.detect_clashes(self.clash_tolerance_mm as f64 * 1e-3, self.clearance_buffer_mm as f64 * 1e-3);
                self.export_status = Some(format!("Clash Detection complete: {} hard clashes, {} clearance violations across {} pairs.", report.hard_clashes, report.clearance_violations, report.total_evaluated_pairs));
                self.clash_report = Some(report);
            }
        }

        if run_diff {
            if let (Some(ref base), Some(ref cur)) = (&self.baseline_model, &self.ifc_model) {
                let diff = base.diff(cur);
                self.export_status = Some(format!("Model Diff computed: +{} added, -{} deleted, *{} modified. Seal: {}.", diff.added_count, diff.deleted_count, diff.modified_count, &diff.revision_hash[0..8]));
                self.model_diff = Some(diff);
            }
        }

        if run_ids {
            if let Some(ref m) = self.ifc_model {
                let report = m.validate_ids();
                self.export_status = Some(format!("IDS Audit complete: {:.1}% compliance score across {} rules.", report.compliance_score_pct, report.rules.len()));
                self.ids_report = Some(report);
            }
        }

        if export_bcf {
            if let Some(ref m) = self.ifc_model {
                let empty = Vec::new();
                let clashes = self.clash_report.as_ref().map(|r| r.clashes.as_slice()).unwrap_or(&empty);
                let xml = m.export_bcf_xml(clashes);
                self.export_status = Some(format!("Exported buildingSMART BCF topics XML ({} bytes, {} issues).", xml.len(), clashes.len()));
            }
        }

        if export_json {
            if let Some(ref m) = self.ifc_model {
                let json = m.to_ifc_json();
                self.export_status = Some(format!("Exported buildingSMART ifcJSON-4 document ({} bytes).", json.len()));
            }
        }
    }
}
