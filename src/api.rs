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
    let profile_url = config.get_profile_url();

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

    // panic!("{:?}", response.text().await);

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

#[cfg(test)]
mod tests {
    use fake::{
        faker::internet::en::{Password, Username},
        Fake,
    };

    use crate::config::Config;

    use super::verify_token;

    #[tokio::test]
    async fn test_test() {
        let mut mock_server = mockito::Server::new();

        let base_url = format!("http://{}", mock_server.host_with_port());

        let config = Config {
            base_url: base_url.clone(),
            local_username: Username().fake(),
            full_username: Username().fake(),
            password: Password(16..24).fake(),
            token: Some(Password(16..24).fake()),
        };

        let full_profile_url = config.get_profile_url();
        let profile_url = full_profile_url
            .strip_prefix(base_url.as_str())
            .expect("Base URL missing from profile url");
        let profile_response_body = format!(
            r#"
{{
    "displayname": "{}",
    "avatar_url": null
}}
"#,
            config.local_username
        );

        let mock_endpoint = mock_server
            .mock("GET", profile_url)
            .with_status(200)
            .with_body(profile_response_body.as_str())
            .create();

        let client = reqwest::Client::new();

        let something =
            verify_token(config.token.clone().unwrap().as_str(), &config, &client).await;

        mock_endpoint.assert();

        assert!(something.is_ok());
        assert_eq!(something.unwrap(), config.token.unwrap().as_str());
    }
}
