export function clampMinutes(value: number, min = 1, max = 180): number {
  if (!Number.isFinite(value)) return min;
  return Math.min(max, Math.max(min, Math.round(value)));
}

export function stepMinutes(
  value: number,
  delta: number,
  min = 1,
  max = 180,
): number {
  return clampMinutes(value + delta, min, max);
}
