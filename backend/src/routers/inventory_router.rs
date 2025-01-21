use rocket::http::Status;
use rocket::{form::FromForm, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use crate::controller::account_controller::AccountController;
use crate::controller::inventory_controller::InventoryController;
use crate::controller::{CStat, new_cstat_from_ref};
use crate::frontend_model::InventoryReturn;
use crate::model::ItemPreset;

#[derive(FromForm)]
pub struct InventoryUUIDParams {
    inventory_uuid: String
}

#[derive(Serialize, Deserialize)]
pub struct GetAllInventoriesReturn{
    inventories: Vec<InventoryReturn>
}

#[get("/inventory/all")]
pub async fn get_all_inventories(user: super::AuthenticatedUser,
        inv_con: &State<InventoryController>, acc_con: &State<AccountController>) -> Result<Json<GetAllInventoriesReturn>, CStat>  {
    Ok(Json(GetAllInventoriesReturn {
        inventories: inv_con.get_inventories_parsed(user.user_id.clone(), acc_con.user_is_dm(user.user_id.clone())?)?
    }))
}

#[get("/inventory?<params..>")]
pub async fn get_specific_inventory(params: InventoryUUIDParams,  user: super::AuthenticatedUser,
    inv_con: &State<InventoryController>, acc_con: &State<AccountController>) -> Result<Json<InventoryReturn>, CStat> {
    if !acc_con.user_has_read_access_to_inventory(params.inventory_uuid.clone(), user.user_id.clone())? {
        return Err(new_cstat_from_ref(Status::Forbidden, "Not Authorized"))
    }
    Ok(Json(inv_con.get_inventory_parsed(params.inventory_uuid.clone(), acc_con.user_is_dm(user.user_id.clone())?)?))
}

#[derive(FromForm)]
pub struct InventoryCreateParams {
    name: String
}

#[put("/inventory?<params..>")]
pub async fn create_inventory(params: InventoryCreateParams,  user: super::AuthenticatedUser, inv_con: &State<InventoryController>,
         acc_con: &State<AccountController>) -> Result<Json<InventoryReturn>, CStat> {
    let inv = inv_con.insert_inventory(params.name, user.user_id.clone())?;
    get_specific_inventory(InventoryUUIDParams {inventory_uuid: inv.uuid},user, inv_con, acc_con).await
}

#[derive(FromForm)]
pub struct InventoryAddItemByPresetParams {
    inventory_uuid: String,
    preset_uuid: String,
    amount:i32
}

#[put("/inventory/item/addPreset?<params..>")]
pub async fn add_preset_to_inventory(params: InventoryAddItemByPresetParams,  user: super::AuthenticatedUser,
        inv_con: &State<InventoryController>, acc_con: &State<AccountController>) -> Result<Status, CStat> {
    if !acc_con.user_has_write_access_to_inventory(params.inventory_uuid.clone(), user.user_id)? {
        return Err(new_cstat_from_ref(Status::Forbidden, "Not Authorized"))
    }
    inv_con.add_preset_to_inventory(params.inventory_uuid, params.preset_uuid, params.amount)?;
    Ok(Status::NoContent)
}

#[derive(FromForm)]
pub struct InventoryAddItemByNameParams {
    inventory_uuid: String,
    name: String,
    amount:i32
}

#[put("/inventory/item/addNew?<params..>")]
pub async fn add_new_item_to_inventory(params:InventoryAddItemByNameParams,  user: super::AuthenticatedUser,
        inv_con: &State<InventoryController>, acc_con: &State<AccountController>) -> Result<Json<ItemPreset>, CStat> {
    if !acc_con.user_has_write_access_to_inventory(params.inventory_uuid.clone(), user.user_id.clone())? {
        return Err(new_cstat_from_ref(Status::Forbidden, "Not Authorized"))
    }
    Ok(Json(inv_con.add_new_item_to_inventory(params.inventory_uuid, params.name, params.amount, user.user_id)?))
    
}

#[derive(FromForm)]
pub struct ItemEditParams {
    inventory_uuid: String,
    item_preset_uuid: String,
    amount: Option<i32>,
    sorting: Option<i32>,
    inventory_item_note: Option<String>
}

#[patch("/inventory/item/edit?<params..>")]
pub async fn edit_item(params: ItemEditParams, user: super::AuthenticatedUser, inv_con: &State<InventoryController>,
        acc_con: &State<AccountController>) -> Result<Status, CStat> {
    if !acc_con.user_has_write_access_to_inventory(params.inventory_uuid.clone(), user.user_id.clone())? {
        return Err(new_cstat_from_ref(Status::Forbidden, "Not Authorized"))
    }
    inv_con.edit_inventory_item(params.inventory_uuid, params.item_preset_uuid, params.amount, params.sorting, params.inventory_item_note)?;
    Ok(Status::NoContent)
}

#[derive(FromForm)]
pub struct NoteAddParams {
    item_preset_uuid: String,
    inventory_uuid: String,
    note: String
}

#[patch("/inventory/item/addNote?<params..>")]
pub async fn add_note_to_item(params: NoteAddParams, user: super::AuthenticatedUser, inv_con: &State<InventoryController>,
        acc_con: &State<AccountController>) -> Result<Status, CStat> {
    if !acc_con.user_is_dm(user.user_id.clone())? {
        return Err(new_cstat_from_ref(Status::Forbidden, "Not Authorized"))
    }
    inv_con.edit_item_dm_note(params.inventory_uuid, params.item_preset_uuid, params.note)?;
    Ok(Status::NoContent)
}

#[derive(FromForm)]
pub struct ItemDeleteParams {
    inventory_uuid: String,
    item_preset_uuid: String
}

#[delete("/inventory/item/remove?<params..>")]
pub async fn delete_item_from_inventory(params: ItemDeleteParams, user: super::AuthenticatedUser,
        inv_con: &State<InventoryController>, acc_con: &State<AccountController>) -> Result<Status, CStat> {
    if !acc_con.user_has_write_access_to_inventory(params.inventory_uuid.clone(), user.user_id.clone())? {
        return Err(new_cstat_from_ref(Status::Forbidden, "Not Authorized"))
    }
    inv_con.delete_item_from_inventory(params.inventory_uuid, params.item_preset_uuid)?;
    Ok(Status::NoContent)
}

#[derive(Debug, FromForm)]
pub struct InventoryModifyMoneyParams {
    inventory_uuid:String,
    amount: i32
}

#[patch("/inventory/money?<params..>")]
pub async fn modify_money(params: InventoryModifyMoneyParams,  user: super::AuthenticatedUser,
        inv_con: &State<InventoryController>, acc_con: &State<AccountController>) -> Result<Status, CStat> {
    if !acc_con.user_has_write_access_to_inventory(params.inventory_uuid.clone(), user.user_id.clone())? {
        return Err(new_cstat_from_ref(Status::Forbidden, "Not Authorized"))
    }
    inv_con.edit_money_in_inventory(params.inventory_uuid, params.amount)?;
    Ok(Status::NoContent)
}

#[derive(FromForm)]
pub struct InventoryShareParams {
    inventory_uuid: String,
    reader_uuid: Option<String>,
    writer_uuid: Option<String>
}

#[patch("/inventory/addShare?<params..>")] //TODO: Add Public
pub async fn add_share_to_inventory(params: InventoryShareParams,  user: super::AuthenticatedUser,
        inv_con: &State<InventoryController>) -> Result<Status, CStat> {
    if !inv_con.is_creator_of_inventory(params.inventory_uuid.clone(), user.user_id.clone())? {
        return Err(new_cstat_from_ref(Status::Forbidden, "Not Authorized"));
    }
    let reader = params.reader_uuid;
    let writer = params.writer_uuid;
    if let Some(reader) = reader {
        let _ = inv_con.add_reader_to_inventory(params.inventory_uuid.clone(), reader)?;
    }
    if let Some(writer) = writer {
        let _ = inv_con.add_writer_to_inventory(params.inventory_uuid.clone(), writer)?;
    }
    Ok(Status::NoContent)
}

#[patch("/inventory/removeShare?<params..>")]
pub async fn remove_share_from_inventory(params: InventoryShareParams,  user: super::AuthenticatedUser,
        inv_con: &State<InventoryController>, acc_con: &State<AccountController>) -> Result<Status, CStat> {
    if !inv_con.is_creator_of_inventory(params.inventory_uuid.clone(), user.user_id.clone())? {
        return Err(new_cstat_from_ref(Status::Forbidden, "Not Authorized"));
    }
    let readers_resolved = params.reader_uuid.unwrap_or("".to_string());
    let readers = readers_resolved.split(',');
    let writers_resolved = params.writer_uuid.unwrap_or("".to_string());
    let writers = writers_resolved.split(',');
    for reader in readers {
        if inv_con.is_creator_of_inventory(params.inventory_uuid.clone(), user.user_id.clone())? {continue;}
        let _ = inv_con.remove_reader_from_inventory(params.inventory_uuid.clone(), reader.to_string());
    }
    for writer in writers {
        if inv_con.is_creator_of_inventory(params.inventory_uuid.clone(), user.user_id.clone())? {continue;}
        if acc_con.user_is_dm(user.user_id.clone())? {continue;}
        let _ = inv_con.remove_writer_from_inventory(params.inventory_uuid.clone(), writer.to_string());
    }
    Ok(Status::NoContent)
}

#[delete("/inventory/delete?<params..>")]
pub async fn delete_inventory(params:InventoryUUIDParams,  user: super::AuthenticatedUser,
        inv_con: &State<InventoryController>) -> Result<Status, CStat> {
    if !inv_con.is_creator_of_inventory(params.inventory_uuid.clone(), user.user_id.clone())? {
        return Err(new_cstat_from_ref(Status::Forbidden, "Not Authorized"));
    }
    inv_con.delete_inventory(params.inventory_uuid)?;
    Ok(Status::NoContent)
}
