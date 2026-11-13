
use rocket::Route;


pub mod account_router;
pub mod backup_router;
pub mod inventory_router;
pub mod item_preset_router;
mod router_utility;

use account_router::*;
use backup_router::*;
use inventory_router::*;
use item_preset_router::*;

/// Returns all inventory-related routes.
pub fn get_inventory_routes() -> Vec<Route> {
    routes![
        get_all_inventories,
        get_specific_inventory,
        create_inventory,
        add_preset_to_inventory,
        add_new_item_to_inventory,
        edit_inventory,
        set_inventory_share_permissions,
        delete_inventory,
        edit_item,
        delete_item_from_inventory,
        move_item_between_inventories,
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

/// Returns all backup-related routes.
pub fn get_backup_routes() -> Vec<Route> {
    routes![create_database_backup]
}



