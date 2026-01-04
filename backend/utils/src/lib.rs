
use rocket::http::Status;
use rocket::request::Outcome;
use rocket::Request;
use rocket::{request::FromRequest, Route};
use repos::repos::user_repository::UserRepository;
use anyhow::Result;

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