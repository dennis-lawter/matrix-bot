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

    let message_send_url = config.get_send_message_url(room);

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

#[cfg(test)]
mod tests {
    use fake::{
        faker::internet::en::{Password, Username},
        faker::lorem::en::{Sentence, Word},
        Fake,
    };

    use crate::config::Config;

    use super::send_message;

    #[tokio::test]
    async fn test_send_message_room() {
        let mut mock_server = mockito::Server::new();

        let base_url = format!("http://{}", mock_server.host_with_port());

        let config = Config {
            base_url: base_url.clone(),
            local_username: Username().fake(),
            full_username: Username().fake(),
            password: None,
            token: Some(Password(16..24).fake()),
        };

        let room: String = Word().fake();
        let message: String = Sentence(1..2).fake();

        let full_send_message_url = config.get_send_message_url(room.as_str());
        let send_message_url = full_send_message_url
            .strip_prefix(base_url.as_str())
            .expect("Base URL missing from profile url");
        let send_message_response_body = format!(
            r#"
{{
}}
"#,
        );

        let mock_endpoint = mock_server
            .mock("POST", send_message_url)
            .with_status(200)
            .with_body(send_message_response_body.as_str())
            .create();

        let client = reqwest::Client::new();

        let func_result = send_message(message.as_str(), room.as_str(), &config, &client).await;

        mock_endpoint.assert();

        assert!(func_result.is_ok(), "{:?}", func_result);
    }
}
