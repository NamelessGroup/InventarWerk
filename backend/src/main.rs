#[macro_use] extern crate rocket;

mod routers;
mod controller;

use rocket::fs::{FileServer, relative};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;




#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routers::get_routes())
        .launch().await;
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}