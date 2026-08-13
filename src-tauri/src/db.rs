use std::{fs, path::PathBuf, sync::Mutex};

use chrono::Utc;
use rusqlite::{Connection, OptionalExtension, params};
use tauri::{AppHandle, Manager};

use crate::models::{AppSettings, BackupDocument, FocusRecord, FocusSession, Task};

pub struct Database(pub Mutex<Connection>);

pub fn initialize(app: &AppHandle) -> Result<Database, String> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    fs::create_dir_all(&data_dir).map_err(|error| error.to_string())?;
    let path = data_dir.join("timepact.db");
    let connection = Connection::open(path).map_err(|error| error.to_string())?;
    connection
        .execute_batch(
            r#"
            PRAGMA foreign_keys = ON;
            PRAGMA journal_mode = WAL;
            CREATE TABLE IF NOT EXISTS tasks (
              id TEXT PRIMARY KEY,
              cycle_id TEXT NOT NULL,
              status TEXT NOT NULL,
              due_at TEXT,
              archived INTEGER NOT NULL DEFAULT 0,
              data TEXT NOT NULL,
              updated_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_tasks_active_due
              ON tasks(archived, status, due_at);
            CREATE TABLE IF NOT EXISTS task_events (
              id TEXT PRIMARY KEY,
              task_id TEXT NOT NULL,
              cycle_id TEXT NOT NULL,
              kind TEXT NOT NULL,
              occurred_at TEXT NOT NULL,
              payload TEXT NOT NULL DEFAULT '{}'
            );
            CREATE INDEX IF NOT EXISTS idx_task_events_cycle
              ON task_events(cycle_id, occurred_at);
            CREATE TABLE IF NOT EXISTS app_state (
              key TEXT PRIMARY KEY,
              value TEXT NOT NULL,
              updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS focus_history (
              id TEXT PRIMARY KEY,
              ended_at TEXT NOT NULL,
              data TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_focus_history_ended
              ON focus_history(ended_at);
            CREATE TABLE IF NOT EXISTS schema_migrations (
              version INTEGER PRIMARY KEY,
              applied_at TEXT NOT NULL
            );
            INSERT OR IGNORE INTO schema_migrations(version, applied_at)
              VALUES (1, datetime('now'));
            INSERT OR IGNORE INTO schema_migrations(version, applied_at)
              VALUES (2, datetime('now'));
            "#,
        )
        .map_err(|error| error.to_string())?;
    Ok(Database(Mutex::new(connection)))
}

pub fn list_tasks(connection: &Connection, archived: bool) -> Result<Vec<Task>, String> {
    let mut statement = connection
        .prepare(
            "SELECT data FROM tasks WHERE archived = ?1
             ORDER BY CASE WHEN status = 'waiting' THEN 0 ELSE 1 END, due_at, updated_at",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map([archived as i64], |row| row.get::<_, String>(0))
        .map_err(|error| error.to_string())?;
    rows.map(|row| {
        serde_json::from_str::<Task>(&row.map_err(|error| error.to_string())?)
            .map_err(|error| error.to_string())
    })
    .collect()
}

pub fn get_task(connection: &Connection, task_id: &str) -> Result<Option<Task>, String> {
    let raw = connection
        .query_row("SELECT data FROM tasks WHERE id = ?1", [task_id], |row| {
            row.get::<_, String>(0)
        })
        .optional()
        .map_err(|error| error.to_string())?;
    raw.map(|value| serde_json::from_str(&value).map_err(|error| error.to_string()))
        .transpose()
}

pub fn save_task(connection: &Connection, task: &Task) -> Result<(), String> {
    task.validate()?;
    let json = serde_json::to_string(task).map_err(|error| error.to_string())?;
    connection
        .execute(
            "INSERT INTO tasks(id, cycle_id, status, due_at, archived, data, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(id) DO UPDATE SET
               cycle_id = excluded.cycle_id,
               status = excluded.status,
               due_at = excluded.due_at,
               archived = excluded.archived,
               data = excluded.data,
               updated_at = excluded.updated_at",
            params![
                task.id,
                task.cycle_id,
                task.status,
                task.due_at(),
                task.is_archived() as i64,
                json,
                task.updated_at
            ],
        )
        .map_err(|error| error.to_string())?;
    Ok(())
}

pub fn append_event(
    connection: &Connection,
    task: &Task,
    kind: &str,
    payload: serde_json::Value,
) -> Result<(), String> {
    connection
        .execute(
            "INSERT INTO task_events(id, task_id, cycle_id, kind, occurred_at, payload)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                uuid::Uuid::new_v4().to_string(),
                task.id,
                task.cycle_id,
                kind,
                Utc::now().to_rfc3339(),
                payload.to_string()
            ],
        )
        .map_err(|error| error.to_string())?;
    Ok(())
}

fn get_state<T: serde::de::DeserializeOwned>(
    connection: &Connection,
    key: &str,
) -> Result<Option<T>, String> {
    let value = connection
        .query_row("SELECT value FROM app_state WHERE key = ?1", [key], |row| {
            row.get::<_, String>(0)
        })
        .optional()
        .map_err(|error| error.to_string())?;
    value
        .map(|raw| serde_json::from_str(&raw).map_err(|error| error.to_string()))
        .transpose()
}

fn set_state<T: serde::Serialize>(
    connection: &Connection,
    key: &str,
    value: &T,
) -> Result<(), String> {
    connection
        .execute(
            "INSERT INTO app_state(key, value, updated_at) VALUES (?1, ?2, ?3)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
            params![
                key,
                serde_json::to_string(value).map_err(|error| error.to_string())?,
                Utc::now().to_rfc3339()
            ],
        )
        .map_err(|error| error.to_string())?;
    Ok(())
}

pub fn get_settings(connection: &Connection) -> Result<AppSettings, String> {
    Ok(get_state(connection, "settings")?.unwrap_or_default())
}

pub fn set_settings(connection: &Connection, settings: &AppSettings) -> Result<(), String> {
    set_state(connection, "settings", settings)
}

pub fn get_focus(connection: &Connection) -> Result<Option<FocusSession>, String> {
    Ok(get_state::<Option<FocusSession>>(connection, "focus")?.flatten())
}

pub fn set_focus(connection: &Connection, focus: &Option<FocusSession>) -> Result<(), String> {
    set_state(connection, "focus", focus)
}

pub fn list_focus_history(connection: &Connection) -> Result<Vec<FocusRecord>, String> {
    let mut statement = connection
        .prepare("SELECT data FROM focus_history ORDER BY ended_at DESC")
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|error| error.to_string())?;
    rows.map(|row| {
        serde_json::from_str::<FocusRecord>(&row.map_err(|error| error.to_string())?)
            .map_err(|error| error.to_string())
    })
    .collect()
}

pub fn get_focus_record(
    connection: &Connection,
    record_id: &str,
) -> Result<Option<FocusRecord>, String> {
    let raw = connection
        .query_row(
            "SELECT data FROM focus_history WHERE id = ?1",
            [record_id],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|error| error.to_string())?;
    raw.map(|value| serde_json::from_str(&value).map_err(|error| error.to_string()))
        .transpose()
}

pub fn save_focus_record(connection: &Connection, record: &FocusRecord) -> Result<(), String> {
    connection
        .execute(
            "INSERT INTO focus_history(id, ended_at, data) VALUES (?1, ?2, ?3)
             ON CONFLICT(id) DO UPDATE SET ended_at = excluded.ended_at, data = excluded.data",
            params![
                record.id,
                record.ended_at,
                serde_json::to_string(record).map_err(|error| error.to_string())?
            ],
        )
        .map_err(|error| error.to_string())?;
    Ok(())
}

pub fn pause_running_state(connection: &Connection) -> Result<(), String> {
    let now = Utc::now();
    for mut task in list_tasks(connection, false)? {
        let Some(timer) = task.timer.as_mut() else {
            continue;
        };
        if task.status == "running" && timer.kind == "countdown" {
            timer.remaining_seconds = timer
                .current_due_at
                .as_deref()
                .and_then(|value| chrono::DateTime::parse_from_rfc3339(value).ok())
                .map(|due| (due.with_timezone(&Utc) - now).num_seconds().max(0))
                .or(timer.remaining_seconds);
            timer.current_due_at = None;
            timer.paused_at = Some(now.to_rfc3339());
            task.status = "paused".into();
            task.updated_at = now.to_rfc3339();
            save_task(connection, &task)?;
            append_event(connection, &task, "exit_paused", serde_json::json!({}))?;
        }
    }
    if let Some(mut focus) = get_focus(connection)?
        && focus.running
    {
        focus.running = false;
        set_focus(connection, &Some(focus))?;
    }
    Ok(())
}

pub fn export_backup(connection: &Connection) -> Result<BackupDocument, String> {
    let mut tasks = list_tasks(connection, false)?;
    tasks.extend(list_tasks(connection, true)?);
    Ok(BackupDocument {
        format_version: 1,
        exported_at: Utc::now().to_rfc3339(),
        tasks,
        settings: get_settings(connection)?,
        focus: get_focus(connection)?,
        focus_history: list_focus_history(connection)?,
    })
}

pub fn import_backup(connection: &Connection, backup: &BackupDocument) -> Result<(), String> {
    if backup.format_version != 1 {
        return Err("不支持这个备份版本。".into());
    }
    for task in &backup.tasks {
        task.validate()?;
    }

    connection
        .execute_batch("BEGIN IMMEDIATE;")
        .map_err(|error| error.to_string())?;
    let result = (|| {
        connection
            .execute("DELETE FROM task_events", [])
            .map_err(|error| error.to_string())?;
        connection
            .execute("DELETE FROM tasks", [])
            .map_err(|error| error.to_string())?;
        connection
            .execute("DELETE FROM focus_history", [])
            .map_err(|error| error.to_string())?;
        for task in &backup.tasks {
            save_task(connection, task)?;
            append_event(
                connection,
                task,
                "restored",
                serde_json::json!({ "backupExportedAt": backup.exported_at }),
            )?;
        }
        set_settings(connection, &backup.settings)?;
        set_focus(connection, &backup.focus)?;
        for record in &backup.focus_history {
            save_focus_record(connection, record)?;
        }
        Ok::<(), String>(())
    })();

    match result {
        Ok(()) => connection
            .execute_batch("COMMIT;")
            .map_err(|error| error.to_string()),
        Err(error) => {
            let _ = connection.execute_batch("ROLLBACK;");
            Err(error)
        }
    }
}

pub fn database_path(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map(|path| path.join("timepact.db"))
        .map_err(|error| error.to_string())
}

#[cfg(test)]
mod focus_state_tests {
    use super::{get_focus, set_focus};
    use rusqlite::Connection;

    #[test]
    fn stored_json_null_means_no_active_focus() {
        let connection = Connection::open_in_memory().expect("open in-memory database");
        connection
            .execute_batch(
                "CREATE TABLE app_state (
                    key TEXT PRIMARY KEY NOT NULL,
                    value TEXT NOT NULL,
                    updated_at TEXT NOT NULL
                );",
            )
            .expect("create app_state table");

        set_focus(&connection, &None).expect("persist empty focus");

        assert!(get_focus(&connection).expect("read empty focus").is_none());
    }
}
