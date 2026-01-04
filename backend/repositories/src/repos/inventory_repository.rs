use crate::model_inventarwerk::{FrontendItem, FullFrontendInventory, InventoryItem, RawInventory};
use anyhow::{self, Result};
use sqlx::{Error, PgPool};
use uuid::Uuid;
pub struct InventoryRepository {
    pool: PgPool,
}

impl InventoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Returns all user UUIDs with read access to the given inventory.
    pub async fn get_readers(&self, inventory_uuid: &str) -> Result<Vec<String>> {
        let readers = sqlx::query!(
            "SELECT user_uuid FROM inventory_reader WHERE inventory_uuid = $1",
            inventory_uuid
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|r| r.user_uuid)
        .collect();
        Ok(readers)
    }

    /// Returns all user UUIDs with write access to the given inventory.
    pub async fn get_writers(&self, inventory_uuid: &str) -> Result<Vec<String>> {
        let writers = sqlx::query!(
            "SELECT user_uuid FROM inventory_writer WHERE inventory_uuid = $1",
            inventory_uuid
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|w| w.user_uuid)
        .collect();
        Ok(writers)
    }

    /// Retrieves the full inventory data, including readers, writers, and items, for the given inventory UUID.
    pub async fn get_full_inventory(&self, uuid: &str) -> Result<FullFrontendInventory> {
        let inventory = sqlx::query!(
            "SELECT uuid, owner_uuid, money, name, creation FROM inventory WHERE uuid = $1",
            uuid
        )
        .fetch_one(&self.pool)
        .await?;

        let readers = self.get_readers(&inventory.uuid).await?;
        let writers = self.get_writers(&inventory.uuid).await?;
        let items = self
            .get_frontend_items_in_inventory(&inventory.uuid)
            .await?;
        Ok(FullFrontendInventory {
            uuid: inventory.uuid,
            owner_uuid: inventory.owner_uuid,
            money: inventory.money,
            name: inventory.name,
            reader: readers,
            writer: writers,
            items: items,
            creation: inventory.creation,
        })
    }

    /// Returns all inventory UUIDs owned by the given user.
    pub async fn get_user_inventory_ids(&self, user_uuid: &str) -> Result<Vec<String>> {
        let inventory_ids = sqlx::query!(
            "SELECT uuid FROM inventory WHERE owner_uuid = $1",
            user_uuid
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|record| record.uuid)
        .collect();
        Ok(inventory_ids)
    }

    /// Returns all inventory UUIDs where the user is a reader.
    pub async fn get_inventories_by_reader(&self, user_uuid: &str) -> Result<Vec<String>> {
        let inventory_ids = sqlx::query!(
            "SELECT inventory_uuid FROM inventory_reader WHERE user_uuid = $1",
            user_uuid
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|record| record.inventory_uuid)
        .collect();

        Ok(inventory_ids)
    }

    pub async fn get_owned_and_readable_inventory_ids(
        &self,
        user_uuid: &str,
    ) -> Result<Vec<String>> {
        let mut invs = self.get_user_inventory_ids(user_uuid).await?;
        let read_invs = self.get_inventories_by_reader(user_uuid).await?;
        invs.extend(read_invs);
        return Ok(invs);
    }

    /// Returns all inventory UUIDs where the user is a writer.
    pub async fn get_inventories_by_writer(&self, user_uuid: &str) -> Result<Vec<String>> {
        let inventory_ids = sqlx::query!(
            "SELECT inventory_uuid FROM inventory_writer WHERE user_uuid = $1",
            user_uuid
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|record| record.inventory_uuid)
        .collect();

        Ok(inventory_ids)
    }

    /// Returns all inventories (as `FullFrontendInventory`) where the user is owner or reader.
    pub async fn get_all_inventories(&self, user_uuid: &str) -> Result<Vec<FullFrontendInventory>> {
        let query = sqlx::query!(
            "SELECT DISTINCT i.uuid
             FROM inventory i
             LEFT JOIN inventory_reader ir ON i.uuid = ir.inventory_uuid
             WHERE i.owner_uuid = $1 OR ir.user_uuid = $1",
            user_uuid
        );

        let inventories = query.fetch_all(&self.pool).await?;

        let mut full_inventories = Vec::new();

        for inv in inventories {
            full_inventories.push(self.get_full_inventory(&inv.uuid).await?);
        }

        Ok(full_inventories)
    }

    /// Creates a new inventory and adds the owner as reader and writer.
    pub async fn create_inventory(
        &self,
        owner_uuid: &str,
        money: i32,
        name: &str,
    ) -> Result<RawInventory> {
        let uuid = Uuid::new_v4().to_string();
        let rec = sqlx::query_as!(RawInventory,
            "INSERT INTO inventory (uuid, owner_uuid, money, name) VALUES ($1, $2, $3, $4) RETURNING *",
            uuid, owner_uuid, money, name
        )
        .fetch_one(&self.pool)
        .await?;
        self.add_reader(&uuid, owner_uuid).await?;
        self.add_writer(&uuid, owner_uuid).await?;
        Ok(rec)
    }

    /// Retrieves the raw inventory data for the given UUID.
    pub async fn get_raw_inventory(&self, uuid: &str) -> Result<RawInventory> {
        let inventory = sqlx::query_as!(
            RawInventory,
            "SELECT * FROM inventory WHERE uuid = $1",
            uuid
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(inventory)
    }

    /// Updates the money and/or name of an inventory.
    pub async fn update_inventory(
        &self,
        uuid: &str,
        money: Option<i32>,
        name: Option<&str>,
    ) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE inventory SET money = COALESCE($1, money), name = COALESCE($2, name) WHERE uuid = $3",
            money,
            name,
            uuid
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Deletes an inventory by UUID.
    pub async fn delete_inventory(&self, uuid: &str) -> Result<()> {
        sqlx::query!("DELETE FROM inventory WHERE uuid = $1", uuid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Adds a user as a reader to an inventory.
    pub async fn add_reader(&self, inventory_uuid: &str, user_uuid: &str) -> Result<()> {
        sqlx::query!(
            "INSERT INTO inventory_reader (user_uuid, inventory_uuid) VALUES ($1, $2)",
            user_uuid,
            inventory_uuid
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Removes a user as a reader from an inventory.
    pub async fn remove_reader(&self, inventory_uuid: &str, user_uuid: &str) -> Result<()> {
        sqlx::query!(
            "DELETE FROM inventory_reader WHERE user_uuid = $1 AND inventory_uuid = $2",
            user_uuid,
            inventory_uuid
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Adds a user as a writer to an inventory.
    pub async fn add_writer(&self, inventory_uuid: &str, user_uuid: &str) -> Result<()> {
        sqlx::query!(
            "INSERT INTO inventory_writer (user_uuid, inventory_uuid) VALUES ($1, $2)",
            user_uuid,
            inventory_uuid
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Removes a user as a writer from an inventory.
    pub async fn remove_writer(&self, inventory_uuid: &str, user_uuid: &str) -> Result<()> {
        sqlx::query!(
            "DELETE FROM inventory_writer WHERE user_uuid = $1 AND inventory_uuid = $2",
            user_uuid,
            inventory_uuid
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Adds an item to an inventory.
    pub async fn add_inventory_item(
        &self,
        inventory_uuid: &str,
        item_preset_uuid: &str,
        dm_note: &str,
        amount: i32,
        sorting: i32,
        inventory_item_note: &str,
    ) -> Result<()> {
        sqlx::query!("INSERT INTO inventory_item (inventory_uuid, item_preset_uuid, dm_note, amount, sorting, inventory_item_note) VALUES ($1, $2, $3, $4, $5, $6)",
            inventory_uuid, item_preset_uuid, dm_note, amount, sorting, inventory_item_note)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Updates an item in an inventory.
    pub async fn update_inventory_item(
        &self,
        inventory_uuid: &str,
        item_preset_uuid: &str,
        dm_note: Option<&str>,
        amount: Option<i32>,
        sorting: Option<i32>,
        inventory_item_note: Option<&str>,
    ) -> Result<()> {
        sqlx::query!("UPDATE inventory_item SET dm_note = COALESCE($3, dm_note), amount = COALESCE($4, amount), sorting = COALESCE($5, sorting), inventory_item_note = COALESCE($6, inventory_item_note) WHERE inventory_uuid = $1 AND item_preset_uuid = $2",
            inventory_uuid, item_preset_uuid, dm_note, amount, sorting, inventory_item_note)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Removes an item from an inventory.
    pub async fn remove_inventory_item(
        &self,
        inventory_uuid: &str,
        item_preset_uuid: &str,
    ) -> Result<()> {
        sqlx::query!(
            "DELETE FROM inventory_item WHERE inventory_uuid = $1 AND item_preset_uuid = $2",
            inventory_uuid,
            item_preset_uuid
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Checks if an item exists in an inventory.
    pub async fn item_exists(&self, inventory_uuid: &str, item_preset_uuid: &str) -> Result<bool> {
        let result = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM inventory_item WHERE inventory_uuid = $1 AND item_preset_uuid = $2) AS exists",
            inventory_uuid,
            item_preset_uuid
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.exists.unwrap_or(false))
    }

    /// Returns all items in an inventory as `InventoryItem`.
    pub async fn get_items_in_inventory(&self, inventory_uuid: &str) -> Result<Vec<InventoryItem>> {
        let items = sqlx::query_as!(
            InventoryItem,
            "SELECT inventory_uuid, item_preset_uuid, dm_note, amount, sorting, inventory_item_note, creation 
             FROM inventory_item 
             WHERE inventory_uuid = $1",
            inventory_uuid
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }

    /// Returns all items in an inventory as `FrontendItem` (joined with preset data).
    pub async fn get_frontend_items_in_inventory(
        &self,
        inventory_uuid: &str,
    ) -> Result<Vec<FrontendItem>> {
        let items = sqlx::query!(
            "SELECT ii.inventory_uuid, ii.item_preset_uuid, ii.dm_note, ii.amount, ii.sorting, ii.inventory_item_note, ii.creation,
                    ip.name, ip.description, ip.price, ip.creator AS preset_creator, ip.weight, ip.item_type
             FROM inventory_item ii
             INNER JOIN item_preset ip ON ii.item_preset_uuid = ip.uuid
             WHERE ii.inventory_uuid = $1",
            inventory_uuid
        )
        .fetch_all(&self.pool)
        .await?;

        let frontend_items = items
            .into_iter()
            .map(|item| FrontendItem {
                name: item.name,
                amount: item.amount,
                dm_note: item.dm_note,
                description: item.description,
                price: item.price,
                preset_creator: item.preset_creator,
                weight: item.weight,
                sorting: item.sorting,
                item_type: item.item_type,
                preset_reference: item.item_preset_uuid,
                inventory_item_note: item.inventory_item_note,
            })
            .collect();

        Ok(frontend_items)
    }
}
