use diesel::associations::HasTable;
use diesel::dsl::exists;
use diesel::r2d2::ConnectionManager;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use r2d2::PooledConnection;
use rocket::http::Status;

use crate::dbmod::DbPool;
use crate::model::User;
use crate::schema::{inventory_reader, inventory_writer};
use crate::schema::user::dsl::*;
use crate::schema::inventory_reader::dsl::*;
use crate::schema::inventory_writer::dsl::*;

use super::{format_result_to_cstat, new_cstat_from_ref, CStat};

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

    pub fn has_users(&self) -> Result<bool, CStat> {
        let number_of_users = user.count().get_result::<i64>(&mut self.get_conn());
        let result = match number_of_users {
            Ok(res) => Ok(res > 0),
            Err(e) => Err(e)
        };
        format_result_to_cstat(result, Status::InternalServerError, "Failed to load user table")
    }

    pub fn add_user(&self, id:String, user_name: String) -> Result<User, CStat> {
        if self.has_user(id.clone())? {
            return Err(new_cstat_from_ref(Status::BadRequest, "User already exists"))
        }
        let db_has_users = self.has_users()?;

        let new_user = User {
            uuid: id,
            name: user_name,
            dm: !db_has_users as i32
        };
        let query_result = diesel::insert_into(user::table()).values(&new_user)
        .execute(&mut self.get_conn());
        format_result_to_cstat(query_result, Status::InternalServerError, "Failed to insert into db")?;
        Ok(new_user)
    }

    pub fn has_user(&self, id:String) -> Result<bool, CStat> {
        let query_result = diesel::select(exists(user.filter(uuid.eq(id))))
            .get_result(&mut self.get_conn());
        format_result_to_cstat(query_result, Status::InternalServerError, "Failed to load Table")
    }

    fn get_account(&self, id: String) -> Result<User, CStat> {
        if !self.has_user(id.clone())? {
            return Err(new_cstat_from_ref(Status::BadRequest, "User does not exists"))
        }
        let queried_user = user.find(id).get_result::<User>(&mut self.get_conn());
        format_result_to_cstat(queried_user, Status::InternalServerError, "Failed to load user in get_account")
    }

    pub fn user_is_dm(&self, id: String) -> Result<bool, CStat> {
        let acc = self.get_account(id)?;
        Ok(acc.dm == 1)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, CStat> {
        let users =  user.load::<User>(&mut self.get_conn());
        format_result_to_cstat(users, Status::InternalServerError, "Failed load Users")
    }

    pub fn user_has_read_access_to_inventory(&self, searched_inventory_uuid: String, searcher_uuid: String)
            -> Result<bool, CStat> {
        let query_result = diesel::select(exists(
            inventory_reader.filter(inventory_reader::dsl::inventory_uuid.eq(searched_inventory_uuid))
                .filter(inventory_reader::dsl::user_uuid.eq(searcher_uuid))))
            .get_result::<bool>(&mut self.get_conn());
        format_result_to_cstat(query_result, Status::InternalServerError, "Failed to load any user")
    }

    pub fn user_has_write_access_to_inventory(&self, searched_inventory_uuid: String, searcher_uuid: String)
            -> Result<bool, CStat> {
        let query_result = diesel::select(exists(
            inventory_writer.filter(inventory_writer::dsl::inventory_uuid.eq(searched_inventory_uuid))
                .filter(inventory_writer::dsl::user_uuid.eq(searcher_uuid))))
            .get_result::<bool>(&mut self.get_conn());
        format_result_to_cstat(query_result, Status::InternalServerError, "Failed to load any result")
    }

}