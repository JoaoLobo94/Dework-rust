FROM amd64/rust:latest

WORKDIR /app

RUN rustup default nightly

RUN cargo install cargo-watch && cargo install diesel_cli && cargo install cross

COPY create_db.sh /app/create_db.sh