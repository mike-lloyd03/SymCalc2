use anyhow;
use sqlx::Error;

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum CalcError {
    #[error("Failed to calculate the result")]
    MathError,
    #[error("Error from database: {msg}")]
    DBError { msg: String },
}

impl From<Error> for CalcError {
    fn from(err: Error) -> Self {
        CalcError::DBError {
            msg: err.to_string(),
        }
    }
}

impl From<anyhow::Error> for CalcError {
    fn from(err: anyhow::Error) -> Self {
        CalcError::DBError {
            msg: format!("{err:?}"),
        }
    }
}
