import { describe, expect, it } from "vitest";
import {
  DEFAULT_WORKDAY_END_TIMES,
  normalizeWorkdayEndTimes,
  stepClockPart,
  workdayEndForDate,
} from "./work-hours";

describe("weekly work-end times", () => {
  it("provides an independent 18:00 value for every weekday", () => {
    expect(DEFAULT_WORKDAY_END_TIMES).toEqual([
      "18:00",
      "18:00",
      "18:00",
      "18:00",
      "18:00",
      "18:00",
      "18:00",
    ]);
    expect(normalizeWorkdayEndTimes(undefined)).toEqual(DEFAULT_WORKDAY_END_TIMES);
  });

  it("normalizes malformed or incomplete saved values by weekday", () => {
    expect(normalizeWorkdayEndTimes(["17:30", "bad", "19:05"])).toEqual([
      "17:30",
      "18:00",
      "19:05",
      "18:00",
      "18:00",
      "18:00",
      "18:00",
    ]);
  });

  it("maps local dates onto a Monday-first settings array", () => {
    const values = ["17:00", "17:10", "17:20", "17:30", "17:40", "17:50", "18:00"];
    expect(workdayEndForDate(new Date(2026, 6, 27), values)).toEqual({ hour: 17, minute: 0 });
    expect(workdayEndForDate(new Date(2026, 7, 2), values)).toEqual({ hour: 18, minute: 0 });
  });

  it("wraps hours and minutes in both directions", () => {
    expect(stepClockPart(23, 1, 24)).toBe(0);
    expect(stepClockPart(0, -1, 24)).toBe(23);
    expect(stepClockPart(59, 1, 60)).toBe(0);
    expect(stepClockPart(0, -1, 60)).toBe(59);
  });
});
