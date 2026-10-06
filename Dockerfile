# syntax=docker/dockerfile:1

FROM rust:1-bookworm AS builder
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 wget \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -u 10001 -g nogroup appuser

WORKDIR /app

COPY config/default.toml config/docker.toml ./config/
COPY --from=builder /app/target/release/jev-mcp-rust /usr/local/bin/jev-mcp-rust

ENV APP_ENV=docker

EXPOSE 8766

USER appuser

ENTRYPOINT ["jev-mcp-rust"]
