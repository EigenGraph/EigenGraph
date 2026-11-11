<div align="center">

<img src="assets/logo.png" width="380" alt="EigenGraph Logo" />

# 🌐 EigenGraph Web Workstation

### High-Performance 10D BIM & Structural Graph Analytics Running Natively in WebAssembly (Rust + egui/eframe)

[![Live Web App](https://img.shields.io/badge/web%20app-live%20on%20GitHub%20Pages-0A84FF?style=for-the-badge&logo=githubpages&logoColor=white)](https://eigengraph.github.io/Website/)
[![Core Engine](https://img.shields.io/badge/engine-Rust%20%2B%20WebAssembly%20(WebGL2)-orange?style=for-the-badge&logo=rust&logoColor=white)](https://github.com/EigenGraph/EigenGraph)
[![License](https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue?style=for-the-badge)](LICENSE)

*Full Rust/egui **EigenGraph** 10D BIM & Structural Mechanics Workstation compiled directly to WebAssembly (`wasm32-unknown-unknown`) via `eframe` and `Trunk`.*

[**Launch EigenGraph Web App ↗**](https://eigengraph.github.io/Website/) • [**Desktop Repository ↗**](https://github.com/EigenGraph/EigenGraph) • [**Download Desktop Releases ↗**](https://github.com/EigenGraph/EigenGraph/releases)

</div>

> [!IMPORTANT]
> **Disclaimer**: *EigenGraph is an experimental computational engineering and BIM platform currently under active development. Features, architecture, and capabilities are subject to change.*

---

## 🏛️ Overview

This repository (`EigenGraph/Website`) hosts the **EigenGraph Web Application**, directly ported from the [`EigenGraph/EigenGraph`](https://github.com/EigenGraph/EigenGraph) Rust codebase (`v0.2.5`, Phase 25) and compiled into a client-side **WebAssembly + WebGL2** application deployed automatically to GitHub Pages.

### 15-Pillar Master Shell Architecture
1. **📁 My Library** — Project records, parametric component catalog, interactive HTML5 3D viewer & Markdown dossier browser downloads.
2. **🤖 AI Agent** — Virtual engineering firm & multi-agent collaboration board.
3. **💡 Research** — Interactive Slippy Map (Web Mercator Quadtree), WGS84/UTM geodetic CRS transformations, and DEM topographic TIN mesh.
4. **📐 Parametric** — 2D vector CAD studio with magnetic snaps, live 3D solid extrusion companion, and PDF/SVG/IFC4 exports.
5. **🏗️ Geometry** — Clean BIM spatial assembly across multi-level Z-planes and structural grids.
6. **⚡ Analysis** — 3D FEM solver, Sherman-Morrison $O(N^2)$ linear ARI, Woodbury rank-2 ($N-2$) disruption screening, SVD kinematic mechanism guard, pushover, nonlinear time-history, seismic isolation, geotech/SSI, blast, and fire engineering.
7. **⏱️ Time & Scheduling** — 4D phased erection timeline, CPM scheduling, and S-Curve tracking.
8. **💰 Cost & Budgeting** — 5D cost heatmap, Quantity Take-Off (QTO), and parametric BoQ.
9. **🌱 Sustainability & Energy** — 6D solar heliodon sun-path analysis and cradle-to-gate embodied carbon LCA.
10. **🏢 Facility Management** — 7D Digital Twin spatial asset tagging and telemetry logs.
11. **🛡️ Safety & Health** — 8D tower crane sweep envelopes and slab-edge hazard guards.
12. **♻️ Lean Construction** — 9D material staging, 1D Cutting Stock Optimizer (CSO), and Bar Bending Schedules (BBS).
13. **🤖 Industrialized & Automation** — 10D DfMA modular prefabrication, OpenBIM IFC 4.3 STEP parser/editor, clash detection, model diff, and IDS validation.
14. **🔮 Prediction** — Structural stress concentration forecasts and aging risk diagnostics.
15. **📑 Report** — Living publishing suite for engineering docs, spreadsheets, and executive decks.

---

## 💻 Local Development

### Run as WebAssembly Web App (Recommended)
```bash
# Install the WebAssembly Rust target and Trunk bundler
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Serve locally with live reload
trunk serve --open
```

### Run as Native Desktop Application
```bash
cargo run --release --bin eigengraph
```

### Run Automated Tests & Checks
```bash
cargo test --lib
cargo check --bin eigengraph --target wasm32-unknown-unknown
```

---

## 🚀 Continuous Deployment (GitHub Pages)

Every push to `main` triggers [`.github/workflows/pages.yml`](.github/workflows/pages.yml):
1. Runs `cargo test --lib` and verifies `cargo check --bin eigengraph --target wasm32-unknown-unknown`.
2. Builds the optimized WebAssembly bundle with `trunk build --release`.
3. Publishes `dist/` directly to GitHub Pages.

---

## 📄 License

Copyright © 2026 Harids Irawan.  
Dual-licensed under the [MIT License](LICENSE) and [Apache License 2.0](LICENSE).
