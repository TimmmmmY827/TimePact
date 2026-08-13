<script setup lang="ts">
import { computed, ref } from "vue";
import { CalendarPlus, Check, Clock3, Pause, Play, Trash2, X } from "@lucide/vue";
import { format, formatDistanceToNowStrict } from "date-fns";
import { zhCN } from "date-fns/locale";
import type { Task } from "../../domain/types";
import { isFutureDeadline } from "../../domain/deadline";
import { taskProgress } from "../../domain/task";
import { useAppStore } from "../../stores/app";
import DeadlinePicker from "./DeadlinePicker.vue";

const props = defineProps<{ task: Task }>();
const store = useAppStore();
const scheduling = ref(false);
const deadlineAt = ref("");
const canSchedule = computed(() => isFutureDeadline(deadlineAt.value));

const progress = computed(() => {
  if (props.task.status === "waiting") return 1;
  return taskProgress(props.task);
});

const progressTone = computed(() => {
  if (props.task.status === "waiting") return "danger";
  if (props.task.status === "paused") return "paused";
  if (!props.task.timer) return "unscheduled";
  return "active";
});

const timeLabel = computed(() => {
  const task = props.task;
  if (task.status === "waiting") return "等待确认";
  if (task.status === "paused") return "已暂停";
  if (!task.timer) return "未安排";
  if (task.timer.kind === "countdown") {
    const total = Math.max(0, task.timer.remainingSeconds ?? task.timer.durationSeconds ?? 0);
    const hours = Math.floor(total / 3600);
    const minutes = Math.floor((total % 3600) / 60);
    const seconds = total % 60;
    return `${hours ? `${hours}:` : ""}${minutes.toString().padStart(2, "0")}:${seconds
      .toString()
      .padStart(2, "0")}`;
  }
  if (task.timer.currentDueAt) {
    const due = new Date(task.timer.currentDueAt);
    if (due.getTime() <= Date.now()) {
      return `已逾期 ${formatDistanceToNowStrict(due, { locale: zhCN })}`;
    }
    return format(due, "M月d日 HH:mm");
  }
  return "未安排";
});

const metadata = computed(() => {
  const task = props.task;
  const parts = [task.priority === "high" ? "高优先级" : task.priority === "low" ? "低优先级" : ""];
  if (task.tags.length) parts.push(task.tags.slice(0, 2).join(" · "));
  if (task.timer) parts.push("Deadline");
  return parts.filter(Boolean).join(" · ") || "未添加详情";
});

async function primaryTimerAction() {
  if (props.task.timer?.kind !== "countdown") return;
  if (props.task.status === "ready") await store.startCountdown(props.task);
  else if (props.task.status === "running" || props.task.status === "paused") {
    await store.togglePause(props.task);
  }
}

async function applySchedule() {
  if (props.task.timer || !canSchedule.value) return;
  const now = new Date();
  const due = new Date(deadlineAt.value);
  props.task.timer = {
    kind: "deadline",
    originalDueAt: due.toISOString(),
    currentDueAt: due.toISOString(),
    durationSeconds: null,
    remainingSeconds: null,
    startedAt: now.toISOString(),
    locked: true,
    pausedAt: null,
  };
  props.task.status = "ready";
  await store.saveTask(props.task);
  scheduling.value = false;
}

async function cancel() {
  if (window.confirm(`取消并归档“${props.task.title}”？`)) {
    await store.cancelTask(props.task);
  }
}
</script>

<template>
  <article class="task-row" :class="`tone-${progressTone}`">
    <div class="task-progress" aria-hidden="true">
      <span :style="{ transform: `scaleX(${Math.max(progress, task.timer ? 0.03 : 1)})` }" />
    </div>
    <div class="task-body">
      <button
        class="task-complete"
        type="button"
        :aria-label="`完成：${task.title}`"
        title="标记完成"
        @click="store.completeTask(task)"
      >
        <Check :size="15" aria-hidden="true" />
      </button>

      <div class="task-copy">
        <strong>{{ task.title }}</strong>
        <span>{{ metadata }}</span>
      </div>

      <div class="task-time" :class="{ urgent: progressTone === 'danger' }">
        <Clock3 v-if="task.timer" :size="15" aria-hidden="true" />
        <span class="numeric">{{ timeLabel }}</span>
      </div>

      <div class="task-actions">
        <button
          v-if="task.timer?.kind === 'countdown'"
          class="icon-button"
          type="button"
          :aria-label="task.status === 'running' ? '暂停 Deadline 计时' : '开始或继续 Deadline 计时'"
          @click="primaryTimerAction"
        >
          <Pause v-if="task.status === 'running'" :size="17" aria-hidden="true" />
          <Play v-else :size="17" aria-hidden="true" />
        </button>
        <button
          v-if="task.status === 'unscheduled'"
          class="icon-button"
          type="button"
          :aria-label="`为 ${task.title} 设置时间`"
          title="设置时间"
          @click="scheduling = !scheduling"
        >
          <CalendarPlus :size="16" aria-hidden="true" />
        </button>
        <button
          v-if="task.status === 'unscheduled' || (task.timer?.kind === 'countdown' && !task.timer.locked)"
          class="icon-button destructive-on-hover"
          type="button"
          :aria-label="`删除：${task.title}`"
          @click="store.deleteTask(task)"
        >
          <Trash2 :size="16" aria-hidden="true" />
        </button>
        <button
          v-else
          class="icon-button destructive-on-hover"
          type="button"
          :aria-label="`取消并归档：${task.title}`"
          title="取消并归档"
          @click="cancel"
        >
          <X :size="17" aria-hidden="true" />
        </button>
      </div>
    </div>
    <form v-if="scheduling" class="schedule-panel" @submit.prevent="applySchedule">
      <DeadlinePicker
        v-model="deadlineAt"
        :workday-end-times="store.settings.workdayEndTimes"
      />
      <div class="schedule-actions">
        <button class="button primary" type="submit" :disabled="!canSchedule">确认</button>
        <button class="button ghost" type="button" @click="scheduling = false">取消</button>
      </div>
    </form>
  </article>
</template>

<style scoped>
.task-row {
  --tone: var(--accent);
  --progress: var(--accent);
  position: relative;
  overflow: clip;
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
}

.task-row.tone-danger {
  --tone: var(--danger);
  --progress: var(--danger);
}

.task-row.tone-paused {
  --tone: var(--paused);
  --progress: var(--paused);
}

.task-row.tone-unscheduled {
  --tone: var(--text-tertiary);
  --progress: var(--unscheduled);
}

.task-progress {
  width: 100%;
  height: 4px;
  overflow: hidden;
  background: color-mix(in srgb, var(--progress) 20%, var(--bg-subtle));
}

.task-progress span {
  display: block;
  width: 100%;
  height: 100%;
  background: var(--progress);
  border-radius: 0 4px 4px 0;
  transform-origin: left center;
  transition: transform 350ms var(--ease-out);
}

.task-body {
  display: grid;
  grid-template-columns: auto minmax(0, 1fr) auto auto;
  align-items: center;
  gap: 10px;
  min-height: 62px;
  padding: 9px 9px 9px 12px;
}

.task-complete {
  display: grid;
  width: 24px;
  height: 24px;
  place-items: center;
  color: transparent;
  background: transparent;
  border: 1.5px solid var(--border-strong);
  border-radius: 50%;
  cursor: pointer;
  transition:
    color 160ms var(--ease-out),
    border-color 160ms var(--ease-out),
    background-color 160ms var(--ease-out);
}

.task-complete:hover,
.task-complete:focus-visible {
  color: #fff;
  background: var(--accent);
  border-color: var(--accent);
}

.task-copy {
  display: grid;
  min-width: 0;
  gap: 3px;
  padding: 2px 0;
  text-align: left;
}

.task-copy strong {
  overflow: hidden;
  font-size: 0.91rem;
  font-weight: 620;
  line-height: 1.25;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.task-copy span {
  overflow: hidden;
  color: var(--text-secondary);
  font-size: 0.75rem;
  line-height: 1.35;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.task-time {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  color: var(--tone);
  font-size: 0.81rem;
  font-weight: 600;
  white-space: nowrap;
}

.task-time.urgent {
  color: var(--danger);
}

.task-actions {
  display: flex;
  align-items: center;
}

.destructive-on-hover:hover {
  color: var(--danger);
  background: var(--danger-soft);
}

.schedule-panel {
  display: grid;
  gap: 10px;
  padding: 12px;
  background: var(--bg-subtle);
  border-top: 1px solid var(--border);
}

.schedule-actions {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 7px;
}

@media (max-width: 700px) {
  .task-body {
    grid-template-columns: auto minmax(0, 1fr) auto;
  }

  .task-time {
    grid-column: 2;
    justify-self: start;
  }

  .task-actions {
    grid-row: 1 / span 2;
    grid-column: 3;
  }
}
</style>
