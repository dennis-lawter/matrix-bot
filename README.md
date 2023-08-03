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
matrix-notify --help
```
```bash
matrix-notify --room "\!roomid:matrix.org" --message "Lorem ipsum dolor sit amet"
```
