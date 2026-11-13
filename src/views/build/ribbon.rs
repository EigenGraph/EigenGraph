//! views/build/ribbon.rs — Multi-Discipline 12-Tab BIM Authoring Ribbon.

use super::*;

impl BuildView {
    pub(super) fn render_bim_master_ribbon(&mut self, ui: &mut Ui, theme: &MacTheme) {
        Frame::new()
            .fill(theme.bg_sidebar)
            .inner_margin(Margin::symmetric(10, 6))
            .stroke(Stroke::new(1.0, theme.border_subtle))
            .show(ui, |ui| {
                // Tier 1: Ribbon Tab Selector Chips
                ScrollArea::horizontal()
                    .id_salt("bim_ribbon_tabs_scroll")
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let tabs = [
                                BimRibbonTab::Architecture,
                                BimRibbonTab::Structure,
                                BimRibbonTab::MepSystems,
                                BimRibbonTab::CivilInfra,
                                BimRibbonTab::AnalysisFea,
                                BimRibbonTab::Time4D,
                                BimRibbonTab::Cost5D,
                                BimRibbonTab::Sustainability6D,
                                BimRibbonTab::FacilityTwin7D,
                                BimRibbonTab::CoordinationAudit,
                                BimRibbonTab::OpenBimSpec,
                                BimRibbonTab::SheetsDed,
                            ];
                            for tab in tabs {
                                let is_active = self.active_ribbon_tab == tab;
                                let bg = if is_active { theme.accent_blue } else { theme.bg_card };
                                let fg = if is_active { Color32::WHITE } else { theme.text_secondary };

                                if ui.add(egui::Button::new(RichText::new(tab.label()).size(10.5).color(fg))
                                    .fill(bg)
                                    .corner_radius(CornerRadius::same(6))
                                    .min_size(Vec2::new(0.0, 24.0))).clicked() {
                                    self.active_ribbon_tab = tab;
                                }
                            }
                        });
                    });

                ui.add_space(4.0);
                ui.separator();
                ui.add_space(4.0);

                // Tier 2: Contextual Action Buttons for Active Tab
                ScrollArea::horizontal()
                    .id_salt("bim_ribbon_actions_scroll")
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            match self.active_ribbon_tab {
                                BimRibbonTab::Architecture => {
                                    let tools = [
                                        ("🧱 Compound Wall", "Draft 3D layered wall with compound finishes (WA)"),
                                        ("🪟 Curtain Wall", "Parametric glazed curtain grid wall"),
                                        ("🚪 Single Door", "Insert hosted swing door (DR)"),
                                        ("🚪 Double Door", "Insert hosted double glass door"),
                                        ("🪟 Window", "Place casement window void (WN)"),
                                        ("🏛️ Column", "Place architectural column encasement (CL)"),
                                        ("🔲 Floor Slab", "Cast-in-place concrete floor slab (SB)"),
                                        ("🏠 Roof Skeleton", "Generate straight-skeleton hip/gable roof"),
                                        ("🪜 Stairs", "Monolithic concrete circulation stair (ST)"),
                                        ("🛡️ Railing", "Safety perimeter handrail"),
                                        ("🏷️ Room Tag", "Auto-detect room polygon and tag net area (RM)"),
                                        ("⬛ Shaft Void", "Punch multi-story vertical penetration shaft"),
                                        ("📐 Grid Axis", "Add structural grid line datum (GR)"),
                                        ("📏 Level Datum", "Add building elevation datum level (LL)"),
                                    ];
                                    for (tool, tip) in tools {
                                        if ui.add(egui::Button::new(RichText::new(tool).size(10.0).color(theme.text_primary))
                                            .fill(theme.bg_card).corner_radius(CornerRadius::same(5))).on_hover_text(tip).clicked() {
                                            self.active_tool_status = format!("🔧 Active Architectural Tool: {} • Click viewport to place or define geometry", tool);
                                            self.bim_console_logs.push(format!("Architectural tool activated: {}", tool));
                                        }
                                    }
                                }
                                BimRibbonTab::Structure => {
                                    let tools = [
                                        ("🏗️ Structural Column", "Place W-Shape, HSS, or RC column"),
                                        ("📏 Primary Girder", "Place structural primary steel/concrete beam"),
                                        ("📐 Joist System", "Span repetitive secondary beam floor grid"),
                                        ("🕸️ Space Truss", "Place 3D double-layer MERO lattice"),
                                        ("⚡ Chevron Brace", "Place lateral seismic chevron brace"),
                                        ("📦 Pad Footing", "Place isolated spread pad foundation"),
                                        ("🟦 Raft Mat Slab", "Place heavy foundation raft slab"),
                                        ("📍 Piles & Cap", "Place bored pile foundation with cap"),
                                        ("🧱 Shear Wall", "Cast ductile RC seismic shear core wall"),
                                        ("🪡 Rebar Cage 3D", "Detail longitudinal bars and stirrup ties"),
                                        ("〰️ PT Tendon", "Route parabolic post-tensioned tendon profile"),
                                        ("🔩 Base Plate", "Model anchor bolt steel base plate"),
                                        ("🔗 Moment Joint", "Model beam-to-column moment flange connection"),
                                        ("📍 Support Restraint", "Apply Fixed, Pinned, or Roller boundary conditions"),
                                    ];
                                    for (tool, tip) in tools {
                                        if ui.add(egui::Button::new(RichText::new(tool).size(10.0).color(theme.text_primary))
                                            .fill(theme.bg_card).corner_radius(CornerRadius::same(5))).on_hover_text(tip).clicked() {
                                            self.active_tool_status = format!("🔧 Active Structural Tool: {} • Ready for member placement", tool);
                                            self.bim_console_logs.push(format!("Structural tool activated: {}", tool));
                                        }
                                    }
                                }
                                BimRibbonTab::MepSystems => {
                                    let tools = [
                                        ("💨 Supply Duct", "Route rectangular supply air duct run (DT)"),
                                        ("🌬️ Air Terminal", "Place ceiling supply/return air diffuser"),
                                        ("❄️ Rooftop AHU", "Place primary air handling package unit"),
                                        ("🌀 VAV Terminal", "Place variable air volume terminal box"),
                                        ("🚰 Domestic Pipe", "Route cold/hot domestic water supply (PI)"),
                                        ("🚽 Sanitary DWV", "Route gravity wastewater and soil stack"),
                                        ("💧 Booster Pump", "Place domestic water pressure booster pump"),
                                        ("⚡ Cable Tray", "Route perforated electrical cable ladder (CT)"),
                                        ("🔌 Conduit Run", "Route electrical branch circuit conduit"),
                                        ("🔋 Main Switchboard", "Place primary low-voltage MSB panel"),
                                        ("💡 LED Troffer", "Place 600x600 architectural ceiling light"),
                                        ("🚒 Fire Sprinkler", "Place automatic pendant sprinkler head (SP)"),
                                        ("🚿 Wet Riser", "Route vertical fire standpipe riser"),
                                    ];
                                    for (tool, tip) in tools {
                                        if ui.add(egui::Button::new(RichText::new(tool).size(10.0).color(theme.text_primary))
                                            .fill(theme.bg_card).corner_radius(CornerRadius::same(5))).on_hover_text(tip).clicked() {
                                            self.active_tool_status = format!("🔧 Active MEP Tool: {} • Routing active across level", tool);
                                            self.bim_console_logs.push(format!("MEP systems tool activated: {}", tool));
                                        }
                                    }
                                }
                                BimRibbonTab::CivilInfra => {
                                    let tools = [
                                        ("🛣️ Road Centerline", "Define 3D horizontal alignment spline"),
                                        ("📐 Profile Corridor", "Extrude road cross-section corridor"),
                                        ("⛰️ DEM Terrain", "Import digital elevation model TIN surface"),
                                        ("🚜 Cut & Fill Mesh", "Compute earthwork volumetric balance"),
                                        ("🌉 Box Girder", "Place prestressed segmental concrete bridge girder"),
                                        ("🏛️ Pier & Pier Cap", "Place reinforced concrete bridge substructure"),
                                        ("🧱 Abutment", "Model end retaining bridge abutment"),
                                        ("🕳️ Drainage Culvert", "Place stormwater box culvert run"),
                                    ];
                                    for (tool, tip) in tools {
                                        if ui.add(egui::Button::new(RichText::new(tool).size(10.0).color(theme.text_primary))
                                            .fill(theme.bg_card).corner_radius(CornerRadius::same(5))).on_hover_text(tip).clicked() {
                                            self.active_tool_status = format!("🔧 Active Civil Infrastructure Tool: {}", tool);
                                            self.bim_console_logs.push(format!("Civil infrastructure tool activated: {}", tool));
                                        }
                                    }
                                }
                                BimRibbonTab::AnalysisFea => {
                                    let tools = [
                                        ("⚡ Run Linear Static", "Execute 3D space frame matrix stiffness solver"),
                                        ("🔄 P-Delta Second Order", "Compute geometric non-linear stiffness"),
                                        ("🌊 Modal Dynamics", "Solve natural frequencies & mode shapes (Lanczos)"),
                                        ("📉 Pushover Analysis", "Perform non-linear static capacity spectrum test"),
                                        ("💥 N-1 Disruption", "Evaluate progressive collapse redundancy ARI"),
                                        ("📊 Von Mises Stress", "Display composite stress gradient contour"),
                                        ("📈 NFD Axial Force", "Render 3D axial force envelope"),
                                        ("⚡ SFD Shear Force", "Render 3D shear force envelope"),
                                        ("📐 BMD Moment", "Render 3D bending moment envelope"),
                                    ];
                                    for (tool, tip) in tools {
                                        if ui.add(egui::Button::new(RichText::new(tool).size(10.0).color(theme.text_primary))
                                            .fill(theme.bg_card).corner_radius(CornerRadius::same(5))).on_hover_text(tip).clicked() {
                                            self.active_tool_status = format!("⚡ Executed Solver Command: {}", tool);
                                            self.bim_console_logs.push(format!("Analysis command executed: {}", tool));
                                            if tool.contains("NFD") { self.render_mode = Render3DMode::AxialForceNFD; }
                                            else if tool.contains("SFD") { self.render_mode = Render3DMode::ShearForceSFD; }
                                            else if tool.contains("BMD") { self.render_mode = Render3DMode::BendingMomentBMD; }
                                        }
                                    }
                                }
                                BimRibbonTab::Time4D => {
                                    let tools = [
                                        ("▶ Play 4D", "Start real-time construction phase playback"),
                                        ("⏸ Pause", "Pause 4D erection timeline"),
                                        ("⏮ Day 1", "Jump to foundation kickoff"),
                                        ("⏭ Day 180", "Jump to completed building handover"),
                                        ("📅 CPM Gantt View", "Open critical path schedule chart"),
                                        ("🔗 Auto WBS Linker", "Map building storeys to construction tasks"),
                                    ];
                                    for (tool, tip) in tools {
                                        if ui.add(egui::Button::new(RichText::new(tool).size(10.0).color(theme.text_primary))
                                            .fill(theme.bg_card).corner_radius(CornerRadius::same(5))).on_hover_text(tip).clicked() {
                                            if tool.contains("Play") { self.simulation_4d_playing = true; }
                                            else if tool.contains("Pause") { self.simulation_4d_playing = false; }
                                            else if tool.contains("Day 1") { self.simulation_4d_day = 1; }
                                            else if tool.contains("Day 180") { self.simulation_4d_day = 180; }
                                            self.active_tool_status = format!("⏱️ 4D Construction Simulation: {}", tool);
                                            self.bottom_deck_expanded = true;
                                            self.active_bottom_deck_tab = BottomDeckTab::TimeSimulation4D;
                                        }
                                    }
                                }
                                BimRibbonTab::Cost5D => {
                                    let tools = [
                                        ("📊 Dynamic QTO", "Extract automatic geometric quantity takeoff"),
                                        ("💲 Bill of Quantities (BoQ)", "Compute unit-rate cost ledger (RAB)"),
                                        ("🗺️ 3D Cost Heatmap", "Display color-coded financial cost intensity"),
                                        ("📈 S-Curve Cash Flow", "Simulate monthly progress expenditure curve"),
                                        ("🪙 Currency USD/EUR/IDR", "Toggle project base currency"),
                                    ];
                                    for (tool, tip) in tools {
                                        if ui.add(egui::Button::new(RichText::new(tool).size(10.0).color(theme.text_primary))
                                            .fill(theme.bg_card).corner_radius(CornerRadius::same(5))).on_hover_text(tip).clicked() {
                                            self.active_tool_status = format!("💰 5D Cost Estimating: {}", tool);
                                            self.bottom_deck_expanded = true;
                                            self.active_bottom_deck_tab = BottomDeckTab::ScheduleQto;
                                        }
                                    }
                                }
                                BimRibbonTab::Sustainability6D => {
                                    let tools = [
                                        ("🌱 Embodied Carbon (A1-A3)", "Calculate cradle-to-gate GWP carbon ledger"),
                                        ("💨 Operational EUI", "Simulate heating/cooling energy intensity"),
                                        ("☀️ Solar Heliodon", "Render 365-day solar sunpath dome"),
                                        ("⛅ Cast Shadows", "Calculate shadow envelope on adjacent parcels"),
                                        ("🏆 LEED Scorecard", "Audit credits for Green Building rating"),
                                    ];
                                    for (tool, tip) in tools {
                                        if ui.add(egui::Button::new(RichText::new(tool).size(10.0).color(theme.text_primary))
                                            .fill(theme.bg_card).corner_radius(CornerRadius::same(5))).on_hover_text(tip).clicked() {
                                            self.active_tool_status = format!("🌱 6D Sustainability Command: {}", tool);
                                        }
                                    }
                                }
                                BimRibbonTab::FacilityTwin7D => {
                                    let tools = [
                                        ("🏷️ Asset Tagging", "Inspect COBie component tags & serial numbers"),
                                        ("📡 IoT Sensor Stream", "Read structural strain and vibration telemetry"),
                                        ("🔧 Work Orders", "Schedule preventative maintenance tasks"),
                                        ("📄 COBie Spreadsheet", "Export building asset management ledger"),
                                    ];
                                    for (tool, tip) in tools {
                                        if ui.add(egui::Button::new(RichText::new(tool).size(10.0).color(theme.text_primary))
                                            .fill(theme.bg_card).corner_radius(CornerRadius::same(5))).on_hover_text(tip).clicked() {
                                            self.active_tool_status = format!("🏢 7D Digital Twin Command: {}", tool);
                                        }
                                    }
                                }
                                BimRibbonTab::CoordinationAudit => {
                                    let tools = [
                                        ("⚔️ Run Clash Detection", "Detect hard collisions and clearance breaches"),
                                        ("🛡️ Solibri Health Audit", "Execute automated rule-based compliance check"),
                                        ("➕ Create BCF Topic", "Record issue with 3D camera viewpoint snapshot"),
                                        ("📬 Export BCF XML", "Package topics into standardized BCF 3.0 archive"),
                                    ];
                                    for (tool, tip) in tools {
                                        if ui.add(egui::Button::new(RichText::new(tool).size(10.0).color(theme.text_primary))
                                            .fill(theme.bg_card).corner_radius(CornerRadius::same(5))).on_hover_text(tip).clicked() {
                                            self.active_tool_status = format!("🛡️ Coordination: {}", tool);
                                            self.active_sidebar_tab = SidebarTab::ClashMatrix;
                                        }
                                    }
                                }
                                BimRibbonTab::OpenBimSpec => {
                                    let tools = [
                                        ("📥 Import IFC4 STEP", "Parse OpenBIM ISO 10303-21 physical file"),
                                        ("📤 Export IFC4 RV", "Export buildingSMART certified IFC4 Reference View"),
                                        ("📋 Export ifcJSON-4", "Serialize spatial graph into buildingSMART JSON"),
                                        ("🌐 Speckle Sync", "Sync geometry stream with Speckle server"),
                                        ("🧊 Wavefront OBJ", "Export triangulated 3D mesh for visualization"),
                                        ("📏 AutoCAD 3D DXF", "Export 3D CAD vectors with layers"),
                                    ];
                                    for (tool, tip) in tools {
                                        if ui.add(egui::Button::new(RichText::new(tool).size(10.0).color(theme.text_primary))
                                            .fill(theme.bg_card).corner_radius(CornerRadius::same(5))).on_hover_text(tip).clicked() {
                                            self.active_tool_status = format!("🌐 OpenBIM Action: {}", tool);
                                            self.bim_console_logs.push(format!("OpenBIM action executed: {}", tool));
                                        }
                                    }
                                }
                                BimRibbonTab::SheetsDed => {
                                    let tools = [
                                        ("📄 New Sheet (A1)", "Create standard 841x594mm title block drawing"),
                                        ("🔲 Viewport Place", "Drag and drop floor plan or 3D view onto sheet"),
                                        ("🏷️ Title Block Stamp", "Fill project metadata, revisions, and approval stamps"),
                                        ("📐 Smart Dimensions", "Place aligned, linear, and spot elevation tags"),
                                        ("📑 Rebar BBS Table", "Generate ACI/BS bar bending schedule schedule"),
                                    ];
                                    for (tool, tip) in tools {
                                        if ui.add(egui::Button::new(RichText::new(tool).size(10.0).color(theme.text_primary))
                                            .fill(theme.bg_card).corner_radius(CornerRadius::same(5))).on_hover_text(tip).clicked() {
                                            self.active_tool_status = format!("📑 Documentation Action: {}", tool);
                                        }
                                    }
                                }
                            }
                        });
                    });

                ui.add_space(3.0);
                // Tier 3: Active Tool Prompt Toast Bar
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&self.active_tool_status).size(10.0).color(theme.accent_blue));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new("[Press ESC to cancel / Select]").size(9.5).color(theme.text_muted));
                    });
                });
            });
    }


}
