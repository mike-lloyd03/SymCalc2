use kalk::parser;
use tauri::State;

use crate::{
    error::CalcError, history_item::HistoryItem, symbols::Symbol, utils::load_context, App,
};

#[tauri::command]
pub async fn calc(state: State<'_, App>, expression: &str) -> Result<Option<f64>, CalcError> {
    log::info!("Calculating {expression}");
    let mut context = load_context(&state.db).await?;

    if let Some(declaration) = Symbol::from_string(expression)? {
        declaration.upsert(&state.db).await?;
    }

    let result = parser::eval(&mut context, expression)?.map(|r| r.to_f64());

    let history_item = HistoryItem::new(expression, result);
    history_item.create(&state.db).await?;

    Ok(result)
}

#[tauri::command]
pub async fn get_history(state: State<'_, App>) -> Result<Vec<HistoryItem>, CalcError> {
    log::info!("Getting history");
    let history = HistoryItem::get_all(&state.db).await?;
    Ok(history)
}

#[tauri::command]
pub async fn delete_history_item(state: State<'_, App>, id: i64) -> Result<(), CalcError> {
    log::info!("Deleting history item {id}");
    HistoryItem::delete_by_id(&state.db, id).await?;
    Ok(())
}
