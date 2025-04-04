use sqlx::{PgPool, Error};
use crate::model::ItemPreset;
use anyhow::Result;
use uuid::Uuid;

pub struct ItemPresetRepository {
    pool: PgPool,
}

impl ItemPresetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_from_name(&self, name: &str, owner: &str) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        sqlx::query!(
            "INSERT INTO item_preset (uuid, name, creator) VALUES ($1, $2, $3)",
            id, name, owner
        )
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    pub async fn create(&self, item: &ItemPreset) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO item_preset (uuid, name, price, weight, description, creator, item_type, creation)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            item.uuid, item.name, item.price, item.weight, item.description, item.creator, item.item_type, item.creation
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_by_uuid(&self, uuid: &str) -> Result<ItemPreset, Error> {
        let item = sqlx::query_as!(ItemPreset,
            "SELECT * FROM item_preset WHERE uuid = $1",
            uuid
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(item)
    }

    pub async fn update(&self, item: &ItemPreset) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE item_preset SET name = $1, price = $2, weight = $3, description = $4, creator = $5, item_type = $6 WHERE uuid = $7",
            item.name, item.price, item.weight, item.description, item.creator, item.item_type, item.uuid
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, uuid: &str) -> Result<(), Error> {
        sqlx::query!("DELETE FROM item_preset WHERE uuid = $1", uuid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_public_presets(&self) -> Result<Vec<ItemPreset>, Error> {
        let presets = sqlx::query_as!(ItemPreset,
            "SELECT * FROM item_preset WHERE creator LIKE 'public%'")
            .fetch_all(&self.pool)
            .await?;
        Ok(presets)
    }
}