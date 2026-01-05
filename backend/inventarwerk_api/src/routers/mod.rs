
use rocket::Route;


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



