<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from "vue";
import { Minus, Square, X } from "@lucide/vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { runningInTauri } from "../../services/backend";

const maximized = ref(false);
const previewMode = import.meta.env.DEV;
let stopResizeListener: (() => void) | undefined;

function runningInMainWindow(): boolean {
  return runningInTauri() && getCurrentWindow().label === "main";
}

async function refreshMaximized() {
  if (!runningInMainWindow()) return;
  maximized.value = await getCurrentWindow().isMaximized();
}

async function minimize() {
  if (runningInMainWindow()) await getCurrentWindow().minimize();
}

async function toggleMaximize() {
  if (!runningInMainWindow()) return;
  await getCurrentWindow().toggleMaximize();
  await refreshMaximized();
}

async function close() {
  if (runningInMainWindow()) await getCurrentWindow().close();
}

onMounted(async () => {
  if (!runningInMainWindow()) return;
  await refreshMaximized();
  stopResizeListener = await getCurrentWindow().onResized(refreshMaximized);
});

onBeforeUnmount(() => stopResizeListener?.());
</script>

<template>
  <header class="window-titlebar">
    <div
      class="window-drag-area"
      data-tauri-drag-region
      @dblclick="toggleMaximize"
    >
      <span class="window-app-icon" aria-hidden="true">
        <span class="window-app-check" />
      </span>
      <strong data-tauri-drag-region>TimePact</strong>
      <span v-if="previewMode" class="preview-badge" data-tauri-drag-region>Preview</span>
    </div>
    <div class="window-controls" aria-label="窗口控制">
      <button type="button" aria-label="最小化" title="最小化" @click="minimize">
        <Minus :size="16" :stroke-width="1.7" aria-hidden="true" />
      </button>
      <button
        type="button"
        :aria-label="maximized ? '还原' : '最大化'"
        :title="maximized ? '还原' : '最大化'"
        @click="toggleMaximize"
      >
        <span v-if="maximized" class="restore-icon" aria-hidden="true" />
        <Square v-else :size="13" :stroke-width="1.6" aria-hidden="true" />
      </button>
      <button class="window-close" type="button" aria-label="关闭" title="关闭" @click="close">
        <X :size="17" :stroke-width="1.7" aria-hidden="true" />
      </button>
    </div>
  </header>
</template>

<style scoped>
.window-titlebar {
  display: flex;
  height: 40px;
  align-items: stretch;
  justify-content: space-between;
  color: var(--text-secondary);
  background: var(--bg-surface);
  border-bottom: 1px solid var(--border);
  user-select: none;
}

.window-drag-area {
  display: flex;
  min-width: 0;
  flex: 1;
  align-items: center;
  gap: 8px;
  padding-left: 14px;
  cursor: default;
}

.window-drag-area strong {
  overflow: hidden;
  color: var(--text-primary);
  font-size: 0.78rem;
  font-weight: 620;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.preview-badge {
  padding: 3px 6px;
  color: var(--accent);
  background: var(--accent-soft);
  border-radius: var(--radius-sm);
  font-size: 0.75rem;
  font-weight: 650;
  line-height: 1;
}

.window-app-icon {
  position: relative;
  width: 18px;
  height: 18px;
  border: 2px solid var(--accent);
  border-radius: 50%;
}

.window-app-check {
  position: absolute;
  top: 4px;
  left: 4px;
  width: 7px;
  height: 4px;
  border-bottom: 1.8px solid var(--accent);
  border-left: 1.8px solid var(--accent);
  transform: rotate(-45deg);
}

.window-controls {
  display: flex;
}

.window-controls button {
  display: grid;
  width: 46px;
  height: 39px;
  place-items: center;
  padding: 0;
  color: var(--text-secondary);
  background: transparent;
  border-radius: 0;
  cursor: default;
}

.window-controls button:hover {
  color: var(--text-primary);
  background: var(--bg-hover);
}

.window-controls .window-close:hover {
  color: #fff;
  background: var(--danger);
}

.restore-icon {
  position: relative;
  width: 11px;
  height: 11px;
  border: 1.5px solid currentColor;
}

.restore-icon::before {
  position: absolute;
  top: -4px;
  left: 2px;
  width: 9px;
  height: 9px;
  content: "";
  border-top: 1.5px solid currentColor;
  border-right: 1.5px solid currentColor;
}
</style>
