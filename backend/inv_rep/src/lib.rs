pub mod model;
pub mod repos;
use anyhow::Result;
use anyhow::anyhow;

use sqlx::PgPool;

pub type DbPool = PgPool;

pub async fn create_pg_pool(database_url: String) -> Result<PgPool> {
    Ok(PgPool::connect(&database_url).await?)
}

pub fn unwrap<T>(r: Option<T>) -> Result<T> {
    r.ok_or_else(|| anyhow!("Empty option"))
}