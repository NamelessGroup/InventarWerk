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

/// Returns the major version of the connected PostgreSQL server (e.g. `15`).
pub async fn server_major_version(pool: &PgPool) -> Result<u32> {
    // server_version_num is e.g. "150004" for 15.4
    let version_num: String = sqlx::query_scalar("SHOW server_version_num")
        .fetch_one(pool)
        .await?;
    Ok(version_num.trim().parse::<u32>()? / 10000)
}
