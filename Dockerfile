FROM rust:latest

WORKDIR /app

RUN rustup default nightly

RUN cargo install cargo-watch