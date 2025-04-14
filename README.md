# rust-template-for-vscode-remotecontainers

[![CI](https://github.com/siruku6/rust-sample/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/siruku6/rust-sample/actions/workflows/ci.yml)
[![Security audit](https://github.com/siruku6/rust-sample/actions/workflows/security-audit.yml/badge.svg?branch=main)](https://github.com/siruku6/rust-sample/actions/workflows/security-audit.yml)

Rust template development environment using VSCode's RemoteContainers

## Requirements

- [Docker](https://www.docker.com/)
- [Visual Studio Code](https://azure.microsoft.com/ja-jp/products/visual-studio-code/)
- [Remote - Containers(VSCode extension)](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

## Usage

Run `Remote-containers: Reopen in Container` from the command palette ( `⌘` + `⇧` + `P` ).
And if you see "Dev Container: Rust Template" in the bottom left corner, you're good to go!

## The following are installed.

- Docker image
    - rust:1.85.0-slim-bullseye
- CLI tools
    - git
    - cargo-edit
    - cargo-watch
    - cargo-cache
- VSCode extensions
    - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)


## Init dev environment

- [README.md](./docker/rust/README.md)
