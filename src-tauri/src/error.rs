use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "kind", content = "msg")]
#[serde(rename_all = "camelCase")]
pub enum CalcError {
    #[error("Failed to calculate the result: {0}")]
    MathError(String),
    #[error("Error from database: {0}")]
    DBError(String),
    #[error("Error from OS: {0}")]
    OSError(String),
}

impl From<sqlx::Error> for CalcError {
    fn from(err: sqlx::Error) -> Self {
        Self::DBError(err.to_string())
    }
}

impl From<kalk::errors::KalkError> for CalcError {
    fn from(err: kalk::errors::KalkError) -> Self {
        Self::MathError(err.to_string())
    }
}

impl From<sqlx::migrate::MigrateError> for CalcError {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        Self::MathError(err.to_string())
    }
}

impl From<tauri::Error> for CalcError {
    fn from(err: tauri::Error) -> Self {
        Self::OSError(err.to_string())
    }
}
