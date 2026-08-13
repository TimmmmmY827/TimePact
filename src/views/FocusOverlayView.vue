<script setup lang="ts">
import { onBeforeUnmount, onMounted } from "vue";

const rootClass = "screen-focus-overlay";

onMounted(() => document.documentElement.classList.add(rootClass));
onBeforeUnmount(() => document.documentElement.classList.remove(rootClass));
</script>

<template>
  <div class="screen-edge-aura" aria-hidden="true" />
</template>

<style scoped>
:global(html.screen-focus-overlay),
:global(html.screen-focus-overlay body),
:global(html.screen-focus-overlay #app) {
  background: transparent !important;
}

.screen-edge-aura {
  position: fixed;
  inset: 0;
  pointer-events: none;
  background:
    linear-gradient(to bottom, rgb(10 112 77 / 86%), rgb(25 188 123 / 24%) 44%, transparent)
      top / 100% 12px no-repeat,
    linear-gradient(to top, rgb(10 112 77 / 86%), rgb(25 188 123 / 24%) 44%, transparent)
      bottom / 100% 12px no-repeat,
    linear-gradient(to right, rgb(10 112 77 / 86%), rgb(25 188 123 / 24%) 44%, transparent)
      left / 12px 100% no-repeat,
    linear-gradient(to left, rgb(10 112 77 / 86%), rgb(25 188 123 / 24%) 44%, transparent)
      right / 12px 100% no-repeat;
  box-shadow: inset 0 0 0 1px rgb(5 77 54 / 82%);
  animation: screen-edge-breathe 3.2s ease-in-out infinite alternate;
}

@keyframes screen-edge-breathe {
  from {
    opacity: 0.76;
  }

  to {
    opacity: 1;
  }
}

@media (prefers-reduced-motion: reduce) {
  .screen-edge-aura {
    animation: none;
  }
}
</style>
