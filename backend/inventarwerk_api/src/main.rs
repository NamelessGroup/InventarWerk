#[macro_use]
extern crate rocket;

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
use std::env;

use routers::account_router::AccountApiDoc;
use routers::backup_router::BackupApiDoc;
use routers::inventory_router::InventoryApiDoc;
use routers::item_preset_router::ItemPresetApiDoc;

use utoipa::OpenApi;

use utoipa_swagger_ui::SwaggerUi;

/// Builds the Swagger UI of this service.
///
/// In production all services are served behind one origin (see `frontend/nginx.conf`),
/// so this Swagger UI additionally lists the API docs of `zauberwerk_api` and
/// `last_changes_api`. In development every service runs on its own port and
/// only the own API docs are listed.
fn swagger_ui() -> SwaggerUi {
    let swagger = SwaggerUi::new("/swagger-ui/<_..>")
        .url("/api-docs/openapi_inventory.json", InventoryApiDoc::openapi())
        .url("/api-docs/openapi_item_preset.json", ItemPresetApiDoc::openapi())
        .url("/api-docs/openapi_account.json", AccountApiDoc::openapi())
        .url("/api-docs/openapi_backup.json", BackupApiDoc::openapi());

    #[cfg(not(feature = "dev"))]
    let swagger = {
        use utoipa_swagger_ui::{Config as SwaggerConfig, Url};
        swagger.config(SwaggerConfig::new([
            // inventarwerk_api
            Url::new("InventarWerk: Inventory", "/api-docs/openapi_inventory.json"),
            Url::new("InventarWerk: Item Preset", "/api-docs/openapi_item_preset.json"),
            Url::new("InventarWerk: Account", "/api-docs/openapi_account.json"),
            Url::new("InventarWerk: Backup", "/api-docs/openapi_backup.json"),
            // zauberwerk_api
            Url::new("ZauberWerk: Spell List", "/api-docs/openapi_spell_list.json"),
            Url::new("ZauberWerk: Spell Preset", "/api-docs/openapi_spell_preset.json"),
            Url::new("ZauberWerk: Spell Slot", "/api-docs/openapi_spell_slot.json"),
            Url::new("ZauberWerk: Concentration", "/api-docs/openapi_concentration.json"),
            Url::new("ZauberWerk: Spell Attack Modifier", "/api-docs/openapi_spell_attack_modifier.json"),
            Url::new("ZauberWerk: Spell Cast", "/api-docs/openapi_spell_cast.json"),
            // last_changes_api
            Url::new("Last Changes", "/api-docs/openapi_last_changes.json"),
        ]))
    };

    swagger
}

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

    // Use shared secret key from environment variable or generate one
    let mut secret_key = [0u8; 32];
    if let Ok(key_str) = env::var("SECRET_KEY") {
        let key_bytes = key_str.as_bytes();
        let len = key_bytes.len().min(32);
        secret_key[..len].copy_from_slice(&key_bytes[..len]);
    } else {
        eprintln!("WARNING: SECRET_KEY not set, generating random key. Sessions won't work across API restarts!");
        let _ = rand_bytes(&mut secret_key);
    }

    if !usr_rep
        .any_user_exists()
        .await
        .expect("DB failed during startup, can not recover from this.")
    {
        lock_toggle!();
    }

    let mut figment = Config::figment().merge(("secret_key", secret_key));

    #[cfg(any(feature = "dev"))]
    {
        figment = figment.merge(("address", "127.0.0.1"))
        .merge(("port", 8000));
    }

    let config = Config::from(figment);
    #[allow(unused_mut)]
    let mut r = rocket::build()
        .configure(config)
        .manage(inv_rep)
        .manage(usr_rep)
        .manage(ipr_rep)
        .manage(dbconn)
        .mount("/", routers::get_account_routes())
        .mount("/", routers::get_inventory_routes())
        .mount("/", routers::get_item_preset_routes())
        .mount("/", routers::get_backup_routes())
        .mount("/", swagger_ui());

    #[cfg(any(feature = "dev"))]
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
