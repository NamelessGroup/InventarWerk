use rocket::{Build, Rocket, Route};

#[get("/test")]
async fn test() -> &'static str {
    // Simuliere eine asynchrone Operation
    rocket::tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    "Hello, Rocket with async!"
}

pub fn get_routes() -> Vec<Route> {
    routes![test]
}