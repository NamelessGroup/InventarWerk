use diesel::associations::HasTable;
use diesel::dsl::exists;
use diesel::r2d2::ConnectionManager;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use r2d2::PooledConnection;

use crate::dbmod::DbPool;
use crate::model::User;
use crate::schema::{inventory_reader, inventory_writer};
use crate::schema::user::dsl::*;
use crate::schema::inventory_reader::dsl::*;
use crate::schema::inventory_writer::dsl::*;

use super::format_result_to_custom_err;

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

    pub fn has_users(&self) -> Result<bool, &'static str> {
        let number_of_users = user.count().get_result::<i64>(&mut self.get_conn());
        match number_of_users {
            Ok(users) => Ok(users > 0),
            Err(_e) => Err("Couldn't count Users")
        }
    }

    pub fn add_user(&self, id:String, user_name: String) -> Result<User, &str> {
        let db_has_users = self.has_users()?;

        let new_user = User {
            uuid: id,
            name: user_name,
            dm: !db_has_users as i32
        };
        match diesel::insert_into(user::table()).values(&new_user)
        .execute(&mut self.get_conn()) {
            Err(_e) => return Err("Couldn't insert into db"),
            Ok(_) => ()
        };
        Ok(new_user)
    }

    pub fn has_user(&self, id:String) -> Result<bool, &'static str> {
        match diesel::select(exists(user.filter(uuid.eq(id)))).get_result(&mut self.get_conn()) {
            Ok(r) => Ok(r),
            Err(_e) => Err("Couldn't load Users")
        }
    }

    fn get_account(&self, id: String) -> Result<User, &'static str> {
        let u = user.find(id).get_result::<User>(&mut self.get_conn());
        match u {
            Ok(res) => Ok(res),
            Err(_e) => Err("Couldn't find user with this id!")
        }
    }

    pub fn user_is_dm(&self, id: String) -> Result<bool, &'static str> {
        let acc = self.get_account(id);
        match acc {
            Ok(res) => Ok(res.dm == 1),
            Err(e) => Err(e)
        }
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, &'static str> {
        match user.load::<User>(&mut self.get_conn()) {
            Ok(res) => Ok(res),
            Err(_e) => Err("Couldn't load Users")
        }
    }

    pub fn user_has_read_access_to_inventory(&self, searched_inventory_uuid: String, searcher_uuid: String) -> Result<bool, &'static str> {
        format_result_to_custom_err( 
            diesel::select(exists(
                inventory_reader.filter(inventory_reader::dsl::inventory_uuid.eq(searched_inventory_uuid))
                    .filter(inventory_reader::dsl::user_uuid.eq(searcher_uuid))))
                .get_result::<bool>(&mut self.get_conn()), "Failed to load any result")
    }

    pub fn user_has_write_access_to_inventory(&self, searched_inventory_uuid: String, searcher_uuid: String) -> Result<bool, &'static str> {
        format_result_to_custom_err( 
            diesel::select(exists(
                inventory_writer.filter(inventory_writer::dsl::inventory_uuid.eq(searched_inventory_uuid))
                    .filter(inventory_writer::dsl::user_uuid.eq(searcher_uuid))))
                .get_result::<bool>(&mut self.get_conn()), "Failed to load any result")
    }

}