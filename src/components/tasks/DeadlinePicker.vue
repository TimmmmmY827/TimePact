<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, useId, watch } from "vue";
import {
  CalendarClock,
  CalendarDays,
  Check,
  ChevronLeft,
  ChevronRight,
} from "@lucide/vue";
import TimeWheelPicker from "../time/TimeWheelPicker.vue";
import {
  DEADLINE_PRESETS,
  calendarMonthDays,
  combineLocalDeadline,
  isFutureDeadline,
  parseLocalDateTimeValue,
  resolveDeadlinePreset,
  toLocalDateValue,
  toLocalDateTimeValue,
  type DeadlinePreset,
} from "../../domain/deadline";
import { DEFAULT_WORKDAY_END_TIMES } from "../../domain/work-hours";

const props = withDefaults(
  defineProps<{
    modelValue: string;
    workdayEndTimes?: readonly string[];
  }>(),
  {
    workdayEndTimes: () => [...DEFAULT_WORKDAY_END_TIMES],
  },
);

const emit = defineEmits<{
  "update:modelValue": [value: string];
}>();

const pickerId = `deadline-${useId()}`;
const dateControl = ref<HTMLElement | null>(null);
const calendarOpen = ref(false);
const mountedAt = new Date();
const initialDate = parseLocalDateTimeValue(props.modelValue)
  ? new Date(props.modelValue)
  : mountedAt;
const viewYear = ref(initialDate.getFullYear());
const viewMonth = ref(initialDate.getMonth());
const weekdays = ["一", "二", "三", "四", "五", "六", "日"];

const presetOptions = computed(() =>
  DEADLINE_PRESETS.map((preset) => {
    const date = resolveDeadlinePreset(
      preset.value,
      mountedAt,
      props.workdayEndTimes,
    );
    return {
      key: preset.value,
      label: preset.label,
      date,
      inputValue: toLocalDateTimeValue(date),
      clockLabel: preset.value.endsWith("work-end")
        ? toLocalDateTimeValue(date).slice(11)
        : "",
      disabled: date.getTime() <= mountedAt.getTime(),
    };
  }),
);

const selectedPreset = computed<DeadlinePreset | null>(() => {
  return (
    presetOptions.value.find((preset) => preset.inputValue === props.modelValue)?.key ?? null
  );
});

const invalid = computed(
  () => Boolean(props.modelValue) && !isFutureDeadline(props.modelValue),
);

const selectedParts = computed(() => parseLocalDateTimeValue(props.modelValue));
const selectedDateValue = computed(() => selectedParts.value?.dateValue ?? "");
const selectedHour = computed(() => selectedParts.value?.hour ?? 18);
const selectedMinute = computed(() => selectedParts.value?.minute ?? 0);
const calendarDays = computed(() =>
  calendarMonthDays(viewYear.value, viewMonth.value, mountedAt),
);
const monthLabel = computed(() => `${viewYear.value}年${viewMonth.value + 1}月`);
const canGoPrevious = computed(() => {
  const currentMonth = new Date(mountedAt.getFullYear(), mountedAt.getMonth(), 1).getTime();
  return new Date(viewYear.value, viewMonth.value, 1).getTime() > currentMonth;
});
const dateLabel = computed(() => {
  if (!selectedDateValue.value) return "选择日期";
  const date = new Date(`${selectedDateValue.value}T12:00`);
  return new Intl.DateTimeFormat("zh-CN", {
    year: date.getFullYear() === mountedAt.getFullYear() ? undefined : "numeric",
    month: "long",
    day: "numeric",
    weekday: "short",
  }).format(date);
});

watch(
  () => props.modelValue,
  (value) => {
    const parsed = parseLocalDateTimeValue(value);
    if (!parsed) return;
    const date = new Date(`${parsed.dateValue}T12:00`);
    viewYear.value = date.getFullYear();
    viewMonth.value = date.getMonth();
  },
);

function choosePreset(value: string) {
  calendarOpen.value = false;
  emit("update:modelValue", value);
}

function toggleCalendar() {
  calendarOpen.value = !calendarOpen.value;
}

function shiftMonth(offset: number) {
  const date = new Date(viewYear.value, viewMonth.value + offset, 1);
  viewYear.value = date.getFullYear();
  viewMonth.value = date.getMonth();
}

function selectDate(value: string) {
  emit(
    "update:modelValue",
    combineLocalDeadline(value, selectedHour.value, selectedMinute.value),
  );
  calendarOpen.value = false;
}

function selectTime(part: "hour" | "minute", value: number) {
  let dateValue = selectedDateValue.value;
  if (!dateValue) {
    const todayWorkEnd = resolveDeadlinePreset(
      "today-work-end",
      mountedAt,
      props.workdayEndTimes,
    );
    const defaultDate = todayWorkEnd.getTime() > mountedAt.getTime()
      ? todayWorkEnd
      : resolveDeadlinePreset(
          "tomorrow-work-end",
          mountedAt,
          props.workdayEndTimes,
        );
    dateValue = toLocalDateValue(defaultDate);
  }
  const hour = part === "hour" ? value : selectedHour.value;
  const minute = part === "minute" ? value : selectedMinute.value;
  emit("update:modelValue", combineLocalDeadline(dateValue, hour, minute));
}

function isPastDay(value: string): boolean {
  return value < toLocalDateValue(mountedAt);
}

function onDocumentPointerDown(event: PointerEvent) {
  if (
    calendarOpen.value &&
    event.target instanceof Node &&
    !dateControl.value?.contains(event.target)
  ) {
    calendarOpen.value = false;
  }
}

onMounted(() => document.addEventListener("pointerdown", onDocumentPointerDown));
onBeforeUnmount(() => document.removeEventListener("pointerdown", onDocumentPointerDown));
</script>

<template>
  <div class="deadline-picker">
    <div class="deadline-heading">
      <span>
        <CalendarClock :size="15" aria-hidden="true" />
        Deadline
      </span>
      <small>设置后不可修改</small>
    </div>

    <div class="deadline-presets" role="group" aria-label="Deadline 快捷设置">
      <button
        v-for="preset in presetOptions"
        :key="preset.key"
        class="deadline-preset"
        :class="{ selected: selectedPreset === preset.key }"
        type="button"
        :disabled="preset.disabled"
        :aria-pressed="selectedPreset === preset.key"
        @click="choosePreset(preset.inputValue)"
      >
        <Check
          v-if="selectedPreset === preset.key"
          :size="14"
          aria-hidden="true"
        />
        <span>{{ preset.label }}</span>
        <small v-if="preset.clockLabel" class="preset-clock numeric">
          {{ preset.clockLabel }}
        </small>
      </button>
    </div>

    <div class="deadline-direct">
      <span class="deadline-direct-label">具体时间</span>
      <div class="deadline-direct-controls">
        <div ref="dateControl" class="deadline-date-control">
          <button
            :id="`${pickerId}-date`"
            class="deadline-date-trigger"
            type="button"
            :aria-expanded="calendarOpen"
            aria-haspopup="dialog"
            @click="toggleCalendar"
            @keydown.esc.stop="calendarOpen = false"
          >
            <CalendarDays :size="16" aria-hidden="true" />
            <span>{{ dateLabel }}</span>
          </button>

          <div
            v-if="calendarOpen"
            class="deadline-calendar"
            role="dialog"
            aria-label="选择 Deadline 日期"
            @keydown.esc.stop="calendarOpen = false"
          >
            <div class="deadline-calendar-header">
              <button
                class="calendar-nav"
                type="button"
                aria-label="上个月"
                :disabled="!canGoPrevious"
                @click="shiftMonth(-1)"
              >
                <ChevronLeft :size="17" aria-hidden="true" />
              </button>
              <strong>{{ monthLabel }}</strong>
              <button
                class="calendar-nav"
                type="button"
                aria-label="下个月"
                @click="shiftMonth(1)"
              >
                <ChevronRight :size="17" aria-hidden="true" />
              </button>
            </div>
            <div class="calendar-weekdays" aria-hidden="true">
              <span v-for="weekday in weekdays" :key="weekday">{{ weekday }}</span>
            </div>
            <div class="calendar-days" role="grid">
              <button
                v-for="day in calendarDays"
                :key="day.value"
                class="calendar-day"
                :class="{
                  outside: !day.inMonth,
                  today: day.isToday,
                  selected: day.value === selectedDateValue,
                }"
                type="button"
                :disabled="isPastDay(day.value)"
                :aria-label="`${day.date.getFullYear()}年${day.date.getMonth() + 1}月${day.dayNumber}日`"
                :aria-selected="day.value === selectedDateValue"
                @click="selectDate(day.value)"
              >
                {{ day.dayNumber }}
              </button>
            </div>
          </div>
        </div>

        <TimeWheelPicker
          class="deadline-time-control"
          :hour="selectedHour"
          :minute="selectedMinute"
          @update:hour="selectTime('hour', $event)"
          @update:minute="selectTime('minute', $event)"
        />
      </div>
    </div>
    <span v-if="invalid" class="deadline-error">请选择晚于当前时间的 Deadline。</span>
  </div>
</template>

<style scoped>
.deadline-picker {
  --deadline-input-height: 56px;

  display: grid;
  gap: 9px;
}

.deadline-heading {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.deadline-heading > span {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  color: var(--text-primary);
  font-size: 0.86rem;
  font-weight: 600;
}

.deadline-heading small {
  color: var(--text-tertiary);
  font-size: 0.75rem;
  font-weight: 400;
}

.deadline-presets {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 6px;
}

.deadline-preset {
  display: inline-flex;
  flex-direction: column;
  min-width: 0;
  min-height: 36px;
  align-items: center;
  justify-content: center;
  gap: 5px;
  padding: 6px 8px;
  overflow: hidden;
  color: var(--text-secondary);
  font-size: 0.75rem;
  font-weight: 600;
  text-overflow: ellipsis;
  white-space: nowrap;
  background: var(--bg-subtle);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition:
    color 160ms var(--ease-out),
    background-color 160ms var(--ease-out),
    border-color 160ms var(--ease-out);
}

.preset-clock {
  color: var(--text-tertiary);
  font-size: 0.75rem;
  font-weight: 400;
}

.deadline-preset.selected .preset-clock {
  color: var(--accent);
}

.deadline-preset:hover:not(:disabled) {
  color: var(--accent);
  background: var(--accent-soft);
  border-color: var(--accent);
}

.deadline-preset.selected {
  color: var(--accent);
  background: var(--accent-soft);
  border-color: var(--accent);
}

.deadline-preset:disabled {
  cursor: not-allowed;
  opacity: 0.38;
}

.deadline-direct {
  display: grid;
  gap: 6px;
}

.deadline-direct-label {
  color: var(--text-secondary);
  font-size: 0.75rem;
  font-weight: 600;
}

.deadline-direct-controls {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 208px));
  align-items: start;
  justify-content: start;
  gap: 8px;
}

.deadline-time-control {
  --time-wheel-width: 100%;
}

.deadline-date-control {
  position: relative;
  display: grid;
  min-width: 0;
  gap: 6px;
}

.deadline-date-trigger {
  display: flex;
  width: 100%;
  min-width: 0;
  min-height: var(--deadline-input-height);
  align-items: center;
  gap: 8px;
  padding: 5px 10px;
  color: var(--text-primary);
  text-align: left;
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.deadline-date-trigger:hover,
.deadline-date-trigger[aria-expanded="true"] {
  border-color: var(--accent);
}

.deadline-date-trigger span {
  min-width: 0;
  overflow: hidden;
  font-size: 0.86rem;
  font-weight: 600;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.deadline-calendar {
  z-index: 4;
  width: min(100%, 304px);
  padding: 9px;
  background: var(--bg-surface);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-float);
}

.deadline-calendar-header {
  display: grid;
  grid-template-columns: 34px 1fr 34px;
  align-items: center;
  margin-bottom: 7px;
}

.deadline-calendar-header strong {
  color: var(--text-primary);
  font-size: 0.86rem;
  font-weight: 620;
  text-align: center;
}

.calendar-nav {
  display: grid;
  width: 34px;
  height: 34px;
  place-items: center;
  padding: 0;
  color: var(--text-secondary);
  background: transparent;
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.calendar-nav:hover:not(:disabled) {
  color: var(--accent);
  background: var(--accent-soft);
}

.calendar-nav:disabled {
  opacity: 0.28;
}

.calendar-weekdays,
.calendar-days {
  display: grid;
  grid-template-columns: repeat(7, minmax(0, 1fr));
}

.calendar-weekdays span {
  display: grid;
  height: 25px;
  place-items: center;
  color: var(--text-tertiary);
  font-size: 0.75rem;
}

.calendar-day {
  display: grid;
  min-width: 30px;
  min-height: 30px;
  place-items: center;
  padding: 0;
  color: var(--text-primary);
  font-size: 0.75rem;
  font-variant-numeric: tabular-nums;
  background: transparent;
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.calendar-day:hover:not(:disabled) {
  color: var(--accent);
  background: var(--accent-soft);
}

.calendar-day.outside {
  color: var(--text-tertiary);
}

.calendar-day.today {
  box-shadow: inset 0 0 0 1px var(--accent);
}

.calendar-day.selected {
  color: #fff;
  font-weight: 600;
  background: var(--accent);
  box-shadow: none;
}

.calendar-day:disabled {
  cursor: not-allowed;
  opacity: 0.25;
}

.deadline-error {
  color: var(--danger);
  font-size: 0.75rem;
}

@media (max-width: 700px) {
  .deadline-presets {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}

@media (max-width: 560px) {
  .deadline-direct-controls {
    grid-template-columns: minmax(0, 208px);
  }
}
</style>
