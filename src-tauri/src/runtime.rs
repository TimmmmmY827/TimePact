use std::{
    fs::{self, OpenOptions},
    io::Write,
    thread,
    time::{Duration, Instant, SystemTime},
};

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use tauri::{AppHandle, Emitter, Manager};

use crate::{
    commands::default_snapshot_name,
    db::{Database, append_event, export_backup, get_focus, get_settings, list_tasks, save_task},
    focus_overlay,
};

fn create_snapshot(app: &AppHandle) -> Result<(), String> {
    let state = app.state::<Database>();
    let connection = state.0.lock().map_err(|_| "数据库锁不可用。")?;
    let backup = export_backup(&connection)?;
    drop(connection);

    let snapshot_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?
        .join("snapshots");
    fs::create_dir_all(&snapshot_dir).map_err(|error| error.to_string())?;
    fs::write(
        snapshot_dir.join(default_snapshot_name()),
        serde_json::to_string_pretty(&backup).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;

    let cutoff = SystemTime::now() - Duration::from_secs(8 * 24 * 60 * 60);
    for entry in fs::read_dir(snapshot_dir).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        if entry
            .metadata()
            .and_then(|metadata| metadata.modified())
            .is_ok_and(|modified| modified < cutoff)
        {
            let _ = fs::remove_file(entry.path());
        }
    }
    Ok(())
}

pub(crate) fn write_log(app: &AppHandle, message: &str) {
    let Ok(data_dir) = app.path().app_data_dir() else {
        return;
    };
    let log_dir = data_dir.join("logs");
    if fs::create_dir_all(&log_dir).is_err() {
        return;
    }
    let name = format!("{}.log", Utc::now().format("%Y-%m-%d"));
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_dir.join(name))
    {
        let _ = writeln!(file, "{} {message}", Utc::now().to_rfc3339());
    }
    let cutoff = SystemTime::now() - Duration::from_secs(8 * 24 * 60 * 60);
    if let Ok(entries) = fs::read_dir(log_dir) {
        for entry in entries.flatten() {
            if entry
                .metadata()
                .and_then(|metadata| metadata.modified())
                .is_ok_and(|modified| modified < cutoff)
            {
                let _ = fs::remove_file(entry.path());
            }
        }
    }
}

fn update_due_tasks(app: &AppHandle, suspended_seconds: i64) -> Result<Vec<String>, String> {
    let state = app.state::<Database>();
    let connection = state.0.lock().map_err(|_| "数据库锁不可用。")?;
    let mut tasks = list_tasks(&connection, false)?;
    let now = Utc::now();
    let mut due_ids = Vec::new();

    for task in &mut tasks {
        let Some(timer) = task.timer.as_mut() else {
            continue;
        };

        if suspended_seconds > 0
            && timer.kind == "countdown"
            && task.status == "running"
            && timer.current_due_at.is_some()
            && let Some(due) = timer
                .current_due_at
                .as_deref()
                .and_then(|value| DateTime::parse_from_rfc3339(value).ok())
        {
            timer.current_due_at =
                Some((due + ChronoDuration::seconds(suspended_seconds)).to_rfc3339());
        }

        let is_due = matches!(task.status.as_str(), "ready" | "running")
            && timer
                .current_due_at
                .as_deref()
                .and_then(|value| DateTime::parse_from_rfc3339(value).ok())
                .is_some_and(|due| due <= now);
        if is_due {
            task.status = "waiting".into();
            timer.remaining_seconds = Some(0);
            task.updated_at = now.to_rfc3339();
            save_task(&connection, task)?;
            append_event(&connection, task, "due", serde_json::json!({}))?;
            due_ids.push(task.id.clone());
        }
    }
    Ok(due_ids)
}

fn show_reminder(app: &AppHandle, task_ids: &[String]) {
    if task_ids.is_empty() {
        if let Some(window) = app.get_webview_window("reminder") {
            let _ = window.hide();
        }
        return;
    }

    let _ = app.emit("timepact://task-due", task_ids);
    if let Some(window) = app.get_webview_window("reminder") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

fn waiting_task_ids(app: &AppHandle) -> Result<Vec<String>, String> {
    let state = app.state::<Database>();
    let connection = state.0.lock().map_err(|_| "数据库锁不可用。")?;
    Ok(list_tasks(&connection, false)?
        .into_iter()
        .filter(|task| task.status == "waiting")
        .map(|task| task.id)
        .collect())
}

pub fn spawn(app: AppHandle) {
    thread::spawn(move || {
        write_log(&app, "runtime started");
        let _ = create_snapshot(&app);
        match waiting_task_ids(&app) {
            Ok(task_ids) => show_reminder(&app, &task_ids),
            Err(error) => write_log(&app, &format!("startup reminder error: {error}")),
        }
        let mut previous_tick = Instant::now();
        let mut previous_snapshot = Instant::now();
        let mut previous_repeat = Instant::now();
        let mut previous_overlay_sync = Instant::now();
        let mut overlay_was_active = false;

        loop {
            thread::sleep(Duration::from_secs(1));
            let elapsed = previous_tick.elapsed();
            previous_tick = Instant::now();
            let suspended_seconds = if elapsed > Duration::from_secs(5) {
                elapsed.as_secs().saturating_sub(1) as i64
            } else {
                0
            };

            match update_due_tasks(&app, suspended_seconds) {
                Ok(task_ids) if !task_ids.is_empty() => {
                    show_reminder(&app, &task_ids);
                    previous_repeat = Instant::now();
                }
                Ok(_) => {}
                Err(error) => write_log(&app, &format!("scheduler error: {error}")),
            }

            let repeat_minutes = app
                .state::<Database>()
                .0
                .lock()
                .ok()
                .and_then(|connection| get_settings(&connection).ok())
                .and_then(|settings| settings.repeat_reminder_minutes);
            if repeat_minutes.is_some_and(|minutes| {
                minutes > 0
                    && previous_repeat.elapsed()
                        >= Duration::from_secs((minutes as u64).saturating_mul(60))
            }) {
                let waiting_ids = waiting_task_ids(&app).unwrap_or_default();
                if !waiting_ids.is_empty() {
                    show_reminder(&app, &waiting_ids);
                }
                previous_repeat = Instant::now();
            }

            if previous_snapshot.elapsed() >= Duration::from_secs(24 * 60 * 60) {
                if let Err(error) = create_snapshot(&app) {
                    write_log(&app, &format!("snapshot error: {error}"));
                }
                previous_snapshot = Instant::now();
            }

            if previous_overlay_sync.elapsed() >= Duration::from_secs(2) {
                let overlay_active = app
                    .state::<Database>()
                    .0
                    .lock()
                    .ok()
                    .and_then(|connection| get_focus(&connection).ok())
                    .flatten();
                let overlay_active = focus_overlay::should_show(overlay_active.as_ref());
                if (overlay_active || overlay_was_active)
                    && let Err(error) = focus_overlay::set_active(&app, overlay_active)
                {
                    write_log(&app, &format!("focus overlay sync error: {error}"));
                }
                overlay_was_active = overlay_active;
                previous_overlay_sync = Instant::now();
            }
        }
    });
}
