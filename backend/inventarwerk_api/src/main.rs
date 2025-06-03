#[macro_use]
extern crate rocket;

mod last_changes_map_macro;
mod locked_macros;
mod routers;

use dotenvy::dotenv;
use openssl::rand::rand_bytes;
use repos::create_pg_pool;
use repos::repos::inventory_repository::InventoryRepository;
use repos::repos::item_preset_repository::ItemPresetRepository;
use repos::repos::user_repository::UserRepository;
use repos::DbPool;
use rocket::config::Config;
use rocket::fs::FileServer;
use std::env;

use routers::account_router::AccountApiDoc;
use routers::inventory_router::InventoryApiDoc;
use routers::item_preset_router::ItemPresetApiDoc;
use routers::last_changes_router::LastChangesApiDoc;

use utoipa::OpenApi;

use utoipa_swagger_ui::SwaggerUi;

/// Main async entry point for the backend server.
#[rocket::main]
async fn main() {
    dotenv().ok();

    let dbconn: DbPool =
        create_pg_pool(env::var("DATABASE_URL").expect("Database url must be set"))
            .await
            .expect("Couldn't connect to database");

    let inv_rep = InventoryRepository::new(dbconn.clone());
    let usr_rep = UserRepository::new(dbconn.clone());
    let ipr_rep = ItemPresetRepository::new(dbconn.clone());

    let mut secret_key = [0u8; 32];
    let _ = rand_bytes(&mut secret_key);

    if !usr_rep
        .any_user_exists()
        .await
        .expect("DB failed during startup, can not recover from this.")
    {
        lock_toggle!();
    }

    let figment = Config::figment().merge(("secret_key", secret_key));
    let config = Config::from(figment);

    let mut r = rocket::build()
        .configure(config)
        .manage(inv_rep)
        .manage(usr_rep)
        .manage(ipr_rep)
        .mount("/", FileServer::from("/usr/src/app/static"))
        .mount("/", routers::get_account_routes())
        .mount("/", routers::get_inventory_routes())
        .mount("/", routers::get_item_preset_routes())
        .mount("/", routers::get_last_changes_routes())
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>")
                .url(
                    "/api-docs/openapi_inventory.json",
                    InventoryApiDoc::openapi(),
                )
                .url(
                    "/api-docs/openapi_item_preset.json",
                    ItemPresetApiDoc::openapi(),
                )
                .url("/api-docs/openapi_account.json", AccountApiDoc::openapi())
                .url(
                    "/api-docs/openapi_last_changes.json",
                    LastChangesApiDoc::openapi(),
                ),
        );

    #[cfg(any(feature = "dev", feature = "dev-deploy"))]
    {
        println!("Starting with CORS.");
        println!("Only do this in development.");
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
