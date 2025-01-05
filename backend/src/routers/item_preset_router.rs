use rocket::{form::FromForm, http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use crate::controller::{CStat, new_cstat_from_ref};
use crate::{controller::item_preset_controller::ItemPresetController, model::ItemPreset};
use crate::controller::inventory_controller::InventoryController;


#[derive(Serialize, Deserialize)]
pub struct GetItemPresetReturn{
    item_presets: Vec<ItemPreset>
}

#[derive(FromForm)]
pub struct ItemPresetUUIDParams {
    item_preset_uuid: String
}

#[derive(FromForm)]
pub struct ItemModifyParams {
    item_preset_uuid: String,
    name: Option<String>,
    price:Option<i32>,
    description: Option<String>,
    item_type: Option<String>
}

fn has_access_to(searched_item_preset: String, inventories: Vec<String>, inv_con: &State<InventoryController>) -> Result<bool, CStat> {
    let mut has_access = false;
    for i in inventories {
        if inv_con.item_exits(i, searched_item_preset.clone())? {
            has_access = true
        }
    }
    return Ok(has_access)
}

#[get("/itemPreset?<params..>")]
pub async fn get_item_preset(params: ItemPresetUUIDParams,  user: super::AuthenticatedUser, ipc_con: &State<ItemPresetController>,
        inv_con: &State<InventoryController>) -> Result<Json<ItemPreset>, CStat> {
    let invs = inv_con.get_all_inventories_ids(user.user_id)?;
    
    if !has_access_to(params.item_preset_uuid.clone(), invs, inv_con)? {
        return Err(new_cstat_from_ref(Status::Forbidden, "No access"));
    }
    Ok(Json(ipc_con.get_item_preset(params.item_preset_uuid)?))
}

#[patch("/itemPreset/modify?<params..>")]
pub async fn modify_item_preset(params: ItemModifyParams,  user: super::AuthenticatedUser,
        ipc_con: &State<ItemPresetController>, inv_con: &State<InventoryController>) -> Result<Status, CStat> {
    let invs = inv_con.get_all_inventories_ids_with_read_access(user.user_id)?;
    if !has_access_to(params.item_preset_uuid.clone(), invs, inv_con)? {
        return Err(new_cstat_from_ref(Status::Forbidden, "No access"));
    }
    ipc_con.edit_item_preset(params.item_preset_uuid, params.name, params.price, params.description, params.item_type)?;
    Ok(Status::NoContent)
}

#[patch("/itemPreset/delete?<params..>")]
pub async fn delete_item_preset(params: ItemPresetUUIDParams,  user: super::AuthenticatedUser,
        ipc_con: &State<ItemPresetController>, inv_con: &State<InventoryController>) -> Result<Status, CStat> {
    let invs = inv_con.get_all_inventories_ids_with_read_access(user.user_id)?;
    if !has_access_to(params.item_preset_uuid.clone(), invs, inv_con)? {
        return Err(new_cstat_from_ref(Status::Forbidden, "No access"));
    }

    ipc_con.delete_item_preset(params.item_preset_uuid)?;
    Ok(Status::NoContent)
}

#[patch("/itemPreset/all")]
pub async fn get_all_item_presets(user: super::AuthenticatedUser, inv_con: &State<InventoryController>,
        ipc_con: &State<ItemPresetController>) -> Result<Json<GetItemPresetReturn>, CStat> {
    let mut ips: Vec<ItemPreset> = Vec::new();
    let invs = inv_con.get_all_inventories_ids(user.user_id)?;
    for i in invs {
        ips.extend(ipc_con.get_item_preset_in_inventory(i)?)
    }
    Ok(Json(
        GetItemPresetReturn {
            item_presets: ips
        }
    ))
}