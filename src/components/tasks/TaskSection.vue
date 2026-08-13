<script setup lang="ts">
import type { Task } from "../../domain/types";
import TaskRow from "./TaskRow.vue";

defineProps<{
  title: string;
  tasks: Task[];
  emptyText?: string;
}>();
</script>

<template>
  <section v-if="tasks.length" class="task-section" :aria-labelledby="`section-${title}`">
    <header class="task-section-header">
      <h2 :id="`section-${title}`">{{ title }}</h2>
      <span class="numeric">{{ tasks.length }}</span>
    </header>
    <div class="task-stack">
      <TaskRow v-for="task in tasks" :key="task.id" :task="task" />
    </div>
  </section>
</template>

<style scoped>
.task-section + .task-section {
  margin-top: 22px;
}

.task-section-header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  margin-bottom: 8px;
  padding: 0 2px;
}

.task-section-header h2 {
  font-size: 0.87rem;
  font-weight: 650;
}

.task-section-header span {
  color: var(--text-tertiary);
  font-size: 0.75rem;
}

.task-stack {
  display: grid;
  gap: 8px;
}
</style>
