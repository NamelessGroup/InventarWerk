use repos::repos::{inventory_repository::InventoryRepository};
use rocket_errors::anyhow::Result;

pub static ACCESS_DENIAL_MESSAGE: &str = "no access";

/// Checks if a user has read access to a specific inventory.
///
/// # Arguments
/// * `inv_rep` - Reference to the inventory repository.
/// * `inventory_uuid` - The UUID of the inventory.
/// * `user_id` - The UUID of the user.
///
/// # Returns
/// `true` if the user is a reader of the inventory, otherwise `false`.
pub async fn user_has_read_access_to_inventory(
    inv_rep: &InventoryRepository,
    inventory_uuid: String,
    user_id: String,
) -> Result<bool> {
    Ok(inv_rep
        .get_full_inventory(&inventory_uuid)
        .await?
        .reader
        .contains(&user_id))
}

/// Checks if a user has write access to a specific inventory.
///
/// # Arguments
/// * `inv_rep` - Reference to the inventory repository.
/// * `inventory_uuid` - The UUID of the inventory.
/// * `user_id` - The UUID of the user.
///
/// # Returns
/// `true` if the user is a writer of the inventory, otherwise `false`.
pub async fn user_has_write_access_to_inventory(
    inv_rep: &InventoryRepository,
    inventory_uuid: String,
    user_id: String,
) -> Result<bool> {
    Ok(inv_rep
        .get_full_inventory(&inventory_uuid)
        .await?
        .writer
        .contains(&user_id))
}

/// Checks if a user is the creator (owner) of a specific inventory.
///
/// # Arguments
/// * `inv_rep` - Reference to the inventory repository.
/// * `inventory_uuid` - The UUID of the inventory.
/// * `user_id` - The UUID of the user.
///
/// # Returns
/// `true` if the user is the creator of the inventory, otherwise `false`.
pub async fn user_is_creator_of_inventory(
    inv_rep: &InventoryRepository,
    inventory_uuid: String,
    user_id: String,
) -> Result<bool> {
    Ok(inv_rep.get_raw_inventory(&inventory_uuid).await?.owner_uuid == user_id)
}

/// Checks if a user has read access to a specific item preset (by being a reader or owner of any inventory containing the preset).
///
/// # Arguments
/// * `inv_rep` - Reference to the inventory repository.
/// * `user_id` - The UUID of the user.
/// * `searched_item_preset` - The UUID of the item preset.
///
/// # Returns
/// `true` if the user has read access to the item preset, otherwise `false`.
pub async fn user_has_read_access_to_item_preset(
    inv_rep: &InventoryRepository,
    user_id: &str,
    searched_item_preset: &str,
) -> Result<bool> {
    let mut inventories = inv_rep.get_inventories_by_reader(user_id).await?;
    inv_rep
        .get_user_inventory_ids(user_id)
        .await?
        .iter()
        .for_each(|id| inventories.push(id.to_string()));
    has_access_to_item(inv_rep, searched_item_preset, inventories).await
}

/// Helper function to check if any of the given inventories contain the specified item preset.
///
/// # Arguments
/// * `inv_rep` - Reference to the inventory repository.
/// * `searched_item_preset` - The UUID of the item preset.
/// * `inventories` - A list of inventory UUIDs to check.
///
/// # Returns
/// `true` if the item preset exists in any of the inventories, otherwise `false`.
pub async fn has_access_to_item(
    inv_rep: &InventoryRepository,
    searched_item_preset: &str,
    inventories: Vec<String>,
) -> Result<bool> {
    let mut has_access = false;
    for i in inventories {
        if inv_rep.item_exists(&i, searched_item_preset).await? {
            has_access = true;
            break;
        }
    }
    return Ok(has_access);
}
