use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskTimer {
    pub kind: String,
    pub original_due_at: Option<String>,
    pub current_due_at: Option<String>,
    pub duration_seconds: Option<i64>,
    pub remaining_seconds: Option<i64>,
    pub started_at: Option<String>,
    pub locked: bool,
    pub paused_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub cycle_id: String,
    pub title: String,
    pub notes: String,
    pub priority: String,
    pub tags: Vec<String>,
    pub status: String,
    pub timer: Option<TaskTimer>,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
    pub postponement_count: i64,
    pub total_postponement_seconds: i64,
    pub focused_seconds: i64,
}

impl Task {
    pub fn is_archived(&self) -> bool {
        matches!(self.status.as_str(), "completed" | "cancelled")
    }

    pub fn due_at(&self) -> Option<&str> {
        self.timer.as_ref()?.current_due_at.as_deref()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() || self.title.chars().count() > 160 {
            return Err("标题长度必须为 1–160 个字符。".into());
        }
        if self.notes.chars().count() > 2_000 {
            return Err("备注不能超过 2000 个字符。".into());
        }
        if !matches!(self.priority.as_str(), "low" | "medium" | "high") {
            return Err("优先级无效。".into());
        }
        if let Some(timer) = &self.timer {
            if !matches!(timer.kind.as_str(), "deadline" | "countdown") {
                return Err("计时类型无效。".into());
            }
            if timer.kind == "countdown" && !matches!(timer.duration_seconds, Some(60..=359_940)) {
                return Err("倒计时必须为 1 分钟至 99 小时 59 分钟。".into());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskDraft {
    pub title: String,
    #[serde(default)]
    pub notes: String,
    #[serde(default = "default_priority")]
    pub priority: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub timer_kind: Option<String>,
    pub deadline_at: Option<String>,
    pub countdown_minutes: Option<i64>,
}

fn default_priority() -> String {
    "medium".into()
}

impl TaskDraft {
    pub fn into_task(self) -> Result<Task, String> {
        let now: DateTime<Utc> = Utc::now();
        let now_string = now.to_rfc3339();
        let timer = match self.timer_kind.as_deref() {
            None => None,
            Some("deadline") => {
                let due = self
                    .deadline_at
                    .ok_or_else(|| "Deadline 缺少截止时间。".to_string())?;
                let due_time = DateTime::parse_from_rfc3339(&due)
                    .map_err(|_| "Deadline 时间格式无效。")?
                    .with_timezone(&Utc);
                if due_time <= now {
                    return Err("Deadline 必须晚于当前时间。".into());
                }
                Some(TaskTimer {
                    kind: "deadline".into(),
                    original_due_at: Some(due_time.to_rfc3339()),
                    current_due_at: Some(due_time.to_rfc3339()),
                    duration_seconds: None,
                    remaining_seconds: None,
                    started_at: Some(now_string.clone()),
                    locked: true,
                    paused_at: None,
                })
            }
            Some("countdown") => {
                let minutes = self
                    .countdown_minutes
                    .ok_or_else(|| "倒计时缺少时长。".to_string())?;
                if !(1..=5_999).contains(&minutes) {
                    return Err("倒计时必须为 1 分钟至 99 小时 59 分钟。".into());
                }
                Some(TaskTimer {
                    kind: "countdown".into(),
                    original_due_at: None,
                    current_due_at: None,
                    duration_seconds: Some(minutes * 60),
                    remaining_seconds: Some(minutes * 60),
                    started_at: None,
                    locked: false,
                    paused_at: None,
                })
            }
            Some(_) => return Err("计时类型无效。".into()),
        };

        let task = Task {
            id: uuid::Uuid::new_v4().to_string(),
            cycle_id: uuid::Uuid::new_v4().to_string(),
            title: self.title.trim().into(),
            notes: self.notes.trim().into(),
            priority: self.priority,
            tags: self.tags,
            status: if timer.is_some() {
                "ready".into()
            } else {
                "unscheduled".into()
            },
            timer,
            created_at: now_string.clone(),
            updated_at: now_string,
            completed_at: None,
            postponement_count: 0,
            total_postponement_seconds: 0,
            focused_seconds: 0,
        };
        task.validate()?;
        Ok(task)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoundSettings {
    pub task_due: Option<String>,
    pub focus_done: Option<String>,
    pub rest_done: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub theme: String,
    pub autostart: bool,
    pub repeat_reminder_minutes: Option<i64>,
    pub focus_minutes: i64,
    pub rest_minutes: i64,
    #[serde(default = "default_workday_end_times")]
    pub workday_end_times: Vec<String>,
    pub sounds: SoundSettings,
}

fn default_workday_end_times() -> Vec<String> {
    vec!["18:00".into(); 7]
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "system".into(),
            autostart: true,
            repeat_reminder_minutes: None,
            focus_minutes: 25,
            rest_minutes: 5,
            workday_end_times: default_workday_end_times(),
            sounds: SoundSettings {
                task_due: None,
                focus_done: None,
                rest_done: None,
            },
        }
    }
}

#[cfg(test)]
mod settings_tests {
    use super::AppSettings;

    #[test]
    fn old_settings_default_each_weekday_to_six_pm() {
        let settings: AppSettings = serde_json::from_value(serde_json::json!({
            "theme": "system",
            "autostart": true,
            "repeatReminderMinutes": null,
            "focusMinutes": 30,
            "restMinutes": 10,
            "sounds": {
                "taskDue": null,
                "focusDone": null,
                "restDone": null
            }
        }))
        .expect("legacy settings should deserialize");

        assert_eq!(settings.workday_end_times, vec!["18:00"; 7]);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusSession {
    pub id: String,
    pub task_id: Option<String>,
    pub phase: String,
    pub planned_focus_seconds: i64,
    pub planned_rest_seconds: i64,
    pub remaining_seconds: i64,
    pub elapsed_seconds: i64,
    pub paused_seconds: i64,
    pub round: i64,
    pub running: bool,
    pub started_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusRecord {
    pub id: String,
    pub task_id: Option<String>,
    pub started_at: String,
    pub ended_at: String,
    pub focused_seconds: i64,
    pub rounds: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusFinishResult {
    pub task: Option<Task>,
    pub record: FocusRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupDocument {
    pub format_version: i64,
    pub exported_at: String,
    pub tasks: Vec<Task>,
    pub settings: AppSettings,
    pub focus: Option<FocusSession>,
    #[serde(default)]
    pub focus_history: Vec<FocusRecord>,
}
