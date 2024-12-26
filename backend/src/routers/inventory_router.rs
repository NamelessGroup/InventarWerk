use std::str::Split;

use rocket::http::Status;
use rocket::{form::FromForm, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use crate::controller::account_controller::AccountController;
use crate::controller::inventory_controller::InventoryController;
use crate::frontend_model::{InventoryReturn, Item};
use crate::model::ItemPreset;


use rocket::response::status::{self, Custom};

use super::{tthe, ttjhe};

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
        inventories: inv_con.get_inventories_parsed(user.user_id.clone())?
    }))
}

#[get("/inventory?<params..>")]
pub async fn get_specific_inventory(params: InventoryUUIDParams,  user: super::AuthenticatedUser,
    inv_con: &State<InventoryController>, acc_con: &State<AccountController>) -> Result<Json<InventoryReturn>, Custom<&'static str>> {
    //if(acc_con.user_has_read_access_to_inventory(params.inventory_uuid.clone(), user.user_id)?) {}
    Ok(Json(inv_con.get_inventory_parsed(params.inventory_uuid.clone())?))
}

#[put("/inventory?<params..>")]
pub async fn create_inventory(params: InventoryCreateParams,  user: super::AuthenticatedUser, inv_con: &State<InventoryController>, acc_con: &State<AccountController>) -> Result<Json<InventoryReturn>, Custom<&'static str>> {
    let inv = inv_con.insert_inventory(params.name, user.user_id.clone())?;
    get_specific_inventory(InventoryUUIDParams {inventory_uuid: inv.uuid},user, inv_con, acc_con).await
}

#[put("/inventory/item/addPreset?<params..>")]
pub async fn add_preset_to_inventory(params: InventoryAddItemByPresetParams,  _user: super::AuthenticatedUser,
        inv_con: &State<InventoryController>) -> Result<Status, Custom<&'static str>> {
    inv_con.add_preset_to_inventory(params.inventory_uuid, params.preset_uuid, params.amount)?;
    Ok(Status::NoContent)
}

#[put("/inventory/item/addNew?<params..>")]
pub async fn add_new_item_to_inventory(params:InvnetoryAddItemByNameParams,  user: super::AuthenticatedUser,
    inv_con: &State<InventoryController>) -> Result<Json<ItemPreset>, Custom<&'static str>> {
    Ok(Json(inv_con.add_new_item_to_inventory(params.inventory_uuid, params.name, params.amount, user.user_id)?))
    
}

#[patch("/inventory/item/edit?<params..>")]
pub async fn edit_item(params: ItemEditParams, user: super::AuthenticatedUser, inv_con: &State<InventoryController>) -> Result<Status, Custom<&'static str>> {
    inv_con.edit_item_amount(params.inventory_uuid, params.item_preset_uuid, params.amount)?;
    Ok(Status::NoContent)
}

#[get("/inventory/item/addNote?<params..>")]
pub async fn add_note_to_item(params: NoteAddParams, user: super::AuthenticatedUser, inv_con: &State<InventoryController>) -> Result<Status, Custom<&'static str>> {
    inv_con.edit_item_dm_note(params.inventory_uuid, params.item_preset_uuid, params.note)?;
    Ok(Status::NoContent)
}

#[get("/inventory/item/remove?<params..>")]
pub async fn delete_item_from_inventory(params: ItemDeleteParams, user: super::AuthenticatedUser,
        inv_con: &State<InventoryController>) -> Result<Status, Custom<&'static str>> {
    inv_con.delete_item_from_inventory(params.inventory_uuid, params.item_preset_uuid)?;
    Ok(Status::NoContent)
}

#[patch("/inventory/money?<params..>")]
pub async fn modify_money(params: InventoryModifyMoneyParams,  user: super::AuthenticatedUser,
        inv_con: &State<InventoryController>) -> Result<Status, Custom<&'static str>> {
    inv_con.edit_money_in_inventory(params.inventory_uuid, params.amount)?;
    Ok(Status::NoContent)
}

#[patch("/inventory/addShare?<params..>")] //TODO: Add Public
pub async fn add_share_to_inventory(params: InventoryShareParams,  user: super::AuthenticatedUser,
        inv_con: &State<InventoryController>) -> Status {
    let readers_resolved = params.reader_uuid.unwrap_or("".to_string());
    let readers = readers_resolved.split(',');
    let writers_resolved = params.writer_uuid.unwrap_or("".to_string());
    let writers = writers_resolved.split(',');
    for reader in readers {
        let _ = inv_con.add_reader_to_inventory(params.inventory_uuid.clone(), reader.to_string());
    }
    for writer in writers {
        let _ = inv_con.add_writer_to_inventory(params.inventory_uuid.clone(), writer.to_string());
    }
    Status::NoContent
}

#[patch("/inventory/removeShare?<params..>")]
pub async fn remove_share_from_inventory(params: InventoryShareParams,  user: super::AuthenticatedUser,
        inv_con: &State<InventoryController>) -> Status {
    let readers_resolved = params.reader_uuid.unwrap_or("".to_string());
    let readers = readers_resolved.split(',');
    let writers_resolved = params.writer_uuid.unwrap_or("".to_string());
    let writers = writers_resolved.split(',');
    for reader in readers {
        let _ = inv_con.remove_reader_from_inventory(params.inventory_uuid.clone(), reader.to_string());
    }
    for writer in writers {
        let _ = inv_con.remove_writer_from_inventory(params.inventory_uuid.clone(), writer.to_string());
    }
    Status::NoContent
}

#[delete("/inventory/delete?<params..>")]
pub async fn delete_inventory(params:InventoryUUIDParams,  user: super::AuthenticatedUser,
        inv_con: &State<InventoryController>) -> Result<Status, Custom<&'static str>> {
    inv_con.delete_inventory(params.inventory_uuid)?;
    Ok(Status::NoContent)
}
