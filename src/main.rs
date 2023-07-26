mod api;
mod config;

use crate::{api::send_message, config::Config};
use api::login::login;
use api::verify_token::verify_token;
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
async fn main() {
    let args = Args::parse();
    let mut config = match Config::new("config.toml") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to create new config: {}", e);
            return;
        }
    };
    let client = reqwest::Client::new();

    match get_token(&config, &client).await {
        Ok(token) => {
            config.token = Some(token);
            if let Err(e) = config.save("config.toml") {
                eprintln!("Failed to save config: {}", e);
                return;
            }
        }
        Err(e) => {
            eprintln!("Failed to get token: {}", e);
            return;
        }
    };

    let room = args.room;
    let message = args.message;
    if let Err(e) = send_message(message.as_str(), room.as_str(), &config, &client).await {
        eprintln!("Failed to send message: {}", e);
    }
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
