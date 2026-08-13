<script setup lang="ts">
import { computed, ref } from "vue";
import {
  BellRing,
  BriefcaseBusiness,
  DatabaseBackup,
  FolderOpen,
  Info,
  Palette,
  Volume2,
} from "@lucide/vue";
import { open, save } from "@tauri-apps/plugin-dialog";
import { openPath } from "@tauri-apps/plugin-opener";
import { useAppStore } from "../stores/app";
import { backend, runningInTauri } from "../services/backend";
import { playSound } from "../services/sound";
import type { AppSettings } from "../domain/types";

const store = useAppStore();
const fileInput = ref<HTMLInputElement | null>(null);
const message = ref("");
const draft = computed({
  get: () => store.settings,
  set: (value: AppSettings) => store.updateSettings(value),
});

function patch<K extends keyof AppSettings>(key: K, value: AppSettings[K]) {
  store.updateSettings({ ...store.settings, [key]: value });
}

const soundRows = [
  { key: "taskDue", label: "待办到期" },
  { key: "focusDone", label: "专注结束" },
  { key: "restDone", label: "休息结束" },
] as const;

const weekdayRows = ["周一", "周二", "周三", "周四", "周五", "周六", "周日"];

async function patchWorkdayEnd(index: number, value: string) {
  const workdayEndTimes = [...store.settings.workdayEndTimes];
  workdayEndTimes[index] = value;
  await store.updateSettings({ ...store.settings, workdayEndTimes });
  message.value = `${weekdayRows[index]}下班时间已保存。`;
}

async function chooseSound(key: (typeof soundRows)[number]["key"]) {
  if (!runningInTauri()) {
    message.value = "安装版中可选择本地音频文件。";
    return;
  }
  const path = await open({
    multiple: false,
    filters: [{ name: "音频", extensions: ["mp3", "wav", "ogg", "m4a"] }],
  });
  if (typeof path === "string") {
    await store.updateSettings({
      ...store.settings,
      sounds: { ...store.settings.sounds, [key]: path },
    });
    message.value = "提示音已保存。";
  }
}

function previewSound(key: (typeof soundRows)[number]["key"]) {
  const path = store.settings.sounds[key];
  void playSound(path);
  message.value = path ? "正在试听自定义提示音。" : "正在试听默认提示音。";
}

async function exportBackup() {
  const content = await backend.exportBackup();
  const suggested = `timepact-backup-${new Date().toISOString().slice(0, 10)}.json`;
  const path = runningInTauri()
    ? await save({ defaultPath: suggested, filters: [{ name: "TimePact 备份", extensions: ["json"] }] })
    : suggested;
  if (path) {
    await backend.writeExportFile(path, content);
    message.value = "备份已导出。";
  }
}

async function importBackup() {
  if (!runningInTauri()) {
    fileInput.value?.click();
    return;
  }
  const path = await open({
    multiple: false,
    filters: [{ name: "TimePact 备份", extensions: ["json"] }],
  });
  if (typeof path !== "string") return;
  const content = await backend.readImportFile(path);
  await backend.importBackup(content);
  await store.reloadData();
  message.value = "备份已恢复。";
}

async function importPreviewFile(event: Event) {
  const file = (event.target as HTMLInputElement).files?.[0];
  if (!file) return;
  await backend.importBackup(await file.text());
  await store.reloadData();
  message.value = "备份已恢复。";
}

async function openDataDirectory() {
  const path = await backend.getDataDirectory();
  if (path && runningInTauri()) await openPath(path);
  else message.value = "数据目录仅在安装版中可打开。";
}
</script>

<template>
  <div class="view settings-view">
    <header class="view-header">
      <div>
        <h1 class="view-title">设置</h1>
        <p class="view-description">所有数据和日志仅保存在这台电脑上。</p>
      </div>
    </header>

    <section class="setting-section">
      <header>
        <Palette :size="18" aria-hidden="true" />
        <h2>外观与启动</h2>
      </header>
      <div class="setting-list panel">
        <label class="setting-row">
          <span><strong>主题</strong><small>默认跟随 Windows 设置</small></span>
          <select
            class="field compact-field"
            :value="draft.theme"
            @change="patch('theme', ($event.target as HTMLSelectElement).value as AppSettings['theme'])"
          >
            <option value="system">跟随系统</option>
            <option value="light">浅色</option>
            <option value="dark">深色</option>
          </select>
        </label>
        <label class="setting-row">
          <span><strong>开机自动启动</strong><small>启动后只驻留托盘</small></span>
          <input
            :checked="draft.autostart"
            type="checkbox"
            role="switch"
            @change="patch('autostart', ($event.target as HTMLInputElement).checked)"
          />
        </label>
      </div>
    </section>

    <section class="setting-section">
      <header>
        <BriefcaseBusiness :size="18" aria-hidden="true" />
        <h2>每周下班时间</h2>
      </header>
      <div class="setting-list panel">
        <label v-for="(weekday, index) in weekdayRows" :key="weekday" class="setting-row workday-row">
          <span>
            <strong>{{ weekday }}</strong>
            <small>用于对应日期的“下班” Deadline 快捷设置</small>
          </span>
          <input
            class="field workday-time numeric"
            type="time"
            step="60"
            :value="draft.workdayEndTimes[index]"
            :aria-label="`${weekday}下班时间`"
            @change="patchWorkdayEnd(index, ($event.target as HTMLInputElement).value)"
          />
        </label>
      </div>
    </section>

    <section class="setting-section">
      <header>
        <Volume2 :size="18" aria-hidden="true" />
        <h2>提醒声音</h2>
      </header>
      <div class="setting-list panel">
        <div v-for="item in soundRows" :key="item.key" class="setting-row">
          <span>
            <strong>{{ item.label }}</strong>
            <small>{{ draft.sounds[item.key] ? "自定义音频 · 单次播放" : "默认提示音 · 单次播放" }}</small>
          </span>
          <div class="setting-actions">
            <button class="button ghost" type="button" @click="previewSound(item.key)">试听</button>
            <button class="button" type="button" @click="chooseSound(item.key)">选择文件</button>
          </div>
        </div>
        <label class="setting-row">
          <span><strong>重复响铃</strong><small>默认关闭；提醒窗口仍会保持待确认</small></span>
          <select
            class="field compact-field"
            :value="draft.repeatReminderMinutes ?? 0"
            @change="
              patch(
                'repeatReminderMinutes',
                Number(($event.target as HTMLSelectElement).value) || null,
              )
            "
          >
            <option :value="0">关闭</option>
            <option :value="5">每 5 分钟</option>
            <option :value="10">每 10 分钟</option>
            <option :value="30">每 30 分钟</option>
          </select>
        </label>
      </div>
    </section>

    <section class="setting-section">
      <header>
        <DatabaseBackup :size="18" aria-hidden="true" />
        <h2>数据</h2>
      </header>
      <div class="setting-list panel">
        <div class="setting-row">
          <span><strong>备份与恢复</strong><small>每天自动备份，保留最近 7 份</small></span>
          <div class="setting-actions">
            <button class="button" type="button" @click="exportBackup">导出备份</button>
            <button class="button" type="button" @click="importBackup">导入备份</button>
            <input
              ref="fileInput"
              class="visually-hidden"
              type="file"
              accept=".json,application/json"
              @change="importPreviewFile"
            />
          </div>
        </div>
        <div class="setting-row">
          <span><strong>本地日志</strong><small>自动保留 7 天，不包含遥测</small></span>
          <button class="button" type="button" @click="openDataDirectory">
            <FolderOpen :size="15" aria-hidden="true" />
            打开目录
          </button>
        </div>
      </div>
    </section>

    <p v-if="message" class="settings-message" role="status">{{ message }}</p>

    <section class="about panel">
      <Info :size="18" aria-hidden="true" />
      <div><strong>TimePact</strong><span>版本 0.1.3 · 本地优先</span></div>
      <BellRing :size="17" aria-hidden="true" />
    </section>
  </div>
</template>

<style scoped>
.setting-section + .setting-section {
  margin-top: 28px;
}

.setting-section > header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin: 0 2px 9px;
}

.setting-section > header svg {
  color: var(--text-tertiary);
}

.setting-section h2 {
  font-size: 0.92rem;
}

.setting-list {
  overflow: hidden;
}

.setting-row {
  display: flex;
  min-height: 64px;
  align-items: center;
  justify-content: space-between;
  gap: 18px;
  padding: 11px 14px;
}

.setting-row + .setting-row {
  border-top: 1px solid var(--border);
}

.setting-row > span {
  display: grid;
  gap: 4px;
}

.setting-row strong {
  font-size: 0.84rem;
}

.setting-row small {
  color: var(--text-secondary);
  font-size: 0.73rem;
}

.compact-field {
  width: 136px;
}

.workday-row {
  min-height: 54px;
}

.workday-time {
  width: 112px;
}

.setting-actions {
  display: flex;
  gap: 6px;
}

input[type="checkbox"][role="switch"] {
  width: 36px;
  height: 20px;
  margin: 0;
  accent-color: var(--accent);
}

.about {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-top: 30px;
  padding: 14px;
  color: var(--text-secondary);
}

.settings-message {
  margin-top: 12px;
  color: var(--accent);
  font-size: 0.76rem;
}

.about > div {
  display: grid;
  flex: 1;
  gap: 3px;
}

.about strong {
  color: var(--text-primary);
  font-size: 0.83rem;
}

.about span {
  font-size: 0.72rem;
}
</style>
