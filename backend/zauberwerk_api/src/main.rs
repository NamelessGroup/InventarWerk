#[macro_use]
extern crate rocket;

mod routers;

use dotenvy::dotenv;
use openssl::rand::rand_bytes;

use repos::repos::zauberwek::concentration_repository::ConcentrationRepository;
use repos::repos::zauberwek::spell_list_repository::SpellListRepository;
use repos::repos::zauberwek::spell_preset_repository::SpellPresetRepository;
use repos::repos::user_repository::UserRepository;
use repos::repos::zauberwek::spell_slot_repository::SpellSlotRepository;
use rocket::config::Config;
use rocket::fs::FileServer;
use std::env;

use repos::DbPool;
use repos::create_pg_pool;

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



    let mut secret_key = [0u8; 32];
    let _ = rand_bytes(&mut secret_key);

    let spp_rep = SpellPresetRepository::new(dbconn.clone());
    let usr_rep = UserRepository::new(dbconn.clone());
    let con_rep = ConcentrationRepository::new(dbconn.clone());
    let ssl_rep = SpellSlotRepository::new(dbconn.clone());
    let sli_rep = SpellListRepository::new(dbconn.clone());


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
        .manage(ssl_rep)
        .manage(sli_rep)
        .mount("/", FileServer::from("./static"))
        .mount("/", routers::get_spell_list_routes())
        .mount("/", routers::get_concentration_routes())
        .mount("/", routers::get_spell_preset_routes())
        .mount("/", routers::get_spell_slot_routes());

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
