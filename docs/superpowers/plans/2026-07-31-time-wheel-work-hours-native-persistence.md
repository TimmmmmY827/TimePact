# Time Wheel, Weekly Work Hours, and Native Persistence Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ship a screenshot-aligned wheel-adjustable Deadline time control, configurable per-weekday work-end times, and verified native persistence for focus presets.

**Architecture:** Add a reusable `TimeWheelPicker` that exposes numeric hour/minute values and owns arrow, direct-entry, keyboard, and mouse-wheel behavior. Extend `AppSettings` in TypeScript and Rust with a seven-entry `workdayEndTimes` array using backward-compatible defaults, then resolve “today/tomorrow after work” from those settings. Remove the focus page’s dependency on a newly hot-added Pinia action and enable Pinia HMR so the development preview updates safely.

**Tech Stack:** Tauri 2, Vue 3, TypeScript, Pinia, Rust, SQLite, Vitest

## Global Constraints

- Deadline hours wrap through 00–23 and minutes through 00–59.
- Arrow buttons and mouse-wheel movement change one unit per step.
- Hovering either numeric segment and scrolling must adjust that segment without scrolling the page.
- Each weekday from Monday through Sunday has an independently configurable `HH:mm` work-end time.
- Existing settings without `workdayEndTimes` default every day to `18:00`.
- “今天下班” and “明天下班” use the configured time for the corresponding local weekday.
- Focus/rest settings must be verifiably written to the native preview SQLite `app_state.settings` record.

---

### Task 1: Time and weekly-work-hours domain

**Files:**
- Create: `src/domain/work-hours.ts`
- Create: `src/domain/work-hours.test.ts`
- Modify: `src/domain/deadline.ts`
- Modify: `src/domain/deadline.test.ts`

**Interfaces:**
- Produces: `DEFAULT_WORKDAY_END_TIMES`, `normalizeWorkdayEndTimes()`, `workdayEndForDate()`, `stepClockPart()`, and work-hours-aware `resolveDeadlinePreset()`.

- [ ] **Step 1: Write failing tests**

Cover seven-day defaults, malformed saved arrays, weekday lookup, hour/minute wrapping, and today/tomorrow presets with different configured work-end times.

- [ ] **Step 2: Run focused tests**

Run: `npm run test:run -- src/domain/work-hours.test.ts src/domain/deadline.test.ts`

Expected: FAIL because the new utilities and preset parameter do not exist.

- [ ] **Step 3: Implement domain behavior**

Validate each time with `HH:mm`, clone defaults rather than sharing mutable arrays, and resolve weekday indices using local `Date.getDay()`.

- [ ] **Step 4: Rerun focused tests**

Expected: all focused tests pass.

### Task 2: Backward-compatible settings persistence

**Files:**
- Modify: `src/domain/types.ts`
- Modify: `src/services/backend.ts`
- Modify: `src-tauri/src/models.rs`
- Modify: `src-tauri/src/commands.rs`
- Modify: `src/stores/app.ts`

**Interfaces:**
- Adds: `AppSettings.workdayEndTimes: string[]`.
- Preserves: deserialization of settings written before this field existed.

- [ ] **Step 1: Extend defaults in both runtimes**

Add seven `18:00` values to frontend fallback settings and Rust defaults; mark the Rust field with a serde default function.

- [ ] **Step 2: Normalize loaded settings**

Merge missing or malformed frontend values with the canonical seven-day default before exposing settings to views.

- [ ] **Step 3: Validate native updates**

Reject arrays that are not exactly seven valid `HH:mm` strings in `update_settings`.

### Task 3: Screenshot-aligned time wheel

**Files:**
- Create: `src/components/time/TimeWheelPicker.vue`
- Modify: `src/components/tasks/DeadlinePicker.vue`
- Modify: `DESIGN.md`

**Interfaces:**
- Consumes: `hour`, `minute`, and `stepClockPart()`.
- Emits: `update:hour` and `update:minute`.

- [ ] **Step 1: Build the time wheel**

Use large tabular numeric inputs between full-width up/down buttons. Support direct numeric entry, ArrowUp/ArrowDown, and `wheel.prevent` on each hovered number.

- [ ] **Step 2: Replace Deadline selects**

Keep the custom calendar and four presets, but replace native hour/minute dropdowns with the time wheel.

### Task 4: Weekly work-end settings

**Files:**
- Modify: `src/views/SettingsView.vue`
- Modify: `src/components/tasks/DeadlinePicker.vue`
- Modify: `src/components/tasks/TaskComposer.vue`
- Modify: `src/components/tasks/TaskRow.vue`

**Interfaces:**
- Passes: `workdayEndTimes` from Pinia settings into every Deadline picker.

- [ ] **Step 1: Add the seven-day settings group**

Render Monday–Sunday rows with compact native `time` fields, save each edit through `store.updateSettings()`, and show the common default in helper text.

- [ ] **Step 2: Apply configured preset times**

Generate today/tomorrow shortcut values and labels from the matching weekday configuration.

### Task 5: Native focus persistence and HMR

**Files:**
- Modify: `src/views/FocusView.vue`
- Modify: `src/stores/app.ts`

**Interfaces:**
- Preserves: debounced saves, unmount flush, and start-time flush.

- [ ] **Step 1: Remove the hot-added action dependency**

Stage focus/rest values directly on the existing reactive settings object before calling the existing `updateSettings()` action.

- [ ] **Step 2: Enable Pinia HMR**

Register `acceptHMRUpdate(useAppStore, import.meta.hot)` so future development-preview action changes replace the live store definition.

- [ ] **Step 3: Verify SQLite persistence**

Change the native preview preset, wait for the debounce, and query `app_state` read-only to confirm the JSON contains the new focus and rest values.

### Task 6: Verification

**Files:**
- Test: all Vitest suites
- Verify: native Tauri preview and SQLite

- [ ] **Step 1: Run automated checks**

Run: `npm run test:run`

Run: `npm run build`

Run: `cargo test --manifest-path src-tauri/Cargo.toml`

Expected: all checks pass.

- [ ] **Step 2: Verify at 820×640**

Exercise arrow, keyboard, and mouse-wheel changes on the new time control; update two weekday work-end values; verify matching today/tomorrow preset values; and confirm focus persistence after route changes and native reload.

