use utils::ApiDocAddon;
use std::time::Duration;

use repos::{
    model_inventarwerk::ItemPreset,
    repos::{
        inventory_repository::InventoryRepository, item_preset_repository::ItemPresetRepository,
    },
};
use rocket::{form::FromForm, http::Status, serde::json::Json, State};
use utils::Result;
use serde::{Deserialize, Serialize};

use utoipa::IntoParams;
use utoipa::OpenApi;
use utoipa::ToSchema;

use utils::{ApiError, DmUser};

use utils::AuthenticatedUser;

use super::{
    router_utility::user_has_read_access_to_item_preset,
};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GetItemPresetReturn {
    item_presets: Vec<ItemPreset>,
}

#[derive(FromForm, IntoParams)]
pub struct ItemPresetUUIDParams {
    /// UUID of the item preset
    item_preset_uuid: String,
}

#[utoipa::path(
    get,
    path = "/itemPreset",
    summary = "Retrieve a specific item preset",
    description = r#"Retrieves a specific item preset by UUID.

Public presets are readable by everybody, all other presets only if they are contained in an inventory the user owns or can read."#,
    params(ItemPresetUUIDParams),
    responses(
        (status = 200, description = "Returns the requested item preset", body = ItemPreset),
        (status = 403, description = "Forbidden - no read access"),
        (status = 404, description = "Item preset not found"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Item Presets"
)]
#[get("/itemPreset?<params..>")]
pub async fn get_item_preset(
    params: ItemPresetUUIDParams,
    user: AuthenticatedUser,
    ipr_rep: &State<ItemPresetRepository>,
    inv_rep: &State<InventoryRepository>,
) -> Result<Json<ItemPreset>> {
    let preset = ipr_rep.get_by_uuid(&params.item_preset_uuid).await?;
    if !preset.creator.starts_with("public")
        && !user_has_read_access_to_item_preset(
            inv_rep.inner(),
            &user.user_id,
            &params.item_preset_uuid,
        )
        .await?
    {
        return Err(ApiError::forbidden());
    }
    Ok(Json(preset))
}

#[derive(FromForm, IntoParams)]
pub struct ItemModifyParams {
    /// UUID of the item preset
    item_preset_uuid: String,
    /// New name. Unchanged if omitted.
    name: Option<String>,
    /// New price. Unchanged if omitted.
    price: Option<i32>,
    /// New weight. Unchanged if omitted.
    weight: Option<f32>,
    /// New description. Unchanged if omitted.
    description: Option<String>,
    /// New item type. Unchanged if omitted.
    item_type: Option<String>,
}

#[utoipa::path(
    patch,
    path = "/itemPreset/modify",
    summary = "Modify an existing item preset",
    description = r#"Modifies an item preset. Only the creator can modify their preset.

Requires authentication and creator privileges. Returns an error if the user is not the creator."#,
    params(ItemModifyParams),
    responses(
        (status = 204, description = "Item preset successfully modified"),
        (status = 403, description = "Forbidden - user is not the creator"),
        (status = 404, description = "Item preset not found"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Item Presets"
)]
#[patch("/itemPreset/modify?<params..>")]
pub async fn modify_item_preset(
    params: ItemModifyParams,
    user: AuthenticatedUser,
    ipr_rep: &State<ItemPresetRepository>,
) -> Result<Status> {
    let preset = ipr_rep.get_by_uuid(&params.item_preset_uuid).await?;
    if preset.creator != user.user_id {
        return Err(ApiError::forbidden());
    }
    ipr_rep
        .update_item_preset(
            &params.item_preset_uuid,
            params.name.as_deref(),
            params.price,
            params.weight,
            params.description.as_deref(),
            params.item_type.as_deref(),
        )
        .await?;
    Ok(Status::NoContent)
}

#[utoipa::path(
    delete,
    path = "/itemPreset/delete",
    summary = "Delete an existing item preset",
    description = r#"Deletes an item preset. Only the creator can delete their preset.

Requires authentication and creator privileges. Returns an error if the user is not the creator."#,
    params(ItemPresetUUIDParams),
    responses(
        (status = 204, description = "Item preset successfully deleted"),
        (status = 403, description = "Forbidden - user is not the creator"),
        (status = 404, description = "Item preset not found"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Item Presets"
)]
#[delete("/itemPreset/delete?<params..>")]
pub async fn delete_item_preset(
    params: ItemPresetUUIDParams,
    user: AuthenticatedUser,
    ipr_rep: &State<ItemPresetRepository>,
) -> Result<Status> {
    let preset = ipr_rep.get_by_uuid(&params.item_preset_uuid).await?;
    if preset.creator != user.user_id {
        return Err(ApiError::forbidden());
    }

    ipr_rep.delete(&params.item_preset_uuid).await?;
    Ok(Status::NoContent)
}

#[utoipa::path(
    get,
    path = "/itemPreset/all",
    summary = "Retrieve all accessible item presets",
    description = r#"Retrieves all public item presets and those in the user's inventories.

Requires authentication. Returns an error if the retrieval fails."#,
    responses(
        (status = 200, description = "Returns a list of all accessible item presets", body = GetItemPresetReturn),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Item Presets"
)]
#[get("/itemPreset/all")]
pub async fn get_all_item_presets(
    user: AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
    ipr_rep: &State<ItemPresetRepository>,
) -> Result<Json<GetItemPresetReturn>> {
    let mut item_presets = ipr_rep.get_public_presets().await?;
    let invs = inv_rep
        .get_owned_and_readable_inventory_ids(&user.user_id)
        .await?;
    for i in invs {
        item_presets.extend(ipr_rep.get_presets_in_inventory(&i).await?);
    }
    Ok(Json(GetItemPresetReturn {
        item_presets: item_presets,
    }))
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ExternPresetData {
    name: String,
    uuid: String,
    price: i32,
    weight: f32,
    description: String,
    creator: String,
    itemType: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ExternPresetDataList {
    presets: Vec<ExternPresetData>,
}

#[utoipa::path(
    put,
    path = "/itemPreset/addExtern",
    summary = "Import external item presets",
    description = r#"Imports external item presets from the provided JSON list.

Requires DM privileges. Retries up to 5 times on creation errors, then skips the preset."#,
    request_body = ExternPresetDataList,
    responses(
        (status = 204, description = "External item presets successfully imported"),
        (status = 403, description = "Forbidden - user is not a DM"),
        (status = 422, description = "Request body is missing fields or has invalid values")
    ),
    security(("cookie_auth" = [])),
    tag = "Item Presets"
)]
#[put("/itemPreset/addExtern", data = "<json_data>")]
pub async fn add_extern(
    json_data: Json<ExternPresetDataList>,
    _dm: DmUser,
    ipr_rep: &State<ItemPresetRepository>,
) -> Result<Status> {
    for x in &json_data.presets {
        let mut i = 0;
        loop {
            let preset = ItemPreset {
                uuid: "".to_string(),
                name: x.name.clone(),
                price: x.price,
                weight: x.weight,
                description: x.description.clone(),
                creator: x.creator.clone(),
                item_type: x.itemType.clone(),
                creation: None,
            };
            let res = ipr_rep.create(&preset).await;
            match res {
                Ok(_res) => break,
                Err(e) => {
                    println!("Error creating preset {}: {}", x.name, e);
                    rocket::tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
            i += 1;
            if i > 5 {
                print!("Skipped {}", x.name);
                break;
            }
        }
    }

    Ok(Status::NoContent)
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&ApiDocAddon),
    paths(
        get_item_preset,
        modify_item_preset,
        delete_item_preset,
        get_all_item_presets,
        add_extern
    ),
    components(
        schemas(
            GetItemPresetReturn,
            ItemPreset,
            ExternPresetData,
            ExternPresetDataList
        )
    ),
    tags(
        (name = "Item Presets", description = "Endpoints for managing item presets.")
    )
)]
pub struct ItemPresetApiDoc;
