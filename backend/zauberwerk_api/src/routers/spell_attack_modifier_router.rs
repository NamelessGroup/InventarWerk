use repos::{
    model_zauberwerk::SpellAttackModifier,
    repos::zauberwek::spell_attack_modifier_repository::SpellAttackModifierRepository,
};
use rocket::{http::Status, serde::json::Json, State};
use rocket_errors::anyhow::Result;
use serde::{Deserialize, Serialize};
use utils::{DmUser, TargetUser};
use utoipa::{IntoParams, OpenApi, ToSchema};

use super::OwnerUUID;

#[utoipa::path(
    get,
    path = "/spellAttackModifier",
    summary = "Retrieve the spell attack modifier of a user",
    description = r#"Returns the spell attack modifier of the requesting user. `modifier` is `null` if no modifier was set yet.

If `owner_uuid` is given, the modifier of that user is returned instead. This is only allowed for DMs, unless it is the requesting user's own UUID."#,
    params(OwnerUUID),
    responses(
        (status = 200, description = "Returns the spell attack modifier", body = SpellAttackModifier),
        (status = 403, description = "Forbidden - foreign owner_uuid given but user is not a DM"),
        (status = 500, description = "Database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Spell Attack Modifier"
)]
#[get("/spellAttackModifier")]
pub async fn get_spell_attack_modifier(
    target: TargetUser,
    sam_rep: &State<SpellAttackModifierRepository>,
) -> Result<Json<SpellAttackModifier>> {
    Ok(Json(sam_rep.get_spell_attack_modifier(&target.owner_uuid).await?))
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SpellAttackModifierList {
    spell_attack_modifiers: Vec<SpellAttackModifier>,
}

#[utoipa::path(
    get,
    path = "/spellAttackModifier/all",
    summary = "Retrieve the spell attack modifiers of all users",
    description = r#"Returns the spell attack modifiers of all users that have one set.

Only accessible for DMs."#,
    responses(
        (status = 200, description = "Returns all spell attack modifiers", body = SpellAttackModifierList),
        (status = 403, description = "Forbidden - user is not a DM"),
        (status = 500, description = "Database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Spell Attack Modifier"
)]
#[get("/spellAttackModifier/all")]
pub async fn get_all_spell_attack_modifiers(
    _dm: DmUser,
    sam_rep: &State<SpellAttackModifierRepository>,
) -> Result<Json<SpellAttackModifierList>> {
    Ok(Json(SpellAttackModifierList {
        spell_attack_modifiers: sam_rep.get_all_spell_attack_modifiers().await?,
    }))
}

#[derive(FromForm, ToSchema, IntoParams)]
pub struct SetSpellAttackModifierParams {
    /// New spell attack modifier (may be negative)
    modifier: i32,
}

#[utoipa::path(
    put,
    path = "/spellAttackModifier",
    summary = "Set the spell attack modifier of a user",
    description = r#"Sets the spell attack modifier of the requesting user, replacing any previous value.

If `owner_uuid` is given and differs from the requesting user, DM privileges are required."#,
    params(OwnerUUID, SetSpellAttackModifierParams),
    responses(
        (status = 204, description = "Spell attack modifier successfully set"),
        (status = 403, description = "Forbidden - foreign owner_uuid given but user is not a DM"),
        (status = 500, description = "User does not exist or database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Spell Attack Modifier"
)]
#[put("/spellAttackModifier?<params..>")]
pub async fn set_spell_attack_modifier(
    target: TargetUser,
    params: SetSpellAttackModifierParams,
    sam_rep: &State<SpellAttackModifierRepository>,
) -> Result<Status> {
    sam_rep
        .set_spell_attack_modifier(&target.owner_uuid, params.modifier)
        .await?;
    Ok(Status::NoContent)
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_spell_attack_modifier,
        get_all_spell_attack_modifiers,
        set_spell_attack_modifier
    ),
    components(
        schemas(
            SpellAttackModifier,
            SpellAttackModifierList,
            SetSpellAttackModifierParams
        )
    ),
    tags(
        (name = "Spell Attack Modifier", description = "Endpoints for managing the spell attack modifier of users")
    )
)]
pub struct SpellAttackModifierApiDoc;
