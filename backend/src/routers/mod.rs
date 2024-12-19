use rocket::{Build, Rocket, Route};

pub mod accountRouter;
pub mod inventoryRouter;
pub mod itemPresetRouter;
pub mod itemRouter;
use inventoryRouter::*;

pub fn get_inventory_routes() -> Vec<Route> {
    routes![getAllInventories, getSpecificInventory, createInventory, addPresetToInventory, addNewItemToInventory, modifyMoney,
        shareInventoryWithAll, shareInventory, deleteInventory]
}