use reqwest::Client;

use crate::config::Config;

use super::ApiError;

pub async fn join_room(room: &str, config: &Config, client: &Client) -> Result<(), ApiError> {
    match &config.token {
        Some(token) => {
            let join_url = config.get_join_url(room);
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

#[cfg(test)]
mod tests {
    use fake::{
        faker::internet::en::{Password, Username},
        faker::lorem::en::Word,
        Fake,
    };

    use crate::config::Config;

    use super::join_room;

    #[tokio::test]
    async fn test_join_room() {
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

        let full_join_url = config.get_join_url(room.as_str());
        let join_url = full_join_url
            .strip_prefix(base_url.as_str())
            .expect("Base URL missing from profile url");
        let join_response_body = format!(
            r#"
{{
}}
"#,
        );

        let mock_endpoint = mock_server
            .mock("POST", join_url)
            .with_status(200)
            .with_body(join_response_body.as_str())
            .create();

        let client = reqwest::Client::new();

        let func_result = join_room(room.as_str(), &config, &client).await;

        mock_endpoint.assert();

        assert!(func_result.is_ok(), "{:?}", func_result);
    }
}
