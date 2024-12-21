
use rocket::form::FromForm;

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

#[get("/itemPreset?<params..>")]
pub async fn get_item_preset(params: ItemPresetUUIDParams) -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}

#[patch("/itemPreset/modify?<params..>")]
pub async fn modify_item_preset(params: ItemModifyParams) -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}

#[patch("/itemPreset/delete?<params..>")]
pub async fn delete_item_preset(params: ItemPresetUUIDParams) -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}

#[patch("/itemPreset/all")]
pub async fn get_all_item_preset() -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}