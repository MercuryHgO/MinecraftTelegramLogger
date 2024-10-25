FROM rust:1.72 as builder

WORKDIR /usr/src/app


RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y musl-tools

COPY Cargo.toml ./
COPY src src

RUN cargo build --target x86_64-unknown-linux-musl --release
 
FROM scratch

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/minecraft_telegram_logger /usr/bin/logger

CMD ["logger"]

