use diesel::prelude::SqliteConnection;


pub struct ItemsController {
    db_manager: SqliteConnection,
}

impl ItemsController {
    pub fn new(db_manager: SqliteConnection) -> Self {
        Self { db_manager }
    }
}