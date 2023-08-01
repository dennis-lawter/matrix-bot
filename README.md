# Matrix Bot
A command line tool for sending messages to a matrix chatroom.

<p align="center">
  <!-- CI -->
  <img src="https://github.com/dennis-lawter/matrix-bot/workflows/CI/badge.svg" />
  <!-- codecov -->
  <img src="https://codecov.io/gh/dennis-lawter/matrix-bot/branch/master/graph/badge.svg" />
  <!-- unsafe forbidden -->
  <a href="https://github.com/rust-secure-code/safety-dance/">
    <img src="https://img.shields.io/badge/unsafe-forbidden-success.svg"
      alt="Unsafe Rust forbidden" />
  </a>
</p>

## Config
Run `matrix-notify generate` to create an example config file named `matrix-notify.toml` in the current directory.

The config file will need to be modified to suit your connection.

```cfg
base_url = "https://example.org"
local_username = "matrix-bot"
full_username = "@matrix-bot:example.org"

# optional, will be used to generate a token by logging in
password = "Plaintext password"
# optional, will be populated by login automatically if using password
token = "access_token from previous api calls"
```

## Usage
```bash
matrix-bot --help
```
```bash
matrix-bot --room "\!roomid:matrix.org" --message "Lorem ipsum dolor sit amet"
```
