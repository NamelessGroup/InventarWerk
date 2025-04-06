#[macro_use] extern crate rocket;

mod routers;
mod last_changes_map_macro;

use openssl::rand::rand_bytes;
use rocket::fs::{FileServer, relative};
use dotenvy::dotenv;
use std::env;
use rocket::config::Config;
use inv_rep::DbPool;
use inv_rep::repos::inventory_repository::InventoryRepository;
use inv_rep::repos::item_preset_repository::ItemPresetRepository;
use inv_rep::repos::user_repository::UserRepository;
use inv_rep::create_pg_pool;

#[rocket::main]
async fn main() {
    dotenv().ok();
    
    let dbconn:DbPool = create_pg_pool(env::var("DATABASE_URL").expect("Database url must be set")).await.expect("Couldn't connect to database");




    let inv_rep = InventoryRepository::new(dbconn.clone());
    let acc_rep = UserRepository::new(dbconn.clone());
    let ipr_rep = ItemPresetRepository::new(dbconn.clone());

    let mut secret_key = [0u8;32];
    let _ = rand_bytes(&mut secret_key);

    let figment = Config::figment().merge(("secret_key", secret_key));
    let config = Config::from(figment);    

    let mut r = rocket::build();
    r = r.configure(config)
        .manage(inv_rep)
        .manage(acc_rep)
        .manage(ipr_rep)
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
