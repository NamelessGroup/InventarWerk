use sqlx::PgPool;
use crate::{model::User, unwrap};
use anyhow::Result;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, uuid: &str, name: &str, avatar: &str) -> Result<User> {
        let dm: i32 = if self.dm_exists().await? { 0 } else { 1 };
        let user = sqlx::query_as!(User,
            "INSERT INTO \"user\" (uuid, name, avatar, dm) VALUES ($1, $2, $3, $4) RETURNING *",
            uuid, name, avatar, dm
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(user)
    }

    pub async fn get_user(&self, uuid: &str) -> Result<User> {
        let user = sqlx::query_as!(User, "SELECT * FROM \"user\" WHERE uuid = $1", uuid)
            .fetch_one(&self.pool)
            .await?;
        
        Ok(user)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as!(User, "SELECT * FROM \"user\"")
            .fetch_all(&self.pool)
            .await?;
        
        Ok(users)
    }

    pub async fn delete_user(&self, uuid: &str) -> Result<u64> {
        let result = sqlx::query!("DELETE FROM \"user\" WHERE uuid = $1", uuid)
            .execute(&self.pool)
            .await?;
        
        Ok(result.rows_affected())
    }

    pub async fn update_user(&self, uuid: &str, name: &str, avatar: &str, dm: i32) -> Result<User> {
        let user = sqlx::query_as!(User,
            "UPDATE \"user\" SET name = $1, avatar = $2, dm = $3 WHERE uuid = $4 RETURNING *",
            name, avatar, dm, uuid
        )
        .fetch_optional(&self.pool)
        .await?;
        
        unwrap(user)
    }

    pub async fn user_exists(&self, uuid: &str) -> Result<bool> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM \"user\" WHERE uuid = $1)",
            uuid
        )
        .fetch_one(&self.pool)
        .await?;
        
        unwrap(exists)
    }

    pub async fn dm_exists(&self) -> Result<bool> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM \"user\" WHERE dm = 1)"
        )
        .fetch_one(&self.pool)
        .await?;
        
        return unwrap(exists);
    }

    pub async fn any_user_exists(&self) -> Result<bool> {
        let result = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM \"user\") AS exists"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.exists.unwrap_or(false))
    }
}
