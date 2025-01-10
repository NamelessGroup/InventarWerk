use rocket::form::FromForm;
use rocket::response::Redirect;
use rocket::State;
use std::env;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use reqwest::Client;
use rocket::response::status::Custom;

use crate::controller::account_controller::AccountController;
use crate::controller::CStat;
use crate::model::User;


#[derive(Serialize, Deserialize)]
pub struct DMResponse {
    is_dm: bool
}
#[derive(Serialize, Deserialize)]
pub struct AccountResponse {
    accounts: Vec<User>
}

#[derive(Serialize, Deserialize)]
pub struct LoggedInResponse {
    logged_in: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    refresh_token: String,
    scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordUser {
    id: String,
    username: String,
    discriminator: String,
    avatar: Option<String>,
}
#[derive(FromForm)]
pub struct CodeParams {
    code: String
}

#[derive(FromForm)]
pub struct AccountUUIDParams {
    account_uuid: String
}

#[get("/account/get")]
pub async fn get_accounts(_user: super::AuthenticatedUser, acc_con: &State<AccountController>)
 -> Result<Json<AccountResponse>, CStat> {
    let all_users =  acc_con.get_all_users()?;
    Ok(Json(
        AccountResponse {
            accounts: all_users
        }
    ))

    
}

#[get("/account/isDm?<params..>")]
pub async fn is_account_dm(params: AccountUUIDParams,  _user: super::AuthenticatedUser, acc_con: &State<AccountController>)
 -> Result<Json<DMResponse>, CStat> {
    let user_is_dm =  acc_con.user_is_dm(params.account_uuid)?;
    Ok(Json(DMResponse {
        is_dm: user_is_dm
    }))
    
}

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

#[get("/account/info")]
pub async fn account_info(user: super::AuthenticatedUser) -> String {
    format!("{}", user.user_id)
}

#[get("/account/isLoggedIn")]
pub async fn user_logged_in(cookies: &CookieJar<'_>) -> Json<LoggedInResponse> {
    if let Some(_cookie) = cookies.get_private("user_id") {
        return Json(LoggedInResponse {
            logged_in: true
        })
    } else {
        return Json(LoggedInResponse {
            logged_in: false
        })
    }
}

#[get("/account/logut")]
pub async fn logout(cookies: &CookieJar<'_>) -> Status {
    if let Some(_cookie) = cookies.get_private("user_id") {
        cookies.remove_private("user_id");
        Status::NoContent
    } else {
        Status::BadRequest
    }
}

#[get("/account/oauth/callback?<params..>")]
pub async fn callback(params: CodeParams, cookies: &CookieJar<'_>, acc_con: &State<AccountController>)
 -> Result<Redirect, Custom<String>> {
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

    // Austausch des Authorization Codes gegen einen Token
    let token_response = client
        .post(token_url)
        .form(&params)
        .send()
        .await
        .map_err(|err| {Custom(Status::InternalServerError, err.to_string())})?
        .json::<TokenResponse>()
        .await
        .map_err(|_| Custom(Status::InternalServerError, "Conversion to Tokenresponse failed".to_string()))?;

    // Abrufen der Benutzerinformationen mit dem Access Token
    let user_response = client
        .get("https://discord.com/api/users/@me")
        .header("Authorization", format!("Bearer {}", token_response.access_token))
        .send()
        .await
        .map_err(|_| Custom(Status::InternalServerError, "Userrequest failed".to_string()))?
        .json::<DiscordUser>()
        .await
        .map_err(|_| Custom(Status::InternalServerError, "Conversion to DiscordUser failed".to_string()))?;

    // revoke refresh token
    let revoke_url = "https://discord.com/api/oauth2/token/revoke";
    let params = [
        ("client_id", client_id.as_str()),
        ("client_secret", client_secret.as_str()),
        ("token", token_response.refresh_token.as_str()),
        ("token_type_hint", "access_token")
    ];

    let _response = client 
        .post(revoke_url)
        .form(&params)
        .send()
        .await
        .map_err(|_| Custom(Status::InternalServerError, "Revoke refresh token failed".to_string()))?;

    let has_user = acc_con.has_user(user_response.id.clone()).unwrap_or_default();
    if !has_user {
        let _res = acc_con.add_user(user_response.id.clone(), user_response.username.clone());
    }
    // Speichern eines Cookies als Beispiel
    let new_cookie = Cookie::build(("user_id", user_response.id.clone())).http_only(false);
    cookies.add_private(new_cookie);

    #[cfg(feature = "dev")] {
        return Ok(Redirect::to(uri!("http://localhost:5173")));
    }
    #[cfg(not(feature = "dev"))] {
        return Ok(Redirect::to(uri!("/")));
    }
}
