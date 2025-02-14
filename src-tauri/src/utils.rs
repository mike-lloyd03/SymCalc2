use kalk::parser::{self, Context};
use regex::Regex;
use sqlx::SqlitePool;

use crate::{error::CalcError, symbols::Symbol};

// Loads the symbol table from the db and adds it into the new context
pub async fn load_context(pool: &SqlitePool) -> Result<Context, CalcError> {
    let mut context = parser::Context::new();
    let symbol_table = Symbol::get_all_as_string(pool).await?;
    parser::eval(&mut context, &symbol_table)?;

    Ok(context)
}
