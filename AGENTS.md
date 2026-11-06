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
