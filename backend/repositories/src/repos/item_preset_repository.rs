use crate::model_inventarwerk::ItemPreset;
use anyhow::Result;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct ItemPresetRepository {
    pool: PgPool,
}

impl ItemPresetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Creates a new item preset with the given name and owner, using default values for other fields.
    pub async fn create_from_name(&self, name: &str, owner: &str) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        sqlx::query!(
            "INSERT INTO item_preset (uuid, name, price, weight, description, creator, item_type) 
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            id,
            name,
            0,
            0.0,
            "",
            owner,
            ""
        )
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    /// Creates a new item preset from an `ItemPreset` struct.
    pub async fn create(&self, item: &ItemPreset) -> Result<(), Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query!(
            "INSERT INTO item_preset (uuid, name, price, weight, description, creator, item_type, creation)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            id, item.name, item.price, item.weight, item.description, item.creator, item.item_type, item.creation
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Retrieves an item preset by its UUID.
    pub async fn get_by_uuid(&self, uuid: &str) -> Result<ItemPreset, Error> {
        let item = sqlx::query_as!(
            ItemPreset,
            "SELECT * FROM item_preset WHERE uuid = $1",
            uuid
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(item)
    }

    /// Updates an item preset's fields by UUID. Only non-`None` fields are updated.
    pub async fn update_item_preset(
        &self,
        uuid: &str,
        name: Option<&str>,
        price: Option<i32>,
        weight: Option<f32>,
        description: Option<&str>,
        item_type: Option<&str>,
    ) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE item_preset SET 
                name = COALESCE($1, name), 
                price = COALESCE($2, price), 
                weight = COALESCE($3, weight), 
                description = COALESCE($4, description), 
                item_type = COALESCE($5, item_type) 
             WHERE uuid = $6",
            name,
            price,
            weight,
            description,
            item_type,
            uuid
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Deletes an item preset by UUID.
    pub async fn delete(&self, uuid: &str) -> Result<(), Error> {
        sqlx::query!("DELETE FROM item_preset WHERE uuid = $1", uuid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Retrieves all public item presets (where the creator starts with "public").
    pub async fn get_public_presets(&self) -> Result<Vec<ItemPreset>, Error> {
        let presets = sqlx::query_as!(
            ItemPreset,
            "SELECT * FROM item_preset WHERE creator LIKE 'public%'"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(presets)
    }

    /// Retrieves all item presets present in a specific inventory.
    pub async fn get_presets_in_inventory(&self, inventory_uuid: &str) -> Result<Vec<ItemPreset>> {
        let presets = sqlx::query_as!(
            ItemPreset,
            "SELECT ip.uuid, ip.name, ip.price, ip.weight, ip.description, ip.creator, ip.item_type, ip.creation
             FROM item_preset ip
             INNER JOIN inventory_item ii ON ip.uuid = ii.item_preset_uuid
             WHERE ii.inventory_uuid = $1",
            inventory_uuid
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(presets)
    }
}
