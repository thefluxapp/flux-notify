FROM rust:1-alpine as build

RUN apk update
RUN apk add musl-dev openssl
RUN apk add libressl-dev

WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest
COPY --from=build /app/target/release/flux-notify /app/
