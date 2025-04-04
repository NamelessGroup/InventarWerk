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