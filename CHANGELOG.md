# Changelog

All notable changes to the **EigenGraph** structural analysis and civil engineering platform will be documented in this file.

The project strictly adheres to sequential versioning:
`v0.0.1` ➔ `v0.0.2` ➔ `v0.0.3` ... ➔ `v0.0.9` ➔ `v0.1.0` ... ➔ `v0.9.9` ➔ `v1.0.0` ... (never skip patch numbers).

---

## [v0.0.5] - 2026-11-06 — Phase 4: Geotechnical Engineering, Deep Foundations & Soil-Structure Interaction (SSI)

### Added
- **Winkler Subgrade Reaction & 6-DOF Foundation Spring Matrix (`WinklerSpringMatrix`, `SoilLayer`, `SoilType`)**:
  - Implemented the Bowles / Vesic elastic subgrade reaction formulation in `src/core/geotech.rs` for spread and mat foundations.
  - Formulated equivalent 6-DOF foundation stiffness matrix:
    - Translational vertical stiffness: $K_z = k_s \cdot B \cdot L$ (kN/m).
    - Translational horizontal shear stiffness: $K_x = K_y = 0.82 \cdot K_z$ (kN/m).
    - Rocking rotational stiffness about primary axes: $K_{\theta x} = k_s \cdot \frac{B \cdot L^3}{12}$, $K_{\theta y} = k_s \cdot \frac{L \cdot B^3}{12}$ (kNm/rad).
    - Torsional rotational stiffness: $K_{\theta z} = 0.65 \cdot k_s \cdot (I_x + I_y)$ (kNm/rad).
  - Built stratigraphy models for 7 standard soil layers: Dense Sand, Medium Dense Sand, Loose Sand, Stiff Cohesive Clay, Medium Stiff Clay, Soft Clay, and Competent Bedrock ($\gamma, c_u, \phi, k_s, E_s, \nu$).
- **Deep Foundation Pile Group Converse-Labarre Efficiency & Biaxial Distribution (`evaluate_pile_group`, `PileElement`, `PileGroupReport`)**:
  - Implemented the Converse-Labarre pile group efficiency formulation:
    $$E_g = 1 - \theta \cdot \frac{(n_x - 1)n_y + (n_y - 1)n_x}{90 \cdot n_x \cdot n_y}, \quad \text{where } \theta = \arctan(d / s)$$
  - Formulated single pile ultimate compressive and uplift capacities ($q_{\text{end}} + q_{\text{skin}}$) per Meyerhof / Tomlinson standards.
  - Evaluated rigorous 2D grid biaxial eccentric load distribution:
    $$P_i = \frac{P_u}{N} \pm \frac{M_{uy} \cdot x_i}{\sum x^2} \pm \frac{M_{ux} \cdot y_i}{\sum y^2}$$
  - Computed lateral subgrade resistance and lateral pile head deflection $\delta_{\text{lat}}$ under tributary horizontal shear.
- **Two-Way Concrete Punching Shear Audit per SNI 2847:2019 / ACI 318-19 Section 22.6 (`audit_punching_shear`, `PunchingShearAudit`)**:
  - Established critical shear perimeter $b_o = 2(c_1 + d) + 2(c_2 + d)$ at $d/2$ from column perimeter.
  - Evaluated governing nominal concrete shear capacity per SNI 2847:2019 Eq. 22.6.5.2:
    $$V_c = \min \left\{ 0.33 \sqrt{f'_c}, \; 0.17 \left(1 + \frac{2}{\beta}\right) \sqrt{f'_c}, \; 0.083 \left(2 + \frac{\alpha_s d}{b_o}\right) \sqrt{f'_c} \right\} \cdot b_o d$$
  - Factored punching shear stress with direct shear and unbalanced eccentric moment transfer:
    $$\nu_u = \frac{V_u}{b_o d} + \frac{\gamma_v M_{unb} c}{J_c}$$
  - Automated safety verdict, demand-capacity ratio ($DCR_{\text{punch}}$), and prescriptive shear stud reinforcement advisories.
- **Veletsos & Meek Dynamic Soil-Structure Interaction (SSI) Modal Period Lengthening (`evaluate_ssi_modal_shift`, `SsiModalShiftReport`)**:
  - Formulated Veletsos & Meek dynamic SSI period lengthening:
    $$\left(\frac{\tilde{T}}{T}\right)^2 = 1 + \frac{K_{\text{fixed}}}{K_h} + \frac{K_{\text{fixed}} h^2}{K_r}$$
  - Evaluated ASCE 7-22 Chapter 19 foundation radiation and hysteretic damping $\beta_{\text{ssi}}$.
  - Derived design base shear reduction allowance $(1 / (\tilde{T}/T)^{0.85})$ and lateral drift amplification.
- **Interactive Geotechnical & SSI Dashboard in Analyze View (`src/views/analyze.rs`)**:
  - Added new sub-tab `AnalyzeSubTab::GeotechSsi` (`🌱 Geotech & SSI`).
  - Integrated 4 executive metric cards: `SUBGRADE MODULUS (ks)`, `CONVERSE-LABARRE (Eg)`, `PUNCHING SHEAR (SNI 22.6)`, and `SSI DYNAMIC SHIFT (T̃/T)`.
  - Added interactive dual visualizer:
    1. 2D Pile Group Plan Layout with cap geometry, column base, and individual pile axial load status colors.
    2. Critical Punching Shear Perimeter $b_o$ with column cross-section, $d/2$ reticles, and stress callouts.
  - Added 6-DOF Winkler Spring Matrix summary and individual pile biaxial load/deflection schedule table.
- **Exportable Geotechnical & SSI Calculation Dossier (`generate_markdown_dossier`)**:
  - Produces publication-ready Markdown calculation sheets with LaTeX equations, spring matrices, pile schedules, and SNI 2847 verification tables.
  - Added one-click export button `🌱 Geotech & SSI Dossier (.md)` in Results View (`src/views/results.rs`).

---

## [v0.0.4] - 2026-11-06 — Phase 3: Advanced Seismic & Pushover Analysis (ASCE 41-17 / ATC-40)

### Added
- **Capacity Spectrum Method Engine (`PushoverEngine`, `CapacitySpectrum`, `InelasticDemandSpectrum`)**:
  - Implemented the complete ATC-40 / ASCE 41-17 Capacity Spectrum Method in `src/core/pushover.rs`.
  - Formulated conversion of Base Shear vs Roof Displacement $(V_b, D_{\text{roof}})$ into Acceleration-Displacement Response Spectrum ($ADRS$) format $(S_a, S_d)$ using modal participation factor $\Gamma_1$ and modal mass coefficient $\alpha_1$:
    $$S_d = \frac{D_{\text{roof}}}{\Gamma_1 \cdot \phi_{\text{roof}, 1}}, \quad S_a = \frac{V_b / W}{\alpha_1}$$
  - Synthesized 5% damped elastic design response spectrum per SNI 1726:2019 / ASCE 7-22 ($S_s, S_1, F_a, F_v, S_{DS}, S_{D1}, T_0, T_s$).
  - Derived equivalent hysteretic viscous damping $\beta_{\text{eff}} = \beta_0 + \kappa \cdot \frac{2}{\pi} \frac{A_y D_p - D_y A_p}{A_p D_p}$ and spectral damping reduction factors $SR_A$ and $SR_V$.
  - Implemented ATC-40 Procedure A iterative solver converging to the **Performance Point** $(S_d^*, S_a^*)$, target displacement $D_t$, and target base shear $V_t$.
- **Performance-Based Earthquake Engineering (PBEE) Classification (`SeismicPerformanceLevel`)**:
  - Automated structural audit against ASCE 41-17 Table 9-6 / FEMA 356 performance levels:
    - `ImmediateOccupancy` (IO): Minor micro-yielding, no residual drift, structure safe for immediate post-earthquake occupancy.
    - `LifeSafety` (LS): Controlled inelastic plastification, wide collapse margin, main structural framing stable under Design Basis Earthquake (DBE).
    - `CollapsePrevention` (CP): Severe structural damage, minimal residual margin, prevents global collapse under Maximum Considered Earthquake (MCE).
    - `CollapseExceeded`: Target displacement exceeds ultimate capacity; structure requires retrofitting or shear wall stiffening.
  - Comprehensive plastic hinge distribution tracking at the Performance Point across Elastic, IO, LS, CP, and Mechanism states.
- **Configurable Lateral Load Patterns (`LateralLoadPattern`)**:
  - Added support for `FirstModeModal` (proportional to fundamental mode shape), `UniformMass` (proportional to floor mass acceleration), and `InvertedTriangular` (ASCE 7 $k$-exponent height distribution).
- **Interactive Dual Vector Visualizer in Analyze View (`src/views/analyze.rs`)**:
  - Integrated side-by-side interactive vector plots:
    1. Base Shear vs Roof Drift capacity curve with target displacement $D_t$ reticle indicator.
    2. ADRS Capacity-Demand Spectrum showing Elastic Demand (red dashed), Inelastic Reduced Demand (amber), Capacity Spectrum (emerald green), and intersecting Performance Point $P^*(S_d^*, S_a^*)$.
  - Added live toolbar controls for Lateral Load Pattern, Soil Site Class (SB, SC, SD, SE), and spectral parameters $S_s, S_1$.
- **Official Exportable ASCE 41 Pushover Calculation Dossier**:
  - Added `generate_markdown_dossier` producing publication-ready Markdown calculation sheets with LaTeX equations, performance metrics, and hinge distribution matrices.
  - Added one-click export button `📉 ASCE 41 Pushover Dossier (.md)` in Results View (`src/views/results.rs`).

---

## [v0.0.3] - 2026-11-06 — Phase 2: Core Non-Linear & Geometry Upgrade

### Added
- **2D Mindlin-Reissner Flat Shell / Plate Finite Elements (`ShellElement4N`)**:
  - Implemented 4-node isoparametric quadrilateral flat shell elements in `src/core/model.rs` for reinforced concrete shear walls (*dinding geser*) and floor diaphragms.
  - Formulated combined membrane stiffness ($u, v$ in-plane behavior) and Mindlin-Reissner plate bending ($w, \theta_x, \theta_y$ with transverse shear deformation) plus drilling degree of freedom $\theta_z$, providing 6 degrees of freedom per node (24 DOFs per quad element).
  - Evaluated stiffness and internal stresses using $2 \times 2$ Gauss-Legendre numerical quadrature.
  - Added geometric attribute helpers for shear wall classification (`as_shear_wall`) and exact shell surface area integration.
  - Integrated `pub shells: Vec<ShellElement4N>` into `StructuralModel`.
- **Second-Order Geometric Non-Linearity $P-\Delta$ & $P-\delta$ Solver (`SecondOrderStabilitySolver`)**:
  - Implemented geometric stiffness matrix $\mathbf{K}_g(\mathbf{P})$ in `src/core/solver.rs` accounting for axial load softening/stiffening effects.
  - Formulated tangent stiffness matrix $\mathbf{K}_t = \mathbf{K}_e - \mathbf{K}_g(\mathbf{P})$.
  - Evaluated Euler elastic critical buckling load $P_{\text{cr}} = \frac{\pi^2 E I}{(K L)^2}$ for slender compression columns.
  - Evaluated AISC 360-16 / 360-22 $B_1$ curvature amplification factor:
    $$B_1 = \frac{C_m}{1 - P_u / P_{e1}} \ge 1.0$$
  - Evaluated ASCE 7-22 Section 12.8.7 story drift stability coefficient:
    $$\theta = \frac{P_x \Delta I_e}{V_x h_{sx} C_d} \le \theta_{\max} = \frac{0.50}{\beta C_d} \le 0.25$$
  - Added automated stability classification (`StableLinear`, `PDeltaAmplified`, `CriticalBucklingRisk`, `DynamicPDeltaInstability`).
- **OpenBIM Industry Foundation Classes (IFC4 / IFC4.3) Structural Analysis Model Exporter (`IfcSamExporter`)**:
  - Implemented ISO 16739-1 compliant `.ifc` step-physical text exporter in `src/core/interop.rs`.
  - Serializes `IFCSTRUCTURALANALYSISMODEL`, `IFCSTRUCTURALCURVEMEMBER` (beams and columns with coordinate axes), `IFCSTRUCTURALSURFACEMEMBER` (4-node shear walls and slabs), `IFCSTRUCTURALPOINTCONNECTION` (nodes and boundary conditions), and `IFCMATERIAL` (structural concrete and structural steel).
  - Validated compatibility with Autodesk Revit Structure, Tekla Structures, ArchiCAD, and BlenderBIM.
- **UI Integration in Results View (`src/views/results.rs`)**:
  - Added export trigger button `🏛️ IFC4 SAM (.ifc)` alongside ETABS and SIMBG exports.

---

## [v0.0.2] - 2026-11-06 — Phase 1: Quick Wins (ETABS Interop, PBG/SIMBG & Rebar BBS)

### Added
- **CSI ETABS `.e2k` Native Importer & Exporter (`EtabsE2kExporter`, `EtabsE2kImporter`)**:
  - Implemented complete bidirectional exchange in `src/core/interop.rs` for CSI ETABS text definition files (`.e2k` / `.$et`).
  - Added parser and serializer blocks for `PROGRAM INFORMATION`, `STORIES`, `GRIDS`, `POINT COORDINATES`, `FRAME SECTIONS`, `LINE CONNECTIVITY` (`LINE OBJECTS`), and point/line structural loads.
  - Enables zero-loss structural migration between CSI ETABS v18–v21 and EigenGraph.
- **Indonesian Municipal PBG/SIMBG Technical Calculation Dossier Generator (`PbgReportGenerator`, `PbgProjectMetadata`)**:
  - Implemented automated structural calculation report generator in `src/core/interop.rs` tailored for the Indonesian Ministry of Public Works and Public Housing (PUPR) SIMBG portal under Government Regulation PP No. 16 Tahun 2021.
  - Encoded regulatory compliance audits:
    - **SNI 1726:2019** (Earthquake Resistant Building Design): Inelastic drift limit $\Delta_a = 0.020 h_{sx} / \rho$, P-Delta stability coefficients, modal mass participation ratio $\ge 90\%$.
    - **SNI 2847:2019** (Structural Concrete Requirements): Strength reduction factors ($\phi = 0.90$ flexure, $\phi = 0.75$ shear/torsion, $\phi = 0.65$ tied axial compression), and Strong-Column Weak-Beam flexural capacity ratio $\sum M_{nc} / \sum M_{nb} \ge 1.20$.
    - **SNI 1727:2020** (Minimum Design Loads for Buildings).
  - Generates comprehensive technical dossiers formatted in publication-ready Markdown/PDF with engineering stamp seals, SKA/SKK Utama verification, and project registration metadata.
- **Automated Bar Bending Schedule (BBS) & Rebar Detailing (`RebarBbsDetailer`, `RebarBbsSchedule`)**:
  - Implemented automated rebar cutting schedule and detailing engine in `src/core/interop.rs` in accordance with SNI 2847:2019 Pasal 25.3 / ACI 318-19 Chapter 25.
  - Standardized deformed rebar grades (BJTS 420B: D10, D13, D16, D19, D22, D25) with exact unit weights per meter ($w = 0.006165 \times d_b^2 \text{ kg/m}$).
  - Automatic calculation of development length ($l_d$), seismic standard hooks ($135^\circ$ with $6d_b \ge 75\text{ mm}$ extension), lap splice lengths ($1.3 l_d$ Class B), and total rebar tonnage including scrap allowance.
  - Produces tabular CSV output for procurement, cost estimation (BoQ), and fabrication shops.
- **UI Action Triggers in Results View (`src/views/results.rs`)**:
  - Added dedicated one-click export buttons:
    - `📄 SIMBG Dossier (.md)`
    - `📊 Bar Schedule (.csv)`
    - `🔄 Export ETABS (.e2k)`

---

## [v0.0.1] - 2026-11-06 — Initial Structural Core Engine

### Added
- **Non-Linear Sherman-Morrison & Woodbury Plastic Redistribution Engine (`src/core/nonlinear.rs`)**:
  - Rank-1 matrix perturbation solver computing exact redistribution vector $\Delta \mathbf{N}$ without full stiffness inversion $\mathcal{O}(N^3) \to \mathcal{O}(N^2)$.
  - Redundancy Reserve Ratio ($RRR(\lambda)$) tracking progressive plastic hinge formation.
- **3D Biaxial P-M-M Concrete Interaction Surface (`src/core/concrete.rs`)**:
  - Bresler reciprocal load method and PCA load contour formulation per SNI 2847:2019 and ACI 318-19.
  - Capacity ratio checks across axial-flexural limit states.
- **CQC and SRSS Modal Dynamic Response Combination (`src/core/modal.rs`)**:
  - Complete Quadratic Combination with Der Kiureghian cross-modal correlation coefficients $\rho_{ij}$ accounting for closely spaced modal frequencies.
  - 85% static base shear minimum scaling per SNI 1726:2019 Pasal 7.9.4.1.
- **High-Performance Linear Equation Solvers**:
  - Sparse Cholesky $\mathbf{L}\mathbf{L}^T$ and Preconditioned Conjugate Gradient (PCG) in Compressed Sparse Row (CSR) format.
- **Vector Blueprint Exporter**:
  - Direct CAD-to-Vector generator exporting architectural and structural sheets to SVG and vector PDF.
