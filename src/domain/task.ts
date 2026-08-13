import type { Task, TaskDraft } from "./types";

const UUID_FALLBACK = () =>
  `${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}`;

export function createTaskFromDraft(draft: TaskDraft, now = new Date()): Task {
  const id = globalThis.crypto?.randomUUID?.() ?? UUID_FALLBACK();
  const createdAt = now.toISOString();
  const timer =
    draft.timerKind === "deadline" && draft.deadlineAt
      ? {
          kind: "deadline" as const,
          originalDueAt: new Date(draft.deadlineAt).toISOString(),
          currentDueAt: new Date(draft.deadlineAt).toISOString(),
          durationSeconds: null,
          remainingSeconds: null,
          startedAt: createdAt,
          locked: true,
          pausedAt: null,
        }
      : draft.timerKind === "countdown" && draft.countdownMinutes
        ? {
            kind: "countdown" as const,
            originalDueAt: null,
            currentDueAt: null,
            durationSeconds: Math.round(draft.countdownMinutes * 60),
            remainingSeconds: Math.round(draft.countdownMinutes * 60),
            startedAt: null,
            locked: false,
            pausedAt: null,
          }
        : null;

  return {
    id,
    cycleId: globalThis.crypto?.randomUUID?.() ?? UUID_FALLBACK(),
    title: draft.title.trim(),
    notes: draft.notes?.trim() ?? "",
    priority: draft.priority ?? "medium",
    tags: draft.tags ?? [],
    status: timer ? "ready" : "unscheduled",
    timer,
    createdAt,
    updatedAt: createdAt,
    completedAt: null,
    postponementCount: 0,
    totalPostponementSeconds: 0,
    focusedSeconds: 0,
  };
}

export function urgencyTimestamp(task: Task, nowMs = Date.now()): number {
  if (task.status === "waiting") return Number.NEGATIVE_INFINITY;
  if (task.timer?.kind === "deadline" && task.timer.currentDueAt) {
    return new Date(task.timer.currentDueAt).getTime();
  }
  if (task.timer?.kind === "countdown" && task.timer.remainingSeconds != null) {
    return nowMs + task.timer.remainingSeconds * 1000;
  }
  return Number.POSITIVE_INFINITY;
}

export function sortByUrgency(tasks: Task[]): Task[] {
  return [...tasks].sort((a, b) => {
    const urgencyA = urgencyTimestamp(a);
    const urgencyB = urgencyTimestamp(b);
    if (urgencyA !== urgencyB) return urgencyA < urgencyB ? -1 : 1;
    const priority = { high: 0, medium: 1, low: 2 };
    const priorityDelta = priority[a.priority] - priority[b.priority];
    return priorityDelta || a.createdAt.localeCompare(b.createdAt);
  });
}

export function taskProgress(task: Task, nowMs = Date.now()): number {
  const timer = task.timer;
  if (!timer) return 0;
  if (timer.kind === "countdown" && timer.durationSeconds && timer.remainingSeconds != null) {
    return Math.min(1, Math.max(0, 1 - timer.remainingSeconds / timer.durationSeconds));
  }
  if (timer.kind === "deadline" && timer.originalDueAt) {
    const start = new Date(timer.startedAt ?? task.createdAt).getTime();
    const end = new Date(timer.originalDueAt).getTime();
    return Math.min(1, Math.max(0, (nowMs - start) / Math.max(1, end - start)));
  }
  return 0;
}

export function taskBand(task: Task): "attention" | "active" | "unscheduled" {
  if (task.status === "waiting") return "attention";
  if (task.timer?.kind === "deadline" && task.timer.currentDueAt) {
    if (new Date(task.timer.currentDueAt).getTime() <= Date.now()) return "attention";
  }
  if (!task.timer) return "unscheduled";
  return "active";
}
