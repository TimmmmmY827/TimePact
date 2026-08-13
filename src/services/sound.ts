import { convertFileSrc } from "@tauri-apps/api/core";
import { runningInTauri } from "./backend";

export async function playSound(path: string | null) {
  if (path) {
    await new Audio(runningInTauri() ? convertFileSrc(path) : path).play();
    return;
  }

  const AudioContextClass = window.AudioContext;
  const context = new AudioContextClass();
  const gain = context.createGain();
  gain.gain.setValueAtTime(0.0001, context.currentTime);
  gain.gain.exponentialRampToValueAtTime(0.18, context.currentTime + 0.02);
  gain.gain.exponentialRampToValueAtTime(0.0001, context.currentTime + 0.5);
  gain.connect(context.destination);
  for (const [frequency, offset] of [
    [660, 0],
    [880, 0.12],
  ] as const) {
    const oscillator = context.createOscillator();
    oscillator.type = "sine";
    oscillator.frequency.value = frequency;
    oscillator.connect(gain);
    oscillator.start(context.currentTime + offset);
    oscillator.stop(context.currentTime + offset + 0.34);
  }
  window.setTimeout(() => void context.close(), 700);
}
