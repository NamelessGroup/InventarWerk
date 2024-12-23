use diesel::r2d2::ConnectionManager;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use r2d2::PooledConnection;

use crate::model::{InventoryItem, ItemPreset, User};
use crate::{dbmod::DbPool, model::Inventory};
use crate::schema::{inventory, inventory_item, inventory_reader, inventory_writer, user};
use crate::schema::inventory::dsl::*;
use crate::schema::inventory_reader::dsl::*;
use crate::schema::inventory_writer::dsl::*;
use crate::schema::inventory_item::dsl::*;
use crate::schema::item_preset::dsl::*;
use crate::schema::user::dsl::*;
use crate::frontend_model::{InventoryReturn, Item};

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

    fn get_all_inventories(&self, inventory_user_uuid: String) -> Vec<Inventory> {
        inventory
            .inner_join(inventory_reader.on(inventory_reader::inventory_uuid.eq(inventory::dsl::uuid)))
            .filter(inventory_reader::user_uuid.eq(inventory_user_uuid))
            .select((inventory::dsl::uuid, owner_uuid, money, inventory::dsl::name))
            .load::<Inventory>(&mut self.get_conn()).expect("faild to load inventories")
    }

    fn get_readers_for_inventory(&self, searched_inventory_uuid: String) -> Vec<String> {
        inventory_reader.filter(inventory_reader::inventory_uuid.eq(searched_inventory_uuid))
            .select(inventory_reader::dsl::user_uuid)
            .load::<String>(&mut self.get_conn()).expect("Couldn't load inventories")
    }

    fn get_writers_for_inventories(&self, searched_inventory_uuid: String) -> Vec<String> {
        inventory_writer.filter(inventory_writer::inventory_uuid.eq(searched_inventory_uuid))
            .select(inventory_writer::dsl::user_uuid)
            .load::<String>(&mut self.get_conn()).expect("Couldn't load inventories")
    }

    fn get_items_in_inventory(&self, searched_inventory_uuid: String) -> Vec<(String, i32)> {
        inventory_item.filter(inventory_item::inventory_uuid.eq(searched_inventory_uuid))
            .select((inventory_item::dsl::item_preset_uuid, inventory_item::dsl::amount))
            .load::<(String, i32)>(&mut self.get_conn()).expect("Could not load any item")
    }

    fn get_item_preset(&self, searched_item_preset: String) -> ItemPreset{
        item_preset.find(searched_item_preset).get_result(&mut self.get_conn())
            .expect("Could not load any item preset")
    }

    pub fn get_dm_note(&self, searched_inventory_uuid: String, searched_item_preset: String) -> String {
        let item = inventory_item.find((searched_inventory_uuid, searched_item_preset))
            .get_result::<InventoryItem>(&mut self.get_conn()).expect("Could not load dm Note");
        item.dm_note
    }
    pub fn get_inventories_parsed(&self, searcher_uuid: String) -> Vec<InventoryReturn> {
        let inv = self.get_all_inventories(searcher_uuid.clone());
        let mut inventories: Vec<InventoryReturn> = Vec::new();
        let user_is_dm = user.find(searcher_uuid).get_result::<User>(&mut self.get_conn())
            .expect("user could not be queried").dm == 1;
        for i in inv.iter() {
            let mut specific_inventory = InventoryReturn{
                uuid: i.uuid.clone(),
                name: i.name.clone(),
                owner: i.owner_uuid.clone(),
                money: i.money,
                items: Vec::new(),
                reader: self.get_readers_for_inventory(i.uuid.clone()),
                writer: self.get_writers_for_inventories(i.uuid.clone())
            };
            let items = self.get_items_in_inventory(i.uuid.clone());
            for item in items.iter() {
                let preset = self.get_item_preset(item.0.clone());
                specific_inventory.items.push(Item {
                    name: preset.name.clone(),
                    presetReference: item.0.clone(),
                    amount: item.1,
                    dmNote: if user_is_dm {self.get_dm_note(specific_inventory.uuid.clone(), item.0.clone())} else {"".to_string()},
                    description: preset.description.clone()
                });
            }
            inventories.push(specific_inventory);
        }
        inventories
    }
}