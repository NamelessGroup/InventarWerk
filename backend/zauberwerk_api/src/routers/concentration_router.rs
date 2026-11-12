use repos::{
    model_zauberwerk::Concentration,
    repos::{
        user_repository::UserRepository,
        zauberwek::{concentration_repository::ConcentrationRepository, spell_preset_repository::SpellPresetRepository},
    },
};
use rocket::tokio::sync::broadcast::Sender;
use rocket::{State, http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use utils::{DmUser, TargetUser};

use super::OwnerUUID;
use super::spell_cast_router::{broadcast_concentration_ended, SpellEvent};
use rocket_errors::anyhow::Result;
use utoipa::{OpenApi, ToSchema};

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

#[utoipa::path(
    delete,
    path = "/concentration",
    summary = "End the concentration of a user",
    description = r#"Ends the concentration of the requesting user. Does nothing if the user is not concentrating.

If a concentration was ended, a `concentrationEnded` event is broadcast on `GET /spellPreset/cast/stream`.

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
    con_rep: &State<ConcentrationRepository>,
    usr_rep: &State<UserRepository>,
    spp_rep: &State<SpellPresetRepository>,
    queue: &State<Sender<SpellEvent>>) -> Result<Status> {
    if let Some(ended) = con_rep.remove_concentration(&target.owner_uuid).await? {
        broadcast_concentration_ended(
            queue, usr_rep, spp_rep,
            target.owner_uuid, target.requester_uuid,
            &ended,
        ).await?;
    }
    Ok(Status::NoContent)
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_concentration,
        get_all_concentrations,
        remove_concentration
    ),
    components(
        schemas(
            Concentration,
            ConcentrationReturnList
        )
    ),
    tags(
        (name = "Concentration", description = "Endpoints for managing the concentration of users")
    )
)]
pub struct ConcentrationApiDoc;
