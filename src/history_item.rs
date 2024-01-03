use sqlx::SqlitePool;
use uniffi::deps::anyhow::Context;

use crate::error::CalcError;

#[derive(Debug, Clone, uniffi::Record)]
pub struct HistoryItem {
    id: Option<i64>,
    equation: String,
    solution: f64,
}

impl HistoryItem {
    pub fn new(equation: String, solution: f64) -> Self {
        Self {
            id: None,
            equation,
            solution,
        }
    }

    pub async fn create(&self, pool: &SqlitePool) -> Result<(), CalcError> {
        Ok(sqlx::query!(
            "insert into history (equation, solution) values (?, ?)",
            self.equation,
            self.solution
        )
        .execute(pool)
        .await
        .map(|_| ())?)
    }

    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Self>, CalcError> {
        Ok(sqlx::query_as!(Self, "select * from history")
            .fetch_all(pool)
            .await
            .context("Failed to get all history.")?)
    }

    pub async fn delete(&self, pool: &SqlitePool) -> Result<(), CalcError> {
        Ok(sqlx::query!("delete from history where id = ?", self.id)
            .execute(pool)
            .await
            .map(|_| ())?)
    }
}

impl PartialEq for HistoryItem {
    fn eq(&self, other: &Self) -> bool {
        self.equation == other.equation && self.solution == other.solution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_create(pool: SqlitePool) -> Result<(), CalcError> {
        let new_hist = HistoryItem::new("1+1".into(), 2.0);
        assert!(new_hist.create(&pool).await.is_ok());

        Ok(())
    }

    #[sqlx::test]
    async fn test_delete(pool: SqlitePool) -> Result<(), CalcError> {
        let new_hist = HistoryItem::new("1+1".into(), 2.0);
        new_hist.create(&pool).await?;
        let hist = HistoryItem::get_all(&pool).await?;

        assert!(hist.first().unwrap().delete(&pool).await.is_ok());

        Ok(())
    }
}
