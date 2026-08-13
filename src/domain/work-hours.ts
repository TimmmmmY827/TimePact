export const DEFAULT_WORKDAY_END_TIMES = Object.freeze([
  "18:00",
  "18:00",
  "18:00",
  "18:00",
  "18:00",
  "18:00",
  "18:00",
]);

const CLOCK_VALUE_PATTERN = /^(?:[01]\d|2[0-3]):[0-5]\d$/;

export function isClockValue(value: unknown): value is string {
  return typeof value === "string" && CLOCK_VALUE_PATTERN.test(value);
}

export function normalizeWorkdayEndTimes(value: unknown): string[] {
  const source = Array.isArray(value) ? value : [];
  return DEFAULT_WORKDAY_END_TIMES.map((fallback, index) =>
    isClockValue(source[index]) ? source[index] : fallback,
  );
}

export function workdayEndForDate(
  date: Date,
  workdayEndTimes: readonly string[],
): { hour: number; minute: number } {
  const mondayFirstIndex = (date.getDay() + 6) % 7;
  const normalized = normalizeWorkdayEndTimes(workdayEndTimes);
  const [hour, minute] = normalized[mondayFirstIndex].split(":").map(Number);
  return { hour, minute };
}

export function stepClockPart(value: number, delta: number, cycle: number): number {
  const normalized = Math.round(value) + Math.sign(delta);
  return ((normalized % cycle) + cycle) % cycle;
}
