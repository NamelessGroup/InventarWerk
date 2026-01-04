use repos::{model_zauberwerk::Concentration, repos::{user_repository::UserRepository, zauberwek::concentration_repository::ConcentrationRepository}};
use rocket::{State, http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use utils::{AuthenticatedUser, user_is_dm, create_error};
use rocket_errors::anyhow::Result;

//optional id

#[derive(FromForm)]
pub struct ConcentrationUUID{
    uuid: Option<String>
}

#[get("/concentration?<params..>")]
pub async fn get_concentration(
    user: AuthenticatedUser,
    usr_rep: &State<UserRepository>,
    con_rep: &State<ConcentrationRepository>,
    params: ConcentrationUUID) -> Result<Json<Concentration>> {
    if params.uuid.is_some() && !user_is_dm(usr_rep, user.user_id.clone()).await? {
        return Err(create_error("Forbidden"));
    }

    let concentration = match params.uuid {
        Some(uuid) => con_rep.get_concentration(&uuid).await?,
        None => con_rep.get_concentration(&user.user_id).await?
    };
    
    Ok(Json(concentration))
}

#[derive(Serialize, Deserialize)]
pub struct ConcentrationReturnList {
    concentrations: Vec<Concentration>
}

#[get("/concentration/all")]
pub async fn get_all_concentrations(
    user: AuthenticatedUser,
    usr_rep: &State<UserRepository>,
    con_rep: &State<ConcentrationRepository>) -> Result<Json<ConcentrationReturnList>> {
    if !user_is_dm(usr_rep, user.user_id).await? {
        return Err(create_error("Forbidden"));
    }
    let cons = con_rep.get_all_concentrations().await?;

    Ok(Json(ConcentrationReturnList { concentrations: cons }))

}
#[derive(Serialize, Deserialize, FromForm)]
pub struct SetConcentrationParams {
    uuid: Option<String>,
    con: i32
}
#[put("/concentration?<params..>")]
pub async fn modify_concentration(
    user: AuthenticatedUser,
    params: SetConcentrationParams,
    con_rep: &State<ConcentrationRepository>,
    usr_rep: &State<UserRepository>) -> Result<Status> {
    if params.uuid.is_some() && params.uuid.as_ref() != Some(&user.user_id) && !user_is_dm(usr_rep, user.user_id.clone()).await? {
        return Err(create_error("Forbidden"));
    }
    match params.uuid {
        Some(uuid) => con_rep.set_concentration(&uuid, params.con).await?,
        None => con_rep.set_concentration(&user.user_id, params.con).await?
    };
    Ok(Status::NoContent)
}
