# EigenGraph Release Notes & Changelog

All official versioned releases of EigenGraph are cataloged here.

The project strictly adheres to sequential versioning and strict 1-to-1 phase mapping:
`v0.0.1` (Phase 1) ➔ `v0.0.2` (Phase 2) ➔ `v0.0.3` (Phase 3) ➔ `v0.0.4` (Phase 4) ➔ `v0.0.5` (Phase 5) ➔ `v0.0.6` (Phase 6) ➔ `v0.0.7` (Phase 7) ... (never skip patch numbers).

---

## 🚀 EigenGraph v0.0.9 — Phase 9: Modeling & Advanced Computational Geometry Engine
**Release Date:** 2026-11-06  
**Git Tag:** `v0.0.9`  
**Target:** Production Engine

EigenGraph v0.0.9 introduces comprehensive **Modeling & Advanced Computational Geometry**, equipping structural engineers with an algorithmic spatial structure generator wizard, exact Green's Theorem boundary line integrals for arbitrary 2D cross-sections, duplicate vertex consolidation, and 3D Wavefront OBJ / AutoCAD DXF mesh exporters.

### 🌟 Key Features & Mathematical Formulation

#### 1. Green's Theorem Arbitrary Cross-Section Integrator
- **Exact Line Integrals for Closed Polygons**:
  $$A = \frac{1}{2} \left| \sum_{i=0}^{n-1} (x_i y_{i+1} - x_{i+1} y_i) \right|$$
  $$\bar{x} = \frac{1}{6A} \sum_{i=0}^{n-1} (x_i + x_{i+1}) (x_i y_{i+1} - x_{i+1} y_i), \quad \bar{y} = \frac{1}{6A} \sum_{i=0}^{n-1} (y_i + y_{i+1}) (x_i y_{i+1} - x_{i+1} y_i)$$
- **Centroidal Moments of Inertia via Parallel Axis Theorem**:
  $$I_{xx} = \frac{1}{12} \sum_{i=0}^{n-1} (y_i^2 + y_i y_{i+1} + y_{i+1}^2) (x_i y_{i+1} - x_{i+1} y_i) - A \bar{y}^2$$
  $$I_{yy} = \frac{1}{12} \sum_{i=0}^{n-1} (x_i^2 + x_i x_{i+1} + x_{i+1}^2) (x_i y_{i+1} - x_{i+1} y_i) - A \bar{x}^2$$
  $$I_{xy} = \frac{1}{24} \sum_{i=0}^{n-1} (x_i y_{i+1} + 2 x_i y_i + 2 x_{i+1} y_{i+1} + x_{i+1} y_i) (x_i y_{i+1} - x_{i+1} y_i) - A \bar{x} \bar{y}$$
- **Principal Inertias & Orientation**:
  $$I_1, I_2 = \frac{I_{xx} + I_{yy}}{2} \pm \sqrt{\left(\frac{I_{xx} - I_{yy}}{2}\right)^2 + I_{xy}^2}, \quad \theta_p = \frac{1}{2} \operatorname{atan2}(-2 I_{xy}, \; I_{xx} - I_{yy})$$
- Formulates elastic section moduli ($S_x, S_y$), sliced numerical plastic moduli ($Z_x, Z_y$), polar inertia ($J = I_{xx} + I_{yy}$), and Saint-Venant torsional constant ($J_t$).

#### 2. Parametric Spatial Structure Generator Wizard
- **3D Double-Layer Space Truss (MERO Octet)**:
  Generates orthogonal two-way top chord grid, bottom chord offset grid, and diagonal pyramidal web braces with perimeter corner pin supports.
- **3D Geodesic Lattice Dome (Schwedler Dome)**:
  Generates spherical surface subdivisions with hoop rings, meridional ribs, and diagonal cross-braces converging to the crown apex.
- **Hyperbolic Paraboloid Ruled Shell (Hypar Inverted Umbrella)**:
  Generates doubly-curved ruled surfaces ($z = 4c \cdot \xi \cdot \eta$) with orthogonal straight-line timber/steel framework and 4-node quad shell panel meshing.
- **Multi-Story 3D Tower with Core Walls**:
  Generates multi-story frame towers with 3D columns, beams, outrigger links, and central concrete shear wall shell panels.

#### 3. Topological Mesh Operations & Cleaners
- **Distance-Based Node Consolidation**: Eliminates duplicate spatial nodes within tolerance $\varepsilon_{\text{tol}} = 0.05\text{ m}$, remapping beam and shell connectivity.
- **Certified 3D Exporters**: Wavefront 3D OBJ (`export_to_obj`) and AutoCAD 3D DXF (`export_to_dxf_3d`).

#### 4. Interactive Desktop UI Integration
- **`SidebarTab::Geometry` ("📐 Geometry & Wizard")**:
  - Interactive sliders for dimensions, bays, radius, rings, and stories.
  - Section studio with live 2D CAD vector cross-section preview, centroid reticle, and properties table.
- **Top Ribbon Preset Selector**: Seamless 1-click switching between spatial archetypes in 360° interactive turntable viewport.
- **Results View Exporters**: One-click generation of `EigenGraph_Computational_Geometry_Dossier.md` and `EigenGraph_Spatial_Truss.obj`.

---

## 🚀 EigenGraph v0.0.8 — Phase 8: Composite Steel-Concrete Beam & Slab Design per AISC 360-16 / SNI 1729:2020
**Release Date:** 2026-11-06  
**Git Tag:** `v0.0.8`  
**Target:** Production Engine

EigenGraph v0.0.8 introduces comprehensive **Composite Steel-Concrete Beam & Slab Design**, delivering an automated code-compliance and plastic stress distribution engine per AISC 360-16 Chapter I and SNI 1729:2020 Pasal I for composite floor systems, headed shear stud connectors, ribbed metal decks, unshored construction stages, and long-term creep deflections.

### 🌟 Key Features & Mathematical Formulation

#### 1. Headed Shear Stud Nominal Shear Strength ($Q_n$)
- **AISC 360-16 Eq. I8-1**:
  $$Q_n = \min\left(0.5 A_{sa} \sqrt{f'_c E_c}, \; R_g R_p A_{sa} F_u\right)$$
  where $A_{sa}$ is the stud cross-sectional area ($\frac{\pi d_{sa}^2}{4}$), $f'_c$ is the concrete compressive strength, $E_c = 4700 \sqrt{f'_c}$ (MPa), and $F_u$ is the stud tensile strength (450 MPa for ASTM A108).
- **Deck Rib Geometry Reduction Factors**:
  - Perpendicular ribs: $R_g = 1.0$ (single stud per rib) or $0.85$ (multiple studs); $R_p = 0.75$ (for rib height $h_r \le 75\text{ mm}$ with $e_{mid-ht} \ge 50\text{ mm}$).
  - Parallel ribs: $R_g = 1.0, R_p = 0.75$.
  - Flat solid slab: $R_g = 1.0, R_p = 1.0$.

#### 2. Effective Slab Flange Width ($b_{\text{eff}}$) & Concrete Compression
- **AISC 360-16 Section I3.1.1**:
  $$b_{\text{eff}} = \min\left(\frac{L}{4}, \; S\right)$$
  where $L$ is the beam span and $S$ is the center-to-center beam spacing.
- **Maximum Concrete Compression Capacity**:
  $$C_{\text{max}} = 0.85 f'_c b_{\text{eff}} t_c$$
  where $t_c$ is the slab cover thickness above the metal deck ribs.

#### 3. Plastic Neutral Axis (PNA) Tracking & Flexural Resistance ($\phi M_n$)
- Total horizontal shear transfer $V' = \min(T_s, C_{\text{max}}, \sum Q_n)$, where $T_s = A_s F_y$.
- Plastic stress distribution evaluates three distinct equilibrium regimes:
  - **Case 1 (PNA in concrete slab)**: When $\sum Q_n \ge T_s$ and $a = \frac{T_s}{0.85 f'_c b_{\text{eff}}} \le t_c$.
    $$M_n = T_s \left( \frac{d}{2} + h_r + t_c - \frac{a}{2} \right)$$
  - **Case 2 (PNA in top steel flange)**: When $C_c < T_s$ but $C_c + 2 F_y b_f t_f \ge T_s$.
    $$y_p = \frac{T_s - C_c}{2 F_y b_f}, \quad M_n = C_c y_{c} + M_{\text{steel, PNA}}$$
  - **Case 3 (PNA in steel web)**: When PNA descends into the web.
    $$y_w = \frac{T_s - C_c - 2 F_y b_f t_f}{2 F_y t_w}, \quad M_n = C_c y_c + M_{\text{steel, PNA}}$$
- LRFD flexural design strength: $\phi M_n = 0.90 M_n$.

#### 4. Minimum Composite Action Verification
- AISC 360-16 requires at least 25% composite action:
  $$\text{Degree of Composite Action} = \frac{\sum Q_n}{C_{\text{max}}} \times 100\% \ge 25\%$$
  Prevents excessive slip and ductility loss at steel-concrete interface.

#### 5. Unshored Construction Stage Stability
- Evaluates wet concrete dead load ($w_{\text{conc}} = 24\text{ kN/m}^3 \times [t_c + 0.5 h_r] \times S$) + construction live load ($1.0\text{ kPa}$).
- Verifies bare steel section capacity $\phi M_n \ge M_{u,\text{const}}$ to prevent lateral-torsional or plastic failure during pouring prior to concrete curing.

#### 6. Lower-Bound Effective Moment of Inertia ($I_{\text{eff}}$) & Creep Deflection
- **AISC 360-16 Commentary Eq. C-I3-1**:
  $$I_{\text{eff}} = I_s + \sqrt{\frac{\sum Q_n}{C_{\text{max}}}} \left(I_{\text{tr}} - I_s\right)$$
  where $I_s$ is bare steel moment of inertia and $I_{\text{tr}}$ is elastic transformed section inertia.
- Serviceability checks:
  - Immediate live load deflection: $\Delta_{\text{LL}} = \frac{5 w_{\text{LL}} L^4}{384 E_s I_{\text{eff}}} \le \frac{L}{360}$.
  - Total sustained deflection including concrete creep ($k_{\text{creep}} = 1.35$): $\Delta_{\text{total}} \le \frac{L}{240}$.

#### 7. Interactive Analyze View Studio & Dossier Export
- **Sub-Tab `🥞 AISC Composite`**:
  - Real-time parameter controls for span, beam spacing, wide flange section, slab thickness, deck ribs, shear studs, and loads.
  - Dual vector graphics rendering 2D cross-section detailing (hatch, ribs, studs, W-beam, PNA elevation) + plastic stress block diagram and deflection gauge.
  - Limit states verification table with DCR indicators.
- **Export Action**: One-click generation of `AISC360_Composite_Beam_Audit.md` in Results View.

---

## 🚀 EigenGraph v0.0.7 — Phase 7: Wind Engineering & Computational Aero-Dynamics per ASCE 7-22 / SNI 1727:2020
**Release Date:** 2026-11-06  
**Git Tag:** `v0.0.7`  
**Target:** Production Engine

EigenGraph v0.0.7 introduces comprehensive **Wind Engineering & Computational Aero-Dynamics**, delivering an automated code-compliance and aero-elastic engine per ASCE 7-22 Chapters 26–31 and SNI 1727:2020 for Main Wind Force Resisting Systems (MWFRS), dynamic gust effect factors ($G_f$), along-wind / cross-wind vortex shedding acceleration for human comfort (ISO 10137), and wind vs. seismic governing envelope audits.

### 🌟 Key Features & Mathematical Formulation

#### 1. Terrain Profiles & Velocity Pressure Profiles ($q_z$)
- **Terrain Exposure Categories**: Implemented Exposure B (urban/suburban $z_g = 365.76\text{ m}, \alpha = 7.0$), Exposure C (open grassland $z_g = 274.32\text{ m}, \alpha = 9.5$), and Exposure D (unobstructed coastal $z_g = 213.36\text{ m}, \alpha = 11.5$).
- **Exposure Coefficient $K_z$ & Velocity Pressure $q_z$**:
  $$K_z = 2.01 \left( \frac{\max(z, 4.5)}{z_g} \right)^{2/\alpha}, \quad q_z = 0.613 K_z K_{zt} K_d K_e V^2 \; (\text{Pa})$$
- Evaluates directional wind pressure factoring wind directionality $K_d = 0.85$, topographic factor $K_{zt}$, and ground elevation factor $K_e$.

#### 2. Dynamic Gust Effect Factor ($G_f$) for Flexible High-Rise Structures
- **Background Response Factor ($Q$) & Resonant Factor ($R$)**:
  $$Q = \sqrt{ \frac{1}{1 + 0.63 \left( \frac{B + h}{L_{\bar{z}}} \right)^{0.63}} }, \quad R = \sqrt{ \frac{1}{\beta} R_n R_h R_b (0.53 + 0.47 R_L) }$$
- Unified formulation automatically switching between rigid structures ($G = 0.85$ for $f_1 \ge 1\text{ Hz}$) and flexible resonant high-rises ($f_1 < 1\text{ Hz}$) per ASCE 7-22 Section 26.11.

#### 3. MWFRS Directional Procedure & Multi-Story Shear Aggregation
- **Windward & Leeward Pressures**:
  $$p_{\text{windward}} = q_z G C_p \; (C_p = 0.80), \quad p_{\text{leeward}} = q_h G C_p \; (C_p = -0.50 \text{ to } -0.30)$$
- **Enclosure Internal Pressure**: Incorporates $GC_{pi} = \pm 0.18$ (Enclosed) and $\pm 0.55$ (Partially Enclosed).
- **Story Force & Overturning Moment**:
  $$F_i = p_{\text{net}, i} \cdot A_{\text{trib}, i}, \quad V_w(z) = \sum_{j=i}^{N} F_j, \quad M_{\text{OTM}} = \sum_{i=1}^{N} F_i z_i$$

#### 4. Human Occupant Dynamic Comfort (ISO 10137 / ASCE 7-22 CC)
- **Along-Wind Acceleration**: Peak acceleration $a_{\text{along}}$ formulated from background and resonant energy under 10-year MRI wind.
- **Cross-Wind Vortex Shedding**: Evaluates vortex shedding dynamic acceleration based on reduced velocity $V_r = \frac{V}{f_1 B}$.
- **Comfort Verification**: Compares resultant building top acceleration $a_{\text{res}} = \sqrt{a_{\text{along}}^2 + a_{\text{cross}}^2}$ against the ISO 10137 residential/commercial comfort threshold of $20\text{ milli-g}$.

#### 5. Interactive Desktop UI & Official Dossier Export
- **Sub-Tab `🌪️ ASCE 7 Wind`**: Dual-vector visualizer rendering elevation velocity pressure profiles, building silhouette, and dynamic comfort acceleration gauges.
- **Wind Engineering Dossier**: One-click generation of `ASCE7_Wind_Engineering_Dossier.md` via Results View export suite.

---

## 🚀 EigenGraph v0.0.6 — Phase 6: Steel Connection Design & AISC 360-16 / SNI 1729:2020 Limit States Engine
**Release Date:** 2026-11-06  
**Git Tag:** `v0.0.6`  
**Target:** Production Engine

EigenGraph v0.0.6 introduces comprehensive **Structural Steel Connection Design & Limit States Verification**, providing automated code compliance checks per AISC 360-16 / AISC 360-22, SNI 1729:2020, and AISC 358-16 / SNI 7860:2020 for moment and shear connections.

### 🌟 Key Features & Mathematical Formulation

#### 1. AISC 360-16 & SNI 1729:2020 Limit States Suite
- **High-Strength Bolt Shear Rupture**: Sizing for ASTM A325-N/X, ASTM A490-N/X, and ISO Class 8.8 / 10.9 fasteners per Table J3.2 ($\phi R_n = \phi \cdot n_b \cdot F_{nv} A_b$).
- **Bolt Bearing & Tearout (Section J3.10)**: Clear edge distance $l_c$ tracking with governing limit $R_n = \min(1.2 l_c t F_u, 2.4 d_b t F_u)$ ($\phi = 0.75$).
- **Block Shear Rupture (Section J4.3 / SNI 1729 Section J4.3)**:
  $$R_n = \min \left( 0.60 F_u A_{nv} + U_{bs} F_u A_{nt}, \; 0.60 F_y A_{gv} + U_{bs} F_u A_{nt} \right) \quad (\phi = 0.75, \; U_{bs} = 1.0)$$
- **Plate Gross Shear Yielding & Net Shear Rupture**: Gross yield ($\phi R_n = 1.0 \cdot 0.60 F_y A_g$) and net fracture ($\phi R_n = 0.75 \cdot 0.60 F_u A_{nv}$).
- **Double Fillet Weld Shear Resistance**: Weld throat $t_e = 0.707 w$ with E70XX filler metal ($R_n = 0.60 F_{EXX} A_{we}$).

#### 2. AISC 358-16 4E Extended End-Plate Moment Connection
- **Yield-Line Mechanism**:
  $$Y_p = \frac{b_p}{2} \left[ h_1 \left(\frac{1}{p_{fi}} + \frac{1}{s}\right) + h_0 \left(\frac{1}{p_{fo}}\right) \right] + \frac{2}{g} \left[ h_1 (p_{fi} + s) + h_0 p_{fo} \right]$$
- **Flexural Moment Capacity**: $\phi M_n = \phi_b F_{yp} t_p^2 Y_p$ ($\phi_b = 0.90$).
- **Flange Tension Bolt Group**: Evaluates tension demands across 4 critical bolts with prying action checks.

#### 3. Column Panel Zone & Stiffener Audit (AISC 360 Section J10)
- **Panel Zone Shear Capacity**:
  $$R_n = 0.60 F_y d_c t_w \left[ 1 + \frac{3 b_{cf} t_{cf}^2}{d_b d_c t_w} \right] \quad (\phi = 0.90)$$
- **Web Doubler Plate Sizing**: Automatically computes required doubler plate thickness $t_{\text{doubler}} = \frac{V_{pz} - \phi R_n}{0.60 \phi F_y d_c}$.
- **Web Local Yielding & Crippling**: Sizes column web against concentrated flange forces, triggering transverse continuity plate warnings when required.

#### 4. Interactive Desktop UI & Official Dossier Export
- **Sub-Tab `🔩 Steel Connections`**: Features interactive 2D CAD elevation blueprint with column, beam, end-plate, weld beads, and bolt group circles, alongside an AISC limit state DCR bar chart.
- **Audit Dossier Export**: One-click generation of `AISC360_Steel_Connection_Audit.md` via the Results View export suite.

---

## 🚀 EigenGraph v0.0.5 — Phase 5: Geotechnical Engineering, Deep Foundations & Soil-Structure Interaction (SSI)
**Release Date:** 2026-11-06  
**Git Tag:** `v0.0.5`  
**Target:** Production Engine

EigenGraph v0.0.5 introduces full-stack **Geotechnical Engineering, Deep Foundation Analytics, and Dynamic Soil-Structure Interaction (SSI)**, bridging the critical gap between super-structure structural analysis and sub-structure soil foundations in accordance with SNI 2847:2019, SNI 1726:2019, ACI 318-19, and ASCE 7-22 Chapter 19.

### 🌟 Key Features & Mathematical Formulation

#### 1. Winkler Subgrade Reaction & 6-DOF Foundation Spring Matrix
- **Bowles / Vesic Elastic Formulation**: Converts soil stratigraphy subgrade modulus $k_s$ into equivalent 6-DOF uncoupled spring stiffnesses for spread footings and mat foundations:
  - Vertical translation: $K_z = k_s \cdot B \cdot L$ (kN/m)
  - Horizontal shear translation: $K_x = K_y = 0.82 \cdot K_z$ (kN/m)
  - Rocking rotation about primary axes: $K_{\theta x} = k_s \cdot \frac{B \cdot L^3}{12}$, $K_{\theta y} = k_s \cdot \frac{L \cdot B^3}{12}$ (kNm/rad)
  - Torsional rotation about vertical axis: $K_{\theta z} = 0.65 \cdot k_s \cdot (I_x + I_y)$ (kNm/rad)
- **Soil Stratum Library**: Pre-configured parameters for 7 geotechnical profiles (Dense Sand, Medium Sand, Loose Sand, Stiff Clay, Medium Clay, Soft Clay, Bedrock).

#### 2. Deep Foundation Pile Group Converse-Labarre Efficiency & Biaxial Distribution
- **Converse-Labarre Group Efficiency ($E_g$)**:
  $$E_g = 1 - \theta \cdot \frac{(n_x - 1)n_y + (n_y - 1)n_x}{90 \cdot n_x \cdot n_y}, \quad \text{where } \theta = \arctan(d / s)$$
- **Biaxial Eccentric Load Distribution**: Calculates individual pile vertical reaction forces $P_i$ under combined axial load $P_u$ and moments $M_{ux}, M_{uy}$:
  $$P_i = \frac{P_u}{N} \pm \frac{M_{uy} \cdot x_i}{\sum x^2} \pm \frac{M_{ux} \cdot y_i}{\sum y^2}$$
- **Individual Pile Capacity & Deflection**: Verifies compressive, tensile uplift, and lateral shear head deflection $\delta_{\text{lat}}$ for every pile in the group.

#### 3. Two-Way Concrete Punching Shear Audit (SNI 2847:2019 / ACI 318-19 Section 22.6)
- **Critical Perimeter ($b_o$)**: Evaluated at distance $d/2$ from column face ($b_o = 2(c_1 + d) + 2(c_2 + d)$).
- **Nominal Concrete Shear Capacity ($V_c$)**:
  $$V_c = \min \left\{ 0.33 \sqrt{f'_c}, \; 0.17 \left(1 + \frac{2}{\beta}\right) \sqrt{f'_c}, \; 0.083 \left(2 + \frac{\alpha_s d}{b_o}\right) \sqrt{f'_c} \right\} \cdot b_o d$$
- **Unbalanced Moment Transfer**: Integrates polar moment of inertia $J_c$ and eccentric shear stress fraction $\gamma_v = 1 - \gamma_f$ into total factored shear stress $\nu_u$.

#### 4. Veletsos & Meek Dynamic Soil-Structure Interaction (SSI)
- **Modal Period Lengthening**:
  $$\left(\frac{\tilde{T}}{T}\right)^2 = 1 + \frac{K_{\text{fixed}}}{K_h} + \frac{K_{\text{fixed}} h^2}{K_r}$$
- **ASCE 7-22 Chapter 19 Damping & Base Shear Reduction**: Accounts for radiation and hysteretic soil damping $\beta_{\text{ssi}}$ to reduce design seismic base shear while verifying lateral drift amplification.

#### 5. Interactive Desktop UI & Official Dossier Export
- **Sub-Tab `🌱 Geotech & SSI`**: Interactive dual visualizer rendering the 2D pile cap layout with axial stress gradients alongside the critical punching shear perimeter.
- **Audit Dossier Export**: Generates `Geotechnical_SSI_Foundation_Audit.md` directly via the export button in the Results pillar.

---

## 🚀 EigenGraph v0.0.4 — Phase 4: Advanced Seismic & Pushover Analysis (ASCE 41-17 / ATC-40)
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

## 🚀 EigenGraph v0.0.3 — Phase 3: Core Upgrade (Shell FEA, P-Delta & IFC4 SAM)
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

## ⚡ EigenGraph v0.0.2 — Phase 2: Quick Wins (ETABS Interop, PBG/SIMBG & Rebar BBS)
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

## 🏛️ EigenGraph v0.0.1 — Phase 1: Structural Mechanics & Non-Linear Core Engine
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
