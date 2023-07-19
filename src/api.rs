use serde::{Deserialize, Serialize};

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
