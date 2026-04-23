# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

All routine tasks go through the `Makefile` (run `make help` for the full list).

### Rust
- `make dev` — `cargo run --bin mailify` (assumes Postgres + Mailpit already up via `make up-deps`)
- `make build` — release build
- `make test` — `cargo test --workspace`
- `make check` — `cargo check --workspace --all-targets`
- `make clippy` — `cargo clippy --workspace --all-targets -- -D warnings` (CI treats warnings as errors)
- `make fmt` / `make fmt-check`
- `make ci` — same checks as CI: fmt-check + clippy + test

Run a single test: `cargo test -p <crate> <test_name>` (e.g. `cargo test -p mailify-auth jwt::tests::issues_and_validates`). Testcontainers is declared but integration tests are still planned.

### Templates (Bun / React Email)
- `make setup` — `bun install` inside `templates-parser/` (run once)
- `make gen` — regenerate `.tsx` + sidecar files from `scripts/templates.config.ts`
- `make build-templates` — full pipeline: generate → `email export` → `post-build.ts` (entity-decodes minijinja spans, copies `subject.*.txt` / `text.*.txt`, writes `catalog.json`) into `templates-parser/out/<id>/<locale>.html`
- `make dev-templates` — React Email preview server on `:3000`

Rust reads the compiled HTML at boot from `MAILIFY_TEMPLATES__PATH` (default `./templates-parser/out`). Editing a `.tsx` requires `make build-templates` before restarting the server.

### Docker stack
- `make up` — full stack (postgres + mailpit + mailify) via `docker-compose.yml`
- `make up-deps` — only postgres + mailpit (for local `make dev`)
- `make down` / `make down-volumes` (drops queue data)
- `make docker-build` — local image `mailify:local`

### Ops helpers
- `make hash-key KEY=<plaintext> ID=<id>` — argon2-hash an API key; prints the `MAILIFY_AUTH__API_KEYS__<ID>=…` line to paste into `.env`. Uses `cargo run -p mailify-auth --example hash-key`.
- `make issue-token SUBJECT=<sub> SCOPES=<csv>` — mint a JWT offline with the server's secret (`--example issue-token`).
- `make openapi` — curl the running server's `/api-docs/openapi.json` into `openapi.json`.

## Architecture

Cargo workspace (`resolver = "2"`). Seven crates layered so that `mailify-api` is the only binary:

```
mailify-core      → domain types (EmailMessage, Priority, SmtpOverride, CoreError)
mailify-config    → figment loader + Theme (TOML + dotenv + env; precedence: defaults → TOML → .env chain → process env)
mailify-templates → TemplateRegistry (loads compiled HTML dir) + minijinja renderer
mailify-smtp      → lettre wrapper, accepts per-job SmtpOverride (in-memory credentials)
mailify-queue     → apalis + apalis-sql/postgres MailJob storage + worker runtime
mailify-auth      → argon2 API-key verify + JWT issuer + axum `require_jwt` middleware
mailify-api       → axum router, OpenAPI (utoipa), Swagger UI, binary `mailify`
```

### Request → send flow
1. `POST /auth/token` verifies `api_key` against the argon2 hash in `cfg.auth.api_keys` and returns a short-lived JWT.
2. Protected routes run through `AuthLayer` / `require_jwt` (bearer in `Authorization`).
3. `/mail/send` resolves a `template_id` against `TemplateRegistry`; `/mail/send-custom` accepts raw HTML + optional `smtp_override`.
4. Handlers build a `MailJob` and push it onto `QueueHandle` (apalis `PostgresStorage<MailJob>`). Priority comes from `Priority::weight()` (lower = earlier).
5. `QueueRuntime::run` (spawned in `main.rs`) runs the apalis worker with `worker_concurrency`, `max_retries`, `retry_backoff_secs`. Worker renders the template via `TemplateRenderer` with a `RenderContext { theme, vars, locale }`, then dispatches through `default_sender` **or** a per-job `SmtpSender` built from `SmtpOverride`.
6. Jobs persist in Postgres (`PostgresStorage::setup` runs apalis migrations at boot) — they survive restarts and resume.

### Config precedence (`AppConfig::load`)
1. Built-in defaults (`AppConfig::default` in `crates/mailify-config/src/lib.rs:117`)
2. Optional TOML at `$MAILIFY_CONFIG`
3. Dotenv chain (first match wins, does NOT override already-set vars): `$MAILIFY_DOTENV_PATH` → `.env.<MAILIFY_ENV>.local` → `.env.<MAILIFY_ENV>` → `.env.local` → `.env` → `../../.env`. Disable with `MAILIFY_DOTENV=false`.
4. Env vars prefixed `MAILIFY_`, nested via `__` (e.g. `MAILIFY_THEME__COLORS__PRIMARY=#2563eb`).

`AppConfig` is cloned into `AppState` (shared `Arc`) + passed to `QueueRuntime::init`. Re-theming / re-branding is a config change only — templates read `{{ theme.* }}` at render time.

### Template contract
Directory layout the Rust registry expects at `templates.path`:
```
<id>/<locale>.html           # pre-rendered React Email output with minijinja spans intact
<id>/subject.<locale>.txt    # optional, minijinja-rendered at send
<id>/text.<locale>.txt       # optional plaintext alt
```
Source of truth is `templates-parser/scripts/templates.config.ts`. The `post-build.ts` step is critical: React Email HTML-encodes `{{ }}` / `{% %}`; post-build decodes them back so minijinja can parse at runtime. `strict` mode fails boot if any built-in id is missing for the default locale.

### Docker build
Three-stage `docker/Dockerfile`: `oven/bun:1.3-alpine` compiles templates → `rust:1.82-slim` builds the release binary (cargo registry + `/w/target` cache mounts) → `gcr.io/distroless/cc-debian12:nonroot` ships `/app/mailify` + `/app/templates` (~20 MB, non-root, port 8080). CI builds multi-arch (linux/amd64 + linux/arm64) and pushes to Docker Hub on `v*` tag; requires `DOCKERHUB_USERNAME` + `DOCKERHUB_TOKEN` secrets.
