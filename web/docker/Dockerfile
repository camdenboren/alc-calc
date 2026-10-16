# SPDX-FileCopyrightText: Camden Boren
# SPDX-License-Identifier: GPL-3.0-or-later

FROM docker.io/library/rust:1-bookworm AS builder
WORKDIR /app
RUN rustup toolchain install nightly --profile minimal \
        --target wasm32-unknown-unknown --component rust-src \
    && cargo install trunk --locked
ENV RUSTUP_TOOLCHAIN=nightly
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libfontconfig-dev \
    && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock Trunk.toml ./
COPY web ./web
COPY src ./src
COPY img ./img
RUN trunk build --release

FROM quay.io/nginx/nginx-unprivileged:alpine-slim
COPY --chown=nginx:nginx web/docker/nginx.conf /etc/nginx/nginx.conf
COPY --chown=nginx:nginx --from=builder /app/target/wasm32-unknown-unknown/dist /usr/share/nginx/html

EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]
