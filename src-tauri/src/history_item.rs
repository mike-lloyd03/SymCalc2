use serde::Serialize;
use sqlx::SqlitePool;

use crate::error::CalcError;

#[derive(Debug, Clone, Serialize)]
pub struct HistoryItem {
    id: Option<i64>,
    equation: String,
    solution: Option<f64>,
}

impl HistoryItem {
    pub fn new(equation: &str, solution: Option<f64>) -> Self {
        Self {
            id: None,
            equation: equation.to_string(),
            solution,
        }
    }

    pub async fn create(&self, pool: &SqlitePool) -> Result<(), CalcError> {
        sqlx::query!(
            "insert into history (equation, solution) values (?, ?)",
            self.equation,
            self.solution
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Self>, CalcError> {
        Ok(sqlx::query_as!(Self, "select * from history")
            .fetch_all(pool)
            .await?)
    }

    pub async fn delete_by_id(pool: &SqlitePool, id: i64) -> Result<(), CalcError> {
        sqlx::query!("delete from history where id = ?", id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

impl PartialEq for HistoryItem {
    fn eq(&self, other: &Self) -> bool {
        self.equation == other.equation && self.solution == other.solution
    }
}
