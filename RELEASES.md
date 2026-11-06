# EigenGraph Release Notes & Changelog

All official versioned releases of EigenGraph are cataloged here.

Per Section 6 of `AGENTS.md`, versioning strictly follows:
`v0.0.1` ➔ `v0.0.2` ➔ `v0.0.3` ... ➔ `v0.0.9` ➔ `v0.1.0` ... ➔ `v0.9.9` ➔ `v1.0.0` ... (never skip patch numbers).

---

## 🚀 EigenGraph v0.0.4 — Phase 3: Advanced Seismic & Pushover Analysis (ASCE 41-17 / ATC-40)
**Release Date:** 2026-11-06  
**Git Tag:** `v0.0.4`  
**Target:** Production Engine

EigenGraph v0.0.4 introduces the **Non-Linear Static Pushover Engine & Capacity Spectrum Method (CSM / ATC-40)**, enabling structural engineers to conduct Performance-Based Earthquake Engineering (PBEE) assessments in accordance with ASCE 41-17, FEMA 356, and SNI 1726:2019.

### 🌟 Key Features & Mathematical Formulation

#### 1. Capacity Spectrum Method (CSM / ATC-40) in ADRS Space
- **ADRS Coordinate Transformation**: Converts traditional Pushover curve coordinates $(D_{\text{roof}}, V_b)$ into Acceleration-Displacement Response Spectrum ($ADRS$) spectral coordinates $(S_d, S_a)$:
  $$S_d = \frac{D_{\text{roof}}}{\Gamma_1 \cdot \phi_{\text{roof}, 1}}, \quad S_a = \frac{V_b / W}{\alpha_1}$$
  where $\Gamma_1$ is the first-mode participation factor and $\alpha_1$ is the effective modal mass coefficient.
- **Inelastic Damping Reduction**: Derives equivalent hysteretic viscous damping:
  $$\beta_{\text{eff}} = \beta_0 + \kappa \cdot \frac{2}{\pi} \frac{A_y D_p - D_y A_p}{A_p D_p}$$
  with spectral damping reduction factors $SR_A = \frac{3.21 - 0.68 \ln(\beta_{\text{eff}})}{2.12}$ and $SR_V = \frac{2.31 - 0.41 \ln(\beta_{\text{eff}})}{1.65}$.
- **Performance Point Convergence**: Uses iterative ATC-40 Procedure A to locate the intersecting Performance Point $P^*(S_d^*, S_a^*)$, determining target roof drift $D_t$, target base shear $V_t$, and secant period $T_{\text{eff}}$.

#### 2. Performance-Based Earthquake Engineering (PBEE) Classification
- Evaluates the structural system against ASCE 41-17 Table 9-6 / FEMA 356 performance levels:
  - `ImmediateOccupancy` (IO): Minor micro-yielding, no residual drift, immediate re-entry.
  - `LifeSafety` (LS): Controlled plastification, wide collapse margin under Design Basis Earthquake (DBE).
  - `CollapsePrevention` (CP): Severe structural damage, prevents global collapse under Maximum Considered Earthquake (MCE).
  - `CollapseExceeded`: Demand exceeds inelastic capacity limit, indicating structural collapse risk.
- Tallies plastic hinge performance states (Elastic, IO, LS, CP, Mechanism) across all frame members.

#### 3. Dual Vector Visualization in Desktop UI
- Side-by-side interactive vector plots:
  - **Plot 1**: Base Shear vs Roof Drift capacity curve with target displacement $D_t$ reticle indicator.
  - **Plot 2**: ADRS Capacity-Demand Spectrum showing Elastic 5% Demand (red dashed), Inelastic Reduced Demand (amber), Capacity Spectrum (emerald green), and intersecting Performance Point $P^*(S_d^*, S_a^*)$.
- Dynamic toolbar controls for Lateral Load Patterns (`FirstModeModal`, `UniformMass`, `InvertedTriangular`) and Soil Site Classes (SB, SC, SD, SE).

---

## 🚀 EigenGraph v0.0.3 — Phase 2: Core Upgrade (Shell FEA, P-Delta & IFC4 SAM)
**Release Date:** 2026-11-06  
**Git Tag:** `v0.0.3`  
**Target:** Production Engine

EigenGraph v0.0.3 delivers three critical core upgrades: Mindlin-Reissner 2D shell quad elements for shear walls and slabs, second-order geometric non-linearity ($P-\Delta$ and $P-\delta$) with tangent stiffness formulation, and certified OpenBIM IFC4 / IFC4.3 Structural Analysis Model export.

### 🌟 Key Features & Mathematical Formulation

#### 1. 2D Mindlin-Reissner Flat Shell / Plate Finite Elements (`ShellElement4N`)
- **4-Node Isoparametric Quadrilateral Shell**: Designed for reinforced concrete shear walls (*dinding geser*) and structural diaphragms/slabs.
- **Combined 24-DOF Element**:
  - In-plane membrane behavior ($u, v$ displacements).
  - Out-of-plane Mindlin-Reissner plate bending ($w, \theta_x, \theta_y$ rotations) incorporating transverse shear deformation.
  - Drilling degree of freedom ($\theta_z$) to eliminate numerical singularities when connecting non-coplanar elements.
- **Numerical Integration**: $2 \times 2$ Gauss-Legendre quadrature for exact stiffness evaluation.
- **Automatic Classification**: Classifies vertical elements (`is_shear_wall`) and computes shell surface areas.

#### 2. Second-Order Geometric Non-Linearity $P-\Delta$ & $P-\delta$ Solver (`SecondOrderStabilitySolver`)
- **Geometric Stiffness Matrix $\mathbf{K}_g(\mathbf{P})$**: Accounts for compressive softening and tensile stiffening under axial force vectors $\mathbf{P}$.
- **Tangent Stiffness Formulation**:
  $$\mathbf{K}_t = \mathbf{K}_e - \mathbf{K}_g(\mathbf{P})$$
- **Euler Elastic Critical Buckling Load**:
  $$P_{\text{cr}} = \frac{\pi^2 E I}{(K L)^2}$$
- **AISC 360-16 / 360-22 $B_1$ Moment Amplification Factor**:
  $$B_1 = \frac{C_m}{1 - P_u / P_{e1}} \ge 1.0$$
- **ASCE 7-22 Section 12.8.7 Story Drift Stability Coefficient**:
  $$\theta = \frac{P_x \Delta I_e}{V_x h_{sx} C_d} \le \theta_{\max} = \frac{0.50}{\beta C_d} \le 0.25$$
- **Automated Instability Diagnostics**: Categorizes stability as `StableLinear`, `PDeltaAmplified`, `CriticalBucklingRisk`, or `DynamicPDeltaInstability`.

#### 3. OpenBIM IFC4 / IFC4.3 Structural Analysis Model Exporter (`IfcSamExporter`)
- **ISO 16739-1 Compliant Output**: Zero-dependency `.ifc` step-physical text generation.
- **Schema Entities**:
  - `IFCSTRUCTURALANALYSISMODEL`
  - `IFCSTRUCTURALCURVEMEMBER` (linear framing: beams, columns, bracings with local axes)
  - `IFCSTRUCTURALSURFACEMEMBER` (2D planar elements: shear walls, floor slabs)
  - `IFCSTRUCTURALPOINTCONNECTION` (nodes with full 6-DOF elastic/rigid boundary conditions)
  - `IFCMATERIAL` (structural concrete and structural steel)
- **Multi-BIM Interoperability**: Seamlessly imported by Autodesk Revit Structure, Tekla Structures, ArchiCAD, BlenderBIM, and Solibri.

---

## ⚡ EigenGraph v0.0.2 — Phase 1: Quick Wins (ETABS Interop, PBG/SIMBG & Rebar BBS)
**Release Date:** 2026-11-06  
**Git Tag:** `v0.0.2`  
**Target:** Production Engine

EigenGraph v0.0.2 introduces critical interoperability and compliance automation: bidirectional CSI ETABS `.e2k` translation, Indonesian municipal PBG/SIMBG calculation dossier generation per PP 16/2021 & SNI 1726/2847, and automated Bar Bending Schedules (BBS) per SNI 2847:2019 Pasal 25.3.

### 🌟 Key Features & Engineering Formulations

#### 1. CSI ETABS `.e2k` Native Importer & Exporter (`EtabsE2kExporter`, `EtabsE2kImporter`)
- Complete bidirectional exchange with CSI ETABS text definition files (`.e2k` / `.$et`).
- Serializes and parses `PROGRAM INFORMATION`, `STORIES`, `GRIDS`, `POINT COORDINATES`, `FRAME SECTIONS`, `LINE OBJECTS`, and point/line structural loads.
- Allows structural engineers to migrate models seamlessly between ETABS v18–v21 and EigenGraph.

#### 2. Indonesian Municipal PBG/SIMBG Technical Calculation Dossier (`PbgReportGenerator`, `PbgProjectMetadata`)
- Automated generation of structural calculation reports (*Dokumen Perhitungan Teknis Struktur*) tailored for the Indonesian Ministry of Public Works and Public Housing (PUPR) SIMBG portal under Government Regulation PP No. 16 Tahun 2021.
- **Automated Code Compliance Checks**:
  - **SNI 1726:2019**: Inelastic drift limit $\Delta_a = 0.020 h_{sx} / \rho$, P-Delta stability coefficients, modal mass participation ratio $\ge 90\%$.
  - **SNI 2847:2019**: Strength reduction factors ($\phi = 0.90$ flexure, $\phi = 0.75$ shear/torsion, $\phi = 0.65$ tied axial compression), and Strong-Column Weak-Beam flexural capacity ratio $\sum M_{nc} / \sum M_{nb} \ge 1.20$.
  - **SNI 1727:2020**: Minimum design loads for buildings.
- Formatted in publication-ready Markdown/PDF with engineering stamp seals, SKA/SKK Utama certification, and project registration metadata.

#### 3. Automated Bar Bending Schedule (BBS) & Rebar Detailing (`RebarBbsDetailer`, `RebarBbsSchedule`)
- Complies with SNI 2847:2019 Pasal 25.3 / ACI 318-19 Chapter 25.
- Supports deformed rebar grades BJTS 420B (D10, D13, D16, D19, D22, D25) with exact unit weights per meter ($w = 0.006165 \times d_b^2 \text{ kg/m}$).
- Calculates development length ($l_d$), seismic standard hooks ($135^\circ$ with $6d_b \ge 75\text{ mm}$ extension), lap splice lengths ($1.3 l_d$ Class B), and total rebar tonnage including scrap allowance.
- Exports tabular CSV for procurement, cost estimation (BoQ), and fabrication shops.

---

## 🏛️ EigenGraph v0.0.1 — Core Non-Linear Solver & Structural Mechanics Engine
**Release Date:** 2026-11-06  
**Git Tag:** `v0.0.1`  
**Target:** Initial Release

- **Non-Linear Sherman-Morrison & Woodbury Redistribution Engine (`core::nonlinear`)**:
  - Rank-1 matrix perturbation solver computing exact redistribution vector $\Delta \mathbf{N}$ without full stiffness inversion $\mathcal{O}(N^3) \to \mathcal{O}(N^2)$.
  - Redundancy Reserve Ratio ($RRR(\lambda)$) tracking progressive plastic hinge formation.
- **3D Biaxial P-M-M Concrete Interaction Surface (`core::concrete`)**:
  - Bresler reciprocal load method and PCA load contour formulation per SNI 2847:2019 and ACI 318-19.
  - Capacity ratio checks across axial-flexural limit states.
- **CQC and SRSS Modal Dynamic Response Combination (`core::modal`)**:
  - Complete Quadratic Combination with Der Kiureghian cross-modal correlation coefficients $\rho_{ij}$ accounting for closely spaced modal frequencies.
  - 85% static base shear minimum scaling per SNI 1726:2019 Pasal 7.9.4.1.
- **Linear High-Performance Solvers**:
  - Sparse Cholesky $\mathbf{L}\mathbf{L}^T$ and Preconditioned Conjugate Gradient (PCG) in Compressed Sparse Row (CSR) format.
- **Vector Blueprint Exporter**:
  - Direct CAD-to-Vector generator exporting architectural and structural sheets to SVG and vector PDF.
