use rocket::{Build, Rocket, Route};
use rocket::form::{Form, FromForm};

struct InventoryUUIDParams {
    inventory_uuid: String
}

struct InvnetoryAddItemByNameParams {
    inventory_uuid: String,
    name: String,
    amount:i32
}

struct InventoryAddItemByPresetParams {
    inventory_uuid: String,
    preset_uuid: String,
    amount:i32
}

struct ShareInventoryParams {
    inventory_uuid: String,
    reader_uuid: Option<String>,
    writer_uuid: Option<String>
}

#[get("/inventar/all")]
pub async fn getAllInventories() -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}

#[get("/inventar")]
pub async fn getSpecificInventory() -> &'static str {
    // return specific inventory
    "Hello, Rocket with async!"
}

#[put("/inventar")]
pub async fn createInventory() -> &'static str {
    // create New Inventory
    "Hello, Rocket with async!"
}

#[put("/inventar/addPreset")]
pub async fn addPresetToInventory() -> &'static str {
    // add Preset to Inventory
    "Hello, Rocket with async!"
}

#[put("/inventar/addNew")]
pub async fn addNewItemToInventory() -> &'static str {
    // add Item to Inventory
    "Hello, Rocket with async!"
}

#[patch("/inventar/money")]
pub async fn modifyMoney() -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}

#[patch("/inventar/share")]
pub async fn shareInventory() -> &'static str {
    // share Inventory
    "Hello, Rocket with async!"
}

#[delete("/inventar/delete")]
pub async fn deleteInventory() -> &'static str {
    // delete Inventory
    "Hello, Rocket with async!"
}
