import { describe, expect, it } from "vitest";
import { createTaskFromDraft, sortByUrgency, taskProgress } from "./task";

describe("task domain", () => {
  it("creates an unscheduled task when no timer is supplied", () => {
    const task = createTaskFromDraft({ title: "  写周报  " }, new Date("2026-07-30T00:00:00Z"));
    expect(task.title).toBe("写周报");
    expect(task.status).toBe("unscheduled");
    expect(task.timer).toBeNull();
  });

  it("locks a deadline at creation", () => {
    const task = createTaskFromDraft(
      { title: "提交", timerKind: "deadline", deadlineAt: "2026-08-01T10:00:00Z" },
      new Date("2026-07-30T00:00:00Z"),
    );
    expect(task.timer?.kind).toBe("deadline");
    expect(task.timer?.locked).toBe(true);
  });

  it("sorts waiting work before unscheduled work", () => {
    const unscheduled = createTaskFromDraft({ title: "未安排" });
    const waiting = { ...createTaskFromDraft({ title: "待确认" }), status: "waiting" as const };
    expect(sortByUrgency([unscheduled, waiting])[0].title).toBe("待确认");
  });

  it("calculates countdown progress from remaining time", () => {
    const task = createTaskFromDraft({ title: "倒计时", timerKind: "countdown", countdownMinutes: 60 });
    if (task.timer) task.timer.remainingSeconds = 1800;
    expect(taskProgress(task)).toBe(0.5);
  });

  it("starts deadline progress when a previously unscheduled task is scheduled", () => {
    const task = createTaskFromDraft(
      { title: "稍后安排" },
      new Date("2026-07-01T00:00:00Z"),
    );
    task.timer = {
      kind: "deadline",
      originalDueAt: "2026-07-30T12:00:00Z",
      currentDueAt: "2026-07-30T12:00:00Z",
      durationSeconds: null,
      remainingSeconds: null,
      startedAt: "2026-07-30T10:00:00Z",
      locked: true,
      pausedAt: null,
    };
    expect(taskProgress(task, new Date("2026-07-30T11:00:00Z").getTime())).toBe(0.5);
  });
});
