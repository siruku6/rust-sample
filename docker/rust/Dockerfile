FROM rust:1.85.0-slim-bullseye

RUN apt-get update && \
    apt-get -y install git && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN rustup component add rls rust-analysis rust-src rustfmt clippy && \
    cargo install cargo-edit cargo-watch

# COPY ./docker/rust/Cargo.toml ./Cargo.toml
