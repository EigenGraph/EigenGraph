# EigenGraph Agent Guidelines & Architectural Rules

## 🔒 CORE RULE: LOCKED SIDEBAR & SLIDE-MODEL WORKSPACE FREEDOM

### 1. 🛡️ Absolute Locked Sidebar Layout (Until Explicit Instructions):
The Left Sidebar Layout (`src/app.rs`) and macOS Window Chrome are **STRICTLY LOCKED AND MUST NOT BE MODIFIED** unless the user explicitly provides written instructions (e.g., `CHANGE_SIDEBAR:` or `CHANGE_UI:`).

**Mandatory Sidebar Rules (macOS App Store Style):**
- **Full-Height Geometry**: Sidebar extends continuously from top (`y = 0`) to bottom of the screen.
- **Window Controls Inside Sidebar**: macOS traffic lights (`🔴  🟡  🟢`) sit in the top corner of the sidebar.
- **Pill Search Bar (No Logo)**: Search pill (`🔍 Search`) is located directly below the traffic lights without branding/logo header.
- **No Divider Headers / Badges**: Do not add section headers such as `LIBRARY`, `1D..10D`, or pill badges.
- **Fixed List of 13 Navigation Pillars**:
  1. `📁   My Library`
  2. `💡   Research & Concept`
  3. `📐   Drafting & Drawings`
  4. `🏗️   Modeling & Geometry`
  5. `⏱️   Time & Scheduling`
  6. `💰   Cost & Budgeting`
  7. `🌱   Sustainability & Energy`
  8. `🏢   Facility Management`
  9. `🛡️   Safety/Health & Safety`
  10. `♻️   Lean Construction`
  11. `🤖   Industrialized & Automation`
  12. `🔮   Prediction`
  13. `📊   Results`
- **Pinned Bottom Section**: User Profile (`👤 Harids Irawan`) with gear `⚙️` (System Preferences ⌘,). The Update button is located inside the Settings Preferences menu, and the AI Copilot button in the sidebar is removed for a clean minimalist UI.

---

### 2. 🚀 Workspace Canvas Modification Freedom:
For the **Workspace (Main Canvas Area)** across all navigation pillars:
- **Free to Modify & Enhance**: Agents have full authorization to modify, enrich graphic components, add algorithms, control panels, and new analytic features according to research and development needs.
- **Mandatory Vertical Slide Model (My Library Style)**:
  - Workspace displays **must use smooth vertical scrolling** (`egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| { ... })`).
  - Content structure is organized in modern macOS stacked cards:
    1. *Minimalist Pillar Header*: Clean title with subtle line divider (no double strip-bars / breadcrumbs).
    2. *Hero/Featured Master Card*: Main project overview, ARI status, and quick actions.
    3. *Data & Parameter Control Cards*: Numerical parameter configuration, simulation, and stiffness matrix controls.
    4. *Visualizer / Blueprint / Interactive Canvas*: 2D/3D technical drawing or convergence plot areas.
    5. *Detailed Audit Tables & Data*: BoQ recaps, carbon emissions, or response spectra.
- **Minimalist & Clean**: Avoid horizontal strip toolbars that clutter the visual canvas area.

---

### 3. Window & Asset Icon:
- Window icon is loaded from `assets/icon.png` via `eframe::icon_data::from_png_bytes`.
- macOS application bundle icon is `assets/AppIcon.icns`.

---

### 4. ⚡ Token Efficiency & Local Execution Protocol (Local-First Development):
- **Local-First by Default**:
  - All new feature work, interface modifications, bug fixes, and unit tests run **100% locally within the user workspace**.
  - Agents **MUST NOT** proactively scan remote GitHub repos, query APIs (`curl api.github.com`), check GitHub Actions workflows, or read git remotes when handling coding or feature requests.
- **Exceptions (Explicit Commands Only)**:
  - Operations involving external network / remote GitHub (such as `git push`, monitoring CI/CD builds, or publishing releases) **ONLY** execute when the user explicitly gives written instructions (e.g., *"push to github"*, *"create a release"*, *"check build status"*).
- **Token Conservation**:
  - Agents must focus directly on target files without unnecessary directory scanning or consuming context tokens outside the scope of the task.

---

### 5. 🌐 Mandatory English Consistency for All GitHub Inputs & Contributions:
- **Strict English-Only Policy**:
  - All content committed or pushed to GitHub across all repositories (`EigenGraph`, `Website`, `EigenFlow`, `EigenGate`) **MUST BE EXCLUSIVELY WRITTEN IN PROFESSIONAL CIVIL & STRUCTURAL ENGINEERING ENGLISH**.
- **Automatic Language Conversion**:
  - If a user prompt, issue, task description, or feedback is submitted in Indonesian (or any language other than English), agents **must automatically translate and adapt** all resulting code, comments, documentation, UI labels, and commit messages into standard English before staging, committing, or pushing to GitHub.
- **Enforced Scope**:
  1. *Git Metadata*: Commit messages, branch names, pull request titles/descriptions, and release notes.
  2. *User Interfaces & Web*: All buttons, ribbons, HUD inspectors, dialog modals, tooltips, cards, and web pages.
  3. *Source Code & Architecture*: All code comments, docstrings (`///` or `"""`), runtime error messages, log statements, and test assertions.
  4. *Technical Artifacts*: All calculation dossiers, structural redundancy reports, BoQ divisions, BBS schedules, and exported BIM/IFC models.

