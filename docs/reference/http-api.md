---
title: HTTP API
description: Every endpoint Mailify exposes — auth, send, templates, jobs, health.
sidebar:
  order: 2
---

# HTTP API

Live OpenAPI spec is served at `/api-docs/openapi.json` and a Swagger UI at `/swagger-ui` on every running Mailify instance. This page is the narrated overview; the JSON is the source of truth.

## Authentication

All routes under `/mail/*`, `/templates/*`, and `/config` require a **Bearer JWT** in the `Authorization` header. Acquire one via `POST /auth/token` by exchanging a long-lived API key.

```
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

Public routes: `GET /health`, `POST /auth/token`, `GET /swagger-ui*`, `GET /api-docs/openapi.json`.

## Endpoints

### `GET /health`

Liveness probe. Returns `200 OK` with a JSON body when the process is running. Does not check Postgres or SMTP — see [Observability](../guides/observability.md) for how to add deeper probes.

### `POST /auth/token`

Exchange an API key for a short-lived JWT.

**Request**

```json
{ "api_key": "mky_abc123..." }
```

**Response** `200 OK`

```json
{
  "token": "eyJ...",
  "expires_at": "2026-04-23T14:00:00Z",
  "issuer": "mailify"
}
```

**Errors**

- `401` — invalid or unknown API key (argon2 verification failed).
- `400` — malformed request body.

### `POST /mail/send` *(protected)*

Enqueue a send using a **registered** template from `TemplateRegistry`.

**Request**

```json
{
  "template_id": "welcome",
  "locale": "en",
  "from": { "address": "no-reply@example.com", "name": "Example" },
  "to": [{ "address": "alice@example.com", "name": "Alice" }],
  "cc": [],
  "bcc": [],
  "reply_to": null,
  "priority": "normal",
  "vars": { "first_name": "Alice", "activation_url": "https://..." },
  "headers": { "X-Campaign-Id": "welcome-v2" },
  "subject_override": null,
  "smtp_override": null
}
```

**Response** `202 Accepted`

```json
{ "job_id": "01J8KZ7...", "status": "pending" }
```

- `job_id` is an apalis ULID (string). Use it with `GET /mail/jobs/:id`.
- `status` is always `"pending"` at enqueue time — check the jobs endpoint for live state.

### `POST /mail/send-custom` *(protected)*

Send a one-shot email with caller-supplied raw HTML (no template registry lookup).

**Request**

```json
{
  "from": { "address": "no-reply@example.com", "name": "Example" },
  "to":   [{ "address": "alice@example.com" }],
  "subject": "Hello",
  "html": "<h1>Hi</h1>",
  "text": "Hi",
  "locale": "en",
  "priority": "high",
  "smtp_override": {
    "host": "smtp.tenant.com",
    "port": 587,
    "tls": "starttls",
    "username": "postmaster",
    "password": "secret"
  }
}
```

Same response shape as `/mail/send`. Custom HTML is still sent through the priority queue and the worker.

### `GET /mail/jobs/:id` *(protected)*

Look up the state of a previously enqueued job.

**Response** `200 OK`

```json
{
  "id": "01J8KZ7...",
  "status": "Done",
  "attempts": 1,
  "last_error": null,
  "run_at": "2026-04-23T13:00:00Z",
  "done_at": "2026-04-23T13:00:02Z"
}
```

`status` is one of: `Pending`, `Scheduled`, `Running`, `Done`, `Failed`, `Killed` (apalis state enum).

**Errors**

- `404` — unknown id (or id mismatch — apalis uses ULIDs, not the caller's UUID).

### `GET /templates` *(protected)*

List every template id + locale currently loaded in `TemplateRegistry`.

### `GET /templates/:id/preview` *(protected)*

Render a registered template with placeholder data — useful for design review without sending.

### `POST /templates/:id/preview` *(protected)*

Same, with caller-supplied `vars` to inspect a specific render.

### `GET /config` *(protected)*

Dump the resolved config with secrets redacted. Useful when debugging "which env var actually took effect".

## Priority

`priority` on any send request controls scheduling weight. Lower weight runs earlier.

| Value | Weight |
|-------|--------|
| `critical` | 0 |
| `high` | 10 |
| `normal` | 50 (default) |
| `low` | 100 |

## Rate limiting

Not built-in. Put Mailify behind your reverse proxy (nginx, Caddy, Traefik) or API gateway and rate-limit at the edge.

## OpenAPI schema

```
GET /api-docs/openapi.json
```

Pipe to a file:

```bash
make openapi
# writes ./openapi.json
```
