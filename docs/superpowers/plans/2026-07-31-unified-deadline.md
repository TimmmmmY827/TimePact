# Unified Deadline Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace separate Deadline/countdown scheduling choices with one Deadline flow that supports direct date-time entry and four local-time presets.

**Architecture:** Keep the persisted countdown timer shape readable so existing user data remains valid, but stop creating new countdown tasks. Centralize all preset calculations and local `datetime-local` formatting in a domain helper, then reuse one accessible `DeadlinePicker` in both task creation and later scheduling.

**Tech Stack:** Tauri 2, Vue 3, TypeScript, Pinia, Vitest, Rust

## Global Constraints

- New task scheduling exposes only `未安排` and `Deadline`.
- Deadline remains optional at creation and immutable after it is saved.
- Presets are `今天下班` (18:00 local), `明天下班` (18:00 local), `本周结束` (Sunday 23:59 local), and `本月结束` (last calendar day 23:59 local).
- A preset that is no longer in the future is disabled.
- Existing persisted countdown tasks remain loadable and operational for backward compatibility.

---

### Task 1: Deadline preset domain

**Files:**
- Create: `src/domain/deadline.ts`
- Create: `src/domain/deadline.test.ts`

**Interfaces:**
- Produces: `DeadlinePreset`, `DEADLINE_PRESETS`, `resolveDeadlinePreset()`, `toLocalDateTimeValue()`, and `isFutureDeadline()`.

- [ ] **Step 1: Write failing tests**

Cover the local values for the four presets, Sunday and month boundaries, and the expired `今天下班` state using a fixed local `Date`.

- [ ] **Step 2: Run the focused test**

Run: `npm run test:run -- src/domain/deadline.test.ts`

Expected: FAIL because `src/domain/deadline.ts` does not exist.

- [ ] **Step 3: Implement preset calculations**

Use local calendar constructors instead of UTC string slicing so `datetime-local` values remain correct in every system timezone.

- [ ] **Step 4: Run the focused test**

Run: `npm run test:run -- src/domain/deadline.test.ts`

Expected: PASS.

### Task 2: Reusable Deadline picker

**Files:**
- Create: `src/components/tasks/DeadlinePicker.vue`
- Modify: `DESIGN.md`

**Interfaces:**
- Consumes: `DEADLINE_PRESETS`, `resolveDeadlinePreset()`, and `toLocalDateTimeValue()`.
- Produces: Vue `v-model` contract through `modelValue: string` and `update:modelValue`.

- [ ] **Step 1: Implement the picker**

Render one labelled `datetime-local` input and a compact four-button shortcut group. Mark the selected preset, disable expired presets, and preserve keyboard focus visibility.

- [ ] **Step 2: Document the control**

Add the unified Deadline picker and its shortcut-button behavior to the existing control guidance in `DESIGN.md`.

### Task 3: Use one Deadline flow everywhere

**Files:**
- Modify: `src/components/tasks/TaskComposer.vue`
- Modify: `src/components/tasks/TaskRow.vue`
- Modify: `src/views/HomeView.vue`
- Modify: `src/views/ReminderView.vue`

**Interfaces:**
- Consumes: `DeadlinePicker`.
- Preserves: Legacy `TaskTimer.kind === "countdown"` rendering and pause/resume behavior only for existing saved tasks.

- [ ] **Step 1: Simplify task creation**

Remove the countdown radio and duration input, submit only `timerKind: "deadline"` or `null`, and reset only the unified Deadline value.

- [ ] **Step 2: Simplify later scheduling**

Replace the timer-kind select and conditional inputs with the same `DeadlinePicker`; save a locked deadline timer.

- [ ] **Step 3: Unify user-facing copy**

Replace Deadline/countdown wording in the home hint, metadata, reminder, and legacy action labels with Deadline wording.

### Task 4: Verification

**Files:**
- Test: `src/domain/deadline.test.ts`
- Test: existing Vitest suite

**Interfaces:**
- Verifies: domain boundaries, TypeScript contracts, production frontend build, and live preview interactions.

- [ ] **Step 1: Run automated checks**

Run: `npm run test:run`

Expected: all tests pass.

Run: `npm run typecheck`

Expected: no TypeScript errors.

Run: `npm run build`

Expected: Vite production build completes.

- [ ] **Step 2: Inspect the live development preview**

Verify both creation and later scheduling show one Deadline control; exercise all four presets and direct date-time entry at the compact application viewport.

