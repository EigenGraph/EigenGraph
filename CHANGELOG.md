# Changelog

All notable changes to the **EigenGraph** structural analysis and civil engineering platform will be documented in this file.

The project strictly adheres to sequential versioning and strict 1-to-1 phase mapping:
`v0.0.1` (Phase 1) ➔ `v0.0.2` (Phase 2) ➔ `v0.0.3` (Phase 3) ➔ `v0.0.4` (Phase 4) ➔ `v0.0.5` (Phase 5) ➔ `v0.0.6` (Phase 6) ➔ `v0.0.7` (Phase 7) ... (never skip patch numbers).

---

## [v0.1.1] - 2026-11-07 — Phase 11: Special Structural Concrete Shear Walls & Diagonally Reinforced Coupling Beams Engine (ACI 318-19 / SNI 2847:2019)

### Added
- **RC Special Structural Shear Walls & Coupling Beams Engine (`src/core/shear_wall.rs`)**:
  - Full compliance with **ACI 318-19 Chapter 18 (Section 18.10)**, **SNI 2847:2019 Pasal 18.10**, and **NIST GCR 11-917-11 / NEHRP Seismic Design Brief No. 6**.
  - **In-Plane Nominal Shear Strength ($V_n$) (§18.10.4)**:
    - Aspect ratio factor $\alpha_c$ calculation:
      $$\alpha_c = \begin{cases} 0.25, & \frac{h_w}{l_w} \le 1.5 \\ 0.25 - 0.16\left(\frac{h_w}{l_w} - 1.5\right), & 1.5 < \frac{h_w}{l_w} < 2.0 \\ 0.17, & \frac{h_w}{l_w} \ge 2.0 \end{cases}$$
    - Nominal in-plane shear strength: $V_n = A_{cv} \left( \alpha_c \lambda \sqrt{f'_c} + \rho_t f_y \right) \le 0.83 A_{cv} \sqrt{f'_c}$ (with $\phi = 0.75$).
    - Automated enforcement of double curtain web reinforcement when $V_u > 0.17 A_{cv} \lambda \sqrt{f'_c}$ or $h_w / l_w \ge 2.0$.
  - **Dual Boundary Element Trigger Analysis (§18.10.6)**:
    - **Displacement-Based Trigger (§18.10.6.2)**: Evaluates $c \ge \frac{l_w}{600 (\delta_u / h_w)}$ with mandatory minimum design drift ratio $\delta_u / h_w \ge 0.005$.
    - **Stress-Based Trigger (§18.10.6.3)**: Normal compressive stress check $\sigma_{\text{comp}} = \frac{P_u}{A_g} + \frac{M_u (l_w/2)}{I_g} \ge 0.20 f'_c$.
  - **Special Boundary Element (SBE) Geometric & Confinement Detailing**:
    - Horizontal boundary extension length: $b_E \ge \max\left(c - 0.1 l_w, \; \frac{c}{2}\right)$.
    - Out-of-plane lateral buckling thickness: $b \ge \max\left(t_w, \; \frac{h_u}{16}\right)$.
    - Transverse confinement reinforcement ratio per Table 18.10.6.4(f):
      $$\frac{A_{sh}}{s \cdot b_c} \ge \max\left( 0.3 \left(\frac{A_g}{A_{ch}} - 1\right) \frac{f'_c}{f_{yt}}, \; 0.09 \frac{f'_c}{f_{yt}} \right)$$
    - Maximum vertical tie pitch: $s_{\max} = \min\left( \frac{b}{3}, \; 6 d_{b,\text{long}}, \; 150\text{ mm} \right)$.
  - **Diagonally Reinforced Coupling Beams (§18.10.7)**:
    - Classification across aspect ratios: Diagonally Reinforced ($l_n / h < 2.0$ and $V_u \ge 0.33 \lambda \sqrt{f'_c} b_w d$), Conventional Flexural ($l_n / h \ge 4.0$), and Transition ($2.0 \le l_n / h < 4.0$).
    - Diagonal bar inclination angle: $\alpha = \arctan\left(\frac{h - 2 d'}{l_n}\right)$.
    - Nominal shear strength: $V_n = 2 A_{vd} f_y \sin\alpha \le 0.83 \sqrt{f'_c} b_w d$.
    - Confinement hoop cages around individual diagonal bar groups: $A_{sh} \ge 0.09 s b_c \frac{f'_c}{f_{yt}}$.
  - **Wall Pier Axial-Flexural Capacity**:
    - Sliced Whitney stress block ($a = \beta_1 c$) with boundary rebar couple and axial force $P_u$ for design moment $\phi M_n$.
  - **Coupled Core Tower 3D Archetype Generator (`generate_coupled_core_archetype`)**:
    - Synthesizes 12-story 3D structural model with two C-shaped wall piers, central door openings, and diagonally reinforced coupling beams.
  - **Engineering Markdown Dossier Generator (`generate_markdown_dossier`)**:
    - Generates complete calculation report with KaTeX mathematical formulas, code citations, and limit state verification matrix.
- **Deep Analysis Studio (`src/views/analyze.rs`)**:
  - Added subtab `AnalyzeSubTab::ShearWallDesign` ("🏢 RC Shear Walls & Coupling Beams (ACI 318 / SNI 2847)").
  - 4 Executive metric cards: Overall Compliance & Governing DCR, Boundary Element (SBE) Trigger, In-Plane Web Shear $\phi V_n$, and Coupling Beam $\phi V_n$.
  - Parameter Studio with interactive controls for wall geometry, material strengths, applied demands, and coupling beam parameters.
  - 2D Vector CAD Canvas displaying wall plan section with SBE boundary zones, confining tie cages, longitudinal rebar dots, double curtain web mesh, neutral axis indicator, and coupling beam elevation with diagonal X-reinforcement.
  - Comprehensive ACI 318-19 / SNI 2847:2019 regulatory compliance audit table.
- **CAD Viewport Modeling Studio (`src/views/build.rs`)**:
  - Added `ModelArchetype::CoupledShearWallCore3D` ("🏢 Coupled Shear Wall Core (12-Story)").
  - 3D Viewport renders 12-story coupled shear wall tower with vertical wall piers, floor spandrels, and central coupling beams.
- **Results View Disk Exporters (`src/views/results.rs`)**:
  - Added export action `🏢 RC Shear Wall Dossier (.md)` generating `EigenGraph_RC_Shear_Wall_Coupling_Beam_Dossier.md`.

---

## [v0.1.0] - 2026-11-07 — Phase 10: Prestressed & Post-Tensioned Concrete Engine (ACI 318-19 / SNI 2847:2019)

### Added
- **Prestressed & Post-Tensioned Computational Engine (`src/core/prestressed_concrete.rs`)**:
  - Full compliance with **ACI 318-19 Chapter 24**, **SNI 2847:2019 Pasal 24**, **AASHTO LRFD Section 5**, and **PCI Design Handbook (8th Edition)**.
  - **Parametric Tendon Profiling**:
    - Parabolic draped continuous profile with midspan and end eccentricities: $e(x) = e_{mid} - 4(e_{mid} - e_{end})(x/L - 0.5)^2$.
    - Harped 2-point depressed profile with adjustable hold-down stations.
    - Straight eccentric profile with constant eccentricity.
    - Equivalent upward balancing load calculations: $w_{bal} = \frac{8 P_{eff} (e_{mid} - e_{end})}{L^2}$.
  - **Prestress Losses Engine**:
    - **Immediate Losses**:
      - Curvature & wobble friction loss: $\Delta f_{pF}(x) = f_{pj} [1 - \exp(-(\mu \alpha + K x))]$.
      - Anchorage seating draw-in slip: $\Delta f_{pA} = \frac{\Delta_{anc} E_p}{L_{set}}$.
      - Concrete elastic shortening: $\Delta f_{pES} = \frac{N-1}{2N} \frac{E_p}{E_{ci}} f_{cgp}$ (post-tensioned) or $\frac{E_p}{E_{ci}} f_{cgp}$ (pre-tensioned).
    - **Long-Term Time-Dependent Losses**:
      - Concrete creep loss: $\Delta f_{pCR} = K_{cr} \frac{E_p}{E_c} (f_{cgp} - f_{cds})$.
      - Concrete shrinkage loss: $\Delta f_{pSH} = 8.2 \times 10^{-6} K_{sh} E_p (1 - 0.06 V/S)(100 - RH)$.
      - Steel strand relaxation: $\Delta f_{pRE} = [K_{re} - J(\Delta f_{pSH} + \Delta f_{pCR} + \Delta f_{pES})] C$.
      - Total effective prestress: $P_{eff} = A_{ps} f_{se}$.
  - **Concrete Fiber Stresses & Serviceability Verification**:
    - Initial transfer stage stresses ($\sigma_{top,i}, \sigma_{bot,i}$) vs allowable compression ($0.60 f'_{ci}$) and tension ($0.25 \sqrt{f'_{ci}}$).
    - Service limit state stresses ($\sigma_{top,s}, \sigma_{bot,s}$) vs sustained compression ($0.45 f'_c$) and tensile limits.
    - ACI 318-19 Table 24.5.2.1 Classification: Class U (Uncracked), Class T (Transition), Class C (Cracked).
    - Cracking moment: $M_{cr} = S_b [f_r + P_{eff}/A + P_{eff} e / S_b]$.
  - **Ultimate Flexural & Shear Capacities**:
    - Prestressing steel stress $f_{ps}$ at nominal flexural strength per ACI 318-19 Eq. 20.3.2.3.1.
    - Equivalent rectangular Whitney compression block depth $a$ and neutral axis $c$.
    - Flexural capacity $\phi M_n$ and minimum flexure requirement ($\phi M_n \ge 1.2 M_{cr}$).
    - Concrete shear resistance: flexure-shear cracking $V_{ci}$ and web-shear cracking $V_{cw}$, with stirrup reinforcement capacity $V_s$ and design strength $\phi V_n$.
  - **Camber & Deflection Analytics**:
    - Upward prestress camber $\Delta_{p,i}$, self-weight downward deflection $\Delta_{sw}$, net transfer camber, and long-term camber via PCI Martin Multipliers.
  - **Bridge Archetype Generator (`generate_bridge_archetype_elements`)**:
    - Generates complete 3D structural model with precast AASHTO girders, transverse RC diaphragms, and 3D parabolic tendon runs.
  - **Markdown Design Dossier Generator (`generate_markdown_dossier`)**:
    - Formats comprehensive engineering calculation reports with KaTeX math, tables, and regulatory compliance status.
- **Deep Analysis Studio (`src/views/analyze.rs`)**:
  - Added `AnalyzeSubTab::PrestressedDesign` ("🪢 PT Concrete (ACI 318)").
  - 4 Metric cards: Overall Compliance & Governing DCR, Effective Prestress & Total Losses, Flexural Strength $\phi M_n$, and Service Class & Camber.
  - Real-time 2D Vector Canvas rendering girder elevation, neutral axis, and parabolic tendon trajectory with anchor plates.
  - Prestress losses breakdown, fiber stress checks table, and code compliance audit.
- **CAD Viewport Modeling Studio (`src/views/build.rs`)**:
  - Added `ModelArchetype::PrestressedGirderBridge3D` ("🌉 Prestressed Bridge (30m PT Girder)").
  - 3D Viewport renders precast I-girders, transverse diaphragms, and 3D internal parabolic prestressing cables.
- **Results View Disk Exporters (`src/views/results.rs`)**:
  - Added export action `🪢 Prestressed PT Dossier (.md)` generating `EigenGraph_Prestressed_Concrete_Dossier.md`.

---

## [v0.0.9] - 2026-11-06 — Phase 9: Modeling & Advanced Computational Geometry Engine

### Added
- **Green's Theorem Arbitrary Cross-Section Property Integrator (`src/core/geometry_engine.rs`)**:
  - Exact boundary line integrals for arbitrary 2D closed planar polygons (solid, hollow, asymmetric):
    - Area: $A = \frac{1}{2} \sum (x_i y_{i+1} - x_{i+1} y_i)$
    - Centroids: $\bar{x} = \frac{1}{6A} \sum (x_i + x_{i+1}) (x_i y_{i+1} - x_{i+1} y_i), \quad \bar{y} = \frac{1}{6A} \sum (y_i + y_{i+1}) (x_i y_{i+1} - x_{i+1} y_i)$
    - Moments of Inertia: $I_{xx0} = \frac{1}{12} \sum (y_i^2 + y_i y_{i+1} + y_{i+1}^2) (x_i y_{i+1} - x_{i+1} y_i), \quad I_{yy0} = \frac{1}{12} \sum (x_i^2 + x_i x_{i+1} + x_{i+1}^2) (x_i y_{i+1} - x_{i+1} y_i)$
    - Product of Inertia: $I_{xy0} = \frac{1}{24} \sum (x_i y_{i+1} + 2 x_i y_i + 2 x_{i+1} y_{i+1} + x_{i+1} y_i) (x_i y_{i+1} - x_{i+1} y_i)$
    - Parallel axis theorem centroidal transfer: $I_{xx} = I_{xx0} - A \bar{y}^2, \quad I_{yy} = I_{yy0} - A \bar{x}^2, \quad I_{xy} = I_{xy0} - A \bar{x} \bar{y}$
    - Principal Moments of Inertia ($I_1, I_2$) and Principal Orientation Angle $\theta_p$:
      $$\theta_p = \frac{1}{2} \operatorname{atan2}(-2 I_{xy}, \; I_{xx} - I_{yy})$$
    - Radii of gyration ($r_x, r_y$), elastic section moduli ($S_x, S_y$), sliced numerical plastic section moduli ($Z_x, Z_y$), polar moment ($J$), and Saint-Venant torsional constant ($J_t$).
  - Standard parametric polygon generators for Wide-Flange (I-Shape), Channel (UNP), Angle (L-Shape), and T-Shape profiles.
- **Parametric Spatial Structure Generator Wizard (`ParametricSpatialWizard`)**:
  - **3D Double-Layer Space Truss (MERO Octet Lattice)**: Parametric orthogonal two-way top/bottom chord grid with diagonal pyramidal web bracing and perimeter support restraints.
  - **3D Geodesic Lattice Dome (Schwedler Dome)**: Spherical geometry subdivision generating hoop rings, meridional ribs, and diagonal triangulating ties with crown apex convergence.
  - **Hyperbolic Paraboloid Ruled Shell (Hypar Saddle / Inverted Umbrella)**: Doubly-curved ruled surface generation ($z = 4c \cdot \xi \cdot \eta$) with orthogonal straight-line structural framing and 4-node quad shell panel meshing.
  - **Multi-Story Building Frame with Core Walls**: Parametric multi-story 3D building tower with columns, floor beams, and central concrete shear wall shell panels.
- **Topological Mesh Operations & Cleaners (`MeshTopologyEngine`)**:
  - Distance-based node consolidation: merges duplicate vertices within $\varepsilon_{\text{tol}}$ tolerance and remaps 1D beam and 2D shell element connectivity.
  - Certified Wavefront 3D OBJ mesh exporter (`export_to_obj`) and AutoCAD 3D DXF exporter (`export_to_dxf_3d`).
  - Automated computational geometry markdown dossier generator (`generate_markdown_dossier`).
- **Interactive 3D Viewport Modeling Studio in Build View (`src/views/build.rs`)**:
  - Added `SidebarTab::Geometry` ("📐", "Geometry & Wizard") to the CAD activity bar.
  - Integrated Parametric Generator Wizard with interactive sliders for span dimensions, divisions, radius, rings, and stories.
  - Integrated Green's Theorem Section Integrator Studio with 2D vector cross-section canvas preview, centroid crosshair reticle, and live structural section properties table.
  - Added 4 new spatial archetypes to 3D Viewport Top Ribbon and camera presets.
- **Results View Disk Exporters (`src/views/results.rs`)**:
  - Added export action `📐 Spatial Geometry Dossier (.md)` generating `EigenGraph_Computational_Geometry_Dossier.md`.
  - Added export action `🧊 Wavefront 3D (.obj)` generating `EigenGraph_Spatial_Truss.obj`.

---

## [v0.0.8] - 2026-11-06 — Phase 8: Composite Steel-Concrete Beam & Slab Design (AISC 360-16 / SNI 1729:2020)

### Added
- **AISC 360-16 Chapter I & SNI 1729:2020 Pasal I Composite Beam Design Engine (`src/core/composite.rs`)**:
  - **Headed Shear Stud Nominal Strength ($Q_n$)**:
    $$Q_n = \min\left(0.5 A_{sa} \sqrt{f'_c E_c}, \; R_g R_p A_{sa} F_u\right)$$
    Accounting for deck geometry reduction factors $R_g$ and $R_p$ across perpendicular and parallel ribs per AISC Eq. I8-1.
  - **Effective Slab Flange Width ($b_{\text{eff}}$)**:
    $$b_{\text{eff}} = \min\left(\frac{L}{4}, \; S\right)$$
    Per AISC 360-16 Section I3.1.1 for interior beams with span $L$ and beam center-to-center spacing $S$.
  - **Plastic Neutral Axis (PNA) Tracking & Plastic Moment Strength ($M_n$)**:
    - Complete plastic stress distribution analysis across concrete compression zone $C_c = 0.85 f'_c b_{\text{eff}} a$, shear stud force $\sum Q_n$, and steel tensile/compressive zones.
    - Automated detection and equilibrium calculation across 3 distinct cases:
      - **Case 1 (PNA in concrete slab)**: $a \le t_c$, entire steel section in pure tension.
      - **Case 2 (PNA in top steel flange)**: Concrete in compression, portion of top flange in compression, rest in tension.
      - **Case 3 (PNA in steel web)**: Concrete and top flange in compression, web depth in tension/compression equilibrium.
    - Design flexural capacity $\phi M_n$ with LRFD resistance factor $\phi_b = 0.90$.
  - **Degree of Composite Action Check**:
    $$\text{Composite Action} = \frac{\sum Q_n}{C_{\text{max}}} \times 100\% \ge 25\%$$
    Enforces AISC 360-16 minimum threshold of 25% composite interaction.
  - **Unshored Construction Stage Wet Concrete Stability**:
    - Evaluates wet concrete dead load and construction live load prior to concrete curing.
    - Verifies bare steel section flexural resistance $\phi M_n \ge M_{u,\text{const}}$ to prevent lateral-torsional buckling or plastic failure during pouring.
  - **Lower-Bound Effective Moment of Inertia ($I_{\text{eff}}$) & Serviceability Deflections**:
    $$I_{\text{eff}} = I_s + \sqrt{\frac{\sum Q_n}{C_{\text{max}}}} \left(I_{\text{tr}} - I_s\right)$$
    - Live load deflection check: $\Delta_{\text{LL}} \le L/360$.
    - Long-term creep deflection check (accounting for sustained slab dead load): $\Delta_{\text{total}} \le L/240$.
- **Interactive AISC Composite Beam Studio in Analyze View (`src/views/analyze.rs`)**:
  - Added new sub-tab `AnalyzeSubTab::CompositeDesign` (`🥞 AISC Composite`).
  - 4 Executive Metric Cards: Design Capacity ($\phi M_n$), DCR Flexure ratio, Composite Action percentage, and Live Load Deflection ($\Delta_{\text{LL}}$ vs. $L/360$).
  - Control Toolbar: Interactive configuration for span length $L$, beam spacing $S$, wide-flange steel dimensions ($d, b_f, t_f, t_w, F_y$), concrete slab thickness $t_c$, deck rib height $h_r$, $f'_c$, shear stud diameter/quantity, unshored construction toggle, and applied moment/live loads.
  - Dual Vector Visualizer:
    1. *Left*: 2D CAD cross-section detailing showing hatched concrete slab, trapezoidal metal deck ribs, headed shear studs, W-beam profile, and colored dashed line tracking the exact PNA elevation.
    2. *Right*: Plastic Stress Distribution block diagram and Deflection Gauge comparing $\Delta_{\text{LL}}$ against $L/360$.
  - Comprehensive AISC limit states verification table auditing flexure, shear stud capacity, minimum composite action, unshored construction stage, and serviceability creep deflection.
- **AISC 360-16 Composite Beam Markdown Dossier Export (`generate_markdown_dossier`)**:
  - Added one-click export button `🥞 AISC Composite Dossier (.md)` in Results View generating `AISC360_Composite_Beam_Audit.md`.

---

## [v0.0.7] - 2026-11-06 — Phase 7: Wind Engineering & Computational Aero-Dynamics per ASCE 7-22 / SNI 1727:2020

### Added
- **ASCE 7-22 & SNI 1727:2020 Computational Wind Engineering Engine (`src/core/wind.rs`)**:
  - **Exposure Categories & Terrain Profiles**: Implemented Exposure B (urban/suburban), Exposure C (open terrain), and Exposure D (unobstructed coastal) with terrain power-law constants ($\alpha$, $z_g$, $c_{\text{turb}}$) per Table 26.11-1.
  - **Velocity Pressure Exposure Coefficient ($K_z$) & Mean Roof Pressure ($q_h$)**:
    $$K_z = 2.01 \left( \frac{\max(z, 4.5)}{z_g} \right)^{2/\alpha}, \quad q_z = 0.613 K_z K_{zt} K_d K_e V^2 \; (\text{Pa})$$
    Formulated velocity pressure profiles factoring ground elevation $K_e$, wind directionality $K_d = 0.85$, and topographic factor $K_{zt}$.
  - **Dynamic Gust Effect Factor ($G_f$) for Flexible Structures**:
    - Formulated background factor $Q$ and resonant factor $R$ accounting for turbulent integral length scale $L_{\bar{z}}$, damping $\beta$, and aerodynamic admittance $R_h, R_b$.
    - Unified rigid ($G = 0.85$ for $f_1 \ge 1\text{ Hz}$) and flexible structure formulation ($f_1 < 1\text{ Hz}$) per ASCE 7-22 Section 26.11.
  - **Directional MWFRS Design Lateral Pressures & Story Shear Envelopes**:
    - Windward pressure: $p_{\text{windward}} = q_z G C_p$ ($C_p = 0.80$).
    - Leeward pressure: $p_{\text{leeward}} = q_h G C_p$ ($C_p = -0.50$ to $-0.30$ based on $L/B$).
    - Internal pressure variations with $GC_{pi} = \pm 0.18$ (Enclosed) and $\pm 0.55$ (Partially Enclosed).
    - Multi-story tributary shear aggregation $V_w(z) = \sum p_{\text{net}} A_{\text{trib}}$ and base overturning moment $M_{\text{OTM}} = \sum F_i z_i$.
  - **Dynamic Occupant Comfort & Vortex Shedding Acceleration (ISO 10137 / ASCE 7-22 CC)**:
    - Peak along-wind dynamic acceleration: $a_{\text{along}} = g \cdot \frac{3 \sigma_x \cdot (2 \ln(3600 f_1))^{1/2}}{M_{\text{gen}}}$ (milli-g).
    - Cross-wind vortex shedding acceleration incorporating reduced velocity $V_r = \frac{V}{f_1 B}$.
    - Resultant acceleration vs. standard occupant comfort limit ($20\text{ mg}$ for 10-year MRI wind per ISO 10137).
  - **Lateral Governing Demand Comparison**: Automated comparison of total wind base shear $V_w$ against seismic base shear $V_s$ to identify governing lateral resistance mechanism.
- **Interactive ASCE 7 Wind Engineering Dashboard in Analyze View (`src/views/analyze.rs`)**:
  - Added new sub-tab `AnalyzeSubTab::WindEngineering` (`🌪️ ASCE 7 Wind`).
  - 4 Executive Metric Cards: Basic Wind Speed ($V$), Mean Roof Pressure ($q_h$), Wind Base Shear ($V_w$ and $M_{\text{OTM}}$), and Occupant Comfort Acceleration ($a_{\text{res}}$ vs. ISO 10137 limit).
  - Control Toolbar: Real-time adjustments for wind speed (m/s), exposure category, building enclosure, building dimensions ($H \times B \times L$), number of stories, and fundamental period $T_1$.
  - Dual Vector Visualizer:
    1. *Left*: Elevation Velocity Pressure Profile $q_z(z)$ with CAD building silhouette and windward pressure arrows scaled by height.
    2. *Right*: Occupant Acceleration Bar Chart (Along-wind, Cross-wind, Resultant vs ISO 10137 limit) and Wind ($V_w$) vs. Seismic ($V_s$) base shear envelope comparison.
  - Story-by-story wind force schedule table displaying $K_z$, $q_z$, windward/leeward pressures, tributary story forces, and cumulative shear $V_x(z)$.
- **Official Wind Engineering Dossier Export (`generate_markdown_dossier`)**:
  - Added one-click export button `🌪️ ASCE 7 Wind Dossier (.md)` in Results View generating `ASCE7_Wind_Engineering_Dossier.md`.

---

## [v0.0.6] - 2026-11-06 — Phase 6: Steel Connection Design & AISC 360-16 / SNI 1729:2020 Limit States Engine

### Added
- **AISC 360-16 & SNI 1729:2020 Steel Connection Limit States Engine (`src/core/connection.rs`)**:
  - **Bolt Group Shear Rupture**: Formulated nominal bolt shear capacities ($F_{nv} A_b$) for ASTM A325-N/X, A490-N/X, and ISO Class 8.8 / 10.9 fasteners per AISC 360-16 Table J3.2.
  - **Bolt Bearing and Tearout**: Evaluated clear distance $l_c$ across edge and interior pitch spaces, calculating governing bearing resistance $R_n = \min(1.2 l_c t F_u, 2.4 d_b t F_u)$ per Section J3.10.
  - **Block Shear Rupture (AISC 360 Section J4.3 / SNI 1729 Section J4.3)**:
    $$R_n = \min \left( 0.60 F_u A_{nv} + U_{bs} F_u A_{nt}, \; 0.60 F_y A_{gv} + U_{bs} F_u A_{nt} \right) \quad (\phi = 0.75, \; U_{bs} = 1.0)$$
  - **Plate Gross Shear Yielding & Net Shear Rupture**: Sized connection plates for shear yield ($0.60 F_y A_g$) and net shear fracture ($0.60 F_u A_{nv}$).
  - **Double Fillet Weld Shear Resistance**: Formulated E70XX/E80XX weld metal shear capacity across effective throat $t_e = 0.707 w$ ($R_n = 0.60 F_{EXX} A_{we}$).
- **AISC 358-16 / SNI 7860:2020 4E Extended End-Plate Moment Connection (`evaluate_extended_end_plate`)**:
  - Implemented yield-line mechanism parameter $Y_p$ for 4E unstiffened extended end-plates:
    $$Y_p = \frac{b_p}{2} \left[ h_1 \left(\frac{1}{p_{fi}} + \frac{1}{s}\right) + h_0 \left(\frac{1}{p_{fo}}\right) \right] + \frac{2}{g} \left[ h_1 (p_{fi} + s) + h_0 p_{fo} \right]$$
  - Flexural design moment capacity: $\phi M_n = \phi_b F_{yp} t_p^2 Y_p$ ($\phi_b = 0.90$).
  - Evaluated bolt tension demands in tension flange group and gravity shear in compression zone bolts.
- **Column Panel Zone Shear & Web Stiffener Audit (AISC 360 Section J10)**:
  - Formulated column panel zone shear strength:
    $$R_n = 0.60 F_y d_c t_w \left[ 1 + \frac{3 b_{cf} t_{cf}^2}{d_b d_c t_w} \right] \quad (\phi = 0.90)$$
  - Automated calculation of required column web doubler plate thickness ($t_{\text{doubler}} = \frac{V_{pz} - \phi R_n}{0.60 \phi F_y d_c}$).
  - Evaluated Column Web Local Yielding ($\phi R_n = 1.0 F_{yw} t_w (5k + l_b)$) and Web Local Crippling, automatically assessing transverse continuity plate requirements.
- **Interactive Steel Connection CAD Blueprint & DCR Dashboard in Analyze View (`src/views/analyze.rs`)**:
  - Added new sub-tab `AnalyzeSubTab::SteelConnection` (`🔩 Steel Connections`).
  - Integrated 4 executive summary cards: Governing DCR, Critical Limit State, Fastener Configuration, and Panel Zone Audit.
  - Controls toolbar for connection typology, bolt grades, bolt diameter, plate thickness, and factored forces ($V_u, M_u$).
  - Dual Vector Visualizer:
    1. *Left*: 2D Elevation Blueprint showing column profile, beam flanges/web, end-plate/shear tab, tension/compression bolts, and weld beads.
    2. *Right*: Limit States DCR Utilization horizontal bar chart with color-coded utilization thresholds.
  - Detailed Limit States Verification Table displaying nominal capacity, $\phi$, design capacity, and safety status.
- **Official Steel Connection Dossier Export (`generate_markdown_dossier`)**:
  - Added one-click export button `🔩 AISC 360 Connection Dossier (.md)` in Results View generating `AISC360_Steel_Connection_Audit.md`.

---

## [v0.0.5] - 2026-11-06 — Phase 5: Geotechnical Engineering, Deep Foundations & Soil-Structure Interaction (SSI)

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

## [v0.0.4] - 2026-11-06 — Phase 4: Advanced Seismic & Pushover Analysis (ASCE 41-17 / ATC-40)

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

## [v0.0.3] - 2026-11-06 — Phase 3: Core Non-Linear & Geometry Upgrade (Shell FEA, P-Delta & IFC4 SAM)

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

## [v0.0.2] - 2026-11-06 — Phase 2: Quick Wins (ETABS Interop, PBG/SIMBG & Rebar BBS)

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

## [v0.0.1] - 2026-11-06 — Phase 1: Structural Mechanics & Non-Linear Core Engine

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
