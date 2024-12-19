use rocket::form::FromForm;

#[derive(FromForm)]
pub struct AccountUUIDParams {
    itempreset_uuid: String
}

#[get("/account")]
pub async fn get_accounts() -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}

#[get("/account/isDm?<params..>")]
pub async fn is_account_dm(params: AccountUUIDParams) -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}