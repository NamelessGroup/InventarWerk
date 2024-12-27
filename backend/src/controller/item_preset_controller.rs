use diesel::r2d2::ConnectionManager;
use diesel::{QueryDsl, RunQueryDsl};
use r2d2::PooledConnection;
use diesel::prelude::*;
use rocket::http::Status;

use crate::dbmod::DbPool;
use crate::model::{ItemPreset, UpdateItemPreset};
use crate::schema::inventory_item;
use crate::schema::item_preset::dsl::*;
use crate::schema::inventory_item::dsl::*;

use super::{cstat, format_result_to_cstat};
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
            Ok(_res) => Ok(true),
            Err(_e) => Err("Couldn't delete item preset")
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
                Ok(_res) => Ok(true),
                Err(_e) => Err("Couldn't update item")
            }
    }

    pub fn get_item_preset_in_inventory(&self, searched_item_preset_uuid: String) -> Result<Vec<ItemPreset>, cstat> {
        let query = inventory_item
            .filter(inventory_item::inventory_uuid.eq(searched_item_preset_uuid))
            .inner_join(item_preset)
            .select((uuid, name, price, description, creator, item_type))
            .load::<ItemPreset>(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Couldn't load tables inventory_item and item_preset")
    }

}