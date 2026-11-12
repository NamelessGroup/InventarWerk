#[macro_use]
extern crate rocket;

mod routers;

use dotenvy::dotenv;
use openssl::rand::rand_bytes;

use repos::repos::zauberwek::concentration_repository::ConcentrationRepository;
use repos::repos::zauberwek::spell_attack_modifier_repository::SpellAttackModifierRepository;
use repos::repos::zauberwek::spell_list_repository::SpellListRepository;
use repos::repos::zauberwek::spell_preset_repository::SpellPresetRepository;
use repos::repos::user_repository::UserRepository;
use repos::repos::zauberwek::spell_slot_repository::SpellSlotRepository;
use repos::repos::zauberwek::spell_cast_repository::SpellCastRepository;
use rocket::tokio::sync::broadcast::channel;
use rocket::config::Config;
use rocket::fs::FileServer;
use std::env;

use repos::DbPool;
use repos::create_pg_pool;

use routers::concentration_router::ConcentrationApiDoc;
use routers::spell_attack_modifier_router::SpellAttackModifierApiDoc;
use routers::spell_list_router::SpellListApiDoc;
use routers::spell_preset_router::SpellPresetApiDoc;
use routers::spell_slot_router::SpellSlotApiDoc;
use routers::spell_cast_router::{SpellCastApiDoc, SpellEvent};

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

    let spp_rep = SpellPresetRepository::new(dbconn.clone());
    let usr_rep = UserRepository::new(dbconn.clone());
    let con_rep = ConcentrationRepository::new(dbconn.clone());
    let sam_rep = SpellAttackModifierRepository::new(dbconn.clone());
    let ssl_rep = SpellSlotRepository::new(dbconn.clone());
    let sli_rep = SpellListRepository::new(dbconn.clone());
    let cast_rep = SpellCastRepository::new(dbconn.clone());

    // Broadcast channel for the spell cast SSE stream
    let (cast_tx, _cast_rx) = channel::<SpellEvent>(1024);


    let mut figment = Config::figment().merge(("secret_key", secret_key));

    #[cfg(any(feature = "dev"))]
    {
        figment = figment.merge(("address", "127.0.0.1"))
        .merge(("port", 8001));
    }

    let config = Config::from(figment);
    #[allow(unused_mut)]
    let mut r = rocket::build()
        .configure(config)
        .manage(spp_rep)
        .manage(usr_rep)
        .manage(con_rep)
        .manage(sam_rep)
        .manage(ssl_rep)
        .manage(sli_rep)
        .manage(cast_rep)
        .manage(cast_tx)
        .mount("/", FileServer::from("./static"))
        .mount("/", routers::get_spell_list_routes())
        .mount("/", routers::get_concentration_routes())
        .mount("/", routers::get_spell_attack_modifier_routes())
        .mount("/", routers::get_spell_preset_routes())
        .mount("/", routers::get_spell_slot_routes())
        .mount("/", routers::get_spell_cast_routes())
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>")
                .url("/api-docs/openapi_spell_list.json", SpellListApiDoc::openapi())
                .url("/api-docs/openapi_spell_preset.json", SpellPresetApiDoc::openapi())
                .url("/api-docs/openapi_spell_slot.json", SpellSlotApiDoc::openapi())
                .url("/api-docs/openapi_concentration.json", ConcentrationApiDoc::openapi())
                .url("/api-docs/openapi_spell_attack_modifier.json", SpellAttackModifierApiDoc::openapi())
                .url("/api-docs/openapi_spell_cast.json", SpellCastApiDoc::openapi()),
        );

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
