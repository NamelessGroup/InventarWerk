pub mod model;
pub mod repos;
use anyhow::Result;

use sqlx::PgPool;

pub type DbPool = PgPool;

pub async fn create_pg_pool(database_url: String) -> Result<PgPool> {
    Ok(PgPool::connect(&database_url).await?)
}
