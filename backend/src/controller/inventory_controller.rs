use diesel::associations::HasTable;
use diesel::dsl::exists;
use diesel::r2d2::ConnectionManager;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use r2d2::PooledConnection;
use rocket::http::Status;

use crate::model::{InventoryItem, InventoryReader, InventoryWriter, ItemPreset, UpdateInventoryItem, UpdateInventoryMoney, User};
use crate::{dbmod::DbPool, model::Inventory};
use crate::schema::{inventory, inventory_item, inventory_reader, inventory_writer, item_preset};
use crate::schema::inventory::dsl::*;
use crate::schema::inventory_reader::dsl::*;
use crate::schema::inventory_writer::dsl::*;
use crate::schema::inventory_item::dsl::*;
use crate::schema::item_preset::dsl::*;
use crate::frontend_model::{InventoryReturn, Item};
use super::{cstat, format_result_to_cstat, new_cstst};

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

    fn get_all_inventories(&self, inventory_user_uuid: String) -> Result<Vec<Inventory>, cstat> {
        let query = inventory
            .inner_join(inventory_reader.on(inventory_reader::inventory_uuid.eq(inventory::dsl::uuid)))
            .filter(inventory_reader::user_uuid.eq(inventory_user_uuid))
            .select((inventory::dsl::uuid, owner_uuid, money, inventory::dsl::name))
            .load::<Inventory>(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Couldn't load any Inventory")
    }

    fn get_readers_for_inventory(&self, searched_inventory_uuid: String) -> Result<Vec<String>, cstat> {
        let query = inventory_reader.filter(inventory_reader::inventory_uuid.eq(searched_inventory_uuid))
        .select(inventory_reader::dsl::user_uuid)
        .load::<String>(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Couldn't load inventory readers")
    }

    fn get_writers_for_inventories(&self, searched_inventory_uuid: String) -> Result<Vec<String>, cstat> {
        let query = inventory_writer.filter(inventory_writer::inventory_uuid.eq(searched_inventory_uuid))
        .select(inventory_writer::dsl::user_uuid)
        .load::<String>(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Couldn't load inventory writers")
    }

    fn get_items_in_inventory(&self, searched_inventory_uuid: String) -> Result<Vec<(String, i32)>, cstat> {
        let query = inventory_item.filter(inventory_item::inventory_uuid.eq(searched_inventory_uuid))
        .select((inventory_item::dsl::item_preset_uuid, inventory_item::dsl::amount))
        .load::<(String, i32)>(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Couldn't load items in inventory")
    }

    fn get_item_preset(&self, searched_item_preset: String) -> Result<ItemPreset, cstat>{
        let query = item_preset.find(searched_item_preset).get_result(&mut self.get_conn());
        format_result_to_cstat( query, Status::InternalServerError, "Couldn't load item preset")
    }

    fn get_inventory(&self, searched_inventory_uuid: String) -> Result<Inventory, cstat> {
        let query = inventory.find(searched_inventory_uuid).get_result(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Couldn't load requested Inventory")
    }

    pub fn get_dm_note(&self, searched_inventory_uuid: String, searched_item_preset: String) -> Result<String, cstat> {
        let query = inventory_item.find((searched_inventory_uuid, searched_item_preset))
        .get_result::<InventoryItem>(&mut self.get_conn()); 
        let result = format_result_to_cstat(query, Status::InternalServerError, "Couldn't load dm Note")?;
        Ok(result.dm_note)
    }

    pub fn get_inventory_parsed(&self, searched_inventory_uuid: String) -> Result<InventoryReturn, cstat> {
        let inv = self.get_inventory(searched_inventory_uuid)?;
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
                dmNote: "".to_string(),
                description: preset.description.clone()
            });
        }
        Ok(inventory_parsed)
    }

    pub fn get_inventories_parsed(&self, searcher_uuid: String) -> Result<Vec<InventoryReturn>, cstat> {
        let inv = self.get_all_inventories(searcher_uuid.clone())?;
        let mut inventories: Vec<InventoryReturn> = Vec::new();
        for i in inv.iter() {
            inventories.push(self.get_inventory_parsed(i.uuid.clone())?);
        }
        Ok(inventories)
    }

    pub fn add_reader_to_inventory(&self, searched_inventory_uuid: String, reader_uuid: String) -> Result<bool, cstat> {
        let inv_read = InventoryReader {
            user_uuid: reader_uuid,
            inventory_uuid: searched_inventory_uuid
        };
        let query =  diesel::insert_into(inventory_reader::table)
            .values(inv_read).execute(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Couldn't insert reader")?;
        Ok(true)
    }

    pub fn add_writer_to_inventory(&self, searched_inventory_uuid: String, writer_uuid: String) -> Result<bool, cstat> {
        let inv_write = InventoryWriter {
            user_uuid: writer_uuid,
            inventory_uuid: searched_inventory_uuid
        };
        let query = diesel::insert_into(inventory_writer::table)
            .values(inv_write).execute(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Couldn't insert writer")?;
        Ok(true)
    }

    pub fn insert_inventory(&self, inventory_name: String, creator_uuid: String) -> Result<Inventory, cstat> {
        let new_inv = Inventory {
            uuid: super::generate_uuid_v4(),
            owner_uuid: creator_uuid.clone(),
            money: 0,
            name: inventory_name
        };
        let query = diesel::insert_into(inventory::table).values(&new_inv)
            .execute(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Couldn't load inventory")?;
        self.add_writer_to_inventory(new_inv.owner_uuid.clone(), creator_uuid.clone())?;
        self.add_reader_to_inventory(new_inv.owner_uuid.clone(), creator_uuid.clone())?;
        Ok(new_inv)
    }

    // possible future duplicate
    fn preset_exists(&self, preset_uuid: String) -> Result<bool, cstat>{
        let query = diesel::select(exists(
            item_preset.filter(item_preset::dsl::uuid.eq(preset_uuid)))).get_result(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Couldn't load presets")
    }

    pub fn add_preset_to_inventory(&self, searched_inventory_uuid: String, preset_uuid: String, item_amount: i32)
            -> Result<InventoryItem, cstat> {
        if !self.preset_exists(preset_uuid.clone())? {
            return Err(new_cstst(Status::NotFound, "Preset does not exists"));
        }
        let preset_inventory_pair = InventoryItem {
            inventory_uuid: searched_inventory_uuid,
            item_preset_uuid: preset_uuid,
            dm_note: "".to_string(),
            amount: item_amount
        };
        let query = diesel::insert_into(inventory_item::table).values(&preset_inventory_pair)
            .execute(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to insert into table")?;
        Ok(preset_inventory_pair)
    }

    pub fn add_new_item_to_inventory(&self, searched_inventory_uuid: String,
            preset_name: String, item_amount: i32, creator_uuid: String) -> Result<ItemPreset, cstat> {
        let new_item_preset = ItemPreset {
            name: preset_name,
            uuid: super::generate_uuid_v4(),
            price: 0,
            description: "".to_string(),
            creator: creator_uuid,
            item_type: "".to_string()
        };
        let query = diesel::insert_into(item_preset::table).values(&new_item_preset)
            .execute(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to insert into table");
        self.add_preset_to_inventory(searched_inventory_uuid, new_item_preset.uuid.clone(), item_amount)?;
        return Ok(new_item_preset);
    }

    pub fn edit_item_amount(&self, searched_inventory_uuid: String, searched_item_preset: String, new_amount:i32)
        -> Result<bool, cstat> {
        let query = diesel::update(inventory_item.find((searched_inventory_uuid, searched_item_preset)))
            .set(UpdateInventoryItem {
                amount: Some(new_amount),
                dm_note: None
            }).execute(&mut self.get_conn());
        match query {
            Err(_e) => Err(new_cstst(Status::InternalServerError, "Failed to insert into table")),
            Ok(_res) => Ok(true)
        }
    }

    pub fn edit_item_dm_note(&self, searched_inventory_uuid: String, searched_item_preset: String, new_dm_note:String)
    -> Result<bool, cstat> {
        let query = diesel::update(inventory_item.find((searched_inventory_uuid, searched_item_preset)))
            .set(UpdateInventoryItem {
                amount: None,
                dm_note: Some(new_dm_note)
            }).execute(&mut self.get_conn());
        match query {
            Ok(_res) => Ok(true),
            Err(_e) => Err(new_cstst(Status::InternalServerError,"Couldn't update item"))
        }
    }

    pub fn delete_item_from_inventory(&self, searched_inventory_uuid: String, searched_item_preset: String)
        -> Result<bool, cstat> {
        let query = diesel::delete(inventory_item.find((searched_inventory_uuid, searched_item_preset)))
            .execute(&mut self.get_conn());
        match query {
                Ok(_res) => Ok(true),
                Err(_e) => Err(new_cstst(Status::InternalServerError, "Couldn't delete Entry"))
            }
    }
    pub fn edit_money_in_inventory(&self, searched_inventory_uuid: String, new_money:i32) -> Result<bool, cstat>{
        let query = diesel::update(inventory.find(searched_inventory_uuid))
            .set(UpdateInventoryMoney{
            money: new_money
        }).execute(&mut self.get_conn());
        match query {
            Ok(_res) => Ok(true),
            Err(_e) => Err(new_cstst(Status::InternalServerError, "Couldn't update Money"))
        }
    }


    pub fn delete_inventory(&self, searched_inventory_uuid: String) -> Result<bool, cstat> {
        let query =  diesel::delete(inventory.find(searched_inventory_uuid))
            .execute(&mut self.get_conn());
        match query {
            Ok(_res) => Ok(true),
            Err(_e) => Err(new_cstst(Status::InternalServerError, "Couldn't delete inventory"))
        }
    }

    pub fn remove_reader_from_inventory(&self, searched_inventory_uuid: String, reader_uuid: String) -> Result<bool, cstat> {
        let query = diesel::delete(inventory_reader.find((searched_inventory_uuid, reader_uuid)))
            .execute(&mut self.get_conn());
        match query {
            Ok(_res) => Ok(true),
            Err(_e) => Err(new_cstst(Status::InternalServerError, "Couldn't remove pair"))
        }
    }

    pub fn remove_writer_from_inventory(&self, searched_inventory_uuid: String, writer_uuid: String) -> Result<bool, cstat> {
        let query =  diesel::delete(inventory_writer.find((searched_inventory_uuid, writer_uuid)))
            .execute(&mut self.get_conn());
        match query {
            Ok(_res) => Ok(true),
            Err(_e) => Err(new_cstst(Status::InternalServerError, "Couldn't remove pair"))
        }
    }

    pub fn is_creator_of_inventory(&self, searched_inventory_uuid: String, creator_canidate: String) -> Result<bool, cstat> {
        Ok(self.get_inventory(searched_inventory_uuid)?.owner_uuid == creator_canidate)
    }
}