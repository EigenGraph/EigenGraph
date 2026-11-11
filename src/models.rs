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
    Geometry,
    Sections,
    Materials,
    Loads,
    Settings,
    Properties,
    ProjectBrowser,
    Modify,
    Layers,
}

impl SidebarTab {
    pub fn icon_and_name(&self) -> (&'static str, &'static str) {
        match self {
            Self::Geometry => ("📐", "Geometry"),
            Self::Sections => ("📏", "Sections"),
            Self::Materials => ("🧱", "Materials"),
            Self::Loads => ("⬇️", "Loads"),
            Self::Settings => ("⚙️", "Settings"),
            Self::Properties => ("📋", "Properties"),
            Self::ProjectBrowser => ("📁", "Browser"),
            Self::Modify => ("✏️", "Modify"),
            Self::Layers => ("📑", "Layers"),
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
