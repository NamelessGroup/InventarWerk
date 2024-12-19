use rocket::form::FromForm;

#[derive(FromForm)]
pub struct ItemEditParams {
    item_uuid: String,
    name: Option<String>,
    amount: Option<String>,
    description: Option<String>
}

#[patch("/item/edit?<params..>")]
pub async fn edit_item(params: ItemEditParams) -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}
