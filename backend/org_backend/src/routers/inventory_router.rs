use inv_rep::model::FullInventory;
use inv_rep::repos::inventory_repository::InventoryRepository;
use inv_rep::repos::user_repository::UserRepository;
use rocket::http::Status;
use rocket::{form::FromForm, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use anyhow::anyhow;
use rocket_errors::anyhow::Result;

#[derive(FromForm)]
pub struct InventoryUUIDParams {
    inventory_uuid: String
}

#[derive(Serialize, Deserialize)]
pub struct GetAllInventoriesReturn{
    inventories: Vec<FullInventory>
}

/// Handles the `/inventory/all` route, returns all inventories in the InventoryReturn form
#[get("/inventory/all")]
pub async fn get_all_inventories(user: super::AuthenticatedUser,
        inv_rep: &State<InventoryRepository>, usr_rep: &State<UserRepository>) -> Result<Json<GetAllInventoriesReturn>>  {
    let allinvs = inv_rep.get_all_inventories().await?
        .into_iter()
        .filter(|i| i.reader.contains(&user.user_id))
        .collect::<Vec<_>>();
    Ok(Json(GetAllInventoriesReturn{
        inventories: allinvs
    }))
}

#[get("/inventory?<params..>")]
pub async fn get_specific_inventory(params: InventoryUUIDParams,  user: super::AuthenticatedUser,
    inv_rep: &State<InventoryRepository>) -> Result<Json<FullInventory>> {
    if !inv_rep.get_readers(&params.inventory_uuid).await?.contains(&user.user_id) {
        return Err(anyhow!("No access"))
    }
    Ok(Json(inv_rep.get_full_inventory(&params.inventory_uuid).await?))
}

#[derive(FromForm)]
pub struct InventoryCreateParams {
    name: String
}

#[put("/inventory?<params..>")]
pub async fn create_inventory(params: InventoryCreateParams,  user: super::AuthenticatedUser, inv_rep: &State<InventoryRepository>,
        ) -> Result<Json<FullInventory>> {
    let inv = inv_rep.create_inventory(&user.user_id, 0, &params.name).await?;
    get_specific_inventory(InventoryUUIDParams {inventory_uuid: inv.uuid}, user, inv_rep).await
}

#[derive(FromForm)]
pub struct InventoryAddItemByPresetParams {
    inventory_uuid: String,
    preset_uuid: String,
    amount:i32
}

#[put("/inventory/item/addPreset?<params..>")]
pub async fn add_preset_to_inventory(params: InventoryAddItemByPresetParams,  user: super::AuthenticatedUser,
        inv_rep: &State<InventoryRepository>, usr_rep: &State<UserRepository>) -> Result<Status> {
    if !acc_con.user_has_write_access_to_inventory(params.inventory_uuid.clone(), user.user_id)? {
        return Err(new_cstat_from_ref(Status::Forbidden, "Not Authorized"))
    }
    inv_con.add_preset_to_inventory(params.inventory_uuid, params.preset_uuid, params.amount)?;
    Ok(Status::NoContent)
}

#[derive(FromForm)]
pub struct InventoryAddItemByNameParams {
    inventory_uuid: String,
    name: String,
    amount:i32
}

#[put("/inventory/item/addNew?<params..>")]
pub async fn add_new_item_to_inventory(params:InventoryAddItemByNameParams,  user: super::AuthenticatedUser,
        inv_rep: &State<InventoryRepository>, usr_rep: &State<UserRepository>) -> Result<Json<ItemPreset>> {
    if !acc_con.user_has_write_access_to_inventory(params.inventory_uuid.clone(), user.user_id.clone())? {
        return Err(new_cstat_from_ref(Status::Forbidden, "Not Authorized"))
    }
    Ok(Json(inv_con.add_new_item_to_inventory(params.inventory_uuid, params.name, params.amount, user.user_id)?))
    
}

#[derive(FromForm)]
pub struct ItemEditParams {
    inventory_uuid: String,
    item_preset_uuid: String,
    amount: Option<i32>,
    sorting: Option<i32>,
    inventory_item_note: Option<String>
}

#[patch("/inventory/item/edit?<params..>")]
pub async fn edit_item(params: ItemEditParams, user: super::AuthenticatedUser, inv_rep: &State<InventoryRepository>,
        usr_rep: &State<UserRepository>) -> Result<Status> {
    if !acc_con.user_has_write_access_to_inventory(params.inventory_uuid.clone(), user.user_id.clone())? {
        return Err(new_cstat_from_ref(Status::Forbidden, "Not Authorized"))
    }
    inv_con.edit_inventory_item(params.inventory_uuid, params.item_preset_uuid, params.amount, params.sorting, params.inventory_item_note)?;
    Ok(Status::NoContent)
}

#[derive(FromForm)]
pub struct NoteAddParams {
    item_preset_uuid: String,
    inventory_uuid: String,
    note: String
}

#[patch("/inventory/item/addNote?<params..>")]
pub async fn add_note_to_item(params: NoteAddParams, user: super::AuthenticatedUser, inv_rep: &State<InventoryRepository>,
        usr_rep: &State<UserRepository>) -> Result<Status> {
    if !acc_con.user_is_dm(user.user_id.clone())? {
        return Err(new_cstat_from_ref(Status::Forbidden, "Not Authorized"))
    }
    inv_con.edit_item_dm_note(params.inventory_uuid, params.item_preset_uuid, params.note)?;
    Ok(Status::NoContent)
}

#[derive(FromForm)]
pub struct ItemDeleteParams {
    inventory_uuid: String,
    item_preset_uuid: String
}

#[delete("/inventory/item/remove?<params..>")]
pub async fn delete_item_from_inventory(params: ItemDeleteParams, user: super::AuthenticatedUser,
        inv_rep: &State<InventoryRepository>, usr_rep: &State<UserRepository>) -> Result<Status> {
    if !acc_con.user_has_write_access_to_inventory(params.inventory_uuid.clone(), user.user_id.clone())? {
        return Err(new_cstat_from_ref(Status::Forbidden, "Not Authorized"))
    }
    inv_con.delete_item_from_inventory(params.inventory_uuid, params.item_preset_uuid)?;
    Ok(Status::NoContent)
}

#[derive(Debug, FromForm)]
pub struct InventoryEditParams {
    inventory_uuid:String,
    amount: Option<i32>,
    name: Option<String>
}

#[patch("/inventory/edit?<params..>")]
pub async fn modify_money(params: InventoryEditParams,  user: super::AuthenticatedUser,
        inv_rep: &State<InventoryRepository>, usr_rep: &State<UserRepository>) -> Result<Status> {
    if !acc_con.user_has_write_access_to_inventory(params.inventory_uuid.clone(), user.user_id.clone())? {
        return Err(new_cstat_from_ref(Status::Forbidden, "Not Authorized"))
    }
    inv_con.edit_inventory(params.inventory_uuid, params.amount, params.name)?;
    Ok(Status::NoContent)
}

#[derive(FromForm)]
pub struct InventoryShareParams {
    inventory_uuid: String,
    reader_uuid: Option<String>,
    writer_uuid: Option<String>
}

#[patch("/inventory/addShare?<params..>")]
pub async fn add_share_to_inventory(params: InventoryShareParams,  user: super::AuthenticatedUser,
        inv_rep: &State<InventoryRepository>, usr_rep: &State<UserRepository>) -> Result<Status> {
    if !inv_con.is_creator_of_inventory(params.inventory_uuid.clone(), user.user_id.clone())? {
        return Err(new_cstat_from_ref(Status::Forbidden, "Not Authorized"));
    }
    let reader = if params.reader_uuid == None && params.writer_uuid != None &&
        !acc_con.user_has_read_access_to_inventory(params.inventory_uuid.clone(), params.writer_uuid.clone().unwrap())?
        {params.writer_uuid.clone()} else {params.reader_uuid};
    let writer = params.writer_uuid;
    if reader == None && writer == None {
        let users = (acc_con.get_all_users()?).into_iter().map(|x| x.uuid.clone());
        let current_readers = inv_con.get_readers_for_inventory(params.inventory_uuid.clone())?;
        for reader in users {
            if current_readers.contains(&reader) {
                continue;
            }
            inv_con.add_reader_to_inventory(params.inventory_uuid.clone(), reader)?;
        }
    }
    if let Some(reader) = reader {
        inv_con.add_reader_to_inventory(params.inventory_uuid.clone(), reader)?;
    }
    if let Some(writer) = writer {
        inv_con.add_writer_to_inventory(params.inventory_uuid.clone(), writer)?;
    }
    Ok(Status::NoContent)
}

#[patch("/inventory/removeShare?<params..>")]
pub async fn remove_share_from_inventory(params: InventoryShareParams,  user: super::AuthenticatedUser,
        inv_rep: &State<InventoryRepository>) -> Result<Status> {
    let reader = params.reader_uuid;
    let writer = params.writer_uuid;
    let some_own_user = Some(user.user_id.clone());
    if !inv_con.is_creator_of_inventory(params.inventory_uuid.clone(), user.user_id.clone())? &&
     reader.clone() != some_own_user && writer.clone() != some_own_user {
        return Err(new_cstat_from_ref(Status::Forbidden, "Not Authorized"));
    }

    if let Some(reader) = reader {
        inv_con.remove_reader_from_inventory(params.inventory_uuid.clone(), reader)?;
    }
    if let Some(writer) = writer {
        inv_con.remove_writer_from_inventory(params.inventory_uuid.clone(), writer)?;
    }
    
    Ok(Status::NoContent)
}

#[delete("/inventory/delete?<params..>")]
pub async fn delete_inventory(params:InventoryUUIDParams,  user: super::AuthenticatedUser,
        inv_rep: &State<InventoryRepository>) -> Result<Status> {
    if !inv_con.is_creator_of_inventory(params.inventory_uuid.clone(), user.user_id.clone())? {
        return Err(new_cstat_from_ref(Status::Forbidden, "Not Authorized"));
    }
    inv_con.delete_inventory(params.inventory_uuid)?;
    Ok(Status::NoContent)
}
