<script setup lang="ts">
import { computed, ref } from "vue";
import { ChevronDown, ChevronUp, Plus, X } from "@lucide/vue";
import { useAppStore } from "../../stores/app";
import { isFutureDeadline } from "../../domain/deadline";
import type { Priority } from "../../domain/types";
import DeadlinePicker from "./DeadlinePicker.vue";

const store = useAppStore();
const expanded = ref(false);
const saving = ref(false);
const title = ref("");
const notes = ref("");
const priority = ref<Priority>("medium");
const tagsText = ref("");
const timerKind = ref<"deadline" | null>(null);
const deadlineAt = ref("");

const canSubmit = computed(() => {
  if (!title.value.trim()) return false;
  if (timerKind.value === "deadline") return isFutureDeadline(deadlineAt.value);
  return true;
});

async function submit() {
  if (!canSubmit.value || saving.value) return;
  saving.value = true;
  try {
    await store.addTask({
      title: title.value,
      notes: notes.value,
      priority: priority.value,
      tags: tagsText.value
        .split(/[,，]/)
        .map((tag) => tag.trim())
        .filter(Boolean),
      timerKind: timerKind.value,
      deadlineAt: timerKind.value === "deadline" ? deadlineAt.value : null,
    });
    title.value = "";
    notes.value = "";
    priority.value = "medium";
    tagsText.value = "";
    timerKind.value = null;
    deadlineAt.value = "";
    expanded.value = false;
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <form class="composer panel" @submit.prevent="submit">
    <div class="composer-primary">
      <Plus :size="18" aria-hidden="true" />
      <label class="sr-only" for="quick-task-title">添加待办</label>
      <input
        id="quick-task-title"
        v-model="title"
        class="composer-title"
        placeholder="添加一个待办…"
        maxlength="160"
        autocomplete="off"
      />
      <button
        class="icon-button"
        type="button"
        :aria-label="expanded ? '收起更多选项' : '展开更多选项'"
        :aria-expanded="expanded"
        @click="expanded = !expanded"
      >
        <ChevronUp v-if="expanded" :size="17" aria-hidden="true" />
        <ChevronDown v-else :size="17" aria-hidden="true" />
      </button>
      <button class="button primary composer-submit" type="submit" :disabled="!canSubmit || saving">
        {{ saving ? "保存中" : "添加" }}
      </button>
    </div>

    <div v-if="expanded" class="composer-details">
      <label class="field-label">
        备注
        <textarea v-model="notes" class="field" rows="2" maxlength="2000" placeholder="可选" />
      </label>
      <div class="composer-grid">
        <label class="field-label">
          优先级
          <select v-model="priority" class="field">
            <option value="low">低</option>
            <option value="medium">中</option>
            <option value="high">高</option>
          </select>
        </label>
        <label class="field-label">
          标签
          <input v-model="tagsText" class="field" placeholder="用逗号分隔" />
        </label>
      </div>
      <fieldset class="timer-options">
        <legend>时间安排</legend>
        <div class="timer-kind">
          <label><input v-model="timerKind" type="radio" :value="null" /> 未安排</label>
          <label><input v-model="timerKind" type="radio" value="deadline" /> Deadline</label>
        </div>
        <DeadlinePicker
          v-if="timerKind === 'deadline'"
          v-model="deadlineAt"
          :workday-end-times="store.settings.workdayEndTimes"
        />
      </fieldset>
      <button class="button ghost composer-cancel" type="button" @click="expanded = false">
        <X :size="15" aria-hidden="true" />
        收起
      </button>
    </div>
  </form>
</template>

<style scoped>
.composer {
  margin-bottom: 24px;
  overflow: clip;
}

.composer-primary {
  display: grid;
  grid-template-columns: auto minmax(0, 1fr) auto auto;
  align-items: center;
  gap: 7px;
  min-height: 46px;
  padding: 4px 5px 4px 13px;
  color: var(--text-tertiary);
}

.composer-title {
  min-width: 0;
  padding: 8px 3px;
  color: var(--text-primary);
  background: transparent;
  border: 0;
  outline: 0;
}

.composer-title::placeholder {
  color: var(--text-tertiary);
}

.composer-submit:disabled {
  cursor: not-allowed;
  opacity: 0.45;
}

.composer-details {
  position: relative;
  display: grid;
  gap: 14px;
  padding: 14px;
  border-top: 1px solid var(--border);
}

.composer-grid {
  display: grid;
  grid-template-columns: 0.7fr 1.3fr;
  gap: 12px;
}

.timer-options {
  display: grid;
  gap: 10px;
  margin: 0;
  padding: 12px;
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
}

.timer-options legend {
  padding: 0 5px;
  color: var(--text-secondary);
  font-size: 0.78rem;
  font-weight: 600;
}

.timer-kind {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
  color: var(--text-secondary);
  font-size: 0.83rem;
}

.timer-kind label {
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.composer-cancel {
  justify-self: start;
}
</style>
