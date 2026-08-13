<script setup lang="ts">
import { computed } from "vue";
import FocusLauncher from "../components/focus/FocusLauncher.vue";
import TaskComposer from "../components/tasks/TaskComposer.vue";
import TaskSection from "../components/tasks/TaskSection.vue";
import { useAppStore } from "../stores/app";

const store = useAppStore();
const hasAny = computed(() => store.tasks.length > 0);
</script>

<template>
  <div class="view home-view">
    <header class="view-header">
      <div>
        <h1 class="view-title">所有进行中的待办</h1>
        <p class="view-description">{{ store.tasks.length }} 个待办，按紧迫程度排列</p>
      </div>
    </header>

    <TaskComposer />
    <div class="home-focus-entry">
      <FocusLauncher />
    </div>

    <div v-if="store.busy && !store.initialized" class="empty-state panel" aria-live="polite">
      正在读取本地数据…
    </div>
    <div v-else-if="!hasAny" class="empty-state panel">
      <div>
        <strong>先记下第一件事</strong>
        <span>可以暂不安排时间，稍后再设置 Deadline。</span>
      </div>
    </div>
    <template v-else>
      <TaskSection title="需要处理" :tasks="store.attentionTasks" />
      <TaskSection title="进行中" :tasks="store.activeTasks" />
      <TaskSection title="未安排" :tasks="store.unscheduledTasks" />
    </template>
  </div>
</template>

<style scoped>
.home-view {
  width: min(100%, 720px);
}

.home-focus-entry {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  margin: -14px 0 18px;
}
</style>
