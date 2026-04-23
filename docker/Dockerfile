# syntax=docker/dockerfile:1.7

# ─────────────────────────── 1. Bun builder: React Email → HTML ───────────────────────────
FROM oven/bun:1.3-alpine AS tpl-builder
WORKDIR /w
COPY templates-parser/package.json templates-parser/bun.lock* ./templates-parser/
WORKDIR /w/templates-parser
RUN bun install --frozen-lockfile
COPY templates-parser/ ./
RUN bun run build

# ─────────────────────────── 2a. Rust chef base ───────────────────────────
FROM rust:1.88-slim AS chef
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev ca-certificates curl unzip build-essential && rm -rf /var/lib/apt/lists/*
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install cargo-chef --locked --version ^0.1
WORKDIR /w

# ─────────────────────────── 2b. Planner: compute dep recipe ───────────────────────────
FROM chef AS planner
COPY Cargo.toml Cargo.lock* ./
COPY crates/ ./crates/
RUN cargo chef prepare --recipe-path recipe.json

# ─────────────────────────── 2c. Cacher: build deps only (stable layer) ───────────────────────────
FROM chef AS cacher
COPY --from=planner /w/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/w/target \
    cargo chef cook --release --recipe-path recipe.json

# ─────────────────────────── 2d. Builder: compile the workspace binary ───────────────────────────
FROM chef AS rs-builder
COPY --from=cacher /w/target /w/target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY Cargo.toml Cargo.lock* ./
COPY crates/ ./crates/
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/w/target \
    cargo build --release --bin mailify && \
    cp /w/target/release/mailify /w/mailify

# ─────────────────────────── 3. Runtime: distroless ───────────────────────────
FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
WORKDIR /app

COPY --from=rs-builder /w/mailify /app/mailify
COPY --from=tpl-builder /w/templates-parser/out /app/templates

ENV MAILIFY_TEMPLATES__PATH=/app/templates \
    MAILIFY_SERVER__HOST=0.0.0.0 \
    MAILIFY_SERVER__PORT=8080

EXPOSE 8080
USER nonroot:nonroot
ENTRYPOINT ["/app/mailify"]
