#![forbid(unsafe_code)]

mod api;
mod config;

use std::fs;

use crate::config::Config;
use api::join_room;
use api::login;
use api::send_message;
use api::verify_in_room;
use api::verify_token;
use api::ApiError;
use clap::arg;
use clap::crate_name;
use clap::crate_version;
use clap::Arg;
use clap::ArgAction;
use clap::Command;

const CONFIG_FILE: &str = "matrix-notify.toml";

fn perform_generate() -> Result<(), ApiError> {
    if fs::metadata(CONFIG_FILE).is_ok() {
        eprintln!("{} already exists, if you intend to generate the example config, please remove this file first", CONFIG_FILE);
        return Err(ApiError::ConfigAlreadyExists);
    }
    let config = Config {
        base_url: "https://example.org".to_owned(),
        local_username: "matrix-bot".to_owned(),
        full_username: "@matrix-bot:example.org".to_owned(),
        password: Some("Plaintext password, can be omitted if you have a token already".to_owned()),
        token: Some(
            "access_token from previous api calls, remove to populate via password driven login"
                .to_owned(),
        ),
    };
    config.save(CONFIG_FILE)?;

    Ok(())
}

async fn perform_send_message(room: &str, message: &str) -> Result<(), ApiError> {
    let mut config = Config::load(CONFIG_FILE)?;
    let client = reqwest::Client::new();

    let valid_token = get_token(&config, &client).await?;
    config.token = Some(valid_token);
    config.save(CONFIG_FILE)?;

    if !verify_in_room(room, &config, &client).await? {
        join_room(room, &config, &client).await?
    }

    if let Err(e) = send_message(message, room, &config, &client).await {
        eprintln!("Failed to send message: {}", e);
    }

    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), ApiError> {
    let m = Command::new(crate_name!())
        .version(crate_version!())
        .about("A command line tool for sending messages to a matrix chatroom")
        .arg(arg!(-r --room <ROOM_ID> "Room ID, typically in the format !roomid:matrix.org"))
        .arg(arg!(-m --message <MESSAGE> "Text to be sent"))
        .subcommand(Command::new("generate").about("Generates an example config file"))
        .get_matches();
    if m.subcommand_matches("generate").is_some() {
        perform_generate()
    } else {
        let room = m
            .get_one::<String>("room")
            .expect("ROOM_ID must be provided, please see --help");
        let message = m
            .get_one::<String>("message")
            .expect("MESSAGE must be provided, please see --help");
        perform_send_message(room.as_str(), message.as_str()).await
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
