use rocket::form::FromForm;

#[derive(FromForm)]
pub struct NoteAddParams {
    item_uuid: String,
    note: String
}

#[get("/note/add?<params..>")]
pub async fn add_note_to_item(params: NoteAddParams) -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}