# Mailify

Rust mail server with React Email templates, persistent priority queue, per-request SMTP overrides, JWT auth, and a built-in Swagger UI.

- **Dynamic config** via env vars / `.env` / TOML — no code changes to rebrand or swap SMTP.
- **React Email templates** compiled to HTML at build time, with full i18n (EN/FR shipped, any locale added by convention).
- **Tailwind theming** — brand colors, fonts, radius, logo, social links injected into every built-in template.
- **Priority queue** backed by Postgres (apalis). Jobs survive restarts and resume automatically.
- **JWT + argon2 API keys** — issue short-lived bearer tokens from long-lived keys.
- **Per-request SMTP override** for `/mail/send-custom` — credentials held in memory only.
- **Swagger UI** at `/swagger-ui` wired with the bearer JWT auth scheme so you can test every route from the browser.
- **Dockerized, multi-arch** (linux/amd64 + linux/arm64) CI/CD pipeline pushes to Docker Hub on tag.

## Stack

| Concern       | Tech                                    |
|---------------|-----------------------------------------|
| HTTP          | axum + tokio + tower                    |
| SMTP          | lettre                                  |
| Templates     | React Email (Bun) → HTML + minijinja    |
| Queue         | apalis + Postgres                       |
| Auth          | jsonwebtoken + argon2                   |
| Config        | figment (env + TOML) + dotenvy          |
| Docs          | utoipa + utoipa-swagger-ui              |
| Observability | tracing + tracing-subscriber            |
| Tests         | cargo test + testcontainers (planned)   |

## Workspace layout

```
Mailify/
├── crates/
│   ├── mailify-core/         # domain types, errors
│   ├── mailify-config/       # figment loader, theming, dotenv
│   ├── mailify-templates/    # HTML loader + minijinja renderer
│   ├── mailify-smtp/         # lettre wrapper + per-request override
│   ├── mailify-queue/        # apalis jobs (priority, retry, resume)
│   ├── mailify-auth/         # JWT issuer + axum middleware
│   └── mailify-api/          # axum bin `mailify` + OpenAPI
├── templates-parser/         # React Email source (Bun)
├── docker/Dockerfile         # multi-stage (bun-builder → rust-builder → distroless)
├── docker-compose.yml        # postgres + mailpit + mailify
└── .github/workflows/ci.yml  # fmt, clippy, test, docker push
```

## Quick start (Docker Compose)

```bash
cp .env.example .env
make up
# http://localhost:8080/swagger-ui      → interactive API docs
# http://localhost:8025                 → Mailpit inbox (SMTP sink)
```

## Local dev (no Docker)

```bash
# 1. Compile templates (first run only / after edits)
make gen build-templates

# 2. Start Postgres + Mailpit
docker compose up -d postgres mailpit

# 3. Run server
make dev
```

Or run the full flow with `make setup && make dev`.

## Routes

| Method | Path                           | Auth    | Description                                            |
|--------|--------------------------------|---------|--------------------------------------------------------|
| GET    | `/health`                      | —       | Liveness probe                                         |
| POST   | `/auth/token`                  | —       | Exchange API key for JWT                               |
| GET    | `/config`                      | JWT     | Sanitized runtime config (secrets redacted)            |
| GET    | `/templates`                   | JWT     | Template catalog (id, category, locales)               |
| GET    | `/templates/{id}/preview`      | JWT     | Render template as HTML (or JSON with `?json=true`)    |
| POST   | `/templates/{id}/preview`      | JWT     | Render with a JSON body (`{locale, vars, json}`)       |
| POST   | `/mail/send`                   | JWT     | Queue email using a built-in template                  |
| POST   | `/mail/send-custom`            | JWT     | Queue email with caller-supplied HTML + optional SMTP  |
| GET    | `/swagger-ui`                  | —       | Swagger UI (bearer JWT auth wired)                     |
| GET    | `/api-docs/openapi.json`       | —       | OpenAPI 3 schema                                       |

### Issuing an API key

API keys are stored in config as `<id> → argon2_hash`. Generate a hash:

```bash
make hash-key KEY=super-secret-value
# MAILIFY_AUTH__API_KEYS__WEB=$argon2id$v=19$m=19456,t=2,p=1$...
```

Add the generated line to `.env`, then call `POST /auth/token` with `{"api_key_id":"web","api_key":"super-secret-value"}`.

## Configuration

All config is runtime. Precedence (low → high):

1. Built-in defaults ([`AppConfig::default`](crates/mailify-config/src/lib.rs))
2. Optional TOML file at `MAILIFY_CONFIG`
3. `.env.<MAILIFY_ENV>.local` → `.env.<MAILIFY_ENV>` → `.env.local` → `.env`
4. Process env

Nested fields use `__` separator:

```
MAILIFY_SMTP__HOST=smtp.sendgrid.net
MAILIFY_SMTP__TLS=tls
MAILIFY_THEME__COLORS__PRIMARY=#2563eb
MAILIFY_AUTH__API_KEYS__WEB=$argon2id$...
```

Disable dotenv: `MAILIFY_DOTENV=false`. See [.env.example](.env.example) for the full surface.

## Templates

Built-in template source lives at [templates-parser/emails](templates-parser/emails). Add a new template by appending an entry to [scripts/templates.config.ts](templates-parser/scripts/templates.config.ts), then:

```bash
make build-templates
```

This runs the generator, exports HTML via `email export`, decodes HTML entities inside `{{ }}`/`{% %}` spans, copies subject/text sidecars, and writes `catalog.json`. Output lands in `templates-parser/out/<id>/<locale>.html` — exactly what the Rust registry reads at boot.

Theming tokens are referenced as `{{ theme.colors.primary }}`, `{{ theme.fonts.body }}`, etc. Any variable under `vars` in the send payload is available as `{{ vars.* }}`.

## Deployment

Tag a release — CI builds multi-arch images and pushes to Docker Hub.

```bash
git tag v0.1.0
git push origin v0.1.0
```

Required repo secrets: `DOCKERHUB_USERNAME`, `DOCKERHUB_TOKEN`.

Distroless runtime image is `~20 MB` compressed, runs as non-root, exposes port `8080`.

## Testing

```bash
make test        # cargo test --workspace
make check       # cargo check --workspace --all-targets
make clippy      # cargo clippy --workspace --all-targets -- -D warnings
make fmt         # cargo fmt --all
```

## License

MIT.
