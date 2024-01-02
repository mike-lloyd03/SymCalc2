use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use uniffi::deps::{anyhow::Context, log::info};

use crate::error::CalcError;

async fn create_db(db_path: &str) -> Result<SqlitePool, CalcError> {
    info!("Creating database");
    Sqlite::create_database(db_path)
        .await
        .context(format!("Failed to create database at {db_path}"))?;
    let db = SqlitePool::connect(db_path)
        .await
        .context(format!("Failed to connect to database at {db_path}"))?;
    sqlx::migrate!()
        .run(&db)
        .await
        .context("Failed to run migrations")?;
    Ok(db)
}

pub async fn connect(db_path: &str) -> Result<SqlitePool, CalcError> {
    if !Sqlite::database_exists(db_path).await? {
        return create_db(db_path).await;
    }

    info!("Connecting to database");
    Ok(SqlitePool::connect(db_path)
        .await
        .context(format!("Failed to connect to database at {db_path}"))?)
}
