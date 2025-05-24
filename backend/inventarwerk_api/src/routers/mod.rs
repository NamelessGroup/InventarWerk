use rocket::http::Status;
use rocket::request::Outcome;
use rocket::Request;
use rocket::{request::FromRequest, Route};

use anyhow::anyhow;
use rocket_errors::anyhow::AnyhowError;

pub mod account_router;
pub mod inventory_router;
pub mod item_preset_router;
pub mod last_changes_router;
mod router_utility;

use account_router::*;
use inventory_router::*;
use item_preset_router::*;
use last_changes_router::*;

/// Returns all inventory-related routes.
pub fn get_inventory_routes() -> Vec<Route> {
    routes![
        get_all_inventories,
        get_specific_inventory,
        create_inventory,
        add_preset_to_inventory,
        add_new_item_to_inventory,
        edit_inventory,
        add_share_to_inventory,
        remove_share_from_inventory,
        delete_inventory,
        edit_item,
        delete_item_from_inventory,
        add_note_to_item
    ]
}

/// Returns all account-related routes.
pub fn get_account_routes() -> Vec<Route> {
    routes![
        get_accounts,
        is_account_dm,
        callback,
        login,
        account_info,
        user_logged_in,
        logout,
        is_locked,
        toggle_lock
    ]
}

/// Returns all last-changes-related routes.
pub fn get_last_changes_routes() -> Vec<Route> {
    routes![last_changes]
}

/// Returns all item preset-related routes.
pub fn get_item_preset_routes() -> Vec<Route> {
    routes![
        get_item_preset,
        modify_item_preset,
        delete_item_preset,
        get_all_item_presets,
        add_extern
    ]
}

/// Extractor for authenticated users based on a private `user_id` cookie.
pub struct AuthenticatedUser {
    /// The user id of the authenticated user
    pub user_id: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();

        if let Some(cookie) = cookies.get_private("user_id") {
            let user_id = cookie.value().to_string();
            Outcome::Success(AuthenticatedUser { user_id })
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}

/// Helper to create an error for API responses.
fn create_error(msg: &str) -> AnyhowError {
    anyhow!(msg.to_string()).into()
}
