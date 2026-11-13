
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
    Ok(usr_rep.get_user(&user_id).await?.dm)
}


/// Helper to create an error for API responses.
pub fn create_error(msg: &str) -> AnyhowError {
    anyhow!(msg.to_string()).into()
}

/// Name of the OpenAPI security scheme used by all authenticated endpoints.
pub const SECURITY_SCHEME: &str = "cookie_auth";

/// OpenAPI modifier shared by all API docs.
///
/// - Registers the `cookie_auth` security scheme (private `user_id` cookie).
/// - Adds a `401 Unauthorized` response to every operation that requires authentication,
///   because every auth guard (`AuthenticatedUser`, `DmUser`, `TargetUser`) responds with it.
///
/// Use with `#[openapi(modifiers(&ApiDocAddon), ...)]` after `use utils::ApiDocAddon;`.
pub struct ApiDocAddon;

impl utoipa::Modify for ApiDocAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
        use utoipa::openapi::ResponseBuilder;

        openapi
            .components
            .get_or_insert_with(Default::default)
            .add_security_scheme(
                SECURITY_SCHEME,
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::with_description(
                    "user_id",
                    "Private (encrypted) session cookie, set by `GET /account/oauth/callback` after logging in via `GET /account/login`. \
                     The browser sends it automatically; it can not be entered manually.",
                ))),
            );

        for path_item in openapi.paths.paths.values_mut() {
            let operations = [
                &mut path_item.get,
                &mut path_item.put,
                &mut path_item.post,
                &mut path_item.delete,
                &mut path_item.patch,
            ];
            for operation in operations.into_iter().flatten() {
                let requires_auth = operation
                    .security
                    .as_ref()
                    .is_some_and(|security| !security.is_empty());
                if requires_auth {
                    operation
                        .responses
                        .responses
                        .entry("401".to_string())
                        .or_insert_with(|| {
                            ResponseBuilder::new()
                                .description("Unauthorized - user is not logged in")
                                .build()
                                .into()
                        });
                }
            }
        }
    }
}
