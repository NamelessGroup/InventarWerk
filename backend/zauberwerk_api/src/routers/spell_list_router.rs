use utils::ApiDocAddon;
use repos::{model_zauberwerk::{SpellList, SpellListEntry}, repos::{user_repository::UserRepository, zauberwek::{spell_list_repository::SpellListRepository, spell_preset_repository::SpellPresetRepository}}};
use rocket::{State, http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use utils::{ApiError, AuthenticatedUser, user_is_dm};
use utoipa::{IntoParams, OpenApi, ToSchema};
use utils::Result;

#[derive(Serialize, Deserialize, FromForm, IntoParams)]
pub struct SpellListUUID {
    /// UUID of the spell list
    spell_list_uuid: String
}

#[utoipa::path(
    get,
    path = "/spellList",
    summary = "Retrieve a specific spell list",
    description = r#"Returns the spell list with the given UUID.

Only the owner of the list or a DM may read it."#,
    params(SpellListUUID),
    responses(
        (status = 200, description = "Returns the requested spell list", body = SpellList),
        (status = 403, description = "Forbidden - user is neither the owner nor a DM"),
        (status = 404, description = "Spell list not found"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Spell Lists"
)]
#[get("/spellList?<params..>")]
pub async fn get_spell_list(
    user: AuthenticatedUser,
    sli_rep: &State<SpellListRepository>,
    usr_rep: &State<UserRepository>,
    params: SpellListUUID) -> Result<Json<SpellList>> {
    let list = sli_rep.get_spell_list(&params.spell_list_uuid).await?;
    if list.owner_uuid != user.user_id && !user_is_dm(usr_rep, user.user_id).await? {
        return Err(ApiError::forbidden());
    }
    Ok(Json(list))
}


#[derive(Serialize, Deserialize, FromForm, IntoParams)]
pub struct SpellListNameParam {
    /// Name of the new spell list
    spell_list_name: String
}

#[utoipa::path(
    put,
    path = "/spellList",
    summary = "Create a new spell list",
    description = "Creates a new, empty spell list owned by the requesting user.",
    params(SpellListNameParam),
    responses(
        (status = 200, description = "The newly created spell list", body = SpellList),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Spell Lists"
)]
#[put("/spellList?<params..>")]
pub async fn create_spelllist(
    user: AuthenticatedUser,
    sli_rep: &State<SpellListRepository>,
    params: SpellListNameParam) -> Result<Json<SpellList>>{
    let spell_list_uuid: String = sli_rep.create_spell_list(&user.user_id, &params.spell_list_name).await?;
    Ok(Json(sli_rep.get_spell_list(&spell_list_uuid).await?))
}


#[derive(Serialize, Deserialize, ToSchema)]
pub struct AllSpellListResponse {
    spell_lists: Vec<SpellList>
}

#[utoipa::path(
    get,
    path = "/spellList/all",
    summary = "Retrieve all accessible spell lists",
    description = r#"Returns all spell lists the requesting user may read:
- DMs receive the spell lists of all users.
- All other users receive their own spell lists."#,
    responses(
        (status = 200, description = "Returns the accessible spell lists", body = AllSpellListResponse),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Spell Lists"
)]
#[get("/spellList/all")]
pub async fn get_all_spell_list(
    user: AuthenticatedUser,
    sli_rep: &State<SpellListRepository>,
    usr_rep: &State<UserRepository>) -> Result<Json<AllSpellListResponse>> {
    let lists = if user_is_dm(usr_rep, user.user_id.clone()).await? {
        sli_rep.get_all_spell_lists().await?
    } else {
        sli_rep.get_spell_list_for_owner(&user.user_id).await?
    };
    Ok(Json(AllSpellListResponse { spell_lists: lists }))
}

#[utoipa::path(
    delete,
    path = "/spellList/delete",
    summary = "Delete a spell list",
    description = "Deletes the spell list with the given UUID, including all of its entries. The spell presets themselves are not deleted. Only the owner of the list may delete it.",
    params(SpellListUUID),
    responses(
        (status = 204, description = "Spell list successfully deleted"),
        (status = 403, description = "Forbidden - user is not the owner of the list"),
        (status = 404, description = "Spell list not found"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Spell Lists"
)]
#[delete("/spellList/delete?<params..>")]
pub async fn delete_spell_list(
    user: AuthenticatedUser,
    sli_rep: &State<SpellListRepository>,
    params: SpellListUUID) -> Result<Status> {
    let list = sli_rep.get_spell_list(&params.spell_list_uuid).await?;
    if user.user_id != list.owner_uuid {
        return Err(ApiError::forbidden());
    }
    sli_rep.delete_spell_list(&params.spell_list_uuid).await?;
    Ok(Status::NoContent)
}

#[derive(Serialize, Deserialize, FromForm, IntoParams)]
pub struct SpellListEditParam {
    /// UUID of the spell list
    spell_list_uuid: String,
    /// New name of the spell list
    spell_list_name: String
}

#[utoipa::path(
    patch,
    path = "/spellList/edit",
    summary = "Rename a spell list",
    description = "Renames the spell list with the given UUID. Only the owner of the list may rename it.",
    params(SpellListEditParam),
    responses(
        (status = 204, description = "Spell list successfully renamed"),
        (status = 403, description = "Forbidden - user is not the owner of the list"),
        (status = 404, description = "Spell list not found"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Spell Lists"
)]
#[patch("/spellList/edit?<params..>")]
pub async fn patch_spell_list(
    user: AuthenticatedUser,
    sli_rep: &State<SpellListRepository>,
    params: SpellListEditParam) -> Result<Status> {
    let list = sli_rep.get_spell_list(&params.spell_list_uuid).await?;
    if user.user_id != list.owner_uuid {
        return Err(ApiError::forbidden());
    }
    sli_rep.rename_spell_list(&params.spell_list_uuid, &params.spell_list_name).await?;
    Ok(Status::NoContent)
}

#[derive(Serialize, Deserialize, FromForm, IntoParams)]
pub struct SpellListPresetParam {
    /// UUID of the spell list
    spell_list_uuid: String,
    /// UUID of the spell preset
    spell_preset_uuid: String
}

#[derive(Serialize, Deserialize, FromForm, IntoParams)]
pub struct SpellListAddPresetParam {
    /// UUID of the spell list
    spell_list_uuid: String,
    /// UUID of the spell preset
    spell_preset_uuid: String,
    /// Position of the spell in the list. Defaults to the end of the list.
    sorting: Option<i32>,
}

#[utoipa::path(
    put,
    path = "/spellList/spell/addPreset",
    summary = "Add a spell preset to a spell list",
    description = "Adds the spell preset with the given UUID to the spell list. Does nothing if the preset is already in the list. Only the owner of the list may modify it.",
    params(SpellListAddPresetParam),
    responses(
        (status = 204, description = "Spell preset successfully added"),
        (status = 403, description = "Forbidden - user is not the owner of the list"),
        (status = 404, description = "Spell preset not found"),
        (status = 404, description = "Spell list not found"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Spell Lists"
)]
#[put("/spellList/spell/addPreset?<params..>")]
pub async fn add_preset_to_list(
    user: AuthenticatedUser,
    sli_rep: &State<SpellListRepository>,
    spp_rep: &State<SpellPresetRepository>,
    params: SpellListAddPresetParam) -> Result<Status> {
    let list = sli_rep.get_spell_list(&params.spell_list_uuid).await?;
    if user.user_id != list.owner_uuid {
        return Err(ApiError::forbidden());
    }
    if spp_rep.find_spell_preset(&params.spell_preset_uuid).await?.is_none() {
        return Err(ApiError::not_found("spell preset not found"));
    }
    sli_rep.add_preset_to_list(&params.spell_list_uuid, &params.spell_preset_uuid, params.sorting).await?;
    Ok(Status::NoContent)
}

#[derive(Serialize, Deserialize, FromForm, IntoParams)]
pub struct SpellListSortingParam {
    /// UUID of the spell list
    spell_list_uuid: String,
    /// UUID of the spell preset
    spell_preset_uuid: String,
    /// New position of the spell in the list
    sorting: i32,
}

#[utoipa::path(
    patch,
    path = "/spellList/spell/edit",
    summary = "Change the position of a spell in a spell list",
    description = r#"Sets the `sorting` of the spell preset in the spell list. Spells are returned in ascending order of `sorting`. Only the owner of the list may modify it.

The values do not need to be consecutive or unique; spells with the same `sorting` are ordered by their UUID. To move a spell, the client usually sends the new positions of all spells whose position changed."#,
    params(SpellListSortingParam),
    responses(
        (status = 204, description = "Position successfully set"),
        (status = 403, description = "Forbidden - user is not the owner of the list"),
        (status = 404, description = "Spell list not found, or the spell preset is not in the list"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Spell Lists"
)]
#[patch("/spellList/spell/edit?<params..>")]
pub async fn edit_spell_in_list(
    user: AuthenticatedUser,
    sli_rep: &State<SpellListRepository>,
    params: SpellListSortingParam) -> Result<Status> {
    let list = sli_rep.get_spell_list(&params.spell_list_uuid).await?;
    if user.user_id != list.owner_uuid {
        return Err(ApiError::forbidden());
    }
    if !sli_rep.set_sorting(&params.spell_list_uuid, &params.spell_preset_uuid, params.sorting).await? {
        return Err(ApiError::not_found("spell preset is not in the spell list"));
    }
    Ok(Status::NoContent)
}

#[utoipa::path(
    delete,
    path = "/spellList/spell/removePreset",
    summary = "Remove a spell preset from a spell list",
    description = "Removes the spell preset with the given UUID from the spell list. Only the owner of the list may modify it.",
    params(SpellListPresetParam),
    responses(
        (status = 204, description = "Spell preset successfully removed"),
        (status = 403, description = "Forbidden - user is not the owner of the list"),
        (status = 404, description = "Spell list not found"),
        (status = 500, description = "Database error")
    ),
    security(("cookie_auth" = [])),
    tag = "Spell Lists"
)]
#[delete("/spellList/spell/removePreset?<params..>")]
pub async fn delete_preset_from_list(
    user: AuthenticatedUser,
    sli_rep: &State<SpellListRepository>,
    params: SpellListPresetParam) -> Result<Status> {
    let list = sli_rep.get_spell_list(&params.spell_list_uuid).await?;
    if user.user_id != list.owner_uuid {
        return Err(ApiError::forbidden());
    }
    sli_rep.remove_preset_from_list(&params.spell_list_uuid, &params.spell_preset_uuid).await?;
    Ok(Status::NoContent)
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&ApiDocAddon),
    paths(
        get_spell_list,
        create_spelllist,
        get_all_spell_list,
        delete_spell_list,
        patch_spell_list,
        add_preset_to_list,
        edit_spell_in_list,
        delete_preset_from_list
    ),
    components(
        schemas(
            SpellList,
            SpellListEntry,
            AllSpellListResponse,
        )
    ),
    tags(
        (name = "Spell Lists", description = "Endpoints for managing spell lists of users")
    )
)]
pub struct SpellListApiDoc;
