use reqwest::Client;
use rocket::form::FromForm;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use std::env;

use utoipa::IntoParams;
use utoipa::OpenApi;
use utoipa::ToSchema;

use repos::model::User;
use repos::repos::user_repository::UserRepository;

use rocket_errors::anyhow::Result;

use crate::routers::router_utility::user_is_dm;
use crate::{lock_toggle, locked_status};

use super::create_error;




#[derive(FromForm, ToSchema, IntoParams)]
pub struct AccountUUIDParams {
    account_uuid: String,
}

#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
pub struct AccountResponse {
    accounts: Vec<User>,
}

#[utoipa::path(
    get,
    path = "/account/get",
    summary = "Retrieve all users",
    description = r#"Returns a JSON response containing all users. Requires authentication."#,
    responses(
        (status = 200, description = "A list of user accounts", body = AccountResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "Accounts"
)]
#[get("/account/get")]
pub async fn get_accounts(
    _user: super::AuthenticatedUser,
    usr_rep: &State<UserRepository>,
) -> Result<Json<AccountResponse>> {
    let all_users = usr_rep.get_all_users().await?;
    Ok(Json(AccountResponse {
        accounts: all_users,
    }))
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
#[serde(crate = "rocket::serde")]
pub struct DMResponse {
    isDm: bool,
}

#[utoipa::path(
    get,
    path = "/account/isDm",
    params(AccountUUIDParams),
    summary = "Check if a user is a DM",
    description = r#"Checks if a user (by UUID) is a Dungeon Master (DM). Requires authentication."#,
    responses(
        (status = 200, description = "A JSON response indicating whether the user is DM", body = DMResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "Accounts"
)]
#[get("/account/isDm?<params..>")]
pub async fn is_account_dm(
    params: AccountUUIDParams,
    _user: super::AuthenticatedUser,
    usr_rep: &State<UserRepository>,
) -> Result<Json<DMResponse>> {
    let user = usr_rep.get_user(&params.account_uuid).await?;
    Ok(Json(DMResponse {
        isDm: (user.dm == 1),
    }))
}

#[utoipa::path(
    get,
    path = "/account/login",
    summary = "Redirect to the Discord login page",
    description = r#"Redirects the user to the Discord OAuth login page."#,
    responses(
        (status = 307, description = "Redirecting to Discord login")
    ),
    tag = "Accounts"
)]
#[get("/account/login")]
pub async fn login() -> Redirect {
    let client_id = env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID not set");
    let redirect_uri = env::var("DISCORD_REDIRECT_URI").expect("DISCORD_REDIRECT_URI not set");
    let url = format!(
        "https://discord.com/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=identify",
        client_id, redirect_uri
    );
    Redirect::to(url)
}

#[derive(FromForm, ToSchema, IntoParams)]
pub struct CodeParams {
    code: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    refresh_token: String,
    scope: String,
}


#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct DiscordUser {
    id: String,
    username: String,
    discriminator: String,
    avatar: Option<String>,
}

#[utoipa::path(
    get,
    path = "/account/oauth/callback",
    params(CodeParams),
    summary = "Handle OAuth callback from Discord",
    description = r#"Enters the OAuth flow, exchanging the code for a token. Retrieves user info from Discord and creates/updates a user in the DB."#,
    responses(
        (status = 307, description = "Redirects to the base URL after processing the OAuth callback")
    ),
    security(("bearer_auth" = [])),
    tag = "Accounts"
)]
#[get("/account/oauth/callback?<params..>")]
pub async fn callback(
    params: CodeParams,
    cookies: &CookieJar<'_>,
    usr_rep: &State<UserRepository>,
) -> Result<Redirect> {
    let client_id = env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID not set");
    let client_secret = env::var("DISCORD_CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET not set");
    let redirect_uri = env::var("DISCORD_REDIRECT_URI").expect("DISCORD_REDIRECT_URI not set");

    let token_url = "https://discord.com/api/oauth2/token";
    let client = Client::new();
    let params = [
        ("client_id", client_id.as_str()),
        ("client_secret", client_secret.as_str()),
        ("grant_type", "authorization_code"),
        ("code", params.code.as_str()),
        ("redirect_uri", redirect_uri.as_str()),
    ];

    // Exchange Authorization Codes for Token
    let token_response = client
        .post(token_url)
        .form(&params)
        .send()
        .await?
        .json::<TokenResponse>()
        .await?;

    // Get Userinformation with Access Token
    let user_response = client
        .get("https://discord.com/api/users/@me")
        .header(
            "Authorization",
            format!("Bearer {}", token_response.access_token),
        )
        .send()
        .await?
        .json::<DiscordUser>()
        .await?;

    // revoke refresh token
    let revoke_url = "https://discord.com/api/oauth2/token/revoke";
    let params = [
        ("client_id", client_id.as_str()),
        ("client_secret", client_secret.as_str()),
        ("token", token_response.refresh_token.as_str()),
        ("token_type_hint", "access_token"),
    ];

    let _response = client.post(revoke_url).form(&params).send().await?;

    let avatar_unpacked = user_response.avatar.unwrap_or("".to_string());
    let has_user = usr_rep.user_exists(&user_response.id.clone()).await?;
    if !has_user {
        if !locked_status!() {
            return Err(create_error("No new Users allowed"));
        }
        let _res = usr_rep
            .create_user(&user_response.id, &user_response.username, &avatar_unpacked)
            .await?;
    } else {
        let user = usr_rep.get_user(&user_response.id.clone()).await?;
        if user.name != user_response.username || user.avatar != avatar_unpacked {
            usr_rep
                .update_user(
                    &user_response.id,
                    &user_response.username,
                    &avatar_unpacked,
                    user.dm,
                )
                .await?;
        }
    }
    // Save Cookie
    let new_cookie = Cookie::build(("user_id", user_response.id.clone())).http_only(false);
    cookies.add_private(new_cookie);

    #[cfg(feature = "dev")]
    {
        return Ok(Redirect::to(uri!("http://localhost:5173")));
    }
    #[cfg(not(feature = "dev"))]
    {
        return Ok(Redirect::to(uri!("/")));
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct InfoResponse {
    userUUID: String,
}

#[utoipa::path(
    get,
    path = "/account/info",
    summary = "Get authenticated user info",
    description = r#"Returns the UUID of the authenticated user."#,
    responses(
        (status = 200, description = "UUID of the authenticated user", body = InfoResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "Accounts"
)]
#[get("/account/info")]
pub async fn account_info(user: super::AuthenticatedUser) -> Json<InfoResponse> {
    return Json(InfoResponse {
        userUUID: user.user_id,
    });
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
pub struct LoggedInResponse {
    loggedIn: bool,
}

#[utoipa::path(
    get,
    path = "/account/isLoggedIn",
    summary = "Check if a user is logged in",
    description = r#"Returns `true` if the user is currently logged in, or `false` otherwise."#,
    responses(
        (status = 200, description = "Indicates whether the user is logged in", body = LoggedInResponse)
    ),
    tag = "Accounts"
)]
#[get("/account/isLoggedIn")]
pub async fn user_logged_in(cookies: &CookieJar<'_>) -> Json<LoggedInResponse> {
    return Json(LoggedInResponse {
        loggedIn: cookies.get_private("user_id") != None,
    });
}

#[utoipa::path(
    get,
    path = "/account/logout",
    summary = "Log out the current user",
    description = r#"Removes the login cookie for the user. If no cookie is set, returns `BadRequest`."#,
    responses(
        (status = 204, description = "User successfully logged out"),
        (status = 400, description = "No user was logged in")
    ),
    tag = "Accounts"
)]
#[get("/account/logout")]
pub async fn logout(cookies: &CookieJar<'_>) -> Status {
    if let Some(_cookie) = cookies.get_private("user_id") {
        cookies.remove_private("user_id");
        Status::NoContent
    } else {
        Status::BadRequest
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
pub struct IsLockedResponse {
    isLocked: bool,
}

#[utoipa::path(
    get,
    path = "/account/isLocked",
    summary = "Check if the system is locked",
    description = r#"Returns `true` if the system is locked, otherwise `false`."#,
    responses(
        (status = 200, description = "Lock status of the system", body = IsLockedResponse)
    ),
    tag = "Accounts"
)]
#[get("/account/isLocked")]
pub async fn is_locked() -> Json<IsLockedResponse> {
    Json(IsLockedResponse {
        isLocked: locked_status!(),
    })
}

#[utoipa::path(
    patch,
    path = "/account/toggleLock",
    summary = "Toggle the lock status of the system",
    description = r#"Only a DM can toggle the system lock. Returns an error if the user is not a DM."#,
    responses(
        (status = 204, description = "System lock toggled successfully"),
        (status = 418, description = "User is not a Dungeon Master")
    ),
    security(("bearer_auth" = [])),
    tag = "Accounts"
)]
#[patch("/account/toggleLock")]
pub async fn toggle_lock(
    user: super::AuthenticatedUser,
    usr_rep: &State<UserRepository>,
) -> Result<Status> {
    if !user_is_dm(usr_rep.inner(), user.user_id).await? {
        return Ok(Status::ImATeapot);
    }
    lock_toggle!();
    Ok(Status::NoContent)
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_accounts,
        is_account_dm,
        login,
        callback,
        account_info,
        user_logged_in,
        logout,
        is_locked,
        toggle_lock
    ),
    components(
        schemas(
            DMResponse,
            AccountResponse,
            LoggedInResponse,
            TokenResponse,
            InfoResponse,
            DiscordUser,
            CodeParams,
            AccountUUIDParams,
            IsLockedResponse
        )
    ),
    tags(
        (name = "Accounts", description = "Endpoints for managing user accounts")
    )
)]
pub struct AccountApiDoc;
