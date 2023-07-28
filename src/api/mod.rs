pub mod join_room;
pub use join_room::join_room;
pub mod login;
pub use login::login;
pub mod send_message;
pub use send_message::send_message;
pub mod verify_in_room;
pub use verify_in_room::verify_in_room;
pub mod verify_token;
pub use verify_token::verify_token;

use serde::Deserialize;
use thiserror::Error;

use crate::config::ConfigError;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("JSON serialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Config error: {0}")]
    Config(#[from] ConfigError),
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

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct MatrixErrorResponseBody {
    errcode: String,
    error: String,
    retry_after_ms: Option<u64>,
}
