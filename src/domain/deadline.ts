import {
  DEFAULT_WORKDAY_END_TIMES,
  workdayEndForDate,
} from "./work-hours";

export type DeadlinePreset =
  | "today-work-end"
  | "tomorrow-work-end"
  | "week-end"
  | "month-end";

export const DEADLINE_PRESETS: ReadonlyArray<{
  value: DeadlinePreset;
  label: string;
}> = [
  { value: "today-work-end", label: "今天下班" },
  { value: "tomorrow-work-end", label: "明天下班" },
  { value: "week-end", label: "本周结束" },
  { value: "month-end", label: "本月结束" },
];

const pad = (value: number) => value.toString().padStart(2, "0");

export interface CalendarDay {
  date: Date;
  value: string;
  dayNumber: number;
  inMonth: boolean;
  isToday: boolean;
}

export function toLocalDateValue(date: Date): string {
  return [
    date.getFullYear(),
    "-",
    pad(date.getMonth() + 1),
    "-",
    pad(date.getDate()),
  ].join("");
}

export function toLocalDateTimeValue(date: Date): string {
  return [
    toLocalDateValue(date),
    "T",
    pad(date.getHours()),
    ":",
    pad(date.getMinutes()),
  ].join("");
}

export function parseLocalDateTimeValue(value: string): {
  dateValue: string;
  hour: number;
  minute: number;
} | null {
  const match = /^(\d{4}-\d{2}-\d{2})T(\d{2}):(\d{2})$/.exec(value);
  if (!match) return null;
  const hour = Number(match[2]);
  const minute = Number(match[3]);
  const parsed = new Date(`${match[1]}T${match[2]}:${match[3]}`);
  if (
    !Number.isFinite(parsed.getTime()) ||
    hour < 0 ||
    hour > 23 ||
    minute < 0 ||
    minute > 59 ||
    toLocalDateTimeValue(parsed) !== value
  ) {
    return null;
  }
  return { dateValue: match[1], hour, minute };
}

export function combineLocalDeadline(dateValue: string, hour: number, minute: number): string {
  const safeHour = Math.min(23, Math.max(0, Math.round(hour)));
  const safeMinute = Math.min(59, Math.max(0, Math.round(minute)));
  return `${dateValue}T${pad(safeHour)}:${pad(safeMinute)}`;
}

export function calendarMonthDays(
  year: number,
  month: number,
  today = new Date(),
): CalendarDay[] {
  const first = new Date(year, month, 1);
  const mondayFirstOffset = (first.getDay() + 6) % 7;
  const gridStart = new Date(year, month, 1 - mondayFirstOffset);
  const todayValue = toLocalDateValue(today);

  return Array.from({ length: 42 }, (_, index) => {
    const date = new Date(
      gridStart.getFullYear(),
      gridStart.getMonth(),
      gridStart.getDate() + index,
    );
    const value = toLocalDateValue(date);
    return {
      date,
      value,
      dayNumber: date.getDate(),
      inMonth: date.getMonth() === month,
      isToday: value === todayValue,
    };
  });
}

export function resolveDeadlinePreset(
  preset: DeadlinePreset,
  now = new Date(),
  workdayEndTimes: readonly string[] = DEFAULT_WORKDAY_END_TIMES,
): Date {
  const year = now.getFullYear();
  const month = now.getMonth();
  const day = now.getDate();

  if (preset === "today-work-end") {
    const { hour, minute } = workdayEndForDate(now, workdayEndTimes);
    return new Date(year, month, day, hour, minute, 0, 0);
  }

  if (preset === "tomorrow-work-end") {
    const tomorrow = new Date(year, month, day + 1);
    const { hour, minute } = workdayEndForDate(tomorrow, workdayEndTimes);
    return new Date(
      tomorrow.getFullYear(),
      tomorrow.getMonth(),
      tomorrow.getDate(),
      hour,
      minute,
      0,
      0,
    );
  }

  if (preset === "week-end") {
    const daysUntilSunday = (7 - now.getDay()) % 7;
    return new Date(year, month, day + daysUntilSunday, 23, 59, 0, 0);
  }

  return new Date(year, month + 1, 0, 23, 59, 0, 0);
}

export function isFutureDeadline(value: string, now = new Date()): boolean {
  if (!value) return false;
  const timestamp = new Date(value).getTime();
  return Number.isFinite(timestamp) && timestamp > now.getTime();
}
