export type Priority = "low" | "medium" | "high";
export type TimerKind = "deadline" | "countdown" | null;
export type TaskStatus =
  | "unscheduled"
  | "ready"
  | "running"
  | "paused"
  | "waiting"
  | "completed"
  | "cancelled";

export interface TaskTimer {
  kind: Exclude<TimerKind, null>;
  originalDueAt: string | null;
  currentDueAt: string | null;
  durationSeconds: number | null;
  remainingSeconds: number | null;
  startedAt: string | null;
  locked: boolean;
  pausedAt: string | null;
}

export interface Task {
  id: string;
  cycleId: string;
  title: string;
  notes: string;
  priority: Priority;
  tags: string[];
  status: TaskStatus;
  timer: TaskTimer | null;
  createdAt: string;
  updatedAt: string;
  completedAt: string | null;
  postponementCount: number;
  totalPostponementSeconds: number;
  focusedSeconds: number;
}

export interface TaskDraft {
  title: string;
  notes?: string;
  priority?: Priority;
  tags?: string[];
  timerKind?: TimerKind;
  deadlineAt?: string | null;
  countdownMinutes?: number | null;
}

export type FocusPhase = "focus" | "rest" | "waiting-focus" | "waiting-rest";

export interface FocusSession {
  id: string;
  taskId: string | null;
  phase: FocusPhase;
  plannedFocusSeconds: number;
  plannedRestSeconds: number;
  remainingSeconds: number;
  elapsedSeconds: number;
  pausedSeconds: number;
  round: number;
  running: boolean;
  startedAt: string;
}

export interface FocusRecord {
  id: string;
  taskId: string | null;
  startedAt: string;
  endedAt: string;
  focusedSeconds: number;
  rounds: number;
}

export interface FocusFinishResult {
  task: Task | null;
  record: FocusRecord;
}

export interface AppSettings {
  theme: "system" | "light" | "dark";
  autostart: boolean;
  repeatReminderMinutes: number | null;
  focusMinutes: number;
  restMinutes: number;
  workdayEndTimes: string[];
  sounds: {
    taskDue: string | null;
    focusDone: string | null;
    restDone: string | null;
  };
}

export interface PeriodStats {
  completedCycles: number;
  distinctTasks: number;
  onTimeRate: number;
  postponements: number;
  focusedSeconds: number;
  averageLifecycleSeconds: number;
}
