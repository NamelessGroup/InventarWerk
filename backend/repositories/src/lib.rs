pub mod model_inventarwerk;
pub mod model_zauberwerk;
pub mod repos;
use anyhow::Result;

use sqlx::{migrate::Migrator, PgPool};

pub type DbPool = PgPool;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn create_pg_pool(database_url: String) -> Result<PgPool> {
    let pool = PgPool::connect(&database_url).await?;
    MIGRATOR.run(&pool).await?;
    Ok(pool)
}
