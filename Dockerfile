FROM rust:1.85.0 AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock .

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY ./src ./src

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/probe-api .

CMD ["./probe-api"]