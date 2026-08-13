export function shouldHideReminder(initialized: boolean, waitingCount: number): boolean {
  return initialized && waitingCount === 0;
}
