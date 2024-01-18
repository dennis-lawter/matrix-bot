use assert_cmd::prelude::*;
use predicates::prelude::*;
use serial_test::file_serial;
use std::{
    env::set_current_dir,
    fs::{self, File},
    io::Write,
    process::Command,
};
use tempfile::{tempdir, TempDir};

use matrix_notify;

fn prepare() -> Result<TempDir, Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    set_current_dir(temp_dir.path())?;

    Ok(temp_dir)
}

#[test]
#[file_serial]
fn test_no_args() -> Result<(), Box<dyn std::error::Error>> {
    prepare()?;

    let mut cmd = Command::cargo_bin("matrix-notify")?;

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("--help"));

    Ok(())
}

#[test]
#[file_serial]
fn test_generate() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = prepare()?;

    let mut cmd = Command::cargo_bin("matrix-notify")?;
    cmd.arg("generate");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("matrix-notify.toml"));

    let temp_file = temp_dir.path().join("matrix-notify.toml");

    let metadata_result = fs::metadata(temp_file.to_str().unwrap());
    assert!(
        metadata_result.is_ok(),
        "After generation, the config file was not found: {}",
        temp_file.to_str().unwrap()
    );

    let metadata = metadata_result.unwrap();
    assert!(metadata.is_file());
    assert!(metadata.len() > 0);

    let file_contents_result = fs::read(temp_file.to_str().unwrap());
    assert!(file_contents_result.is_ok());
    let file_contents_utf8_result = String::from_utf8(file_contents_result.unwrap());
    assert!(file_contents_utf8_result.is_ok());
    let file_contents = file_contents_utf8_result.unwrap();
    assert!(file_contents.contains("base_url"));
    assert!(file_contents.contains("local_username"));
    assert!(file_contents.contains("full_username"));
    assert!(file_contents.contains("password") || file_contents.contains("token"));

    drop(temp_dir);

    Ok(())
}

#[test]
#[file_serial]
fn test_send_with_password() -> Result<(), Box<dyn std::error::Error>> {
    let _temp_dir = prepare()?;
    let room = "!roomid:testmatrix.org";
    let local_username = "testuser";
    let full_username = "@testuser:testmatrix";
    let password = "testpassword";
    let mock_server = matrix_notify::api::mock_server::MockMatrix::new(room, full_username);

    let base_url = format!("http://{}", mock_server.server.host_with_port());

    let config = format!(
        r#"
base_url = "{}"
local_username = "{}"
full_username = "{}"
password = "{}"
"#,
        base_url, local_username, full_username, password
    );

    let temp_dir = prepare()?;
    let temp_file_path = temp_dir.path().join("matrix-notify.toml");
    let mut temp_file = File::create(&temp_file_path)?;
    temp_file
        .write_all(config.as_bytes())
        .expect("Failed to write to temporary config file");

    let mut cmd = Command::cargo_bin("matrix-notify")?;
    cmd.env("RUST_BACKTRACE", "1");
    cmd.arg("--room");
    cmd.arg("!roomid:testmatrix.org");
    cmd.arg("--message");
    cmd.arg("Lorem ipsum dolor sit amet");

    cmd.assert().success();

    let metadata_result = fs::metadata(&temp_file_path);
    assert!(metadata_result.is_ok());
    let metadata = metadata_result.unwrap();
    assert!(metadata.is_file());
    assert!(metadata.len() > 0);

    // mock_server.profile_endpoint.assert();
    mock_server.login_endpoint.assert();

    mock_server.room_members_endpoint.assert();
    // mock_server.join_room_endpoint.assert();

    mock_server.send_message_endpoint.assert();

    Ok(())
}

#[test]
fn test_send_with_token() -> Result<(), Box<dyn std::error::Error>> {
    let _temp_dir = prepare()?;
    let room = "!roomid:testmatrix.org";
    let local_username = "testuser";
    let full_username = "@testuser:testmatrix";
    let token = "testtoken";
    let mock_server = matrix_notify::api::mock_server::MockMatrix::new(room, full_username);

    let base_url = format!("http://{}", mock_server.server.host_with_port());

    let config = format!(
        r#"
base_url = "{}"
local_username = "{}"
full_username = "{}"
token = "{}"
"#,
        base_url, local_username, full_username, token
    );

    let temp_dir = prepare()?;
    let temp_file_path = temp_dir.path().join("matrix-notify.toml");
    let mut temp_file = File::create(&temp_file_path)?;
    temp_file
        .write_all(config.as_bytes())
        .expect("Failed to write to temporary config file");

    let mut cmd = Command::cargo_bin("matrix-notify")?;
    cmd.env("RUST_BACKTRACE", "1");
    cmd.arg("--room");
    cmd.arg("!roomid:testmatrix.org");
    cmd.arg("--message");
    cmd.arg("Lorem ipsum dolor sit amet");

    cmd.assert().success();

    let metadata_result = fs::metadata(&temp_file_path);
    assert!(metadata_result.is_ok());
    let metadata = metadata_result.unwrap();
    assert!(metadata.is_file());
    assert!(metadata.len() > 0);

    mock_server.profile_endpoint.assert();
    // mock_server.login_endpoint.assert();

    mock_server.room_members_endpoint.assert();
    // mock_server.join_room_endpoint.assert();

    mock_server.send_message_endpoint.assert();

    Ok(())
}
