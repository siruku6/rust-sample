FROM rust:1.85.0-slim-bullseye

RUN apt-get update && \
    apt-get -y install git cmake vim procps locales \
    # For development and debug (procps: `free -h`)
    # curl net-tools vim procps \
    # For ndarray-linalg of Rust : gcc make ??
    pkg-config libssl-dev gfortran \
    # Solution for "error: failed to run custom build command for `freetype-sys v0.20.1"
    g++ \
    # Solution for "error: failed to run custom build command for `yeslogic-fontconfig-sys v6.0.0`"
    pkg-config libfreetype6-dev libfontconfig1-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# 日本語対応
RUN echo "ja_JP UTF-8" > /etc/locale.gen && \
    locale-gen

# 生成した locale をセット
ENV LANG=ja_JP.UTF-8

RUN rustup component add rls rust-analysis rust-src rustfmt clippy && \
    cargo install cargo-edit cargo-watch cargo-cache

# COPY ./docker/rust/Cargo.toml ./Cargo.toml
