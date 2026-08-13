<script setup lang="ts">
import { ref, watch } from "vue";
import { ChevronDown, ChevronUp } from "@lucide/vue";
import { stepClockPart } from "../../domain/work-hours";

const props = defineProps<{
  hour: number;
  minute: number;
}>();

const emit = defineEmits<{
  "update:hour": [value: number];
  "update:minute": [value: number];
}>();

const hourDraft = ref(props.hour.toString().padStart(2, "0"));
const minuteDraft = ref(props.minute.toString().padStart(2, "0"));
const editingHour = ref(false);
const editingMinute = ref(false);

watch(
  () => props.hour,
  (value) => {
    if (!editingHour.value) hourDraft.value = value.toString().padStart(2, "0");
  },
);

watch(
  () => props.minute,
  (value) => {
    if (!editingMinute.value) minuteDraft.value = value.toString().padStart(2, "0");
  },
);

function adjust(part: "hour" | "minute", delta: -1 | 1) {
  if (part === "hour") {
    emit("update:hour", stepClockPart(props.hour, delta, 24));
  } else {
    emit("update:minute", stepClockPart(props.minute, delta, 60));
  }
}

function onWheel(part: "hour" | "minute", event: WheelEvent) {
  adjust(part, event.deltaY < 0 ? 1 : -1);
}

function beginEditing(part: "hour" | "minute", event: FocusEvent) {
  if (part === "hour") editingHour.value = true;
  else editingMinute.value = true;
  (event.currentTarget as HTMLInputElement).select();
}

function commit(part: "hour" | "minute") {
  const isHour = part === "hour";
  const draft = isHour ? hourDraft : minuteDraft;
  const fallback = isHour ? props.hour : props.minute;
  const maximum = isHour ? 23 : 59;
  const parsed = Number(draft.value);
  const next = Number.isFinite(parsed)
    ? Math.min(maximum, Math.max(0, Math.round(parsed)))
    : fallback;

  if (isHour) {
    editingHour.value = false;
    emit("update:hour", next);
  } else {
    editingMinute.value = false;
    emit("update:minute", next);
  }
  draft.value = next.toString().padStart(2, "0");
}

function finishWithEnter(event: KeyboardEvent) {
  (event.currentTarget as HTMLInputElement).blur();
}
</script>

<template>
  <div class="time-wheel" aria-label="Deadline 时间，悬停数字后可用滚轮调节">
    <div class="time-wheel-display">
      <div
        class="time-wheel-segment"
        title="悬停后使用滚轮调节小时"
        @wheel.prevent.stop="onWheel('hour', $event)"
      >
        <span class="sr-only">Deadline 小时</span>
        <input
          v-model="hourDraft"
          class="numeric"
          type="text"
          inputmode="numeric"
          pattern="[0-9]*"
          maxlength="2"
          aria-label="Deadline 小时"
          @focus="beginEditing('hour', $event)"
          @blur="commit('hour')"
          @keydown.enter.prevent="finishWithEnter"
          @keydown.up.prevent="adjust('hour', 1)"
          @keydown.down.prevent="adjust('hour', -1)"
        />
        <span class="time-wheel-stepper">
          <button type="button" aria-label="增加一小时" @click="adjust('hour', 1)">
            <ChevronUp :size="16" aria-hidden="true" />
          </button>
          <button type="button" aria-label="减少一小时" @click="adjust('hour', -1)">
            <ChevronDown :size="16" aria-hidden="true" />
          </button>
        </span>
      </div>
      <span class="time-wheel-colon" aria-hidden="true">:</span>
      <div
        class="time-wheel-segment"
        title="悬停后使用滚轮调节分钟"
        @wheel.prevent.stop="onWheel('minute', $event)"
      >
        <span class="sr-only">Deadline 分钟</span>
        <input
          v-model="minuteDraft"
          class="numeric"
          type="text"
          inputmode="numeric"
          pattern="[0-9]*"
          maxlength="2"
          aria-label="Deadline 分钟"
          @focus="beginEditing('minute', $event)"
          @blur="commit('minute')"
          @keydown.enter.prevent="finishWithEnter"
          @keydown.up.prevent="adjust('minute', 1)"
          @keydown.down.prevent="adjust('minute', -1)"
        />
        <span class="time-wheel-stepper">
          <button type="button" aria-label="增加一分钟" @click="adjust('minute', 1)">
            <ChevronUp :size="16" aria-hidden="true" />
          </button>
          <button type="button" aria-label="减少一分钟" @click="adjust('minute', -1)">
            <ChevronDown :size="16" aria-hidden="true" />
          </button>
        </span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.time-wheel {
  display: grid;
  width: var(--time-wheel-width, 190px);
}

.time-wheel-display {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto minmax(0, 1fr);
  align-items: center;
  height: var(--deadline-input-height, 56px);
  overflow: hidden;
  background: var(--bg-surface);
  border: 1px solid var(--border-strong);
  border-bottom: 2px solid var(--accent);
  border-radius: var(--radius-sm);
}

.time-wheel-segment {
  position: relative;
  display: grid;
  height: 100%;
  min-height: 0;
  align-items: center;
  cursor: ns-resize;
}

.time-wheel-segment:focus-within {
  background: var(--accent-soft);
}

.time-wheel-segment input {
  position: relative;
  z-index: 1;
  width: 100%;
  height: 100%;
  min-width: 0;
  padding: 0 4px;
  color: var(--text-primary);
  font-size: 1.85rem;
  font-weight: 680;
  line-height: 1;
  letter-spacing: -0.03em;
  text-align: center;
  background: transparent;
  border: 0;
  outline: 0;
  cursor: ns-resize;
}

.time-wheel-stepper {
  position: absolute;
  z-index: 2;
  inset: 0;
  pointer-events: none;
}

.time-wheel-stepper button {
  position: absolute;
  left: 0;
  display: grid;
  width: 100%;
  height: 19px;
  place-items: center;
  padding: 0;
  color: var(--text-secondary);
  background: transparent;
  cursor: pointer;
  opacity: 0.28;
  pointer-events: auto;
  transition: color 120ms ease, background-color 120ms ease, opacity 120ms ease;
}

.time-wheel-stepper button:first-child {
  top: 0;
}

.time-wheel-stepper button:last-child {
  bottom: 0;
}

.time-wheel-stepper button:hover {
  color: var(--accent);
  background: var(--accent-soft);
  opacity: 0.86;
}

.time-wheel-segment:focus-within .time-wheel-stepper button {
  opacity: 0.42;
}

.time-wheel-colon {
  color: var(--text-tertiary);
  font-size: 1.45rem;
  font-weight: 600;
}
</style>
