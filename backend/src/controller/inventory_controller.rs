use diesel::r2d2::ConnectionManager;
use diesel::{CombineDsl, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use r2d2::PooledConnection;

use crate::{dbmod::DbPool, model::Inventory};
use crate::schema::{inventory_reader, inventory_writer};
use crate::schema::inventory::dsl::*;
use crate::schema::inventory_reader::dsl::*;
use crate::schema::inventory_writer::dsl::*;

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
            .inner_join(inventory_reader.on(inventory_reader::inventory_uuid.eq(uuid)))
            .filter(inventory_reader::user_uuid.eq(inventory_user_uuid))
            .select((uuid, owner_uuid, money, name)).load::<Inventory>(&mut self.get_conn()).expect("msg")
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
}