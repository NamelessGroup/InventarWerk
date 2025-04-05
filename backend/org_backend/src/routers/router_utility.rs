use inv_rep::repos::{inventory_repository::InventoryRepository, user_repository::UserRepository};
use rocket_errors::anyhow::Result;


pub static ACCESS_DENIAL_MESSAGE: &str = "no access";

pub async fn user_has_read_access_to_inventory(inv_rep: &InventoryRepository, inventory_uuid: String, user_id: String) -> Result<bool> {
    Ok(inv_rep.get_full_inventory(&inventory_uuid).await?.reader.contains(&user_id))
}
pub async fn user_has_write_access_to_inventory(inv_rep: &InventoryRepository, inventory_uuid: String, user_id: String) -> Result<bool> {
    Ok(inv_rep.get_full_inventory(&inventory_uuid).await?.writer.contains(&user_id))
}

pub async fn user_is_dm(usr_rep: &UserRepository, user_id: String) -> Result<bool> {
    Ok(usr_rep.get_user(&user_id).await?.dm == 1)
}

pub async fn user_is_creator_of_inventory(inv_rep: &InventoryRepository, inventory_uuid: String, user_id: String) -> Result<bool> {
    Ok(inv_rep.get_raw_inventory(&inventory_uuid).await?.owner_uuid == user_id)
}

pub async fn user_has_read_access_to_item_preset(inv_rep: &InventoryRepository, user_id: &str, searched_item_preset: &str) -> Result<bool>{
    let mut inventories = inv_rep.get_inventories_by_reader(user_id).await?;
    inv_rep.get_user_inventory_ids(user_id).await?.iter().for_each(|id| inventories.push(id.to_string()));
    has_access_to_item(inv_rep, searched_item_preset, inventories).await
}

pub async fn user_has_write_access_to_item_preset(inv_rep: &InventoryRepository, user_id: &str, searched_item_preset: &str) -> Result<bool>{
    let mut inventories = inv_rep.get_inventories_by_writer(user_id).await?;
    inv_rep.get_user_inventory_ids(user_id).await?.iter().for_each(|id| inventories.push(id.to_string()));
    has_access_to_item(inv_rep, searched_item_preset, inventories).await
}

pub async fn has_access_to_item(inv_rep: &InventoryRepository, searched_item_preset: &str, inventories: Vec<String>) -> Result<bool> {
    let mut has_access = false;
    for i in inventories {
        if inv_rep.item_exists(&i, searched_item_preset).await? {
            has_access = true
        }
    }
    return Ok(has_access)
}

