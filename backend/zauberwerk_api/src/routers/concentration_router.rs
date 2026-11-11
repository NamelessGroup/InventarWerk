use repos::{model_zauberwerk::Concentration, repos::zauberwek::concentration_repository::ConcentrationRepository};
use rocket::{State, http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use utils::{DmUser, TargetUser};

use super::OwnerUUID;
use rocket_errors::anyhow::Result;
use utoipa::{IntoParams, OpenApi, ToSchema};

#[utoipa::path(
    get,
    path = "/concentration",
    summary = "Retrieve the concentration of a user",
    description = r#"Returns the spell the requesting user is currently concentrating on. `spellPresetUuid` is `null` if the user is not concentrating.

If `owner_uuid` is given, the concentration of that user is returned instead. This is only allowed for DMs, unless it is the requesting user's own UUID."#,
    params(OwnerUUID),
    responses(
        (status = 200, description = "Returns the concentration", body = Concentration),
        (status = 403, description = "Forbidden - foreign owner_uuid given but user is not a DM"),
        (status = 500, description = "Database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Concentration"
)]
#[get("/concentration")]
pub async fn get_concentration(
    target: TargetUser,
    con_rep: &State<ConcentrationRepository>) -> Result<Json<Concentration>> {
    Ok(Json(con_rep.get_concentration(&target.owner_uuid).await?))
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
    _dm: DmUser,
    con_rep: &State<ConcentrationRepository>) -> Result<Json<ConcentrationReturnList>> {
    let cons = con_rep.get_all_concentrations().await?;

    Ok(Json(ConcentrationReturnList { concentrations: cons }))

}

#[derive(Serialize, Deserialize, FromForm, ToSchema, IntoParams)]
pub struct SetConcentrationParams {
    /// UUID of the spell preset the user is concentrating on. Replaces any previous concentration.
    spell_preset_uuid: String
}

#[utoipa::path(
    put,
    path = "/concentration",
    summary = "Set the concentration of a user",
    description = r#"Sets the spell the requesting user is concentrating on. A previous concentration is replaced.

Note: casting a concentration spell via `PUT /spellPreset/cast` sets the concentration automatically.

If `owner_uuid` is given and differs from the requesting user, DM privileges are required."#,
    params(OwnerUUID, SetConcentrationParams),
    responses(
        (status = 204, description = "Concentration successfully set"),
        (status = 403, description = "Forbidden - foreign owner_uuid given but user is not a DM"),
        (status = 500, description = "Spell preset does not exist or database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Concentration"
)]
#[put("/concentration?<params..>")]
pub async fn modify_concentration(
    target: TargetUser,
    params: SetConcentrationParams,
    con_rep: &State<ConcentrationRepository>) -> Result<Status> {
    con_rep.set_concentration(&target.owner_uuid, &params.spell_preset_uuid).await?;
    Ok(Status::NoContent)
}

#[utoipa::path(
    delete,
    path = "/concentration",
    summary = "End the concentration of a user",
    description = r#"Ends the concentration of the requesting user. Does nothing if the user is not concentrating.

If `owner_uuid` is given and differs from the requesting user, DM privileges are required."#,
    params(OwnerUUID),
    responses(
        (status = 204, description = "Concentration ended"),
        (status = 403, description = "Forbidden - foreign owner_uuid given but user is not a DM"),
        (status = 500, description = "Database error")
    ),
    security(("bearer_auth" = [])),
    tag = "Concentration"
)]
#[delete("/concentration")]
pub async fn remove_concentration(
    target: TargetUser,
    con_rep: &State<ConcentrationRepository>) -> Result<Status> {
    con_rep.remove_concentration(&target.owner_uuid).await?;
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
            ConcentrationReturnList,
            SetConcentrationParams
        )
    ),
    tags(
        (name = "Concentration", description = "Endpoints for managing the concentration of users")
    )
)]
pub struct ConcentrationApiDoc;
