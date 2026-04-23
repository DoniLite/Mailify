---
title: Architecture
description: Workspace layout, crate responsibilities, and the request-to-send flow.
sidebar:
  order: 1
---

# Architecture

Mailify is a Cargo workspace (`resolver = "2"`) with seven crates, layered so that only `mailify-api` is binary-shaped. Every other crate is a reusable lib.

## Crate map

```
mailify-core       → domain types (EmailMessage, Priority, SmtpOverride, CoreError)
mailify-config     → figment loader + Theme (TOML + dotenv + env)
mailify-templates  → TemplateRegistry (loads compiled HTML dir) + minijinja renderer
mailify-smtp       → lettre wrapper, accepts per-job SmtpOverride
mailify-queue      → apalis + apalis-sql/postgres MailJob storage + worker runtime
mailify-auth       → argon2 API-key verify + JWT issuer + axum require_jwt middleware
mailify-api        → axum router, OpenAPI (utoipa), Swagger UI, binary `mailify`
```

### `mailify-core`

Pure domain types. No I/O, no async runtime dependencies beyond `tokio` re-exports. Every other crate depends on it.

Key exports:

- `EmailMessage`, `EmailAddress`, `Attachment` — value types for mail content.
- `Priority` — the enum used for queue weighting.
- `SmtpOverride`, `TlsMode` — per-job SMTP credentials. The `Debug` impl redacts secrets.
- `CoreError` — the error enum all other crates lift via `From`.

### `mailify-config`

Responsible for turning environment + TOML + defaults into a single `AppConfig` struct at boot.

Built on [figment](https://github.com/SergioBenitez/figment) for precedence-aware layering. The only public entry point is `AppConfig::load()`. Everything else — `Theme`, `ServerConfig`, etc. — is exposed so `AppConfig` can be cloned into `AppState` downstream.

### `mailify-templates`

Owns the `TemplateRegistry` (file-system scan at boot) and the `TemplateRenderer` (minijinja, with `theme` + `vars` + `locale` context injection).

Critically: **does not parse React Email**. Templates are expected to be pre-rendered HTML. See [Template contract](../reference/template-contract.md) for the layout.

### `mailify-smtp`

A thin wrapper around [lettre](https://github.com/lettre/lettre). Two entry points:

- `SmtpSender::default_from_config(&SmtpConfig)` — the process-wide default sender.
- `SmtpSender::from_override(&SmtpOverride)` — a one-off sender for a single job.

The distinction matters for multi-tenant flows — see [Per-job SMTP override](../guides/per-job-smtp-override.md).

### `mailify-queue`

Wraps [apalis](https://github.com/geofmureithi/apalis) with `PostgresStorage<MailJob>`. Exposes:

- `QueueHandle` — the producer side, used by HTTP handlers to enqueue jobs.
- `QueueRuntime` — the consumer side, spawned in `main.rs`; runs the worker loop with configurable concurrency and retries.

Also owns the PostgreSQL connection + migrations. Running `QueueRuntime::init()` runs the apalis migrations automatically.

### `mailify-auth`

Argon2 + JWT. Public surface:

- `hash_api_key(plaintext) -> ArgonHash` — used by the `hash-key` Cargo example.
- `verify_api_key(plaintext, &hash) -> bool` — called by the `/auth/token` handler.
- `issue_jwt(sub, scopes, ttl, secret, issuer) -> String` — mints tokens.
- `require_jwt` — the axum middleware applied via `.route_layer(...)` in `mailify-api`.

Plus the `bootstrap` module (ephemeral key generation at boot when no keys are configured).

### `mailify-api`

The only binary. Owns:

- The axum router (protected vs. public split).
- `AppState` — the `Arc`-shared state cloned into handlers.
- OpenAPI spec via `utoipa` + Swagger UI at `/swagger-ui`.
- `main.rs` — boot sequence, tracing init, DB ping, template load, bootstrap auth, spawn queue worker, start HTTP server.

## Request → send flow

Step by step, what happens when a client hits `/mail/send`:

1. **Auth.** Client calls `POST /auth/token` with their plaintext API key. `argon2::verify_password` compares against hashes in `cfg.auth.api_keys`. On success, returns a short-lived JWT.
2. **Protected request.** Client calls `POST /mail/send` with `Authorization: Bearer <jwt>`. The `require_jwt` axum middleware verifies the signature and claims. On success, the handler runs.
3. **Template lookup.** For `/mail/send`, the handler looks up `template_id` in `TemplateRegistry` to ensure it exists. `/mail/send-custom` skips this — the raw HTML comes in the request.
4. **Job construction.** Handler builds a `MailJob { id, priority, kind, from, to, locale, vars, smtp_override, ... }`.
5. **Enqueue.** `QueueHandle::push(&job)` inserts into `apalis.jobs` via `PostgresStorage`. apalis assigns its own ULID (`TaskId`), which is what Mailify returns to the caller as `job_id` — **not** the `MailJob.id` UUID.
6. **Worker pickup.** The apalis worker loop polls `apalis.jobs`, locks the next runnable row, and hands it to the worker fn.
7. **Render.** The worker fn builds a `RenderContext { theme, vars, locale }` and calls `TemplateRenderer::render(&job)`. For `Custom` kind, the HTML is passed through as-is.
8. **Dispatch.** If `job.smtp_override` is `Some`, build a one-off `SmtpSender`; otherwise use the process-wide `default_sender` from `AppState`. Send via lettre.
9. **Persist outcome.** apalis marks the job `Done` or increments `attempts` + stores `last_error` and moves back to `Pending` / `Failed` depending on retry count.

## Config precedence

Implemented in `AppConfig::load()`:

```
defaults → auto-discovered TOML → env vars (MAILIFY_*)
```

TOML discovery order (first match wins):

1. `$MAILIFY_CONFIG` (explicit path)
2. `./Mailify.toml`
3. `$XDG_CONFIG_HOME/mailify/config.toml` (or fallback `~/.config/mailify/config.toml` / `%APPDATA%\mailify\config.toml`)
4. `/etc/mailify/config.toml`

See the [config reference](../reference/config.md) for the full set of keys.

## Data flow diagram (text)

```
           ┌──────────┐   api_key
   Client ─┤  Backend ├──────────────► POST /auth/token
           └────┬─────┘                       │
                │ JWT                         ▼
                │                     argon2::verify
                │                          │
                ├───► POST /mail/send ◄─────┘ (bearer JWT)
                │        │
                │        ▼
                │   require_jwt
                │        │
                │        ▼
                │   TemplateRegistry (lookup)
                │        │
                │        ▼
                │   MailJob enqueue
                │        │
                │        ▼                                   ┌──────────────────┐
                │   apalis.jobs (Postgres) ───► worker ──┬──►│ default_sender   │
                │                                       │   │  (lettre + SMTP) │
                │                                       │   └──────────────────┘
                │                                       │   ┌──────────────────┐
                │                                       └──►│ SmtpSender       │
                │                                           │  (from override) │
                │                                           └──────────────────┘
                ▼
          GET /mail/jobs/:id  ◄──── apalis_sql::fetch_by_id ───── apalis.jobs
```

## Dockerfile shape

Three-stage build:

1. **tpl-builder** (`oven/bun:1.3-alpine`) — compiles `.tsx` → HTML bundle.
2. **rs-builder** (`rust:1.88-slim` + `cargo-chef`) — dep layer first (stable), sources layer second (changes often). Produces `./target/release/mailify`.
3. **runtime** (`gcr.io/distroless/cc-debian12:nonroot`) — copies the binary + the template bundle + sets env defaults. No shell, no package manager, no root user.

Final image is ~20 MB and runs as uid `nonroot`.

## Testing philosophy

- **Unit tests live next to the code.** `#[cfg(test)] mod tests` inside each module.
- **Integration tests** under `crates/mailify-api/tests/` spin up the full axum app, a real queue (via `testcontainers` — declared, adoption in progress), and Mailpit as an SMTP sink.
- **CI runs the same `make ci` target** a contributor runs locally: fmt-check + clippy (`-D warnings`) + tests.

No mocking of the database. We'd rather rely on a throwaway Postgres in CI than accept the drift between mocked and real behavior.
