use repos::model::{FullFrontendInventory, ItemPreset};
use repos::repos::inventory_repository::InventoryRepository;
use repos::repos::item_preset_repository::ItemPresetRepository;
use repos::repos::user_repository::UserRepository;
use rocket::http::Status;
use rocket::{form::FromForm, serde::json::Json, State};
use rocket_errors::anyhow::Result;
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;
use utoipa::IntoParams;
use utoipa::OpenApi;


use super::create_error;
use super::router_utility::{
    user_has_read_access_to_inventory, user_has_write_access_to_inventory,
    user_is_creator_of_inventory, user_is_dm, ACCESS_DENIAL_MESSAGE,
};

#[derive(FromForm, ToSchema, IntoParams)]
pub struct InventoryUUIDParams {
    inventory_uuid: String,
}

#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
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
        (status = 200, description = "All inventories of the user", body = GetAllInventoriesReturn)
    ),
    security(("bearer_auth" = [])),
    tag = "Inventories"
)]
#[get("/inventory/all")]
pub async fn get_all_inventories(
    user: super::AuthenticatedUser,
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
        (status = 200, description = "Specific inventory details", body = FullFrontendInventory)
    ),
    security(("bearer_auth" = [])),
    tag = "Inventories"
)]
#[get("/inventory?<params..>")]
pub async fn get_specific_inventory(
    params: InventoryUUIDParams,
    user: super::AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
) -> Result<Json<FullFrontendInventory>> {
    let inv = inv_rep.get_full_inventory(&params.inventory_uuid).await?;
    if !inv.reader.contains(&user.user_id) && !(inv.owner_uuid == user.user_id) {
        return Err(create_error(ACCESS_DENIAL_MESSAGE));
    }
    Ok(Json(inv))
}

#[derive(FromForm, ToSchema, IntoParams)]
pub struct InventoryCreateParams {
    name: String,
}

#[utoipa::path(
    put,
    path = "/inventory",
    summary = "Create a new inventory",
    description = r#"Creates a new inventory entry in the system.
Requires authentication. Returns an error if creation fails."#,
    params(InventoryCreateParams),
    responses(
        (status = 200, description = "The newly created inventory", body = FullFrontendInventory)
    ),
    security(("bearer_auth" = [])),
    tag = "Inventories"
)]
#[put("/inventory?<params..>")]
pub async fn create_inventory(
    params: InventoryCreateParams,
    user: super::AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
) -> Result<Json<FullFrontendInventory>> {
    let inv = inv_rep
        .create_inventory(&user.user_id, 0, &params.name)
        .await?;
    get_specific_inventory(
        InventoryUUIDParams {
            inventory_uuid: inv.uuid,
        },
        user,
        inv_rep,
    )
    .await
}

#[derive(FromForm, ToSchema, IntoParams)]
pub struct InventoryAddItemByPresetParams {
    inventory_uuid: String,
    preset_uuid: String,
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
        (status = 204, description = "Item added to inventory by preset")
    ),
    security(("bearer_auth" = [])),
    tag = "Inventories"
)]
#[put("/inventory/item/addPreset?<params..>")]
pub async fn add_preset_to_inventory(
    params: InventoryAddItemByPresetParams,
    user: super::AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
) -> Result<Status> {
    if !user_has_write_access_to_inventory(
        inv_rep.inner(),
        params.inventory_uuid.clone(),
        user.user_id,
    )
    .await?
    {
        return Err(create_error(ACCESS_DENIAL_MESSAGE));
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

#[derive(FromForm, ToSchema, IntoParams)]
pub struct InventoryAddItemByNameParams {
    inventory_uuid: String,
    name: String,
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
        (status = 200, description = "The created item preset", body = ItemPreset)
    ),
    security(("bearer_auth" = [])),
    tag = "Inventories"
)]
#[put("/inventory/item/addNew?<params..>")]
pub async fn add_new_item_to_inventory(
    params: InventoryAddItemByNameParams,
    user: super::AuthenticatedUser,
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
        return Err(create_error(ACCESS_DENIAL_MESSAGE));
    }
    let id = ipr_rep
        .create_from_name(&params.name, &user.user_id)
        .await?;
    inv_rep
        .add_inventory_item(&params.inventory_uuid, &id, "", params.amount, 0, "")
        .await?;
    Ok(Json(ipr_rep.get_by_uuid(&id).await?))
}

#[derive(FromForm, ToSchema, IntoParams)]
pub struct ItemEditParams {
    inventory_uuid: String,
    item_preset_uuid: String,
    amount: Option<i32>,
    sorting: Option<i32>,
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
        (status = 204, description = "Item edited successfully")
    ),
    security(("bearer_auth" = [])),
    tag = "Inventories"
)]
#[patch("/inventory/item/edit?<params..>")]
pub async fn edit_item(
    params: ItemEditParams,
    user: super::AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
) -> Result<Status> {
    if !user_has_write_access_to_inventory(
        inv_rep.inner(),
        params.inventory_uuid.clone(),
        user.user_id.clone(),
    )
    .await?
    {
        return Err(create_error(ACCESS_DENIAL_MESSAGE));
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

#[derive(FromForm, ToSchema, IntoParams)]
pub struct NoteAddParams {
    item_preset_uuid: String,
    inventory_uuid: String,
    note: String,
}

#[utoipa::path(
    patch,
    path = "/inventory/item/addNote",
    summary = "Add a DM note to an item",
    description = r#"Adds a DM-only note to the specified inventory item.
Requires authentication and DM privileges. Returns an error if user is not a DM."#,
    params(NoteAddParams),
    responses(
        (status = 204, description = "DM note successfully added")
    ),
    security(("bearer_auth" = [])),
    tag = "Inventories"
)]
#[patch("/inventory/item/addNote?<params..>")]
pub async fn add_note_to_item(
    params: NoteAddParams,
    user: super::AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
    usr_rep: &State<UserRepository>,
) -> Result<Status> {
    if !user_is_dm(usr_rep.inner(), user.user_id.clone()).await? {
        return Err(create_error(ACCESS_DENIAL_MESSAGE));
    }
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

#[derive(FromForm, ToSchema, IntoParams)]
pub struct ItemDeleteParams {
    inventory_uuid: String,
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
        (status = 204, description = "Item removed successfully")
    ),
    security(("bearer_auth" = [])),
    tag = "Inventories"
)]
#[delete("/inventory/item/remove?<params..>")]
pub async fn delete_item_from_inventory(
    params: ItemDeleteParams,
    user: super::AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
) -> Result<Status> {
    if !user_has_write_access_to_inventory(
        inv_rep.inner(),
        params.inventory_uuid.clone(),
        user.user_id.clone(),
    )
    .await?
    {
        return Err(create_error(ACCESS_DENIAL_MESSAGE));
    }
    inv_rep
        .remove_inventory_item(&params.inventory_uuid, &params.item_preset_uuid)
        .await?;
    Ok(Status::NoContent)
}

#[derive(Debug, FromForm, ToSchema, IntoParams)]
pub struct InventoryEditParams {
    inventory_uuid: String,
    amount: Option<i32>,
    name: Option<String>,
}

#[utoipa::path(
    patch,
    path = "/inventory/edit",
    summary = "Edit an inventory",
    description = r#"Changes properties of an inventory, such as its name or amount field.
Requires authentication and write access. Returns an error if access is denied."#,
    params(InventoryEditParams),
    responses(
        (status = 204, description = "Inventory edited successfully")
    ),
    security(("bearer_auth" = [])),
    tag = "Inventories"
)]
#[patch("/inventory/edit?<params..>")]
pub async fn edit_inventory(
    params: InventoryEditParams,
    user: super::AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
) -> Result<Status> {
    if !user_has_write_access_to_inventory(
        inv_rep.inner(),
        params.inventory_uuid.clone(),
        user.user_id.clone(),
    )
    .await?
    {
        return Err(create_error(ACCESS_DENIAL_MESSAGE));
    }
    inv_rep
        .update_inventory(
            &params.inventory_uuid,
            params.amount,
            params.name.as_deref(),
        )
        .await?;
    Ok(Status::NoContent)
}

#[derive(FromForm, ToSchema, IntoParams)]
pub struct InventoryShareParams {
    inventory_uuid: String,
    reader_uuid: Option<String>,
    writer_uuid: Option<String>,
}


#[utoipa::path(
    patch,
    path = "/inventory/addShare",
    summary = "Add share permissions to an inventory",
    description = r#"Adds reader or writer permissions for an inventory.
Requires authentication and creator privileges. Returns an error if user is not the creator."#,
    params(InventoryShareParams),
    responses(
        (status = 204, description = "Share permissions added successfully")
    ),
    security(("bearer_auth" = [])),
    tag = "Inventories"
)]
#[patch("/inventory/addShare?<params..>")]
pub async fn add_share_to_inventory(
    params: InventoryShareParams,
    user: super::AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
    usr_rep: &State<UserRepository>,
) -> Result<Status> {
    if user_is_creator_of_inventory(inv_rep.inner(), params.inventory_uuid.clone(), user.user_id)
        .await?
    {
        return Err(create_error(ACCESS_DENIAL_MESSAGE));
    }
    let reader = if params.reader_uuid == None
        && params.writer_uuid != None
        && user_has_read_access_to_inventory(
            inv_rep.inner(),
            params.inventory_uuid.clone(),
            params.writer_uuid.clone().unwrap(),
        )
        .await?
    {
        params.writer_uuid.clone()
    } else {
        params.reader_uuid
    };
    let writer = params.writer_uuid;
    if reader == None && writer == None {
        let users = (usr_rep.get_all_users().await?)
            .into_iter()
            .map(|x| x.uuid.clone());
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

#[utoipa::path(
    patch,
    path = "/inventory/removeShare",
    summary = "Remove share permissions from an inventory",
    description = r#"Removes reader or writer permissions from an inventory.
Requires authentication and creator privileges. Returns an error if user is not the creator."#,
    params(InventoryShareParams),
    responses(
        (status = 204, description = "Share permissions removed successfully")
    ),
    security(("bearer_auth" = [])),
    tag = "Inventories"
)]
#[patch("/inventory/removeShare?<params..>")]
pub async fn remove_share_from_inventory(
    params: InventoryShareParams,
    user: super::AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
) -> Result<Status> {
    let reader = params.reader_uuid;
    let writer = params.writer_uuid;
    let some_own_user = Some(user.user_id.clone());
    if user_is_creator_of_inventory(
        inv_rep.inner(),
        params.inventory_uuid.clone(),
        user.user_id.clone(),
    )
    .await?
        && reader.clone() != some_own_user
        && writer.clone() != some_own_user
    {
        return Err(create_error(ACCESS_DENIAL_MESSAGE));
    }

    if let Some(reader) = reader {
        inv_rep
            .remove_reader(&params.inventory_uuid, &reader)
            .await?;
    }
    if let Some(writer) = writer {
        inv_rep
            .remove_writer(&params.inventory_uuid, &writer)
            .await?;
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
        (status = 204, description = "Inventory deleted successfully")
    ),
    security(("bearer_auth" = [])),
    tag = "Inventories"
)]
#[delete("/inventory/delete?<params..>")]
pub async fn delete_inventory(
    params: InventoryUUIDParams,
    user: super::AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
) -> Result<Status> {
    if user_is_creator_of_inventory(
        inv_rep.inner(),
        params.inventory_uuid.clone(),
        user.user_id.clone(),
    )
    .await?
    {
        return Err(create_error(ACCESS_DENIAL_MESSAGE));
    }
    inv_rep.delete_inventory(&params.inventory_uuid).await?;
    Ok(Status::NoContent)
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_all_inventories,
        get_specific_inventory,
        create_inventory,
        add_preset_to_inventory,
        add_new_item_to_inventory,
        edit_item,
        add_note_to_item,
        delete_item_from_inventory,
        edit_inventory,
        add_share_to_inventory,
        remove_share_from_inventory,
        delete_inventory
    ),
    components(
        schemas(
            InventoryUUIDParams,
            InventoryCreateParams,
            InventoryAddItemByPresetParams,
            InventoryAddItemByNameParams,
            ItemEditParams,
            NoteAddParams,
            ItemDeleteParams,
            InventoryEditParams,
            InventoryShareParams,
            GetAllInventoriesReturn,
            FullFrontendInventory,
            ItemPreset
        )
    ),
    tags(
        (name = "Inventories", description = "Endpoints for managing inventories")
    )
)]
pub struct InventoryApiDoc;