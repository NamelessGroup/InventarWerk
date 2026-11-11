use repos::{model_zauberwerk::SpellSlot, repos::zauberwek::spell_slot_repository::SpellSlotRepository};
use rocket::{State, http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use utils::TargetUser;
use::rocket_errors::anyhow::Result;
use utoipa::{IntoParams, OpenApi, ToSchema};

use super::OwnerUUID;

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

If `owner_uuid` is given, the spell slots of that user are returned instead. This is only allowed for DMs, unless it is the requesting user's own UUID."#,
    params(OwnerUUID),
    responses(
        (status = 200, description = "Returns the spell slots", body = SpellSlotResponse),
        (status = 403, description = "Forbidden - foreign owner_uuid given but user is not a DM"),
        (status = 500, description = "Database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Spell Slots"
)]
#[get("/spellSlot")]
pub async fn get_spell_slots(
    target: TargetUser,
    ssl_rep: &State<SpellSlotRepository>) -> Result<Json<SpellSlotResponse>>{
    let ssl = ssl_rep.get_all_spell_slots(&target.owner_uuid).await?;
    Ok(Json(SpellSlotResponse {
        owner_uuid: target.owner_uuid,
        slots: ssl
    }))
}

#[derive(Serialize, Deserialize, FromForm, ToSchema, IntoParams)]
pub struct PatchSpellSlotParams {
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

If `owner_uuid` is given, the spell slots of that user are modified instead. This is only allowed for DMs, unless it is the requesting user's own UUID."#,
    params(OwnerUUID, PatchSpellSlotParams),
    responses(
        (status = 204, description = "Available spell slots successfully set"),
        (status = 403, description = "Forbidden - foreign owner_uuid given but user is not a DM"),
        (status = 500, description = "Database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Spell Slots"
)]
#[patch("/spellSlot/available?<params..>")]
pub async fn patch_available_spell_slots(
    target: TargetUser,
    params: PatchSpellSlotParams,
    ssl_rep: &State<SpellSlotRepository>) -> Result<Status> {
    ssl_rep.modify_current_spell_slots(&target.owner_uuid, params.level, params.amount).await?;
    Ok(Status::NoContent)
}

#[utoipa::path(
    patch,
    path = "/spellSlot/maxima",
    summary = "Set the maximum spell slots of a level",
    description = r#"Sets the maximum number of spell slots for the given level.

If `owner_uuid` is given, the spell slots of that user are modified instead. This is only allowed for DMs, unless it is the requesting user's own UUID."#,
    params(OwnerUUID, PatchSpellSlotParams),
    responses(
        (status = 204, description = "Maximum spell slots successfully set"),
        (status = 403, description = "Forbidden - foreign owner_uuid given but user is not a DM"),
        (status = 500, description = "Database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Spell Slots"
)]
#[patch("/spellSlot/maxima?<params..>")]
pub async fn patch_maxima_spell_slots(
    target: TargetUser,
    params: PatchSpellSlotParams,
    ssl_rep: &State<SpellSlotRepository>) -> Result<Status> {
    ssl_rep.modify_max_spell_slots(&target.owner_uuid, params.level, params.amount).await?;
    Ok(Status::NoContent)
}

#[utoipa::path(
    patch,
    path = "/spellSlot/reset",
    summary = "Reset the spell slots of a user",
    description = r#"Resets the available spell slots of all levels to their maxima (e.g. after a long rest).

If `owner_uuid` is given, the spell slots of that user are reset instead. This is only allowed for DMs, unless it is the requesting user's own UUID."#,
    params(OwnerUUID),
    responses(
        (status = 204, description = "Spell slots successfully reset"),
        (status = 403, description = "Forbidden - foreign owner_uuid given but user is not a DM"),
        (status = 500, description = "Database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Spell Slots"
)]
#[patch("/spellSlot/reset")]
pub async fn reset_spell_slots(
    target: TargetUser,
    ssl_rep: &State<SpellSlotRepository>) -> Result<Status>{
    ssl_rep.reset_spell_slots(&target.owner_uuid).await?;
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
            SpellSlotResponse,
            PatchSpellSlotParams
        )
    ),
    tags(
        (name = "Spell Slots", description = "Endpoints for managing the spell slots of users")
    )
)]
pub struct SpellSlotApiDoc;
