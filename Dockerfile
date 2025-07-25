FROM rust:1.88 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY ./src ./src

RUN cargo build --release

RUN ls -al /app

FROM debian:12-slim
WORKDIR /app
COPY --from=builder /app/target/release/catch-all-server-http /app
CMD ["/app/catch-all-server-http"]
