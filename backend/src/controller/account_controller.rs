use diesel::associations::HasTable;
use diesel::dsl::exists;
use diesel::r2d2::ConnectionManager;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use r2d2::PooledConnection;
use rocket::http::hyper::server::conn;

use crate::dbmod::DbPool;
use crate::model::User;
use crate::schema::user::dsl::*;

#[derive(Clone)]
pub struct AccountController {
    db: DbPool,
}

impl AccountController {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    fn get_conn(&self) -> PooledConnection<ConnectionManager<diesel::SqliteConnection>> {
        self.db.get().expect("Failed to get connection from Pool")
    }

    pub fn has_users(&self) -> bool {
        let number_of_users: i64 = user.count().get_result(&mut self.get_conn()).expect("Error counting Users");
        number_of_users > 0
    }

    pub fn add_user(&self, id:String, user_name: String) -> User {
        let db_has_users = self.has_users();
        let new_user = User {
            uuid: id,
            name: user_name,
            dm: !db_has_users as i32
        };
        diesel::insert_into(user::table()).values(&new_user)
            .execute(&mut self.get_conn()).expect("Error creating User");
        new_user
    }

    pub fn has_user(&self, id:String) -> bool {
        diesel::select(exists(user.filter(uuid.eq(id)))).get_result(&mut self.get_conn())
            .expect("Error checking User")
    }

    fn get_account(&self, id: String) -> User {
        user.find(id).get_result(&mut self.get_conn()).expect("user could not be queried")
    }

    pub fn user_is_dm(&self, id: String) -> bool {
        self.get_account(id).dm == 1
    }

    pub fn get_all_users(&self) -> Vec<User> {
        user.load::<User>(&mut self.get_conn()).expect("Couldn't load any User")
    }
}