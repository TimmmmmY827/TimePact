import { acceptHMRUpdate, defineStore } from "pinia";
import { computed, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { backend, runningInTauri } from "../services/backend";
import { playSound } from "../services/sound";
import { sortByUrgency, taskBand } from "../domain/task";
import type {
  AppSettings,
  FocusRecord,
  FocusSession,
  Task,
  TaskDraft,
} from "../domain/types";
import { DEFAULT_WORKDAY_END_TIMES } from "../domain/work-hours";

export const useAppStore = defineStore("app", () => {
  const tasks = ref<Task[]>([]);
  const archive = ref<Task[]>([]);
  const focus = ref<FocusSession | null>(null);
  const focusHistory = ref<FocusRecord[]>([]);
  const settings = ref<AppSettings>({
    theme: "system",
    autostart: true,
    repeatReminderMinutes: null,
    focusMinutes: 25,
    restMinutes: 5,
    workdayEndTimes: [...DEFAULT_WORKDAY_END_TIMES],
    sounds: { taskDue: null, focusDone: null, restDone: null },
  });
  const initialized = ref(false);
  const busy = ref(false);
  const error = ref<string | null>(null);
  let tickHandle: number | null = null;
  let focusPersistTicks = 0;

  const orderedTasks = computed(() => sortByUrgency(tasks.value));
  const attentionTasks = computed(() =>
    orderedTasks.value.filter((task) => taskBand(task) === "attention"),
  );
  const activeTasks = computed(() =>
    orderedTasks.value.filter((task) => taskBand(task) === "active"),
  );
  const unscheduledTasks = computed(() =>
    orderedTasks.value.filter((task) => taskBand(task) === "unscheduled"),
  );

  async function initialize() {
    if (initialized.value) return;
    busy.value = true;
    try {
      [tasks.value, archive.value, focus.value, settings.value, focusHistory.value] =
        await Promise.all([
        backend.listTasks(),
        backend.listArchive(),
        backend.getFocus(),
        backend.getSettings(),
        backend.listFocusHistory(),
      ]);
      applyTheme(settings.value.theme);
      startTicker();
      if (runningInTauri()) {
        const reminderWindow = getCurrentWindow().label === "reminder";
        await listen<string[]>("timepact://task-due", async () => {
          tasks.value = await backend.listTasks();
          if (reminderWindow) void playSound(settings.value.sounds.taskDue);
        });
      }
      initialized.value = true;
    } catch (cause) {
      console.error("TimePact initialization failed", cause);
      error.value = "无法读取本地数据，请稍后重试。";
    } finally {
      busy.value = false;
    }
  }

  async function reloadData() {
    [tasks.value, archive.value, focus.value, settings.value, focusHistory.value] =
      await Promise.all([
      backend.listTasks(),
      backend.listArchive(),
      backend.getFocus(),
      backend.getSettings(),
      backend.listFocusHistory(),
    ]);
    applyTheme(settings.value.theme);
  }

  async function addTask(draft: TaskDraft) {
    if (!draft.title.trim()) return;
    const task = await backend.createTask(draft);
    tasks.value = [task, ...tasks.value];
  }

  async function saveTask(task: Task) {
    const saved = await backend.saveTask({ ...task, updatedAt: new Date().toISOString() });
    tasks.value = tasks.value.map((item) => (item.id === saved.id ? saved : item));
    archive.value = archive.value.map((item) => (item.id === saved.id ? saved : item));
  }

  async function completeTask(task: Task) {
    const completed = await backend.completeTask(task);
    tasks.value = tasks.value.filter((item) => item.id !== task.id);
    archive.value = [completed, ...archive.value];
  }

  async function cancelTask(task: Task) {
    const cancelled = await backend.cancelTask(task);
    tasks.value = tasks.value.filter((item) => item.id !== task.id);
    archive.value = [cancelled, ...archive.value];
  }

  async function startCountdown(task: Task) {
    if (!task.timer || task.timer.kind !== "countdown" || task.timer.remainingSeconds == null) return;
    const now = new Date();
    task.status = "running";
    task.timer.locked = true;
    task.timer.startedAt = now.toISOString();
    task.timer.currentDueAt = new Date(now.getTime() + task.timer.remainingSeconds * 1000).toISOString();
    task.timer.pausedAt = null;
    await saveTask(task);
  }

  async function togglePause(task: Task) {
    if (!task.timer || task.timer.kind !== "countdown") return;
    if (task.status === "running" && task.timer.currentDueAt) {
      task.timer.remainingSeconds = Math.max(
        0,
        Math.ceil((new Date(task.timer.currentDueAt).getTime() - Date.now()) / 1000),
      );
      task.timer.currentDueAt = null;
      task.timer.pausedAt = new Date().toISOString();
      task.status = "paused";
    } else if (task.status === "paused" && task.timer.remainingSeconds != null) {
      const now = new Date();
      task.timer.currentDueAt = new Date(now.getTime() + task.timer.remainingSeconds * 1000).toISOString();
      task.timer.pausedAt = null;
      task.status = "running";
    }
    await saveTask(task);
  }

  async function deleteTask(task: Task) {
    await backend.removeTask(task.id);
    tasks.value = tasks.value.filter((item) => item.id !== task.id);
  }

  async function postponeTask(task: Task, minutes: number) {
    if (!task.timer || minutes < 1) return;
    const seconds = Math.round(minutes * 60);
    const now = new Date();
    task.postponementCount += 1;
    task.totalPostponementSeconds += seconds;
    task.status = task.timer.kind === "countdown" ? "running" : "ready";
    task.timer.remainingSeconds = task.timer.kind === "countdown" ? seconds : task.timer.remainingSeconds;
    task.timer.currentDueAt = new Date(now.getTime() + seconds * 1000).toISOString();
    task.timer.pausedAt = null;
    await saveTask(task);
  }

  async function startFocus(taskId: string | null = null) {
    if (focus.value) return;
    const total = settings.value.focusMinutes * 60;
    focus.value = {
      id: crypto.randomUUID(),
      taskId,
      phase: "focus",
      plannedFocusSeconds: total,
      plannedRestSeconds: settings.value.restMinutes * 60,
      remainingSeconds: total,
      elapsedSeconds: 0,
      pausedSeconds: 0,
      round: 1,
      running: true,
      startedAt: new Date().toISOString(),
    };
    await backend.saveFocus(focus.value);
  }

  async function toggleFocus() {
    if (!focus.value) return;
    focus.value.running = !focus.value.running;
    await backend.saveFocus(focus.value);
  }

  async function beginRest() {
    if (!focus.value || focus.value.phase !== "waiting-rest") return;
    focus.value.phase = "rest";
    focus.value.remainingSeconds = focus.value.plannedRestSeconds;
    focus.value.running = true;
    await backend.saveFocus(focus.value);
  }

  async function continueFocus() {
    if (!focus.value || focus.value.phase !== "waiting-focus") return;
    focus.value.phase = "focus";
    focus.value.round += 1;
    focus.value.remainingSeconds = focus.value.plannedFocusSeconds;
    focus.value.running = true;
    await backend.saveFocus(focus.value);
  }

  async function skipRest() {
    if (!focus.value || focus.value.phase !== "rest") return;
    focus.value.phase = "waiting-focus";
    focus.value.running = false;
    focus.value.remainingSeconds = 0;
    await backend.saveFocus(focus.value);
  }

  async function endFocus() {
    if (!focus.value) return;
    const result = await backend.finishFocus(focus.value);
    if (result.task) {
      tasks.value = tasks.value.map((item) =>
        item.id === result.task?.id ? result.task : item,
      );
      archive.value = archive.value.map((item) =>
        item.id === result.task?.id ? result.task : item,
      );
    }
    focusHistory.value = [result.record, ...focusHistory.value];
    focus.value = null;
  }

  async function updateSettings(next: AppSettings) {
    settings.value = await backend.saveSettings(next);
    applyTheme(settings.value.theme);
  }

  function applyTheme(theme: AppSettings["theme"]) {
    document.documentElement.dataset.theme = theme;
  }

  function startTicker() {
    if (tickHandle != null) return;
    tickHandle = window.setInterval(() => {
      for (const task of tasks.value) {
        if (
          task.status === "running" &&
          task.timer?.kind === "countdown" &&
          task.timer.currentDueAt
        ) {
          task.timer.remainingSeconds = Math.max(
            0,
            Math.ceil((new Date(task.timer.currentDueAt).getTime() - Date.now()) / 1000),
          );
          if (task.timer.remainingSeconds === 0) task.status = "waiting";
        }
        if (
          task.timer?.kind === "deadline" &&
          task.timer.currentDueAt &&
          new Date(task.timer.currentDueAt).getTime() <= Date.now() &&
          task.status !== "waiting"
        ) {
          task.status = "waiting";
        }
      }
      if (focus.value?.running && focus.value.remainingSeconds > 0) {
        focus.value.remainingSeconds -= 1;
        if (focus.value.phase === "focus") focus.value.elapsedSeconds += 1;
        if (focus.value.remainingSeconds === 0) {
          const completedPhase = focus.value.phase;
          focus.value.running = false;
          focus.value.phase = focus.value.phase === "focus" ? "waiting-rest" : "waiting-focus";
          void backend.saveFocus({ ...focus.value });
          void playSound(
            completedPhase === "focus"
              ? settings.value.sounds.focusDone
              : settings.value.sounds.restDone,
          );
        }
        focusPersistTicks += 1;
        if (focusPersistTicks >= 5 && focus.value) {
          focusPersistTicks = 0;
          void backend.saveFocus({ ...focus.value });
        }
      }
    }, 1000);
  }

  return {
    tasks,
    archive,
    focus,
    focusHistory,
    settings,
    initialized,
    busy,
    error,
    orderedTasks,
    attentionTasks,
    activeTasks,
    unscheduledTasks,
    initialize,
    reloadData,
    addTask,
    saveTask,
    completeTask,
    cancelTask,
    startCountdown,
    togglePause,
    deleteTask,
    postponeTask,
    startFocus,
    toggleFocus,
    beginRest,
    continueFocus,
    skipRest,
    endFocus,
    updateSettings,
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useAppStore, import.meta.hot));
}
