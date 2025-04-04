use inv_rep::model::{FullInventory, ItemPreset};
use inv_rep::repos::inventory_repository::InventoryRepository;
use inv_rep::repos::item_preset_repository::ItemPresetRepository;
use inv_rep::repos::user_repository::UserRepository;
use rocket::http::Status;
use rocket::{form::FromForm, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use anyhow::anyhow;
use rocket_errors::anyhow::{AnyhowError, Result};

use super::router_utility::{user_has_read_access_to_inventory, user_has_write_access_to_inventory, user_is_creator_of_inventory, user_is_dm, ACCESS_DENIAL_MESSAGE};

#[derive(FromForm)]
pub struct InventoryUUIDParams {
    inventory_uuid: String
}

#[derive(Serialize, Deserialize)]
pub struct GetAllInventoriesReturn{
    inventories: Vec<FullInventory>
}

fn create_error(msg: &str) -> AnyhowError {
    anyhow!(msg.to_string()).into()
}

/// Handles the `/inventory/all` route, returns all inventories in the InventoryReturn form
#[get("/inventory/all")]
pub async fn get_all_inventories(user: super::AuthenticatedUser,
        inv_rep: &State<InventoryRepository>) -> Result<Json<GetAllInventoriesReturn>>  {
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
        return Err(create_error(ACCESS_DENIAL_MESSAGE));
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
        inv_rep: &State<InventoryRepository>) -> Result<Status> {
    if user_has_write_access_to_inventory(inv_rep.inner(), params.inventory_uuid.clone(), user.user_id).await? {
        return Err(create_error(ACCESS_DENIAL_MESSAGE))
    }
    inv_rep.add_inventory_item(&params.inventory_uuid, &params.preset_uuid, "", params.amount, 0, "").await?;
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
        inv_rep: &State<InventoryRepository>, ipr_rep: &State<ItemPresetRepository>) -> Result<Json<ItemPreset>> {
    if user_has_write_access_to_inventory(inv_rep.inner(), params.inventory_uuid.clone(), user.user_id.clone()).await? {
        return Err(create_error(ACCESS_DENIAL_MESSAGE))
    }
    let id = ipr_rep.create_from_name(&params.name, &user.user_id).await?;
    inv_rep.add_inventory_item(&params.inventory_uuid, &id, "", params.amount, 0, "").await?;
    Ok(Json(ipr_rep.get_by_uuid(&id).await?))
    
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
        ) -> Result<Status> {
    if user_has_write_access_to_inventory(inv_rep.inner(), params.inventory_uuid.clone(), user.user_id.clone()).await? {
        return Err(create_error(ACCESS_DENIAL_MESSAGE))
    }
    inv_rep.update_inventory_item(
        &params.inventory_uuid,
        &params.item_preset_uuid,
        None,
        params.amount,
        params.sorting,
        params.inventory_item_note.as_deref(),
    ).await?;
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
    if user_is_dm(usr_rep.inner(), user.user_id.clone()).await? {
        return Err(create_error(ACCESS_DENIAL_MESSAGE));
    }
    inv_rep.update_inventory_item(&params.inventory_uuid, &params.item_preset_uuid, Some(&params.note), None, None, None);
    Ok(Status::NoContent)
}

#[derive(FromForm)]
pub struct ItemDeleteParams {
    inventory_uuid: String,
    item_preset_uuid: String
}

#[delete("/inventory/item/remove?<params..>")]
pub async fn delete_item_from_inventory(params: ItemDeleteParams, user: super::AuthenticatedUser,
        inv_rep: &State<InventoryRepository>) -> Result<Status> {
    if user_has_write_access_to_inventory(inv_rep.inner(), params.inventory_uuid.clone(), user.user_id.clone()).await? {
        return Err(create_error(ACCESS_DENIAL_MESSAGE))
    }
    inv_rep.remove_inventory_item(&params.inventory_uuid, &params.item_preset_uuid).await?;
    Ok(Status::NoContent)
}

#[derive(Debug, FromForm)]
pub struct InventoryEditParams {
    inventory_uuid:String,
    amount: Option<i32>,
    name: Option<String>
}

#[patch("/inventory/edit?<params..>")]
pub async fn edit_inventory(params: InventoryEditParams,  user: super::AuthenticatedUser,
        inv_rep: &State<InventoryRepository>) -> Result<Status> {
    if user_has_write_access_to_inventory(inv_rep.inner(), params.inventory_uuid.clone(), user.user_id.clone()).await? {
        return Err(create_error(ACCESS_DENIAL_MESSAGE))
    }
    inv_rep.update_inventory(&params.inventory_uuid, params.amount, params.name.as_deref()).await?;
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
    if user_is_creator_of_inventory(inv_rep.inner(), params.inventory_uuid.clone(), user.user_id).await? {
        return Err(create_error(ACCESS_DENIAL_MESSAGE));
    }
    let reader = if params.reader_uuid == None && params.writer_uuid != None &&
        user_has_read_access_to_inventory(inv_rep.inner(), params.inventory_uuid.clone(), params.writer_uuid.clone().unwrap()).await?
        {params.writer_uuid.clone()} else {params.reader_uuid};
    let writer = params.writer_uuid;
    if reader == None && writer == None {
        let users = (usr_rep.get_all_users().await?).into_iter().map(|x| x.uuid.clone());
        let current_readers = inv_rep.get_readers(&params.inventory_uuid).await?;
        for reader in users {
            if current_readers.contains(&reader) {
                continue;
            }
            inv_rep.add_reader(&params.inventory_uuid, &reader).await?;
        }
    }
    if let Some(reader) = reader {
        inv_rep.add_reader(&params.inventory_uuid, &reader).await?;
    }
    if let Some(writer) = writer {
        inv_rep.add_writer(&params.inventory_uuid, &writer).await?;
    }
    Ok(Status::NoContent)
}

#[patch("/inventory/removeShare?<params..>")]
pub async fn remove_share_from_inventory(params: InventoryShareParams,  user: super::AuthenticatedUser,
        inv_rep: &State<InventoryRepository>) -> Result<Status> {
    let reader = params.reader_uuid;
    let writer = params.writer_uuid;
    let some_own_user = Some(user.user_id.clone());
    if user_is_creator_of_inventory(inv_rep.inner(), params.inventory_uuid.clone(), user.user_id.clone()).await? &&
     reader.clone() != some_own_user && writer.clone() != some_own_user {
        return Err(create_error(ACCESS_DENIAL_MESSAGE));
    }

    if let Some(reader) = reader {
        inv_rep.remove_reader(&params.inventory_uuid, &reader).await?;
    }
    if let Some(writer) = writer {
        inv_rep.remove_writer(&params.inventory_uuid, &writer).await?;
    }
    
    Ok(Status::NoContent)
}

#[delete("/inventory/delete?<params..>")]
pub async fn delete_inventory(params:InventoryUUIDParams,  user: super::AuthenticatedUser,
        inv_rep: &State<InventoryRepository>) -> Result<Status> {
    if user_is_creator_of_inventory(inv_rep.inner(), params.inventory_uuid.clone(), user.user_id.clone()).await? {
        return Err(create_error(ACCESS_DENIAL_MESSAGE));
    }
    inv_rep.delete_inventory(&params.inventory_uuid).await?;
    Ok(Status::NoContent)
}
