use crate::{db, error::CalcError, history_item::HistoryItem};
use async_std::task;
use kalk::parser;
use sqlx::SqlitePool;
use uniffi::deps::log::info;

#[derive(uniffi::Object)]
pub struct Calc {
    db: SqlitePool,
}

#[uniffi::export]
impl Calc {
    #[uniffi::constructor]
    /// Create a new Calc instance with the database stored at the data directory `data_dir`
    pub fn new(data_dir: &str) -> Result<Self, CalcError> {
        info!("Creating Calc instance");
        let db_path = format!("sqlite://{}/data.db", data_dir.trim_end_matches('/'));
        let db = task::block_on(db::connect(&db_path))?;

        Ok(Self { db })
    }

    /// Evaluates the expression, pushes the result into the history, and returns it
    pub fn evaluate(&self, expr: String) -> Result<f64, CalcError> {
        info!("Evaluating {expr}");
        let mut parser_context = parser::Context::new();
        let result = parser::eval(&mut parser_context, &expr)
            .map_err(|_| CalcError::MathError)?
            .ok_or(CalcError::MathError)?;

        self.push_history(expr, result.to_f64())?;

        Ok(result.to_f64())
    }

    /// Add a new history item
    fn push_history(&self, equation: String, solution: f64) -> Result<(), CalcError> {
        info!("Pushing history item {equation} = {}", solution.to_string());
        let hist_item = HistoryItem::new(equation, solution);
        task::block_on(hist_item.create(&self.db))
    }

    /// Gets all history items
    pub fn get_history(&self) -> Result<Vec<HistoryItem>, CalcError> {
        info!("Getting all history");
        task::block_on(HistoryItem::get_all(&self.db))
    }

    /// Delete the history item identified by `hist_id`
    pub fn delete_history(&self, hist_id: i64) -> Result<(), CalcError> {
        info!("Deleting history item {hist_id}");
        task::block_on(HistoryItem::delete_by_id(&self.db, hist_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_calc(pool: SqlitePool) -> Calc {
        Calc { db: pool }
    }

    #[sqlx::test]
    fn test_simple_arithmetic(pool: SqlitePool) -> Result<(), CalcError> {
        let calc = test_calc(pool);

        assert_eq!(calc.evaluate("2 + 2".into())?, 4.0);
        assert_eq!(calc.evaluate("2 - 2".into())?, 0.0);
        assert_eq!(calc.evaluate("2 * 2".into())?, 4.0);
        assert_eq!(calc.evaluate("2 (2)".into())?, 4.0);
        assert_eq!(calc.evaluate("2 / 2".into())?, 1.0);
        Ok(())
    }

    #[sqlx::test]
    fn test_errors(pool: SqlitePool) -> Result<(), CalcError> {
        let calc = test_calc(pool);

        assert!(calc.evaluate("2 x 2".into()).is_err());
        Ok(())
    }

    #[sqlx::test]
    fn test_complex_arithmetic(pool: SqlitePool) -> Result<(), CalcError> {
        let calc = test_calc(pool);

        assert_eq!(calc.evaluate("2 (5 + 7)".into())?, 24.0);
        Ok(())
    }

    #[sqlx::test]
    fn test_history(pool: SqlitePool) -> Result<(), CalcError> {
        let calc = test_calc(pool);

        calc.evaluate("2 * 7".into())?;
        calc.evaluate("sqrt(81)".into())?;
        calc.evaluate("27 / 9 + 3".into())?;

        let expect = vec![
            HistoryItem::new("2 * 7".into(), 14.0),
            HistoryItem::new("sqrt(81)".into(), 9.0),
            HistoryItem::new("27 / 9 + 3".into(), 6.0),
        ];

        assert_eq!(calc.get_history()?, expect);

        calc.evaluate("1 + 1".into())?;

        Ok(())
    }

    #[sqlx::test]
    fn test_history_delete(pool: SqlitePool) -> Result<(), CalcError> {
        let calc = test_calc(pool);

        calc.evaluate("2 * 7".into())?;

        assert_eq!(calc.get_history()?.len(), 1);

        calc.delete_history(1)?;

        assert_eq!(calc.get_history()?.len(), 0);

        Ok(())
    }
}
