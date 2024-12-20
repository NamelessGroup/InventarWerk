#[macro_use] extern crate rocket;

mod routers;
mod controller;
mod dbmod;

use openssl::rand::rand_bytes;
use rocket::figment;
use rocket::fs::{FileServer, relative};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use controller::inventoryController::InventoryController;
use dbmod::DbPool;
use dbmod::establish_connection;
use rocket::config::Config;

#[rocket::main]
async fn main() {
    dotenv().ok();
    let dbconn:DbPool = establish_connection();

    let invCont = InventoryController::new(dbconn);

    let mut secret_key = [0u8;32];
    rand_bytes(&mut secret_key);

    let figment = Config::figment().merge(("secret_key", secret_key));
    let config = Config::from(figment);

    let _r = rocket::build()
        .configure(config)
        .manage(invCont)
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routers::get_inventory_routes())
        .mount("/", routers::get_account_routes())
        .launch().await;
}