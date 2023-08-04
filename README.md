# Matrix Notify
A command line tool for sending messages to matrix chatrooms.

<p align="center">
  <!-- version -->
  <a href="https://crates.io/crates/matrix-notify">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/matrix-notify">
  </a>
  <!-- downloads -->
  <img alt="Crates.io (latest)" src="https://img.shields.io/crates/dv/matrix-notify">
  <!-- unsafe forbidden -->
  <a href="https://github.com/rust-secure-code/safety-dance/">
    <img alt="Unsafe Rust forbidden" src="https://img.shields.io/badge/unsafe-forbidden-success.svg">
  </a>
  <br>
  <!-- CI -->
  <img alt="CI status" src="https://github.com/dennis-lawter/matrix-bot/workflows/CI/badge.svg">
  <!-- codecov -->
  <img alt="Code coverage" src="https://codecov.io/gh/dennis-lawter/matrix-bot/branch/master/graph/badge.svg">
  <!-- quality -->
  <a href="https://app.codacy.com/gh/dennis-lawter/matrix-notify/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade">
    <img alt="Codacy grade" src="https://app.codacy.com/project/badge/Grade/8374737290ba4f3cacfff698040dbccf">
  </a>
</p>

## Installation
To get started, install Matrix Notify using the following command:
```sh
cargo install matrix-notify
```

## Config
Before using Matrix Notify, you'll need to set up a configuration file named `matrix-notify.toml`. To generate a sample config file in the current directory, run the command:
```sh
matrix-notify generate
```

The generated config file should be modified to match your connection settings.

When running `matrix-notify`, the provided token in the config file will be used for authentication. If an authentication error occurs or no token is provided, the password will be used instead. If password authentication succeeds, the config file will be automatically updated with the new token.

You can safely remove the `password` field from the config file once a token is generated. However, if the token becomes invalid, you'll need to provide the password again to obtain a new token.


**Example `matrix-notify.toml`**
```ini
base_url = "https://example.org"
local_username = "matrix-bot"
full_username = "@matrix-bot:example.org"
password = "Plaintext password"
```

## Usage
For optimal security, it's recommended to create a dedicated Matrix user for use with this tool, as user credentials and/or access tokens will be stored in the plaintext config file.

To send a message to a chatroom, make sure to invite the bot user to the chatroom first. The bot user will automatically join the chatroom before sending the message if it's not already a member.

Example usage to send a message:
```sh
matrix-notify --room "\!roomid:matrix.org" --message "Lorem ipsum dolor sit amet"
```
