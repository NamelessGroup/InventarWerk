use rocket::form::FromForm;

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

#[patch("/item/edit?<params..>")]
pub async fn edit_item(params: ItemEditParams) -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}

#[get("/item/addNote?<params..>")]
pub async fn add_note_to_item(params: NoteAddParams) -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}