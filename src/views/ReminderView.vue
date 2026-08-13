<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { AlarmClock, Check, Clock3 } from "@lucide/vue";
import { useAppStore } from "../stores/app";
import { shouldHideReminder } from "../domain/reminder";
import { runningInTauri } from "../services/backend";
import type { Task } from "../domain/types";

const store = useAppStore();
const customMinutes = ref(30);
const waiting = computed(() => store.tasks.filter((task) => task.status === "waiting"));

watch(
  [() => store.initialized, () => waiting.value.length],
  ([initialized, waitingCount]) => {
    if (runningInTauri() && shouldHideReminder(initialized, waitingCount)) {
      void getCurrentWindow().hide();
    }
  },
  { immediate: true },
);

async function postpone(task: Task, minutes = customMinutes.value) {
  await store.postponeTask(task, minutes);
}
</script>

<template>
  <main class="reminder-view">
    <header>
      <span class="reminder-icon"><AlarmClock :size="22" aria-hidden="true" /></span>
      <div>
        <h1>{{ waiting.length > 1 ? `${waiting.length} 个待办需要确认` : "这个待办完成了吗？" }}</h1>
        <p>关闭窗口不会改变任务状态。</p>
      </div>
    </header>

    <div v-if="!waiting.length" class="empty-state">没有等待确认的待办。</div>
    <article v-for="task in waiting" :key="task.id" class="reminder-task panel">
      <strong>{{ task.title }}</strong>
      <span>Deadline 已到期</span>
      <div class="reminder-actions">
        <button class="button primary" type="button" @click="store.completeTask(task)">
          <Check :size="16" aria-hidden="true" />
          已完成
        </button>
        <button class="button" type="button" @click="postpone(task, 10)">10 分钟</button>
        <button class="button" type="button" @click="postpone(task, 30)">30 分钟</button>
        <label class="custom-delay">
          <Clock3 :size="15" aria-hidden="true" />
          <input v-model.number="customMinutes" type="number" min="1" max="5999" />
          <span>分钟</span>
          <button type="button" @click="postpone(task)">稍后提醒</button>
        </label>
      </div>
    </article>
  </main>
</template>

<style scoped>
.reminder-view {
  min-height: 100%;
  padding: 22px;
  background: var(--bg-canvas);
}

.reminder-view > header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 17px;
}

.reminder-view h1 {
  font-size: 1.1rem;
  font-weight: 680;
}

.reminder-view p {
  margin-top: 4px;
  color: var(--text-secondary);
  font-size: 0.76rem;
}

.reminder-icon {
  display: grid;
  width: 42px;
  height: 42px;
  place-items: center;
  color: var(--danger);
  background: var(--danger-soft);
  border-radius: 50%;
}

.reminder-task {
  display: grid;
  gap: 5px;
  padding: 15px;
}

.reminder-task + .reminder-task {
  margin-top: 10px;
}

.reminder-task > strong {
  font-size: 0.94rem;
}

.reminder-task > span {
  color: var(--danger);
  font-size: 0.75rem;
}

.reminder-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 7px;
  margin-top: 9px;
}

.custom-delay {
  display: flex;
  min-height: 36px;
  align-items: center;
  gap: 5px;
  padding-left: 9px;
  color: var(--text-secondary);
  background: var(--bg-subtle);
  border-radius: var(--radius-sm);
  font-size: 0.75rem;
}

.custom-delay input {
  width: 44px;
  padding: 3px;
  color: var(--text-primary);
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 4px;
}

.custom-delay button {
  align-self: stretch;
  padding: 0 10px;
  color: #fff;
  background: var(--accent);
  border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
  cursor: pointer;
}
</style>
