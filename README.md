<div align="center">

# 🌐 EigenGraph Official Website

### High-Performance Computational Framework for Adjusted Redundancy Index (ARI), SVD Kinematic Mechanism Guards, and 10D BIM Construction Lifecycle Analytics

[![Live Website](https://img.shields.io/badge/website-live%20at%20eigengraph.github.io-0A84FF?style=for-the-badge&logo=githubpages&logoColor=white)](https://eigengraph.github.io/)
[![Core Engine](https://img.shields.io/badge/engine-Rust%20Analytical%20Framework-orange?style=for-the-badge&logo=rust&logoColor=white)](https://github.com/EigenGraph/EigenGraph)
[![License](https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue?style=for-the-badge)](LICENSE)

*Interactive web showcase, numerical laboratory, and official documentation portal for the **EigenGraph** computational engineering platform.*

[**Explore Live Website ↗**](https://eigengraph.github.io/) • [**Core Repository ↗**](https://github.com/EigenGraph/EigenGraph) • [**API Documentation ↗**](https://eigengraph.github.io/#features)

</div>

> [!IMPORTANT]
> **Disclaimer**: *EigenGraph is an experimental computational engineering and BIM platform currently under active development. Features, architecture, and capabilities are subject to change.*

---

## 🏛️ Overview

**EigenGraph** is a specialized next-generation computational engineering and 10D BIM platform designed to safeguard civil structures against disproportionate collapse and progressive failure.

This repository (`eigengraph.github.io`) hosts the official client-side interactive web portal, featuring real-time numerical simulations executed directly in the browser via canvas rendering and lightweight analytical solvers.

---

## ⚡ Interactive Computational Sandboxes

### 1. 🔬 Sherman-Morrison Linear ARI & SVD Gate 1 Guard
- **Real-Time Perturbation**: Demonstrates $O(N^2)$ direct inverse updates via the Sherman-Morrison matrix identity:
  $$(\mathbf{K}_{ff}^{(-e)})^{-1} = \mathbf{K}_{ff}^{-1} + \frac{k_e \, (\mathbf{K}_{ff}^{-1} \mathbf{a}_e)(\mathbf{a}_e^T \mathbf{K}_{ff}^{-1})}{1 - h_e}$$
- **Instant Kinematic Detection**: Evaluates Gate 1 spectral determinacy $m_e = 1 - h_e$. When $m_e \le \varepsilon_m$, an instantaneous collapse mechanism is certified ($ARI_e = 0.0$, $CI_e = 1.0$) with zero false-positives.
- **Interactive APM**: Click **"Simulate Member Failure"** or hover over structural truss members to observe instantaneous load redistribution.

### 2. 📐 SNI 2847 / ACI 318 Concrete Column P-M Interaction
- **Whitney Stress Block Generation**: Dynamic axial-moment interaction envelope ($P_n$ vs. $M_n$) calculated with equivalent rectangular stress distribution ($\beta_1$, $0.85 f'_c$).
- **Cardinal Points**: Evaluates Pure Compression ($\phi P_{n,\max}$), Balanced Failure ($\phi P_b, \phi M_b$), and Pure Bending ($\phi M_0$).
- **Live LRFD Audit**: Real-time slider controls for column width ($b$), depth ($h$), concrete strength ($f'_c$), steel yield ($f_y$), and factored load demands ($P_u$, $M_u$) with instantaneous Demand/Capacity Ratio (DCR) safety classification.

### 3. ✂️ 1D Cutting Stock Optimizer (CSO) & Bar Bending Schedule (BBS)
- **Best-Fit Decreasing (BFD)**: Heuristic packing algorithm minimizing rebar waste and offcuts across standard 12 m stock lengths.
- **Kerf & Reusability**: Real-time kerf blade loss compensation ($3\text{ mm}$) and reusable offcut threshold classification ($L \ge 1.0\text{ m}$).
- **BBS Generation**: Automatic rebar cutting schedule generation and waste percentage audit.

### 4. 🌋 SNI 1726:2019 Seismic Design Response Spectrum Generator
- **Indonesian Earthquake Code**: Computes site coefficients ($F_a, F_v$), spectral accelerations ($S_{MS}, S_{M1}, S_{DS}, S_{D1}$), and corner transition periods ($T_0, T_s, T_L$).
- **Live Regional Presets**: Instant parameter selection for DKI Jakarta, Padang, Bandung, Surabaya, Yogyakarta, and IKN Nusantara across Site Classes SA through SE.
- **Seismic Design Category (KDS)**: Real-time KDS classification (KDS A through E) with fundamental building period demand probe ($S_a(T_n)$).

### 5. 💥 Transient Dynamic Amplification Factor (DAF Newmark-β)
- **Sudden Member Removal**: Evaluates progressive collapse dynamics according to DoD UFC 4-023-03.
- **Time History Simulation**: Step-by-step SDOF oscillator response showing peak surge overshoot ($u_{\text{peak}} = u_{\text{static}} \times \text{DAF}$), structural damping attenuation ($\zeta$), and finite removal duration ($t_r$).

### 6. 🌐 Real-Time 3D Diagrid Lattice Viewport
- **Spatial Viewport**: Canvas-based 3D rotating diagrid structural geometry with real-time perspective projection, depth-sorted alpha shading, and interactive orbital controls.

---

## 🗺️ The 10D BIM Platform Ecosystem

| Pillar | Focus | Capabilities |
| :--- | :--- | :--- |
| **1D / 2D** | Research & Drawings | Canonical benchmark verification, LaTeX tables, DXF vector drafting. |
| **3D** | Modeling & Geometry | OpenBIM IFC 4.3 solid extrusion, CSG boolean voids, 3D BVH ray-picking. |
| **4D** | Time & Scheduling | Critical Path Method (CPM), forward/backward pass, and S-Curve tracking. |
| **5D** | Cost & Budgeting | Automatic Bill of Quantities (BoQ) with localized unit rate database. |
| **6D** | Sustainability & Carbon | Cradle-to-gate embodied carbon LCA ($\text{kg CO}_2\text{e}$) across steel and concrete. |
| **7D** | Facility Management | COBie spatial asset tagging and predictive maintenance scheduling. |
| **8D** | Health & Safety | AI clash detection, structural stability alarms, and OSHA spatial envelopes. |
| **9D** | Lean Construction | 1D Cutting Stock Optimizer, Bar Bending Schedules, and Last Planner workflows. |
| **10D** | Industrialized & DfMA | DSTV NC1 steel fabrication export and modular precast standardization. |

---

## 💻 Local Development

Run the website locally without any build tools or dependencies:

```bash
# Clone the repository
git clone https://github.com/EigenGraph/eigengraph.github.io.git
cd eigengraph.github.io

# Launch via Python 3 built-in HTTP server
python3 -m http.server 8080

# Or via Node.js npx
npx serve .
```

Open `http://localhost:8080` in your web browser.

---

## 📖 Citation

If you use EigenGraph, the Adjusted Redundancy Index (ARI) formulations, or the computational tools in your academic research, please cite:

```bibtex
@phdthesis{irawan2026eigengraph,
  author    = {Harids Irawan},
  title     = {Computational Framework for Adjusted Redundancy Index (ARI) and Progressive Collapse Resistance in Steel and Concrete Structures},
  school    = {Department of Civil and Environmental Engineering},
  year      = {2026},
  note      = {EigenGraph: Open-Source 10D BIM and Computational Structural Resilience Platform. Available at: https://eigengraph.github.io/}
}
```

---

## 📄 License

Copyright © 2026 Harids Irawan.  
Licensed under the [MIT License](LICENSE) and [Apache License 2.0](LICENSE).
