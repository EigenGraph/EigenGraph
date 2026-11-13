//! features.rs — Centralized Feature Gate & Unlock Roadmap Registry for EigenGraph.
//! Catalogs active engineering capabilities and upcoming Phase 31+ locked workspace slots
//! across all 15 BIM/Structural domains so layouts stay frozen while capabilities unlock cleanly.

use crate::models::NavigationPillar;

/// Deployment status of an EigenGraph domain capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureStatus {
    /// Fully wired and interactive in the current build.
    Active,
    /// Workspace layout and state contracts are in place, gated until the target unlock phase.
    Locked { target_phase: &'static str },
}

impl FeatureStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    pub fn badge_label(&self) -> String {
        match self {
            Self::Active => "ACTIVE".to_string(),
            Self::Locked { target_phase } => format!("LOCKED ({target_phase})"),
        }
    }
}

/// Canonical identifier for every tracked capability and unlockable slot across the 15 pillars.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeatureId {
    // 1. My Library & Vault
    VaultPortfolioManager,
    GlobalBimFamilyCatalog,
    BrowserIndexedDbPersistence,

    // 2. AI Agent & Virtual Firm
    VirtualEngineeringFirmBoard,
    AutonomousCopilotCalculators,
    LiveGeminiToolOrchestrator,

    // 3. Research & 1D GIS
    SlippyMapWebMercator,
    GeodeticCrsUtmProjection,
    DemTinMeshCutFill,
    LiveRasterMapTileFetcher,

    // 4. Parametric & 2D Studio
    Planar2DCadAuthoring,
    LiveSectionCutPoche,
    DynamoAlgorithmicStudio,
    ParametricBidirectionalSync,

    // 5. Geometry & 3D BIM
    CsgBrep3DViewport,
    BimMasterRibbonAndBrowser,
    Bcf21CoordinationHub,
    LiveBvhGeometricClashSolver,

    // 6. Analysis & Multi-Hazard Simulation
    LinearFemAndSvdAri,
    SeismicModalPushoverNltha,
    MemberCodeDesignSuite,
    ExtremeBlastFireFemaP58,
    LiveBimToFemAutoSync,

    // 7. Time & 4D Scheduling
    Synchro4DErectionPlayer,
    BimDriven4DCriticalPathSync,

    // 8. Cost & 5D Budgeting
    Parametric5DRabMultiCurrency,
    LiveBimQtoBoqSync,

    // 9. Sustainability & 6D Energy/LCA
    WholeLifeCarbonEn15978,
    BimMaterialLcaAutoSync,

    // 10. Facility Management & 7D Digital Twin
    DigitalTwinShmCobie3,
    LiveIotTelemetryIngestion,

    // 11. Safety & 8D Site Risk
    TowerCraneFallRiskRadar,
    SitePerimeter3DCraneCollision,

    // 12. Lean Construction & 9D Logistics
    CuttingStock1DAndLastPlanner,
    LiveRebarBbsCuttingSync,

    // 13. Industrialized & 10D DfMA / OpenBIM
    OpenBimIfcReaderEditorClashDiff,
    DfmaSupplyChainTracker,
    WasmLocalIfcFileDropPicker,

    // 14. Prediction & AI Structural Aging
    FosmReliabilityProgressiveCollapse,
    LiveShmAnomalyForecasting,

    // 15. Report & Publishing Suite
    DedDrawingSheetsPdfDxfSvg,
    LivingDocsSheetsSlides,
    AutoProjectedBimDedSheets,
}

/// Metadata record describing one capability inside the centralized Feature Registry.
#[derive(Debug, Clone)]
pub struct FeatureEntry {
    pub id: FeatureId,
    pub pillar: NavigationPillar,
    pub code: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub status: FeatureStatus,
    pub module_hint: &'static str,
    pub prioritized: bool,
}

/// Centralized catalog of active and unlockable capabilities for Settings & inline feature gates.
#[derive(Debug, Clone)]
pub struct FeatureRegistry {
    pub entries: Vec<FeatureEntry>,
}

impl Default for FeatureRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl FeatureRegistry {
    pub fn new() -> Self {
        use FeatureStatus::{Active, Locked};
        let entries = vec![
            // 1. My Library
            FeatureEntry {
                id: FeatureId::VaultPortfolioManager,
                pillar: NavigationPillar::MyLibrary,
                code: "LIB-01",
                title: "Portfolio Vault & Audit Certificate",
                description: "Local project archive, SHA-256 dossier fingerprinting, and standalone 3D HTML5 viewer export.",
                status: Active,
                module_hint: "src/core/vault.rs + src/views/my_project.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::GlobalBimFamilyCatalog,
                pillar: NavigationPillar::MyLibrary,
                code: "LIB-02",
                title: "Global BIM Family & Parameter Catalog",
                description: "Revit RFA / buildingSMART IFC parameter browser across MasterFormat & UniFormat families.",
                status: Active,
                module_hint: "src/views/my_project.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::BrowserIndexedDbPersistence,
                pillar: NavigationPillar::MyLibrary,
                code: "LIB-03",
                title: "Browser IndexedDB & LocalStorage Vault Sync",
                description: "Automatic client-side WASM persistence across browser reloads with JSON archive import/export.",
                status: Locked { target_phase: "Phase 31" },
                module_hint: "src/core/web_bridge.rs + src/core/vault.rs",
                prioritized: true,
            },

            // 2. AI Agent
            FeatureEntry {
                id: FeatureId::VirtualEngineeringFirmBoard,
                pillar: NavigationPillar::AiAgent,
                code: "AI-01",
                title: "Multi-Agent Virtual Engineering Firm",
                description: "PM, Senior Drafter, FEA Analyst, and QS Specialist organigram, collaboration log, and telemetry.",
                status: Active,
                module_hint: "src/views/ai_firm.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::AutonomousCopilotCalculators,
                pillar: NavigationPillar::AiAgent,
                code: "AI-02",
                title: "Autonomous Engineering Copilot Studio",
                description: "Deterministic ACI 318-19 Shear Wall, ASCE 7-22 Wind MWFRS, and ISO 14040 Carbon solvers.",
                status: Active,
                module_hint: "src/views/ai_firm.rs + src/models.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::LiveGeminiToolOrchestrator,
                pillar: NavigationPillar::AiAgent,
                code: "AI-03",
                title: "Live LLM Tool-Calling BIM Orchestrator",
                description: "Async Gemini REST integration executing validated BimCommand edits against the live structural model.",
                status: Locked { target_phase: "Phase 32" },
                module_hint: "src/core/web_bridge.rs + src/views/ai_assistant.rs",
                prioritized: false,
            },

            // 3. Research & 1D GIS
            FeatureEntry {
                id: FeatureId::SlippyMapWebMercator,
                pillar: NavigationPillar::ResearchConcept,
                code: "1D-01",
                title: "EPSG:3857 Slippy Map & Geodetic Pins",
                description: "Interactive Web Mercator quadtree viewport, seismic site pins, and offline vector terrain.",
                status: Active,
                module_hint: "src/core/gis/map_widget.rs + src/views/research.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::GeodeticCrsUtmProjection,
                pillar: NavigationPillar::ResearchConcept,
                code: "1D-02",
                title: "WGS84 / UTM CRS & IfcMapConversion Engine",
                description: "Forward/inverse UTM coordinate transforms, meridian convergence, and IFC4 georeferencing export.",
                status: Active,
                module_hint: "src/core/gis/crs_projection.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::DemTinMeshCutFill,
                pillar: NavigationPillar::ResearchConcept,
                code: "1D-03",
                title: "DEM Topographic TIN Mesh & Cut/Fill Grading",
                description: "10,082-triangle Delaunay terrain, 1m/5m contours, SNI 8460 >30 deg slope hazard shading, and earthwork balance.",
                status: Active,
                module_hint: "src/core/gis/tin_surface.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::LiveRasterMapTileFetcher,
                pillar: NavigationPillar::ResearchConcept,
                code: "1D-04",
                title: "Live Raster Tile Cache & GeoTIFF/CSV DEM Importer",
                description: "Async OSM/Esri raster tile texture streaming and browser survey point-cloud CSV/XYZ ingestion.",
                status: Locked { target_phase: "Phase 31" },
                module_hint: "src/core/gis/map_widget.rs + src/core/web_bridge.rs",
                prioritized: true,
            },

            // 4. Parametric & 2D Studio
            FeatureEntry {
                id: FeatureId::Planar2DCadAuthoring,
                pillar: NavigationPillar::ParametricDED,
                code: "2D-01",
                title: "2D Vector CAD Plan & Half-Edge Room Engine",
                description: "Object snaps, compound walls, openings, beams, columns, rebar detailing, and automatic room bounding.",
                status: Active,
                module_hint: "src/views/parametric/canvas_2d.rs + src/core/bim.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::LiveSectionCutPoche,
                pillar: NavigationPillar::ParametricDED,
                code: "2D-02",
                title: "Live Section Cut Poche & Sheet A101 Composer",
                description: "Real-time vertical section slicing, rebar cut dots, and ISO Vector PDF/SVG/IFC4 sheet export.",
                status: Active,
                module_hint: "src/views/parametric/canvas_2d.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::DynamoAlgorithmicStudio,
                pillar: NavigationPillar::ParametricDED,
                code: "2D-03",
                title: "Visual Dynamo Node Graph & 3D Wireframe Preview",
                description: "Draggable parametric presets (Diagrid, Geodesic Dome, Voronoi Canopy) with 3D orbit preview & bake.",
                status: Active,
                module_hint: "src/views/parametric/dynamo_studio.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::ParametricBidirectionalSync,
                pillar: NavigationPillar::ParametricDED,
                code: "2D-04",
                title: "Bidirectional 2D/Dynamo <-> 3D BIM Bus Sync",
                description: "Shares one unified BimCommandBus between Parametric 2D/Dynamo authoring and Geometry 3D viewport.",
                status: Locked { target_phase: "Phase 31" },
                module_hint: "src/app.rs + src/core/bim.rs",
                prioritized: true,
            },

            // 5. Geometry & 3D BIM
            FeatureEntry {
                id: FeatureId::CsgBrep3DViewport,
                pillar: NavigationPillar::GeometryAssembly,
                code: "3D-01",
                title: "3D CSG B-Rep & Analytical Frame Viewport",
                description: "360-deg orbit canvas, NFD/SFD/BMD overlays, interactive section box, ViewCube, and 3D rebar cages.",
                status: Active,
                module_hint: "src/views/build/viewport_3d.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::BimMasterRibbonAndBrowser,
                pillar: NavigationPillar::GeometryAssembly,
                code: "3D-02",
                title: "12-Tab Multi-Discipline Ribbon & Spatial Browser",
                description: "Architecture, Structure, MEP, Civil, Sim, 4D-7D ribbon tools, IFC SBS tree, and Solibri quality audit.",
                status: Active,
                module_hint: "src/views/build/ribbon.rs + src/views/build/sidebar.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::Bcf21CoordinationHub,
                pillar: NavigationPillar::GeometryAssembly,
                code: "3D-03",
                title: "BCF 2.1 Issue Threads & Viewpoint Restorer",
                description: "Camera-captured BCF topics, review comments, priority triage, and BCF 2.1 XML export.",
                status: Active,
                module_hint: "src/views/build/clash_bcf.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::LiveBvhGeometricClashSolver,
                pillar: NavigationPillar::GeometryAssembly,
                code: "3D-04",
                title: "Live Bvh3D Spatial Clash Detection Engine",
                description: "Real-time BVH/AABB collision solver across active walls, columns, beams, MEP runs, and rebar cages.",
                status: Locked { target_phase: "Phase 31" },
                module_hint: "src/core/csg.rs + src/views/build/clash_bcf.rs",
                prioritized: true,
            },

            // 6. Analysis & Simulation
            FeatureEntry {
                id: FeatureId::LinearFemAndSvdAri,
                pillar: NavigationPillar::AnalysisSimulation,
                code: "SIM-01",
                title: "Linear 3D FEA, SVD Kinematic Guard & ARI Matrix",
                description: "Sherman-Morrison/Woodbury N-1 & N-2 redundancy screening, SVD rank verification, and cascading collapse.",
                status: Active,
                module_hint: "src/views/analyze/core_fea.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::SeismicModalPushoverNltha,
                pillar: NavigationPillar::AnalysisSimulation,
                code: "SIM-02",
                title: "SNI 1726 Modal, Pushover CSM, NL-THA & Isolation",
                description: "Lanczos modal spectra, capacity spectrum pushover, Newmark nonlinear time-history, and LRB/FPS isolation.",
                status: Active,
                module_hint: "src/views/analyze/seismic_dynamic.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::MemberCodeDesignSuite,
                pillar: NavigationPillar::AnalysisSimulation,
                code: "SIM-03",
                title: "RC P-M, Steel Connection, Composite, PT & Shear Wall",
                description: "SNI/ACI/AISC code verifications across RC columns, steel joints, composite decks, PT girders, and shear walls.",
                status: Active,
                module_hint: "src/views/analyze/member_design.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::ExtremeBlastFireFemaP58,
                pillar: NavigationPillar::AnalysisSimulation,
                code: "SIM-04",
                title: "FEMA P-58 Loss, UFC 3-340 Blast & AISC Fire",
                description: "Probabilistic REDi loss recovery, Kingery-Bulmash SDOF blast, and coupled thermo-mechanical fire ratings.",
                status: Active,
                module_hint: "src/views/analyze/extreme_hazards.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::LiveBimToFemAutoSync,
                pillar: NavigationPillar::AnalysisSimulation,
                code: "SIM-05",
                title: "Live BimProject -> FemSolver Reactive Sync",
                description: "Automatically converts active 3D BIM geometry & 1D GIS seismic parameters into live FEA runs.",
                status: Locked { target_phase: "Phase 31" },
                module_hint: "src/core/bim.rs + src/views/analyze/mod.rs",
                prioritized: true,
            },

            // 7. Time & 4D
            FeatureEntry {
                id: FeatureId::Synchro4DErectionPlayer,
                pillar: NavigationPillar::TimeScheduling,
                code: "4D-01",
                title: "4D Synchro Construction Player & SVD Staging",
                description: "Day 1-360 construction elevation animation, crane progression, CPM tracking, and staged SVD safety.",
                status: Active,
                module_hint: "src/views/planning.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::BimDriven4DCriticalPathSync,
                pillar: NavigationPillar::TimeScheduling,
                code: "4D-02",
                title: "Element-Linked 4D WBS & 3D Erection Coupling",
                description: "Binds WBS tasks directly to active BimProject element IDs to isolate active erection phases in 3D.",
                status: Locked { target_phase: "Phase 32" },
                module_hint: "src/core/schedule.rs + src/views/planning.rs",
                prioritized: false,
            },

            // 8. Cost & 5D
            FeatureEntry {
                id: FeatureId::Parametric5DRabMultiCurrency,
                pillar: NavigationPillar::CostBudgeting,
                code: "5D-01",
                title: "5D Dynamic RAB, Multi-Currency WBS & Cost-ARI",
                description: "Parametric AHSP pricing across IDR/USD/EUR/SGD, MasterFormat WBS ledger, and Cost-vs-ARI trade-off.",
                status: Active,
                module_hint: "src/views/planning.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::LiveBimQtoBoqSync,
                pillar: NavigationPillar::CostBudgeting,
                code: "5D-02",
                title: "Live BimProject Volume & Rebar QTO Ledger Sync",
                description: "Extracts concrete, structural steel, wall area, and rebar tonnage dynamically from active 3D BIM.",
                status: Locked { target_phase: "Phase 32" },
                module_hint: "src/core/cost.rs + src/views/planning.rs",
                prioritized: false,
            },

            // 9. Sustainability & 6D
            FeatureEntry {
                id: FeatureId::WholeLifeCarbonEn15978,
                pillar: NavigationPillar::SustainabilityEnergy,
                code: "6D-01",
                title: "EN 15978 Whole-Life Carbon & Green Scorecards",
                description: "Modules A1-D lifecycle carbon audit, EAF/H2-DRI routes, and GREENSHIP/LEED/BREEAM credit tracking.",
                status: Active,
                module_hint: "src/views/sustainability.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::BimMaterialLcaAutoSync,
                pillar: NavigationPillar::SustainabilityEnergy,
                code: "6D-02",
                title: "Live BIM Material Embodied Carbon Sync",
                description: "Computes real-time A1-A3 CO2e intensity directly from active BimProject volumes and steel profiles.",
                status: Locked { target_phase: "Phase 32" },
                module_hint: "src/core/carbon.rs + src/views/sustainability.rs",
                prioritized: false,
            },

            // 10. Facility & 7D
            FeatureEntry {
                id: FeatureId::DigitalTwinShmCobie3,
                pillar: NavigationPillar::FacilityManagement,
                code: "7D-01",
                title: "7D Digital Twin, SHM Telemetry & COBie 3.0",
                description: "Strain/accelerometer sparklines, buildingSMART COBie spreadsheet inspector, CMMS dispatch, and 30-yr LCC.",
                status: Active,
                module_hint: "src/views/facility.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::LiveIotTelemetryIngestion,
                pillar: NavigationPillar::FacilityManagement,
                code: "7D-02",
                title: "Live WebSocket/MQTT IoT Sensor Stream Ingestion",
                description: "Connects browser WebSocket endpoints for live field strain gauge and accelerometer SHM streaming.",
                status: Locked { target_phase: "Phase 33" },
                module_hint: "src/core/web_bridge.rs + src/views/facility.rs",
                prioritized: false,
            },

            // 11. Safety & 8D
            FeatureEntry {
                id: FeatureId::TowerCraneFallRiskRadar,
                pillar: NavigationPillar::SafetyHealth,
                code: "8D-01",
                title: "8D Tower Crane Radar & Slab Fall Guard",
                description: "Crane pick load vs radius envelope checks, OSHA 1926.502 edge guards, evacuation routes, and HIRADC matrix.",
                status: Active,
                module_hint: "src/views/safety.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::SitePerimeter3DCraneCollision,
                pillar: NavigationPillar::SafetyHealth,
                code: "8D-02",
                title: "3D Crane Jib vs BIM Perimeter Clearance Check",
                description: "Evaluates 3D swing clearance against active BimProject building envelopes and site boundaries.",
                status: Locked { target_phase: "Phase 33" },
                module_hint: "src/views/safety.rs + src/core/bim.rs",
                prioritized: false,
            },

            // 12. Lean & 9D
            FeatureEntry {
                id: FeatureId::CuttingStock1DAndLastPlanner,
                pillar: NavigationPillar::LeanConstruction,
                code: "9D-01",
                title: "1D Cutting Stock Optimizer & Last Planner LPS",
                description: "12m rebar waste minimization, 6-week lookahead constraint clearing, PPC chart, and root-cause Pareto.",
                status: Active,
                module_hint: "src/views/lean.rs + src/core/cso.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::LiveRebarBbsCuttingSync,
                pillar: NavigationPillar::LeanConstruction,
                code: "9D-02",
                title: "Live BIM Rebar Schedule -> 1D Cutting Sync",
                description: "Feeds active BimProject rebar bar marks and lengths directly into the 1D cutting stock optimizer.",
                status: Locked { target_phase: "Phase 32" },
                module_hint: "src/core/cso.rs + src/views/lean.rs",
                prioritized: false,
            },

            // 13. Industrialized & 10D
            FeatureEntry {
                id: FeatureId::OpenBimIfcReaderEditorClashDiff,
                pillar: NavigationPillar::IndustrializedAutomation,
                code: "10D-01",
                title: "OpenBIM IFC2x3/4/4.3 Reader, Editor, Clash & IDS",
                description: "Full ISO 10303-21 STEP parser/serializer, transaction undo/redo, spatial clash, diff, and IDS audit.",
                status: Active,
                module_hint: "src/views/industrialized/openbim_studio.rs + src/core/ifc.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::DfmaSupplyChainTracker,
                pillar: NavigationPillar::IndustrializedAutomation,
                code: "10D-02",
                title: "10D Hub, Prefab DfMA Pipeline & Robotic CNC",
                description: "6-stage prefab supply chain, QR check-in console, hoist SWL gauges, and DSTV .nc1 robotic welding.",
                status: Active,
                module_hint: "src/views/industrialized/dfma_robotics.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::WasmLocalIfcFileDropPicker,
                pillar: NavigationPillar::IndustrializedAutomation,
                code: "10D-03",
                title: "WASM Browser File Picker & Drag-and-Drop IFC",
                description: "Imports local .ifc, .json, and .e2k files directly in browser via HTML5 file picker and drop events.",
                status: Locked { target_phase: "Phase 31" },
                module_hint: "src/core/web_bridge.rs + src/views/industrialized/openbim_studio.rs",
                prioritized: true,
            },

            // 14. Prediction
            FeatureEntry {
                id: FeatureId::FosmReliabilityProgressiveCollapse,
                pillar: NavigationPillar::Prediction,
                code: "PRED-01",
                title: "FOSM Reliability, FEMA Fragility & UFC Collapse",
                description: "Reliability index beta, seismic collapse fragility curves, DoD UFC 4-023-03 APM, and 50-year aging.",
                status: Active,
                module_hint: "src/views/prediction.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::LiveShmAnomalyForecasting,
                pillar: NavigationPillar::Prediction,
                code: "PRED-02",
                title: "Live FEA & SHM Stress Anomaly Forecasting",
                description: "Computes real-time member vulnerability rankings from active FEA forces and 7D SHM sensor trends.",
                status: Locked { target_phase: "Phase 33" },
                module_hint: "src/core/reliability.rs + src/views/prediction.rs",
                prioritized: false,
            },

            // 15. Report & Publishing
            FeatureEntry {
                id: FeatureId::DedDrawingSheetsPdfDxfSvg,
                pillar: NavigationPillar::ReportPublishing,
                code: "PUB-01",
                title: "2D Drawing Sheets (DED) & Vector PDF/DXF/SVG",
                description: "ISO A0-A3 & ANSI Arch D multi-viewport drafting sheets, PE digital seal, and layered DXF/PDF exports.",
                status: Active,
                module_hint: "src/views/results.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::LivingDocsSheetsSlides,
                pillar: NavigationPillar::ReportPublishing,
                code: "PUB-02",
                title: "Living Docs, Formula BoQ Sheets & Client Slides",
                description: "Editable calculation memos, live spreadsheet formulas, executive slide decks, and full audit dossiers.",
                status: Active,
                module_hint: "src/views/results.rs",
                prioritized: false,
            },
            FeatureEntry {
                id: FeatureId::AutoProjectedBimDedSheets,
                pillar: NavigationPillar::ReportPublishing,
                code: "PUB-03",
                title: "Auto-Projected DED Sheets from Active BimProject",
                description: "Dynamically projects active plans, section cuts, and rebar/column schedules onto printable sheets.",
                status: Locked { target_phase: "Phase 32" },
                module_hint: "src/core/bim.rs + src/views/results.rs",
                prioritized: false,
            },
        ];

        Self { entries }
    }

    pub fn entry(&self, id: FeatureId) -> Option<&FeatureEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    pub fn entry_mut(&mut self, id: FeatureId) -> Option<&mut FeatureEntry> {
        self.entries.iter_mut().find(|e| e.id == id)
    }

    pub fn is_active(&self, id: FeatureId) -> bool {
        self.entry(id).is_some_and(|e| e.status.is_active())
    }

    pub fn toggle_prioritized(&mut self, id: FeatureId) {
        if let Some(entry) = self.entry_mut(id) {
            if !entry.status.is_active() {
                entry.prioritized = !entry.prioritized;
            }
        }
    }

    pub fn total_count(&self) -> usize {
        self.entries.len()
    }

    pub fn active_count(&self) -> usize {
        self.entries.iter().filter(|e| e.status.is_active()).count()
    }

    pub fn locked_count(&self) -> usize {
        self.entries.iter().filter(|e| !e.status.is_active()).count()
    }

    pub fn prioritized_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| !e.status.is_active() && e.prioritized)
            .count()
    }

    pub fn prioritize_phase(&mut self, phase_prefix: &str) {
        for entry in &mut self.entries {
            if let FeatureStatus::Locked { target_phase } = entry.status {
                if target_phase.eq_ignore_ascii_case(phase_prefix) {
                    entry.prioritized = true;
                }
            }
        }
    }

    pub fn clear_all_priorities(&mut self) {
        for entry in &mut self.entries {
            entry.prioritized = false;
        }
    }

    pub fn pillar_counts(&self, pillar: NavigationPillar) -> (usize, usize) {
        let mut active = 0;
        let mut total = 0;
        for e in &self.entries {
            if e.pillar == pillar {
                total += 1;
                if e.status.is_active() {
                    active += 1;
                }
            }
        }
        (active, total)
    }

    /// Exports a markdown unlock backlog summarizing prioritized locked features for the next sprint.
    pub fn export_unlock_backlog_markdown(&self) -> String {
        let mut out = String::from("# EigenGraph Feature Unlock Queue\n\n");
        out.push_str(&format!(
            "- Active Capabilities: {}\n- Locked Workspace Slots: {}\n- Prioritized for Next Unlock: {}\n\n",
            self.active_count(),
            self.locked_count(),
            self.prioritized_count()
        ));
        out.push_str("## Locked Capabilities\n\n");
        for e in self.entries.iter().filter(|e| !e.status.is_active()) {
            let mark = if e.prioritized { "[PRIORITIZED]" } else { "[QUEUED]" };
            out.push_str(&format!(
                "- {} **{} — {}** (`{}`) | {} | `{}`\n  - {}\n",
                mark,
                e.code,
                e.title,
                e.pillar.clean_name(),
                e.status.badge_label(),
                e.module_hint,
                e.description
            ));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_registry_covers_all_15_pillars() {
        let reg = FeatureRegistry::new();
        for &pillar in NavigationPillar::all() {
            let (active, total) = reg.pillar_counts(pillar);
            assert!(
                total >= 2 && active >= 1,
                "Pillar {:?} must track at least 1 active and 2 total capabilities",
                pillar
            );
        }
        assert!(reg.active_count() >= 25);
        assert!(reg.locked_count() >= 10);
    }

    #[test]
    fn test_feature_registry_priority_toggle_and_export() {
        let mut reg = FeatureRegistry::new();
        let before = reg
            .entry(FeatureId::LiveGeminiToolOrchestrator)
            .expect("entry exists")
            .prioritized;
        reg.toggle_prioritized(FeatureId::LiveGeminiToolOrchestrator);
        let after = reg
            .entry(FeatureId::LiveGeminiToolOrchestrator)
            .expect("entry exists")
            .prioritized;
        assert_ne!(before, after);

        let md = reg.export_unlock_backlog_markdown();
        assert!(md.contains("AI-03"));
        assert!(md.contains("3D-04"));
    }
}
