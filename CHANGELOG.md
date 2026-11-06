# Changelog

All notable changes to the **EigenGraph** structural analysis and civil engineering platform will be documented in this file.

The project strictly adheres to sequential versioning:
`v0.0.1` ➔ `v0.0.2` ➔ `v0.0.3` ... ➔ `v0.0.9` ➔ `v0.1.0` ... ➔ `v0.9.9` ➔ `v1.0.0` ... (never skip patch numbers).

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
