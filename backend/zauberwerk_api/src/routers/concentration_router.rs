use repos::{model_zauberwerk::Concentration, repos::{user_repository::UserRepository, zauberwek::concentration_repository::ConcentrationRepository}};
use rocket::{State, http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use utils::{AuthenticatedUser, user_is_dm};
use rocket_errors::anyhow::Result;
use utoipa::{IntoParams, OpenApi, ToSchema};

#[derive(FromForm, ToSchema, IntoParams)]
pub struct ConcentrationUUID{
    /// UUID of the user. If omitted, the requesting user is used. Accessing another user requires DM privileges.
    uuid: Option<String>
}

#[utoipa::path(
    get,
    path = "/concentration",
    summary = "Retrieve the concentration of a user",
    description = r#"Returns the spell the requesting user is currently concentrating on. `spellPresetUuid` is `null` if the user is not concentrating.

If `uuid` is given, the concentration of that user is returned instead. This is only allowed for DMs; other users receive an error."#,
    params(ConcentrationUUID),
    responses(
        (status = 200, description = "Returns the concentration", body = Concentration),
        (status = 403, description = "Forbidden - uuid given but user is not a DM"),
        (status = 500, description = "Database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Concentration"
)]
#[get("/concentration?<params..>")]
pub async fn get_concentration(
    user: AuthenticatedUser,
    usr_rep: &State<UserRepository>,
    con_rep: &State<ConcentrationRepository>,
    params: ConcentrationUUID) -> Result<Result<Json<Concentration>, Status>> {
    if params.uuid.is_some() && !user_is_dm(usr_rep, user.user_id.clone()).await? {
        return Ok(Err(Status::Forbidden));
    }

    let concentration = match params.uuid {
        Some(uuid) => con_rep.get_concentration(&uuid).await?,
        None => con_rep.get_concentration(&user.user_id).await?
    };

    Ok(Ok(Json(concentration)))
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ConcentrationReturnList {
    concentrations: Vec<Concentration>
}

#[utoipa::path(
    get,
    path = "/concentration/all",
    summary = "Retrieve the concentrations of all users",
    description = r#"Returns the concentrations of all users that are currently concentrating on a spell.

Only accessible for DMs."#,
    responses(
        (status = 200, description = "Returns all concentrations", body = ConcentrationReturnList),
        (status = 403, description = "Forbidden - user is not a DM"),
        (status = 500, description = "Database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Concentration"
)]
#[get("/concentration/all")]
pub async fn get_all_concentrations(
    user: AuthenticatedUser,
    usr_rep: &State<UserRepository>,
    con_rep: &State<ConcentrationRepository>) -> Result<Result<Json<ConcentrationReturnList>, Status>> {
    if !user_is_dm(usr_rep, user.user_id).await? {
        return Ok(Err(Status::Forbidden));
    }
    let cons = con_rep.get_all_concentrations().await?;

    Ok(Ok(Json(ConcentrationReturnList { concentrations: cons })))

}

#[derive(Serialize, Deserialize, FromForm, ToSchema, IntoParams)]
pub struct SetConcentrationParams {
    /// UUID of the user whose concentration should be set. If omitted, the requesting user is used. Setting another user's value requires DM privileges.
    uuid: Option<String>,
    /// UUID of the spell preset the user is concentrating on. Replaces any previous concentration.
    spell_preset_uuid: String
}

#[utoipa::path(
    put,
    path = "/concentration",
    summary = "Set the concentration of a user",
    description = r#"Sets the spell the requesting user is concentrating on. A previous concentration is replaced.

Note: casting a concentration spell via `PUT /spellPreset/cast` sets the concentration automatically.

If `uuid` is given and differs from the requesting user, DM privileges are required."#,
    params(SetConcentrationParams),
    responses(
        (status = 204, description = "Concentration successfully set"),
        (status = 403, description = "Forbidden - foreign uuid given but user is not a DM"),
        (status = 500, description = "Spell preset does not exist or database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Concentration"
)]
#[put("/concentration?<params..>")]
pub async fn modify_concentration(
    user: AuthenticatedUser,
    params: SetConcentrationParams,
    con_rep: &State<ConcentrationRepository>,
    usr_rep: &State<UserRepository>) -> Result<Status> {
    if params.uuid.is_some() && params.uuid.as_ref() != Some(&user.user_id) && !user_is_dm(usr_rep, user.user_id.clone()).await? {
        return Ok(Status::Forbidden);
    }
    let owner_uuid = params.uuid.unwrap_or(user.user_id);
    con_rep.set_concentration(&owner_uuid, &params.spell_preset_uuid).await?;
    Ok(Status::NoContent)
}

#[utoipa::path(
    delete,
    path = "/concentration",
    summary = "End the concentration of a user",
    description = r#"Ends the concentration of the requesting user. Does nothing if the user is not concentrating.

If `uuid` is given and differs from the requesting user, DM privileges are required."#,
    params(ConcentrationUUID),
    responses(
        (status = 204, description = "Concentration ended"),
        (status = 403, description = "Forbidden - foreign uuid given but user is not a DM"),
        (status = 500, description = "Database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Concentration"
)]
#[delete("/concentration?<params..>")]
pub async fn remove_concentration(
    user: AuthenticatedUser,
    params: ConcentrationUUID,
    con_rep: &State<ConcentrationRepository>,
    usr_rep: &State<UserRepository>) -> Result<Status> {
    if params.uuid.is_some() && params.uuid.as_ref() != Some(&user.user_id) && !user_is_dm(usr_rep, user.user_id.clone()).await? {
        return Ok(Status::Forbidden);
    }
    let owner_uuid = params.uuid.unwrap_or(user.user_id);
    con_rep.remove_concentration(&owner_uuid).await?;
    Ok(Status::NoContent)
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_concentration,
        get_all_concentrations,
        modify_concentration,
        remove_concentration
    ),
    components(
        schemas(
            Concentration,
            ConcentrationUUID,
            ConcentrationReturnList,
            SetConcentrationParams
        )
    ),
    tags(
        (name = "Concentration", description = "Endpoints for managing the concentration of users")
    )
)]
pub struct ConcentrationApiDoc;
