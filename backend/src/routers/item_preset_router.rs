use std::thread;
use std::time::Duration;

use rocket::{form::FromForm, http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use crate::controller::{new_cstat_from_ref, CStat};
use crate::frontend_model::FrontendItemPreset;
use crate::{controller::item_preset_controller::ItemPresetController, model::ItemPreset};
use crate::controller::inventory_controller::InventoryController;


#[derive(Serialize, Deserialize)]
pub struct GetItemPresetReturn{
    item_presets: Vec<FrontendItemPreset>
}

#[derive(FromForm)]
pub struct ItemPresetUUIDParams {
    item_preset_uuid: String
}


fn convert_item_preset_to_frontend(preset: ItemPreset) -> FrontendItemPreset {
    FrontendItemPreset {
        uuid: preset.uuid,
        name: preset.name,
        price: preset.price,
        weight: preset.weight,
        description: preset.description,
        creator: preset.creator,
        itemType: preset.item_type
    }
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
        inv_con: &State<InventoryController>) -> Result<Json<FrontendItemPreset>, CStat> {
    let invs = inv_con.get_all_inventories_ids(user.user_id)?;
    let preset = ipc_con.get_item_preset(params.item_preset_uuid.clone())?;
    if !preset.creator.starts_with("public") && !has_access_to(params.item_preset_uuid.clone(), invs, inv_con)? {
        return Err(new_cstat_from_ref(Status::Forbidden, "No access"));
    }
    Ok(Json(convert_item_preset_to_frontend(preset)))
}

#[derive(FromForm)]
pub struct ItemModifyParams {
    item_preset_uuid: String,
    name: Option<String>,
    price:Option<i32>,
    weight: Option<f32>,
    description: Option<String>,
    item_type: Option<String>
}

#[patch("/itemPreset/modify?<params..>")]
pub async fn modify_item_preset(params: ItemModifyParams,  user: super::AuthenticatedUser,
        ipc_con: &State<ItemPresetController>) -> Result<Status, CStat> {
    let preset = ipc_con.get_item_preset(params.item_preset_uuid.clone())?;
    if preset.creator != user.user_id {
        return Err(new_cstat_from_ref(Status::Forbidden, "No access"))
    }
    ipc_con.edit_item_preset(params.item_preset_uuid, params.name, params.price, params.weight, params.description, params.item_type)?;
    Ok(Status::NoContent)
}

#[delete("/itemPreset/delete?<params..>")]
pub async fn delete_item_preset(params: ItemPresetUUIDParams,  user: super::AuthenticatedUser,
        ipc_con: &State<ItemPresetController>, inv_con: &State<InventoryController>) -> Result<Status, CStat> {
    let invs = inv_con.get_all_inventories_ids_with_read_access(user.user_id)?;
    if !has_access_to(params.item_preset_uuid.clone(), invs, inv_con)? {
        return Err(new_cstat_from_ref(Status::Forbidden, "No access"));
    }

    ipc_con.delete_item_preset(params.item_preset_uuid)?;
    Ok(Status::NoContent)
}

#[get("/itemPreset/all")]
pub async fn get_all_item_presets(user: super::AuthenticatedUser, inv_con: &State<InventoryController>,
        ipc_con: &State<ItemPresetController>) -> Result<Json<GetItemPresetReturn>, CStat> {
    let mut item_presets = ipc_con.get_public_item_presets()?;
    let mut frontend_presets: Vec<FrontendItemPreset> = Vec::new();
    let invs = inv_con.get_all_inventories_ids(user.user_id)?;
    for i in invs {
        item_presets.extend(ipc_con.get_item_preset_in_inventory(i)?);
    }
    for i in item_presets{
        frontend_presets.push(convert_item_preset_to_frontend(i));
    }
    Ok(Json(
        GetItemPresetReturn {
            item_presets: frontend_presets
        }
    ))
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ExternPresetData {
    name: String,
    uuid: String,
    price: i32,
    weight: f32,
    description: String,
    creator: String,
    itemType: String
}

#[derive(Serialize, Deserialize)]
pub struct ExternPresetDataList {
    presets: Vec<ExternPresetData>
}

#[put("/itemPreset/addExtern", data="<json_data>")]
pub async fn add_extern(json_data: Json<ExternPresetDataList>, _user: super::AuthenticatedUser, ipc_con: &State<ItemPresetController>)
    -> Result<Status, CStat>  {
    for x in &json_data.presets {
        loop {
            let res = ipc_con.add_extern_preset(x.name.clone(), x.price, x.weight, x.description.clone(), x.creator.clone(), x.itemType.clone());
            match res {
                Ok(_res) => break,
                Err(e) => {
                    if !(e.0 == Status::InternalServerError && e.1.contains("locked")) {
                        println!("Didn't added {} to the Database, why did this happen?", x.name);
                        break;
                    }
                    thread::sleep(Duration::from_secs(1));
                }

            }
        
        }
    }
    
    Ok(Status::NoContent)
}
