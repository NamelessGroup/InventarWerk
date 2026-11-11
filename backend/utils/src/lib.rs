
use rocket::http::Status;
use rocket::request::Outcome;
use rocket::{outcome::try_outcome, Request, State};
use rocket::{request::FromRequest, Route};
use repos::repos::user_repository::UserRepository;
use anyhow::Result;
use anyhow::anyhow;
use rocket_errors::anyhow::AnyhowError;

/// Extractor for authenticated users based on a private `user_id` cookie.
pub struct AuthenticatedUser {
    /// The user id of the authenticated user
    pub user_id: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();

        if let Some(cookie) = cookies.get_private("user_id") {
            let user_id = cookie.value().to_string();
            Outcome::Success(AuthenticatedUser { user_id })
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}

/// Extractor for authenticated users that are Dungeon Masters (DM).
///
/// Responds with `401 Unauthorized` if the user is not logged in and with
/// `403 Forbidden` if the user is not a DM. Requires a managed `UserRepository`.
pub struct DmUser {
    /// The user id of the authenticated DM
    pub user_id: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DmUser {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = try_outcome!(request.guard::<AuthenticatedUser>().await);
        let usr_rep = try_outcome!(request.guard::<&State<UserRepository>>().await);

        match user_is_dm(usr_rep, user.user_id.clone()).await {
            Ok(true) => Outcome::Success(DmUser { user_id: user.user_id }),
            Ok(false) => Outcome::Error((Status::Forbidden, ())),
            Err(_) => Outcome::Error((Status::InternalServerError, ())),
        }
    }
}

/// Extractor for the user a request acts on.
///
/// Reads the optional `owner_uuid` query parameter:
/// - omitted or equal to the requesting user: acts on the requesting user
/// - another user: requires DM privileges, otherwise responds with `403 Forbidden`
///
/// Responds with `401 Unauthorized` if the user is not logged in. Requires a managed `UserRepository`.
pub struct TargetUser {
    /// The user id of the authenticated user who sent the request
    pub requester_uuid: String,
    /// The user id of the user the request acts on
    pub owner_uuid: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TargetUser {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = try_outcome!(request.guard::<AuthenticatedUser>().await);
        let requested = request
            .query_value::<String>("owner_uuid")
            .and_then(|value| value.ok());

        let owner_uuid = match requested {
            Some(owner) if owner != user.user_id => {
                let usr_rep = try_outcome!(request.guard::<&State<UserRepository>>().await);
                match user_is_dm(usr_rep, user.user_id.clone()).await {
                    Ok(true) => owner,
                    Ok(false) => return Outcome::Error((Status::Forbidden, ())),
                    Err(_) => return Outcome::Error((Status::InternalServerError, ())),
                }
            }
            _ => user.user_id.clone(),
        };

        Outcome::Success(TargetUser {
            requester_uuid: user.user_id,
            owner_uuid,
        })
    }
}

/// Checks if a user is a Dungeon Master (DM).
///
/// # Arguments
/// * `usr_rep` - Reference to the user repository.
/// * `user_id` - The UUID of the user.
///
/// # Returns
/// `true` if the user is a DM, otherwise `false`.
pub async fn user_is_dm(usr_rep: &UserRepository, user_id: String) -> Result<bool> {
    Ok(usr_rep.get_user(&user_id).await?.dm == 1)
}


/// Helper to create an error for API responses.
pub fn create_error(msg: &str) -> AnyhowError {
    anyhow!(msg.to_string()).into()
}