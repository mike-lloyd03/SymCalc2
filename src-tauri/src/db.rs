use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tauri::Manager;

use crate::error::CalcError;

async fn create_db(db_path: &str) -> Result<SqlitePool, CalcError> {
    Sqlite::create_database(db_path).await?;

    Ok(SqlitePool::connect(db_path).await?)
}

pub async fn connect(app: tauri::AppHandle) -> Result<SqlitePool, CalcError> {
    let data_dir = app.path().data_dir()?;
    let db_path = data_dir.join("symcalc2.db");
    let db_path = db_path.to_string_lossy();

    let db = if !Sqlite::database_exists(&db_path).await? {
        create_db(&db_path).await?
    } else {
        SqlitePool::connect(&db_path).await?
    };

    sqlx::migrate!().run(&db).await?;

    Ok(db)
}
