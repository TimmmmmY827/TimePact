<script setup lang="ts">
import { computed } from "vue";
import { Pause, Play, Timer } from "@lucide/vue";
import { useRouter } from "vue-router";
import { useAppStore } from "../../stores/app";

const store = useAppStore();
const router = useRouter();

const displayTime = computed(() => {
  const total = store.focus?.remainingSeconds ?? store.settings.focusMinutes * 60;
  const minutes = Math.floor(total / 60).toString().padStart(2, "0");
  const seconds = (total % 60).toString().padStart(2, "0");
  return `${minutes}:${seconds}`;
});

const phaseLabel = computed(() => {
  if (store.focus?.phase === "rest") return "休息中";
  if (store.focus?.phase === "waiting-rest") return "待开始休息";
  if (store.focus?.phase === "waiting-focus") return "待继续";
  return "专注中";
});

async function launch() {
  if (!store.focus) await store.startFocus();
  await router.push("/focus");
}
</script>

<template>
  <div v-if="store.focus" class="focus-mini" aria-label="当前专注计时">
    <button class="focus-mini-main" type="button" @click="router.push('/focus')">
      <Timer :size="17" aria-hidden="true" />
      <span>{{ phaseLabel }}</span>
      <strong class="numeric">{{ displayTime }}</strong>
    </button>
    <button
      class="icon-button"
      type="button"
      :aria-label="store.focus.running ? '暂停专注' : '继续专注'"
      @click="store.toggleFocus"
    >
      <Pause v-if="store.focus.running" :size="17" aria-hidden="true" />
      <Play v-else :size="17" aria-hidden="true" />
    </button>
  </div>
  <button v-else class="button focus-launcher" type="button" @click="launch">
    <Timer :size="17" aria-hidden="true" />
    <span>开始专注</span>
    <span class="focus-preset numeric">{{ store.settings.focusMinutes }}/{{ store.settings.restMinutes }}</span>
  </button>
</template>

<style scoped>
.focus-launcher {
  white-space: nowrap;
}

.focus-preset {
  color: var(--accent);
  font-size: 0.78rem;
}

.focus-mini {
  display: flex;
  align-items: center;
  padding: 2px;
  background: var(--accent-soft);
  border-radius: var(--radius-md);
}

.focus-mini-main {
  display: flex;
  min-height: 32px;
  align-items: center;
  gap: 7px;
  padding: 0 8px;
  color: var(--accent);
  background: transparent;
  border-radius: var(--radius-sm);
  cursor: pointer;
  font-size: 0.81rem;
}

.focus-mini-main strong {
  color: var(--text-primary);
}
</style>
