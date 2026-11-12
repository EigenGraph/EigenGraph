//! models.rs — Central Data Structures for EigenGraph.
//! Contains model entities, navigation pillars, project catalogs, and UI state models.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationPillar {
    MyLibrary,
    AiAgent,
    ResearchConcept,
    ParametricDED,
    GeometryAssembly,
    AnalysisSimulation,
    TimeScheduling,
    CostBudgeting,
    SustainabilityEnergy,
    FacilityManagement,
    SafetyHealth,
    LeanConstruction,
    IndustrializedAutomation,
    Prediction,
    ReportPublishing,
}

impl NavigationPillar {
    pub fn all() -> &'static [NavigationPillar] {
        &[
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
        ]
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::MyLibrary => "MY LIBRARY & VAULT",
            Self::AiAgent => "AI AGENT & VIRTUAL FIRM",
            Self::ResearchConcept => "1. RESEARCH & GIS INCEPTION",
            Self::ParametricDED => "2. PARAMETRIC & FREEFORM STUDIO",
            Self::GeometryAssembly => "3. GEOMETRY & CLEAN BIM ASSEMBLY",
            Self::AnalysisSimulation => "GEO-STRUCTURAL ANALYSIS & MULTI-HAZARD",
            Self::TimeScheduling => "4. TIME & 4D PHASED ERECTION",
            Self::CostBudgeting => "5. COST & 5D HEATMAP BUDGETING",
            Self::SustainabilityEnergy => "6. SUSTAINABILITY & 6D HELIODON LCA",
            Self::FacilityManagement => "7. FACILITY & 7D DIGITAL TWIN",
            Self::SafetyHealth => "8. SAFETY & 8D CRANE SWEEP RISK",
            Self::LeanConstruction => "9. LEAN & 9D SITE LOGISTICS",
            Self::IndustrializedAutomation => "10. AUTOMATION & 10D DfMA PREFAB",
            Self::Prediction => "PREDICTION & AI STRUCTURAL AGING",
            Self::ReportPublishing => "REPORT PUBLISHING SUITE (DOCS, SHEETS, SLIDES)",
        }
    }

    pub fn badge(&self) -> &'static str {
        match self {
            Self::MyLibrary => "VAULT",
            Self::AiAgent => "AGENTS",
            Self::ResearchConcept => "1D",
            Self::ParametricDED => "2D",
            Self::GeometryAssembly => "3D",
            Self::AnalysisSimulation => "SIM",
            Self::TimeScheduling => "4D",
            Self::CostBudgeting => "5D",
            Self::SustainabilityEnergy => "6D",
            Self::FacilityManagement => "7D",
            Self::SafetyHealth => "8D",
            Self::LeanConstruction => "9D",
            Self::IndustrializedAutomation => "10D",
            Self::Prediction => "PRED",
            Self::ReportPublishing => "PUB",
        }
    }

    pub fn dimension_tag(&self) -> &'static str {
        self.badge()
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::MyLibrary => "📁",
            Self::AiAgent => "🤖",
            Self::ResearchConcept => "💡",
            Self::ParametricDED => "📐",
            Self::GeometryAssembly => "🏗️",
            Self::AnalysisSimulation => "⚡",
            Self::TimeScheduling => "⏱️",
            Self::CostBudgeting => "💰",
            Self::SustainabilityEnergy => "🌱",
            Self::FacilityManagement => "🏢",
            Self::SafetyHealth => "🛡️",
            Self::LeanConstruction => "♻️",
            Self::IndustrializedAutomation => "🤖",
            Self::Prediction => "🔮",
            Self::ReportPublishing => "📑",
        }
    }

    pub fn subtitle(&self) -> &'static str {
        match self {
            Self::MyLibrary => "Local Obsidian-style Vault: Project records, parametric component database, and SNI/AISC regulatory specifications.",
            Self::AiAgent => "Autonomous Virtual Engineering Firm: Multi-agent organigram (PM, Senior Drafter, Structural Analyst, QS Specialist) & collaboration board.",
            Self::ResearchConcept => "Interactive GIS map container, site geodetic telemetry HUD (WGS84 / UTM), cadastral zoning, and pusGen seismic hazard spectra.",
            Self::ParametricDED => "AutoCAD-style 2D vector drafting canvas with magnetic snaps + live real-time 3D solid extrusion companion and rebar detailer.",
            Self::GeometryAssembly => "Clean BIM spatial assembly workspace: Multi-level Z-planes, structural grid network (A-B-C, 1-2-3), and component stamping.",
            Self::AnalysisSimulation => "3D Geo-Structural sandbox on GIS terrain: X-Ray subsurface ground slider, soil bearing capacity, FEA solver, SVD kinematic guard, and ARI index.",
            Self::TimeScheduling => "4D Phased erection timeline slider, Critical Path Method (CPM) Gantt schedule, and staged construction simulation.",
            Self::CostBudgeting => "5D 3D Cost Heatmap canvas, dynamic Quantity Take-Off (QTO), and parametric Bill of Quantities (BoQ) financial ledger.",
            Self::SustainabilityEnergy => "6D Solar heliodon sun-path dome, cast shadow tracer, operational energy demand, and embodied carbon gradient (A1–A3).",
            Self::FacilityManagement => "7D Digital Twin lifecycle management: Spatial MEP equipment asset tags, IoT telemetry pins, and maintenance lifecycle logs.",
            Self::SafetyHealth => "8D Tower crane working sweep envelope, radius hook capacity checks, open slab edge fall hazard boundary guards.",
            Self::LeanConstruction => "9D Material staging and laydown zones, Just-In-Time (JIT) delivery logistics, and concrete mixer transit routing.",
            Self::IndustrializedAutomation => "10D Design for Manufacture and Assembly (DfMA): Modular precast pods, road freight transport envelope checks, and crane rigging equilibrium.",
            Self::Prediction => "AI structural stress concentration forecasts, non-linear dynamic drift anomalies, progressive collapse risk, and serviceability aging.",
            Self::ReportPublishing => "Living Publishing Suite: Docs (WYSIWYG DED drawings + title block), Sheets (dynamic engineering formulas & BoQ), and Slides (executive deck with 3D viewports).",
        }
    }

    pub fn clean_name(&self) -> &'static str {
        match self {
            Self::MyLibrary => "My Library",
            Self::AiAgent => "AI Agent",
            Self::ResearchConcept => "Research",
            Self::ParametricDED => "Parametric",
            Self::GeometryAssembly => "Geometry",
            Self::AnalysisSimulation => "Analysis",
            Self::TimeScheduling => "Time & Scheduling",
            Self::CostBudgeting => "Cost & Budgeting",
            Self::SustainabilityEnergy => "Sustainability & Energy",
            Self::FacilityManagement => "Facility Management",
            Self::SafetyHealth => "Safety & Health",
            Self::LeanConstruction => "Lean Construction",
            Self::IndustrializedAutomation => "Industrialized & Automation",
            Self::Prediction => "Prediction",
            Self::ReportPublishing => "Report",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectStatus {
    Designed,
    UnderReview,
    Approved,
    Archived,
    Safe,
    ReviewNeeded,
}

impl ProjectStatus {
    pub fn badge_label(&self) -> &'static str {
        match self {
            Self::Designed => "DESIGNED",
            Self::UnderReview => "UNDER REVIEW",
            Self::Approved => "APPROVED",
            Self::Archived => "ARCHIVED",
            Self::Safe => "SAFE (ARI ≥ 0.70)",
            Self::ReviewNeeded => "REVIEW NEEDED (ARI < 0.70)",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProjectItem {
    pub id: String,
    pub title: String,
    pub category: String,
    pub description: String,
    pub elements_count: usize,
    pub nodes_count: usize,
    pub min_ari: f64,
    pub max_deflection_mm: f64,
    pub status: ProjectStatus,
    pub last_modified: String,
    pub is_favorite: bool,
    pub is_featured: bool,
    pub tag: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParametricVariable {
    pub name: String,
    pub expression: String,
    pub evaluated_value: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CadTool {
    Select,
    Line,
    Polyline,
    Arc,
    Circle,
    Rectangle,
    Dimension,
    Trim,
    Offset,
}

impl CadTool {
    pub fn label_and_shortcut(&self) -> (&'static str, &'static str) {
        match self {
            Self::Select => ("Select", "Esc"),
            Self::Line => ("Line", "L"),
            Self::Polyline => ("Polyline", "PL"),
            Self::Arc => ("Arc", "A"),
            Self::Circle => ("Circle", "C"),
            Self::Rectangle => ("Rectangle", "REC"),
            Self::Dimension => ("Dimension", "DIM"),
            Self::Trim => ("Trim", "TR"),
            Self::Offset => ("Offset", "O"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructuralMember {
    pub id: usize,
    pub name: String,
    pub section: String,
    pub force_kn: f64,
    pub capacity_kn: f64,
    pub utilization: f64,
    pub moment_my_knm: f64,
    pub moment_mz_knm: f64,
    pub is_critical: bool,
    pub is_active: bool,
    pub ari_score: f64,
    pub svd_sensitivity: f64,
    #[serde(default)]
    pub ari: f64,
    #[serde(default)]
    pub dcr_interaction: f64,
    #[serde(default)]
    pub shear_vy_kn: f64,
    #[serde(default)]
    pub svd_margin: f64,
    #[serde(default)]
    pub ci: f64,
    #[serde(default)]
    pub rr: f64,
}

impl Default for StructuralMember {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            section: "W14x90".to_string(),
            force_kn: 0.0,
            capacity_kn: 100.0,
            utilization: 0.0,
            moment_my_knm: 0.0,
            moment_mz_knm: 0.0,
            is_critical: false,
            is_active: true,
            ari_score: 1.0,
            svd_sensitivity: 0.0,
            ari: 1.0,
            dcr_interaction: 0.0,
            shear_vy_kn: 0.0,
            svd_margin: 1.0,
            ci: 0.0,
            rr: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub is_user: bool,
    pub message: String,
    pub time: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SidebarTab {
    Properties,
    ProjectBrowser,
    ClashMatrix,
    ModelAudit,
    BcfIssues,
    Geometry,
    Modify,
    Layers,
    Sections,
    Materials,
    Loads,
    Settings,
}

impl SidebarTab {
    pub fn icon_and_name(&self) -> (&'static str, &'static str) {
        match self {
            Self::Properties => ("📋", "Properties & IFC Psets"),
            Self::ProjectBrowser => ("📁", "Project Browser & SBS"),
            Self::ClashMatrix => ("⚔️", "Clash Detection Matrix"),
            Self::ModelAudit => ("🛡️", "Solibri BIM Audit"),
            Self::BcfIssues => ("💬", "BCF 3.0 Issues"),
            Self::Geometry => ("📐", "Geometry & Wizards"),
            Self::Modify => ("✏️", "Modify & Reinforce"),
            Self::Layers => ("📑", "Layers & Worksets"),
            Self::Sections => ("📏", "Sections"),
            Self::Materials => ("🧱", "Materials"),
            Self::Loads => ("⬇️", "Loads"),
            Self::Settings => ("⚙️", "Settings"),
        }
    }
}

/// A stored parametric structural component in the local database library
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseComponent {
    pub id: String,
    pub name: String,
    pub category: String, // e.g. "Truss", "Portal Frame", "Space Frame"
    pub description: String,
    pub span_m: f64,
    pub height_m: f64,
    pub divisions: usize,
    pub default_section: String,
    pub driving_formula: String, // e.g. "H = L / 7.5"
    pub elements_count: usize,
    pub nodes_count: usize,
    pub est_weight_ton: f64,
    pub min_system_ari: f64,
    pub created_at: String,
    #[serde(default)]
    pub variables: Vec<ParametricVariable>,
}

impl Default for DatabaseComponent {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            category: String::new(),
            description: String::new(),
            span_m: 24.0,
            height_m: 3.2,
            divisions: 6,
            default_section: "W14x90".to_string(),
            driving_formula: "H = L / 7.5".to_string(),
            elements_count: 25,
            nodes_count: 14,
            est_weight_ton: 12.8,
            min_system_ari: 0.942,
            created_at: "2026-10-30".to_string(),
            variables: Vec::new(),
        }
    }
}

impl DatabaseComponent {
    pub fn warren_truss_default() -> Self {
        Self {
            id: "comp_warren_24m".to_string(),
            name: "Standard Warren Girder (24m)".to_string(),
            category: "Steel Truss".to_string(),
            description: "24m modular Warren steel bridge truss with diagonal braces and top chord loads.".to_string(),
            span_m: 24.0,
            height_m: 3.2,
            divisions: 6,
            default_section: "W14x90".to_string(),
            driving_formula: "H = L / 7.5".to_string(),
            elements_count: 25,
            nodes_count: 14,
            est_weight_ton: 12.8,
            min_system_ari: 0.942,
            created_at: "2026-10-30".to_string(),
            variables: Vec::new(),
        }
    }

    pub fn pratt_truss_default() -> Self {
        Self {
            id: "comp_pratt_30m".to_string(),
            name: "Industrial Pratt Roof Truss (30m)".to_string(),
            category: "Roof Structure".to_string(),
            description: "30m heavy industrial pitched roof truss with vertical tension and diagonal compression ties.".to_string(),
            span_m: 30.0,
            height_m: 4.5,
            divisions: 8,
            default_section: "W12x65".to_string(),
            driving_formula: "H = L / 6.67".to_string(),
            elements_count: 33,
            nodes_count: 18,
            est_weight_ton: 18.4,
            min_system_ari: 0.915,
            created_at: "2026-10-30".to_string(),
            variables: Vec::new(),
        }
    }

    pub fn diagrid_module_default() -> Self {
        Self {
            id: "comp_diagrid_48m".to_string(),
            name: "High-Rise Diagrid Perimeter Module".to_string(),
            category: "Space Frame".to_string(),
            description: "Modular triangular interlocking diagrid system for tall building wind/seismic shear resistance.".to_string(),
            span_m: 18.0,
            height_m: 12.0,
            divisions: 4,
            default_section: "HSS16x16x0.5".to_string(),
            driving_formula: "θ = 65° (Optimum Diagrid Angle)".to_string(),
            elements_count: 42,
            nodes_count: 20,
            est_weight_ton: 26.5,
            min_system_ari: 0.968,
            created_at: "2026-10-30".to_string(),
            variables: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Phase 27: Master BIM Superstation Models (Family Catalog, Dynamo Nodes, Sheets)
// ---------------------------------------------------------------------------

/// Standardized BIM Component Family for Global Library (UniFormat & MasterFormat)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BimFamilyItem {
    pub id: String,
    pub name: String,
    pub category: String,
    pub classification: String,
    pub ifc_entity: String,
    pub lod: String,
    pub material: String,
    pub dimensions_desc: String,
    pub embodied_carbon_kg: f32,
    pub fire_rating_hr: f32,
    pub type_params: Vec<(String, String)>,
    pub instance_params: Vec<(String, String)>,
}

impl BimFamilyItem {
    pub fn default_catalog() -> Vec<Self> {
        vec![
            Self {
                id: "fam_w14x90".to_string(),
                name: "AISC W14x90 Wide Flange Steel Girder".to_string(),
                category: "Structural Framing".to_string(),
                classification: "MasterFormat 05 12 00 / UniFormat B1010.10".to_string(),
                ifc_entity: "IfcBeam".to_string(),
                lod: "LOD 350".to_string(),
                material: "ASTM A992 Gr 50 Steel".to_string(),
                dimensions_desc: "d=356mm, bf=368mm, tf=18.0mm, tw=11.2mm".to_string(),
                embodied_carbon_kg: 134.2,
                fire_rating_hr: 2.0,
                type_params: vec![
                    ("Section Depth (d)".to_string(), "356 mm".to_string()),
                    ("Flange Width (bf)".to_string(), "368 mm".to_string()),
                    ("Elastic Modulus (Sx)".to_string(), "2340 cm³".to_string()),
                    ("Yield Strength (Fy)".to_string(), "345 MPa".to_string()),
                ],
                instance_params: vec![
                    ("Structural Mark".to_string(), "B-101".to_string()),
                    ("Top Elevation".to_string(), "+4.000 m".to_string()),
                    ("Camber".to_string(), "15 mm".to_string()),
                    ("Phase Created".to_string(), "Phase 1 - Superstructure".to_string()),
                ],
            },
            Self {
                id: "fam_rc_col_600".to_string(),
                name: "Cast-in-Place RC Square Column 600x600".to_string(),
                category: "Columns".to_string(),
                classification: "MasterFormat 03 30 00 / UniFormat B1010.20".to_string(),
                ifc_entity: "IfcColumn".to_string(),
                lod: "LOD 350".to_string(),
                material: "Concrete C40/50 + Rebar D25".to_string(),
                dimensions_desc: "600 mm x 600 mm (12-D25 Main + D10@100 Ties)".to_string(),
                embodied_carbon_kg: 285.6,
                fire_rating_hr: 3.0,
                type_params: vec![
                    ("Width / Depth".to_string(), "600 x 600 mm".to_string()),
                    ("Compressive Strength f'c".to_string(), "40 MPa".to_string()),
                    ("Rebar Longitudinal Ratio".to_string(), "2.45%".to_string()),
                    ("Cover Depth".to_string(), "40 mm".to_string()),
                ],
                instance_params: vec![
                    ("Column Tag".to_string(), "C-1".to_string()),
                    ("Base Level".to_string(), "Level 0 GF".to_string()),
                    ("Top Level".to_string(), "Level 1 (+4.00m)".to_string()),
                    ("Slenderness kl/r".to_string(), "24.2 (Short Column)".to_string()),
                ],
            },
            Self {
                id: "fam_curtain_wall_mullion".to_string(),
                name: "Double-Glazed Unitized Curtain Wall Facade".to_string(),
                category: "Curtain Wall".to_string(),
                classification: "MasterFormat 08 44 00 / UniFormat B2020".to_string(),
                ifc_entity: "IfcCurtainWall".to_string(),
                lod: "LOD 300".to_string(),
                material: "Extruded Aluminium 6063-T6 + Low-E Glass".to_string(),
                dimensions_desc: "1500 mm module width x 4000 mm story height".to_string(),
                embodied_carbon_kg: 88.4,
                fire_rating_hr: 1.0,
                type_params: vec![
                    ("U-Value Thermal".to_string(), "1.25 W/m²K".to_string()),
                    ("SHGC Solar Heat Gain".to_string(), "0.32".to_string()),
                    ("Acoustic Rating (STC)".to_string(), "42 dB".to_string()),
                    ("Mullion Profile Depth".to_string(), "180 mm".to_string()),
                ],
                instance_params: vec![
                    ("Facade Elevation".to_string(), "South Elevation Grid 1-4".to_string()),
                    ("Wind Pressure Design".to_string(), "2.40 kPa (ASCE 7-22)".to_string()),
                    ("Glass Type".to_string(), "6mm Low-E + 12mm Argon + 6mm Clr".to_string()),
                ],
            },
            Self {
                id: "fam_hvac_ahu_rooftop".to_string(),
                name: "Variable Air Volume Rooftop AHU Package".to_string(),
                category: "MEP Equipment".to_string(),
                classification: "MasterFormat 23 74 13 / UniFormat D3030".to_string(),
                ifc_entity: "IfcAirTerminal".to_string(),
                lod: "LOD 350".to_string(),
                material: "Galvanized Steel Casing + EC Motors".to_string(),
                dimensions_desc: "3200 mm L x 1800 mm W x 1650 mm H".to_string(),
                embodied_carbon_kg: 420.0,
                fire_rating_hr: 0.0,
                type_params: vec![
                    ("Nominal Airflow".to_string(), "12,000 CFM (5,660 L/s)".to_string()),
                    ("Cooling Capacity".to_string(), "350 kW (100 Tons)".to_string()),
                    ("External Static Pressure".to_string(), "500 Pa".to_string()),
                    ("Refrigerant".to_string(), "R-454B (Low GWP)".to_string()),
                ],
                instance_params: vec![
                    ("Equipment Tag".to_string(), "AHU-ROOF-01".to_string()),
                    ("Location".to_string(), "Rooftop Plant Deck Grid B-2".to_string()),
                    ("BMS Address".to_string(), "BACnet/IP ID 40012".to_string()),
                ],
            },
            Self {
                id: "fam_pt_i_girder_30m".to_string(),
                name: "PCI Type IV Prestressed I-Girder 30m Span".to_string(),
                category: "Civil Infrastructure".to_string(),
                classification: "MasterFormat 34 71 00 / UniFormat G2010".to_string(),
                ifc_entity: "IfcBeam".to_string(),
                lod: "LOD 350".to_string(),
                material: "Precast Concrete 55 MPa + 0.6\" Strands".to_string(),
                dimensions_desc: "Height 1370mm, Length 30.0m, Web 200mm".to_string(),
                embodied_carbon_kg: 680.0,
                fire_rating_hr: 3.0,
                type_params: vec![
                    ("Span Length".to_string(), "30.0 m".to_string()),
                    ("Girder Depth".to_string(), "1370 mm".to_string()),
                    ("PT Tendons".to_string(), "34 x 15.2mm Low-Relaxation Strands".to_string()),
                    ("Prestress Force Pjack".to_string(), "6,800 kN".to_string()),
                ],
                instance_params: vec![
                    ("Span Mark".to_string(), "GIRDER-SPAN-02-G1".to_string()),
                    ("Casting Date".to_string(), "2026-10-15".to_string()),
                    ("Measured Camber".to_string(), "42 mm".to_string()),
                ],
            },
            Self {
                id: "fam_box_culvert".to_string(),
                name: "Precast Stormwater Box Culvert 3.0x2.5m".to_string(),
                category: "Civil Infrastructure".to_string(),
                classification: "MasterFormat 33 42 16 / UniFormat G3020".to_string(),
                ifc_entity: "IfcDistributionElement".to_string(),
                lod: "LOD 300".to_string(),
                material: "Reinforced Concrete C35/45".to_string(),
                dimensions_desc: "3000 mm span x 2500 mm rise x 2000 mm lay length".to_string(),
                embodied_carbon_kg: 510.0,
                fire_rating_hr: 0.0,
                type_params: vec![
                    ("Hydraulic Capacity".to_string(), "18.5 m³/s (Manning n=0.013)".to_string()),
                    ("Wall Thickness".to_string(), "250 mm".to_string()),
                    ("Live Load Rating".to_string(), "AASHTO HL-93 Highway Loading".to_string()),
                ],
                instance_params: vec![
                    ("Station Invert".to_string(), "Sta 0+450 Invert EL -1.25m".to_string()),
                    ("Bedding Condition".to_string(), "Class B Granular Cradle".to_string()),
                ],
            },
        ]
    }
}

/// Visual Node Programming Category for Dynamo/Grasshopper Studio
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisualNodeCategory {
    Input,
    MathLogic,
    GeometryGen,
    BimBake,
}

impl VisualNodeCategory {
    pub fn badge_color_and_name(&self) -> (&'static str, (u8, u8, u8)) {
        match self {
            Self::Input => ("Input Parameter", (52, 199, 89)),
            Self::MathLogic => ("Math & Transform", (255, 149, 0)),
            Self::GeometryGen => ("3D Solid Generator", (10, 132, 255)),
            Self::BimBake => ("BIM Scene Baker", (175, 82, 222)),
        }
    }
}

/// A computational node on the visual programming graph canvas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualGraphNode {
    pub id: usize,
    pub title: String,
    pub category: VisualNodeCategory,
    pub pos: [f32; 2],
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub value_display: String,
    pub is_active: bool,
}

/// Connection wire linking two sockets on the visual programming canvas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualGraphWire {
    pub from_node: usize,
    pub from_socket: usize,
    pub to_node: usize,
    pub to_socket: usize,
}

/// Full Visual Dynamo/Grasshopper computational graph model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualNodeGraph {
    pub nodes: Vec<VisualGraphNode>,
    pub wires: Vec<VisualGraphWire>,
}

impl Default for VisualNodeGraph {
    fn default() -> Self {
        Self {
            nodes: vec![
                VisualGraphNode {
                    id: 1,
                    title: "Slider: Span L (m)".to_string(),
                    category: VisualNodeCategory::Input,
                    pos: [40.0, 60.0],
                    inputs: vec![],
                    outputs: vec!["Length (m)".to_string()],
                    value_display: "24.0".to_string(),
                    is_active: true,
                },
                VisualGraphNode {
                    id: 2,
                    title: "Slider: Bay Width B (m)".to_string(),
                    category: VisualNodeCategory::Input,
                    pos: [40.0, 190.0],
                    inputs: vec![],
                    outputs: vec!["Width (m)".to_string()],
                    value_display: "6.0".to_string(),
                    is_active: true,
                },
                VisualGraphNode {
                    id: 3,
                    title: "Math: Formula H = L / 8".to_string(),
                    category: VisualNodeCategory::MathLogic,
                    pos: [290.0, 60.0],
                    inputs: vec!["L".to_string()],
                    outputs: vec!["Depth H".to_string()],
                    value_display: "3.00 m".to_string(),
                    is_active: true,
                },
                VisualGraphNode {
                    id: 4,
                    title: "Truss Octet Generator".to_string(),
                    category: VisualNodeCategory::GeometryGen,
                    pos: [520.0, 90.0],
                    inputs: vec!["L".to_string(), "B".to_string(), "H".to_string()],
                    outputs: vec!["Mesh 3D".to_string(), "Elem Count".to_string()],
                    value_display: "48 Elements".to_string(),
                    is_active: true,
                },
                VisualGraphNode {
                    id: 5,
                    title: "Bake to BIM 3D Scene".to_string(),
                    category: VisualNodeCategory::BimBake,
                    pos: [790.0, 110.0],
                    inputs: vec!["Geometry".to_string()],
                    outputs: vec!["IfcExport".to_string()],
                    value_display: "Live Synced".to_string(),
                    is_active: true,
                },
            ],
            wires: vec![
                VisualGraphWire { from_node: 1, from_socket: 0, to_node: 3, to_socket: 0 },
                VisualGraphWire { from_node: 1, from_socket: 0, to_node: 4, to_socket: 0 },
                VisualGraphWire { from_node: 2, from_socket: 0, to_node: 4, to_socket: 1 },
                VisualGraphWire { from_node: 3, from_socket: 0, to_node: 4, to_socket: 2 },
                VisualGraphWire { from_node: 4, from_socket: 0, to_node: 5, to_socket: 0 },
            ],
        }
    }
}

/// A Drawing Sheet Item for Production Working Drawings (DED)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawingSheetItem {
    pub id: String,
    pub sheet_number: String,
    pub sheet_title: String,
    pub paper_size: String,
    pub scale: String,
    pub discipline: String,
    pub engineer_stamp: String,
    pub revision: String,
    pub viewports: Vec<String>,
}

impl DrawingSheetItem {
    pub fn default_set() -> Vec<Self> {
        vec![
            Self {
                id: "sheet_s101".to_string(),
                sheet_number: "S-101".to_string(),
                sheet_title: "FOUNDATION & GROUND FLOOR FRAMING PLAN".to_string(),
                paper_size: "ISO A1 (841 x 594 mm)".to_string(),
                scale: "1 : 100".to_string(),
                discipline: "Structural (STR)".to_string(),
                engineer_stamp: "PE Structural License #49120-ST (Digital Sealed)".to_string(),
                revision: "REV-0 (Issued for Construction)".to_string(),
                viewports: vec![
                    "Main Framing Grid A-D & 1-4 (Scale 1:100)".to_string(),
                    "Isolated Footing Schedule Table".to_string(),
                    "Typical RC Column Detail Callout 1:20".to_string(),
                ],
            },
            Self {
                id: "sheet_s201".to_string(),
                sheet_number: "S-201".to_string(),
                sheet_title: "LEVEL 2 TYPICAL FLOOR BEAM & SLAB REBAR".to_string(),
                paper_size: "ISO A1 (841 x 594 mm)".to_string(),
                scale: "1 : 100".to_string(),
                discipline: "Structural (STR)".to_string(),
                engineer_stamp: "PE Structural License #49120-ST (Digital Sealed)".to_string(),
                revision: "REV-B (Coordination Approval)".to_string(),
                viewports: vec![
                    "Floor Slab Two-Way Mesh Detailing".to_string(),
                    "W14x90 Moment Frame Connection Detail 1:10".to_string(),
                    "Bar Bending Schedule (BBS) Table".to_string(),
                ],
            },
            Self {
                id: "sheet_a101".to_string(),
                sheet_number: "A-101".to_string(),
                sheet_title: "ARCHITECTURAL FLOOR PLAN & ROOM SCHEDULE".to_string(),
                paper_size: "ISO A1 (841 x 594 mm)".to_string(),
                scale: "1 : 100".to_string(),
                discipline: "Architectural (ARC)".to_string(),
                engineer_stamp: "Licensed Architect #IAI-88241".to_string(),
                revision: "REV-1 (Client Signoff)".to_string(),
                viewports: vec![
                    "Ground Floor Partition & Door Layout".to_string(),
                    "Room Boundary Area Tag Matrix".to_string(),
                    "Curtain Wall Mullion Elevation 1:50".to_string(),
                ],
            },
            Self {
                id: "sheet_m301".to_string(),
                sheet_number: "M-301".to_string(),
                sheet_title: "HVAC DUCTING & FIRE SPRINKLER DISTRIBUTION".to_string(),
                paper_size: "ISO A1 (841 x 594 mm)".to_string(),
                scale: "1 : 100".to_string(),
                discipline: "Mechanical & Fire (MEP)".to_string(),
                engineer_stamp: "Professional Mechanical Engineer #MEP-3310".to_string(),
                revision: "REV-0 (Issued for Construction)".to_string(),
                viewports: vec![
                    "Level 1 Supply/Return Air Run (Scale 1:100)".to_string(),
                    "VAV Terminal Box Wiring & Riser Diagram".to_string(),
                    "Automatic Sprinkler Hydraulic Calculation Summary".to_string(),
                ],
            },
        ]
    }
}

// ============================================================================
// Phase 28: 4D-7D BIM Superstation Data Models
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstructionSimulationPhase {
    pub id: String,
    pub name: String,
    pub day_start: u32,
    pub day_end: u32,
    pub planned_progress_pct: f32,
    pub actual_progress_pct: f32,
    pub manpower_count: u32,
    pub equipment_count: u32,
    pub concrete_volume_m3: f32,
    pub color_rgb: (u8, u8, u8),
    pub wbs_code: String,
    pub is_critical: bool,
}

impl ConstructionSimulationPhase {
    pub fn default_phases() -> Vec<Self> {
        vec![
            Self {
                id: "ph_sub".to_string(),
                name: "1. Substructure Piling & Deep Foundation".to_string(),
                day_start: 1,
                day_end: 45,
                planned_progress_pct: 100.0,
                actual_progress_pct: 100.0,
                manpower_count: 42,
                equipment_count: 6,
                concrete_volume_m3: 1450.0,
                color_rgb: (160, 100, 45),
                wbs_code: "02.40.00".to_string(),
                is_critical: true,
            },
            Self {
                id: "ph_raft".to_string(),
                name: "2. Raft Slab & Perimeter Retaining Walls".to_string(),
                day_start: 46,
                day_end: 90,
                planned_progress_pct: 100.0,
                actual_progress_pct: 96.0,
                manpower_count: 58,
                equipment_count: 8,
                concrete_volume_m3: 2200.0,
                color_rgb: (120, 130, 145),
                wbs_code: "03.30.00".to_string(),
                is_critical: true,
            },
            Self {
                id: "ph_steel".to_string(),
                name: "3. Structural Steel Erection L1-L15 & Composite Decks".to_string(),
                day_start: 91,
                day_end: 195,
                planned_progress_pct: 78.0,
                actual_progress_pct: 82.5,
                manpower_count: 85,
                equipment_count: 12,
                concrete_volume_m3: 1850.0,
                color_rgb: (0, 122, 255),
                wbs_code: "05.12.00".to_string(),
                is_critical: true,
            },
            Self {
                id: "ph_facade".to_string(),
                name: "4. Unitized Curtain Wall & Building Envelope".to_string(),
                day_start: 180,
                day_end: 270,
                planned_progress_pct: 45.0,
                actual_progress_pct: 42.0,
                manpower_count: 64,
                equipment_count: 5,
                concrete_volume_m3: 0.0,
                color_rgb: (52, 199, 89),
                wbs_code: "08.44.00".to_string(),
                is_critical: false,
            },
            Self {
                id: "ph_mep".to_string(),
                name: "5. MEP Core Risers, HVAC & Fire Suppression".to_string(),
                day_start: 210,
                day_end: 320,
                planned_progress_pct: 30.0,
                actual_progress_pct: 28.0,
                manpower_count: 72,
                equipment_count: 4,
                concrete_volume_m3: 0.0,
                color_rgb: (255, 149, 0),
                wbs_code: "23.00.00".to_string(),
                is_critical: true,
            },
            Self {
                id: "ph_com".to_string(),
                name: "6. Integrated Testing, Commissioning & Handover".to_string(),
                day_start: 321,
                day_end: 360,
                planned_progress_pct: 5.0,
                actual_progress_pct: 0.0,
                manpower_count: 24,
                equipment_count: 2,
                concrete_volume_m3: 0.0,
                color_rgb: (175, 82, 222),
                wbs_code: "01.91.00".to_string(),
                is_critical: true,
            },
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrencyUnit {
    Idr,
    Usd,
    Eur,
    Sgd,
}

impl CurrencyUnit {
    pub fn all() -> &'static [CurrencyUnit] {
        &[CurrencyUnit::Idr, CurrencyUnit::Usd, CurrencyUnit::Eur, CurrencyUnit::Sgd]
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::Idr => "IDR",
            Self::Usd => "USD",
            Self::Eur => "EUR",
            Self::Sgd => "SGD",
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Idr => "Rp",
            Self::Usd => "$",
            Self::Eur => "€",
            Self::Sgd => "S$",
        }
    }

    pub fn rate_from_idr(&self) -> f64 {
        match self {
            Self::Idr => 1.0,
            Self::Usd => 1.0 / 16_200.0,
            Self::Eur => 1.0 / 17_400.0,
            Self::Sgd => 1.0 / 12_100.0,
        }
    }

    pub fn format_value(&self, idr_val: f64) -> String {
        let converted = idr_val * self.rate_from_idr();
        match self {
            Self::Idr => {
                let rounded = idr_val.round() as i64;
                let s = rounded.abs().to_string();
                let mut res = String::new();
                let mut c = 0;
                for ch in s.chars().rev() {
                    if c > 0 && c % 3 == 0 { res.push('.'); }
                    res.push(ch);
                    c += 1;
                }
                let rev: String = res.chars().rev().collect();
                if rounded < 0 { format!("-Rp {}", rev) } else { format!("Rp {}", rev) }
            }
            Self::Usd => {
                let int_part = converted.round() as i64;
                let s = int_part.abs().to_string();
                let mut res = String::new();
                let mut c = 0;
                for ch in s.chars().rev() {
                    if c > 0 && c % 3 == 0 { res.push(','); }
                    res.push(ch);
                    c += 1;
                }
                let rev: String = res.chars().rev().collect();
                if int_part < 0 { format!("-${}", rev) } else { format!("${}", rev) }
            }
            Self::Eur => {
                let int_part = converted.round() as i64;
                let s = int_part.abs().to_string();
                let mut res = String::new();
                let mut c = 0;
                for ch in s.chars().rev() {
                    if c > 0 && c % 3 == 0 { res.push('.'); }
                    res.push(ch);
                    c += 1;
                }
                let rev: String = res.chars().rev().collect();
                if int_part < 0 { format!("-€{}", rev) } else { format!("€{}", rev) }
            }
            Self::Sgd => {
                let int_part = converted.round() as i64;
                let s = int_part.abs().to_string();
                let mut res = String::new();
                let mut c = 0;
                for ch in s.chars().rev() {
                    if c > 0 && c % 3 == 0 { res.push(','); }
                    res.push(ch);
                    c += 1;
                }
                let rev: String = res.chars().rev().collect();
                if int_part < 0 { format!("-S${}", rev) } else { format!("S${}", rev) }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WbsCostItem {
    pub wbs_code: String,
    pub description: String,
    pub division: String,
    pub unit: String,
    pub quantity: f64,
    pub unit_price_idr: f64,
    pub total_cost_idr: f64,
    pub progress_pct: f32,
}

impl WbsCostItem {
    pub fn default_catalog() -> Vec<Self> {
        vec![
            Self {
                wbs_code: "02.41.16".to_string(),
                description: "Deep Bored Piling D=1000mm (L=32m)".to_string(),
                division: "02 Substructure".to_string(),
                unit: "m".to_string(),
                quantity: 4800.0,
                unit_price_idr: 2_850_000.0,
                total_cost_idr: 13_680_000_000.0,
                progress_pct: 100.0,
            },
            Self {
                wbs_code: "03.31.00".to_string(),
                description: "Structural Concrete fc' 40 MPa (Foundation & Columns)".to_string(),
                division: "03 Cast-in-Place Concrete".to_string(),
                unit: "m3".to_string(),
                quantity: 5500.0,
                unit_price_idr: 1_450_000.0,
                total_cost_idr: 7_975_000_000.0,
                progress_pct: 88.0,
            },
            Self {
                wbs_code: "03.21.00".to_string(),
                description: "Deformed High-Strength Rebar BJTS 420B".to_string(),
                division: "03 Reinforcing Steel".to_string(),
                unit: "kg".to_string(),
                quantity: 680_000.0,
                unit_price_idr: 14_800.0,
                total_cost_idr: 10_064_000_000.0,
                progress_pct: 85.0,
            },
            Self {
                wbs_code: "05.12.23".to_string(),
                description: "Structural Steel Framing ASTM A992 (W-Shapes & Trusses)".to_string(),
                division: "05 Structural Metals".to_string(),
                unit: "ton".to_string(),
                quantity: 940.0,
                unit_price_idr: 28_500_000.0,
                total_cost_idr: 26_790_000_000.0,
                progress_pct: 76.0,
            },
            Self {
                wbs_code: "08.44.13".to_string(),
                description: "Glazed Aluminum Curtain Wall System (Double Low-E)".to_string(),
                division: "08 Openings & Glazing".to_string(),
                unit: "m2".to_string(),
                quantity: 7200.0,
                unit_price_idr: 2_600_000.0,
                total_cost_idr: 18_720_000_000.0,
                progress_pct: 42.0,
            },
            Self {
                wbs_code: "23.05.00".to_string(),
                description: "Common Work Results for HVAC & Central Chillers".to_string(),
                division: "23 Mechanical & HVAC".to_string(),
                unit: "lot".to_string(),
                quantity: 1.0,
                unit_price_idr: 15_200_000_000.0,
                total_cost_idr: 15_200_000_000.0,
                progress_pct: 25.0,
            },
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LcaStageEmission {
    pub stage_code: String,
    pub stage_name: String,
    pub emissions_t_co2e: f64,
    pub pct_of_total: f32,
    pub description: String,
    pub color_rgb: (u8, u8, u8),
}

impl LcaStageEmission {
    pub fn default_stages() -> Vec<Self> {
        vec![
            Self {
                stage_code: "A1-A3".to_string(),
                stage_name: "Raw Material & Product Manufacturing".to_string(),
                emissions_t_co2e: 1420.5,
                pct_of_total: 54.2,
                description: "Cradle-to-gate extraction of iron ore, blast furnace smelting, Portland cement clinker.".to_string(),
                color_rgb: (235, 75, 55),
            },
            Self {
                stage_code: "A4-A5".to_string(),
                stage_name: "Transport & Construction Process".to_string(),
                emissions_t_co2e: 312.4,
                pct_of_total: 11.9,
                description: "Fleet freight logistics, crane diesel consumption, batch plant site energy.".to_string(),
                color_rgb: (255, 149, 0),
            },
            Self {
                stage_code: "B1-B7".to_string(),
                stage_name: "Operational Energy & Building Use".to_string(),
                emissions_t_co2e: 648.0,
                pct_of_total: 24.7,
                description: "50-year structural serviceability, cathodic protection power, maintenance coating.".to_string(),
                color_rgb: (0, 122, 255),
            },
            Self {
                stage_code: "C1-C4".to_string(),
                stage_name: "Deconstruction & Waste Processing".to_string(),
                emissions_t_co2e: 140.2,
                pct_of_total: 5.3,
                description: "Controlled demolition, concrete crushing sorting, landfill transport.".to_string(),
                color_rgb: (142, 142, 147),
            },
            Self {
                stage_code: "Module D".to_string(),
                stage_name: "Beyond Lifecycle Circular Reuse Offset".to_string(),
                emissions_t_co2e: -480.0,
                pct_of_total: 3.9,
                description: "Net carbon credit from 95% structural steel electric arc furnace remelting recycling.".to_string(),
                color_rgb: (52, 199, 89),
            },
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GreenBuildingCredit {
    pub standard: String,
    pub category: String,
    pub credit_title: String,
    pub points_achieved: u32,
    pub points_max: u32,
    pub status: String,
}

impl GreenBuildingCredit {
    pub fn default_credits() -> Vec<Self> {
        vec![
            Self {
                standard: "GREENSHIP (GBCI)".to_string(),
                category: "Material Resource & Cycle (MRC)".to_string(),
                credit_title: "MRC 1: Recycled Content in Structural Steel (EAF > 75%)".to_string(),
                points_achieved: 3,
                points_max: 3,
                status: "Compliant".to_string(),
            },
            Self {
                standard: "GREENSHIP (GBCI)".to_string(),
                category: "Energy Efficiency & Conservation (EEC)".to_string(),
                credit_title: "EEC 1: Overall Thermal Transfer Value (OTTV < 35 W/m2)".to_string(),
                points_achieved: 14,
                points_max: 15,
                status: "Compliant".to_string(),
            },
            Self {
                standard: "LEED v4.1 BD+C".to_string(),
                category: "Materials and Resources (MR)".to_string(),
                credit_title: "Building Life-Cycle Impact Reduction (Option 4 Whole-Building LCA)".to_string(),
                points_achieved: 4,
                points_max: 4,
                status: "Compliant".to_string(),
            },
            Self {
                standard: "LEED v4.1 BD+C".to_string(),
                category: "Energy and Atmosphere (EA)".to_string(),
                credit_title: "Optimize Energy Performance (32% Improvement over ASHRAE 90.1)".to_string(),
                points_achieved: 16,
                points_max: 18,
                status: "Under Review".to_string(),
            },
            Self {
                standard: "BREEAM International".to_string(),
                category: "Mat 01 Life Cycle Impacts".to_string(),
                credit_title: "Environmental Product Declaration (ISO 14025 Type III EPD Verified)".to_string(),
                points_achieved: 5,
                points_max: 6,
                status: "Compliant".to_string(),
            },
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShmSensorTelemetry {
    pub sensor_id: String,
    pub sensor_name: String,
    pub location: String,
    pub sensor_type: String,
    pub current_value: f32,
    pub unit: String,
    pub threshold_warning: f32,
    pub threshold_critical: f32,
    pub status_level: String,
    pub sparkline_history: [f32; 8],
}

impl ShmSensorTelemetry {
    pub fn default_sensors() -> Vec<Self> {
        vec![
            Self {
                sensor_id: "SG-L01-C03".to_string(),
                sensor_name: "Fiber-Optic Strain Gauge SG-01".to_string(),
                location: "Grid B-2 Ground Column Base".to_string(),
                sensor_type: "Fiber Bragg Grating (FBG)".to_string(),
                current_value: 342.5,
                unit: "με".to_string(),
                threshold_warning: 600.0,
                threshold_critical: 950.0,
                status_level: "Optimal".to_string(),
                sparkline_history: [330.0, 335.0, 340.0, 338.0, 345.0, 341.0, 343.0, 342.5],
            },
            Self {
                sensor_id: "ACC-ROOF-01".to_string(),
                sensor_name: "Triaxial Accelerometer ACC-01".to_string(),
                location: "Level 15 Roof Core Top".to_string(),
                sensor_type: "MEMS Seismic Sensor".to_string(),
                current_value: 1.48,
                unit: "Hz".to_string(),
                threshold_warning: 1.20,
                threshold_critical: 0.95,
                status_level: "Optimal".to_string(),
                sparkline_history: [1.52, 1.51, 1.50, 1.49, 1.48, 1.49, 1.48, 1.48],
            },
            Self {
                sensor_id: "CM-BASE-04".to_string(),
                sensor_name: "Laser Displacement Crack Meter CM-04".to_string(),
                location: "Basement 2 South Retaining Wall".to_string(),
                sensor_type: "Potentiometric LVDT".to_string(),
                current_value: 0.12,
                unit: "mm".to_string(),
                threshold_warning: 0.30,
                threshold_critical: 0.50,
                status_level: "Optimal".to_string(),
                sparkline_history: [0.08, 0.09, 0.10, 0.10, 0.11, 0.11, 0.12, 0.12],
            },
            Self {
                sensor_id: "TM-CORE-02".to_string(),
                sensor_name: "Biaxial Inclinometer Tiltmeter TM-02".to_string(),
                location: "Level 8 Mid-Tower Shear Core".to_string(),
                sensor_type: "Electrolytic Tilt Sensor".to_string(),
                current_value: 0.018,
                unit: "deg".to_string(),
                threshold_warning: 0.050,
                threshold_critical: 0.100,
                status_level: "Optimal".to_string(),
                sparkline_history: [0.015, 0.016, 0.017, 0.017, 0.018, 0.018, 0.018, 0.018],
            },
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CobieAssetRecord {
    pub sheet_tab: String,
    pub name: String,
    pub created_by: String,
    pub category: String,
    pub space_or_location: String,
    pub serial_or_doc_no: String,
    pub warranty_or_date: String,
    pub condition_index: f32,
}

impl CobieAssetRecord {
    pub fn default_records() -> Vec<Self> {
        vec![
            Self {
                sheet_tab: "Facility".to_string(),
                name: "Menara Sudirman Commercial Tower".to_string(),
                created_by: "EigenGraph BIM Team".to_string(),
                category: "High-Rise Office Complex".to_string(),
                space_or_location: "Kav. 52-53 Sudirman, Jakarta".to_string(),
                serial_or_doc_no: "FAC-JKT-2026-001".to_string(),
                warranty_or_date: "10-Year Structural Defect Liability".to_string(),
                condition_index: 99.2,
            },
            Self {
                sheet_tab: "Floor".to_string(),
                name: "Level 01 Ground Lobby".to_string(),
                created_by: "Architectural Lead".to_string(),
                category: "Commercial Public Realm".to_string(),
                space_or_location: "Elevation +0.000m".to_string(),
                serial_or_doc_no: "FLR-L01".to_string(),
                warranty_or_date: "Commissioned 2026-11-12".to_string(),
                condition_index: 100.0,
            },
            Self {
                sheet_tab: "Component".to_string(),
                name: "W14x132 Transfer Girder TG-01".to_string(),
                created_by: "Structural Engineer of Record".to_string(),
                category: "ASTM A992 Heavy Section".to_string(),
                space_or_location: "Grid C/2-3 Level 2 Ceiling".to_string(),
                serial_or_doc_no: "HEAT-#88392-AISC".to_string(),
                warranty_or_date: "50-Year Design Life".to_string(),
                condition_index: 98.6,
            },
            Self {
                sheet_tab: "Type".to_string(),
                name: "Square RC Column 600x600 C1".to_string(),
                created_by: "BIM Modeler".to_string(),
                category: "C40/50 Self-Consolidating Concrete".to_string(),
                space_or_location: "Ground through L15 Columns".to_string(),
                serial_or_doc_no: "TYP-COL-600".to_string(),
                warranty_or_date: "Tested to SNI 2847:2019".to_string(),
                condition_index: 99.0,
            },
            Self {
                sheet_tab: "Document".to_string(),
                name: "As-Built Structural Certification Dossier".to_string(),
                created_by: "Chief Structural Engineer".to_string(),
                category: "Statutory Approval PBG".to_string(),
                space_or_location: "Electronic Vault / SIMBG".to_string(),
                serial_or_doc_no: "DOC-PBG-2026-881".to_string(),
                warranty_or_date: "Certified 2026-11-12".to_string(),
                condition_index: 100.0,
            },
        ]
    }
}


