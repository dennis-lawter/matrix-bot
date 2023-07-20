mod config;
mod api;

use crate::{config::Config, api::send_message};

use api::{verify_token, login};
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
async fn main() -> Result<(), ()> {
    let args = Args::parse();
    let mut config = Config::new().map_err(|_| ())?;
    let client = reqwest::Client::new();

    let token = get_token(&config, &client).await?;
    config.token = Some(token);
    config.save().map_err(|_| ())?;

    let room = args.room;
    let message = args.message;
    send_message(message.as_str(), room.as_str(), &config, &client).await.map_err(|_| ())?;

    Ok(())
}

async fn get_token(config: &Config, client: &reqwest::Client) -> Result<String, ()> {
    match config.token.clone() {
        Some(found_token) => match verify_token(found_token.as_str(), config, client).await {
            Ok(valid_token) => Ok(valid_token),
            Err(_) => login(config, client).await,
        },
        None => login(config, client).await,
    }
}

