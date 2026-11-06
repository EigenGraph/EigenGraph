# Changelog

All notable changes to the **EigenGraph** structural analysis and civil engineering platform will be documented in this file.

The project strictly adheres to sequential versioning:
`v0.0.1` ➔ `v0.0.2` ➔ `v0.0.3` ... ➔ `v0.0.9` ➔ `v0.1.0` ... ➔ `v0.9.9` ➔ `v1.0.0` ... (never skip patch numbers).

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
