mod config;

use crate::config::Config;

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
    let config = Config::new();
    let client = reqwest::Client::new();

    let token = get_token();

    let room_id = "!HyEiSOhyISCMJNiWOE:matrix.bitonicsoft.com";

    // let room_name_url = format!(
    //     "{}/_matrix/client/r0/rooms/%21636q39766251%3Aexample.com/state/m.room.name?access_token={}"
    // )

    // let join_url = format!(
    //     "{}/_matrix/client/r0/rooms/{}/join?access_token={}",
    //     base_url, room_id, token
    // );
    // let join_response_json = client
    //     .post(join_url)
    //     .send()
    //     .await
    //     .expect("Join error")
    //     .text()
    //     .await
    //     .expect("Join error");

    let message_send_body_obj = MessageSendRequestBody::new("Hello World!");
    let message_send_body_json =
        serde_json::to_string(&message_send_body_obj).expect("Serialize failed on message body");

    let message_send_url = format!(
        "{}/_matrix/client/r0/rooms/{}/send/m.room.message",
        base_url, room_id
    );
    let message_send_response_json = client
        .post(message_send_url)
        .body(message_send_body_json)
        .bearer_auth(token)
        .send()
        .await
        .expect("Send error")
        .text()
        .await
        .expect("Send error");
    println!("{}", message_send_response_json);

    Ok(())
}

async fn get_token(config: &Config, client: &reqwest::Client) -> Result<String, ()> {
    match std::env::var("MATRIX_TOKEN") {
        Ok(found_token) => match verify_token(found_token, base_url, &client) {
            Ok(valid_token) => Ok(valid_token),
            Err(_) => login(base_url, &client),
        },
        Err(_) => login(base_url, &client),
    };
}

async fn verify_token(token: &str, base_url: &str, client: &reqwest::Client) -> Result<String, ()> {
    todo!()
}

async fn login(base_url: &str, client: &reqwest::Client) -> Result<String, ()> {
    let user = std::env::var("MATRIX_USER").expect("The MATRIX_USER env var cannot be found");
    let password = std::env::var("MATRIX_PASS").expect("The MATRIX_PASS env var cannot be found");

    let login_url = format!("{}/_matrix/client/r0/login", base_url);
    let login_send_body_obj = LoginRequestBody::new(user.as_str(), password.as_str());
    let login_send_body_json =
        serde_json::to_string(&login_send_body_obj).expect("Bad json request");
    // println!("{:?}", login_send_body_json);
    let login_response_json = client
        .post(login_url)
        .body(login_send_body_json)
        .send()
        .await
        .expect("Login error")
        .text()
        .await
        .expect("Login error");
    println!("{}", login_response_json);
    let login_response_err_obj_result =
        serde_json::from_str::<MatrixErrorResponseBody>(&login_response_json);
    match login_response_err_obj_result {
        Ok(login_response_err_obj) => {
            println!("{:?}", login_response_err_obj);
        }
        Err(_) => {
            let login_response_obj =
                serde_json::from_str::<LoginResponseBody>(&login_response_json)
                    .expect("Bad json response");
            println!("{:?}", login_response_obj);
        }
    }
}
