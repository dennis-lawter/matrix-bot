use reqwest::Client;
use serde_json::Value;

use crate::config::Config;

use super::ApiError;

pub async fn verify_in_room(
    room: &str,
    config: &Config,
    client: &Client,
) -> Result<bool, ApiError> {
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
