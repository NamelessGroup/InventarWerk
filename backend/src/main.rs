#[macro_use] extern crate rocket;

mod routers;
mod controller;
mod dbmod;
mod model;
mod schema;
mod frontend_model;
mod last_changes_map_macro;


use controller::account_controller::AccountController;
use controller::item_preset_controller::ItemPresetController;
use diesel::RunQueryDsl;
use openssl::rand::rand_bytes;
use rocket::fs::{FileServer, relative};
use dotenvy::dotenv;
use std::env;
use controller::inventory_controller::InventoryController;
use dbmod::DbPool;
use dbmod::establish_connection;
use rocket::config::Config;

#[rocket::main]
async fn main() {
    dotenv().ok();
    
    let dbconn:DbPool = establish_connection();

    let mut conn = dbconn.get().expect("Failed to get connection from pool");


    
    diesel::sql_query("PRAGMA journal_mode = WAL;")
        .execute(&mut conn)
        .expect("Failed to set journal mode");

    let inv_cont = InventoryController::new(dbconn.clone());
    let acc_con = AccountController::new(dbconn.clone());
    let ip_con = ItemPresetController::new(dbconn.clone());

    let mut secret_key = [0u8;32];
    let _ = rand_bytes(&mut secret_key);

    let figment = Config::figment().merge(("secret_key", secret_key));
    let config = Config::from(figment);    

    let mut r = rocket::build();
    r = r.configure(config)
        .manage(inv_cont)
        .manage(acc_con)
        .manage(ip_con)
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routers::get_account_routes())
        .mount("/", routers::get_inventory_routes())
        .mount("/", routers::get_item_preset_routes())
        .mount("/", routers::get_last_changes_routes());

    #[cfg(any(feature = "dev", feature="dev-deploy"))] {
        println!("Starting with CORS.\nOnly do this in development.");
        use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
         // Configure CORS
        let cors = CorsOptions {
            allowed_origins: AllowedOrigins::all(), // Allow all origins, or customize this
            allowed_methods: vec!["GET", "POST", "PUT", "DELETE", "OPTIONS", "PATCH"]
                .into_iter()
                .map(|method| method.parse().unwrap())
                .collect(),
            allowed_headers: AllowedHeaders::all(), // Allow all headers, or customize this
            allow_credentials: true,
            ..Default::default()
        }
        .to_cors()
        .expect("Error configuring CORS");
        
        
        r = r.attach(cors); // Attach the CORS fairing
    }
    let _res = r.launch().await;
}
