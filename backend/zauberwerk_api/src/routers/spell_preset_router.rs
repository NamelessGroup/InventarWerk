use utils::ApiDocAddon;
use repos::{model_zauberwerk::SpellPreset, repos::{user_repository::UserRepository, zauberwek::spell_preset_repository::SpellPresetRepository}};
use rocket::tokio::sync::broadcast::Sender;
use rocket::{State, http::Status};

use super::spell_cast_router::{broadcast_concentration_ended, SpellEvent};
use utils::{ApiError, AuthenticatedUser, DmUser};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, OpenApi, ToSchema};
use utils::Result;
use rocket::serde::json::Json;


#[derive(Serialize, Deserialize, FromForm, IntoParams)]
pub struct SpellPresetUUID{
    /// UUID of the spell preset
    pub spell_preset_uuid: String
}

#[utoipa::path(
    get,
    path = "/spellPreset",
    summary = "Retrieve a specific spell preset",
    description = "Returns the spell preset with the given UUID. Accessible for every authenticated user.",
    params(SpellPresetUUID),
    responses(
        (status = 200, description = "Returns the requested spell preset", body = SpellPreset),
        (status = 404, description = "Spell preset not found"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Spell Presets"
)]
#[get("/spellPreset?<params..>")]
pub async fn get_spell_preset(
    _user: AuthenticatedUser,
    params: SpellPresetUUID,
    spp_rep: &State<SpellPresetRepository>) -> Result<Json<SpellPreset>> {
    Ok(Json(spp_rep.get_spell_preset(&params.spell_preset_uuid).await?))
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AllSpellPresetResult {
    spell_presets: Vec<SpellPreset>
}

#[utoipa::path(
    get,
    path = "/spellPreset/all",
    summary = "Retrieve all spell presets",
    description = "Returns all spell presets. Accessible for every authenticated user.",
    responses(
        (status = 200, description = "Returns all spell presets", body = AllSpellPresetResult),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Spell Presets"
)]
#[get("/spellPreset/all")]
pub async fn get_all_spell_presets(
    _user: AuthenticatedUser,
    spp_rep: &State<SpellPresetRepository>) -> Result<Json<AllSpellPresetResult>> {
    Ok(Json(AllSpellPresetResult {
        spell_presets: spp_rep.get_all_spell_presets().await?
    }))
}

#[utoipa::path(
    delete,
    path = "/spellPreset/delete",
    summary = "Delete a spell preset",
    description = r#"Deletes the spell preset with the given UUID. The preset is also removed from all spell lists.

All concentrations on the preset are ended; for each of them a `concentrationEnded` event is broadcast on `GET /spellPreset/cast/stream`.

Only accessible for DMs."#,
    params(SpellPresetUUID),
    responses(
        (status = 204, description = "Spell preset successfully deleted"),
        (status = 403, description = "Forbidden - user is not a DM"),
        (status = 404, description = "Spell preset not found"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Spell Presets"
)]
#[delete("/spellPreset/delete?<params..>")]
pub async fn delete_spell_preset(
    dm: DmUser,
    params: SpellPresetUUID,
    spp_rep: &State<SpellPresetRepository>,
    usr_rep: &State<UserRepository>,
    queue: &State<Sender<SpellEvent>>) -> Result<Status> {
    // Fetched before deleting, as it is part of the concentrationEnded events.
    let Some(spell) = spp_rep.find_spell_preset(&params.spell_preset_uuid).await? else {
        return Err(ApiError::not_found("spell preset not found"));
    };
    let ended_concentrations = spp_rep.delete_spell_preset(&params.spell_preset_uuid).await?;
    for owner_uuid in ended_concentrations {
        broadcast_concentration_ended(queue, usr_rep, owner_uuid, dm.user_id.clone(), spell.clone()).await?;
    }
    Ok(Status::NoContent)
}

#[utoipa::path(
    patch,
    path = "/spellPreset/modify",
    summary = "Modify a spell preset",
    description = r#"Replaces all fields of the spell preset identified by `uuid` in the request body with the given data.

The preset keeps its UUID, so spell lists and concentrations referencing it are not affected.

Only accessible for DMs."#,
    request_body(content = SpellPreset, description = "The complete, modified spell preset", content_type = "application/json"),
    responses(
        (status = 204, description = "Spell preset successfully modified"),
        (status = 403, description = "Forbidden - user is not a DM"),
        (status = 404, description = "No spell preset with the given `uuid` exists"),
        (status = 422, description = "Request body is missing fields or has invalid values"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Spell Presets"
)]
#[patch("/spellPreset/modify", data = "<json_data>")]
pub async fn edit_spell_preset(
    _dm: DmUser,
    spp_rep: &State<SpellPresetRepository>,
    json_data: Json<SpellPreset>) -> Result<Status> {
    if !spp_rep.modify_spell_preset(json_data.into_inner()).await? {
        return Err(ApiError::not_found("spell preset not found"));
    }
    Ok(Status::NoContent)
}


#[derive(Serialize, Deserialize, ToSchema)]
pub struct ExternSpellPreset {
    spell_presets: Vec<SpellPreset>
}

#[utoipa::path(
    put,
    path = "/spellPreset/addExtern",
    summary = "Import spell presets",
    description = r#"Creates new spell presets from the given list (e.g. for importing external spell data). The `uuid` field of each preset is ignored; new UUIDs are generated.

Duplicate entries within `components`, `class`, `subClass`, `species` or `feats` are ignored.

Each preset is retried up to 5 times on failure and skipped afterwards, so the request succeeds even if individual presets could not be created.

Only accessible for DMs."#,
    request_body(content = ExternSpellPreset, description = "List of spell presets to import", content_type = "application/json"),
    responses(
        (status = 204, description = "Spell presets processed"),
        (status = 403, description = "Forbidden - user is not a DM"),
        (status = 422, description = "Request body is missing fields or has invalid values")
    ),
    security(("cookie_auth" = [])),
    tag = "Spell Presets"
)]
#[put("/spellPreset/addExtern", data = "<json_data>")]
pub async fn add_extern_spell_preset(
    _dm: DmUser,
    spp_rep: &State<SpellPresetRepository>,
    json_data: Json<ExternSpellPreset>) -> Result<Status> {

    for spell in &json_data.spell_presets {
        let mut i = 0;
        loop {
            // The uuid is ignored, a new one is generated on creation.
            let preset = SpellPreset {
                uuid: String::new(),
                ..spell.clone()
            };
            let res = spp_rep.create_spell_preset(preset).await;
            match res {
                Ok(_res) => break,
                Err(e) => {
                    println!("Error creating spell preset {}: {}", spell.name, e);
                    rocket::tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
            i += 1;
            if i > 5 {
                println!("Skipped {}", spell.name);
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
        get_spell_preset,
        get_all_spell_presets,
        delete_spell_preset,
        edit_spell_preset,
        add_extern_spell_preset
    ),
    components(
        schemas(
            SpellPreset,
            AllSpellPresetResult,
            ExternSpellPreset
        )
    ),
    tags(
        (name = "Spell Presets", description = "Endpoints for managing spell presets")
    )
)]
pub struct SpellPresetApiDoc;
