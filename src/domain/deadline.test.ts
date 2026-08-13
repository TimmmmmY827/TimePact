import { describe, expect, it } from "vitest";
import {
  calendarMonthDays,
  combineLocalDeadline,
  isFutureDeadline,
  parseLocalDateTimeValue,
  resolveDeadlinePreset,
  toLocalDateValue,
  toLocalDateTimeValue,
} from "./deadline";

describe("deadline presets", () => {
  const morning = new Date(2026, 6, 31, 10, 30);

  it("uses 18:00 local time for today and tomorrow after work", () => {
    expect(toLocalDateTimeValue(resolveDeadlinePreset("today-work-end", morning))).toBe(
      "2026-07-31T18:00",
    );
    expect(toLocalDateTimeValue(resolveDeadlinePreset("tomorrow-work-end", morning))).toBe(
      "2026-08-01T18:00",
    );
  });

  it("uses the configured weekday work-end time for today and tomorrow", () => {
    const workdayEndTimes = ["17:00", "17:10", "17:20", "17:30", "17:40", "17:50", "18:00"];
    expect(
      toLocalDateTimeValue(
        resolveDeadlinePreset("today-work-end", morning, workdayEndTimes),
      ),
    ).toBe("2026-07-31T17:40");
    expect(
      toLocalDateTimeValue(
        resolveDeadlinePreset("tomorrow-work-end", morning, workdayEndTimes),
      ),
    ).toBe("2026-08-01T17:50");
  });

  it("uses Sunday 23:59 for the end of the current week", () => {
    expect(toLocalDateTimeValue(resolveDeadlinePreset("week-end", morning))).toBe(
      "2026-08-02T23:59",
    );

    const sunday = new Date(2026, 6, 26, 9, 0);
    expect(toLocalDateTimeValue(resolveDeadlinePreset("week-end", sunday))).toBe(
      "2026-07-26T23:59",
    );
  });

  it("uses the last calendar day at 23:59 for the end of the month", () => {
    const february = new Date(2028, 1, 3, 9, 0);
    expect(toLocalDateTimeValue(resolveDeadlinePreset("month-end", february))).toBe(
      "2028-02-29T23:59",
    );
  });

  it("recognizes whether a local deadline remains selectable", () => {
    expect(isFutureDeadline("2026-07-31T18:00", morning)).toBe(true);
    expect(isFutureDeadline("2026-07-31T18:00", new Date(2026, 6, 31, 18, 1))).toBe(false);
    expect(isFutureDeadline("", morning)).toBe(false);
  });

  it("builds a stable six-week Monday-first calendar grid", () => {
    const days = calendarMonthDays(2026, 7, morning);
    expect(days).toHaveLength(42);
    expect(days[0]).toMatchObject({ value: "2026-07-27", inMonth: false });
    expect(days[5]).toMatchObject({ value: "2026-08-01", inMonth: true });
    expect(days[41]).toMatchObject({ value: "2026-09-06", inMonth: false });
  });

  it("keeps leap days available in the calendar grid", () => {
    const days = calendarMonthDays(2028, 1, new Date(2028, 1, 3));
    expect(days.some((day) => day.value === "2028-02-29" && day.inMonth)).toBe(true);
  });

  it("parses and recombines local date and time segments", () => {
    expect(parseLocalDateTimeValue("2026-08-01T18:05")).toEqual({
      dateValue: "2026-08-01",
      hour: 18,
      minute: 5,
    });
    expect(combineLocalDeadline("2026-08-01", 18, 5)).toBe("2026-08-01T18:05");
    expect(toLocalDateValue(new Date(2026, 7, 1, 18, 5))).toBe("2026-08-01");
  });
});
