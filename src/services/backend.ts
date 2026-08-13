import { invoke, type InvokeArgs } from "@tauri-apps/api/core";
import type {
  AppSettings,
  FocusFinishResult,
  FocusRecord,
  FocusSession,
  Task,
  TaskDraft,
} from "../domain/types";
import { createTaskFromDraft } from "../domain/task";
import {
  DEFAULT_WORKDAY_END_TIMES,
  normalizeWorkdayEndTimes,
} from "../domain/work-hours";

const TASK_KEY = "timepact.tasks.v1";
const ARCHIVE_KEY = "timepact.archive.v1";
const FOCUS_KEY = "timepact.focus.v1";
const FOCUS_HISTORY_KEY = "timepact.focusHistory.v1";
const SETTINGS_KEY = "timepact.settings.v1";

const defaultSettings: AppSettings = {
  theme: "system",
  autostart: true,
  repeatReminderMinutes: null,
  focusMinutes: 25,
  restMinutes: 5,
  workdayEndTimes: [...DEFAULT_WORKDAY_END_TIMES],
  sounds: { taskDue: null, focusDone: null, restDone: null },
};

function normalizeSettings(value: Partial<AppSettings> | null | undefined): AppSettings {
  return {
    ...defaultSettings,
    ...value,
    workdayEndTimes: normalizeWorkdayEndTimes(value?.workdayEndTimes),
    sounds: {
      ...defaultSettings.sounds,
      ...value?.sounds,
    },
  };
}

function isNative() {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

function read<T>(key: string, fallback: T): T {
  try {
    const value = localStorage.getItem(key);
    return value ? (JSON.parse(value) as T) : fallback;
  } catch {
    return fallback;
  }
}

function write<T>(key: string, value: T) {
  localStorage.setItem(key, JSON.stringify(value));
}

async function nativeOrFallback<T>(
  command: string,
  args: InvokeArgs,
  fallback: () => T | Promise<T>,
) {
  if (isNative()) {
    return invoke<T>(command, args);
  }
  return fallback();
}

export const backend = {
  listTasks: () =>
    nativeOrFallback<Task[]>("list_active_tasks", {}, () => read<Task[]>(TASK_KEY, [])),

  listArchive: () =>
    nativeOrFallback<Task[]>("list_archived_tasks", {}, () => read<Task[]>(ARCHIVE_KEY, [])),

  createTask: (draft: TaskDraft) => {
    const nativeDraft = {
      ...draft,
      deadlineAt: draft.deadlineAt ? new Date(draft.deadlineAt).toISOString() : null,
    };
    return nativeOrFallback<Task>("create_task", { draft: nativeDraft }, () => {
      const task = createTaskFromDraft(draft);
      const tasks = read<Task[]>(TASK_KEY, []);
      write(TASK_KEY, [task, ...tasks]);
      return task;
    });
  },

  saveTask: (task: Task) =>
    nativeOrFallback<Task>("save_task", { task }, () => {
      const tasks = read<Task[]>(TASK_KEY, []);
      write(
        TASK_KEY,
        tasks.map((item) => (item.id === task.id ? task : item)),
      );
      return task;
    }),

  completeTask: (task: Task) =>
    nativeOrFallback<Task>("complete_task", { taskId: task.id }, () => {
      const completed = {
        ...task,
        status: "completed" as const,
        completedAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      write(
        TASK_KEY,
        read<Task[]>(TASK_KEY, []).filter((item) => item.id !== task.id),
      );
      write(ARCHIVE_KEY, [completed, ...read<Task[]>(ARCHIVE_KEY, [])]);
      return completed;
    }),

  cancelTask: (task: Task) =>
    nativeOrFallback<Task>("cancel_task", { taskId: task.id }, () => {
      const cancelled = {
        ...task,
        status: "cancelled" as const,
        completedAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      };
      write(
        TASK_KEY,
        read<Task[]>(TASK_KEY, []).filter((item) => item.id !== task.id),
      );
      write(ARCHIVE_KEY, [cancelled, ...read<Task[]>(ARCHIVE_KEY, [])]);
      return cancelled;
    }),

  removeTask: (taskId: string) =>
    nativeOrFallback<void>("delete_task", { taskId }, () => {
      write(
        TASK_KEY,
        read<Task[]>(TASK_KEY, []).filter((item) => item.id !== taskId),
      );
    }),

  getSettings: () =>
    nativeOrFallback<Partial<AppSettings>>("get_settings", {}, () =>
      read<Partial<AppSettings>>(SETTINGS_KEY, {}),
    ).then(normalizeSettings),

  saveSettings: (settings: AppSettings) => {
    const normalized = normalizeSettings(settings);
    return nativeOrFallback<AppSettings>("update_settings", { settings: normalized }, () => {
      write(SETTINGS_KEY, normalized);
      return normalized;
    });
  },

  getFocus: () =>
    nativeOrFallback<FocusSession | null>("get_active_focus", {}, () =>
      read<FocusSession | null>(FOCUS_KEY, null),
    ),

  saveFocus: (focus: FocusSession | null) =>
    nativeOrFallback<FocusSession | null>("save_focus", { focus }, () => {
      write(FOCUS_KEY, focus);
      return focus;
    }),

  listFocusHistory: () =>
    nativeOrFallback<FocusRecord[]>("list_focus_records", {}, () =>
      read<FocusRecord[]>(FOCUS_HISTORY_KEY, []),
    ),

  finishFocus: (focus: FocusSession) =>
    nativeOrFallback<FocusFinishResult>("finish_focus", { focus }, () => {
      const record: FocusRecord = {
        id: focus.id,
        taskId: focus.taskId,
        startedAt: focus.startedAt,
        endedAt: new Date().toISOString(),
        focusedSeconds: Math.max(0, focus.elapsedSeconds),
        rounds: Math.max(1, focus.round),
      };
      let updatedTask: Task | null = null;
      if (focus.taskId) {
        for (const key of [TASK_KEY, ARCHIVE_KEY]) {
          const items = read<Task[]>(key, []);
          const index = items.findIndex((task) => task.id === focus.taskId);
          if (index >= 0) {
            updatedTask = {
              ...items[index],
              focusedSeconds: items[index].focusedSeconds + record.focusedSeconds,
              updatedAt: new Date().toISOString(),
            };
            items[index] = updatedTask;
            write(key, items);
            break;
          }
        }
      }
      write(FOCUS_HISTORY_KEY, [record, ...read<FocusRecord[]>(FOCUS_HISTORY_KEY, [])]);
      write(FOCUS_KEY, null);
      return { task: updatedTask, record };
    }),

  exportBackup: () =>
    nativeOrFallback<string>("export_backup_json", {}, () =>
      JSON.stringify(
        {
          formatVersion: 1,
          exportedAt: new Date().toISOString(),
          tasks: [...read<Task[]>(TASK_KEY, []), ...read<Task[]>(ARCHIVE_KEY, [])],
          settings: read<AppSettings>(SETTINGS_KEY, defaultSettings),
          focus: read<FocusSession | null>(FOCUS_KEY, null),
          focusHistory: read<FocusRecord[]>(FOCUS_HISTORY_KEY, []),
        },
        null,
        2,
      ),
    ),

  importBackup: (content: string) =>
    nativeOrFallback<void>("import_backup_json", { content }, () => {
      const backup = JSON.parse(content) as {
        formatVersion: number;
        tasks: Task[];
        settings: AppSettings;
        focus: FocusSession | null;
        focusHistory?: FocusRecord[];
      };
      if (backup.formatVersion !== 1 || !Array.isArray(backup.tasks)) {
        throw new Error("备份格式无效");
      }
      write(
        TASK_KEY,
        backup.tasks.filter((task) => !["completed", "cancelled"].includes(task.status)),
      );
      write(
        ARCHIVE_KEY,
        backup.tasks.filter((task) => ["completed", "cancelled"].includes(task.status)),
      );
      write(SETTINGS_KEY, backup.settings);
      write(FOCUS_KEY, backup.focus);
      write(FOCUS_HISTORY_KEY, backup.focusHistory ?? []);
    }),

  exportCsv: () =>
    nativeOrFallback<string>("export_archive_csv", {}, () => {
      const escape = (value: unknown) => `"${String(value ?? "").replace(/"/g, '""')}"`;
      const header =
        "记录类型,标题或关联待办,状态,创建或开始时间,完成或结束时间,原始截止时间,延期次数,延期秒数,专注秒数,总耗时秒数";
      const rows = read<Task[]>(ARCHIVE_KEY, []).map((task) => {
        const total = task.completedAt
          ? Math.max(0, Math.round((+new Date(task.completedAt) - +new Date(task.createdAt)) / 1000))
          : 0;
        return [
          "待办",
          task.title,
          task.status,
          task.createdAt,
          task.completedAt,
          task.timer?.originalDueAt,
          task.postponementCount,
          task.totalPostponementSeconds,
          task.focusedSeconds,
          total,
        ]
          .map(escape)
          .join(",");
      });
      const focusRows = read<FocusRecord[]>(FOCUS_HISTORY_KEY, []).map((record) =>
        [
          "专注",
          record.taskId ?? "自由专注",
          "completed",
          record.startedAt,
          record.endedAt,
          "",
          0,
          0,
          record.focusedSeconds,
          record.focusedSeconds,
        ]
          .map(escape)
          .join(","),
      );
      return `\ufeff${[header, ...rows, ...focusRows].join("\n")}`;
    }),

  writeExportFile: (path: string, content: string) =>
    nativeOrFallback<void>("write_export_file", { path, content }, () => {
      const blob = new Blob([content], { type: "application/octet-stream" });
      const anchor = document.createElement("a");
      anchor.href = URL.createObjectURL(blob);
      anchor.download = path.split(/[\\/]/).pop() || "timepact-export";
      anchor.click();
      URL.revokeObjectURL(anchor.href);
    }),

  readImportFile: (path: string) =>
    nativeOrFallback<string>("read_import_file", { path }, () =>
      Promise.reject(new Error("网页预览请使用文件内容导入。")),
    ),

  getDataDirectory: () =>
    nativeOrFallback<string>("get_data_directory", {}, () => ""),
};

export const runningInTauri = isNative;
