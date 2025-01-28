use diesel::dsl::{exists, max};
use diesel::r2d2::ConnectionManager;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use r2d2::PooledConnection;
use rocket::http::Status;

use crate::model::{InventoryItem, InventoryReader, InventoryWriter, ItemPreset, UpdateInventoryItem, UpdateInventory};
use crate::report_change_on_inventory;
use crate::{dbmod::DbPool, model::Inventory};
use crate::schema::{inventory, inventory_item, inventory_reader, inventory_writer, item_preset, user};
use crate::schema::inventory::dsl::*;
use crate::schema::inventory_reader::dsl::*;
use crate::schema::inventory_writer::dsl::*;
use crate::schema::inventory_item::dsl::*;
use crate::schema::item_preset::dsl::*;
use crate::schema::user::dsl::*;
use crate::frontend_model::{InventoryReturn, Item};
use super::{CStat, format_result_to_cstat, new_cstat_from_ref};

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

    pub fn get_all_inventories_ids(&self, searcher_uuid: String) -> Result<Vec<String>, CStat> {
        let query = inventory_reader.filter(inventory_reader::user_uuid.eq(searcher_uuid))
            .select(inventory_reader::inventory_uuid).load::<String>(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to load reader table")
    }

    pub fn get_all_inventories_ids_with_read_access(&self, searcher_uuid: String) -> Result<Vec<String>, CStat> {
        let query = inventory_writer.filter(inventory_writer::user_uuid.eq(searcher_uuid))
            .select(inventory_writer::inventory_uuid).load::<String>(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to load reader table")
    }

    fn get_all_inventories(&self, searcher_uuid: String) -> Result<Vec<Inventory>, CStat> {
        let query = inventory
            .inner_join(inventory_reader.on(inventory_reader::inventory_uuid.eq(inventory::dsl::uuid)))
            .filter(inventory_reader::user_uuid.eq(searcher_uuid))
            .select((inventory::dsl::uuid, owner_uuid, money, inventory::dsl::name))
            .load::<Inventory>(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to load any Inventory")
    }

    pub fn get_readers_for_inventory(&self, searched_inventory_uuid: String) -> Result<Vec<String>, CStat> {
        let query = inventory_reader.filter(inventory_reader::inventory_uuid.eq(searched_inventory_uuid))
        .select(inventory_reader::dsl::user_uuid)
        .load::<String>(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to load inventory readers")
    }

    fn get_writers_for_inventories(&self, searched_inventory_uuid: String) -> Result<Vec<String>, CStat> {
        let query = inventory_writer.filter(inventory_writer::inventory_uuid.eq(searched_inventory_uuid))
        .select(inventory_writer::dsl::user_uuid)
        .load::<String>(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to load inventory writers")
    }

    fn get_items_in_inventory(&self, searched_inventory_uuid: String) -> Result<Vec<InventoryItem>, CStat> {
        let query = inventory_item.filter(inventory_item::inventory_uuid.eq(searched_inventory_uuid))
        .load::<InventoryItem>(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to load items in inventory")
    }

    fn get_item_preset(&self, searched_item_preset: String) -> Result<ItemPreset, CStat>{
        let query = item_preset.find(searched_item_preset).get_result(&mut self.get_conn());
        format_result_to_cstat( query, Status::InternalServerError, "Failed to load item preset")
    }

    fn get_inventory(&self, searched_inventory_uuid: String) -> Result<Inventory, CStat> {
        let query = inventory.find(searched_inventory_uuid).get_result(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to load requested Inventory")
    }

    pub fn item_exits(&self, searched_inventory_uuid: String, searched_item_preset: String) -> Result<bool, CStat> {
        let query = diesel::select(exists(
            inventory_item.filter(item_preset_uuid.eq(searched_item_preset))
            .filter(inventory_item::inventory_uuid.eq(searched_inventory_uuid))))
            .get_result::<bool>(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to load inventory item table")
    }

    pub fn get_dm_note(&self, searched_inventory_uuid: String, searched_item_preset: String) -> Result<String, CStat> {
        let query = inventory_item.find((searched_inventory_uuid, searched_item_preset))
        .get_result::<InventoryItem>(&mut self.get_conn()); 
        let result = format_result_to_cstat(query, Status::InternalServerError, "Failed to load dm Note")?;
        Ok(result.dm_note)
    }

    pub fn get_inventory_parsed(&self, searched_inventory_uuid: String, is_dm: bool) -> Result<InventoryReturn, CStat> {
        let inv = self.get_inventory(searched_inventory_uuid.clone())?;
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
            let preset = self.get_item_preset(item.item_preset_uuid.clone())?;
            inventory_parsed.items.push(Item {
                name: preset.name.clone(),
                presetReference: item.item_preset_uuid.clone(),
                amount: item.amount,
                presetCreator: preset.creator,
                price: preset.price,
                dmNote: if is_dm {self.get_dm_note(searched_inventory_uuid.clone(), item.item_preset_uuid.clone())?} else {"".to_string()} ,
                description: preset.description.clone(),
                weight: preset.weight,
                sorting: item.sorting,
                inventoryItemNote: item.inventory_item_note.clone()
            });
        }
        Ok(inventory_parsed)
    }

    pub fn get_inventories_parsed(&self, searcher_uuid: String, is_dm: bool) -> Result<Vec<InventoryReturn>, CStat> {
        let inv = self.get_all_inventories(searcher_uuid.clone())?;
        let mut inventories: Vec<InventoryReturn> = Vec::new();
        for i in inv.iter() {
            inventories.push(self.get_inventory_parsed(i.uuid.clone(), is_dm)?);
        }
        Ok(inventories)
    }

    pub fn add_reader_to_inventory(&self, searched_inventory_uuid: String, reader_uuid: String) -> Result<bool, CStat> {
        let inv_read = InventoryReader {
            user_uuid: reader_uuid,
            inventory_uuid: searched_inventory_uuid.clone()
        };
        let query =  diesel::insert_into(inventory_reader::table)
            .values(inv_read).execute(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to insert reader")?;
        report_change_on_inventory!(searched_inventory_uuid.clone());
        Ok(true)
    }

    pub fn add_writer_to_inventory(&self, searched_inventory_uuid: String, writer_uuid: String) -> Result<bool, CStat> {
        let inv_write = InventoryWriter {
            user_uuid: writer_uuid,
            inventory_uuid: searched_inventory_uuid.clone()
        };
        let query = diesel::insert_into(inventory_writer::table)
            .values(inv_write).execute(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to insert writer")?;
        report_change_on_inventory!(searched_inventory_uuid.clone());
        Ok(true)
    }

    fn get_dm_accounts(&self) -> Result<Vec<String>, CStat> {
        let accs = user.filter(user::dm.eq(1)).select(user::uuid).load::<String>(&mut self.get_conn());
        format_result_to_cstat(accs, Status::InternalServerError, "Failed to load users")
    }

    pub fn insert_inventory(&self, inventory_name: String, creator_uuid: String) -> Result<Inventory, CStat> {
        let new_inv = Inventory {
            uuid: super::generate_uuid_v4(),
            owner_uuid: creator_uuid.clone(),
            money: 0,
            name: inventory_name
        };
        let query = diesel::insert_into(inventory::table).values(&new_inv)
            .execute(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to insert inventory")?;
        self.add_writer_to_inventory(new_inv.uuid.clone(), creator_uuid.clone())?;
        self.add_reader_to_inventory(new_inv.uuid.clone(), creator_uuid.clone())?;
        for acc in self.get_dm_accounts()? {
            if acc == creator_uuid {
                continue;
            }
            self.add_reader_to_inventory(new_inv.uuid.clone(), acc.clone())?;
            self.add_writer_to_inventory(new_inv.uuid.clone(), acc.clone())?;
        }
        report_change_on_inventory!(new_inv.uuid.clone());
        Ok(new_inv)
    }

    fn preset_exists(&self, preset_uuid: String) -> Result<bool, CStat>{
        let query = diesel::select(exists(
            item_preset.filter(item_preset::dsl::uuid.eq(preset_uuid)))).get_result(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to load presets")
    }

    pub fn add_preset_to_inventory(&self, searched_inventory_uuid: String, preset_uuid: String, item_amount: i32)
            -> Result<InventoryItem, CStat> {
        if !self.preset_exists(preset_uuid.clone())? {
            return Err(new_cstat_from_ref(Status::NotFound, "Preset does not exists"));
        }
        let max_sorting_query = inventory_item.select(max(inventory_item::sorting)).first::<Option<i32>>(&mut self.get_conn());
        let max_sorting = format_result_to_cstat(max_sorting_query, Status::InternalServerError, "Coudn't load sorting")?.unwrap_or(0);

        let preset_inventory_pair = InventoryItem {
            inventory_uuid: searched_inventory_uuid.clone(),
            item_preset_uuid: preset_uuid,
            dm_note: "".to_string(),
            amount: item_amount,
            inventory_item_note: "".to_string(),
            sorting: max_sorting
        };
        let query = diesel::insert_into(inventory_item::table).values(&preset_inventory_pair)
            .execute(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to insert into table")?;
        report_change_on_inventory!(searched_inventory_uuid.clone());
        Ok(preset_inventory_pair)
    }

    pub fn add_new_item_to_inventory(&self, searched_inventory_uuid: String,
            preset_name: String, item_amount: i32, creator_uuid: String) -> Result<ItemPreset, CStat> {
        let new_item_preset = ItemPreset {
            name: preset_name,
            uuid: super::generate_uuid_v4(),
            price: 0,
            weight: 0.0,
            description: "".to_string(),
            creator: creator_uuid,
            item_type: "".to_string()
        };
        let query = diesel::insert_into(item_preset::table).values(&new_item_preset)
            .execute(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to insert into table")?;
        self.add_preset_to_inventory(searched_inventory_uuid.clone(), new_item_preset.uuid.clone(), item_amount)?;
        report_change_on_inventory!(searched_inventory_uuid.clone());
        return Ok(new_item_preset);
    }

    pub fn edit_inventory_item(&self, searched_inventory_uuid: String, searched_item_preset: String, new_amount:Option<i32>,
        new_sorting: Option<i32>, new_inventory_item_note: Option<String>)-> Result<bool, CStat> {
        let query = diesel::update(inventory_item.find((searched_inventory_uuid.clone(), searched_item_preset)))
            .set(UpdateInventoryItem {
                amount: new_amount,
                dm_note: None,
                sorting: new_sorting,
                inventory_item_note: new_inventory_item_note
            }).execute(&mut self.get_conn());
        report_change_on_inventory!(searched_inventory_uuid.clone());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to insert into table")?;
        Ok(true)
    }

    pub fn edit_item_dm_note(&self, searched_inventory_uuid: String, searched_item_preset: String, new_dm_note:String)
    -> Result<bool, CStat> {
        let query = diesel::update(inventory_item.find((searched_inventory_uuid.clone(), searched_item_preset)))
            .set(UpdateInventoryItem {
                amount: None,
                dm_note: Some(new_dm_note),
                sorting: None,
                inventory_item_note: None
            }).execute(&mut self.get_conn());
        report_change_on_inventory!(searched_inventory_uuid.clone());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to update item")?;
        Ok(true)
    }

    pub fn delete_item_from_inventory(&self, searched_inventory_uuid: String, searched_item_preset: String)
        -> Result<bool, CStat> {
        let query = diesel::delete(inventory_item.find((searched_inventory_uuid.clone(), searched_item_preset)))
            .execute(&mut self.get_conn());
        report_change_on_inventory!(searched_inventory_uuid.clone());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to delete Entry")?;
        Ok(true)
    }
    pub fn edit_inventory(&self, searched_inventory_uuid: String, new_money: Option<i32>, new_name: Option<String>) -> Result<bool, CStat> {
        let query = diesel::update(inventory.find(searched_inventory_uuid.clone()))
            .set(UpdateInventory{
            money: new_money,
            name: new_name
        }).execute(&mut self.get_conn());
        report_change_on_inventory!(searched_inventory_uuid.clone());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to update money")?;
        Ok(true)
    }


    pub fn delete_inventory(&self, searched_inventory_uuid: String) -> Result<bool, CStat> {
        let query =  diesel::delete(inventory.find(searched_inventory_uuid))
            .execute(&mut self.get_conn());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to delete inventory")?;
        Ok(true)
    }

    pub fn remove_reader_from_inventory(&self, searched_inventory_uuid: String, reader_uuid: String) -> Result<bool, CStat> {
        let query = diesel::delete(inventory_reader.find((reader_uuid, searched_inventory_uuid.clone())))
            .execute(&mut self.get_conn());
        report_change_on_inventory!(searched_inventory_uuid.clone());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to remove pair")?;
        Ok(true)
    }

    pub fn remove_writer_from_inventory(&self, searched_inventory_uuid: String, writer_uuid: String) -> Result<bool, CStat> {
        let query =  diesel::delete(inventory_writer.find((writer_uuid, searched_inventory_uuid.clone())))
            .execute(&mut self.get_conn());
        report_change_on_inventory!(searched_inventory_uuid.clone());
        format_result_to_cstat(query, Status::InternalServerError, "Failed to remove pair")?;
        Ok(true)
    }

    pub fn is_creator_of_inventory(&self, searched_inventory_uuid: String, creator_canidate: String) -> Result<bool, CStat> {
        Ok(self.get_inventory(searched_inventory_uuid)?.owner_uuid == creator_canidate)
    }
}