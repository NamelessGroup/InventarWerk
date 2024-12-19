use rocket::form::{Form, FromForm};

#[derive(FromForm)]
pub struct InventoryUUIDParams {
    inventory_uuid: String
}

#[derive(FromForm)]
pub struct InventoryCreateParams {
    name: String
}

#[derive(FromForm)]
pub struct InvnetoryAddItemByNameParams {
    inventory_uuid: String,
    name: String,
    amount:i32
}

#[derive(FromForm)]
pub struct InventoryAddItemByPresetParams {
    inventory_uuid: String,
    preset_uuid: String,
    amount:i32
}

#[derive(Debug, FromForm)]
pub struct InventoryModifyMoneyParams {
    amount: i32
}

#[derive(FromForm)]
pub struct InventoryShareParams {
    inventory_uuid: String,
    reader_uuid: Option<String>,
    writer_uuid: Option<String>
}

#[get("/inventar/all")]
pub async fn getAllInventories() -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}

#[get("/inventar?<params..>")]
pub async fn getSpecificInventory(params: InventoryUUIDParams) -> String {
    // return specific inventory
    format!("Hello, Rocket with async! {}", params.inventory_uuid)
}

#[put("/inventar?<params..>")]
pub async fn createInventory(params: InventoryCreateParams) -> &'static str {
    // create New Inventory
    "Hello, Rocket with async!"
}

#[put("/inventar/addPreset?<params..>")]
pub async fn addPresetToInventory(params: InventoryAddItemByPresetParams) -> &'static str {
    // add Preset to Inventory
    "Hello, Rocket with async!"
}

#[put("/inventar/addNew?<params..>")]
pub async fn addNewItemToInventory(params:InvnetoryAddItemByNameParams) -> &'static str {
    // add Item to Inventory
    "Hello, Rocket with async!"
}

#[patch("/inventar/money?<params..>")]
pub async fn modifyMoney(params: InventoryModifyMoneyParams) -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}

#[patch("/inventar/share?<params..>")]
pub async fn shareInventory(params: InventoryShareParams) -> &'static str {
    // share Inventory
    "Hello, Rocket with async!"
}

#[delete("/inventar/delete?<params..>")]
pub async fn deleteInventory(params:InventoryUUIDParams) -> &'static str {
    // delete Inventory
    "Hello, Rocket with async!"
}
