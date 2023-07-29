use serde::{Deserialize, Serialize};

use crate::config::Config;

use super::ApiError;

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

pub async fn login(config: &Config, client: &reqwest::Client) -> Result<String, ApiError> {
    let user = &config.local_username;
    let password = config.password.clone().ok_or(ApiError::MissingPassword)?;

    let login_url = config.get_login_url();
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

#[cfg(test)]
mod tests {
    use fake::{
        faker::internet::en::{Password, Username},
        Fake,
    };

    use crate::config::Config;

    use super::login;

    #[tokio::test]
    async fn test_login() {
        let mut mock_server = mockito::Server::new();

        let base_url = format!("http://{}", mock_server.host_with_port());

        let config = Config {
            base_url: base_url.clone(),
            local_username: Username().fake(),
            full_username: Username().fake(),
            password: Some(Password(16..24).fake()),
            token: None,
        };

        let token: String = Password(42..43).fake();

        let full_login_url = config.get_login_url();
        let login_url = full_login_url
            .strip_prefix(base_url.as_str())
            .expect("Base URL missing from login url");
        let login_response_body = format!(
            r#"
{{
    "access_token": "{}",
    "user_id": "{}",
    "home_server": "localhost",
    "device_id": "testing"
}}
"#,
            token.as_str(),
            config.full_username,
        );

        let mock_endpoint = mock_server
            .mock("POST", login_url)
            .with_status(200)
            .with_body(login_response_body.as_str())
            .create();

        let client = reqwest::Client::new();

        let func_result = login(&config, &client).await;

        mock_endpoint.assert();

        assert!(func_result.is_ok(), "{:?}", func_result);
        assert_eq!(func_result.unwrap(), token);
    }
}
