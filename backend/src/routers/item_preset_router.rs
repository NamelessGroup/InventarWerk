
use rocket::{form::FromForm, http::Status, response::status::Custom, serde::json::Json, State};
use crate::{controller::item_preset_controller::ItemPresetController, model::ItemPreset};

use super::transform_to_http_error;
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
//TODO: Access controll
#[get("/itemPreset?<params..>")]
pub async fn get_item_preset(params: ItemPresetUUIDParams,  user: super::AuthenticatedUser,
        ipc_con: &State<ItemPresetController>) -> Result<Json<ItemPreset>, Custom<&'static str>> {
    transform_to_http_error(ipc_con.get_item_preset(params.item_preset_uuid), Status::InternalServerError)
}

#[patch("/itemPreset/modify?<params..>")]
pub async fn modify_item_preset(params: ItemModifyParams,  user: super::AuthenticatedUser,
        ipc_con: &State<ItemPresetController>) -> Result<Status, Custom<&'static str>> {
    match ipc_con.edit_item_preset(params.item_preset_uuid, params.name, params.price, params.description, params.item_type){
        Ok(_res) => Ok(Status::NoContent),
        Err(e) => Err(Custom(
            Status::InternalServerError,
            e
        ))
    }
}

#[patch("/itemPreset/delete?<params..>")]
pub async fn delete_item_preset(params: ItemPresetUUIDParams,  user: super::AuthenticatedUser,
        ipc_con: &State<ItemPresetController>) -> Result<Status, Custom<&'static str>> {
    match ipc_con.delete_item_preset(params.item_preset_uuid) {
        Ok(_res) => Ok(Status::NoContent),
        Err(e) => Err(Custom(
            Status::InternalServerError,
            e
        ))
    }
}

#[patch("/itemPreset/all")]
pub async fn get_all_item_presets(user: super::AuthenticatedUser) -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}