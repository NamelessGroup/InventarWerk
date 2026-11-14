
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::{self, Responder};
use rocket::{outcome::try_outcome, Request, State};
use repos::repos::user_repository::UserRepository;

/// Message of `403 Forbidden` responses caused by missing access rights.
pub const ACCESS_DENIAL_MESSAGE: &str = "no access";

/// Result type of all API endpoints.
pub type Result<T, E = ApiError> = std::result::Result<T, E>;

/// Error type of all API endpoints.
///
/// Every error can be converted into it with `?`:
/// - `sqlx::Error::RowNotFound` (also wrapped in an `anyhow::Error`) becomes `404 Not Found`,
/// - every other error becomes `500 Internal Server Error`.
///
/// Errors with other status codes are created with the constructors, e.g. [`ApiError::forbidden`].
/// For `4xx` errors the message is sent as plain text body, `500` errors are only logged.
#[derive(Debug)]
pub struct ApiError {
    status: Status,
    error: anyhow::Error,
}

impl ApiError {
    /// Creates an error with the given status and message.
    pub fn new(status: Status, message: impl Into<String>) -> Self {
        Self { status, error: anyhow::anyhow!(message.into()) }
    }

    /// `403 Forbidden` because of missing access rights.
    pub fn forbidden() -> Self {
        Self::new(Status::Forbidden, ACCESS_DENIAL_MESSAGE)
    }

    /// `400 Bad Request` with the given message.
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(Status::BadRequest, message)
    }

    /// `404 Not Found` with the given message.
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(Status::NotFound, message)
    }

    /// `409 Conflict` with the given message.
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(Status::Conflict, message)
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        let error = error.into();
        match error.downcast_ref::<sqlx::Error>() {
            Some(sqlx::Error::RowNotFound) => Self::not_found("not found"),
            _ => Self { status: Status::InternalServerError, error },
        }
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        if self.status == Status::InternalServerError {
            // Logs the error and responds without leaking internals.
            return response::Debug(self.error).respond_to(request);
        }
        (self.status, self.error.to_string()).respond_to(request)
    }
}

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
pub async fn user_is_dm(usr_rep: &UserRepository, user_id: String) -> anyhow::Result<bool> {
    Ok(usr_rep.get_user(&user_id).await?.dm)
}


/// Creates a `500 Internal Server Error` with the given message.
pub fn create_error(msg: &str) -> ApiError {
    ApiError::new(Status::InternalServerError, msg)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_not_found_becomes_404() {
        assert_eq!(ApiError::from(sqlx::Error::RowNotFound).status, Status::NotFound);
    }

    #[test]
    fn row_not_found_wrapped_in_anyhow_becomes_404() {
        let error: anyhow::Error = sqlx::Error::RowNotFound.into();
        let error = error.context("while loading something");
        assert_eq!(ApiError::from(error).status, Status::NotFound);
    }

    #[test]
    fn other_errors_become_500() {
        assert_eq!(ApiError::from(sqlx::Error::PoolTimedOut).status, Status::InternalServerError);
        assert_eq!(ApiError::from(anyhow::anyhow!("boom")).status, Status::InternalServerError);
        assert_eq!(create_error("boom").status, Status::InternalServerError);
    }

    #[test]
    fn explicit_statuses() {
        assert_eq!(ApiError::forbidden().status, Status::Forbidden);
        assert_eq!(ApiError::bad_request("x").status, Status::BadRequest);
        assert_eq!(ApiError::conflict("x").status, Status::Conflict);
    }
}
