# Website Agent Guidelines & Architectural Rules

## 🌐 MANDATORY ENGLISH CONSISTENCY FOR ALL GITHUB INPUTS & CONTRIBUTIONS

- **Strict English-Only Policy**:
  - All content committed or pushed to GitHub for `Website` **MUST BE EXCLUSIVELY WRITTEN IN PROFESSIONAL CIVIL & STRUCTURAL ENGINEERING ENGLISH**.
- **Automatic Language Conversion**:
  - If a user prompt, issue, task description, or feedback is submitted in Indonesian (or any language other than English), agents **must automatically translate and adapt** all resulting code, comments, documentation, UI labels, and commit messages into standard English before staging, committing, or pushing to GitHub.
- **Enforced Scope**:
  1. *Git Metadata*: Commit messages, branch names, pull request titles/descriptions, and release notes.
  2. *User Interfaces & Web*: All buttons, ribbons, HUD inspectors, dialog modals, tooltips, cards, and web pages (`index.html`, Rust `egui` views).
  3. *Source Code & Frontend Scripts*: All code comments, docstrings (`///`), variable names, error messages, and console logs.
  4. *Technical Artifacts*: All 3D visualizations, calculation summaries, and exported structural documentation.

---

## 🔒 CORE RULE: LOCKED SIDEBAR & SLIDE-MODEL WORKSPACE FREEDOM

### 1. 🛡️ Absolute Locked Sidebar Layout (Until Explicit Instructions):
The Left Sidebar Layout (`src/app.rs`) and macOS-inspired Window Chrome are **STRICTLY LOCKED AND MUST NOT BE MODIFIED** unless the user explicitly provides written instructions (e.g., `CHANGE_SIDEBAR:` or `CHANGE_UI:`).

**Fixed List of 15 Navigation Pillars (Phase 22+ Master Shell):**
1. `📁   My Library`
2. `🤖   AI Agent`
3. `💡   Research`
4. `📐   Parametric`
5. `🏗️   Geometry`
6. `⚡   Analysis`
7. `⏱️   Time & Scheduling`
8. `💰   Cost & Budgeting`
9. `🌱   Sustainability & Energy`
10. `🏢   Facility Management`
11. `🛡️   Safety & Health`
12. `♻️   Lean Construction`
13. `🤖   Industrialized & Automation`
14. `🔮   Prediction`
15. `📑   Report`

### 2. 🌐 WebAssembly & Cross-Platform Compatibility:
- All core mechanics and UI code must compile cleanly on both native targets and `wasm32-unknown-unknown`.
- Never invoke blocking native OS APIs (`std::process::Command`, `std::time::SystemTime::now()`, `std::thread::spawn`, or unconditional filesystem writes) without `#[cfg(not(target_arch = "wasm32"))]` guards or browser-safe alternatives in `src/core/web_bridge.rs`.

---

## ⚡ Continuous Deployment & Automatic GitHub Push Protocol
- All UI modifications, `cargo test`, and `trunk build` checks are validated locally within the workspace before publishing.
- **Automatic Push Enabled**: Every completed feature development, enhancement, and architectural update must be automatically committed and pushed directly to GitHub `main` without waiting for separate manual push instructions.

---

## 🔢 STRICT SEMANTIC VERSIONING PROGRESSION RULE

To maintain predictable engineering cadence and transparent release lineage, all repositories (`EigenGraph`, `Website`, `EigenGate`, `EigenFlow`) follow a **mandatory three-tier numerical progression and 1-to-1 Phase mapping rule**:

1. **Patch Level Progression (Initial Releases):**
   - Versioning begins at initial baseline `v0.0.1` and advances one patch digit at a time (`v0.0.1` → `v0.0.2` → ... → `v0.0.9`).
   - Skipping intermediate patch versions is prohibited.

2. **Minor Level Progression (After v0.0.9):**
   - Advances to `v0.1.0` → `v0.1.1` → ... → `v0.2.5` (Phase 25) → ... → `v0.9.9`.

3. **Major Level Progression (Production Milestone v1.0.0):**
   - Only after `v0.9.9` completes does the version advance to `v1.0.0`.
