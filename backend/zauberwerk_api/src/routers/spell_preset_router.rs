use repos::{model_zauberwerk::SpellPreset, repos::zauberwek::spell_preset_repository::SpellPresetRepository};
use rocket::{State, http::Status};
use utils::{AuthenticatedUser, DmUser};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, OpenApi, ToSchema};
use rocket_errors::anyhow::Result;
use rocket::serde::json::Json;


#[derive(Serialize, Deserialize, FromForm, ToSchema, IntoParams)]
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
        (status = 500, description = "Spell preset not found or database error")
    ),
    security(("bearer_auth" = [])),
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
    security(("bearer_auth" = [])),
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
    description = r#"Deletes the spell preset with the given UUID.

Only accessible for DMs."#,
    params(SpellPresetUUID),
    responses(
        (status = 204, description = "Spell preset successfully deleted"),
        (status = 403, description = "Forbidden - user is not a DM"),
        (status = 500, description = "Database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Spell Presets"
)]
#[delete("/spellPreset/delete?<params..>")]
pub async fn delete_spell_preset(
    _dm: DmUser,
    params: SpellPresetUUID,
    spp_rep: &State<SpellPresetRepository>) -> Result<Status> {
    spp_rep.delete_spell_preset(&params.spell_preset_uuid).await?;
    Ok(Status::NoContent)
}

#[utoipa::path(
    patch,
    path = "/spellPreset/modify",
    summary = "Modify a spell preset",
    description = r#"Replaces the spell preset identified by `uuid` in the request body with the given data.

Only accessible for DMs."#,
    request_body(content = SpellPreset, description = "The complete, modified spell preset", content_type = "application/json"),
    responses(
        (status = 204, description = "Spell preset successfully modified"),
        (status = 403, description = "Forbidden - user is not a DM"),
        (status = 500, description = "Database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Spell Presets"
)]
#[patch("/spellPreset/modify", data = "<json_data>")]
pub async fn edit_spell_preset(
    _dm: DmUser,
    spp_rep: &State<SpellPresetRepository>,
    json_data: Json<SpellPreset>) -> Result<Status> {
    spp_rep.modify_spell_preset(json_data.into_inner()).await?;
    Ok(Status::NoContent)
}


#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
pub struct ExternSpellPreset {
    spell_presets: Vec<SpellPreset>
}

#[utoipa::path(
    put,
    path = "/spellPreset/addExtern",
    summary = "Import spell presets",
    description = r#"Creates new spell presets from the given list (e.g. for importing external spell data). The `uuid` field of each preset is ignored; new UUIDs are generated.

Each preset is retried up to 5 times on failure and skipped afterwards, so the request succeeds even if individual presets could not be created.

Only accessible for DMs."#,
    request_body(content = ExternSpellPreset, description = "List of spell presets to import", content_type = "application/json"),
    responses(
        (status = 204, description = "Spell presets processed"),
        (status = 403, description = "Forbidden - user is not a DM")
    ),
    security(("bearer_auth" = [])),
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
            let preset = SpellPreset {
                uuid: "".to_string(),
                name: spell.name.clone(),
                level: spell.level,
                description: spell.description.clone(),
                time: spell.time.clone(),
                components: spell.components.clone(),
                concentration: spell.concentration,
                duration: spell.duration.clone(),
                school: spell.school.clone(),
                range: spell.range.clone(),
                class: spell.class.clone(),
                sub_class: spell.sub_class.clone(),
                species: spell.species.clone(),
                feats: spell.feats.clone(),
            };
            let res = spp_rep.create_spell_preset(preset).await;
            match res {
                Ok(_res) => break,
                Err(e) => {
                    println!("Error creating spell preset {}: {}", spell.name, e);
                    std::thread::sleep(std::time::Duration::from_secs(1));
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
            SpellPresetUUID,
            AllSpellPresetResult,
            ExternSpellPreset
        )
    ),
    tags(
        (name = "Spell Presets", description = "Endpoints for managing spell presets")
    )
)]
pub struct SpellPresetApiDoc;
