import { describe, expect, it } from "vitest";
import { shouldHideReminder } from "./reminder";

describe("reminder visibility", () => {
  it("hides an initialized reminder with no waiting tasks", () => {
    expect(shouldHideReminder(true, 0)).toBe(true);
  });

  it("does not hide while data is loading or tasks are waiting", () => {
    expect(shouldHideReminder(false, 0)).toBe(false);
    expect(shouldHideReminder(true, 1)).toBe(false);
  });
});
