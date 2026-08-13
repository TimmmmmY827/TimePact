import { createRouter, createWebHashHistory } from "vue-router";
import HomeView from "../views/HomeView.vue";
import FocusView from "../views/FocusView.vue";
import StatsView from "../views/StatsView.vue";
import SettingsView from "../views/SettingsView.vue";
import ReminderView from "../views/ReminderView.vue";
import FocusOverlayView from "../views/FocusOverlayView.vue";

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: "/", name: "home", component: HomeView },
    { path: "/focus", name: "focus", component: FocusView },
    { path: "/stats", name: "stats", component: StatsView },
    { path: "/settings", name: "settings", component: SettingsView },
    { path: "/reminder", name: "reminder", component: ReminderView },
    { path: "/focus-overlay", name: "focus-overlay", component: FocusOverlayView },
  ],
});
