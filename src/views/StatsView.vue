<script setup lang="ts">
import { computed, ref } from "vue";
import { Archive, CalendarRange, Download } from "@lucide/vue";
import { differenceInSeconds, format } from "date-fns";
import { save } from "@tauri-apps/plugin-dialog";
import { useAppStore } from "../stores/app";
import { backend, runningInTauri } from "../services/backend";

const store = useAppStore();
const period = ref<"week" | "month" | "quarter" | "custom">("week");
const customStart = ref(new Date(new Date().setDate(new Date().getDate() - 7)).toISOString().slice(0, 10));
const customEnd = ref(new Date().toISOString().slice(0, 10));
const exportMessage = ref("");

function setPeriod(value: string) {
  period.value = value as typeof period.value;
}

const stats = computed(() => {
  const now = new Date();
  let start = new Date(now);
  if (period.value === "week") start.setDate(now.getDate() - ((now.getDay() + 6) % 7));
  if (period.value === "month") start = new Date(now.getFullYear(), now.getMonth(), 1);
  if (period.value === "quarter") {
    start = new Date(now.getFullYear(), Math.floor(now.getMonth() / 3) * 3, 1);
  }
  if (period.value === "custom") start = new Date(`${customStart.value}T00:00:00`);
  start.setHours(0, 0, 0, 0);
  const end =
    period.value === "custom"
      ? new Date(`${customEnd.value}T23:59:59.999`)
      : new Date(now.getFullYear(), now.getMonth(), now.getDate(), 23, 59, 59, 999);
  const completed = store.archive.filter(
    (task) =>
      task.status === "completed" &&
      task.completedAt &&
      new Date(task.completedAt) >= start &&
      new Date(task.completedAt) <= end,
  );
  const onTime = completed.filter((task) => {
    const original = task.timer?.originalDueAt;
    return !original || !task.completedAt || new Date(task.completedAt) <= new Date(original);
  });
  const lifecycle = completed.reduce((sum, task) => {
    if (!task.completedAt) return sum;
    return sum + Math.max(0, differenceInSeconds(task.completedAt, task.createdAt));
  }, 0);
  return {
    completed: completed.length,
    onTimeRate: completed.length ? Math.round((onTime.length / completed.length) * 100) : 0,
    postponements: completed.reduce((sum, task) => sum + task.postponementCount, 0),
    focusedSeconds: store.focusHistory
      .filter((record) => {
        const endedAt = new Date(record.endedAt);
        return endedAt >= start && endedAt <= end;
      })
      .reduce((sum, record) => sum + record.focusedSeconds, 0),
    lifecycleSeconds: completed.length ? Math.round(lifecycle / completed.length) : 0,
  };
});

async function exportCsv() {
  const content = await backend.exportCsv();
  const suggested = `timepact-history-${new Date().toISOString().slice(0, 10)}.csv`;
  const path = runningInTauri()
    ? await save({ defaultPath: suggested, filters: [{ name: "CSV", extensions: ["csv"] }] })
    : suggested;
  if (path) {
    await backend.writeExportFile(path, content);
    exportMessage.value = "CSV 已导出。";
  }
}

function duration(seconds: number) {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  return hours ? `${hours} 小时 ${minutes} 分` : `${minutes} 分钟`;
}

function lifecycle(task: (typeof store.archive)[number]) {
  if (!task.completedAt) return "—";
  return duration(Math.max(0, differenceInSeconds(task.completedAt, task.createdAt)));
}

function completionOffset(task: (typeof store.archive)[number]) {
  if (!task.completedAt || !task.timer?.originalDueAt) return "";
  const seconds = differenceInSeconds(task.completedAt, task.timer.originalDueAt);
  return seconds <= 0 ? `提前 ${duration(Math.abs(seconds))}` : `超时 ${duration(seconds)}`;
}
</script>

<template>
  <div class="view stats-view">
    <header class="view-header">
      <div>
        <h1 class="view-title">统计</h1>
        <p class="view-description">完成情况、延期历史与实际专注投入。</p>
      </div>
      <button class="button" type="button" @click="exportCsv">
        <Download :size="16" aria-hidden="true" />
        导出 CSV
      </button>
    </header>

    <div class="period-bar panel">
      <CalendarRange :size="17" aria-hidden="true" />
      <button
        v-for="option in [
          ['week', '本周'],
          ['month', '本月'],
          ['quarter', '本季度'],
          ['custom', '自定义'],
        ]"
        :key="option[0]"
        class="period-option"
        :class="{ active: period === option[0] }"
        type="button"
        @click="setPeriod(option[0])"
      >
        {{ option[1] }}
      </button>
    </div>
    <div v-if="period === 'custom'" class="custom-range" aria-label="自定义统计区间">
      <label>开始 <input v-model="customStart" class="field" type="date" /></label>
      <label>结束 <input v-model="customEnd" class="field" type="date" /></label>
    </div>
    <p v-if="exportMessage" class="export-message" role="status">{{ exportMessage }}</p>

    <section class="metric-grid" aria-label="数据概览">
      <article>
        <span>完成周期</span>
        <strong class="numeric">{{ stats.completed }}</strong>
      </article>
      <article>
        <span>按时率</span>
        <strong class="numeric">{{ stats.onTimeRate }}%</strong>
      </article>
      <article>
        <span>延期次数</span>
        <strong class="numeric">{{ stats.postponements }}</strong>
      </article>
      <article>
        <span>累计专注</span>
        <strong class="numeric">{{ duration(stats.focusedSeconds) }}</strong>
      </article>
    </section>

    <section class="history-section">
      <header>
        <div>
          <h2>归档与历史</h2>
          <p>完成、取消和重新打开的执行周期都会保留。</p>
        </div>
        <Archive :size="19" aria-hidden="true" />
      </header>
      <div v-if="!store.archive.length" class="empty-state panel">
        <div>
          <strong>还没有归档记录</strong>
          <span>完成待办后，这里会显示其原计划、延期和实际耗时。</span>
        </div>
      </div>
      <div v-else class="history-list panel">
        <article v-for="task in store.archive" :key="task.cycleId">
          <div>
            <strong>{{ task.title }}</strong>
            <span>
              {{ task.completedAt ? format(new Date(task.completedAt), "yyyy-MM-dd HH:mm") : "已取消" }}
            </span>
          </div>
          <div class="history-meta numeric">
            <span>延期 {{ task.postponementCount }} 次</span>
            <span v-if="completionOffset(task)">{{ completionOffset(task) }}</span>
            <span>总耗时 {{ lifecycle(task) }}</span>
            <span>专注 {{ duration(task.focusedSeconds) }}</span>
          </div>
        </article>
      </div>
    </section>
  </div>
</template>

<style scoped>
.period-bar {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 5px;
}

.period-bar > svg {
  margin: 0 6px;
  color: var(--text-tertiary);
}

.period-option {
  min-height: 32px;
  padding: 0 10px;
  color: var(--text-secondary);
  background: transparent;
  border-radius: var(--radius-sm);
  cursor: pointer;
  font-size: 0.79rem;
}

.period-option:hover,
.period-option.active {
  color: var(--accent);
  background: var(--accent-soft);
}

.metric-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  margin-top: 16px;
  overflow: hidden;
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
}

.custom-range {
  display: flex;
  gap: 10px;
  margin-top: 10px;
}

.custom-range label {
  display: flex;
  align-items: center;
  gap: 7px;
  color: var(--text-secondary);
  font-size: 0.75rem;
}

.custom-range .field {
  min-height: 34px;
  padding: 5px 8px;
}

.export-message {
  margin-top: 8px;
  color: var(--accent);
  font-size: 0.75rem;
}

.metric-grid article {
  display: grid;
  gap: 8px;
  padding: 17px;
}

.metric-grid article + article {
  border-left: 1px solid var(--border);
}

.metric-grid span {
  color: var(--text-secondary);
  font-size: 0.75rem;
}

.metric-grid strong {
  font-size: 1.25rem;
  font-weight: 650;
}

.history-section {
  margin-top: 32px;
}

.history-section > header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 10px;
}

.history-section h2 {
  font-size: 1rem;
}

.history-section p {
  margin-top: 4px;
  color: var(--text-secondary);
  font-size: 0.78rem;
}

.history-section > header svg {
  color: var(--text-tertiary);
}

.history-list {
  overflow: hidden;
}

.history-list article {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 13px 15px;
}

.history-list article + article {
  border-top: 1px solid var(--border);
}

.history-list article > div {
  display: grid;
  gap: 4px;
}

.history-list strong {
  font-size: 0.86rem;
}

.history-list span {
  color: var(--text-secondary);
  font-size: 0.74rem;
}

.history-meta {
  text-align: right;
}

@media (max-width: 720px) {
  .metric-grid {
    grid-template-columns: 1fr 1fr;
  }

  .metric-grid article:nth-child(3) {
    border-top: 1px solid var(--border);
    border-left: 0;
  }

  .metric-grid article:nth-child(4) {
    border-top: 1px solid var(--border);
  }
}
</style>
