use rocket::{form::FromForm, serde::json::Json, State};
use serde::{Deserialize, Serialize};

use crate::controller::inventory_controller::InventoryController;



#[derive(Serialize, Deserialize)]
pub struct InventoryReturn {
    uuid: String,
    name: String,
    owner: String,
    money: i32,
    reader: Vec<String>,
    writer: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct GetAllInventoriesReturn{
    inventories: Vec<InventoryReturn>
}

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
    inventory_uuid:String,
    amount: i32
}

#[derive(FromForm)]
pub struct InventoryShareParams {
    inventory_uuid: String,
    reader_uuid: Option<String>,
    writer_uuid: Option<String>
}

#[get("/inventar/all")]
pub async fn get_all_inventories(user: super::AuthenticatedUser, inv_con: &State<InventoryController>) -> Json<GetAllInventoriesReturn> {
    let inv = inv_con.get_all_inventories(user.user_id);
    let mut inv_ret = GetAllInventoriesReturn {
        inventories: Vec::new()
    };
    for i in inv.iter() {
        inv_ret.inventories.push(InventoryReturn{
            uuid: i.uuid.clone(),
            name: i.name.clone(),
            owner: i.owner_uuid.clone(),
            money: i.money,
            reader: inv_con.get_readers_for_inventory(i.uuid.clone()),
            writer: inv_con.get_writers_for_inventories(i.uuid.clone())
        });
    }
    Json(inv_ret)

}

#[get("/inventar?<params..>")]
pub async fn get_specific_inventory(params: InventoryUUIDParams,  _user: super::AuthenticatedUser) -> String {
    // return specific inventory
    format!("Hello, Rocket with async! {}", params.inventory_uuid)
}

#[put("/inventar?<params..>")]
pub async fn create_inventory(params: InventoryCreateParams,  user: super::AuthenticatedUser) -> &'static str {
    // create New Inventory
    "Hello, Rocket with async!"
}

#[put("/inventar/addPreset?<params..>")]
pub async fn add_preset_to_inventory(params: InventoryAddItemByPresetParams,  user: super::AuthenticatedUser) -> &'static str {
    // add Preset to Inventory
    "Hello, Rocket with async!"
}

#[put("/inventar/addNew?<params..>")]
pub async fn add_new_item_to_inventory(params:InvnetoryAddItemByNameParams,  user: super::AuthenticatedUser) -> &'static str {
    // add Item to Inventory
    "Hello, Rocket with async!"
}

#[patch("/inventar/money?<params..>")]
pub async fn modify_money(params: InventoryModifyMoneyParams,  user: super::AuthenticatedUser) -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}

#[patch("/inventar/share?<params..>")]
pub async fn share_inventory(params: InventoryShareParams,  user: super::AuthenticatedUser) -> &'static str {
    // share Inventory
    "Hello, Rocket with async!"
}

#[delete("/inventar/delete?<params..>")]
pub async fn delete_inventory(params:InventoryUUIDParams,  user: super::AuthenticatedUser) -> &'static str {
    // delete Inventory
    "Hello, Rocket with async!"
}
