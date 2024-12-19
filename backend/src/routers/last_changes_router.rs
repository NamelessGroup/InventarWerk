use rocket::form::FromForm;

#[derive(FromForm)]
pub struct LastChangesParams {
    timestamp: i32
}

#[get("/lastChanges?<params..>")]
pub async fn last_changes(params: LastChangesParams) -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}