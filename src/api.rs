use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use crate::config::Config;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("JSON serialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Missing password in configuration")]
    MissingPassword,
    #[error("Missing token in configuration")]
    MissingToken,
    #[error("Login failed with status: {0}")]
    LoginFailed(u16),
    #[error("Join room failed with status: {0}")]
    JoinRoomFailed(u16),
    #[error("HTTP Error")]
    HttpError { source: reqwest::Error, url: String },
    #[error("Matrix API Error")]
    MatrixApiError {
        status_code: reqwest::StatusCode,
        error_message: String,
    },
}

#[derive(Serialize, Debug)]
struct LoginRequestBody {
    r#type: String,
    user: String,
    password: String,
}
impl LoginRequestBody {
    pub fn new(user: &str, password: &str) -> Self {
        Self {
            r#type: "m.login.password".to_owned(),
            user: user.to_owned(),
            password: password.to_owned(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct LoginResponseBody {
    user_id: String,
    access_token: String,
    home_server: String,
    device_id: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct MatrixErrorResponseBody {
    errcode: String,
    error: String,
    retry_after_ms: Option<u64>,
}

#[derive(Serialize, Debug)]
struct MessageSendRequestBody {
    msgtype: String,
    body: String,
}
impl MessageSendRequestBody {
    fn new(body: &str) -> Self {
        Self {
            msgtype: "m.text".to_owned(),
            body: body.to_owned(),
        }
    }
}

pub async fn verify_token(
    token: &str,
    config: &Config,
    client: &reqwest::Client,
) -> Result<String, ApiError> {
    let profile_url = format!(
        "{}/_matrix/client/r0/profile/{}",
        config.base_url.as_str(),
        config.full_username.as_str()
    );

    let profile_url_clone = profile_url.clone();

    let response = client
        .get(profile_url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|err| ApiError::HttpError {
            source: err,
            url: profile_url_clone,
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(ApiError::MatrixApiError {
            status_code: status,
            error_message: text,
        });
    }

    Ok(token.to_owned())
}

pub async fn login(config: &Config, client: &reqwest::Client) -> Result<String, ApiError> {
    let user = &config.local_username;
    let password = config.password.clone().ok_or(ApiError::MissingPassword)?;

    let login_url = format!("{}/_matrix/client/r0/login", config.base_url.as_str());
    let login_send_body_obj = LoginRequestBody::new(user.as_str(), password.as_str());
    let login_send_body_json =
        serde_json::to_string(&login_send_body_obj).expect("Bad json request");

    let login_response = client
        .post(login_url)
        .body(login_send_body_json)
        .send()
        .await?;

    if login_response.status().is_success() {
        let login_response_json = login_response.text().await?;
        let login_response_obj = serde_json::from_str::<LoginResponseBody>(&login_response_json)?;
        return Ok(login_response_obj.access_token);
    }

    Err(ApiError::LoginFailed(login_response.status().as_u16()))
}

pub async fn send_message(
    message: &str,
    room: &str,
    config: &Config,
    client: &Client,
) -> Result<(), ApiError> {
    if !verify_in_room(room, config, client).await? {
        join_room(room, config, client).await?
    }

    let message_send_body_obj = MessageSendRequestBody::new(message);
    let message_send_body_json =
        serde_json::to_string(&message_send_body_obj).expect("Serialize failed on message body");

    let message_send_url = format!(
        "{}/_matrix/client/r0/rooms/{}/send/m.room.message",
        &config.base_url.as_str(),
        room
    );

    let token = config.token.clone().ok_or(ApiError::MissingToken)?;
    let message_send_response_json = client
        .post(message_send_url)
        .body(message_send_body_json)
        .bearer_auth(token.as_str())
        .send()
        .await
        .expect("Send error")
        .text()
        .await
        .expect("Send error");

    Ok(())
}

async fn join_room(room: &str, config: &Config, client: &Client) -> Result<(), ApiError> {
    match &config.token {
        Some(token) => {
            let join_url = format!(
                "{}/_matrix/client/r0/rooms/{}/join",
                config.base_url.as_str(),
                room,
            );
            let join_response = client
                .post(join_url)
                .bearer_auth(token.as_str())
                .send()
                .await?;

            if join_response.status().is_success() {
                return Ok(());
            }
        }
        None => {}
    }
    Err(ApiError::JoinRoomFailed(403))
}

async fn verify_in_room(room: &str, config: &Config, client: &Client) -> Result<bool, ApiError> {
    match &config.token {
        Some(token) => {
            let join_url = format!(
                "{}/_matrix/client/r0/rooms/{}/joined_members",
                config.base_url.as_str(),
                room,
            );
            let join_response = client
                .get(join_url)
                .bearer_auth(token.as_str())
                .send()
                .await?;

            if join_response.status().is_success() {
                let join_response_json: Value = join_response.json().await?;

                let user_id = &config.full_username;
                let members = join_response_json["joined"].as_object().unwrap();

                return Ok(members.iter().any(|(k, _v)| k == user_id));
            }
        }
        None => {}
    }
    Ok(false)
}
