# Copyright 2026 Pranav R S.
# Use of this source code is governed by the MIT license that can be found in the LICENSE file.

FROM rust:1.94-alpine AS builder
RUN apk add --no-cache musl-dev build-base
WORKDIR /app

COPY Cargo.toml Cargo.lock askama.toml sqlx.toml ./

COPY src ./src
COPY migrations ./migrations
COPY templates ./templates

COPY .sqlx ./.sqlx
ENV SQLX_OFFLINE=true

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo build --release && \
    cp target/release/filebase .

FROM alpine:3.24 AS runtime
WORKDIR /app
COPY --from=builder /app/filebase .

EXPOSE ${SERVER_PORT:-3000}
CMD ["./filebase"]