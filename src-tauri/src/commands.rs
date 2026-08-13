use std::{fs, path::PathBuf};

use chrono::{DateTime, Datelike, Utc};
use rusqlite::params;
use tauri::{AppHandle, State};
use tauri_plugin_autostart::ManagerExt;

use crate::{
    db::{
        Database, append_event, database_path, export_backup, get_focus, get_focus_record,
        get_settings as db_get_settings, get_task, import_backup, list_focus_history, list_tasks,
        save_focus_record, save_task as db_save_task, set_focus, set_settings,
    },
    models::{
        AppSettings, BackupDocument, FocusFinishResult, FocusRecord, FocusSession, Task, TaskDraft,
    },
};

fn with_connection<T>(
    state: &State<'_, Database>,
    callback: impl FnOnce(&rusqlite::Connection) -> Result<T, String>,
) -> Result<T, String> {
    let connection = state.0.lock().map_err(|_| "数据库锁不可用。")?;
    callback(&connection)
}

#[tauri::command]
pub fn list_active_tasks(state: State<'_, Database>) -> Result<Vec<Task>, String> {
    with_connection(&state, |connection| list_tasks(connection, false))
}

#[tauri::command]
pub fn list_archived_tasks(state: State<'_, Database>) -> Result<Vec<Task>, String> {
    with_connection(&state, |connection| list_tasks(connection, true))
}

#[tauri::command]
pub fn create_task(draft: TaskDraft, state: State<'_, Database>) -> Result<Task, String> {
    let task = draft.into_task()?;
    with_connection(&state, |connection| {
        db_save_task(connection, &task)?;
        append_event(connection, &task, "created", serde_json::json!({}))?;
        Ok(task)
    })
}

#[tauri::command]
pub fn save_task(mut task: Task, state: State<'_, Database>) -> Result<Task, String> {
    task.updated_at = Utc::now().to_rfc3339();
    with_connection(&state, |connection| {
        let existing = get_task(connection, &task.id)?.ok_or_else(|| "待办不存在。".to_string())?;
        if let Some(old) = &existing.timer {
            let next = task
                .timer
                .as_ref()
                .ok_or_else(|| "已设置的计时方式不能移除。".to_string())?;
            if old.kind != next.kind {
                return Err("已设置的计时方式不能切换。".into());
            }
            if old.kind == "deadline" && old.original_due_at != next.original_due_at {
                return Err("Deadline 设置后不可修改。".into());
            }
            if old.kind == "countdown"
                && old.locked
                && old.duration_seconds != next.duration_seconds
            {
                return Err("倒计时启动后不可修改初始时长。".into());
            }
        }
        db_save_task(connection, &task)?;
        append_event(connection, &task, "updated", serde_json::json!({}))?;
        Ok(task)
    })
}

#[tauri::command]
pub fn complete_task(task_id: String, state: State<'_, Database>) -> Result<Task, String> {
    with_connection(&state, |connection| {
        let mut task = get_task(connection, &task_id)?.ok_or_else(|| "待办不存在。".to_string())?;
        let now = Utc::now().to_rfc3339();
        task.status = "completed".into();
        task.completed_at = Some(now.clone());
        task.updated_at = now;
        db_save_task(connection, &task)?;
        append_event(connection, &task, "completed", serde_json::json!({}))?;
        Ok(task)
    })
}

#[tauri::command]
pub fn cancel_task(task_id: String, state: State<'_, Database>) -> Result<Task, String> {
    with_connection(&state, |connection| {
        let mut task = get_task(connection, &task_id)?.ok_or_else(|| "待办不存在。".to_string())?;
        let now = Utc::now().to_rfc3339();
        task.status = "cancelled".into();
        task.completed_at = Some(now.clone());
        task.updated_at = now;
        db_save_task(connection, &task)?;
        append_event(connection, &task, "cancelled", serde_json::json!({}))?;
        Ok(task)
    })
}

#[tauri::command]
pub fn delete_task(task_id: String, state: State<'_, Database>) -> Result<(), String> {
    with_connection(&state, |connection| {
        let task = get_task(connection, &task_id)?.ok_or_else(|| "待办不存在。".to_string())?;
        let may_delete = task.status == "unscheduled"
            || (task
                .timer
                .as_ref()
                .is_some_and(|timer| timer.kind == "countdown" && !timer.locked));
        if !may_delete {
            return Err("这个待办已产生执行记录，只能取消并归档。".into());
        }
        connection
            .execute("DELETE FROM tasks WHERE id = ?1", params![task_id])
            .map_err(|error| error.to_string())?;
        Ok(())
    })
}

#[tauri::command]
pub fn get_settings(state: State<'_, Database>) -> Result<AppSettings, String> {
    with_connection(&state, db_get_settings)
}

#[tauri::command]
pub fn update_settings(
    settings: AppSettings,
    state: State<'_, Database>,
    app: AppHandle,
) -> Result<AppSettings, String> {
    if !matches!(settings.theme.as_str(), "system" | "light" | "dark") {
        return Err("主题设置无效。".into());
    }
    if !(1..=180).contains(&settings.focus_minutes) || !(1..=180).contains(&settings.rest_minutes) {
        return Err("专注与休息时长必须为 1–180 分钟。".into());
    }
    if settings.workday_end_times.len() != 7
        || settings
            .workday_end_times
            .iter()
            .any(|value| !valid_clock_value(value))
    {
        return Err("每天的下班时间必须是有效的 HH:mm。".into());
    }
    let saved = with_connection(&state, |connection| {
        set_settings(connection, &settings)?;
        Ok(settings.clone())
    })?;
    if !cfg!(debug_assertions) {
        let autostart = app.autolaunch();
        if saved.autostart {
            autostart.enable().map_err(|error| error.to_string())?;
        } else {
            autostart.disable().map_err(|error| error.to_string())?;
        }
    }
    Ok(saved)
}

fn valid_clock_value(value: &str) -> bool {
    let Some((hour, minute)) = value.split_once(':') else {
        return false;
    };
    value.len() == 5
        && hour.len() == 2
        && minute.len() == 2
        && hour.parse::<u8>().is_ok_and(|value| value < 24)
        && minute.parse::<u8>().is_ok_and(|value| value < 60)
}

#[tauri::command]
pub fn get_active_focus(state: State<'_, Database>) -> Result<Option<FocusSession>, String> {
    with_connection(&state, get_focus)
}

#[tauri::command]
pub async fn save_focus(
    focus: Option<FocusSession>,
    state: State<'_, Database>,
    app: AppHandle,
) -> Result<Option<FocusSession>, String> {
    if let Some(session) = &focus
        && (session.remaining_seconds < 0 || session.round < 1)
    {
        return Err("专注状态无效。".into());
    }
    let show_overlay = crate::focus_overlay::should_show(focus.as_ref());
    let saved = with_connection(&state, |connection| {
        set_focus(connection, &focus)?;
        Ok(focus)
    })?;
    if let Err(error) = crate::focus_overlay::set_active(&app, show_overlay) {
        crate::runtime::write_log(&app, &format!("focus overlay update error: {error}"));
    }
    Ok(saved)
}

#[tauri::command]
pub fn list_focus_records(state: State<'_, Database>) -> Result<Vec<FocusRecord>, String> {
    with_connection(&state, list_focus_history)
}

#[tauri::command]
pub fn finish_focus(
    focus: FocusSession,
    state: State<'_, Database>,
    app: AppHandle,
) -> Result<FocusFinishResult, String> {
    let result = with_connection(&state, |connection| {
        if let Some(record) = get_focus_record(connection, &focus.id)? {
            let task = focus
                .task_id
                .as_deref()
                .map(|task_id| get_task(connection, task_id))
                .transpose()?
                .flatten();
            return Ok(FocusFinishResult { task, record });
        }
        let transaction = connection
            .unchecked_transaction()
            .map_err(|error| error.to_string())?;
        let record = FocusRecord {
            id: focus.id.clone(),
            task_id: focus.task_id.clone(),
            started_at: focus.started_at.clone(),
            ended_at: Utc::now().to_rfc3339(),
            focused_seconds: focus.elapsed_seconds.max(0),
            rounds: focus.round.max(1),
        };
        let mut updated_task = None;
        if let Some(task_id) = &focus.task_id
            && let Some(mut task) = get_task(&transaction, task_id)?
        {
            task.focused_seconds += record.focused_seconds;
            task.updated_at = Utc::now().to_rfc3339();
            db_save_task(&transaction, &task)?;
            append_event(
                &transaction,
                &task,
                "focus_completed",
                serde_json::json!({ "focusedSeconds": record.focused_seconds }),
            )?;
            updated_task = Some(task);
        }
        save_focus_record(&transaction, &record)?;
        set_focus(&transaction, &None)?;
        transaction.commit().map_err(|error| error.to_string())?;
        Ok(FocusFinishResult {
            task: updated_task,
            record,
        })
    })?;
    if let Err(error) = crate::focus_overlay::set_active(&app, false) {
        crate::runtime::write_log(&app, &format!("focus overlay hide error: {error}"));
    }
    Ok(result)
}

fn safe_export_path(path: &str, allowed_extensions: &[&str]) -> Result<PathBuf, String> {
    let path = PathBuf::from(path);
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    if !allowed_extensions.contains(&extension.as_str()) {
        return Err("文件扩展名无效。".into());
    }
    Ok(path)
}

#[tauri::command]
pub fn export_backup_json(state: State<'_, Database>) -> Result<String, String> {
    with_connection(&state, |connection| {
        serde_json::to_string_pretty(&export_backup(connection)?).map_err(|error| error.to_string())
    })
}

#[tauri::command]
pub fn import_backup_json(content: String, state: State<'_, Database>) -> Result<(), String> {
    let backup: BackupDocument =
        serde_json::from_str(&content).map_err(|_| "备份文件不是有效的 TimePact JSON。")?;
    with_connection(&state, |connection| import_backup(connection, &backup))
}

#[tauri::command]
pub fn export_archive_csv(state: State<'_, Database>) -> Result<String, String> {
    let (tasks, focus_records) = with_connection(&state, |connection| {
        Ok((
            list_tasks(connection, true)?,
            list_focus_history(connection)?,
        ))
    })?;
    let mut output = String::from(
        "\u{feff}记录类型,标题或关联待办,状态,创建或开始时间,完成或结束时间,原始截止时间,延期次数,延期秒数,专注秒数,总耗时秒数\n",
    );
    for task in tasks {
        let total_seconds = task
            .completed_at
            .as_deref()
            .and_then(|completed| DateTime::parse_from_rfc3339(completed).ok())
            .and_then(|completed| {
                DateTime::parse_from_rfc3339(&task.created_at)
                    .ok()
                    .map(|created| (completed - created).num_seconds().max(0))
            })
            .unwrap_or(0);
        let columns = [
            "待办".into(),
            task.title,
            task.status,
            task.created_at,
            task.completed_at.unwrap_or_default(),
            task.timer
                .and_then(|timer| timer.original_due_at)
                .unwrap_or_default(),
            task.postponement_count.to_string(),
            task.total_postponement_seconds.to_string(),
            task.focused_seconds.to_string(),
            total_seconds.to_string(),
        ];
        output.push_str(
            &columns
                .iter()
                .map(|value| format!("\"{}\"", value.replace('"', "\"\"")))
                .collect::<Vec<_>>()
                .join(","),
        );
        output.push('\n');
    }
    for record in focus_records {
        let columns = [
            "专注".to_string(),
            record.task_id.unwrap_or_else(|| "自由专注".into()),
            "completed".into(),
            record.started_at,
            record.ended_at,
            String::new(),
            "0".into(),
            "0".into(),
            record.focused_seconds.to_string(),
            record.focused_seconds.to_string(),
        ];
        output.push_str(
            &columns
                .iter()
                .map(|value| format!("\"{}\"", value.replace('"', "\"\"")))
                .collect::<Vec<_>>()
                .join(","),
        );
        output.push('\n');
    }
    Ok(output)
}

#[tauri::command]
pub fn write_export_file(path: String, content: String) -> Result<(), String> {
    let path = safe_export_path(&path, &["json", "csv"])?;
    fs::write(path, content).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn read_import_file(path: String) -> Result<String, String> {
    let path = safe_export_path(&path, &["json"])?;
    fs::read_to_string(path).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_data_directory(app: AppHandle) -> Result<String, String> {
    database_path(&app)?
        .parent()
        .map(|path| path.to_string_lossy().into_owned())
        .ok_or_else(|| "无法定位数据目录。".into())
}

pub fn default_snapshot_name() -> String {
    let now = Utc::now();
    format!(
        "timepact-{:04}-{:02}-{:02}.json",
        now.year(),
        now.month(),
        now.day()
    )
}
