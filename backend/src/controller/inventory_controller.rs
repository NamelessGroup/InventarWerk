use diesel::r2d2::ConnectionManager;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use r2d2::PooledConnection;

use crate::model::{InventoryItem, ItemPreset, User};
use crate::{dbmod::DbPool, model::Inventory};
use crate::schema::{inventory, inventory_item, inventory_reader, inventory_writer, user};
use crate::schema::inventory::dsl::*;
use crate::schema::inventory_reader::dsl::*;
use crate::schema::inventory_writer::dsl::*;
use crate::schema::inventory_item::dsl::*;
use crate::schema::item_preset::dsl::*;
use crate::schema::user::dsl::*;
use crate::frontend_model::{InventoryReturn, Item};
use super::formatResultToCustomErr;

#[derive(Clone)]
pub struct InventoryController {
    db: DbPool,
}

impl InventoryController {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    fn get_conn(&self) -> PooledConnection<ConnectionManager<diesel::SqliteConnection>> {
        self.db.get().expect("Failed to get connection from Pool")
    }

    fn get_all_inventories(&self, inventory_user_uuid: String) -> Result<Vec<Inventory>, &'static str> {
        formatResultToCustomErr( inventory
            .inner_join(inventory_reader.on(inventory_reader::inventory_uuid.eq(inventory::dsl::uuid)))
            .filter(inventory_reader::user_uuid.eq(inventory_user_uuid))
            .select((inventory::dsl::uuid, owner_uuid, money, inventory::dsl::name))
            .load::<Inventory>(&mut self.get_conn()), "Couldn't load any Inventory"
            )
    }

    fn get_readers_for_inventory(&self, searched_inventory_uuid: String) -> Result<Vec<String>, &'static str> {
        formatResultToCustomErr( inventory_reader.filter(inventory_reader::inventory_uuid.eq(searched_inventory_uuid))
            .select(inventory_reader::dsl::user_uuid)
            .load::<String>(&mut self.get_conn()) ,"Couldn't load inventory readers")
    }

    fn get_writers_for_inventories(&self, searched_inventory_uuid: String) -> Result<Vec<String>, &'static str> {
        formatResultToCustomErr(inventory_writer.filter(inventory_writer::inventory_uuid.eq(searched_inventory_uuid))
            .select(inventory_writer::dsl::user_uuid)
            .load::<String>(&mut self.get_conn()) ,"Couldn't load inventory writers")
    }

    fn get_items_in_inventory(&self, searched_inventory_uuid: String) -> Result<Vec<(String, i32)>, &'static str> {
        formatResultToCustomErr(inventory_item.filter(inventory_item::inventory_uuid.eq(searched_inventory_uuid))
            .select((inventory_item::dsl::item_preset_uuid, inventory_item::dsl::amount))
            .load::<(String, i32)>(&mut self.get_conn()), "Couldn't load items in inventory")
    }

    fn get_item_preset(&self, searched_item_preset: String) -> Result<ItemPreset, &'static str>{
        formatResultToCustomErr( item_preset.find(searched_item_preset).get_result(&mut self.get_conn()),"")
    }

    pub fn get_dm_note(&self, searched_inventory_uuid: String, searched_item_preset: String) -> Result<String, &'static str> {
        match inventory_item.find((searched_inventory_uuid, searched_item_preset))
        .get_result::<InventoryItem>(&mut self.get_conn()) {
            Ok(res) => Ok(res.dm_note),
            Err(_e) => Err("")
        }
    }
    pub fn get_inventories_parsed(&self, searcher_uuid: String) -> Result<Vec<InventoryReturn>, &'static str> {
        let inv = match self.get_all_inventories(searcher_uuid.clone()){
            Ok(res) => res,
            Err(e) => return Err(e)
        };
        let mut inventories: Vec<InventoryReturn> = Vec::new();
        let user_is_dm = user.find(searcher_uuid).get_result::<User>(&mut self.get_conn())
            .expect("user could not be queried").dm == 1;
        for i in inv.iter() {
            let mut specific_inventory = InventoryReturn{
                uuid: i.uuid.clone(),
                name: i.name.clone(),
                owner: i.owner_uuid.clone(),
                money: i.money,
                items: Vec::new(),
                reader: match self.get_readers_for_inventory(i.uuid.clone()) {
                    Ok(res) => res,
                    Err(e) => return Err(e)
                },
                writer: match self.get_writers_for_inventories(i.uuid.clone()) {
                    Ok(res) => res,
                    Err(e) => return Err(e)
                }
            };
            let items = match self.get_items_in_inventory(i.uuid.clone()) {
                Ok(res) => res,
                Err(e) => return Err(e)
            };
            for item in items.iter() {
                let preset = match self.get_item_preset(item.0.clone()) {
                    Ok(res) => res,
                    Err(e) => return Err(e)
                };
                specific_inventory.items.push(Item {
                    name: preset.name.clone(),
                    presetReference: item.0.clone(),
                    amount: item.1,
                    dmNote: if user_is_dm {
                        match self.get_dm_note(specific_inventory.uuid.clone(), item.0.clone()) {
                            Ok(res) => res,
                            Err(e) => return Err(e)
                    }} else {"".to_string()},
                    description: preset.description.clone()
                });
            }
            inventories.push(specific_inventory);
        }
        Ok(inventories)
    }
}