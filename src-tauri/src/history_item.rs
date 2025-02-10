use serde::Serialize;
use sqlx::SqlitePool;

#[derive(Debug, Clone, Serialize)]
pub struct HistoryItem {
    id: Option<i64>,
    equation: String,
    solution: f64,
}

impl HistoryItem {}

impl HistoryItem {
    pub fn new(equation: &str, solution: f64) -> Self {
        Self {
            id: None,
            equation: equation.to_string(),
            solution,
        }
    }

    pub async fn create(&self, pool: &SqlitePool) {
        sqlx::query!(
            "insert into history (equation, solution) values (?, ?)",
            self.equation,
            self.solution
        )
        .execute(pool)
        .await
        .unwrap();
    }

    pub async fn get_all(pool: &SqlitePool) -> Vec<Self> {
        sqlx::query_as!(Self, "select * from history")
            .fetch_all(pool)
            .await
            .unwrap()
    }

    pub async fn delete(&self, pool: &SqlitePool) {
        sqlx::query!("delete from history where id = ?", self.id)
            .execute(pool)
            .await
            .unwrap();
    }

    pub async fn delete_by_id(pool: &SqlitePool, id: i64) {
        sqlx::query!("delete from history where id = ?", id)
            .execute(pool)
            .await
            .unwrap();
    }
}

impl PartialEq for HistoryItem {
    fn eq(&self, other: &Self) -> bool {
        self.equation == other.equation && self.solution == other.solution
    }
}
