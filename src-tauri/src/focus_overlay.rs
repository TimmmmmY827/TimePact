use tauri::{
    AppHandle, Manager, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindowBuilder,
    window::Color,
};

use crate::models::FocusSession;

const WINDOW_PREFIX: &str = "focus-overlay-";

#[derive(Debug, Clone, PartialEq, Eq)]
struct OverlaySpec {
    label: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

fn overlay_specs(monitors: impl IntoIterator<Item = (i32, i32, u32, u32)>) -> Vec<OverlaySpec> {
    monitors
        .into_iter()
        .enumerate()
        .map(|(index, (x, y, width, height))| OverlaySpec {
            label: format!("{WINDOW_PREFIX}{index}"),
            x,
            y,
            width,
            height,
        })
        .collect()
}

pub fn should_show(focus: Option<&FocusSession>) -> bool {
    focus.is_some_and(|session| session.phase == "focus" && session.running)
}

pub fn set_active(app: &AppHandle, active: bool) -> Result<(), String> {
    let mut errors = Vec::new();

    if !active {
        for (label, window) in app.webview_windows() {
            if label.starts_with(WINDOW_PREFIX)
                && window.is_visible().unwrap_or(true)
                && let Err(error) = window.hide()
            {
                errors.push(format!("{label}: {error}"));
            }
        }
        return if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("; "))
        };
    }

    let monitors = app
        .available_monitors()
        .map_err(|error| error.to_string())?;
    let specs = overlay_specs(monitors.iter().map(|monitor| {
        (
            monitor.position().x,
            monitor.position().y,
            monitor.size().width,
            monitor.size().height,
        )
    }));
    for spec in &specs {
        let (window, newly_created) = if let Some(window) = app.get_webview_window(&spec.label) {
            (window, false)
        } else {
            match WebviewWindowBuilder::new(
                app,
                &spec.label,
                WebviewUrl::App("index.html#/focus-overlay".into()),
            )
            .title("TimePact Focus Aura")
            .decorations(false)
            .transparent(true)
            .background_color(Color(0, 0, 0, 0))
            .shadow(false)
            .always_on_top(true)
            .visible_on_all_workspaces(true)
            .skip_taskbar(true)
            .focusable(false)
            .resizable(false)
            .focused(false)
            .visible(false)
            .inner_size(1.0, 1.0)
            .build()
            {
                Ok(window) => (window, true),
                Err(error) => {
                    errors.push(format!("{}: {error}", spec.label));
                    continue;
                }
            }
        };

        let target_position = PhysicalPosition::new(spec.x, spec.y);
        if window
            .outer_position()
            .map_or(true, |value| value != target_position)
            && let Err(error) = window.set_position(target_position)
        {
            errors.push(format!("{} position: {error}", spec.label));
        }

        let target_size = PhysicalSize::new(spec.width, spec.height);
        if window
            .inner_size()
            .map_or(true, |value| value != target_size)
            && let Err(error) = window.set_size(target_size)
        {
            errors.push(format!("{} size: {error}", spec.label));
        }

        if newly_created && let Err(error) = window.set_ignore_cursor_events(true) {
            errors.push(format!("{} click-through: {error}", spec.label));
        }
        if !window.is_visible().unwrap_or(false)
            && let Err(error) = window.show()
        {
            errors.push(format!("{} show: {error}", spec.label));
        }
    }

    for (label, window) in app.webview_windows() {
        if let Some(index) = label
            .strip_prefix(WINDOW_PREFIX)
            .and_then(|value| value.parse::<usize>().ok())
            && index >= specs.len()
            && window.is_visible().unwrap_or(true)
            && let Err(error) = window.hide()
        {
            errors.push(format!("{label}: {error}"));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("; "))
    }
}

#[cfg(test)]
mod tests {
    use super::{OverlaySpec, overlay_specs, should_show};
    use crate::models::FocusSession;

    fn session(phase: &str, running: bool) -> FocusSession {
        FocusSession {
            id: "test".into(),
            task_id: None,
            phase: phase.into(),
            planned_focus_seconds: 1500,
            planned_rest_seconds: 300,
            remaining_seconds: 1200,
            elapsed_seconds: 300,
            paused_seconds: 0,
            round: 1,
            running,
            started_at: "2026-08-12T00:00:00Z".into(),
        }
    }

    #[test]
    fn only_running_focus_shows_screen_overlay() {
        assert!(should_show(Some(&session("focus", true))));
        assert!(!should_show(Some(&session("focus", false))));
        assert!(!should_show(Some(&session("rest", true))));
        assert!(!should_show(Some(&session("waiting-rest", false))));
        assert!(!should_show(None));
    }

    #[test]
    fn creates_one_exact_overlay_for_each_monitor() {
        assert_eq!(
            overlay_specs([
                (0, 0, 2560, 1440),
                (2560, 0, 3840, 2160),
                (-1080, -320, 1080, 1920),
            ]),
            vec![
                OverlaySpec {
                    label: "focus-overlay-0".into(),
                    x: 0,
                    y: 0,
                    width: 2560,
                    height: 1440,
                },
                OverlaySpec {
                    label: "focus-overlay-1".into(),
                    x: 2560,
                    y: 0,
                    width: 3840,
                    height: 2160,
                },
                OverlaySpec {
                    label: "focus-overlay-2".into(),
                    x: -1080,
                    y: -320,
                    width: 1080,
                    height: 1920,
                },
            ]
        );
    }
}
