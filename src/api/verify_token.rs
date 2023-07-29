use crate::config::Config;

use super::ApiError;

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

#[cfg(test)]
mod tests {
    use fake::{
        faker::internet::en::{Password, Username},
        Fake,
    };

    use crate::config::Config;

    use super::verify_token;

    #[tokio::test]
    async fn test_verify_token() {
        let mut mock_server = mockito::Server::new();

        let base_url = format!("http://{}", mock_server.host_with_port());

        let config = Config {
            base_url: base_url.clone(),
            local_username: Username().fake(),
            full_username: Username().fake(),
            password: None,
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

        let func_result =
            verify_token(config.token.clone().unwrap().as_str(), &config, &client).await;

        mock_endpoint.assert();

        assert!(func_result.is_ok(), "{:?}", func_result);
        assert_eq!(func_result.unwrap(), config.token.unwrap().as_str());
    }
}
