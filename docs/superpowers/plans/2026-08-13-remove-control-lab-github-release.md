# Remove Control Lab and GitHub Release Implementation Plan

> **For agentic workers:** Execute the checked steps in order and verify each deliverable before continuing.

**Goal:** Remove the temporary time-control comparison page and prepare TimePact for reproducible Windows releases through GitHub.

**Architecture:** Keep only shipped application routes and components. Add a Windows GitHub Actions release pipeline that validates both frontend and Rust code, then lets the official Tauri action build and attach the NSIS installer to a GitHub Release.

**Tech Stack:** Tauri 2, Vue 3, TypeScript, Rust, GitHub Actions, tauri-apps/tauri-action.

## Global Constraints

- Keep all production task, deadline, focus, reminder, settings, statistics, and overlay behavior unchanged.
- Release Windows x64 NSIS installers under the English product name TimePact.
- Do not create or publish a remote GitHub repository without an authenticated GitHub account.

---

### Task 1: Remove the temporary control laboratory

**Files:**
- Modify: `src/App.vue`
- Modify: `src/router/index.ts`
- Delete: `src/views/ControlLabView.vue`
- Delete: `src/components/lab/FocusDurationVariants.vue`
- Delete: `src/components/lab/ScheduleControlVariants.vue`
- Delete: `docs/superpowers/plans/2026-07-31-compact-shell-time-control-variants.md`

- [x] Remove the navigation destination and route import.
- [x] Delete the orphaned experimental components and historical temporary-page plan.
- [x] Run the frontend tests, type check, production build, and dead-reference search.

### Task 2: Prepare reproducible GitHub releases

**Files:**
- Create: `.github/workflows/release.yml`
- Modify: `.gitignore`
- Modify: `README.md`

- [x] Add a Windows release workflow using Node 22, Rust stable, `npm ci`, frontend tests, Rust tests, and `tauri-apps/tauri-action@v1`.
- [x] Ignore generated Rust build products and local environment files.
- [x] Document the initial repository upload, tag-based release, download, and unsigned-installer warning.
- [x] Initialize the local Git repository and verify the complete publishable file set contains no obvious secrets.
- [x] Run the final frontend and Rust validation suite.
