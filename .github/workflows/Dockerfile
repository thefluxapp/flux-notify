FROM rust:alpine AS builder

WORKDIR /app

COPY . .
RUN apk --no-cache add musl-dev protoc protobuf-dev openssl-dev openssl-libs-static
RUN cargo build --release -p flux-notify -p flux-notify-migration

FROM alpine

WORKDIR /app

RUN apk --no-cache add libssl3

COPY --from=builder /app/target/release/flux-notify /app/
COPY --from=builder /app/target/release/flux-notify-migration /app/
COPY --from=builder /app/settings/default.toml /app/
