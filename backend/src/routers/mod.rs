use item_preset_router::{get_item_preset, modify_item_preset};
use item_router::edit_item;
use last_changes_router::last_changes;
use rocket::{Build, Rocket, Route};

pub mod account_router;
pub mod inventory_router;
pub mod item_preset_router;
pub mod item_router;
pub mod note_router;
pub mod last_changes_router;
use inventory_router::*;
use account_router::*;
use item_preset_router::*;
use item_router::*;
use note_router::*;
use last_changes_router::*;

pub fn get_inventory_routes() -> Vec<Route> {
    routes![get_all_inventories, get_specific_inventory, create_inventory, add_preset_to_inventory, add_new_item_to_inventory,
        modify_money, share_inventory, delete_inventory]
}

pub fn get_account_routes() -> Vec<Route> {
    routes![get_accounts, is_account_dm, callback, login, account_info]
}

pub fn get_last_changes_routes() -> Vec<Route> {
    routes![last_changes]
}

pub fn get_item_routes() -> Vec<Route> {
    routes![edit_item]
}

pub fn get_item_preset_routes() -> Vec<Route> {
    routes![get_item_preset, modify_item_preset]
}

pub fn get_note_routes() -> Vec<Route> {
    routes![add_note_to_item]
}