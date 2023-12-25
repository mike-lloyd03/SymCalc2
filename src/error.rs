#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum ArithmeticError {
    #[error("Failed to calculate the result")]
    Error,
}
