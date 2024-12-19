use rocket::{Build, Rocket, Route};

pub mod test;

pub fn get_routes() -> Vec<Route> {
    routes![test::test]
}