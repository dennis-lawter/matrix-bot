use reqwest::Client;
use serde::Serialize;

use crate::config::Config;

use super::{ApiError, MatrixErrorResponseBody};

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
