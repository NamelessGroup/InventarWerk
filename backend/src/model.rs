use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::schema::{inventory, inventory_reader, inventory_writer, item_preset, user, inventory_item};

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = user)]
#[diesel(primary_key(uuid))]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub dm: i32,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key=owner_uuid))]
#[diesel(table_name = inventory)]
#[diesel(primary_key(uuid))]
pub struct Inventory {
    pub uuid: String,
    pub owner_uuid: String,
    pub money: i32,
    pub name: String,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key=user_uuid))]
#[diesel(belongs_to(Inventory, foreign_key=inventory_uuid))]
#[diesel(table_name = inventory_reader)]
#[diesel(primary_key(user_uuid, inventory_uuid))]
pub struct InventoryReader {
    pub user_uuid: String,
    pub inventory_uuid: String,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key=user_uuid))]
#[diesel(belongs_to(Inventory, foreign_key=inventory_uuid))]
#[diesel(table_name = inventory_writer)]
#[diesel(primary_key(user_uuid, inventory_uuid))]
pub struct InventoryWriter {
    pub user_uuid: String,
    pub inventory_uuid: String,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = item_preset)]
#[diesel(primary_key(uuid))]
pub struct ItemPreset {
    pub uuid: String,
    pub name: String,
    pub price: i32,
    pub description: String,
    pub creator: String,
    pub item_type: String,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(Inventory, foreign_key=inventory_uuid))]
#[diesel(belongs_to(ItemPreset, foreign_key=item_preset_uuid))]
#[diesel(table_name = inventory_item)]
#[diesel(primary_key(inventory_uuid, item_preset_uuid))]
pub struct InventoryItem {
    pub inventory_uuid: String,
    pub item_preset_uuid: String,
    pub dm_note: String,
    pub amount: i32,
}