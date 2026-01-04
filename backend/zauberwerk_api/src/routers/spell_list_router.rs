use repos::{model_zauberwerk::SpellList, repos::{user_repository::UserRepository, zauberwek::spell_list_repository::SpellListRepository}};
use rocket::{State, futures::stream::All, http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use utils::{AuthenticatedUser, create_error, user_is_dm};
use utoipa::ToSchema;
use::rocket_errors::anyhow::Result;

#[derive(Serialize, Deserialize, FromForm, ToSchema)]
pub struct SpellListUUID {
    spell_list_uuid: String
}

#[get("/spelllist?<params..>")]
pub async fn get_spell_list(
    user: AuthenticatedUser,
    sli_rep: &State<SpellListRepository>,
    usr_rep: &State<UserRepository>,
    params: SpellListUUID) -> Result<Json<SpellList>> {
    let list = sli_rep.get_spell_list(&params.spell_list_uuid).await?;
    if list.owner_uuid != user.user_id && !user_is_dm(usr_rep, user.user_id).await? {
        return Err(create_error("Forbidden"));
    }
    Ok(Json(list))
}


#[derive(Serialize, Deserialize, FromForm, ToSchema)]
pub struct SpellListNameParam {
    spell_list_name: String
}

#[put("/spelllist?<params..>")]
pub async fn create_spelllist(
    user: AuthenticatedUser,
    sli_rep: &State<SpellListRepository>,
    params: SpellListNameParam) -> Result<Status>{
    sli_rep.create_spell_list(&user.user_id, &params.spell_list_name).await?;
    Ok(Status::NoContent)
}


#[derive(Serialize, Deserialize)]
pub struct AllSpellListResponse {
    spell_lists: Vec<SpellList>
}

#[get("/spelllist/all")]
pub async fn get_all_spell_list(
    user: AuthenticatedUser,
    sli_rep: &State<SpellListRepository>,
    usr_rep: &State<UserRepository>) -> Result<Json<AllSpellListResponse>> {
    if !user_is_dm(usr_rep, user.user_id).await? {
        return Err(create_error("Forbidden"));
    }
    let lists = sli_rep.get_all_spell_lists().await?;
    Ok(Json(AllSpellListResponse { spell_lists: lists }))
}

#[delete("/spelllist/delete?<params..>")]
pub async fn delete_spell_list(
    user: AuthenticatedUser,
    sli_rep: &State<SpellListRepository>,
    params: SpellListUUID) -> Result<Status> {
    let list = sli_rep.get_spell_list(&params.spell_list_uuid).await?;
    if user.user_id != list.owner_uuid {
        return Ok(Status::Forbidden);
    }
    sli_rep.delete_spell_list(&params.spell_list_uuid).await?;
    Ok(Status::NoContent)
}

#[derive(Serialize, Deserialize, FromForm, ToSchema)]
pub struct SpellListEditParam {
    spell_list_uuid: String,
    spell_list_name: String
}

#[patch("/spelllist/edit?<params..>")]
pub async fn patch_spell_list(
    user: AuthenticatedUser,
    sli_rep: &State<SpellListRepository>,
    params: SpellListEditParam) -> Result<Status> {
    let list = sli_rep.get_spell_list(&params.spell_list_uuid).await?;
    if user.user_id != list.owner_uuid {
        return Ok(Status::Forbidden);
    }
    sli_rep.rename_spell_list(&params.spell_list_uuid, &params.spell_list_name).await?;
    Ok(Status::NoContent)
}

#[derive(Serialize, Deserialize, FromForm, ToSchema)]
pub struct SpellListPresetParam {
    spell_list_uuid: String,
    spell_preset_uuid: String
}
#[put("/spelllist/spell/addPreset?<params..>")]
pub async fn add_preset_to_list(
    user: AuthenticatedUser,
    sli_rep: &State<SpellListRepository>,
    params: SpellListPresetParam) -> Result<Status> {
    let list = sli_rep.get_spell_list(&params.spell_list_uuid).await?;
    if user.user_id != list.owner_uuid {
        return Ok(Status::Forbidden);
    }
    sli_rep.add_preset_to_list(&params.spell_list_uuid, &params.spell_preset_uuid).await?;
    Ok(Status::NoContent)
}

#[delete("/spelllist/spell/removePreset?<params..>")]
pub async fn delete_preset_from_list(
    user: AuthenticatedUser,
    sli_rep: &State<SpellListRepository>,
    params: SpellListPresetParam) -> Result<Status> {
    let list = sli_rep.get_spell_list(&params.spell_list_uuid).await?;
    if user.user_id != list.owner_uuid {
        return Ok(Status::Forbidden);
    }
    sli_rep.remove_preset_from_list(&params.spell_list_uuid, &params.spell_preset_uuid).await?;
    Ok(Status::NoContent)
}
