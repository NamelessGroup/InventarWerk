use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;


#[derive(Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub avatar: String,
    pub dm: i32,
    pub creation: Option<PrimitiveDateTime>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullFrontendInventory {
    pub uuid: String,
    pub owner_uuid: String,
    pub money: i32,
    pub name: String,
    pub items: Vec<FrontendItem>,
    pub reader: Vec<String>,
    pub writer: Vec<String>,
    pub creation: Option<PrimitiveDateTime>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawInventory {
    pub uuid: String,
    pub owner_uuid: String,
    pub money: i32,
    pub name: String,
    pub creation: Option<PrimitiveDateTime>
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryReader {
    pub user_uuid: String,
    pub inventory_uuid: String,
    pub creation: Option<PrimitiveDateTime>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryWriter {
    pub user_uuid: String,
    pub inventory_uuid: String,
    pub creation: Option<PrimitiveDateTime>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemPreset {
    pub uuid: String,
    pub name: String,
    pub price: i32,
    pub weight: f32,
    pub description: String,
    pub creator: String,
    pub item_type: String,
    pub creation: Option<PrimitiveDateTime>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItem {
    pub inventory_uuid: String,
    pub item_preset_uuid: String,
    pub dm_note: String,
    pub amount: i32,
    pub sorting: i32,
    pub inventory_item_note: String,
    pub creation: Option<PrimitiveDateTime>
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontendItem {
    pub name: String,
    pub amount: i32,
    pub dm_note: String,
    pub description: String,
    pub price: i32,
    pub preset_creator: String,
    pub weight: f32,
    pub sorting: i32,
    pub item_type: String,
    pub preset_reference: String,
    pub inventory_item_note: String,
  }