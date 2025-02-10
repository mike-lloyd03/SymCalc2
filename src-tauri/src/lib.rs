use async_std::task;
use history_item::HistoryItem;
use kalk::parser;
use sqlx::SqlitePool;
use tauri::{Manager, State};

mod db;
mod history_item;

struct App {
    db: SqlitePool,
}

#[tauri::command]
async fn calc(state: State<'_, App>, expression: &str) -> Result<f64, String> {
    log::info!("Calculating {expression}");
    let mut parser_context = parser::Context::new();
    let result = parser::eval(&mut parser_context, &expression)
        .map_err(|e| e.to_string())?
        .unwrap();

    let history_item = HistoryItem::new(expression, result.to_f64());
    history_item.create(&state.db).await;

    Ok(result.to_f64())
}

#[tauri::command]
async fn get_history(state: State<'_, App>) -> Result<Vec<HistoryItem>, String> {
    log::info!("Getting history");
    let history = HistoryItem::get_all(&state.db).await;
    Ok(history)
}

#[tauri::command]
async fn delete_history_item(state: State<'_, App>, id: i64) -> Result<(), String> {
    log::info!("Deleting history item {id}");
    HistoryItem::delete_by_id(&state.db, id).await;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            let database = task::block_on(db::connect(handle));
            app.manage(App { db: database });
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
