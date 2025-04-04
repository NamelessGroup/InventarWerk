pub mod model;
pub mod repos;
use anyhow::Result;
use anyhow::anyhow;

use sqlx::PgPool;
use tokio::runtime::Runtime;

pub type DbPool = PgPool;

pub fn create_pg_pool(database_url: String) -> Result<PgPool> {
    let runtime = Runtime::new()?;

    Ok(runtime.block_on(PgPool::connect(&database_url))?)
}

pub fn unwrap<T>(r: Option<T>) -> Result<T> {
    r.ok_or_else(|| anyhow!("Empty option"))
}