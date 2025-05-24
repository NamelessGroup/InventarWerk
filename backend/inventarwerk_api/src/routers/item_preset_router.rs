use std::thread;
use std::time::Duration;

use repos::{
    model::ItemPreset,
    repos::{
        inventory_repository::InventoryRepository, item_preset_repository::ItemPresetRepository,
    },
};
use rocket::{form::FromForm, http::Status, serde::json::Json, State};
use rocket_errors::anyhow::Result;
use serde::{Deserialize, Serialize};

use utoipa::IntoParams;
use utoipa::OpenApi;
use utoipa::ToSchema;

use super::{
    create_error,
    router_utility::{user_has_read_access_to_item_preset, ACCESS_DENIAL_MESSAGE},
};

#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
pub struct GetItemPresetReturn {
    item_presets: Vec<ItemPreset>,
}

#[derive(FromForm, ToSchema, IntoParams)]
pub struct ItemPresetUUIDParams {
    item_preset_uuid: String,
}

#[utoipa::path(
    get,
    path = "/itemPreset",
    summary = "Retrieve a specific item preset",
    description = r#"Retrieves a specific item preset by UUID.

Requires authentication and read access. Returns an error if the user lacks access or the preset does not exist."#,
    params(ItemPresetUUIDParams),
    responses(
        (status = 200, description = "Returns the requested item preset", body = ItemPreset)
    ),
    security(("bearer_auth" = [])),
    tag = "Item Presets"
)]
#[get("/itemPreset?<params..>")]
pub async fn get_item_preset(
    params: ItemPresetUUIDParams,
    user: super::AuthenticatedUser,
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
        return Err(create_error(ACCESS_DENIAL_MESSAGE));
    }
    Ok(Json(preset))
}

#[derive(FromForm, ToSchema, IntoParams)]
pub struct ItemModifyParams {
    item_preset_uuid: String,
    name: Option<String>,
    price: Option<i32>,
    weight: Option<f32>,
    description: Option<String>,
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
        (status = 204, description = "Item preset successfully modified")
    ),
    security(("bearer_auth" = [])),
    tag = "Item Presets"
)]
#[patch("/itemPreset/modify?<params..>")]
pub async fn modify_item_preset(
    params: ItemModifyParams,
    user: super::AuthenticatedUser,
    ipr_rep: &State<ItemPresetRepository>,
) -> Result<Status> {
    let preset = ipr_rep.get_by_uuid(&params.item_preset_uuid).await?;
    if preset.creator != user.user_id {
        return Err(create_error(ACCESS_DENIAL_MESSAGE));
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
        (status = 204, description = "Item preset successfully deleted")
    ),
    security(("bearer_auth" = [])),
    tag = "Item Presets"
)]
#[delete("/itemPreset/delete?<params..>")]
pub async fn delete_item_preset(
    params: ItemPresetUUIDParams,
    user: super::AuthenticatedUser,
    ipr_rep: &State<ItemPresetRepository>,
) -> Result<Status> {
    let preset = ipr_rep.get_by_uuid(&params.item_preset_uuid).await?;
    if preset.creator != user.user_id {
        return Err(create_error(ACCESS_DENIAL_MESSAGE));
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
        (status = 200, description = "Returns a list of all accessible item presets", body = GetItemPresetReturn)
    ),
    security(("bearer_auth" = [])),
    tag = "Item Presets"
)]
#[get("/itemPreset/all")]
pub async fn get_all_item_presets(
    user: super::AuthenticatedUser,
    inv_rep: &State<InventoryRepository>,
    ipr_rep: &State<ItemPresetRepository>,
) -> Result<Json<GetItemPresetReturn>> {
    let mut item_presets = ipr_rep.get_public_presets().await?;
    let invs = inv_rep.get_owned_and_readable_inventory_ids(&user.user_id).await?;
    for i in invs {
        item_presets.extend(ipr_rep.get_presets_in_inventory(&i).await?);
    }
    Ok(Json(GetItemPresetReturn {
        item_presets: item_presets,
    }))
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
pub struct ExternPresetData {
    name: String,
    uuid: String,
    price: i32,
    weight: f32,
    description: String,
    creator: String,
    itemType: String,
}

#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
pub struct ExternPresetDataList {
    presets: Vec<ExternPresetData>,
}

#[utoipa::path(
    put,
    path = "/itemPreset/addExtern",
    summary = "Import external item presets",
    description = r#"Imports external item presets from the provided JSON list.

Requires authentication. Retries up to 5 times on creation errors, then skips the preset."#,
    request_body = ExternPresetDataList,
    responses(
        (status = 204, description = "External item presets successfully imported")
    ),
    security(("bearer_auth" = [])),
    tag = "Item Presets"
)]
#[put("/itemPreset/addExtern", data = "<json_data>")]
pub async fn add_extern(
    json_data: Json<ExternPresetDataList>,
    _user: super::AuthenticatedUser,
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
                    thread::sleep(Duration::from_secs(1));
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
            ItemPresetUUIDParams,
            ItemModifyParams,
            ItemPreset,
            ExternPresetData,
            ExternPresetDataList
        )
    ),
    tags(
        (name = "Item Presets", description = "Endpoints for managing item presets")
    )
)]
pub struct ItemPresetApiDoc;
