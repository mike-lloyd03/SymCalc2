use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tauri::Manager;

async fn create_db(db_path: &str) -> SqlitePool {
    Sqlite::create_database(db_path).await.unwrap();

    let db = SqlitePool::connect(db_path).await.unwrap();

    sqlx::migrate!().run(&db).await.unwrap();

    db
}

pub async fn connect(app: tauri::AppHandle) -> SqlitePool {
    let data_dir = app.path().data_dir().unwrap();
    let db_path = data_dir.join("symcalc2.db");
    let db_path = db_path.to_string_lossy();

    if !Sqlite::database_exists(&db_path).await.unwrap() {
        return create_db(&db_path).await;
    }

    SqlitePool::connect(&db_path).await.unwrap()
}
