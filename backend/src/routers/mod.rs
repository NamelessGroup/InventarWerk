use rocket::request::Outcome;
use rocket::{request::FromRequest, Route};
use rocket::Request;
use rocket::http::Status;


pub mod account_router;
pub mod inventory_router;
pub mod item_preset_router;
pub mod last_changes_router;
use inventory_router::*;
use account_router::*;
use item_preset_router::*;
use last_changes_router::*;

pub fn get_inventory_routes() -> Vec<Route> {
    routes![get_all_inventories, get_specific_inventory, create_inventory, add_preset_to_inventory, add_new_item_to_inventory,
        modify_money, add_share_to_inventory, remove_share_from_inventory, delete_inventory, edit_item, delete_item_from_inventory, add_note_to_item]
}

pub fn get_account_routes() -> Vec<Route> {
    routes![get_accounts, is_account_dm, callback, login, account_info, user_logged_in]
}

pub fn get_last_changes_routes() -> Vec<Route> {
    routes![last_changes]
}

pub fn get_item_preset_routes() -> Vec<Route> {
    routes![get_item_preset, modify_item_preset, delete_item_preset, get_all_item_presets]
}


pub struct AuthenticatedUser {
    pub user_id: String
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();

        if let Some(cookie) = cookies.get_private("user_id") {
            let user_id = cookie.value().to_string();
            Outcome::Success(AuthenticatedUser {user_id})
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}