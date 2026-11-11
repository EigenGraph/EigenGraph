# 🏛️ EigenGraph: Master Engineering Specification (Phases 22 – 50)
## Implementation Blueprint for Versions `v0.2.2` through `v0.5.0`

---

## Document Overview

This engineering specification establishes the formal, step-by-step technical roadmap for **Phases 22 through 50** of the EigenGraph ecosystem. Every phase adheres strictly to the **1-to-1 Phase-to-Semantic Version Mapping Rule** (`v0.2.2` to `v0.5.0`).

Under this architecture:
* **Phase 22 (`v0.2.2`)** establishes **The Master Shell & 15-Pillar High-Fidelity Layout** (the complete Apple-inspired UI container, sidebar hierarchy, and high-fidelity mock views across all 15 modules).
* **Phases 23 through 50 (`v0.2.3` – `v0.5.0`)** systematically populate the underlying geometric, computational, geotechnical, simulation, and publishing engines into the already-established UI containers.

---

## 🏛️ PHASE 22 (`v0.2.2`): The Master Shell & 15-Pillar High-Fidelity Layout

* **Target Subsystem:** `src/app.rs`, `src/models.rs`, `src/views/`
* **Objective:** Construct the definitive, locked macOS App Store-style sidebar and instantiate high-fidelity UI containers for all 15 navigation pillars.
* **Navigation Hierarchy (Apple Minimalist HIG):**
  1. `📁 My Library` ➔ Local Obsidian-style Vault container (project cards, SNI/AISC regulatory records).
  2. `🤖 AI Agent` ➔ Virtual Engineering Firm container (Organigram: Manager, Drafter, Analyst, QS + Chat Board).
  3. `💡 Research` ➔ Interactive GIS map container + coordinate telemetry HUD.
  4. `📐 Parametric` ➔ AutoCAD-style 2D drafting canvas container + 3D live extrusion companion.
  5. `🏗️ Geometry` ➔ Clean BIM assembly container (hidden ground, multi-level $Z$-planes, grid network).
  6. `⚡ Analysis` ➔ 3D Geo-Structural sandbox container on GIS terrain + X-Ray ground slider + hazard buttons.
  7. `⏱️ Time & Scheduling` ➔ 4D Phased erection timeline slider and Gantt view container.
  8. `💰 Cost & Budgeting` ➔ 5D 3D Cost Heatmap canvas and dynamic BoQ panel.
  9. `🌱 Sustainability & Energy` ➔ 6D Solar heliodon sun-path dome and embodied carbon gradient.
  10. `🏢 Facility Management` ➔ 7D Digital Twin asset pins and maintenance telemetry log.
  11. `🛡️ Safety & Health` ➔ 8D Tower crane swing radius arc and slab edge fall hazard boundary.
  12. `♻️ Lean Construction` ➔ 9D Material staging/laydown layout map and concrete mixer routing.
  13. `🤖 Industrialized & Automation` ➔ 10D DfMA precast modular transport envelope check and crane rigging.
  14. `🔮 Prediction` ➔ AI structural stress concentration and drift anomaly prediction canvas.
  15. `📑 Report` ➔ Living Publishing Suite container: Docs (DED + title block), Sheets (BoQ formulas), Slides.
* **Rust Architecture & Structures:**
  ```rust
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
  ```
* **Definition of Done (DoD):**
  - Seamless switching between all 15 pillars at $\ge 60\text{ FPS}$ with consistent Apple-style theme and typography.
  - Zero UI clipping, full-height macOS sidebar geometry, and dedicated view containers rendered for every pillar.

---

## 🌐 GROUP 1: GIS Site Inception & Geotechnical Foundation (`Research`)

---

### Phase 23 (`v0.2.3`) — `walkers` & Slippy Map Engine Integration
* **Target Module:** `src/views/research.rs` / `src/core/gis/map_widget.rs`
* **Theoretical Foundation:** Web Mercator projection (EPSG:3857), slippy map quadtree tiling system ($2^z \times 2^z$ tiles at zoom level $z$).
* **Rust Architecture & Structures:**
  ```rust
  pub struct SlippyMapState {
      pub center_lat_lon: (f64, f64),
      pub zoom_level: f32, // 0.0 to 19.0
      pub tile_provider: TileSource, // OpenStreetMap, Mapbox Satellite
      pub tile_cache: std::collections::HashMap<(u32, u32, u8), egui::TextureHandle>,
      pub active_pin: Option<(f64, f64)>,
  }
  ```
* **Implementation Steps:**
  1. Embed the pure-Rust `walkers` slippy map widget into the Phase 22 `Research` container.
  2. Implement an asynchronous tile-fetcher with local caching under `assets/cache/tiles/`.
  3. Render an interactive crosshair and coordinate telemetry HUD (Latitude, Longitude, Zoom).
* **Definition of Done (DoD):**
  - Continuous pan and smooth zoom inside `ResearchConceptView` at 60 FPS; clicking drops a geodetic site pin.

---

### Phase 24 (`v0.2.4`) — Geodetic Projection Engine (`proj4rs`)
* **Target Module:** `src/core/gis/crs_projection.rs`
* **Theoretical Foundation:** Coordinate Reference System (CRS) transformation between WGS84 Geodetic ($\phi, \lambda$) and Universal Transverse Mercator (UTM Zones 46N–54S) based on the WGS84 ellipsoid.
* **Rust Architecture & Structures:**
  ```rust
  pub struct GeoProjectionEngine {
      pub source_crs: String, // "EPSG:4326" (WGS84)
      pub target_utm_zone: u8, // e.g., 48 for West Java
      pub is_southern_hemisphere: bool,
      pub meridian_convergence_deg: f64,
  }
  impl GeoProjectionEngine {
      pub fn wgs84_to_utm(&self, lat: f64, lon: f64) -> (f64, f64, f64); // Easting, Northing, Convergence
      pub fn utm_to_wgs84(&self, easting: f64, northing: f64) -> (f64, f64);
  }
  ```
* **Implementation Steps:**
  1. Embed pure-Rust `proj4rs` routines (zero C++ dependencies).
  2. Auto-detect UTM zones based on pinpointed site longitude: $\text{Zone} = \lfloor(\lambda + 180)/6\rfloor + 1$.
  3. Calculate True North rotation angle $\theta$ (meridian convergence $\gamma = \Delta\lambda \sin\phi$) to orient local BIM Project North.
* **Definition of Done (DoD):**
  - Monas Jakarta benchmark ($-6.1754^\circ, 106.8272^\circ$) transforms to UTM Zone 48S ($\approx 702220\text{ m E}, 9317070\text{ m N}$) with error $\le 1.0\text{ mm}$.

---

### Phase 25 (`v0.2.5`) — Digital Elevation Model (DEM) & Topographic TIN Mesh
* **Target Module:** `src/core/gis/tin_surface.rs`
* **Theoretical Foundation:** 2D Delaunay Triangulation maximizing minimum interior angles generating a Triangulated Irregular Network (TIN) surface from raster DEM elevation points.
* **Rust Architecture & Structures:**
  ```rust
  pub struct TinPoint {
      pub id: usize,
      pub x: f32, // Local Easting offset (m)
      pub y: f32, // Local Northing offset (m)
      pub z: f32, // Elevation (m MSL)
  }
  pub struct TinSurfaceMesh {
      pub points: Vec<TinPoint>,
      pub triangles: Vec<[usize; 3]>,
      pub min_elevation: f32,
      pub max_elevation: f32,
  }
  ```
* **Implementation Steps:**
  1. Ingest raster elevation grids (SRTM / DEMNAS GeoTIFF or synthetic point clouds).
  2. Generate 2D Delaunay triangulation into a 3D ground mesh.
  3. Generate iso-elevation contour polylines at selectable intervals ($1.0\text{ m}, 5.0\text{ m}$) with slope hazard shading ($> 30^\circ$).
* **Definition of Done (DoD):**
  - Render a $500\text{ m} \times 500\text{ m}$ terrain mesh ($10,000+$ triangles) at smooth 60 FPS in the preview viewport.

---

### Phase 26 (`v0.2.6`) — Geotechnical Boreholes & Multi-Hazard PusGeN Ingestion
* **Target Module:** `src/core/geotech/borehole_db.rs` / `src/core/gis/hazard_maps.rs`
* **Theoretical Foundation:** USCS soil stratigraphy classification, $N\text{-SPT}$ overburden correction ($N_{60}$). Seismic design spectrum per SNI 1726:2019 / ASCE 7-22 ($S_s, S_1, F_a, F_v, S_{DS}, S_{D1}$).
* **Rust Architecture & Structures:**
  ```rust
  pub struct SoilLayer {
      pub depth_top: f32,
      pub depth_bottom: f32,
      pub classification: String, // "Soft Clay", "Medium Sand", "Hard Rock"
      pub unit_weight_gamma: f32,
      pub n_spt: u32,
  }
  pub struct SeismicHazardProfile {
      pub site_class: char,
      pub s_s: f32,
      pub s_1: f32,
      pub s_ds: f32,
      pub s_d1: f32,
      pub spectrum_curve: Vec<(f32, f32)>, // (T in sec, Sa in g)
  }
  ```
* **Implementation Steps:**
  1. Add borehole logging UI (BH-1, BH-2, CPT) with stratigraphy and $N\text{-SPT}$ curves.
  2. Implement PusGeN seismic acceleration database lookup based on site coordinates.
  3. Generate continuous response spectrum curve $S_a(T)$ and cadastral setbacks (GSB, KDB, KLB).
* **Definition of Done (DoD):**
  - Pinning Jakarta coordinates auto-populates $S_s \approx 0.72\text{g}$, $S_1 \approx 0.38\text{g}$, and calculates Site Class D spectrum.

---

## 📐 GROUP 2: The Freeform Component Studio (`Parametric`)

---

### Phase 27 (`v0.2.7`) — AutoCAD-Style 2D Freehand Vector Drafting Canvas & Snaps
* **Target Module:** `src/views/parametric.rs` / `src/core/cad/canvas2d.rs`
* **Theoretical Foundation:** Planar vector primitives, mouse-driven polyline construction, magnetic nearest-neighbor snap algorithms.
* **Rust Architecture & Structures:**
  ```rust
  pub enum CadPrimitive2D {
      Line { start: [f32; 2], end: [f32; 2] },
      Polyline { points: Vec<[f32; 2]>, closed: bool },
      Arc { center: [f32; 2], radius: f32, start_angle: f32, end_angle: f32 },
      Circle { center: [f32; 2], radius: f32 },
  }
  pub struct CadCanvas2DState {
      pub primitives: Vec<CadPrimitive2D>,
      pub active_tool: CadTool,
      pub snap_modes: CadSnapSettings, // End, Mid, Cen, Int, Ortho F8
      pub cursor_world_pos: [f32; 2],
  }
  ```
* **Implementation Steps:**
  1. Populate the Phase 22 `Parametric` container with an infinite 2D vector drafting canvas.
  2. Implement magnetic snap indicators (green square EndPoint, triangle MidPoint, hourglass Intersection).
  3. Support Ortho mode (F8 toggle) for pure horizontal/vertical constraints.
* **Definition of Done (DoD):**
  - Draw a closed I-beam cross-section with fillet corners using vector snaps without constraint errors.

---

### Phase 28 (`v0.2.8`) — Real-Time Synchronous 3D Solid Extrusion Companion View
* **Target Module:** `src/views/parametric/companion3d.rs`
* **Theoretical Foundation:** Planar polygon orientation check (Shoelace formula). Normal vector extrusion into closed 3D triangular B-Rep.
* **Rust Architecture & Structures:**
  ```rust
  pub struct ExtrudedMesh3D {
      pub profile_poly: Vec<[f32; 2]>,
      pub extrusion_length: f32,
      pub vertices: Vec<[f32; 3]>,
      pub indices: Vec<[u32; 3]>,
  }
  ```
* **Implementation Steps:**
  1. Detect closed loops in the 2D drafting canvas using planar cycle traversal.
  2. Triangulate capping polygons (ear-clipping) and generate quad-strip side walls.
  3. Render the resulting 3D solid companion view in real time beside the 2D canvas.
* **Definition of Done (DoD):**
  - Moving any vertex in 2D updates the 3D companion extrusion in $< 16\text{ ms}$ (60 FPS real-time sync).

---

### Phase 29 (`v0.2.9`) — Parametric Cross-Sections & Family Formulas
* **Target Module:** `src/core/cad/parametric_families.rs`
* **Theoretical Foundation:** Section property integral calculus: Area $A = \int dA$, Centroid $\bar{y}$, Moment of Inertia $I_x = \int y^2 dA$, Section Modulus $S_x$, Plastic Modulus $Z_x$.
* **Rust Architecture & Structures:**
  ```rust
  pub enum ParametricFamilyTemplate {
      WideFlange { d: f32, bf: f32, tf: f32, tw: f32, r: f32 },
      BoxGirder { h: f32, b: f32, t_web: f32, t_flange: f32 },
      PrecastIGirder { h: f32, b_top: f32, b_bot: f32, t_web: f32 },
  }
  pub struct CrossSectionProperties {
      pub area_cm2: f32,
      pub ix_cm4: f32,
      pub iy_cm4: f32,
      pub weight_kg_per_m: f32,
  }
  ```
* **Implementation Steps:**
  1. Implement live parameter sliders (Beam Depth $D$, Flange Width $B_f$, Web Thickness $t_w$).
  2. Compute real-time section properties via closed-form analytical equations.
  3. Add AISC 360-16 compact section checks ($b/t \le 0.38 \sqrt{E/F_y}$).
* **Definition of Done (DoD):**
  - Selecting standard WF 400x200x8x13 outputs exact catalog properties within $0.1\%$ error.

---

### Phase 30 (`v0.3.0`) — Micro DED Detailing Engine & Asset Packaging (`.egcomp`)
* **Target Module:** `src/core/cad/rebar_detailer.rs` / `src/core/cad/asset_catalog.rs`
* **Theoretical Foundation:** ACI 318 / SNI 2847 standard hooks: $135^\circ$ seismic hook with $6d_b$ extension, bend deduction algorithms. Binary serialization of vector entities.
* **Rust Architecture & Structures:**
  ```rust
  #[derive(serde::Serialize, serde::Deserialize)]
  pub struct ExportableComponentAsset {
      pub asset_uuid: String,
      pub asset_name: String, // e.g., "PT_Girder_30m"
      pub category: ComponentCategory,
      pub cross_section_poly: Vec<[f32; 2]>,
      pub rebar_schedule: Vec<RebarBarMark>,
      pub thumbnail_png_base64: String,
  }
  ```
* **Implementation Steps:**
  1. Add freeform rebar detailing: longitudinal bars, closed stirrups with $135^\circ$ seismic hooks.
  2. Implement steel connection detailing: bolt hole matrices and weld symbols.
  3. Build one-click asset packaging saving components into the project asset palette for `Geometry`.
* **Definition of Done (DoD):**
  - Saved components appear instantly with thumbnail previews in the `Geometry` component palette.

---

## 🏗️ GROUP 3: Clean BIM Spatial Assembly (`Geometry`)

---

### Phase 31 (`v0.3.1`) — Zero-Occlusion Multi-Level Elevation Workspace ($Z$-Planes)
* **Target Module:** `src/views/build/elevation_manager.rs`
* **Theoretical Foundation:** Orthogonal $Z$-plane spatial slicing. Separate datum systems for superstructure ($Z \ge 0.00\text{m}$) and substructure ($Z < 0.00\text{m}$).
* **Rust Architecture & Structures:**
  ```rust
  pub struct BuildingLevelPlane {
      pub id: usize,
      pub name: String, // e.g., "Level 2 (Fl. +4.20m)"
      pub elevation_m: f32,
      pub is_subsurface: bool, // true for pile caps, basements
  }
  ```
* **Implementation Steps:**
  1. Implement an interactive level editor on the left sidebar: add/edit floor elevations.
  2. Suppress terrain visualization in `Geometry` by default for $100\%$ clean sightlines to subsurface pile caps.
  3. Add a *"Level Isolation"* toggle to view a single floor slice at a time.
* **Definition of Done (DoD):**
  - Switching between levels slides the modeling camera and grid plane to the exact elevation datum ($Z$).

---

### Phase 32 (`v0.3.2`) — Structural Grid Network ($A\text{-}B\text{-}C$, $1\text{-}2\text{-}3$ Snapping)
* **Target Module:** `src/views/build/grid_system.rs`
* **Theoretical Foundation:** Structural grid intersection matrices. 2D snapping along Cartesian lines ($X_i, Y_j$) and Radial rays ($R_i, \theta_j$).
* **Rust Architecture & Structures:**
  ```rust
  pub struct StructuralGridSystem {
      pub grid_lines_x: Vec<GridLine>,
      pub grid_lines_y: Vec<GridLine>,
      pub intersection_points: Vec<([f32; 3], String)>, // (Pos, "A-1")
  }
  ```
* **Implementation Steps:**
  1. Render infinite dashed grid lines with circular bubble callouts.
  2. Auto-compute all intersection points $(X_i, Y_j, Z_{\text{active}})$ tagged as `Grid A-1`, `Grid B-2`.
  3. Implement magnetic 3D snapping targeting grid intersections when placing columns or footings.
* **Definition of Done (DoD):**
  - Modifying Grid "2" spacing from $6.0\text{ m}$ to $7.5\text{ m}$ instantly moves all connected columns and beams.

---

### Phase 33 (`v0.3.3`) — Asset Instantiation Engine (`[+ Add Component]` Stamping)
* **Target Module:** `src/views/build/assembly_engine.rs`
* **Theoretical Foundation:** 3D Affine transformation matrices $[T] = [R][S][P]$ mapping local component coordinates into global world BIM coordinates.
* **Rust Architecture & Structures:**
  ```rust
  pub struct PlacedInstanceBim {
      pub instance_uuid: String,
      pub asset_ref_id: String,
      pub start_node: [f32; 3],
      pub end_node: [f32; 3],
      pub rotation_roll_deg: f32,
  }
  ```
* **Implementation Steps:**
  1. Wire the `[+ Add Component]` palette into the viewport click-and-place loop.
  2. Implement single-click column placement on grid intersections and two-click beam framing.
  3. Implement boundary polygon extrusion for reinforced concrete floor slabs ($t = 120\text{ mm} - 250\text{ mm}$).
* **Definition of Done (DoD):**
  - User can assemble a 3-bay $\times$ 2-bay 2-story building frame with pre-drafted components in $< 60$ seconds.

---

### Phase 34 (`v0.3.4`) — Coordination Sleeves & Dynamic DED Cuts ($A\text{-}A$)
* **Target Module:** `src/core/bim/clash_sleeve.rs` / `src/views/build/section_cut.rs`
* **Theoretical Foundation:** Sutherland-Hodgman polygon clipping against arbitrary planes. AISC Design Guide 2 beam web opening reinforcing rules.
* **Rust Architecture & Structures:**
  ```rust
  pub struct BeamWebOpening {
      pub beam_instance_id: String,
      pub distance_from_start_m: f32,
      pub opening_dia_mm: f32,
      pub requires_stiffener_ring: bool,
  }
  pub struct Ded2DDrawingSheet {
      pub cut_polygons: Vec<Vec<[f32; 2]>>,
      pub level_callouts: Vec<(f32, String)>,
      pub dimension_chains: Vec<CadPrimitive2D>,
  }
  ```
* **Implementation Steps:**
  1. Detect clashes between structural beams and MEP route placeholders, auto-generating reinforced web sleeves.
  2. Draw interactive section lines across the building plan ($A\text{-}A, B\text{-}B$).
  3. Slice 3D solid elements intersecting the plane, generating filled cross-section hatches.
* **Definition of Done (DoD):**
  - Clicking *"Generate Section A-A"* produces a crisp, fully hatched 2D cross-sectional view ready for `Report -> Docs`.

---

## ⚡ GROUP 4: Subsurface Geotechnics & Structural Solver (`Analysis` Part 1)

---

### Phase 35 (`v0.3.5`) — Building-on-Terrain Geo-Referencing & Boolean Cut & Fill
* **Target Module:** `src/views/analyze/terrain_embed.rs`
* **Theoretical Foundation:** Constructive Solid Geometry (CSG) Boolean difference: $\mathcal{V}_{\text{excavation}} = \mathcal{V}_{\text{basement}} \setminus \mathcal{V}_{\text{terrain}}$. Prismoidal volumetric integration.
* **Rust Architecture & Structures:**
  ```rust
  pub struct EarthworkVolumeSummary {
      pub total_cut_volume_m3: f64,
      pub total_fill_volume_m3: f64,
      pub net_balance_m3: f64,
      pub excavation_polygon: Vec<[f32; 3]>,
  }
  ```
* **Implementation Steps:**
  1. Project the assembled BIM model from `Geometry` onto the actual GIS terrain mesh from `Research`.
  2. Subtract basement and pile-cap volumes from the terrain surface mesh.
  3. Compute Cut and Fill volumes in cubic meters ($m^3$).
* **Definition of Done (DoD):**
  - A $20\text{ m} \times 30\text{ m}$ basement embedded $3.5\text{ m}$ into flat terrain outputs $V_{\text{cut}} = 2100.0\text{ m}^3 \pm 1\%$.

---

### Phase 36 (`v0.3.6`) — Subsurface Inspection Tools (X-Ray & Dynamic Slicer)
* **Target Module:** `src/views/analyze/subsurface_inspection.rs`
* **Theoretical Foundation:** GPU alpha-blended transparency sorting; depth-buffer write masking for transparent geometry. Geometric 3D axis-aligned clipping planes.
* **Rust Architecture & Structures:**
  ```rust
  pub struct SubsurfaceInspectionState {
      pub ground_opacity: f32, // 0.0 (transparent) to 1.0 (opaque)
      pub enable_clipping_box: bool,
      pub clip_plane_x: Option<f32>,
      pub clip_plane_y: Option<f32>,
  }
  ```
* **Implementation Steps:**
  1. Add an interactive `Ground Opacity (0% - 100%)` slider to the analysis toolbar.
  2. Add an interactive 3D clipping slicer with draggable arrows to cut the ground in half.
  3. Render colored vertical strata bands along the cut soil wall reflecting borehole data.
* **Definition of Done (DoD):**
  - At $30\%$ opacity, underground bored piles, pile caps, and bedrock socketing are fully visible while retaining the ground surface profile.

---

### Phase 37 (`v0.3.7`) — Geotechnical Bearing Capacity (Shallow Footings & Deep Piles)
* **Target Module:** `src/core/geotech/bearing_capacity.rs`
* **Theoretical Foundation:** 
  1. Terzaghi / Meyerhof General Bearing Capacity: $q_{\text{ult}} = c' N_c s_c + q N_q s_q + \frac{1}{2} \gamma B N_\gamma s_\gamma$.
  2. Deep foundation axial capacity: $Q_{\text{ult}} = q_p A_b + \sum (\alpha c_u) A_s$.
  3. Converse-Labarre pile group efficiency: $\eta = 1 - \theta \frac{(n-1)m + (m-1)n}{90mn}$.
* **Rust Architecture & Structures:**
  ```rust
  pub struct FootingCapacityResult {
      pub q_ult_kpa: f32,
      pub q_allowable_kpa: f32,
      pub applied_stress_kpa: f32,
      pub dcr_soil: f32,
  }
  pub struct PileCapacityResult {
      pub pile_mark: String,
      pub q_skin_kn: f32,
      pub q_end_kn: f32,
      pub q_total_allowable_kn: f32,
      pub applied_p_axial_kn: f32,
  }
  ```
* **Implementation Steps:**
  1. Query support reactions ($P, M_x, M_y$) from superstructure columns.
  2. Calculate allowable bearing pressure for shallow spread footings.
  3. Integrate skin friction along soil strata for deep bored piles.
  4. Color-code footings: Green if $\sigma_{\text{applied}} \le q_a$, Red if overstressed.
* **Definition of Done (DoD):**
  - A $2\text{ m} \times 2\text{ m}$ footing on clay ($c = 50\text{ kPa}, \gamma = 18\text{ kN/m}^3$) yields $q_{\text{ult}}$ matching manual calculation within $0.5\%$.

---

### Phase 38 (`v0.3.8`) — Soil Settlement & Hillside Slope Stability Solvers
* **Target Module:** `src/core/geotech/settlement_slope.rs`
* **Theoretical Foundation:** Primary Consolidation Settlement: $S_c = \frac{C_c H}{1 + e_0} \log_{10}\left(\frac{\sigma'_{v0} + \Delta\sigma}{\sigma'_{v0}}\right)$. Simplified Bishop Method of Slices for circular slope slip surface.
* **Rust Architecture & Structures:**
  ```rust
  pub struct DifferentialSettlementReport {
      pub max_settlement_mm: f32,
      pub angular_distortion: f32, // delta / L
      pub risk_category: SettlementRiskLevel,
  }
  pub struct SlopeStabilityResult {
      pub factor_of_safety: f32,
      pub critical_slip_center: [f32; 2],
      pub critical_radius: f32,
  }
  ```
* **Implementation Steps:**
  1. Compute stress distribution beneath foundations using Boussinesq $2:1$ approximation.
  2. Compute consolidation settlement per column footing and evaluate angular distortion $\beta = \Delta/L$.
  3. Search for critical slip circles along cut embankments using the Bishop slice method.
* **Definition of Done (DoD):**
  - Warns in amber if angular distortion exceeds the Eurocode / SNI limit of $\beta > 1/500$.
  - Slope Factor of Safety ($FS$) correctly evaluates whether hillside terrain requires retaining walls ($FS < 1.50$).

---

### Phase 39 (`v0.3.9`) — Superstructure FEA, SVD Kinematic Guard & ARI Solver
* **Target Module:** `src/core/solver/fea_ari_core.rs`
* **Theoretical Foundation:** 3D linear elastic frame element stiffness matrix $[k_e]$ ($12 \times 12$). Global assembly $[K]\{u\} = \{F\}$. Singular Value Decomposition (SVD): $[K_{ff}] = [U] [\Sigma] [V]^T$. Adjusted Redundancy Index: $\text{ARI}_e = \text{RR}_e \times (1.0 - U_e^*)$.
* **Rust Architecture & Structures:**
  ```rust
  pub struct FeaAnalysisResults {
      pub member_end_forces: std::collections::HashMap<String, [f32; 12]>,
      pub max_displacement_mm: f32,
      pub svd_min_singular_value: f64,
      pub is_kinematic_mechanism: bool,
      pub member_dcr: std::collections::HashMap<String, f32>,
      pub global_ari_score: f32,
  }
  ```
* **Implementation Steps:**
  1. Assemble global sparse stiffness matrix from beam, column, and truss elements.
  2. Factorize via Cholesky decomposition ($LL^T$) to solve for nodal displacements.
  3. Perform SVD on the unconstrained submatrix to detect nullspace collapse mechanisms ($\sigma_{\min} = 0$).
  4. Render 3D Bending Moment Diagrams (BMD) and colorize members by Demand-Capacity Ratio (DCR).
* **Definition of Done (DoD):**
  - Statically unstable truss correctly identified with $\sigma_{\min} = 0.000$ triggering Gate 1 SVD warning.
  - Simply-supported 6m WF beam under $20\text{ kN/m}$ load evaluates peak moment $M_{\max} = 90.0\text{ kNm} \pm 0.01\%$.

---

## 🌪️ GROUP 5: Multi-Hazard World Simulation & Live Tweak (`Analysis` Part 2)

---

### Phase 40 (`v0.4.0`) — Dynamic Seismic Ground Motion Propagation (Shake Table)
* **Target Module:** `src/views/analyze/seismic_simulation.rs`
* **Theoretical Foundation:** Dynamic equation of motion: $[M]\{\ddot{u}\} + [C]\{\dot{u}\} + [K]\{u\} = -[M]\{r\}\ddot{u}_g(t)$. Direct numerical integration via Newmark-$\beta$ method ($\beta = 1/4, \gamma = 1/2$).
* **Rust Architecture & Structures:**
  ```rust
  pub struct AccelerogramRecord {
      pub record_name: String,
      pub time_step_dt: f32,
      pub ground_accel_series: Vec<f32>,
  }
  pub struct DynamicTimeHistoryPlayback {
      pub current_time_step: usize,
      pub is_playing: bool,
      pub deformed_node_positions: Vec<[f32; 3]>,
      pub interstory_drift_ratios: Vec<f32>,
  }
  ```
* **Implementation Steps:**
  1. Ingest recorded ground motion accelerograms scaled to site $S_{DS}$ / $S_{D1}$.
  2. Run dynamic time-step integration propagating ground shaking from bedrock through foundations into the building.
  3. Animate 3D building sway at 60 FPS with user-controlled Play/Pause/Scrub timeline slider.
  4. Highlight potential plastic hinge locations where moment exceeds plastic capacity ($M > M_p$).
* **Definition of Done (DoD):**
  - Smooth 3D animation showing resonance when earthquake frequency matches building fundamental period ($T \approx T_1$).
  - Real-time display of interstory drift vs. ASCE 7-22 $2.0\%$ code drift limit.

---

### Phase 41 (`v0.4.1`) — Aerodynamic Wind Flow & Tornado Pressure Streamlines
* **Target Module:** `src/views/analyze/wind_streamlines.rs`
* **Theoretical Foundation:** ASCE 7-22 MWFRS velocity pressure equation: $p = q G C_p - q_i (G C_{pi})$. Particle streamline advection through 3D vector velocity field: $\mathbf{x}(t + \Delta t) = \mathbf{x}(t) + \mathbf{v}(\mathbf{x}) \Delta t$.
* **Rust Architecture & Structures:**
  ```rust
  pub struct WindParticle {
      pub pos: [f32; 3],
      pub vel: [f32; 3],
      pub life: f32,
  }
  pub struct FacadeWindPressureZone {
      pub panel_id: usize,
      pub cp_coefficient: f32,
      pub surface_pressure_kpa: f32,
  }
  ```
* **Implementation Steps:**
  1. Generate animated 3D particle streamlines blowing across the GIS terrain and splitting around building facades.
  2. Calculate surface pressure contours: compression on the windward face, high suction on leeward faces and roof edges.
  3. Include a "Tornado Vortex" mode simulating rotational vortex winds with localized suction peaks.
* **Definition of Done (DoD):**
  - Render $1,000+$ flowing particles at 60 FPS with facade color map showing pressure gradients.

---

### Phase 42 (`v0.4.2`) — Hydrological Inundation & Flood Water Level Rise
* **Target Module:** `src/views/analyze/flood_simulation.rs`
* **Theoretical Foundation:** Hydrostatic pressure distribution: $p(z) = \rho_w g (h_w - z)$. Archimedes buoyancy uplift force: $F_{\text{buoyancy}} = \rho_w g \mathcal{V}_{\text{submerged}}$.
* **Rust Architecture & Structures:**
  ```rust
  pub struct FloodSimulationState {
      pub water_level_msl: f32,
      pub total_submerged_volume_m3: f64,
      pub net_buoyancy_force_kn: f64,
      pub basement_float_factor_of_safety: f32,
  }
  ```
* **Implementation Steps:**
  1. Render a dynamic horizontal water plane that rises and falls with a user slider.
  2. Compute flooded topography areas based on the DEM terrain mesh.
  3. Check buoyancy safety factor ($FS = \frac{W_{\text{dead}}}{F_{\text{buoyancy}}} \ge 1.20$) to ensure the building does not float.
* **Definition of Done (DoD):**
  - Basement water ingress warning alerts when water level surpasses basement retaining wall crest.

---

### Phase 43 (`v0.4.3`) — In-Place Live-Tweak Geometry Adaptation Loop
* **Target Module:** `src/views/analyze/live_tweak.rs`
* **Theoretical Foundation:** Reactive dependency graph. Invalidation and re-execution of FEA and geotechnical solvers upon local geometry mutation.
* **Rust Architecture & Structures:**
  ```rust
  pub struct LiveTweakHandle {
      pub selected_member_id: String,
      pub override_section_size: Option<String>,
      pub override_foundation_depth_m: Option<f32>,
  }
  ```
* **Implementation Steps:**
  1. Allow the user to click any structural member or foundation pile directly inside the `Analysis` canvas.
  2. Open an immediate contextual tweak inspector: increase beam depth, add a brace, or deepen pile.
  3. Recalculate FEA and geotechnical checks in background threads without switching away from the simulation viewport.
* **Definition of Done (DoD):**
  - Enlarging an overstressed red beam immediately turns it green ($DCR < 1.0$) on the 3D canvas within $< 200\text{ ms}$.

---

## 🎨 GROUP 6: Specialized Interactive Canvases (BIM Dimensions 4D – 10D)

---

### Phase 44 (`v0.4.4`) — 4D Phased Erection & 5D Cost Heatmap Canvases
* **Target Module:** `src/views/planning/time_cost_canvases.rs`
* **Theoretical Foundation:** Critical Path Method (CPM) construction scheduling linked to spatial geometry. Automated quantity takeoff (QTO) cost modeling ($C_{\text{total}} = \sum V_i \times U_i$).
* **Rust Architecture & Structures:**
  ```rust
  pub struct ErectionPhaseTask {
      pub task_id: String,
      pub day_start: u32,
      pub day_end: u32,
      pub assigned_member_uuids: Vec<String>,
  }
  pub struct MemberCostAttribute {
      pub unit_rate: f64,
      pub total_cost: f64,
      pub cost_density_rank: f32, // 0.0 to 1.0
  }
  ```
* **Implementation Steps:**
  1. In `Time & Scheduling`, provide a 4D construction playback slider: watch excavation dig out, piles appear, columns erect, and slabs cast sequentially.
  2. In `Cost & Budgeting`, shade the 3D model as a cost heatmap: elements glowing red represent highest cost drivers.
* **Definition of Done (DoD):**
  - Scrubbing the 4D slider to Day 30 shows exactly the progress of construction planned for that milestone.
  - Clicking any element on the 5D heatmap allows direct editing of the unit rate ($\text{Rp/unit}$).

---

### Phase 45 (`v0.4.5`) — 6D Solar Heliodon / Carbon & 7D Facility Digital Twin Canvases
* **Target Module:** `src/views/sustainability.rs` / `src/views/facility.rs`
* **Theoretical Foundation:** Solar azimuth & zenith angle calculation based on latitude, longitude, and hour. Embodied carbon coefficient database ($A1-A3$ $\text{kg CO}_2\text{e}/\text{kg}$).
* **Rust Architecture & Structures:**
  ```rust
  pub struct SolarHeliodonState {
      pub day_of_year: u16,
      pub hour_of_day: f32,
      pub sun_vector: [f32; 3],
  }
  pub struct FacilityAssetTag {
      pub equipment_id: String,
      pub anchor_pos: [f32; 3],
      pub maintenance_interval_days: u32,
  }
  ```
* **Implementation Steps:**
  1. In `Sustainability & Energy`, render a dynamic sun dome (Heliodon) projecting cast shadows across the terrain and building envelope.
  2. Colorize elements by embodied carbon intensity ($\text{kg CO}_2\text{e}$).
  3. In `Facility Management`, place 3D spatial pins on MEP equipment with maintenance history popovers.
* **Definition of Done (DoD):**
  - Sun position matches astronomical solar coordinates for Jakarta at noon on June 21st within $0.5^\circ$.

---

### Phase 46 (`v0.4.6`) — 8D Safety & Crane Sweep, 9D Lean Laydown Canvases
* **Target Module:** `src/views/analyze.rs` / `src/views/lean.rs`
* **Theoretical Foundation:** Construction occupational safety radius sweeps. Just-In-Time (JIT) material buffer staging logistics.
* **Rust Architecture & Structures:**
  ```rust
  pub struct TowerCraneEnvelope {
      pub crane_base_pos: [f32; 3],
      pub jib_length_m: f32,
      pub max_hook_capacity_ton: f32,
  }
  pub struct MaterialLaydownZone {
      pub zone_id: String,
      pub material_type: String, // "Rebar Bundles", "Formwork Panels"
      pub boundary_poly: Vec<[f32; 2]>,
  }
  ```
* **Implementation Steps:**
  1. In `Safety & Risk`, display the rotating coverage circle of the tower crane and highlight open slab edges with glowing red fall-hazard warning tape.
  2. In `Lean Construction`, delineate material laydown pads on the GIS site for rebar and steel bundles.
* **Definition of Done (DoD):**
  - Warning alert triggers if a pick's load exceeds crane capacity at working radius.

---

### Phase 47 (`v0.4.7`) — 10D DfMA Prefab Rigging & Prediction AI Drift Canvases
* **Target Module:** `src/views/industrialized.rs` / `src/views/prediction.rs`
* **Theoretical Foundation:** Rigging mechanics: Center of Gravity equilibrium $\sum \mathbf{M}_{\text{CoG}} = 0$ and sling tension. Machine learning stress concentration and fatigue drift forecasts.
* **Rust Architecture & Structures:**
  ```rust
  pub struct DfmaModularPod {
      pub pod_id: String,
      pub dry_weight_ton: f32,
      pub center_of_gravity: [f32; 3],
      pub road_freight_legal: bool,
  }
  pub struct AiAnomalyForecast {
      pub member_id: String,
      pub predicted_drift_amplification: f32,
      pub fatigue_risk_score: f32,
  }
  ```
* **Implementation Steps:**
  1. In `Industrialized / DfMA`, check whether modular precast pods exceed road transportation limits (12m x 2.5m x 3.2m) or crane lifting capacities.
  2. In `Prediction`, highlight members prone to future fatigue or non-linear drift failure under extreme seismic loads.
* **Definition of Done (DoD):**
  - Pod dimensions exceeding road freight legal boundaries are flagged with amber clearance warnings.

---

## 📑 GROUP 7: The `Report` Publishing Suite (`Docs, Sheets, Slides`)

---

### Phase 48 (`v0.4.8`) — `Report -> Docs`: Living Document & DED Layout Canvas
* **Target Module:** `src/views/results/docs_engine.rs`
* **Theoretical Foundation:** WYSIWYG hybrid word-processor and CAD Paper Space page compositor. Dynamic token evaluation (`{{variable}}` substitution).
* **Rust Architecture & Structures:**
  ```rust
  pub struct ReportDocPage {
      pub page_size: PageStandard, // A4, A3, A1
      pub title_block_data: TitleBlockMetadata,
      pub embedded_viewports: Vec<EmbeddedDedViewport>,
      pub text_blocks: Vec<ReportTextBlock>,
  }
  pub struct TitleBlockMetadata {
      pub project_name: String,
      pub sheet_number: String,
      pub lead_engineer: String,
  }
  ```
* **Implementation Steps:**
  1. Build a living document layout canvas supporting standard engineering sheet sizes (A3 / A1 / A4).
  2. Embed live DED drawings (plans, elevations, sections generated in `Geometry`) onto printable drawing sheets.
  3. Allow engineers to click directly on the title block (*kop gambar*) to edit project name, engineer signature name, and revision number.
  4. Live-bind numerical variables in technical narratives (e.g., `Max DCR = {{dcr.peak}}`).
* **Definition of Done (DoD):**
  - Changing a beam size in the model automatically updates both the drawing view cut and the text in the calculation dossier without manual re-export.
  - Direct PDF export produces crisp vector lines, standard title blocks, and scalable fonts.

---

### Phase 49 (`v0.4.9`) — `Report -> Sheets`: Dynamic Engineering Spreadsheet Engine
* **Target Module:** `src/views/results/sheets_engine.rs`
* **Theoretical Foundation:** Grid cell formula evaluation parser ($=SUM(A1:A10)$, arithmetic $+ - * /$, cell referencing) and table serialization.
* **Rust Architecture & Structures:**
  ```rust
  pub struct SpreadsheetCell {
      pub raw_content: String, // e.g., "=B2*C2" or "45000"
      pub evaluated_value: CellValue,
      pub format_currency: bool,
  }
  pub struct DynamicWorkbook {
      pub sheet_name: String,
      pub cells: std::collections::HashMap<(usize, usize), SpreadsheetCell>,
  }
  ```
* **Implementation Steps:**
  1. Implement an in-memory interactive spreadsheet grid inside `Report -> Sheets`.
  2. Pre-load auto-generated sheets: Dynamic BoQ (steel tonnage, concrete volume, Cut & Fill $m^3$) and Bar Bending Schedule (BBS).
  3. Support full cell editing, column resizing, formula insertion, and currency formatting.
* **Definition of Done (DoD):**
  - Editing unit cost in cell `D5` immediately recalculates total budget in sum cell `E20` via formula `=SUM(E2:E19)`.
  - Export to `.xlsx` / `.csv` functions seamlessly.

---

### Phase 50 (`v0.5.0`) — `Report -> Slides`: Presentation Deck & 3D Interactive Viewports
* **Target Module:** `src/views/results/slides_engine.rs`
* **Theoretical Foundation:** High-impact client deck presentation authoring; interactive 3D WebGL/egui viewport embedding within slide canvas.
* **Rust Architecture & Structures:**
  ```rust
  pub struct PresentationSlide {
      pub slide_index: usize,
      pub title: String,
      pub bullet_points: Vec<String>,
      pub embedded_3d_state: Option<View3dCameraSnapshot>,
  }
  pub struct PresentationDeck {
      pub slides: Vec<PresentationSlide>,
      pub active_slide_index: usize,
      pub is_fullscreen_presentation: bool,
  }
  ```
* **Implementation Steps:**
  1. Build a presentation deck authoring view with slide thumbnails on the left and master slide canvas on the center.
  2. Pre-generate executive engineering slides: *Project Summary, GIS & Environmental Context, Structural System & ARI Redundancy, Cost & Schedule Breakdown*.
  3. Embed live interactive 3D model viewports into the slide: during client meetings or municipal permit hearings, the presenter can orbit, zoom, and demonstrate earthquake/wind simulation directly inside the slide.
  4. Provide a full-screen presentation mode (`F5` toggle).
* **Definition of Done (DoD):**
  - Full-screen presentation mode allows seamless navigation between slides with interactive 3D orbiting running smoothly at 60 FPS.
  - One-click export to multi-page PDF presentation slides.

---

## 🏁 Summary Matrix: Phases 22 – 50 Progression

| Milestone | Phase Range | Version Range | Focus Deliverable |
| :--- | :--- | :--- | :--- |
| **Pillar Shell Baseline** | **Phase 22** | `v0.2.2` | **The Master Shell & 15-Pillar High-Fidelity UI Layout (Apple Minimalist HIG)** |
| **Group 1: GIS Site Inception** | Phases 23 – 26 | `v0.2.3` – `v0.2.6` | Slippy map `walkers`, UTM `proj4rs`, DEM TIN mesh, boreholes, PusGeN spectrum. |
| **Group 2: Parametric Studio** | Phases 27 – 30 | `v0.2.7` – `v0.3.0` | 2D vector drafting, 3D live extrusion, cross-section rules, rebar DED, asset packaging. |
| **Group 3: Clean BIM Geometry** | Phases 31 – 34 | `v0.3.1` – `v0.3.4` | $Z$-level planes, grids $A\text{-}B\text{-}C$, component stamping, MEP sleeves, dynamic DED cuts. |
| **Group 4: Geo-Structural Engine** | Phases 35 – 39 | `v0.3.5` – `v0.3.9` | Boolean Cut & Fill ($m^3$), X-Ray ground slider, bearing capacity, settlement, FEA/SVD/ARI. |
| **Group 5: Multi-Hazard Sandbox** | Phases 40 – 43 | `v0.4.0` – `v0.4.3` | Earthquake shake table, wind streamlines, flood rise, in-place live-tweak loop. |
| **Group 6: Visual 4D–10D Canvases** | Phases 44 – 47 | `v0.4.4` – `v0.4.7` | 4D Erection timeline, 5D cost heatmap, 6D solar/carbon, 7D facility, 8D crane, 10D DfMA. |
| **Group 7: Report Publishing Suite** | Phases 48 – 50 | `v0.4.8` – `v0.5.0` | Living `Docs` (DED + title blocks), dynamic `Sheets` (formulas), interactive 3D `Slides`. |
