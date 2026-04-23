---
title: Concepts
description: The vocabulary of Mailify — template, theme, job, queue, override, bootstrap.
sidebar:
  order: 3
---

# Concepts

Reading the rest of these docs is easier once the core vocabulary is locked in. Each term below maps to a specific type or module in the codebase.

## Template

A **template** is a directory keyed by an `id` under `MAILIFY_TEMPLATES__PATH`. Inside:

```
<id>/
  <locale>.html            # pre-rendered React Email HTML with minijinja spans
  subject.<locale>.txt     # optional, minijinja-rendered at send
  text.<locale>.txt        # optional plaintext alternative
```

Templates are loaded into an in-memory `TemplateRegistry` at boot. Editing a `.tsx` in `templates-parser/` requires re-running `make build-templates` before restart for changes to appear.

See [Template contract](../reference/template-contract.md).

## Theme

The **theme** is a set of branding tokens (colors, fonts, radius, logo URL, footer text, social links, arbitrary extras) declared once in config and injected into *every* render via the minijinja `{{ theme.* }}` namespace.

Re-branding your entire email output = change config, restart. No template edit needed.

```toml
[theme]
brand_name = "Acme Corp"
brand_logo_url = "https://cdn.acme.com/logo.png"
footer_text = "© Acme. All rights reserved."

[theme.colors]
primary = "#FF4500"
primary_foreground = "#ffffff"
```

See [Configure theme](../guides/configure-theme.md).

## MailJob

Every send becomes a **MailJob** — a struct pushed onto the durable Postgres queue. Key fields:

- `id` — UUID for caller-side tracking.
- `kind` — either `Registered { template_id }` (render from the registry) or `Custom { html, subject, text? }` (one-shot raw HTML).
- `priority` — weighted scheduling. `Low` / `Normal` / `High` / `Critical`.
- `smtp_override` — per-job SMTP credentials, memory-only, never logged. Enables multi-tenant fan-out.
- `vars` — arbitrary JSON passed to the minijinja renderer.
- `locale` — selects which `<id>/<locale>.html` to render.

## Queue / worker

Mailify uses [apalis](https://github.com/geofmureithi/apalis) with `PostgresStorage<MailJob>`. The queue table is `apalis.jobs`. On boot, `PostgresStorage::setup()` runs the apalis migrations automatically.

The worker loop pulls jobs concurrently (`MAILIFY_QUEUE__WORKER_CONCURRENCY`, default 4), retries on failure up to `MAX_RETRIES` with `RETRY_BACKOFF_SECS`, and persists `last_error` on the job row so the `GET /mail/jobs/:id` endpoint can surface it.

Jobs survive server restarts.

## SMTP override

By default every job goes through the **process-wide default SMTP sender** built from `MAILIFY_SMTP__*` config.

When a job carries `smtp_override: { host, port, username?, password?, tls }`, the worker builds a one-off `SmtpSender` for that job. The credentials never hit disk, logs, or the queue table — they live in the in-flight `MailJob` and are wiped when the job finishes.

Primary use case: a multi-tenant SaaS where each tenant brings their own SMTP provider.

See [Per-job SMTP override](../guides/per-job-smtp-override.md).

## Authentication

Two layers:

1. **API key** — long-lived secret, stored as an argon2 hash in `MAILIFY_AUTH__API_KEYS__<ID>`. Generated via `make hash-key` or auto-provisioned at boot in bootstrap mode.
2. **JWT** — short-lived bearer token obtained by `POST /auth/token` with the API key. Used on all protected routes.

The split lets you rotate JWTs without rotating API keys, and lets clients cache tokens instead of re-hashing on every request.

See [Auth & tokens](../guides/auth-and-tokens.md).

## Bootstrap mode

`MAILIFY_AUTH__BOOTSTRAP=true` (default). If no API keys are configured at boot, Mailify:

1. Generates a random key in memory.
2. Hashes it with argon2 in memory.
3. Logs the plaintext + the `MAILIFY_AUTH__API_KEYS__<ID>=<hash>` env line you should copy into a `.env` or config file for persistence.
4. Accepts requests using that key for the lifetime of the process.

This is the zero-friction first-boot path. Disable by setting `MAILIFY_AUTH__BOOTSTRAP=false` in production once you've persisted a real key.

## Config precedence

Lowest → highest:

1. Compile-time defaults (see `AppConfig::default`).
2. Auto-discovered TOML file (see [Config reference](../reference/config.md#discovery)).
3. Dotenv chain (`.env.<MAILIFY_ENV>.local`, etc — does not override existing env vars).
4. Process environment variables prefixed `MAILIFY_`, nested via `__`.

So: `MAILIFY_SMTP__HOST=foo` beats `[smtp] host = "bar"` in `Mailify.toml`, which beats the compile-time default of `localhost`.
