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
Copy `example.config.toml` to `config.toml` and change the fields to suit your needs.

## Usage
```bash
matrix-bot --help
```
```bash
matrix-bot --room "\!roomid:matrix.org" --message "Lorem ipsum dolor sit amet"
```
