use repos::{model_zauberwerk::SpellSlot, repos::{user_repository::UserRepository, zauberwek::spell_slot_repository::SpellSlotRepository}};
use rocket::{State, http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use utils::AuthenticatedUser;
use::rocket_errors::anyhow::Result;
use utoipa::{IntoParams, OpenApi, ToSchema};

use utils::user_is_dm;

#[derive(Serialize, Deserialize, FromForm, ToSchema, IntoParams)]
pub struct OwnerUUID {
    /// UUID of the user whose spell slots should be used. If omitted, the requesting user is used. Only DMs may set this.
    owner_uuid: Option<String>
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SpellSlotResponse {
    owner_uuid: String,
    slots: Vec<SpellSlot>
}

#[utoipa::path(
    get,
    path = "/spellSlot",
    summary = "Retrieve the spell slots of a user",
    description = r#"Returns all spell slots (level 1-9) of the requesting user.

If `owner_uuid` is given, the spell slots of that user are returned instead. This is only allowed for DMs."#,
    params(OwnerUUID),
    responses(
        (status = 200, description = "Returns the spell slots", body = SpellSlotResponse),
        (status = 403, description = "Forbidden - owner_uuid given but user is not a DM"),
        (status = 500, description = "Database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Spell Slots"
)]
#[get("/spellSlot?<params..>")]
pub async fn get_spell_slots(
    user: AuthenticatedUser,
    params: OwnerUUID,
    usr_rep: &State<UserRepository>,
    ssl_rep: &State<SpellSlotRepository>) -> Result<Result<Json<SpellSlotResponse>, Status>>{
    if params.owner_uuid.is_some() && !user_is_dm(usr_rep, user.user_id.clone()).await? {
        return Ok(Err(Status::Forbidden));
    }
    let owner_uuid = params.owner_uuid.unwrap_or(user.user_id);
    let ssl = ssl_rep.get_all_spell_slots(&owner_uuid).await?;
    Ok(Ok(Json(SpellSlotResponse {
        owner_uuid,
        slots: ssl
    })))
}

#[derive(Serialize, Deserialize, FromForm, ToSchema, IntoParams)]
pub struct PatchSpellSlotParams {
    /// UUID of the user whose spell slots should be modified. If omitted, the requesting user is used. Only DMs may set this.
    owner_uuid: Option<String>,
    /// Spell slot level (1-9)
    level: i32,
    /// New amount for the given level
    amount: i32
}

#[utoipa::path(
    patch,
    path = "/spellSlot/available",
    summary = "Set the available spell slots of a level",
    description = r#"Sets the number of currently available spell slots for the given level.

If `owner_uuid` is given, the spell slots of that user are modified instead. This is only allowed for DMs."#,
    params(PatchSpellSlotParams),
    responses(
        (status = 204, description = "Available spell slots successfully set"),
        (status = 403, description = "Forbidden - owner_uuid given but user is not a DM"),
        (status = 500, description = "Database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Spell Slots"
)]
#[patch("/spellSlot/available?<params..>")]
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

#[utoipa::path(
    patch,
    path = "/spellSlot/maxima",
    summary = "Set the maximum spell slots of a level",
    description = r#"Sets the maximum number of spell slots for the given level.

If `owner_uuid` is given, the spell slots of that user are modified instead. This is only allowed for DMs."#,
    params(PatchSpellSlotParams),
    responses(
        (status = 204, description = "Maximum spell slots successfully set"),
        (status = 403, description = "Forbidden - owner_uuid given but user is not a DM"),
        (status = 500, description = "Database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Spell Slots"
)]
#[patch("/spellSlot/maxima?<params..>")]
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

#[utoipa::path(
    patch,
    path = "/spellSlot/reset",
    summary = "Reset the spell slots of a user",
    description = r#"Resets the available spell slots of all levels to their maxima (e.g. after a long rest).

If `owner_uuid` is given, the spell slots of that user are reset instead. This is only allowed for DMs."#,
    params(OwnerUUID),
    responses(
        (status = 204, description = "Spell slots successfully reset"),
        (status = 403, description = "Forbidden - owner_uuid given but user is not a DM"),
        (status = 500, description = "Database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Spell Slots"
)]
#[patch("/spellSlot/reset?<params..>")]
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

#[derive(OpenApi)]
#[openapi(
    paths(
        get_spell_slots,
        patch_available_spell_slots,
        patch_maxima_spell_slots,
        reset_spell_slots
    ),
    components(
        schemas(
            SpellSlot,
            OwnerUUID,
            SpellSlotResponse,
            PatchSpellSlotParams
        )
    ),
    tags(
        (name = "Spell Slots", description = "Endpoints for managing the spell slots of users")
    )
)]
pub struct SpellSlotApiDoc;
