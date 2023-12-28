use std::sync::{atomic::AtomicUsize, Arc, Mutex, RwLock};

use crate::error::ArithmeticError;
use kalk::parser;

#[derive(uniffi::Object)]
pub struct Calc {
    // pub history: Arc<Mutex<Vec<HistoryItem>>>,
    pub history: Arc<Mutex<Vec<String>>>,
}

#[derive(uniffi::Object)]
pub struct HistoryItem {
    equation: RwLock<String>,
    solution: RwLock<f64>,
}

#[uniffi::export]
impl HistoryItem {
    #[uniffi::constructor]
    pub fn new(equation: String, solution: f64) -> Self {
        let equation = RwLock::new(equation);
        let solution = RwLock::new(solution);

        Self { equation, solution }
    }
}

#[uniffi::export]
impl Calc {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            history: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn evaluate(&self, expr: String) -> Result<f64, ArithmeticError> {
        let mut parser_context = parser::Context::new();
        let result = parser::eval(&mut parser_context, &expr)
            .map_err(|_| ArithmeticError::Error)?
            .ok_or(ArithmeticError::Error)?;

        self.push_history(expr, result.to_f64());

        Ok(result.to_f64())
    }

    fn push_history(&self, equation: String, solution: f64) {
        // let hist_item = HistoryItem::new(equation, solution);

        self.history.lock().unwrap().push(equation);
    }

    pub fn get_history(&self) -> Vec<String> {
        let hist = self.history.lock().unwrap().clone();
        hist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_arithmetic() -> Result<(), ArithmeticError> {
        let calc = Calc::new();

        assert_eq!(calc.evaluate("2 + 2".into())?, 4.0);
        assert_eq!(calc.evaluate("2 - 2".into())?, 0.0);
        assert_eq!(calc.evaluate("2 * 2".into())?, 4.0);
        assert_eq!(calc.evaluate("2 (2)".into())?, 4.0);
        assert_eq!(calc.evaluate("2 / 2".into())?, 1.0);
        Ok(())
    }

    #[test]
    fn test_errors() -> Result<(), ArithmeticError> {
        let calc = Calc::new();

        assert!(calc.evaluate("2 x 2".into()).is_err());
        Ok(())
    }

    #[test]
    fn test_complex_arithmetic() -> Result<(), ArithmeticError> {
        let calc = Calc::new();

        assert_eq!(calc.evaluate("2 (5 + 7)".into())?, 24.0);
        Ok(())
    }

    #[test]
    fn test_history() -> Result<(), ArithmeticError> {
        let calc = Calc::new();
        calc.evaluate("2 * 7".into())?;
        calc.evaluate("sqrt(81)".into())?;
        calc.evaluate("27 / 9 + 3".into())?;

        let expect = vec![
            HistoryItem::new("2 * 7".into(), 14.0),
            HistoryItem::new("sqrt(81)".into(), 9.0),
            HistoryItem::new("27 / 9 + 3".into(), 9.0),
        ];

        let mut expect: Vec<String> = expect
            .iter()
            .map(|h| h.equation.read().unwrap().clone())
            .collect();

        assert_eq!(calc.get_history(), expect);

        calc.evaluate("1 + 1".into())?;

        expect.push("1 + 1".into());

        assert_eq!(calc.get_history(), expect);

        Ok(())
    }
}
