---
title: Configuration reference
description: Every config option Mailify understands — TOML path, env var, default, and why it exists.
sidebar:
  order: 1
---

# Configuration reference

Every runtime setting Mailify honors lives in this table. The same value can be set three ways — TOML, dotenv, or direct process env — with precedence resolved as documented in [Concepts: Config precedence](../getting-started/concepts.md#config-precedence).

## Discovery

Mailify looks for a TOML config file in this order at startup, using the first one that exists:

1. `$MAILIFY_CONFIG` (explicit path)
2. `./Mailify.toml` in the current working directory
3. Per-user config dir:
   - Linux/macOS: `$XDG_CONFIG_HOME/mailify/config.toml` (fallback `~/.config/mailify/config.toml`)
   - Windows: `%APPDATA%\mailify\config.toml`
4. `/etc/mailify/config.toml` (Unix system-wide, last resort)

If no file is found, defaults + env vars still drive config.

## Environment variables

The env prefix is `MAILIFY_`. Nested keys use double underscore `__`:

| TOML path | Env var |
|-----------|---------|
| `server.host` | `MAILIFY_SERVER__HOST` |
| `smtp.host` | `MAILIFY_SMTP__HOST` |
| `auth.api_keys.web` | `MAILIFY_AUTH__API_KEYS__WEB` |
| `theme.colors.primary` | `MAILIFY_THEME__COLORS__PRIMARY` |

## `[server]`

| Key | Type | Default | Purpose |
|-----|------|---------|---------|
| `host` | string | `0.0.0.0` | Bind address. Use `127.0.0.1` for loopback-only. |
| `port` | u16 | `8080` | HTTP listen port. |
| `request_timeout_secs` | u64 | `30` | Per-request timeout applied by `tower-http`. |
| `body_limit_bytes` | usize | `10485760` (10 MiB) | Max request body. Raise if you attach large files via `/mail/send-custom`. |

## `[database]`

| Key | Type | Default | Purpose |
|-----|------|---------|---------|
| `url` | string | `postgres://mailify:mailify@localhost:5432/mailify` | Postgres connection URL. Used by apalis for job storage and migrations. |
| `max_connections` | u32 | `10` | sqlx pool upper bound. |
| `min_connections` | u32 | `1` | sqlx pool lower bound (kept warm). |

## `[smtp]`

Process-wide default SMTP sender. Can be overridden per-job via `smtp_override`.

| Key | Type | Default | Purpose |
|-----|------|---------|---------|
| `host` | string | `localhost` | SMTP server hostname. |
| `port` | u16 | `1025` | SMTP port. 25/587/465 in prod, 1025 for Mailpit dev. |
| `username` | option\<string> | — | SMTP AUTH user. Omit for unauthenticated. |
| `password` | option\<string> | — | SMTP AUTH password. **Never logged.** |
| `tls` | enum | `none` | One of `none`, `starttls`, `tls`. `starttls` for 587, `tls` for 465. |
| `default_from_email` | string | `no-reply@mailify.local` | Default `From:` when the caller omits it. |
| `default_from_name` | option\<string> | `Mailify` | Display name paired with default From. |
| `timeout_secs` | u64 | `30` | lettre SMTP client timeout. |

## `[auth]`

| Key | Type | Default | Purpose |
|-----|------|---------|---------|
| `jwt_secret` | string | `CHANGE_ME_IN_PRODUCTION` | HS256 secret. **Must** be changed in prod. |
| `jwt_issuer` | string | `mailify` | `iss` claim. |
| `jwt_ttl_secs` | u64 | `3600` | JWT lifetime. |
| `api_keys` | map\<id, hash> | `{}` | Argon2-hashed API keys. Key = id, value = hash. |
| `bootstrap` | bool | `true` | If `true` **and** `api_keys` is empty at boot, auto-generate an ephemeral key. |

Generate an argon2 hash:

```bash
make hash-key KEY=my-secret ID=web
# or
cargo run -p mailify-auth --example hash-key -- "my-secret" "web"
```

Paste the printed `MAILIFY_AUTH__API_KEYS__WEB=…` line into your env source.

## `[queue]`

| Key | Type | Default | Purpose |
|-----|------|---------|---------|
| `worker_concurrency` | usize | `4` | Simultaneous in-flight jobs the worker will run. |
| `max_retries` | usize | `5` | Max retry count before a job is marked `Failed`. |
| `retry_backoff_secs` | u64 | `30` | Seconds between retries. |

## `[templates]`

| Key | Type | Default | Purpose |
|-----|------|---------|---------|
| `path` | path | `./templates-parser/out` | Directory with compiled HTML bundle. |
| `strict` | bool | `false` | If `true`, missing built-in template files for the default locale fail startup. |

## `[theme]`

Branding tokens injected into every render under the `{{ theme.* }}` namespace.

| Key | Type | Default | Purpose |
|-----|------|---------|---------|
| `brand_name` | string | `Mailify` | Shown in subject lines, footer, default logo alt. |
| `brand_logo_url` | option\<string> | — | Public URL to the logo (prefer HTTPS + CDN). |
| `radius` | string | `8px` | CSS radius token applied to buttons/cards in templates. |
| `footer_text` | option\<string> | — | Footer legalese. |
| `social_links` | map\<string, string> | `{}` | Social icon → URL map. |
| `extra` | map\<string, string> | `{}` | Arbitrary key/value bag for custom template variables. |

### `[theme.colors]`

All hex strings.

| Key | Default |
|-----|---------|
| `primary` | — |
| `primary_foreground` | — |
| `secondary` | — |
| `secondary_foreground` | — |
| `background` | — |
| `foreground` | — |
| `muted` | — |
| `border` | — |
| `danger` | — |
| `success` | — |

### `[theme.fonts]`

| Key | Default |
|-----|---------|
| `body` | — |
| `heading` | — |

See [Configure theme](../guides/configure-theme.md) for the full mapping to minijinja spans in templates.

## `[i18n]`

| Key | Type | Default | Purpose |
|-----|------|---------|---------|
| `default_locale` | string | `en` | Used when a send request omits `locale`. |
| `fallback_chain` | list\<string> | `["en"]` | Tried in order when the requested locale's asset is missing. |
| `supported_locales` | list\<string> | `["en", "fr"]` | Declared locales — used by startup strict-mode checks. |

## `[observability]`

| Key | Type | Default | Purpose |
|-----|------|---------|---------|
| `log_level` | string | `info` | `tracing` env filter. Overridden by `RUST_LOG` if set. |
| `log_format` | enum | `pretty` | `pretty` or `json`. Use `json` for structured log shipping. |

## Dotenv

In addition to the OS env, Mailify reads the first existing `.env`-style file from:

1. `$MAILIFY_DOTENV_PATH`
2. `.env.<MAILIFY_ENV>.local`
3. `.env.<MAILIFY_ENV>`
4. `.env.local`
5. `.env`

Existing env vars are **not** overridden — dotenv only fills gaps. Disable dotenv entirely with `MAILIFY_DOTENV=false`.

## Example `Mailify.toml`

```toml
[server]
host = "0.0.0.0"
port = 8080

[database]
url = "postgres://mailify:secret@db.internal:5432/mailify"
max_connections = 20

[smtp]
host = "smtp.resend.com"
port = 587
tls = "starttls"
username = "resend"
# password loaded from env: MAILIFY_SMTP__PASSWORD
default_from_email = "hello@acme.com"
default_from_name = "Acme"

[auth]
jwt_secret = "a-very-long-random-secret-from-a-secret-manager"
jwt_ttl_secs = 900
bootstrap = false

[auth.api_keys]
web = "$argon2id$v=19$m=19456,t=2,p=1$...hash..."

[queue]
worker_concurrency = 8
max_retries = 10

[theme]
brand_name = "Acme"
brand_logo_url = "https://cdn.acme.com/logo.svg"
footer_text = "© Acme Corp · 100 Main St · San Francisco"

[theme.colors]
primary = "#2D5BFF"
primary_foreground = "#ffffff"

[observability]
log_format = "json"
```
