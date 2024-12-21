use rocket::form::FromForm;
use rocket::response::{self, Redirect};
use rocket::State;
use std::env;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::http::{Cookie, CookieJar, Status};
use reqwest::Client;

use crate::controller::account_controller::AccountController;

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
    itempreset_uuid: String
}

#[get("/account/get")]
pub async fn get_accounts() -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
}

#[get("/account/isDm?<params..>")]
pub async fn is_account_dm(params: AccountUUIDParams) -> &'static str {
    // return all inventories
    "Hello, Rocket with async!"
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
pub async fn account_info(cookies: &CookieJar<'_>) -> String {
    let id = cookies.get_private("user_id").map(|cookie| cookie.value().to_string()).unwrap_or("default".to_string());
    format!("{}", id)
}

#[get("/account/oauth/callback?<params..>")]
pub async fn callback(params: CodeParams, cookies: &CookieJar<'_>, acc_con: &State<AccountController>) -> Result<Json<DiscordUser>, Status> {
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
        .map_err(|_| Status::InternalServerError)?
        .json::<TokenResponse>()
        .await
        .map_err(|_| Status::InternalServerError)?;

    // Abrufen der Benutzerinformationen mit dem Access Token
    let user_response = client
        .get("https://discord.com/api/users/@me")
        .header("Authorization", format!("Bearer {}", token_response.access_token))
        .send()
        .await
        .map_err(|_| Status::InternalServerError)?
        .json::<DiscordUser>()
        .await
        .map_err(|_| Status::InternalServerError)?;

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
        .map_err(|_| Status::InternalServerError)?;


    if !acc_con.has_user(user_response.id.clone()) {
        acc_con.add_user(user_response.id.clone(), user_response.username.clone());
    }
    // Speichern eines Cookies als Beispiel
    cookies.add_private(Cookie::new("user_id", user_response.id.clone()));

    Ok(Json(user_response))
}
