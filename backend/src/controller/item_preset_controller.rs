use diesel::r2d2::ConnectionManager;
use diesel::{QueryDsl, RunQueryDsl};
use r2d2::PooledConnection;
use diesel::prelude::*;

use crate::dbmod::DbPool;
use crate::model::{ItemPreset, UpdateItemPreset, User};
use crate::schema::{inventory, inventory_item, item_preset};
use crate::schema::item_preset::dsl::*;
use crate::schema::inventory_item::dsl::*;
#[derive(Clone)]
pub struct ItemPresetController {
    db: DbPool
}

impl ItemPresetController {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    fn get_conn(&self) -> PooledConnection<ConnectionManager<diesel::SqliteConnection>> {
        self.db.get().expect("Failed to get connection from Pool")
    }

    pub fn get_item_preset(&self, searched_item_preset_uuid: String) -> Result<ItemPreset, &'static str> {
        match item_preset.find(searched_item_preset_uuid).get_result::<ItemPreset>(&mut self.get_conn()) {
            Ok(res) => Ok(res),
            Err(_e) => Err("Couldn't load inventory")
        }
    }

    pub fn delete_item_preset(&self, searched_item_preset_uuid: String) -> Result<bool, &'static str> {
        match diesel::delete(item_preset.find(searched_item_preset_uuid)).execute(&mut self.get_conn()) {
            Ok(res) => Ok(true),
            Err(_E) => Err("Couldn't delete item preset")
        }
    }

    pub fn edit_item_preset(&self, searched_item_preset_uuid: String, new_name: Option<String>, new_price: Option<i32>, 
            new_description: Option<String>, new_type: Option<String>) -> Result<bool, &'static str> {
        let item_preset_changes = UpdateItemPreset {
            name: new_name,
            price: new_price,
            description: new_description,
            item_type: new_type
        };
        match diesel::update(item_preset.find(searched_item_preset_uuid)).set(item_preset_changes)
            .execute(&mut self.get_conn()) {
                Ok(res) => Ok(true),
                Err(_e) => Err("Couldn't update item")
            }
    }

    pub fn item_presets_in_inventory(&self, searched_inventory: String) -> Result<Vec<String>, &'static str> {
    match inventory_item
        .filter(inventory_uuid.eq(searched_inventory))
        .select(item_preset_uuid)
        .load::<String>(&mut self.get_conn()) {
            Ok(res) => Ok(res),
            Err(_e) => Err("")
        }
    }
}