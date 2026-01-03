pub mod last_changes_router;

use rocket::Route;

pub fn get_last_changes_routes() -> Vec<Route> {
    routes![
        last_changes_router::last_changes_stream
    ]
}
