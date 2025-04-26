use std::thread;
use std::time::Duration;

use inv_rep::{model::ItemPreset, repos::{inventory_repository::InventoryRepository, item_preset_repository::ItemPresetRepository}};
use rocket::{form::FromForm, http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use rocket_errors::anyhow::Result;

use super::{create_error, router_utility::{user_has_read_access_to_item_preset, ACCESS_DENIAL_MESSAGE}};

//! # Item Preset Router
//!
//! This module provides endpoints for managing item presets, including retrieval, modification, deletion, and import of external presets.
//! All endpoints require authentication via `AuthenticatedUser` unless otherwise noted.
//!
//! ## Endpoints
//! - `GET /itemPreset`: Get a specific item preset by UUID.
//! - `PATCH /itemPreset/modify`: Modify an item preset (only by creator).
//! - `DELETE /itemPreset/delete`: Delete an item preset (only by creator).
//! - `GET /itemPreset/all`: Get all item presets accessible to the user.
//! - `PUT /itemPreset/addExtern`: Import external item presets.


#[derive(Serialize, Deserialize)]
pub struct GetItemPresetReturn{
    item_presets: Vec<ItemPreset>
}

#[derive(FromForm)]
pub struct ItemPresetUUIDParams {
    item_preset_uuid: String
}


/// Retrieves a specific item preset by UUID.
///
/// # Authentication
/// Requires authentication and read access.
///
/// # Errors
/// Returns an error if the user lacks access or the preset does not exist.
#[get("/itemPreset?<params..>")]
pub async fn get_item_preset(params: ItemPresetUUIDParams,  user: super::AuthenticatedUser, ipr_rep: &State<ItemPresetRepository>,
        inv_rep: &State<InventoryRepository>) -> Result<Json<ItemPreset>> {
    let preset = ipr_rep.get_by_uuid(&params.item_preset_uuid).await?;
    if !preset.creator.starts_with("public") && !user_has_read_access_to_item_preset(inv_rep.inner(), &user.user_id, &params.item_preset_uuid).await? {
        return Err(create_error(ACCESS_DENIAL_MESSAGE));
    }
    Ok(Json(preset))
}

#[derive(FromForm)]
pub struct ItemModifyParams {
    item_preset_uuid: String,
    name: Option<String>,
    price:Option<i32>,
    weight: Option<f32>,
    description: Option<String>,
    item_type: Option<String>
}

/// Modifies an item preset. Only the creator can modify their preset.
///
/// # Authentication
/// Requires authentication and creator privileges.
///
/// # Errors
/// Returns an error if the user is not the creator or the operation fails.
#[patch("/itemPreset/modify?<params..>")]
pub async fn modify_item_preset(params: ItemModifyParams,  user: super::AuthenticatedUser,
        ipr_rep: &State<ItemPresetRepository>) -> Result<Status> {
    let preset = ipr_rep.get_by_uuid(&params.item_preset_uuid).await?;
    if preset.creator != user.user_id {
        return Err(create_error(ACCESS_DENIAL_MESSAGE))
    }
    ipr_rep.update_item_preset(&params.item_preset_uuid, params.name.as_deref(), params.price, params.weight, params.description.as_deref(), params.item_type.as_deref()).await?;
    Ok(Status::NoContent)
}

/// Deletes an item preset. Only the creator can delete their preset.
///
/// # Authentication
/// Requires authentication and creator privileges.
///
/// # Errors
/// Returns an error if the user is not the creator or the operation fails.
#[delete("/itemPreset/delete?<params..>")]
pub async fn delete_item_preset(params: ItemPresetUUIDParams,  user: super::AuthenticatedUser,
        ipr_rep: &State<ItemPresetRepository>) -> Result<Status> {
    let preset = ipr_rep.get_by_uuid(&params.item_preset_uuid).await?;
    if preset.creator != user.user_id {
        return Err(create_error(ACCESS_DENIAL_MESSAGE));
    }

    ipr_rep.delete(&params.item_preset_uuid).await?;
    Ok(Status::NoContent)
}

/// Retrieves all item presets accessible to the user (public and those in the user's inventories).
///
/// # Authentication
/// Requires authentication.
///
/// # Errors
/// Returns an error if the retrieval fails.
#[get("/itemPreset/all")]
pub async fn get_all_item_presets(user: super::AuthenticatedUser, inv_rep: &State<InventoryRepository>,
        ipr_rep: &State<ItemPresetRepository>) -> Result<Json<GetItemPresetReturn>> {
    let mut item_presets = ipr_rep.get_public_presets().await?;
    let mut invs = inv_rep.get_user_inventory_ids(&user.user_id).await?;
    let read_invs = inv_rep.get_inventories_by_reader(&user.user_id).await?;
    invs.extend(read_invs);
    for i in invs {
        item_presets.extend(ipr_rep.get_presets_in_inventory(&i).await?);
    }
    Ok(Json(
        GetItemPresetReturn {
            item_presets: item_presets
        }
    ))
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ExternPresetData {
    name: String,
    uuid: String,
    price: i32,
    weight: f32,
    description: String,
    creator: String,
    itemType: String
}

#[derive(Serialize, Deserialize)]
pub struct ExternPresetDataList {
    presets: Vec<ExternPresetData>
}

/// Imports external item presets into the system.
///
/// # Authentication
/// Requires authentication.
///
/// # Errors
/// Retries up to 5 times per preset on failure, then skips the preset.
#[put("/itemPreset/addExtern", data="<json_data>")]
pub async fn add_extern(json_data: Json<ExternPresetDataList>, _user: super::AuthenticatedUser, ipr_rep: &State<ItemPresetRepository>)
    -> Result<Status>  {
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
                creation: None
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
