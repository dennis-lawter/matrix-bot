#![forbid(unsafe_code)]

mod api;
mod config;

use crate::config::Config;
use api::join_room;
use api::login;
use api::send_message;
use api::verify_in_room;
use api::verify_token;
use api::ApiError;
use clap::Parser;

/// A simple to use bot for sending text messages to a Matrix room
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// ID of the room receiving the message
    #[arg(short, long)]
    room: String,

    /// Message to send
    #[arg(short, long)]
    message: String,
}

#[async_std::main]
async fn main() -> Result<(), ApiError> {
    let args = Args::parse();
    let mut config = Config::load("config.toml")?;
    let client = reqwest::Client::new();

    let valid_token = get_token(&config, &client).await?;
    config.token = Some(valid_token);
    config.save("config.toml")?;

    let room = args.room;

    if !verify_in_room(room.as_str(), &config, &client).await? {
        join_room(room.as_str(), &config, &client).await?
    }

    let message = args.message;
    if let Err(e) = send_message(message.as_str(), room.as_str(), &config, &client).await {
        eprintln!("Failed to send message: {}", e);
    }

    Ok(())
}

async fn get_token(config: &Config, client: &reqwest::Client) -> Result<String, ApiError> {
    match config.token.clone() {
        Some(found_token) => match verify_token(found_token.as_str(), config, client).await {
            Ok(valid_token) => Ok(valid_token),
            Err(e) => {
                eprintln!("Failed to verify token: {}", e);
                login(config, client).await
            }
        },
        None => login(config, client).await,
    }
}
