FROM rust:1.85.0 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY . .

RUN cargo build --release

CMD ["./target/release/probe-api"]