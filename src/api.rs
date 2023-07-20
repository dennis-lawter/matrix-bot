use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::Config;

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
struct LoginResponseBody {
    user_id: String,
    access_token: String,
    home_server: String,
    device_id: String,
}

#[derive(Deserialize, Debug)]
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
) -> Result<String, ()> {
    let profile_url = format!(
        "{}/_matrix/client/r0/profile/{}",
        config.base_url.as_str(),
        config.full_username.as_str()
    );
    client
        .get(profile_url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|_| ())?;
    Ok(token.to_owned())
}

pub async fn login(config: &Config, client: &reqwest::Client) -> Result<String, ()> {
    let user = &config.local_username;
    let password = &config.password;

    let login_url = format!("{}/_matrix/client/r0/login", config.base_url.as_str());
    let login_send_body_obj = LoginRequestBody::new(user.as_str(), password.as_str());
    let login_send_body_json =
        serde_json::to_string(&login_send_body_obj).expect("Bad json request");

    let login_response = client
        .post(login_url)
        .body(login_send_body_json)
        .send()
        .await
        .expect("Login error");

    if login_response.status().is_success() {
        let login_response_json = login_response.text().await.expect("Login error");
        let login_response_obj = serde_json::from_str::<LoginResponseBody>(&login_response_json)
            .expect("Bad json response");
        println!("{:?}", login_response_obj);
        return Ok(login_response_obj.access_token);
    }

    Err(())
}

pub async fn send_message(
    message: &str,
    room: &str,
    config: &Config,
    client: &Client,
) -> Result<(), ()> {
    if !verify_in_room(room, config, client).await {
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
    let token_string_opt = config.token.clone();
    let token = token_string_opt.expect("Token could not be found during API calls");
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
    println!("{}", message_send_response_json);

    Ok(())
}

async fn join_room(room: &str, config: &Config, client: &Client) -> Result<(), ()> {
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
                .await
                .expect("Join error");
            if join_response.status().is_success() {
                return Ok(());
            }
        }
        None => {}
    }
    Err(())
}

async fn verify_in_room(room: &str, config: &Config, client: &Client) -> bool {
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
                .await
                .expect("Room check error");
            if join_response.status().is_success() {
                let join_response_json: Value = join_response.json().await.expect("Room check error");

				println!("{:?}", join_response_json);

				let user_id = &config.full_username;
				let members = join_response_json["joined"].as_object().unwrap();

				println!("{:?}", members);
			
				return members.iter().any(|(k,v)| k == user_id);
            }
        }
        None => {}
    }
    false
}
