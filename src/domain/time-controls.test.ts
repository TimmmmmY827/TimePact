import { describe, expect, it } from "vitest";
import { clampMinutes, stepMinutes } from "./time-controls";

describe("time control helpers", () => {
  it("clamps and rounds minute values", () => {
    expect(clampMinutes(0)).toBe(1);
    expect(clampMinutes(181)).toBe(180);
    expect(clampMinutes(24.6)).toBe(25);
  });

  it("normalizes invalid numeric input", () => {
    expect(clampMinutes(Number.NaN)).toBe(1);
    expect(clampMinutes(Number.POSITIVE_INFINITY)).toBe(1);
  });

  it("steps within the supplied range", () => {
    expect(stepMinutes(25, 5)).toBe(30);
    expect(stepMinutes(3, -5)).toBe(1);
    expect(stepMinutes(5990, 15, 1, 5999)).toBe(5999);
  });
});
