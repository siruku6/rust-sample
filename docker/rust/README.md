# Rust 開発環境構築手順

## Docker 環境構築行う

```bash
$ cd docker/rust

# Prepare .env file
$ cp .env.example .env

# Init and enter docker container
$ docker compose build
$ docker compose run rust bash
```


## 処理の動作確認

- 以下のコマンドを Docker コンテナ内で実行する

```
# 実行コードを build したい場合
cargo build

# build と実行をまとめて行う
cargo run
```
