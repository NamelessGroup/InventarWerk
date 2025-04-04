use rocket::form::FromForm;
use rocket::response::Redirect;
use rocket::State;
use std::env;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::http::{Cookie, CookieJar, Status};
use reqwest::Client;

use inv_rep::repos::user_repository::UserRepository;
use inv_rep::model::User;

use rocket_errors::anyhow::Result;


#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DMResponse {
    isDm: bool
}

#[derive(Serialize, Deserialize)]
pub struct AccountResponse {
    accounts: Vec<User>
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LoggedInResponse {
    loggedIn: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    refresh_token: String,
    scope: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct InfoResponse {
    userUUID: String
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
pub async fn get_accounts(_user: super::AuthenticatedUser, usr_rep: &State<UserRepository>)
 -> Result<Json<AccountResponse>> {
    let all_users =  usr_rep.get_all_users().await?;
    Ok(Json(
        AccountResponse {
            accounts: all_users
        }
    ))
}

#[get("/account/isDm?<params..>")]
pub async fn is_account_dm(params: AccountUUIDParams,  _user: super::AuthenticatedUser, usr_rep: &State<UserRepository>)
 -> Result<Json<DMResponse>> {
    let user =  usr_rep.get_user(&params.account_uuid).await?;
    Ok(Json(DMResponse {
        isDm: (user.dm==1)
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

#[get("/account/oauth/callback?<params..>")]
pub async fn callback(params: CodeParams, cookies: &CookieJar<'_>, usr_rep: &State<UserRepository>)
 -> Result<Redirect> {
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
        .await?
        .json::<TokenResponse>()
        .await?;

    // Abrufen der Benutzerinformationen mit dem Access Token
    let user_response = client
        .get("https://discord.com/api/users/@me")
        .header("Authorization", format!("Bearer {}", token_response.access_token))
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
        ("token_type_hint", "access_token")
    ];

    let _response = client 
        .post(revoke_url)
        .form(&params)
        .send()
        .await?;

    let avatar_unpacked = user_response.avatar.unwrap_or("".to_string());
    let has_user = usr_rep.user_exists(&user_response.id.clone()).await?;
    if !has_user {
        let _res = usr_rep.create_user(&user_response.id, &user_response.username,
            &avatar_unpacked);
    } else {
        let user = usr_rep.get_user(&user_response.id.clone()).await?;
        if user.name != user_response.username || user.avatar != avatar_unpacked {
            usr_rep.update_user(&user_response.id, &user_response.username, &avatar_unpacked, user.dm).await?;
        }
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

#[get("/account/info")]
pub async fn account_info(user: super::AuthenticatedUser) -> Json<InfoResponse> {
    return Json(InfoResponse {
        userUUID: user.user_id
    })
}

#[get("/account/isLoggedIn")]
pub async fn user_logged_in(cookies: &CookieJar<'_>) -> Json<LoggedInResponse> {
    return Json(LoggedInResponse {
        loggedIn: cookies.get_private("user_id") != None
    });
}

#[get("/account/logout")]
pub async fn logout(cookies: &CookieJar<'_>) -> Status {
    if let Some(_cookie) = cookies.get_private("user_id") {
        cookies.remove_private("user_id");
        Status::NoContent
    } else {
        Status::BadRequest
    }
}
