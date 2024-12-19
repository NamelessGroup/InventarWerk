#[macro_use] extern crate rocket;

mod routers;
mod controller;
mod dbmod;

use rocket::fs::{FileServer, relative};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use controller::inventoryController::InventoryController;
use dbmod::DbPool;
use dbmod::establish_connection;

#[rocket::main]
async fn main() {
    let dbconn:DbPool = establish_connection();

    let invCont = InventoryController::new(dbconn);
    rocket::build()
        .manage(invCont)
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routers::get_inventory_routes())
        .launch().await;
}