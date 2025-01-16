use diesel::r2d2::ConnectionManager;
use diesel::{QueryDsl, RunQueryDsl};
use r2d2::PooledConnection;
use diesel::prelude::*;
use rocket::http::Status;

use crate::dbmod::DbPool;
use crate::model::{ItemPreset, UpdateItemPreset};
use crate::report_change_on_inventory;
use crate::schema::inventory_item;
use crate::schema::item_preset;
use crate::schema::item_preset::dsl::*;
use crate::schema::inventory_item::dsl::*;

use super::{CStat, format_result_to_cstat};
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

    pub fn get_item_preset(&self, searched_item_preset_uuid: String) -> Result<ItemPreset, CStat> {
        let query = item_preset.find(searched_item_preset_uuid).get_result::<ItemPreset>(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to query preset")
    }

    pub fn delete_item_preset(&self, searched_item_preset_uuid: String) -> Result<bool, CStat> {
        let query = diesel::delete(item_preset.find(searched_item_preset_uuid.clone())).execute(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to update preset")?;
        self.report_change_on_item_preset(searched_item_preset_uuid)?;
        Ok(true)
    }

    pub fn edit_item_preset(&self, searched_item_preset_uuid: String, new_name: Option<String>, new_price: Option<i32>, 
            new_weight: Option<f32>, new_description: Option<String>, new_type: Option<String>) -> Result<bool, CStat> {
        let item_preset_changes = UpdateItemPreset {
            name: new_name,
            price: new_price,
            weight: new_weight,
            description: new_description,
            item_type: new_type
        };
        let query = diesel::update(item_preset.find(searched_item_preset_uuid.clone())).set(item_preset_changes)
            .execute(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to update preset")?;
        self.report_change_on_item_preset(searched_item_preset_uuid)?;
        Ok(true)
    }

    pub fn get_item_preset_in_inventory(&self, searched_item_preset_uuid: String) -> Result<Vec<ItemPreset>, CStat> {
        let query = inventory_item
            .filter(inventory_item::inventory_uuid.eq(searched_item_preset_uuid))
            .inner_join(item_preset)
            .select((uuid, name, price, weight, description, creator, item_type))
            .load::<ItemPreset>(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to load tables inventory_item and item_preset")
    }

    fn report_change_on_item_preset(&self, searched_item_preset_uuid: String) -> Result<bool, CStat> {
        let query = inventory_item.filter(inventory_item::item_preset_uuid.eq(searched_item_preset_uuid))
            .select(inventory_item::inventory_uuid).load::<String>(&mut self.get_conn());
        let inventories = format_result_to_cstat(query, Status::InternalServerError,
             "Couldn't load inventory item")?;
        for i in inventories {
            report_change_on_inventory!(i);
        }
        Ok(true)
    }
    pub fn add_extern_preset(&self, preset_name: String, item_price:i32, preset_weight: f32,
        item_description: String, creator_uuid: String, i_type: String) -> Result<ItemPreset, CStat> {
    let new_item_preset = ItemPreset {
        name: preset_name,
        uuid: super::generate_uuid_v4(),
        price: item_price,
        weight: preset_weight,
        description: item_description,
        creator: creator_uuid,
        item_type: i_type
    };
    let query = diesel::insert_into(item_preset::table).values(&new_item_preset)
        .execute(&mut self.get_conn());
    format_result_to_cstat(query, Status::InternalServerError, "Failed to insert into table")?;
    return Ok(new_item_preset);
}

}