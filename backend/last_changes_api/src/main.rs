#[macro_use]
extern crate rocket;

mod routers;

use dotenvy::dotenv;
use openssl::rand::rand_bytes;
use rocket::config::Config;
use rocket::tokio::sync::broadcast::channel;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use sqlx::postgres::PgListener;
use repos::create_pg_pool;

use routers::last_changes_router::{InventoryChangeEvent, LastChangesApiDoc};

/// Background task that listens to Postgres NOTIFY and forwards to SSE broadcast
async fn postgres_listener_task(
    db_url: String,
    tx: rocket::tokio::sync::broadcast::Sender<InventoryChangeEvent>,
) {
    loop {
        match PgListener::connect(&db_url).await {
            Ok(mut listener) => {
                println!("Connected to Postgres for LISTEN");
                
                if let Err(e) = listener.listen("inventory_changes").await {
                    eprintln!("Failed to LISTEN on channel: {}", e);
                    rocket::tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    continue;
                }

                println!("Listening for inventory changes...");
                
                loop {
                    match listener.recv().await {
                        Ok(notification) => {
                            let payload = notification.payload();
                            
                            // Parse JSON payload from Postgres trigger
                            match serde_json::from_str::<serde_json::Value>(payload) {
                                Ok(json) => {
                                    let uuid = json["uuid"].as_str().unwrap_or("").to_string();
                                    let source = json["source"].as_str().unwrap_or("").to_string();
                                    let change_type = json["type"].as_str().unwrap_or("").to_string();
                                    
                                    let timestamp = std::time::SystemTime::now()
                                        .duration_since(std::time::UNIX_EPOCH)
                                        .expect("Time went backwards")
                                        .as_millis();

                                    let event = InventoryChangeEvent {
                                        uuid,
                                        source,
                                        change_type,
                                        timestamp,
                                    };

                                    // Broadcast to all SSE clients
                                    let _ = tx.send(event);
                                }
                                Err(e) => {
                                    eprintln!("Failed to parse notification payload: {} - {}", payload, e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error receiving notification: {}", e);
                            break; // Reconnect
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to connect to Postgres: {}", e);
                rocket::tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }
}

/// Main async entry point for the last_changes_api server.
#[rocket::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create DB pool for migrations
    let _dbpool = create_pg_pool(database_url.clone())
        .await
        .expect("Couldn't connect to database");

    // Create broadcast channel for SSE events
    let (tx, _rx) = channel::<InventoryChangeEvent>(1024);

    // Spawn background task to listen to Postgres notifications
    let tx_clone = tx.clone();
    let db_url_clone = database_url.clone();
    rocket::tokio::spawn(async move {
        postgres_listener_task(db_url_clone, tx_clone).await;
    });

    let mut secret_key = [0u8; 32];
    let _ = rand_bytes(&mut secret_key);

    let figment = Config::figment().merge(("secret_key", secret_key));

    #[cfg(any(feature = "dev"))]
    let figment = figment
        .merge(("address", "127.0.0.1"))
        .merge(("port", 8001));

    let config = Config::from(figment);
    #[allow(unused_mut)]
    let mut r = rocket::build()
        .configure(config)
        .manage(tx)
        .mount("/", routers::get_last_changes_routes())
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>")
                .url("/api-docs/openapi_last_changes.json", LastChangesApiDoc::openapi()),
        );

    #[cfg(any(feature = "dev"))]
    {
        println!("Starting last_changes_api with CORS.");
        println!("Only do this in development.");
        use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
        let cors = CorsOptions {
            allowed_origins: AllowedOrigins::all(),
            allowed_methods: vec!["GET", "POST", "PUT", "DELETE", "OPTIONS", "PATCH"]
                .into_iter()
                .map(|method| method.parse().unwrap())
                .collect(),
            allowed_headers: AllowedHeaders::all(),
            allow_credentials: true,
            ..Default::default()
        }
        .to_cors()
        .expect("Error configuring CORS");

        r = r.attach(cors);
    }
    let _res = r.launch().await;
}
