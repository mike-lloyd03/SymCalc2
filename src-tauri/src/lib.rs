use async_std::task;
use commands::{calc, delete_history_item, get_history};
use sqlx::SqlitePool;
use tauri::Manager;

mod commands;
mod db;
mod error;
mod history_item;

struct App {
    db: SqlitePool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            let database = task::block_on(db::connect(handle));
            app.manage(App {
                db: database.expect("db should load"),
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            calc,
            get_history,
            delete_history_item
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
