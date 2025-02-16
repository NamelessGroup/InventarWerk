
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use std::env;
use diesel::RunQueryDsl;


#[derive(Debug)]
pub struct SqliteConnectionCustomizer;

impl r2d2::CustomizeConnection<SqliteConnection, diesel::r2d2::Error> for SqliteConnectionCustomizer {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        diesel::sql_query("PRAGMA foreign_keys = ON;")
        .execute(conn)
        .expect("Failed to enable foreign key constraints");
        Ok(())
    }
}

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder().connection_customizer(Box::new(SqliteConnectionCustomizer)).build(manager).expect("Failed to create pool.")
}