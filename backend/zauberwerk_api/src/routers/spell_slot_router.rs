use repos::{model_zauberwerk::SpellSlot, repos::{user_repository::UserRepository, zauberwek::spell_slot_repository::SpellSlotRepository}};
use rocket::{State, http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use utils::AuthenticatedUser;
use::rocket_errors::anyhow::Result;
use utoipa::ToSchema;

use utils::{user_is_dm, create_error};

#[derive(Serialize, Deserialize, FromForm, ToSchema)]
pub struct OwnerUUID {
    owner_uuid: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct SpellSlotResponse {
    owner_uuid: String,
    slots: Vec<SpellSlot>
}

#[get("/spellslot?<params..>")]
pub async fn get_spell_slots(
    user: AuthenticatedUser,
    params: OwnerUUID,
    usr_rep: &State<UserRepository>,
    ssl_rep: &State<SpellSlotRepository>) -> Result<Json<SpellSlotResponse>>{
    if params.owner_uuid.is_some() && !user_is_dm(usr_rep, user.user_id.clone()).await? {
        return Err(create_error("Forbidden"));
    }
    let ssl = match params.owner_uuid {
        Some(owner) =>ssl_rep.get_all_spell_slots(&owner).await?,
        None => ssl_rep.get_all_spell_slots(&user.user_id).await?
    };
    Ok(Json(SpellSlotResponse {
        owner_uuid: user.user_id,
        slots: ssl
    }))
}

#[derive(Serialize, Deserialize, FromForm, ToSchema)]
pub struct PatchSpellSlotParams {
    owner_uuid: Option<String>,
    level: i32,
    amount: i32
}

// (optional) uuid, 1-9
#[patch("/spellslot/available?<params..>")]
pub async fn patch_available_spell_slots(
    user: AuthenticatedUser,
    params: PatchSpellSlotParams,
    usr_rep: &State<UserRepository>,
    ssl_rep: &State<SpellSlotRepository>) -> Result<Status> {
    if params.owner_uuid.is_some() && !user_is_dm(usr_rep, user.user_id.clone()).await? {
        return Ok(Status::Forbidden);
    }
    match params.owner_uuid {
        Some(uuid) => ssl_rep.modify_current_spell_slots(&uuid, params.level, params.amount).await?,
        None => ssl_rep.modify_current_spell_slots(&user.user_id, params.level, params.amount).await?
    }
    Ok(Status::NoContent)
}
// (optional) uuid, 1-9
#[patch("/spellslot/maxima?<params..>")]
pub async fn patch_maxima_spell_slots(
    user: AuthenticatedUser,
    params: PatchSpellSlotParams,
    usr_rep: &State<UserRepository>,
    ssl_rep: &State<SpellSlotRepository>) -> Result<Status> {
    if params.owner_uuid.is_some() && !user_is_dm(usr_rep, user.user_id.clone()).await? {
        return Ok(Status::Forbidden);
    }
    match params.owner_uuid {
        Some(uuid) => ssl_rep.modify_max_spell_slots(&uuid, params.level, params.amount).await?,
        None => ssl_rep.modify_max_spell_slots(&user.user_id, params.level, params.amount).await?
    }
    Ok(Status::NoContent)
}
// (optionl) uuid
#[patch("/spellslot/reset?<params..>")]
pub async fn reset_spell_slots(
    user: AuthenticatedUser,
    params: OwnerUUID,
    usr_rep: &State<UserRepository>,
    ssl_rep: &State<SpellSlotRepository>) -> Result<Status>{
    if params.owner_uuid.is_some() && !user_is_dm(usr_rep, user.user_id.clone()).await? {
        return Ok(Status::Forbidden);
    }
    match params.owner_uuid {
        Some(owner) =>ssl_rep.reset_spell_slots(&owner).await?,
        None => ssl_rep.reset_spell_slots(&user.user_id).await?
    };
    Ok(Status::NoContent)
}

   
