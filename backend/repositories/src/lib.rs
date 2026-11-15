pub mod model_inventarwerk;
pub mod model_zauberwerk;
pub mod repos;
use anyhow::Result;

use sqlx::{
    migrate::Migrator,
    pool::PoolConnection,
    postgres::PgPoolOptions,
    PgPool, Postgres, Transaction,
};

pub type DbPool = PgPool;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

/// Postgres session variable holding the UUID of the user who causes the current changes.
///
/// Set by [`UserDb`] on every connection it hands out and read by the change notification
/// trigger (`notify_inventory_change_generic`), which sends it as `user` with every change event.
pub const ACTING_USER_SETTING: &str = "app.user_id";

pub async fn create_pg_pool(database_url: String) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        // `UserDb::acquire` sets the acting user for the whole session of a connection.
        // Clear it when the connection goes back to the pool, so the next request using this
        // connection is never attributed to the previous user. If clearing fails, the
        // connection is closed instead of being reused.
        .after_release(|conn, _meta| {
            Box::pin(async move {
                sqlx::query("SELECT set_config('app.user_id', '', false)")
                    .execute(&mut *conn)
                    .await?;
                Ok(true)
            })
        })
        .connect(&database_url)
        .await?;
    MIGRATOR.run(&pool).await?;
    Ok(pool)
}

/// Database access on behalf of a user.
///
/// Every connection or transaction handed out by `UserDb` has the acting user set in the
/// Postgres session variable [`ACTING_USER_SETTING`], so the change notification trigger can
/// send who caused a change. Repositories only hold a `UserDb` and never the raw pool, so no
/// query can bypass this.
///
/// Without a user (e.g. at startup or for requests without login) the variable stays empty
/// and change events are sent with `user: null`.
#[derive(Clone)]
pub struct UserDb {
    pool: PgPool,
    user_id: Option<String>,
}

impl UserDb {
    /// Database access on behalf of the given user, or without a user if `None`.
    pub fn new(pool: PgPool, user_id: Option<String>) -> Self {
        Self { pool, user_id }
    }

    /// Database access without a user, e.g. for tasks during startup.
    pub fn system(pool: PgPool) -> Self {
        Self::new(pool, None)
    }

    /// Acquires a connection from the pool with the acting user set for its session.
    ///
    /// The user is cleared again when the connection is returned to the pool
    /// (see [`create_pg_pool`]).
    pub async fn acquire(&self) -> Result<PoolConnection<Postgres>, sqlx::Error> {
        let mut conn = self.pool.acquire().await?;
        if let Some(user_id) = &self.user_id {
            sqlx::query("SELECT set_config('app.user_id', $1, false)")
                .bind(user_id)
                .execute(&mut *conn)
                .await?;
        }
        Ok(conn)
    }

    /// Begins a transaction with the acting user set for the duration of the transaction.
    pub async fn begin(&self) -> Result<Transaction<'static, Postgres>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        if let Some(user_id) = &self.user_id {
            sqlx::query("SELECT set_config('app.user_id', $1, true)")
                .bind(user_id)
                .execute(&mut *tx)
                .await?;
        }
        Ok(tx)
    }
}

/// A repository that is built from a [`UserDb`], e.g. per request for the requesting user.
pub trait Repository {
    fn from_db(db: UserDb) -> Self;
}

/// Returns the major version of the connected PostgreSQL server (e.g. `15`).
pub async fn server_major_version(pool: &PgPool) -> Result<u32> {
    // server_version_num is e.g. "150004" for 15.4
    let version_num: String = sqlx::query_scalar("SHOW server_version_num")
        .fetch_one(pool)
        .await?;
    Ok(version_num.trim().parse::<u32>()? / 10000)
}
