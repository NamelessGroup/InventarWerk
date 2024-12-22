use diesel::r2d2::ConnectionManager;
use diesel::sql_types::Integer;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use r2d2::PooledConnection;

use crate::model::{InventoryItem, ItemPreset};
use crate::{dbmod::DbPool, model::Inventory};
use crate::schema::{inventory_item, inventory_reader, inventory_writer, inventory};
use crate::schema::inventory::dsl::*;
use crate::schema::inventory_reader::dsl::*;
use crate::schema::inventory_writer::dsl::*;
use crate::schema::inventory_item::dsl::*;
use crate::schema::item_preset::dsl::*;

#[derive(Clone)]
pub struct InventoryController {
    db: DbPool,
}

impl InventoryController {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    fn get_conn(&self) -> PooledConnection<ConnectionManager<diesel::SqliteConnection>> {
        self.db.get().expect("Failed to get connection from Pool")
    }

    pub fn get_all_inventories(&self, inventory_user_uuid: String) -> Vec<Inventory> {
        inventory
            .inner_join(inventory_reader.on(inventory_reader::inventory_uuid.eq(inventory::dsl::uuid)))
            .filter(inventory_reader::user_uuid.eq(inventory_user_uuid))
            .select((inventory::dsl::uuid, owner_uuid, money, inventory::dsl::name))
            .load::<Inventory>(&mut self.get_conn()).expect("faild to load inventories")
    }

    pub fn get_readers_for_inventory(&self, searched_inventory_uuid: String) -> Vec<String> {
        inventory_reader.filter(inventory_reader::inventory_uuid.eq(searched_inventory_uuid))
            .select(inventory_reader::dsl::user_uuid)
            .load::<String>(&mut self.get_conn()).expect("Couldn't load inventories")
    }

    pub fn get_writers_for_inventories(&self, searched_inventory_uuid: String) -> Vec<String> {
        inventory_writer.filter(inventory_writer::inventory_uuid.eq(searched_inventory_uuid))
            .select(inventory_writer::dsl::user_uuid)
            .load::<String>(&mut self.get_conn()).expect("Couldn't load inventories")
    }

    pub fn get_items_in_inventory(&self, searched_inventory_uuid: String) -> Vec<(String, i32)> {
        inventory_item.filter(inventory_item::inventory_uuid.eq(searched_inventory_uuid))
            .select((inventory_item::dsl::item_preset_uuid, inventory_item::dsl::amount))
            .load::<(String, i32)>(&mut self.get_conn()).expect("Could not load any item")
    }

    pub fn get_item_preset(&self, searched_item_preset: String) -> ItemPreset{
        item_preset.find(searched_item_preset).get_result(&mut self.get_conn())
            .expect("Could not load any item preset")
    }

    pub fn get_dm_note(&self, searched_inventory_uuid: String, searched_item_preset: String) -> String {
        let item = inventory_item.find((searched_inventory_uuid, searched_item_preset))
            .get_result::<InventoryItem>(&mut self.get_conn()).expect("Could not load dm Note");
        item.dm_note
    }
}