use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub presetReference: String,
    pub amount: i32,
    pub dmNote: String,
    pub description: String,
    pub weight: f32,
    pub sorting: i32,
    pub InventoryItemNote: String
}

#[derive(Serialize, Deserialize)]
pub struct InventoryReturn {
    pub uuid: String,
    pub name: String,
    pub owner: String,
    pub money: i32,
    pub items: Vec<Item>,
    pub reader: Vec<String>,
    pub writer: Vec<String>
}