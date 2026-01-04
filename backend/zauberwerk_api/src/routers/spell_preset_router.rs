use repos::{model_zauberwerk::SpellPreset, repos::{user_repository::UserRepository, zauberwek::spell_preset_repository::SpellPresetRepository}};
use rocket::{State, http::Status};
use utils::{AuthenticatedUser, user_is_dm};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use rocket_errors::anyhow::Result;
use rocket::serde::json::Json;


#[derive(Serialize, Deserialize, FromForm, ToSchema, IntoParams)]
pub struct SpellPresetUUID{
    pub spell_preset_uuid: String
}

//uuid
#[get("/spellpreset?<params..>")]
pub async fn get_spell_preset(
    _user: AuthenticatedUser,
    params: SpellPresetUUID,
    spp_rep: &State<SpellPresetRepository>) -> Result<Json<SpellPreset>> {
    Ok(Json(spp_rep.get_spell_preset(&params.spell_preset_uuid).await?))
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllSpellPresetResult {
    spell_presets: Vec<SpellPreset>
}

#[get("/spellpreset/all")]
pub async fn get_all_spell_presets(
    _user: AuthenticatedUser,
    spp_rep: &State<SpellPresetRepository>) -> Result<Json<AllSpellPresetResult>> {
    Ok(Json(AllSpellPresetResult { 
        spell_presets: spp_rep.get_all_spell_presets().await?
    }))
}

//uuid
#[delete("/spellpreset/delete?<params..>")]
pub async fn delete_spell_preset(
    _user: AuthenticatedUser,
    params: SpellPresetUUID,
    spp_rep: &State<SpellPresetRepository>) -> Result<Status> {
    spp_rep.delete_spell_preset(&params.spell_preset_uuid).await?;
    Ok(Status::NoContent)
}

#[patch("/spellPreset/modify", data = "<json_data>")]
pub async fn edit_spell_preset(
    user: AuthenticatedUser,
    usr_rep: &State<UserRepository>,
    spp_rep: &State<SpellPresetRepository>,
    json_data: Json<SpellPreset>) -> Result<Status> {
    if !user_is_dm(usr_rep, user.user_id).await? {
        return Ok(Status::Forbidden);
    }
    spp_rep.modify_spell_preset(json_data.into_inner()).await?;
    Ok(Status::NoContent)
}


//uuid, level
#[put("/spellpreset/cast")]
pub async fn cast_spell(_user: AuthenticatedUser) {
    //TODO: Implement SSE
}


#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
pub struct ExternSpellPreset {
    spell_presets: Vec<SpellPreset>
}

#[put("/spellpreset/addExtern", data = "<json_data>")]
pub async fn add_extern_spell_preset(
    user: AuthenticatedUser,
    usr_rep: &State<UserRepository>,
    spp_rep: &State<SpellPresetRepository>,
    json_data: Json<ExternSpellPreset>) -> Result<Status> {
    if !user_is_dm(usr_rep, user.user_id).await? {
        return Ok(Status::Forbidden);
    }

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