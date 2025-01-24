use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::schema::{inventory, inventory_reader, inventory_writer, item_preset, user, inventory_item};

#[derive(Queryable, Identifiable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = user)]
#[diesel(primary_key(uuid))]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub avatar: String,
    pub dm: i32,
}

#[derive(Queryable, Identifiable, Associations, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key=owner_uuid))]
#[diesel(table_name = inventory)]
#[diesel(primary_key(uuid))]
pub struct Inventory {
    pub uuid: String,
    pub owner_uuid: String,
    pub money: i32,
    pub name: String,
}

#[derive(Queryable, Identifiable, Associations, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key=user_uuid))]
#[diesel(belongs_to(Inventory, foreign_key=inventory_uuid))]
#[diesel(table_name = inventory_reader)]
#[diesel(primary_key(user_uuid, inventory_uuid))]
pub struct InventoryReader {
    pub user_uuid: String,
    pub inventory_uuid: String,
}

#[derive(Queryable, Identifiable, Associations, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key=user_uuid))]
#[diesel(belongs_to(Inventory, foreign_key=inventory_uuid))]
#[diesel(table_name = inventory_writer)]
#[diesel(primary_key(user_uuid, inventory_uuid))]
pub struct InventoryWriter {
    pub user_uuid: String,
    pub inventory_uuid: String,
}

#[derive(Queryable, Identifiable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = item_preset)]
#[diesel(primary_key(uuid))]
pub struct ItemPreset {
    pub uuid: String,
    pub name: String,
    pub price: i32,
    pub weight: f32,
    pub description: String,
    pub creator: String,
    pub item_type: String,
}

#[derive(Queryable, Identifiable, Associations, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(Inventory, foreign_key=inventory_uuid))]
#[diesel(belongs_to(ItemPreset, foreign_key=item_preset_uuid))]
#[diesel(table_name = inventory_item)]
#[diesel(primary_key(inventory_uuid, item_preset_uuid))]
pub struct InventoryItem {
    pub inventory_uuid: String,
    pub item_preset_uuid: String,
    pub dm_note: String,
    pub amount: i32,
    pub sorting: i32,
    pub inventory_item_note: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = inventory_item)]
pub struct UpdateInventoryItem {
    pub dm_note: Option<String>,
    pub amount: Option<i32>,
    pub sorting: Option<i32>,
    pub inventory_item_note: Option<String>
}

#[derive(AsChangeset)]
#[diesel(table_name = inventory)]
pub struct UpdateInventoryMoney {
    pub money: i32
}

#[derive(AsChangeset)]
#[diesel(table_name = item_preset)]
pub struct UpdateItemPreset {
    pub name: Option<String>,
    pub price: Option<i32>,
    pub weight: Option<f32>,
    pub description: Option<String>,
    pub item_type: Option<String>,
}