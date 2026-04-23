---
title: Quickstart
description: Send your first email through Mailify in five minutes using Docker and Mailpit.
sidebar:
  order: 2
---

# Quickstart

Goal: boot Mailify locally, trade an API key for a JWT, enqueue an email, and inspect it in [Mailpit](https://mailpit.axllent.org/) — all in five minutes.

## 1. Boot the stack

Create a `docker-compose.yml`:

```yaml
services:
  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER: mailify
      POSTGRES_PASSWORD: mailify
      POSTGRES_DB: mailify
    ports: ["5432:5432"]
    healthcheck:
      test: ["CMD", "pg_isready", "-U", "mailify"]
      interval: 5s
      retries: 10

  mailpit:
    image: axllent/mailpit:latest
    ports:
      - "1025:1025"   # SMTP in
      - "8025:8025"   # Web UI
    environment:
      MP_SMTP_AUTH_ACCEPT_ANY: "true"
      MP_SMTP_AUTH_ALLOW_INSECURE: "true"

  mailify:
    image: donighost/mailify:latest
    depends_on:
      postgres:
        condition: service_healthy
    ports: ["8080:8080"]
    environment:
      MAILIFY_DATABASE__URL: postgres://mailify:mailify@postgres:5432/mailify
      MAILIFY_SMTP__HOST: mailpit
      MAILIFY_SMTP__PORT: "1025"
      MAILIFY_SMTP__TLS: none
      MAILIFY_AUTH__JWT_SECRET: dev-secret-change-me
      MAILIFY_AUTH__BOOTSTRAP: "true"
```

Boot it:

```bash
docker compose up -d
```

Mailify starts with `MAILIFY_AUTH__BOOTSTRAP: "true"` and no pre-configured API key, so on first boot it generates an ephemeral key and prints both the plaintext and the `MAILIFY_AUTH__API_KEYS__<ID>=<hash>` line you can persist:

```
docker compose logs mailify | grep -A2 "ephemeral API key"
```

Copy the plaintext (e.g. `mky_abc123…`). You will use it below.

## 2. Trade the API key for a JWT

```bash
API_KEY="mky_abc123..."

TOKEN=$(curl -s -X POST http://localhost:8080/auth/token \
  -H 'content-type: application/json' \
  -d "{\"api_key\": \"$API_KEY\"}" | jq -r '.token')

echo "$TOKEN"
```

The JWT is short-lived (default: 1 hour). See [Auth & tokens](../guides/auth-and-tokens.md) for scopes, TTLs, and refresh strategies.

## 3. Send a custom-HTML email

```bash
curl -s -X POST http://localhost:8080/mail/send-custom \
  -H "authorization: Bearer $TOKEN" \
  -H 'content-type: application/json' \
  -d '{
    "from": { "address": "no-reply@example.com", "name": "Example" },
    "to":   [{ "address": "alice@example.com", "name": "Alice" }],
    "subject": "Hello from Mailify",
    "html": "<h1>It works.</h1><p>Sent via Mailify.</p>",
    "locale": "en"
  }'
```

Response:

```json
{ "job_id": "01J...ULID", "status": "pending" }
```

## 4. Inspect the mail in Mailpit

Open <http://localhost:8025>. Your message appears within seconds of the worker picking up the job.

## 5. Check job state

```bash
curl -s -H "authorization: Bearer $TOKEN" \
  http://localhost:8080/mail/jobs/01J...ULID | jq
```

You should see a status of `Done` (or `Failed` with `last_error` populated) once the worker has processed it.

## Next steps

- [Register your own React Email templates →](../guides/configure-templates.md)
- [Switch SMTP provider →](../guides/configure-smtp.md)
- [Theme your outgoing mail →](../guides/configure-theme.md)
- [Full HTTP API reference →](../reference/http-api.md)
