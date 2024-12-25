use diesel::dsl::exists;
use diesel::r2d2::ConnectionManager;
use diesel::{ExpressionMethods, Insertable, JoinOnDsl, QueryDsl, RunQueryDsl};
use r2d2::PooledConnection;

use crate::model::{InventoryItem, InventoryReader, InventoryWriter, ItemPreset, User};
use crate::routers::inventory_router;
use crate::{dbmod::DbPool, model::Inventory};
use crate::schema::{inventory, inventory_item, inventory_reader, inventory_writer, item_preset};
use crate::schema::inventory::dsl::*;
use crate::schema::inventory_reader::dsl::*;
use crate::schema::inventory_writer::dsl::*;
use crate::schema::inventory_item::dsl::*;
use crate::schema::item_preset::dsl::*;
use crate::schema::user::dsl::*;
use crate::frontend_model::{InventoryReturn, Item};
use super::format_result_to_custom_err;

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
        format_result_to_custom_err( inventory
            .inner_join(inventory_reader.on(inventory_reader::inventory_uuid.eq(inventory::dsl::uuid)))
            .filter(inventory_reader::user_uuid.eq(inventory_user_uuid))
            .select((inventory::dsl::uuid, owner_uuid, money, inventory::dsl::name))
            .load::<Inventory>(&mut self.get_conn()), "Couldn't load any Inventory"
            )
    }

    fn get_readers_for_inventory(&self, searched_inventory_uuid: String) -> Result<Vec<String>, &'static str> {
        format_result_to_custom_err( inventory_reader.filter(inventory_reader::inventory_uuid.eq(searched_inventory_uuid))
            .select(inventory_reader::dsl::user_uuid)
            .load::<String>(&mut self.get_conn()) ,"Couldn't load inventory readers")
    }

    fn get_writers_for_inventories(&self, searched_inventory_uuid: String) -> Result<Vec<String>, &'static str> {
        format_result_to_custom_err(inventory_writer.filter(inventory_writer::inventory_uuid.eq(searched_inventory_uuid))
            .select(inventory_writer::dsl::user_uuid)
            .load::<String>(&mut self.get_conn()) ,"Couldn't load inventory writers")
    }

    fn get_items_in_inventory(&self, searched_inventory_uuid: String) -> Result<Vec<(String, i32)>, &'static str> {
        format_result_to_custom_err(inventory_item.filter(inventory_item::inventory_uuid.eq(searched_inventory_uuid))
            .select((inventory_item::dsl::item_preset_uuid, inventory_item::dsl::amount))
            .load::<(String, i32)>(&mut self.get_conn()), "Couldn't load items in inventory")
    }

    fn get_item_preset(&self, searched_item_preset: String) -> Result<ItemPreset, &'static str>{
        format_result_to_custom_err( item_preset.find(searched_item_preset).get_result(&mut self.get_conn()),
            "Couldn't load item preset")
    }

    fn get_inventory(&self, searched_inventory_uuid: String) -> Result<Inventory, &'static str> {
        format_result_to_custom_err(inventory.find(searched_inventory_uuid).get_result(&mut self.get_conn()),
            "Couldn't load requested Inventory")
    }

    fn user_has_read_access_to_inventory(&self, searched_inventory_uuid: String, searcher_uuid: String) -> Result<bool, &'static str> {
        format_result_to_custom_err( 
            diesel::select(exists(
                inventory_reader.filter(inventory_reader::dsl::inventory_uuid.eq(searched_inventory_uuid))
                    .filter(inventory_reader::dsl::user_uuid.eq(searcher_uuid))))
                .get_result::<bool>(&mut self.get_conn()), "Failed to load any result")
    }

    fn user_has_write_access_to_inventory(&self, searched_inventory_uuid: String, searcher_uuid: String) -> Result<bool, &'static str> {
        format_result_to_custom_err( 
            diesel::select(exists(
                inventory_writer.filter(inventory_writer::dsl::inventory_uuid.eq(searched_inventory_uuid))
                    .filter(inventory_writer::dsl::user_uuid.eq(searcher_uuid))))
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
        let access = match self.user_has_read_access_to_inventory(searched_inventory_uuid.clone(), searcher_uuid.clone()) {
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

    fn add_reader_to_inventory(&self, searched_inventory_uuid: String, reader_uuid: String) -> Result<bool, &'static str> {
        if (self.user_has_read_access_to_inventory(searched_inventory_uuid.clone(), reader_uuid.clone())?) {
            return Err("User already has read access");
        }
        let inv_read = InventoryReader {
            user_uuid: reader_uuid,
            inventory_uuid: searched_inventory_uuid
        };
        match diesel::insert_into(inventory_reader::table).values(inv_read).execute(&mut self.get_conn()) {
            Ok(_res) => (),
            Err(_e) => return Err("Couldn't insert reader")
        };
        Ok(true)
    }

    fn add_writer_to_inventory(&self, searched_inventory_uuid: String, reader_uuid: String) -> Result<bool, &'static str> {
        if (self.user_has_write_access_to_inventory(searched_inventory_uuid.clone(), reader_uuid.clone())?) {
            return Err("User already has write access");
        }
        let inv_write = InventoryWriter {
            user_uuid: reader_uuid,
            inventory_uuid: searched_inventory_uuid
        };
        match diesel::insert_into(inventory_writer::table).values(inv_write).execute(&mut self.get_conn()) {
            Ok(_res) => (),
            Err(_e) => return Err("Couldn't insert writer")
        };
        Ok(true)
    }

    pub fn insert_inventory(&self, inventory_name: String, creator_uuid: String) -> Result<Inventory, &'static str> {
        let new_inv = Inventory {
            uuid: super::generate_uuid_v4(),
            owner_uuid: creator_uuid.clone(),
            money: 0,
            name: inventory_name
        };
        match diesel::insert_into(inventory::table).values(&new_inv).execute(&mut self.get_conn()) {
            Ok(_res) => (),
            Err(_e) => return Err("Couldn't load inventory")
        };
        self.add_writer_to_inventory(new_inv.owner_uuid.clone(), creator_uuid.clone())?;
        self.add_reader_to_inventory(new_inv.owner_uuid.clone(), creator_uuid.clone())?;
        Ok(new_inv)
    }

    // possible future duplicate
    fn preset_exists(&self, preset_uuid: String) -> Result<bool, &'static str>{
        match diesel::select(exists(item_preset.filter(item_preset::dsl::uuid.eq(preset_uuid)))).get_result(&mut self.get_conn()) {
            Ok(res) => Ok(res),
            Err(_e) => Err("Couldn't load presets")
        }
    }

    pub fn add_preset_to_inventory(&self, searched_inventory_uuid: String, preset_uuid: String, item_amount: i32) -> Result<InventoryItem, &'static str> {
        if !self.preset_exists(preset_uuid.clone())? {
            return Err("Preset does not exists");
        }
        let preset_inventory_pair = InventoryItem {
            inventory_uuid: searched_inventory_uuid,
            item_preset_uuid: preset_uuid,
            dm_note: "".to_string(),
            amount: item_amount
        };
        match diesel::insert_into(inventory_item::table).values(preset_inventory_pair).execute(&mut self.get_conn()) {
            Ok(res) => (),
            Err(_e) => return Err("")
        };
        Ok(preset_inventory_pair)
    }

    pub fn add_new_item_to_inventory() {

    }
}