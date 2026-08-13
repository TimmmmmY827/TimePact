<script setup lang="ts">
import { ref, watch } from "vue";
import { ChevronDown, ChevronUp } from "@lucide/vue";
import { clampMinutes, stepMinutes } from "../../domain/time-controls";

const props = withDefaults(
  defineProps<{
    modelValue: number;
    label: string;
    min?: number;
    max?: number;
    step?: number;
  }>(),
  {
    min: 1,
    max: 180,
    step: 5,
  },
);

const emit = defineEmits<{
  "update:modelValue": [value: number];
}>();

const draft = ref(String(props.modelValue));
const editing = ref(false);

watch(
  () => props.modelValue,
  (value) => {
    if (!editing.value) draft.value = String(value);
  },
);

function normalizedDraft(): number {
  const parsed = Number(draft.value);
  return Number.isFinite(parsed)
    ? clampMinutes(parsed, props.min, props.max)
    : props.modelValue;
}

function commit() {
  editing.value = false;
  const next = normalizedDraft();
  draft.value = String(next);
  emit("update:modelValue", next);
}

function adjust(direction: -1 | 1) {
  const next = stepMinutes(
    normalizedDraft(),
    direction * props.step,
    props.min,
    props.max,
  );
  draft.value = String(next);
  emit("update:modelValue", next);
}

function beginEditing(event: FocusEvent) {
  editing.value = true;
  (event.currentTarget as HTMLInputElement).select();
}

function finishWithEnter(event: KeyboardEvent) {
  (event.currentTarget as HTMLInputElement).blur();
}
</script>

<template>
  <div class="minute-stepper">
    <span class="minute-stepper-label">{{ label }}</span>
    <div class="minute-stepper-control">
      <label class="minute-value">
        <span class="sr-only">{{ label }}分钟数，点击后可直接输入</span>
        <input
          v-model="draft"
          class="numeric"
          type="text"
          inputmode="numeric"
          pattern="[0-9]*"
          maxlength="3"
          :aria-label="`${label}分钟数`"
          @focus="beginEditing"
          @blur="commit"
          @keydown.enter.prevent="finishWithEnter"
        />
        <span>分钟</span>
      </label>
      <div class="minute-actions">
        <button type="button" :aria-label="`${label}增加 ${step} 分钟`" @click="adjust(1)">
          <ChevronUp :size="21" :stroke-width="1.7" aria-hidden="true" />
        </button>
        <button type="button" :aria-label="`${label}减少 ${step} 分钟`" @click="adjust(-1)">
          <ChevronDown :size="21" :stroke-width="1.7" aria-hidden="true" />
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.minute-stepper {
  display: grid;
  gap: 7px;
}

.minute-stepper-label {
  color: var(--text-secondary);
  font-size: 0.75rem;
  font-weight: 600;
  text-align: center;
}

.minute-stepper-control {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 48px;
  min-height: 108px;
  overflow: hidden;
  background: var(--bg-surface);
  border: 1px solid var(--border-strong);
  border-bottom-color: var(--text-secondary);
  border-radius: var(--radius-sm);
}

.minute-value {
  display: grid;
  align-content: center;
  justify-items: center;
  gap: 2px;
  min-width: 0;
  cursor: text;
}

.minute-value input {
  width: 100%;
  min-width: 0;
  padding: 0 7px;
  color: var(--text-primary);
  background: transparent;
  border: 0;
  outline: 0;
  font-size: 2.8rem;
  font-weight: 500;
  line-height: 1;
  letter-spacing: -0.03em;
  text-align: center;
}

.minute-value input:focus {
  color: var(--accent);
}

.minute-value > span {
  color: var(--text-secondary);
  font-size: 0.75rem;
}

.minute-actions {
  display: grid;
  grid-template-rows: 1fr 1fr;
  border-left: 1px solid var(--border);
}

.minute-actions button {
  display: grid;
  min-height: 52px;
  place-items: center;
  padding: 0;
  color: var(--text-secondary);
  background: transparent;
  cursor: pointer;
}

.minute-actions button + button {
  border-top: 1px solid var(--border);
}

.minute-actions button:hover {
  color: var(--accent);
  background: var(--accent-soft);
}

.minute-actions button:active {
  background: var(--bg-hover);
}
</style>
