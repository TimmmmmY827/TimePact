---
name: TimePact
description: A compact, local-first desktop workbench for time-bound tasks and focus.
colors:
  canvas: "#f4f6f5"
  surface: "#ffffff"
  surface-subtle: "#eef2f0"
  surface-hover: "#e7eeeb"
  ink: "#17211f"
  ink-secondary: "#5e6b68"
  ink-tertiary: "#65736f"
  border: "#dce3e0"
  border-strong: "#c9d3cf"
  action: "#168a7a"
  action-hover: "#117466"
  action-soft: "#dcefeb"
  danger: "#d45545"
  danger-soft: "#fce9e6"
  paused: "#7d9488"
  unscheduled: "#c8d0cd"
  focus-ring: "#0d7569"
  on-action: "#ffffff"
  canvas-dark: "#111715"
  surface-dark: "#18201e"
  surface-subtle-dark: "#202a27"
  surface-hover-dark: "#26332f"
  ink-dark: "#edf3f1"
  ink-secondary-dark: "#aab8b4"
  ink-tertiary-dark: "#7f908b"
  border-dark: "#2c3935"
  border-strong-dark: "#3b4b46"
  action-dark: "#39ad9b"
  action-hover-dark: "#56c0af"
  action-soft-dark: "#163a34"
  danger-dark: "#ef7567"
  danger-soft-dark: "#432421"
  paused-dark: "#8da69a"
  unscheduled-dark: "#52605c"
  focus-ring-dark: "#68cabb"
typography:
  display:
    fontFamily: '"Segoe UI Variable Text", "Segoe UI", "Microsoft YaHei UI", system-ui, sans-serif'
    fontSize: "clamp(3.3rem, 9vw, 5rem)"
    fontWeight: 620
    lineHeight: 1
    letterSpacing: "-0.04em"
  inputDisplay:
    fontFamily: '"Segoe UI Variable Text", "Segoe UI", "Microsoft YaHei UI", system-ui, sans-serif'
    fontSize: "2.8rem"
    fontWeight: 620
    lineHeight: 1
    letterSpacing: "-0.035em"
  clockInput:
    fontFamily: '"Segoe UI Variable Text", "Segoe UI", "Microsoft YaHei UI", system-ui, sans-serif'
    fontSize: "2.25rem"
    fontWeight: 680
    lineHeight: 1
    letterSpacing: "-0.03em"
  headline:
    fontFamily: '"Segoe UI Variable Text", "Segoe UI", "Microsoft YaHei UI", system-ui, sans-serif'
    fontSize: "1.45rem"
    fontWeight: 680
    lineHeight: 1.2
    letterSpacing: "-0.022em"
  title:
    fontFamily: '"Segoe UI Variable Text", "Segoe UI", "Microsoft YaHei UI", system-ui, sans-serif'
    fontSize: "0.91rem"
    fontWeight: 620
    lineHeight: 1.25
  body:
    fontFamily: '"Segoe UI Variable Text", "Segoe UI", "Microsoft YaHei UI", system-ui, sans-serif'
    fontSize: "0.9rem"
    fontWeight: 400
    lineHeight: 1.5
  label:
    fontFamily: '"Segoe UI Variable Text", "Segoe UI", "Microsoft YaHei UI", system-ui, sans-serif'
    fontSize: "0.86rem"
    fontWeight: 600
    lineHeight: 1
  metadata:
    fontFamily: '"Segoe UI Variable Text", "Segoe UI", "Microsoft YaHei UI", system-ui, sans-serif'
    fontSize: "0.75rem"
    fontWeight: 400
    lineHeight: 1.35
rounded:
  progress: "4px"
  sm: "6px"
  md: "10px"
spacing:
  xs: "4px"
  sm: "8px"
  md: "14px"
  lg: "28px"
components:
  button-primary:
    backgroundColor: "{colors.action}"
    textColor: "{colors.on-action}"
    typography: "{typography.label}"
    rounded: "{rounded.sm}"
    padding: "7px 12px"
    height: "36px"
  button-secondary:
    backgroundColor: "{colors.surface-subtle}"
    textColor: "{colors.ink}"
    typography: "{typography.label}"
    rounded: "{rounded.sm}"
    padding: "7px 12px"
    height: "36px"
  button-ghost:
    backgroundColor: "transparent"
    textColor: "{colors.ink-secondary}"
    typography: "{typography.label}"
    rounded: "{rounded.sm}"
    padding: "7px 12px"
    height: "36px"
  field:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
    rounded: "{rounded.sm}"
    padding: "8px 10px"
    height: "38px"
  panel:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
    rounded: "{rounded.md}"
  nav-item-active:
    backgroundColor: "{colors.action-soft}"
    textColor: "{colors.action}"
    rounded: "{rounded.md}"
    size: "44px"
  task-row:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
    rounded: "{rounded.sm}"
---

# Design System: TimePact

## Overview

**Creative North Star: "The Quiet Time Workbench"**

TimePact is a compact desktop productivity environment built for scanning and acting, not for admiring a dashboard. Its cool neutral canvas, narrow working column, restrained teal actions, and small semantic signals keep time commitments legible without turning urgency into visual noise.

The system feels native to Windows and deliberately ordinary in the best sense: familiar controls, light borders, concise labels, and a single strong hierarchy. A task's title and time state lead; supporting metadata and destructive actions stay quiet until needed. Light, dark, and system modes preserve the same semantic roles.

**Key Characteristics:**

- A fixed icon-only rail framing a narrow, centered work area.
- Bordered and tonal surfaces with little decorative depth.
- Teal for action and active timing; coral only for danger and overdue attention.
- Progress lines placed directly above task content.
- Native system typography with tabular numerals for time.
- Short eased state changes that disappear under reduced-motion preferences.

## Colors

The palette is a cool gray-green neutral field with restrained blue-green action color and coral exception states.

### Primary

- **Workbench Teal** (`action`): Primary actions, active navigation, running timers, and positive status messaging.
- **Deep Workbench Teal** (`action-hover`): Hover feedback for primary actions.
- **Mist Teal** (`action-soft`): Selected navigation and active-focus containers without adding elevation.

### Secondary

- **Alert Coral** (`danger`): Overdue or waiting states, destructive hover states, and reminder emphasis.
- **Soft Alert Coral** (`danger-soft`): Low-intensity backing for danger and error messaging.

### Neutral

- **Cool Canvas** (`canvas`): The desktop work field behind all surfaces.
- **Clean Surface** (`surface`): Navigation rail, task rows, panels, fields, and grouped content.
- **Subtle Surface** (`surface-subtle`): Secondary controls, schedule trays, and tonal separation.
- **Primary Ink** (`ink`): Titles, task names, timer digits, and primary copy.
- **Secondary Ink** (`ink-secondary`): Descriptions, metadata, quiet controls, and helper text.
- **Tertiary Ink** (`ink-tertiary`): Placeholders, counts, and the least prominent supporting text; retain the shipped light-theme value (`#65736f`) for legibility.
- **Quiet Border** (`border`) and **Strong Border** (`border-strong`): Surface boundaries and interactive control outlines.
- **Paused Sage** (`paused`) and **Unscheduled Gray** (`unscheduled`): Semantic progress states that remain quieter than active or urgent work.

The `-dark` color tokens are exact dark-theme counterparts. System mode uses them only when the operating system requests dark appearance.

### Named Rules

**The Semantic Theme Rule.** Theme changes swap values, never meanings; canvas, surface, ink, action, danger, paused, and unscheduled retain their roles in light, dark, and system modes.

**The Exception Color Rule.** Teal denotes action or active time, while coral is reserved for danger, overdue attention, and destructive feedback.

## Typography

**Display Font:** Segoe UI Variable Text, falling back through Segoe UI, Microsoft YaHei UI, system UI, and sans-serif.

**Body Font:** The same native system stack.

**Numeric Font:** Segoe UI Variable Text, Segoe UI, system UI, and sans-serif with tabular numerals.

**Character:** The type system is native, compact, and informational. Weight and spacing create hierarchy; there is no separate decorative display face.

### Hierarchy

- **Display** (620, `clamp(3.3rem, 9vw, 5rem)`, 1): The full focus timer only.
- **Input Display** (620, `2.8rem`, 1): Directly editable duration values in the focus setup stepper.
- **Clock Input** (680, `2.25rem`, 1): Hour and minute segments in the compact Deadline time wheel.
- **Headline** (680, `1.45rem`, 1.2): View titles.
- **Title** (620, `0.91rem`, 1.25): Task names and compact content titles.
- **Body** (400, `0.9rem`, 1.5): Descriptions and explanatory copy, usually constrained to about 62 characters.
- **Label** (600, `0.86rem`, 1): Buttons and compact controls.
- **Metadata** (400, `0.75rem`, 1.35): Task metadata, section counts, and secondary timestamps.

### Named Rules

**The Native Clarity Rule.** Use the system sans stack everywhere; hierarchy comes from size, weight, and density, not a contrasting display family.

**The Time Is Tabular Rule.** All countdowns, deadlines, counts, durations, and metrics use tabular numerals so values do not shift as they change.

## Layout

The desktop shell is a two-column grid: a fixed 64px icon rail and a flexible content region. General views center within a 760px maximum and use 28px horizontal padding; the home task view narrows further to 720px. The default 920×720 window and the supported 760×600 floor show the same rail-first composition.

Content follows a compact vertical rhythm: 18px around view headers, 24px beneath the composer, 22px between task groups, and 8px between task rows. Home keeps the composer first, the small focus launcher immediately after it, and urgency-grouped tasks below.

At 760px, view padding contracts to 20px horizontally. At 700px, task time moves below the task title and row actions occupy the right edge across both lines. Statistics shift from four columns to two at 720px. The rail remains 64px and icon-only at the compact floor.

### Named Rules

**The Narrow Workbench Rule.** Keep the primary working column centered and no wider than 760px; home task lists stop at 720px.

**The Scan Before Detail Rule.** Preserve the order composer, focus launcher, grouped tasks; within a row, title and primary time state precede metadata and actions.

## Elevation & Depth

The shipped system is flat by default. Canvas-to-surface tone changes, 1px borders, and inset grouping create depth; ordinary panels, task rows, fields, navigation, and metrics do not use shadows. State is expressed through color, border strength, and the progress line rather than lift.

### Named Rules

**The Bordered Surface Rule.** Separate working surfaces with tonal contrast and a quiet 1px border; do not add ambient shadows to routine containers.

## Shapes

Corners are gently compact: controls, task rows, and small fields use a 6px radius; navigation states, panels, the composer, and focus containers use 10px. Progress ends use a 4px curve. Completion controls and the brand timer/check mark are the only recurring circular forms.

Borders are light and functional. Containers clip progress bars and expanding trays to their rounded silhouette. The form language avoids oversized pills; circular geometry is reserved for completion and timer identity.

## Components

### Buttons

- **Shape:** Compact rectangles with gently curved corners (6px) and a 36px minimum height.
- **Primary:** White label on Workbench Teal with 7px × 12px padding; hover shifts to Deep Workbench Teal.
- **Secondary:** Primary Ink on Subtle Surface; hover uses Hover Surface.
- **Ghost:** Secondary Ink on transparent; hover gains Hover Surface and Primary Ink.
- **Danger:** Alert Coral on Soft Alert Coral.
- **Focus:** Every button receives a 2px focus-ring outline with a 2px offset.

### Cards / Containers

- **Corner Style:** 10px for general panels; 6px for compact task rows.
- **Background:** Clean Surface over Cool Canvas.
- **Shadow Strategy:** None at rest.
- **Border:** One quiet border around the complete surface.
- **Internal Padding:** Context-specific and compact, typically 14px for panels and 9–15px for rows.

### Inputs / Fields

- **Style:** Clean Surface, Primary Ink, a quiet 1px border, 6px corners, and 8px × 10px padding.
- **Hover:** Border strengthens from Quiet Border to Strong Border.
- **Focus:** Border becomes Workbench Teal with a low-opacity teal outline; the global focus-visible ring remains the keyboard affordance.
- **Placeholder:** Tertiary Ink (`#65736f`) keeps placeholder text quiet but readable.

### Navigation

The navigation rail is 64px wide and uses 44px square icon-only destinations with accessible names and native tooltips. Default icons use Secondary Ink; hover adds Hover Surface and Primary Ink. The active destination uses Mist Teal, Workbench Teal, and a 3px left indicator. Settings anchors to the bottom of the rail.

### Task Composer

The composer is a single bordered panel. Its collapsed row is at least 46px high and places the add icon, title field, expansion control, and primary submit action on one line. Expansion reveals labeled fields in the same panel beneath a quiet top divider. After submission, the composer clears and collapses so keyboard focus can continue into the updated task flow.

Deadline is the only new-task time model. Its picker combines four compact presets with a custom Monday-first calendar and a unified hour/minute time wheel. The direct date and time controls share compact 208px columns and 56px bordered input frames, so both their top and bottom edges align; below the narrow breakpoint they stack in the same reading order. Each hour/minute segment overlays its semi-transparent increment and decrement buttons across the top and bottom of the number field, keeping the controls inside the frame without reserving a separate button column; direct numeric entry and hover-wheel adjustment remain available. Today and tomorrow use the configured work-end time for the matching weekday; Sunday and the last calendar day resolve to 23:59. The calendar stays inline with the task flow, uses the floating-surface elevation token, disables past days, and marks both today and the selected day without relying on platform date-time fields. The active preset uses the existing teal selected state; expired choices are visibly disabled. Persisted legacy countdowns remain readable without reintroducing countdown as a creation choice.

Focus and rest steppers are editable settings, not per-visit drafts. A change updates the shared Pinia preset immediately so the home launcher reflects it on navigation, then persists through the Tauri settings boundary after a short debounce and again when leaving the view.

Weekly work-end settings are stored as seven Monday-first `HH:mm` values. They drive the today/tomorrow Deadline shortcuts according to the target date while leaving week-end and month-end shortcuts at 23:59.

### Task Row

Every task row begins with a 4px semantic progress line directly above its body. Teal marks active timing, coral marks waiting or urgent work, paused sage marks paused work, and unscheduled gray marks items without a timer. The 62px-minimum body prioritizes completion, title, and time; metadata and actions remain smaller and quieter.

Progress changes use a 350ms ease-out transform. Hover and focus feedback use 160ms ease-out color and background changes. Reduced-motion preference collapses transitions and animations to effectively instantaneous behavior.

### Focus Launcher

The home launcher is a compact secondary button that shows a timer icon, action label, and tabular focus/rest preset. An active focus session becomes a soft-teal 10px container with its status, remaining time, and a separate pause/resume action.

While a focus phase is actively counting down, TimePact creates one transparent, always-on-top, click-through Tauri overlay for every connected Windows display. Each overlay uses that monitor's exact physical origin and resolution, including negative desktop coordinates, portrait displays, arbitrary resolutions, and mixed-DPI arrangements. A 12px teal-green aura follows each physical screen edge: its darkest 1px sits against the display boundary and fades inward without covering or intercepting other applications. The active overlay set resynchronizes every two seconds so monitor connections, resolution changes, orientation changes, and DPI changes are reflected during a session. The aura gently breathes to communicate that focus is live, becomes static when reduced motion is requested, and disappears during pause, rest, waiting, and completed states. Browser-only preview intentionally keeps the effect inside no application surface because system-screen overlays require the native Tauri runtime.

## Do's and Don'ts

### Do:

- **Do** preserve the 64px icon rail and narrow centered work column across desktop sizes.
- **Do** put the semantic progress line directly above every task row.
- **Do** use tabular numerals for all changing time and metric values.
- **Do** keep task titles and primary time states visually dominant over metadata and row actions.
- **Do** preserve focus-visible outlines, accessible names for icon-only controls, keyboard operation, and reduced-motion behavior.
- **Do** maintain the same semantic token names when adding or changing theme values.

### Don't:

- **Don't** turn the home surface into a dashboard of summary cards, charts, streaks, or decorative productivity chrome.
- **Don't** add search or filter controls to the shipped home hierarchy; the current surface is an urgency-sorted working list.
- **Don't** use coral as a general accent or teal as decoration; both colors carry state.
- **Don't** add ambient shadows to ordinary rows and panels.
- **Don't** introduce decorative display faces, uppercase kickers, glyph characters as icons, or hard offset shadows.
- **Don't** promote one-off surface concepts into global components or tokens without shipped reuse.
