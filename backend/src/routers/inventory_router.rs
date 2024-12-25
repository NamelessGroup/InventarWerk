use rocket::http::Status;
use rocket::{form::FromForm, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use crate::controller::inventory_controller::InventoryController;
use crate::frontend_model::{InventoryReturn, Item};


use rocket::response::status::{self, Custom};

use super::transform_to_http_error;

#[derive(Serialize, Deserialize)]
pub struct GetAllInventoriesReturn{
    inventories: Vec<InventoryReturn>
}

#[derive(FromForm)]
pub struct ItemDeleteParams {
    inventory_uuid: String,
    item_preset_uuid: String
}

#[derive(FromForm)]
pub struct ItemEditParams {
    inventory_uuid: String,
    item_preset_uuid: String,
    amount: i32
}

#[derive(FromForm)]
pub struct NoteAddParams {
    item_preset_uuid: String,
    inventory_uuid: String,
    note: String
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

#[get("/inventory/all")]
pub async fn get_all_inventories(user: super::AuthenticatedUser,
        inv_con: &State<InventoryController>) -> Result<Json<GetAllInventoriesReturn>, Custom<&'static str>>  {
    
    Ok(Json(GetAllInventoriesReturn {
        inventories: match inv_con.get_inventories_parsed(user.user_id) {
            Ok(res) => res,
            Err(e) => return  Err(Custom(
                Status::InternalServerError,
                e
            ))
        }
    }))

}

#[get("/inventory?<params..>")]
pub async fn get_specific_inventory(params: InventoryUUIDParams,  user: super::AuthenticatedUser,
    inv_con: &State<InventoryController>) -> Result<Json<InventoryReturn>, Custom<&'static str>> {
    transform_to_http_error(inv_con.get_inventory_parsed(params.inventory_uuid.clone(), user.user_id.clone()),
        Status::BadRequest)
}

#[put("/inventory?<params..>")]
pub async fn create_inventory(params: InventoryCreateParams,  user: super::AuthenticatedUser, inv_con: &State<InventoryController>) -> Result<Json<InventoryReturn>, Custom<&'static str>> {
    let inv_uuid = match inv_con.insert_inventory(params.name, user.user_id.clone()) {
        Ok(res) => res.uuid,
        Err(e) => return Err(Custom (
            Status::InternalServerError,
            e
        ))
    };
    get_specific_inventory(InventoryUUIDParams {inventory_uuid: inv_uuid}, user, inv_con).await
}

#[put("/inventory/item/addPreset?<params..>")]
pub async fn add_preset_to_inventory(params: InventoryAddItemByPresetParams,  user: super::AuthenticatedUser, inv_con: &State<InventoryController>) -> Result<Json<InventoryReturn>, Custom<&'static str>> {
    let item_pair = inv_con.add_preset_to_inventory(params.inventory_uuid, params.preset_uuid, params.amount);
    Err(Custom(
        Status::NotImplemented,
        "Not Implemented"
    )) //TODO: implement item
}

#[put("/inventory/item/addNew?<params..>")]
pub async fn add_new_item_to_inventory(params:InvnetoryAddItemByNameParams,  user: super::AuthenticatedUser) -> &'static str {
    // add Item to Inventory
    "Hello, Rocket with async!"
}

#[patch("/inventory/item/edit?<params..>")]
pub async fn edit_item(params: ItemEditParams, user: super::AuthenticatedUser) -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}

#[get("/inventory/item/addNote?<params..>")]
pub async fn add_note_to_item(params: NoteAddParams, user: super::AuthenticatedUser) -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}

#[get("/inventory/item/remove?<params..>")]
pub async fn delete_item_from_inventory(params: ItemDeleteParams, user: super::AuthenticatedUser) -> &'static str {
    // add Item to Inventory
    "Hello, Rocket with async!"
}

#[patch("/inventory/money?<params..>")]
pub async fn modify_money(params: InventoryModifyMoneyParams,  user: super::AuthenticatedUser) -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}

#[patch("/inventory/share?<params..>")]
pub async fn share_inventory(params: InventoryShareParams,  user: super::AuthenticatedUser) -> &'static str {
    // share Inventory
    "Hello, Rocket with async!"
}

#[delete("/inventory/delete?<params..>")]
pub async fn delete_inventory(params:InventoryUUIDParams,  user: super::AuthenticatedUser) -> &'static str {
    // delete Inventory
    "Hello, Rocket with async!"
}
