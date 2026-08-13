# Deadline Calendar and Focus Persistence Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the visually inconsistent native Deadline date-time field with a compact custom calendar/time selector and persist focus/rest adjustments before a focus session starts.

**Architecture:** Extend the existing Deadline domain module with calendar-grid utilities and keep the picker’s public `v-model` contract unchanged, so task creation and later scheduling receive the redesign together. Add an optimistic, debounced preset-save path in `FocusView`; Pinia updates immediately for the home launcher, while the existing Tauri settings command remains the durable storage boundary.

**Tech Stack:** Tauri 2, Vue 3, TypeScript, Pinia, Vitest

## Global Constraints

- Keep the existing four Deadline presets and immutable-Deadline rule.
- Interpret all calendar and clock selections in the user’s local timezone.
- Keep the compact 820×640 application layout usable.
- Focus and rest values remain in the inclusive range 1–180 minutes.
- Each focus stepper arrow continues to change by 5 minutes.
- Leaving the focus page after an adjustment must not lose the new preset.

---

### Task 1: Calendar domain utilities

**Files:**
- Modify: `src/domain/deadline.ts`
- Modify: `src/domain/deadline.test.ts`

**Interfaces:**
- Produces: `CalendarDay`, `calendarMonthDays()`, `parseLocalDateTimeValue()`, and `combineLocalDeadline()`.

- [ ] **Step 1: Add failing boundary tests**

Test a six-week Monday-first calendar grid, adjacent-month days, leap-year February, local value parsing, and date/time recombination.

- [ ] **Step 2: Run the focused test**

Run: `npm run test:run -- src/domain/deadline.test.ts`

Expected: FAIL because the calendar helpers are not exported.

- [ ] **Step 3: Implement local calendar helpers**

Use local `Date` constructors and numeric components; do not derive local dates by slicing UTC ISO strings.

- [ ] **Step 4: Run the focused test**

Run: `npm run test:run -- src/domain/deadline.test.ts`

Expected: PASS.

### Task 2: Custom Deadline date and time control

**Files:**
- Modify: `src/components/tasks/DeadlinePicker.vue`
- Modify: `DESIGN.md`

**Interfaces:**
- Consumes: calendar helpers from `src/domain/deadline.ts`.
- Preserves: `modelValue: string` and `update:modelValue`.

- [ ] **Step 1: Replace the native date-time field**

Render a friendly date button, an inline keyboard-accessible calendar popover, and styled hour/minute selectors. Preserve shortcut selection, invalid-state copy, Escape closing, and outside-click closing.

- [ ] **Step 2: Update design guidance**

Document the custom Monday-first calendar and separate clock segments as the standard Deadline direct-entry pattern.

### Task 3: Persist focus presets on adjustment

**Files:**
- Modify: `src/views/FocusView.vue`
- Modify: `src/stores/app.ts`

**Interfaces:**
- Consumes: existing `clampMinutes()` and `store.updateSettings()`.
- Produces: immediate Pinia preset updates plus a debounced durable save that flushes before route unmount and before starting focus.

- [ ] **Step 1: Make settings updates optimistic**

Assign the validated next settings to Pinia before awaiting the Tauri/localStorage backend, while restoring the previous value if persistence fails.

- [ ] **Step 2: Watch focus and rest controls**

Clamp new values, update the store immediately, debounce durable writes, and flush the latest values on unmount.

- [ ] **Step 3: Reuse the flush before starting**

Ensure a focus session always uses the same saved values currently shown in the steppers and on the home launcher.

### Task 4: Verification

**Files:**
- Test: `src/domain/deadline.test.ts`
- Test: existing Vitest suite

**Interfaces:**
- Verifies: calendar math, type safety, production build, focus persistence across routes, and both Deadline entry points.

- [ ] **Step 1: Run automated checks**

Run: `npm run test:run`

Expected: all tests pass.

Run: `npm run build`

Expected: TypeScript and Vite production build pass.

- [ ] **Step 2: Verify the development preview**

At 820×640, select a custom date and time in both Deadline entry points. Change focus/rest values, return home, confirm the launcher displays them, start focus, and confirm the new session uses them.

