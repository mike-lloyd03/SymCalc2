use std::convert::Into;

use crate::{db, error::CalcError};
use async_std::task;
use kalk::parser;
use uniffi::deps::{anyhow::Context, log::info};

#[derive(uniffi::Object)]
pub struct Calc {
    db_path: String,
}

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct HistoryItem {
    id: Option<i64>,
    equation: String,
    solution: f64,
}

impl HistoryItem {
    fn new(equation: String, solution: f64) -> Self {
        Self {
            id: None,
            equation,
            solution,
        }
    }

    async fn create(&self, db_path: &str) -> Result<(), CalcError> {
        let db = db::connect(db_path).await?;

        Ok(sqlx::query!(
            "insert into history (equation, solution) values (?, ?)",
            self.equation,
            self.solution
        )
        .execute(&db)
        .await
        .map(|_| ())?)
    }

    async fn get_all(db_path: &str) -> Result<Vec<Self>, CalcError> {
        let db = db::connect(db_path).await?;

        Ok(sqlx::query_as!(Self, "select * from history")
            .fetch_all(&db)
            .await
            .context("Failed to get all history.")?)
    }
}

#[uniffi::export]
impl Calc {
    #[allow(clippy::new_without_default)]
    #[uniffi::constructor]
    pub fn new(data_path: &str) -> Self {
        info!("Creating Calc instance");
        let db_path = format!("sqlite://{}/data.db", data_path.trim_end_matches('/'));
        println!("{db_path}");
        Self { db_path }
    }

    pub fn evaluate(&self, expr: String) -> Result<f64, CalcError> {
        info!("Evaluating {expr}");
        let mut parser_context = parser::Context::new();
        let result = parser::eval(&mut parser_context, &expr)
            .map_err(|_| CalcError::MathError)?
            .ok_or(CalcError::MathError)?;

        self.push_history(expr, result.to_f64())?;

        Ok(result.to_f64())
    }

    fn push_history(&self, equation: String, solution: f64) -> Result<(), CalcError> {
        info!("Pushing history item {equation} = {}", solution.to_string());
        let hist_item = HistoryItem::new(equation, solution);
        task::block_on(hist_item.create(&self.db_path))
    }

    pub fn get_history(&self) -> Result<Vec<HistoryItem>, CalcError> {
        info!("Getting all history");
        task::block_on(HistoryItem::get_all(&self.db_path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_PATH: &str = "./test";

    #[sqlx::test]
    fn test_simple_arithmetic() -> Result<(), CalcError> {
        let calc = Calc::new(DATA_PATH);

        assert_eq!(calc.evaluate("2 + 2".into())?, 4.0);
        assert_eq!(calc.evaluate("2 - 2".into())?, 0.0);
        assert_eq!(calc.evaluate("2 * 2".into())?, 4.0);
        assert_eq!(calc.evaluate("2 (2)".into())?, 4.0);
        assert_eq!(calc.evaluate("2 / 2".into())?, 1.0);
        Ok(())
    }

    #[sqlx::test]
    fn test_errors() -> Result<(), CalcError> {
        let calc = Calc::new(DATA_PATH);

        assert!(calc.evaluate("2 x 2".into()).is_err());
        Ok(())
    }

    #[sqlx::test]
    fn test_complex_arithmetic() -> Result<(), CalcError> {
        let calc = Calc::new(DATA_PATH);

        assert_eq!(calc.evaluate("2 (5 + 7)".into())?, 24.0);
        Ok(())
    }

    #[sqlx::test]
    fn test_history() -> Result<(), CalcError> {
        let calc = Calc::new(DATA_PATH);

        let db = db::connect(&calc.db_path).await?;
        sqlx::query!("delete from history").execute(&db).await?;

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
}
