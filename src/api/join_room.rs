use reqwest::Client;

use crate::config::Config;

use super::ApiError;

pub async fn join_room(room: &str, config: &Config, client: &Client) -> Result<(), ApiError> {
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
