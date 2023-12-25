use error::ArithmeticError;
use kalk::parser;

mod error;

uniffi::setup_scaffolding!();

type Result<T, E = ArithmeticError> = std::result::Result<T, E>;

#[uniffi::export]
pub fn evaluate(expr: String) -> Result<f64> {
    let mut parser_context = parser::Context::new();
    let precision = 53;
    let result = parser::eval(&mut parser_context, &expr, precision)
        .map_err(|_| ArithmeticError::Error)?
        .ok_or(ArithmeticError::Error)?;

    Ok(result.to_f64())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_arithmetic() -> Result<()> {
        assert_eq!(evaluate("2 + 2".into())?, 4.0);
        assert_eq!(evaluate("2 - 2".into())?, 0.0);
        assert_eq!(evaluate("2 * 2".into())?, 4.0);
        assert_eq!(evaluate("2 (2)".into())?, 4.0);
        assert_eq!(evaluate("2 / 2".into())?, 1.0);
        Ok(())
    }

    #[test]
    fn test_errors() -> Result<()> {
        assert!(evaluate("2 x 2".into()).is_err());
        Ok(())
    }

    #[test]
    fn test_complex_arithmetic() -> Result<()> {
        assert_eq!(evaluate("2 (5 + 7)".into())?, 24.0);
        Ok(())
    }
}
