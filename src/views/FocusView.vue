<script setup lang="ts">
import { computed, onBeforeUnmount, ref, watch } from "vue";
import { Link2, Pause, Play, RotateCcw, Square, Timer } from "@lucide/vue";
import MinuteStepper from "../components/focus/MinuteStepper.vue";
import { clampMinutes } from "../domain/time-controls";
import { useAppStore } from "../stores/app";

const store = useAppStore();
const selectedTask = ref<string>("");
const focusMinutes = ref(store.settings.focusMinutes);
const restMinutes = ref(store.settings.restMinutes);
let presetSaveHandle: number | null = null;

watch(
  () => [store.settings.focusMinutes, store.settings.restMinutes] as const,
  ([nextFocus, nextRest]) => {
    if (store.focus) return;
    focusMinutes.value = nextFocus;
    restMinutes.value = nextRest;
  },
  { immediate: true },
);

watch(
  [focusMinutes, restMinutes],
  ([nextFocus, nextRest]) => {
    if (store.focus) return;
    const normalizedFocus = clampMinutes(nextFocus);
    const normalizedRest = clampMinutes(nextRest);
    store.settings.focusMinutes = normalizedFocus;
    store.settings.restMinutes = normalizedRest;
    if (presetSaveHandle != null) window.clearTimeout(presetSaveHandle);
    presetSaveHandle = window.setTimeout(() => {
      presetSaveHandle = null;
      void persistPreset();
    }, 240);
  },
  { flush: "sync" },
);

const displayTime = computed(() => {
  const total = store.focus?.remainingSeconds ?? focusMinutes.value * 60;
  const minutes = Math.floor(total / 60).toString().padStart(2, "0");
  const seconds = (total % 60).toString().padStart(2, "0");
  return `${minutes}:${seconds}`;
});

const phaseTitle = computed(() => {
  if (!store.focus) return "准备专注";
  if (store.focus.phase === "focus") return `第 ${store.focus.round} 轮专注`;
  if (store.focus.phase === "rest") return "休息一下";
  if (store.focus.phase === "waiting-rest") return "本轮专注已完成";
  return "休息结束";
});

const phaseProgress = computed(() => {
  if (!store.focus) return 0;
  const total =
    store.focus.phase === "rest" ? store.focus.plannedRestSeconds : store.focus.plannedFocusSeconds;
  return Math.min(1, Math.max(0, 1 - store.focus.remainingSeconds / Math.max(1, total)));
});

async function persistPreset() {
  if (presetSaveHandle != null) {
    window.clearTimeout(presetSaveHandle);
    presetSaveHandle = null;
  }
  const nextFocus = clampMinutes(focusMinutes.value);
  const nextRest = clampMinutes(restMinutes.value);
  store.settings.focusMinutes = nextFocus;
  store.settings.restMinutes = nextRest;
  await store.updateSettings({ ...store.settings });
}

async function start() {
  await persistPreset();
  await store.startFocus(selectedTask.value || null);
}

onBeforeUnmount(() => {
  if (presetSaveHandle != null) void persistPreset();
});
</script>

<template>
  <div class="view focus-view">
    <header class="view-header">
      <div>
        <h1 class="view-title">专注</h1>
        <p class="view-description">每轮结束后由你决定休息、继续或停止。</p>
      </div>
    </header>

    <section class="focus-stage panel" aria-live="polite">
      <div class="focus-stage-heading">
        <Timer :size="18" aria-hidden="true" />
        <span>{{ phaseTitle }}</span>
      </div>
      <strong v-if="store.focus" class="focus-time numeric">{{ displayTime }}</strong>
      <div v-if="store.focus" class="focus-progress" aria-hidden="true">
        <span :style="{ transform: `scaleX(${phaseProgress})` }" />
      </div>

      <template v-if="!store.focus">
        <div class="focus-config">
          <MinuteStepper v-model="focusMinutes" label="专注" :step="5" />
          <MinuteStepper v-model="restMinutes" label="休息" :step="5" />
        </div>
        <label class="field-label focus-task">
          <span><Link2 :size="14" aria-hidden="true" /> 关联待办（可选）</span>
          <select v-model="selectedTask" class="field">
            <option value="">自由专注</option>
            <option v-for="task in store.orderedTasks" :key="task.id" :value="task.id">
              {{ task.title }}
            </option>
          </select>
        </label>
        <button class="button primary focus-primary" type="button" @click="start">
          <Play :size="17" aria-hidden="true" />
          开始专注
        </button>
      </template>

      <template v-else-if="store.focus.phase === 'waiting-rest'">
        <div class="transition-actions">
          <button class="button primary" type="button" @click="store.beginRest">开始休息</button>
          <button class="button" type="button" @click="store.endFocus">结束本次专注</button>
        </div>
      </template>

      <template v-else-if="store.focus.phase === 'waiting-focus'">
        <div class="transition-actions">
          <button class="button primary" type="button" @click="store.continueFocus">
            继续下一轮
          </button>
          <button class="button" type="button" @click="store.endFocus">结束本次专注</button>
        </div>
      </template>

      <template v-else>
        <div class="focus-controls">
          <button class="button primary" type="button" @click="store.toggleFocus">
            <Pause v-if="store.focus.running" :size="17" aria-hidden="true" />
            <Play v-else :size="17" aria-hidden="true" />
            {{ store.focus.running ? "暂停" : "继续" }}
          </button>
          <button
            v-if="store.focus.phase === 'rest'"
            class="button"
            type="button"
            @click="store.skipRest"
          >
            <RotateCcw :size="16" aria-hidden="true" />
            跳过休息
          </button>
          <button class="button danger" type="button" @click="store.endFocus">
            <Square :size="15" aria-hidden="true" />
            提前结束
          </button>
        </div>
      </template>
    </section>
  </div>
</template>

<style scoped>
.focus-view {
  width: min(100%, 650px);
}

.focus-stage {
  display: grid;
  justify-items: center;
  padding: 34px;
}

.focus-stage-heading {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  color: var(--text-secondary);
  font-size: 0.88rem;
  font-weight: 600;
}

.focus-time {
  margin: 18px 0 14px;
  font-size: clamp(3.3rem, 9vw, 5rem);
  font-weight: 620;
  line-height: 1;
  letter-spacing: -0.04em;
}

.focus-progress {
  width: min(100%, 360px);
  height: 5px;
  margin-bottom: 28px;
  overflow: hidden;
  background: var(--bg-subtle);
  border-radius: 4px;
}

.focus-progress span {
  display: block;
  width: 100%;
  height: 100%;
  background: var(--accent);
  border-radius: inherit;
  transform-origin: left center;
  transition: transform 350ms var(--ease-out);
}

.focus-config {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 14px;
  width: min(100%, 390px);
  margin-top: 20px;
}

.focus-task {
  width: min(100%, 360px);
  margin-top: 14px;
}

.focus-task > span {
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.focus-primary {
  min-width: 150px;
  margin-top: 20px;
}

.focus-controls,
.transition-actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 9px;
}
</style>
