# Development Preview Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a one-click TimePact Preview development application that hot-reloads UI changes, can coexist with the installed application, and never reads or writes production data.

**Architecture:** A CLI config overlay changes the preview product name and Tauri identifier while inheriting the production configuration through RFC 7396 merge behavior. A PowerShell launcher establishes the known-good Visual Studio and Rust environment, while a root CMD file provides the double-click entry point. Debug builds suppress autostart mutations and display a Preview badge.

**Tech Stack:** Tauri 2, Vue 3, Rust, PowerShell, Windows CMD.

## Global Constraints

- Preview identifier is `com.timepact.desktop.preview`.
- Preview product name and main-window title are `TimePact Preview`.
- Preview data must be stored outside `com.timepact.desktop`.
- Preview and installed production applications must run simultaneously.
- Preview must not enable, disable, or overwrite production autostart configuration.
- Frontend changes use Vite HMR; Rust/config changes require only a preview restart.

---

### Task 1: Preview flavour and safety boundary

**Files:**
- Create: `src-tauri/tauri.preview.conf.json`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/src/commands.rs`
- Modify: `src/components/shell/WindowTitlebar.vue`

**Interfaces:**
- Consumes: Tauri CLI `--config` JSON merge and Rust `cfg!(debug_assertions)`.
- Produces: a debug-only app with an independent identifier, data directory, window state, and visible Preview label.

- [ ] **Step 1: Add the preview config overlay**

Create a JSON overlay containing:

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "TimePact Preview",
  "identifier": "com.timepact.desktop.preview",
  "app": {
    "windows": [
      {
        "label": "main",
        "title": "TimePact Preview",
        "width": 820,
        "height": 640,
        "minWidth": 700,
        "minHeight": 540,
        "center": true,
        "decorations": false,
        "resizable": true
      },
      {
        "label": "reminder",
        "title": "TimePact Preview Reminder",
        "url": "index.html#/reminder",
        "width": 560,
        "height": 420,
        "minWidth": 480,
        "minHeight": 340,
        "center": true,
        "resizable": true,
        "alwaysOnTop": true,
        "visible": false,
        "maximizable": false,
        "minimizable": false
      }
    ]
  }
}
```

- [ ] **Step 2: Block debug autostart mutations**

Gate setup-time autostart enabling and `update_settings` plugin calls with `!cfg!(debug_assertions)` so preview settings remain local data only.

- [ ] **Step 3: Mark the preview visually**

Render a small `Preview` badge in the custom titlebar only when `import.meta.env.DEV` is true.

### Task 2: One-click launcher and verification

**Files:**
- Create: `scripts/Start-TimePact-Preview.ps1`
- Create: `Start-TimePact-Preview.cmd`
- Modify: `package.json`

**Interfaces:**
- Consumes: local Node dependencies, Visual Studio Build Tools, and `C:\Users\Tim\AppData\Local\Programs\Rust\bin`.
- Produces: `npm run preview:dev`, launched by a double-clickable root CMD file.

- [ ] **Step 1: Add the npm command**

Add:

```json
"preview:dev": "tauri dev --config src-tauri/tauri.preview.conf.json"
```

- [ ] **Step 2: Add the PowerShell launcher**

Resolve the project root from `$PSScriptRoot`, import the x64 Visual Studio developer environment, prepend the standalone Rust path, verify `node_modules`, and run `npm run preview:dev`.

- [ ] **Step 3: Add the CMD entry point**

Invoke the PowerShell launcher with `-NoProfile -ExecutionPolicy Bypass`; pause only when startup exits with an error.

- [ ] **Step 4: Verify**

Run `npm run test:run`, `npm run typecheck`, Rust tests, and the CMD launcher. Confirm the installed executable and `target\debug\TimePact.exe` coexist, the preview title is present, and a new preview data directory contains its own `timepact.db`.
