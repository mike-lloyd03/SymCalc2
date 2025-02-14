use regex::Regex;
use serde::Serialize;
use sqlx::SqlitePool;

use crate::error::CalcError;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Symbol {
    identifier: String,
    value: String,
}

impl Symbol {
    pub fn new(identifier: &str, value: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
            value: value.to_string(),
        }
    }

    // Parses a Symbol from an expression. Returns `None` if the string is not a
    // declaration;
    pub fn from_string(expression: &str) -> Result<Option<Self>, CalcError> {
        let re = Regex::new(r"^[a-zA-Z]+(_\d+)*$").unwrap();

        match expression.split_once("=") {
            Some((_, "")) | None => Ok(None),

            Some((identifier, value)) => {
                let identifier = identifier.trim_ascii();
                let value = value.trim_ascii();

                if re.is_match(identifier) {
                    Ok(Some(Symbol::new(identifier, value)))
                } else {
                    Err(CalcError::MathError(
                        "Declaration identifier is invalid".into(),
                    ))
                }
            }
        }
    }

    pub async fn get(pool: &SqlitePool, identifier: &str) -> Result<Self, CalcError> {
        Ok(sqlx::query_as!(
            Self,
            "SELECT * FROM symbols WHERE identifier = ?",
            identifier,
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Self>, CalcError> {
        Ok(sqlx::query_as!(Self, "SELECT * FROM symbols")
            .fetch_all(pool)
            .await?)
    }

    pub async fn get_all_as_string(pool: &SqlitePool) -> Result<String, CalcError> {
        let symbols = Self::get_all(pool).await?;
        let s: Vec<String> = symbols
            .into_iter()
            .map(|s| format!("{}={};", s.identifier, s.value))
            .collect();

        Ok(s.join("\n"))
    }

    pub async fn upsert(&self, pool: &SqlitePool) -> Result<(), CalcError> {
        sqlx::query!(
            "INSERT INTO symbols (identifier, value) VALUES(?, ?)
              ON CONFLICT(identifier) DO UPDATE SET value = ? where identifier = ?;",
            self.identifier,
            self.value,
            self.value,
            self.identifier,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete(pool: &SqlitePool, identifier: &str) -> Result<(), CalcError> {
        sqlx::query!("DELETE FROM symbols WHERE identifier = ?", identifier,)
            .execute(pool)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use kalk::parser::{eval, Context};

    use super::*;

    #[sqlx::test]
    async fn test_upsert(pool: SqlitePool) -> Result<(), CalcError> {
        let new_sym = Symbol::new("x", "3");
        assert!(new_sym.upsert(&pool).await.is_ok());

        let fetched_sym = Symbol::get(&pool, "x").await?;
        assert_eq!(fetched_sym.value, "3");

        let next_sym = Symbol::new("x", "9");
        assert!(next_sym.upsert(&pool).await.is_ok());

        let fetched_sym = Symbol::get(&pool, "x").await?;
        assert_eq!(fetched_sym.value, "9");

        Ok(())
    }

    #[sqlx::test]
    async fn test_delete(pool: SqlitePool) -> Result<(), CalcError> {
        let new_sym = Symbol::new("x", "3");
        new_sym.upsert(&pool).await?;

        let new_sym = Symbol::new("y", "9");
        new_sym.upsert(&pool).await?;

        let all_symbols = Symbol::get_all(&pool).await?;
        assert_eq!(2, all_symbols.len());

        assert!(Symbol::delete(&pool, "x").await.is_ok());

        let all_symbols = Symbol::get_all(&pool).await?;
        assert_eq!(1, all_symbols.len());

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_all_as_str(pool: SqlitePool) -> Result<(), CalcError> {
        let expect = "x=3;\ny=9;";

        let new_sym = Symbol::new("x", "3");
        new_sym.upsert(&pool).await?;

        let new_sym = Symbol::new("y", "9");
        new_sym.upsert(&pool).await?;

        let symbol_table = Symbol::get_all_as_string(&pool).await?;
        assert_eq!(expect, symbol_table);

        // Test that we can use the resulting string in kalk's parser context
        let mut context = Context::new();
        eval(&mut context, &symbol_table)?;

        let result = eval(&mut context, "x + 5")?;
        assert_eq!(Some(8.0), result.map(|r| r.to_f64()));

        let result = eval(&mut context, "y - 2")?;
        assert_eq!(Some(7.0), result.map(|r| r.to_f64()));

        Ok(())
    }

    #[test]
    fn test_parse_declaration() -> Result<(), CalcError> {
        let expression = "x=9";
        let expect = Some(Symbol::new("x", "9"));
        let got = Symbol::from_string(expression)?;

        assert_eq!(expect, got);
        Ok(())
    }

    #[test]
    fn test_parse_declaration_with_spaces() -> Result<(), CalcError> {
        let expression = "x = 9";
        let expect = Some(Symbol::new("x", "9"));
        let got = Symbol::from_string(expression)?;

        assert_eq!(expect, got);
        Ok(())
    }

    #[test]
    fn test_parse_invalid_declaration() -> Result<(), CalcError> {
        let expression = "5+7";
        let expect = None;
        let got = Symbol::from_string(expression)?;

        assert_eq!(expect, got);
        Ok(())
    }

    #[test]
    fn test_parse_mixed_char_symbol() -> Result<(), CalcError> {
        let expression = "x_2=4";
        let expect = Some(Symbol::new("x_2", "4"));
        let got = Symbol::from_string(expression)?;

        assert_eq!(expect, got);
        Ok(())
    }
}
