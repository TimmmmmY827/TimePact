<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useRoute } from "vue-router";
import {
  ChartNoAxesColumnIncreasing,
  House,
  Settings,
  Timer,
} from "@lucide/vue";
import { useAppStore } from "./stores/app";
import WindowTitlebar from "./components/shell/WindowTitlebar.vue";

const route = useRoute();
const store = useAppStore();
const reminderOnly = computed(() => route.name === "reminder");
const screenOverlayOnly = computed(
  () => route.name === "focus-overlay" || window.location.hash.startsWith("#/focus-overlay"),
);

const primaryNavigation = [
  { to: "/", label: "主页", icon: House },
  { to: "/focus", label: "专注", icon: Timer },
  { to: "/stats", label: "统计", icon: ChartNoAxesColumnIncreasing },
];

const settingsNavigation = { to: "/settings", label: "设置" };

onMounted(() => {
  if (!window.location.hash.startsWith("#/focus-overlay")) void store.initialize();
});
</script>

<template>
  <RouterView v-if="reminderOnly || screenOverlayOnly" />
  <div v-else class="desktop-shell">
    <WindowTitlebar />
    <div class="app-shell">
      <aside class="nav-rail" aria-label="主导航">
        <RouterLink class="brand-mark" to="/" aria-label="TimePact 主页" title="TimePact">
          <span class="brand-ring"><span class="brand-check" /></span>
        </RouterLink>

        <nav class="nav-items">
          <RouterLink
            v-for="item in primaryNavigation"
            :key="item.to"
            :to="item.to"
            class="nav-item"
            :aria-label="item.label"
            :title="item.label"
          >
            <component :is="item.icon" :size="21" :stroke-width="1.8" aria-hidden="true" />
            <span class="sr-only">{{ item.label }}</span>
          </RouterLink>
        </nav>

        <RouterLink
          :to="settingsNavigation.to"
          class="nav-item nav-settings"
          :aria-label="settingsNavigation.label"
          :title="settingsNavigation.label"
        >
          <Settings :size="21" :stroke-width="1.8" aria-hidden="true" />
          <span class="sr-only">设置</span>
        </RouterLink>
      </aside>

      <main class="app-main">
        <div v-if="store.error" class="error-banner" role="alert">{{ store.error }}</div>
        <RouterView />
      </main>
    </div>
  </div>
</template>
