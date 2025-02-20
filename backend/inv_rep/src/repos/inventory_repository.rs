use sqlx::{PgPool, Error};
use uuid::Uuid;
use crate::model::{FullInventory, RawInventory};

pub struct InventoryRepository {
    pool: PgPool
}

impl InventoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_readers(&self, inventory_uuid: &str) -> Result<Vec<String>, Error> {
        let readers = sqlx::query!("SELECT user_uuid FROM inventory_reader WHERE inventory_uuid = $1", inventory_uuid)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|r| r.user_uuid)
            .collect();
        Ok(readers)
    }

    pub async fn get_writers(&self, inventory_uuid: &str) -> Result<Vec<String>, Error> {
        let writers = sqlx::query!("SELECT user_uuid FROM inventory_writer WHERE inventory_uuid = $1", inventory_uuid)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|w| w.user_uuid)
            .collect();
        Ok(writers)
    }

    pub async fn get_full_inventory(&self, uuid: &str) -> Result<Option<FullInventory>, Error> {
        let inventory = sqlx::query!(
            "SELECT uuid, owner_uuid, money, name, creation FROM inventory WHERE uuid = $1",
            uuid
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(inv) = inventory {
            let readers = self.get_readers(&inv.uuid).await?;
            let writers = self.get_writers(&inv.uuid).await?;
            Ok(Some(FullInventory {
                uuid: inv.uuid,
                owner_uuid: inv.owner_uuid,
                money: inv.money,
                name: inv.name,
                reader: readers,
                writer: writers,
                creation: inv.creation,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn create_inventory(&self, owner_uuid: &str, money: i32, name: &str) -> Result<RawInventory, Error> {
        let uuid = Uuid::new_v4().to_string();
        let rec = sqlx::query_as!(RawInventory,
            "INSERT INTO inventory (uuid, owner_uuid, money, name) VALUES ($1, $2, $3, $4) RETURNING *",
            uuid, owner_uuid, money, name
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(rec)
    }

    pub async fn get_raw_inventory(&self, uuid: &str) -> Result<Option<RawInventory>, Error> {
        let inventory = sqlx::query_as!(RawInventory,
            "SELECT * FROM inventory WHERE uuid = $1",
            uuid
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(inventory)
    }

    pub async fn update_inventory_money(&self, uuid: &str, money: i32) -> Result<(), Error> {
        sqlx::query!("UPDATE inventory SET money = $1 WHERE uuid = $2", money, uuid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_inventory(&self, uuid: &str) -> Result<(), Error> {
        sqlx::query!("DELETE FROM inventory WHERE uuid = $1", uuid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn add_reader(&self, user_uuid: &str, inventory_uuid: &str) -> Result<(), Error> {
        sqlx::query!("INSERT INTO inventory_reader (user_uuid, inventory_uuid) VALUES ($1, $2)", user_uuid, inventory_uuid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn remove_reader(&self, user_uuid: &str, inventory_uuid: &str) -> Result<(), Error> {
        sqlx::query!("DELETE FROM inventory_reader WHERE user_uuid = $1 AND inventory_uuid = $2", user_uuid, inventory_uuid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn add_writer(&self, user_uuid: &str, inventory_uuid: &str) -> Result<(), Error> {
        sqlx::query!("INSERT INTO inventory_writer (user_uuid, inventory_uuid) VALUES ($1, $2)", user_uuid, inventory_uuid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn remove_writer(&self, user_uuid: &str, inventory_uuid: &str) -> Result<(), Error> {
        sqlx::query!("DELETE FROM inventory_writer WHERE user_uuid = $1 AND inventory_uuid = $2", user_uuid, inventory_uuid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn add_inventory_item(&self, inventory_uuid: &str, item_preset_uuid: &str, dm_note: &str, amount: i32, sorting: i32, inventory_item_note: &str) -> Result<(), Error> {
        sqlx::query!("INSERT INTO inventory_item (inventory_uuid, item_preset_uuid, dm_note, amount, sorting, inventory_item_note) VALUES ($1, $2, $3, $4, $5, $6)",
            inventory_uuid, item_preset_uuid, dm_note, amount, sorting, inventory_item_note)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_inventory_item(&self, inventory_uuid: &str, item_preset_uuid: &str, dm_note: Option<&str>, amount: Option<i32>, sorting: Option<i32>, inventory_item_note: Option<&str>) -> Result<(), Error> {
        sqlx::query!("UPDATE inventory_item SET dm_note = COALESCE($3, dm_note), amount = COALESCE($4, amount), sorting = COALESCE($5, sorting), inventory_item_note = COALESCE($6, inventory_item_note) WHERE inventory_uuid = $1 AND item_preset_uuid = $2",
            inventory_uuid, item_preset_uuid, dm_note, amount, sorting, inventory_item_note)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn remove_inventory_item(&self, inventory_uuid: &str, item_preset_uuid: &str) -> Result<(), Error> {
        sqlx::query!("DELETE FROM inventory_item WHERE inventory_uuid = $1 AND item_preset_uuid = $2", inventory_uuid, item_preset_uuid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
