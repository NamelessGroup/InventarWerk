use rocket::{Build, Rocket, Route};

#[get("/")]
async fn index() -> &'static str {
    // Simuliere eine asynchrone Operation
    rocket::tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    "Hello, Rocket with async!"
}

pub fn get_routes() -> Vec<Route> {
    routes![index]
}