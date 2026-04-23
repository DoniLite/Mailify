# syntax=docker/dockerfile:1.7

# ─────────────────────────── 1. Bun builder: React Email → HTML ───────────────────────────
FROM oven/bun:1.3-alpine AS tpl-builder
WORKDIR /w
COPY templates-parser/package.json templates-parser/bun.lock* ./templates-parser/
WORKDIR /w/templates-parser
RUN bun install --frozen-lockfile
COPY templates-parser/ ./
RUN bun run build

# ─────────────────────────── 2. Rust builder ───────────────────────────
FROM rust:1.88-slim AS rs-builder
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /w

# Dependency-cache trick: copy manifests first, build stub crates so deps get cached.
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
