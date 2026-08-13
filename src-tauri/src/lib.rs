mod commands;
mod db;
mod focus_overlay;
mod models;
mod runtime;
mod tray;

use tauri::Manager;
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_window_state::StateFlags;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            tray::show_main(app);
        }))
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_denylist(&["reminder"])
                .with_filter(|label| !label.starts_with("focus-overlay-"))
                .with_filename(".window-state-v2.json")
                .with_state_flags(
                    StateFlags::SIZE
                        | StateFlags::POSITION
                        | StateFlags::MAXIMIZED
                        | StateFlags::FULLSCREEN,
                )
                .build(),
        )
        .plugin(
            tauri_plugin_autostart::Builder::new()
                .args(["--autostart"])
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let database = db::initialize(app.handle()).map_err(std::io::Error::other)?;
            let autostart_enabled = {
                let connection = database
                    .0
                    .lock()
                    .map_err(|_| std::io::Error::other("数据库锁不可用。"))?;
                db::get_settings(&connection)
                    .map_err(std::io::Error::other)?
                    .autostart
            };
            let focus_overlay_active = {
                let connection = database
                    .0
                    .lock()
                    .map_err(|_| std::io::Error::other("数据库锁不可用。"))?;
                let focus = db::get_focus(&connection).map_err(std::io::Error::other)?;
                focus_overlay::should_show(focus.as_ref())
            };
            app.manage(database);
            tray::configure(app.handle()).map_err(std::io::Error::other)?;
            if !cfg!(debug_assertions)
                && autostart_enabled
                && !app.autolaunch().is_enabled().unwrap_or(false)
            {
                let _ = app.autolaunch().enable();
            }
            if std::env::args().any(|argument| argument == "--autostart")
                && let Some(window) = app.get_webview_window("main")
            {
                let _ = window.hide();
            }
            runtime::spawn(app.handle().clone());
            if let Err(error) = focus_overlay::set_active(app.handle(), focus_overlay_active) {
                runtime::write_log(
                    app.handle(),
                    &format!("startup focus overlay error: {error}"),
                );
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label().starts_with("focus-overlay-") {
                    return;
                }
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_active_tasks,
            commands::list_archived_tasks,
            commands::create_task,
            commands::save_task,
            commands::complete_task,
            commands::cancel_task,
            commands::delete_task,
            commands::get_settings,
            commands::update_settings,
            commands::get_active_focus,
            commands::save_focus,
            commands::list_focus_records,
            commands::finish_focus,
            commands::export_backup_json,
            commands::import_backup_json,
            commands::export_archive_csv,
            commands::write_export_file,
            commands::read_import_file,
            commands::get_data_directory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running TimePact");
}
