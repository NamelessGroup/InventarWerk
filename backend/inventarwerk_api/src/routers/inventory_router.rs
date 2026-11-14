use utils::ApiDocAddon;
use repos::model_inventarwerk::{FullFrontendInventory, ItemPreset};
use repos::repos::inventory_repository::InventoryRepository;
use repos::repos::item_preset_repository::ItemPresetRepository;
use repos::repos::user_repository::UserRepository;
use rocket::http::Status;
use rocket::{form::FromForm, serde::json::Json, State};
use utils::Result;
use serde::{Deserialize, Serialize};


use utils::AuthenticatedUser;

use utoipa::IntoParams;
use utoipa::OpenApi;
use utoipa::ToSchema;

use utils::{ApiError, DmUser};

use super::router_utility::{user_has_write_access_to_inventory, user_is_creator_of_inventory};

#[derive(FromForm, IntoParams)]
pub struct InventoryUUIDParams {
    /// UUID of the inventory
    inventory_uuid: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GetAllInventoriesReturn {
    inventories: Vec<FullFrontendInventory>,
}

#[utoipa::path(
    get,
    path = "/inventory/all",
    summary = "Retrieve all inventories for the authenticated user",
    description = r#"Retrieves all inventories associated with the authenticated user.  
Requires authentication. Returns an error if retrieval fails."#,
    responses(
        (status = 200, description = "All inventories of the user", body = GetAllInventoriesReturn),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Inventories"
)]
#[get("/inventory/all")]
pub async fn get_all_inventories(
    user: AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
) -> Result<Json<GetAllInventoriesReturn>> {
    let allinvs = inv_rep.get_all_inventories(&user.user_id).await?;
    Ok(Json(GetAllInventoriesReturn {
        inventories: allinvs,
    }))
}

#[utoipa::path(
    get,
    path = "/inventory",
    summary = "Retrieve a specific inventory",
    description = r#"Retrieves the detailed inventory by UUID.
Requires authentication. Returns an error if the user lacks access."#,
    params(InventoryUUIDParams),
    responses(
        (status = 200, description = "Specific inventory details", body = FullFrontendInventory),
        (status = 403, description = "Forbidden - no read access"),
        (status = 404, description = "Inventory not found"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Inventories"
)]
#[get("/inventory?<params..>")]
pub async fn get_specific_inventory(
    params: InventoryUUIDParams,
    user: AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
) -> Result<Json<FullFrontendInventory>> {
    let inv = inv_rep.get_full_inventory(&params.inventory_uuid).await?;
    if !inv.reader.contains(&user.user_id) && !(inv.owner_uuid == user.user_id) {
        return Err(ApiError::forbidden());
    }
    Ok(Json(inv))
}

#[derive(FromForm, IntoParams)]
pub struct InventoryCreateParams {
    /// Name of the new inventory
    name: String,
}

#[utoipa::path(
    put,
    path = "/inventory",
    summary = "Create a new inventory",
    description = r#"Creates a new inventory owned by the requesting user. All DMs automatically get read and write access.
Requires authentication."#,
    params(InventoryCreateParams),
    responses(
        (status = 200, description = "The newly created inventory", body = FullFrontendInventory),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Inventories"
)]
#[put("/inventory?<params..>")]
pub async fn create_inventory(
    params: InventoryCreateParams,
    user: AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
    usr_rep: &State<UserRepository>,
) -> Result<Json<FullFrontendInventory>> {
    let inv = inv_rep
        .create_inventory(&user.user_id, 0, &params.name)
        .await?;
    let dms = usr_rep.get_all_dm_ids().await?;
    for dm_id in dms {
        if dm_id == user.user_id {
            continue;
        }
        inv_rep.add_reader(&inv.uuid, &dm_id).await?;
        inv_rep.add_writer(&inv.uuid, &dm_id).await?;
    }
    get_specific_inventory(
        InventoryUUIDParams {
            inventory_uuid: inv.uuid,
        },
        user,
        inv_rep,
    )
    .await
}

#[derive(FromForm, IntoParams)]
pub struct InventoryAddItemByPresetParams {
    /// UUID of the inventory
    inventory_uuid: String,
    /// UUID of the item preset to add
    preset_uuid: String,
    /// Amount of the item
    amount: i32,
}

#[utoipa::path(
    put,
    path = "/inventory/item/addPreset",
    summary = "Add an item to an inventory by preset",
    description = r#"Adds an item to an inventory by providing a preset UUID.
Requires authentication and write access. Returns an error if access is denied."#,
    params(InventoryAddItemByPresetParams),
    responses(
        (status = 204, description = "Item added to inventory by preset"),
        (status = 403, description = "Forbidden - no write access"),
        (status = 404, description = "Inventory not found"),
        (status = 500, description = "Item preset does not exist, or database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Inventories"
)]
#[put("/inventory/item/addPreset?<params..>")]
pub async fn add_preset_to_inventory(
    params: InventoryAddItemByPresetParams,
    user: AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
) -> Result<Status> {
    if !user_has_write_access_to_inventory(
        inv_rep.inner(),
        params.inventory_uuid.clone(),
        user.user_id,
    )
    .await?
    {
        return Err(ApiError::forbidden());
    }
    inv_rep
        .add_inventory_item(
            &params.inventory_uuid,
            &params.preset_uuid,
            "",
            params.amount,
            0,
            "",
        )
        .await?;
    Ok(Status::NoContent)
}

#[derive(FromForm, IntoParams)]
pub struct InventoryAddItemByNameParams {
    /// UUID of the inventory
    inventory_uuid: String,
    /// Name of the new item preset
    name: String,
    /// Amount of the item
    amount: i32,
}

#[utoipa::path(
    put,
    path = "/inventory/item/addNew",
    summary = "Add a new item to an inventory by name",
    description = r#"Creates a new item preset from a name and adds it to the inventory.
Requires authentication and write access. Returns an error if access is denied."#,
    params(InventoryAddItemByNameParams),
    responses(
        (status = 200, description = "The created item preset", body = ItemPreset),
        (status = 403, description = "Forbidden - no write access"),
        (status = 404, description = "Inventory not found"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Inventories"
)]
#[put("/inventory/item/addNew?<params..>")]
pub async fn add_new_item_to_inventory(
    params: InventoryAddItemByNameParams,
    user: AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
    ipr_rep: &State<ItemPresetRepository>,
) -> Result<Json<ItemPreset>> {
    if !user_has_write_access_to_inventory(
        inv_rep.inner(),
        params.inventory_uuid.clone(),
        user.user_id.clone(),
    )
    .await?
    {
        return Err(ApiError::forbidden());
    }
    let id = ipr_rep
        .create_from_name(&params.name, &user.user_id)
        .await?;
    inv_rep
        .add_inventory_item(&params.inventory_uuid, &id, "", params.amount, 0, "")
        .await?;
    Ok(Json(ipr_rep.get_by_uuid(&id).await?))
}

#[derive(FromForm, IntoParams)]
pub struct ItemEditParams {
    /// UUID of the inventory
    inventory_uuid: String,
    /// UUID of the item preset of the item
    item_preset_uuid: String,
    /// New amount. Unchanged if omitted.
    amount: Option<i32>,
    /// New sorting position. Unchanged if omitted.
    sorting: Option<i32>,
    /// New note of the item, visible to everybody with read access. Unchanged if omitted.
    inventory_item_note: Option<String>,
}

#[utoipa::path(
    patch,
    path = "/inventory/item/edit",
    summary = "Edit an item in an inventory",
    description = r#"Edits item fields, such as amount or note.
Requires authentication and write access. Returns an error if access is denied."#,
    params(ItemEditParams),
    responses(
        (status = 204, description = "Item edited successfully"),
        (status = 403, description = "Forbidden - no write access"),
        (status = 404, description = "Inventory not found"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Inventories"
)]
#[patch("/inventory/item/edit?<params..>")]
pub async fn edit_item(
    params: ItemEditParams,
    user: AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
) -> Result<Status> {
    if !user_has_write_access_to_inventory(
        inv_rep.inner(),
        params.inventory_uuid.clone(),
        user.user_id.clone(),
    )
    .await?
    {
        return Err(ApiError::forbidden());
    }
    inv_rep
        .update_inventory_item(
            &params.inventory_uuid,
            &params.item_preset_uuid,
            None,
            params.amount,
            params.sorting,
            params.inventory_item_note.as_deref(),
        )
        .await?;
    Ok(Status::NoContent)
}

#[derive(FromForm, IntoParams)]
pub struct NoteAddParams {
    /// UUID of the item preset of the item
    item_preset_uuid: String,
    /// UUID of the inventory
    inventory_uuid: String,
    /// The DM note. Replaces any previous DM note.
    note: String,
}

#[utoipa::path(
    patch,
    path = "/inventory/item/addNote",
    summary = "Add a DM note to an item",
    description = r#"Adds a DM-only note to the specified inventory item.
Requires authentication and DM privileges."#,
    params(NoteAddParams),
    responses(
        (status = 204, description = "DM note successfully added"),
        (status = 403, description = "Forbidden - user is not a DM"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Inventories"
)]
#[patch("/inventory/item/addNote?<params..>")]
pub async fn add_note_to_item(
    params: NoteAddParams,
    _dm: DmUser,
    inv_rep: &State<InventoryRepository>,
) -> Result<Status> {
    inv_rep
        .update_inventory_item(
            &params.inventory_uuid,
            &params.item_preset_uuid,
            Some(&params.note),
            None,
            None,
            None,
        )
        .await?;
    Ok(Status::NoContent)
}

#[derive(FromForm, IntoParams)]
pub struct ItemMoveParams {
    /// UUID of the inventory that currently contains the item
    source_inventory_uuid: String,
    /// UUID of the inventory to move the item to
    target_inventory_uuid: String,
    /// UUID of the item preset of the item
    item_preset_uuid: String,
    /// Sorting position in the target inventory. Unchanged if omitted.
    new_sorting: Option<i32>,
}

#[utoipa::path(
    patch,
    path = "/inventory/item/move",
    summary = "Moves an item from one inventory to another",
    description = r#"Moves the specified item from one inventory to another.
Requires authentication and write access on both inventories. Returns an error if access is denied."#,
    params(ItemMoveParams),
    responses(
        (status = 204, description = "Item moved successfully"),
        (status = 400, description = "Source and target inventory are the same"),
        (status = 403, description = "Forbidden - no write access to one of the inventories"),
        (status = 404, description = "Inventory not found, or source inventory does not contain the item"),
        (status = 409, description = "Target inventory already contains the item"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Inventories"
)]
#[patch("/inventory/item/move?<params..>")]
pub async fn move_item_between_inventories(
    params: ItemMoveParams,
    user: AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
) -> Result<Status> {
    if params.source_inventory_uuid == params.target_inventory_uuid {
        return Err(ApiError::bad_request("source and target inventory should not be the same"))
    }
    if !user_has_write_access_to_inventory(
        inv_rep.inner(),
        params.source_inventory_uuid.clone(),
        user.user_id.clone(),
    )
    .await?
    {
        return Err(ApiError::forbidden());
    }
    if !user_has_write_access_to_inventory(
        inv_rep.inner(),
        params.target_inventory_uuid.clone(),
        user.user_id.clone(),
    )
    .await?
    {
        return Err(ApiError::forbidden());
    }
    if !inv_rep.item_exists(&params.source_inventory_uuid, &params.item_preset_uuid).await? {
        return Err(ApiError::not_found("source inventory does not contain the specified item"))
    }
    if inv_rep.item_exists(&params.target_inventory_uuid, &params.item_preset_uuid).await? {
        return Err(ApiError::conflict("target inventory already contains the specified item"))
    }

    inv_rep
        .move_inventory_item(&params.source_inventory_uuid, &params.target_inventory_uuid, &params.item_preset_uuid, params.new_sorting)
        .await?;
    Ok(Status::NoContent)
}

#[derive(FromForm, IntoParams)]
pub struct ItemDeleteParams {
    /// UUID of the inventory
    inventory_uuid: String,
    /// UUID of the item preset of the item
    item_preset_uuid: String,
}

#[utoipa::path(
    delete,
    path = "/inventory/item/remove",
    summary = "Remove an item from an inventory",
    description = r#"Removes the specified item from the inventory.
Requires authentication and write access. Returns an error if access is denied."#,
    params(ItemDeleteParams),
    responses(
        (status = 204, description = "Item removed successfully"),
        (status = 403, description = "Forbidden - no write access"),
        (status = 404, description = "Inventory not found"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Inventories"
)]
#[delete("/inventory/item/remove?<params..>")]
pub async fn delete_item_from_inventory(
    params: ItemDeleteParams,
    user: AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
) -> Result<Status> {
    if !user_has_write_access_to_inventory(
        inv_rep.inner(),
        params.inventory_uuid.clone(),
        user.user_id.clone(),
    )
    .await?
    {
        return Err(ApiError::forbidden());
    }
    inv_rep
        .remove_inventory_item(&params.inventory_uuid, &params.item_preset_uuid).await?;
        
    Ok(Status::NoContent)
}

#[derive(Debug, FromForm, IntoParams)]
pub struct InventoryEditParams {
    /// UUID of the inventory
    inventory_uuid: String,
    /// New amount of money in the inventory. Unchanged if omitted.
    amount: Option<i32>,
    /// New name of the inventory. Unchanged if omitted.
    name: Option<String>,
}

#[utoipa::path(
    patch,
    path = "/inventory/edit",
    summary = "Edit an inventory",
    description = r#"Changes the name and/or the money (`amount`) of an inventory.
Requires authentication and write access. Returns an error if access is denied."#,
    params(InventoryEditParams),
    responses(
        (status = 204, description = "Inventory edited successfully"),
        (status = 403, description = "Forbidden - no write access"),
        (status = 404, description = "Inventory not found"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Inventories"
)]
#[patch("/inventory/edit?<params..>")]
pub async fn edit_inventory(
    params: InventoryEditParams,
    user: AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
) -> Result<Status> {
    if !user_has_write_access_to_inventory(
        inv_rep.inner(),
        params.inventory_uuid.clone(),
        user.user_id.clone(),
    )
    .await?
    {
        return Err(ApiError::forbidden());
    }
    inv_rep
        .update_inventory(
            &params.inventory_uuid,
            params.amount,
            params.name.as_deref(),
        ).await?;
        
    Ok(Status::NoContent)
}

#[derive(FromForm, IntoParams)]
pub struct InventoryShareParams {
    /// UUID of the inventory
    inventory_uuid: String,
    /// User UUID to add/remove/update. If None, all users will be added as readers.
    member_uuid: Option<String>,
    /// "r" for read, "w" for write (write includes read), None for remove access.
    share_type: Option<String>,
}

#[utoipa::path(
    patch,
    path = "/inventory/share",
    summary = "Set share permissions for an inventory",
    description = r#"Sets the share status for a user or all users.
- If `member_uuid` is provided:
  - `share_type` = "w": User gets write and read access.
  - `share_type` = "r": User gets read access (write removed).
  - `share_type` = None: User loses all access.
- If `member_uuid` is NOT provided:
  - All users get read access.
Requires creator privileges (except for removing oneself)."#,
    params(InventoryShareParams),
    responses(
        (status = 204, description = "Permissions updated successfully"),
        (status = 400, description = "Invalid `share_type`"),
        (status = 403, description = "Forbidden - user is not the creator (and does not remove themself)"),
        (status = 404, description = "Inventory not found"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Inventories"
)]
#[patch("/inventory/share?<params..>")]
pub async fn set_inventory_share_permissions(
    params: InventoryShareParams,
    user: AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
    usr_rep: &State<UserRepository>,
) -> Result<Status> {
    if let Some(ref st) = params.share_type {
        if !matches!(st.as_str(), "r" | "w") {
            return Err(ApiError::bad_request("share_type must be \"r\" or \"w\""));
        }
    }

    match params.member_uuid {
        Some(member_uuid) => {
            let is_own = member_uuid == user.user_id;
            let is_creator = user_is_creator_of_inventory(inv_rep.inner(), params.inventory_uuid.clone(), user.user_id).await?;
            
            if params.share_type.is_some() && !is_creator {
                return Err(ApiError::forbidden());
            }
            if params.share_type.is_none() && !is_creator && !is_own {
                return Err(ApiError::forbidden());
            }


            match params.share_type.as_deref() {
                Some("w") => {
                    inv_rep.add_reader(&params.inventory_uuid, &member_uuid).await?;
                    inv_rep.add_writer(&params.inventory_uuid, &member_uuid).await?;
                }
                Some("r") => {
                    inv_rep.add_reader(&params.inventory_uuid, &member_uuid).await?;
                    inv_rep.remove_writer(&params.inventory_uuid, &member_uuid).await?;
                }
                None => {
                    inv_rep.remove_writer(&params.inventory_uuid, &member_uuid).await?;
                    inv_rep.remove_reader(&params.inventory_uuid, &member_uuid).await?;
                }
                _ => return Err(ApiError::bad_request("share_type must be \"r\" or \"w\"")),
            }
        }
        None => {
            if !user_is_creator_of_inventory(inv_rep.inner(), params.inventory_uuid.clone(), user.user_id).await? {
                return Err(ApiError::forbidden());
            }
            
            let current = inv_rep.get_readers(&params.inventory_uuid).await?;
            let all_users = usr_rep.get_all_users().await?;
            for user in all_users {
                if !current.contains(&user.uuid) {
                    inv_rep.add_reader(&params.inventory_uuid, &user.uuid).await?;
                }
            }
        }
    }

    Ok(Status::NoContent)
}



#[utoipa::path(
    delete,
    path = "/inventory/delete",
    summary = "Delete an inventory",
    description = r#"Deletes an entire inventory from the system.
Requires authentication and creator privileges. Returns an error if the user is not the creator."#,
    params(InventoryUUIDParams),
    responses(
        (status = 204, description = "Inventory deleted successfully"),
        (status = 403, description = "Forbidden - user is not the creator"),
        (status = 404, description = "Inventory not found"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Inventories"
)]
#[delete("/inventory/delete?<params..>")]
pub async fn delete_inventory(
    params: InventoryUUIDParams,
    user: AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
) -> Result<Status> {
    if !user_is_creator_of_inventory(
        inv_rep.inner(),
        params.inventory_uuid.clone(),
        user.user_id.clone(),
    )
    .await?
    {
        return Err(ApiError::forbidden());
    }
    inv_rep.delete_inventory(&params.inventory_uuid).await?;
    Ok(Status::NoContent)
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&ApiDocAddon),
    paths(
        get_all_inventories,
        get_specific_inventory,
        create_inventory,
        add_preset_to_inventory,
        add_new_item_to_inventory,
        edit_item,
        add_note_to_item,
        delete_item_from_inventory,
        move_item_between_inventories,
        edit_inventory,
        set_inventory_share_permissions,
        delete_inventory,
    ),
    components(
        schemas(
            GetAllInventoriesReturn,
            FullFrontendInventory,
            ItemPreset
        )
    ),
    tags(
        (name = "Inventories", description = "Endpoints for managing inventories.")
    )
)]
pub struct InventoryApiDoc;
