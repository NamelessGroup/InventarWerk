use diesel::dsl::exists;
use diesel::r2d2::ConnectionManager;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use r2d2::PooledConnection;

use crate::model::{InventoryItem, ItemPreset, User};
use crate::{dbmod::DbPool, model::Inventory};
use crate::schema::{inventory, inventory_item, inventory_reader, inventory_writer};
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

    // TODO: Remove duplicate
    fn user_is_dm(&self, id: String) -> Result<bool, &'static str> {
        let acc = user.find(id).get_result::<User>(&mut self.get_conn());
        match acc {
            Ok(res) => Ok(res.dm == 1),
            Err(_e) => Err("Could not load user")
        }
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
        formatResultToCustomErr( item_preset.find(searched_item_preset).get_result(&mut self.get_conn()),
            "Couldn't load item preset")
    }

    fn get_inventory(&self, searched_inventory_uuid: String) -> Result<Inventory, &'static str> {
        formatResultToCustomErr(inventory.find(searched_inventory_uuid).get_result(&mut self.get_conn()),
            "Couldn't load requested Inventory")
    }

    fn user_has_access_to_inventory(&self, searched_inventory_uuid: String, searcher_uuid: String) -> Result<bool, &'static str> {
        formatResultToCustomErr( 
            diesel::select(exists(
                inventory_reader.filter(inventory_reader::dsl::inventory_uuid.eq(searched_inventory_uuid))
                    .filter(inventory_reader::dsl::user_uuid.eq(searcher_uuid))))
                .get_result::<bool>(&mut self.get_conn()), "Failed to load any result")
    }

    pub fn get_dm_note(&self, searched_inventory_uuid: String, searched_item_preset: String) -> Result<String, &'static str> {
        match inventory_item.find((searched_inventory_uuid, searched_item_preset))
        .get_result::<InventoryItem>(&mut self.get_conn()) {
            Ok(res) => Ok(res.dm_note),
            Err(_e) => Err("Couldn't load dm Note")
        }
    }

    pub fn get_inventory_parsed(&self, searcher_uuid: String, searched_inventory_uuid: String) -> Result<InventoryReturn, &'static str> {
        let access = match self.user_has_access_to_inventory(searched_inventory_uuid.clone(), searcher_uuid.clone()) {
            Ok(res) => res,
            Err(e) => return Err(e)
        };
        if !access {
            return Err("Not authorized");
        }
        let user_is_dm = match self.user_is_dm(searcher_uuid) {
            Ok(res) => res,
            Err(e) => return Err(e)
        };
        let inv = match self.get_inventory(searched_inventory_uuid) {
            Ok(res) => res,
            Err(e) => return Err(e)
        };
        let mut inventory_parsed = InventoryReturn{
            uuid: inv.uuid.clone(),
            name: inv.name.clone(),
            owner: inv.owner_uuid.clone(),
            money: inv.money,
            items: Vec::new(),
            reader: self.get_readers_for_inventory(inv.uuid.clone())?,
            writer: self.get_writers_for_inventories(inv.uuid.clone())?
        };
        let items = match self.get_items_in_inventory(inv.uuid.clone()) {
            Ok(res) => res,
            Err(e) => return Err(e)
        };
        for item in items.iter() {
            let preset = self.get_item_preset(item.0.clone())?;
            inventory_parsed.items.push(Item {
                name: preset.name.clone(),
                presetReference: item.0.clone(),
                amount: item.1,
                dmNote: if user_is_dm {
                    self.get_dm_note(inventory_parsed.uuid.clone(), item.0.clone())?}
                        else {"".to_string()},
                description: preset.description.clone()
            });
        }
        Ok(inventory_parsed)
    }

    pub fn get_inventories_parsed(&self, searcher_uuid: String) -> Result<Vec<InventoryReturn>, &'static str> {
        let inv = self.get_all_inventories(searcher_uuid.clone())?;
        let mut inventories: Vec<InventoryReturn> = Vec::new();
        for i in inv.iter() {
            inventories.push(self.get_inventory_parsed(searcher_uuid.clone(), i.uuid.clone())?);
        }
        Ok(inventories)
    }
}