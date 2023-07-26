pub mod login;
pub mod verify_token;

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
        serde_json::to_string(&message_send_body_obj).map_err(ApiError::SerdeJson)?;

    let message_send_url = format!(
        "{}/_matrix/client/r0/rooms/{}/send/m.room.message",
        &config.base_url.as_str(),
        room
    );

    let token = config.token.clone().ok_or(ApiError::MissingToken)?;

    let response = client
        .post(message_send_url.clone())
        .body(message_send_body_json)
        .bearer_auth(token.as_str())
        .send()
        .await
        .map_err(|e| ApiError::HttpError {
            source: e,
            url: message_send_url.clone(),
        })?;

    let response_status = response.status();
    let message_send_response = response.text().await.map_err(|err| ApiError::HttpError {
        source: err,
        url: message_send_url.clone(),
    })?;

    if !response_status.is_success() {
        let error_message: MatrixErrorResponseBody =
            serde_json::from_str(&message_send_response).map_err(ApiError::SerdeJson)?;

        return Err(ApiError::MatrixApiError {
            status_code: response_status,
            error_message: error_message.error,
        });
    }

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
