# Website Agent Guidelines & Architectural Rules

## 🌐 MANDATORY ENGLISH CONSISTENCY FOR ALL GITHUB INPUTS & CONTRIBUTIONS

- **Strict English-Only Policy**:
  - All content committed or pushed to GitHub for `Website` **MUST BE EXCLUSIVELY WRITTEN IN PROFESSIONAL CIVIL & STRUCTURAL ENGINEERING ENGLISH**.
- **Automatic Language Conversion**:
  - If a user prompt, issue, task description, or feedback is submitted in Indonesian (or any language other than English), agents **must automatically translate and adapt** all resulting code, comments, documentation, UI labels, and commit messages into standard English before staging, committing, or pushing to GitHub.
- **Enforced Scope**:
  1. *Git Metadata*: Commit messages, branch names, pull request titles/descriptions, and release notes.
  2. *User Interfaces & Web*: All buttons, ribbons, HUD inspectors, dialog modals, tooltips, cards, and web pages (`index.html`, `app.js`, `styles.css`).
  3. *Source Code & Frontend Scripts*: All code comments, docstrings, variable names, error messages, and console logs.
  4. *Technical Artifacts*: All 3D WebGL visualizations, calculation summaries, and exported structural documentation.

---

## ⚡ Local-First Execution Protocol
- All UI modifications and local previews run locally within the workspace.
- Operations pushing to external repositories or GitHub Pages execute only when the user explicitly gives instructions.

---

## 🔢 STRICT SEMANTIC VERSIONING PROGRESSION RULE

To maintain predictable engineering cadence and transparent release lineage, all repositories (`EigenGraph`, `Website`, `EigenGate`, `EigenFlow`) follow a **mandatory three-tier numerical progression rule**:

1. **Patch Level Progression (Initial Releases):**
   - Versioning begins at initial baseline `v0.0.1`.
   - Subsequent incremental releases advance strictly one patch digit at a time:
     `v0.0.1` &rarr; `v0.0.2` &rarr; `v0.0.3` &rarr; `v0.0.4` &rarr; `v0.0.5` &rarr; `v0.0.6` &rarr; `v0.0.7` &rarr; `v0.0.8` &rarr; `v0.0.9`.
   - **Prohibited:** Skipping directly from `v0.0.1` to `v0.1.0` or skipping any intermediate patch version is strictly forbidden.

2. **Minor Level Progression (After v0.0.9):**
   - Only after `v0.0.9` has been reached does the version advance to `v0.1.0`.
   - Minor releases then proceed incrementally:
     `v0.1.0` &rarr; `v0.1.1` &rarr; `v0.1.2` &rarr; ... &rarr; `v0.1.9` &rarr; `v0.2.0` &rarr; ... &rarr; `v0.9.9`.

3. **Major Level Progression (Production Milestone v1.0.0):**
   - Only after `v0.9.9` has completed does the version advance to major production milestone `v1.0.0`.
   - Major releases follow the identical incremental rule:
     `v1.0.0` &rarr; `v1.0.1` &rarr; `v1.0.2` &rarr; ...

All agents and developers MUST record and uphold this rule across all `Cargo.toml`, release tags, download links, changelogs, and documentation.
