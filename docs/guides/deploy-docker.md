---
title: Deploy with Docker
description: Production-ready docker-compose stack for Mailify with Postgres, backups, and secret management.
sidebar:
  order: 5
---

# Deploy with Docker

Mailify ships a distroless image (~20 MB, non-root) that expects Postgres + SMTP and nothing else. This guide covers a production-ready compose stack; for local dev, see the [Quickstart](../getting-started/quickstart.md).

## Minimal production stack

```yaml
# docker-compose.yml
services:
  postgres:
    image: postgres:16-alpine
    restart: unless-stopped
    environment:
      POSTGRES_USER: ${POSTGRES_USER}
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
      POSTGRES_DB: ${POSTGRES_DB}
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD", "pg_isready", "-U", "${POSTGRES_USER}"]
      interval: 10s
      timeout: 5s
      retries: 5
    # Don't expose to the public internet — Mailify connects on the private network.
    # ports: ["127.0.0.1:5432:5432"]  # uncomment for host-side psql access

  mailify:
    image: donighost/mailify:${MAILIFY_TAG:-latest}
    restart: unless-stopped
    depends_on:
      postgres:
        condition: service_healthy
    ports:
      - "127.0.0.1:8080:8080"   # put a reverse proxy in front
    environment:
      # Server
      MAILIFY_SERVER__HOST: 0.0.0.0
      MAILIFY_SERVER__PORT: "8080"

      # Database
      MAILIFY_DATABASE__URL: postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@postgres:5432/${POSTGRES_DB}
      MAILIFY_DATABASE__MAX_CONNECTIONS: "20"

      # SMTP (example: Resend)
      MAILIFY_SMTP__HOST: smtp.resend.com
      MAILIFY_SMTP__PORT: "587"
      MAILIFY_SMTP__TLS: starttls
      MAILIFY_SMTP__USERNAME: resend
      MAILIFY_SMTP__PASSWORD: ${RESEND_API_KEY}
      MAILIFY_SMTP__DEFAULT_FROM_EMAIL: no-reply@yourdomain.com
      MAILIFY_SMTP__DEFAULT_FROM_NAME: Your App

      # Auth
      MAILIFY_AUTH__JWT_SECRET: ${MAILIFY_JWT_SECRET}
      MAILIFY_AUTH__JWT_TTL_SECS: "3600"
      MAILIFY_AUTH__BOOTSTRAP: "false"
      MAILIFY_AUTH__API_KEYS__WEB: ${MAILIFY_API_KEY_WEB_HASH}

      # Queue
      MAILIFY_QUEUE__WORKER_CONCURRENCY: "8"
      MAILIFY_QUEUE__MAX_RETRIES: "5"

      # Observability
      MAILIFY_OBSERVABILITY__LOG_LEVEL: info
      MAILIFY_OBSERVABILITY__LOG_FORMAT: json
      RUST_LOG: "mailify=info,mailify_api=info,tower_http=info"

volumes:
  postgres_data:
```

And a `.env` (gitignored) alongside:

```bash
POSTGRES_USER=mailify
POSTGRES_PASSWORD=<long-random>
POSTGRES_DB=mailify
RESEND_API_KEY=re_live_...
MAILIFY_JWT_SECRET=<64-char-random>
MAILIFY_API_KEY_WEB_HASH='$argon2id$v=19$m=19456,t=2,p=1$...'
MAILIFY_TAG=0.1.2
```

Generate the `MAILIFY_JWT_SECRET`:

```bash
openssl rand -hex 32
```

Generate the API key hash:

```bash
docker run --rm -it donighost/mailify:latest \
  /app/mailify --help   # (future) subcommand
# or, from a clone of the repo:
make hash-key KEY=<plaintext> ID=web
```

Boot:

```bash
docker compose up -d
docker compose logs -f mailify
```

## Reverse proxy

Don't expose `8080` directly. Put Caddy / nginx / Traefik in front for TLS + rate limiting.

### Caddy example

```txt
mailify.yourdomain.com {
    reverse_proxy localhost:8080
    encode zstd gzip
    log {
        output file /var/log/caddy/mailify.log
    }
}
```

### nginx example

```nginx
server {
    listen 443 ssl http2;
    server_name mailify.yourdomain.com;

    ssl_certificate     /etc/letsencrypt/live/mailify.yourdomain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/mailify.yourdomain.com/privkey.pem;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Rate limit the auth endpoint to slow down brute force against argon2
    location = /auth/token {
        limit_req zone=mailify_auth burst=5 nodelay;
        proxy_pass http://127.0.0.1:8080;
    }
}
```

## Templates

The Docker image bakes the default template bundle at `/app/templates` (set via `MAILIFY_TEMPLATES__PATH=/app/templates`). To ship your own:

1. Build templates on your side:

   ```bash
   cd templates-parser && bun install && bun run build
   ```

2. Mount the output directory:

   ```yaml
   services:
     mailify:
       volumes:
         - ./my-templates/out:/app/templates:ro
       environment:
         MAILIFY_TEMPLATES__PATH: /app/templates
   ```

3. Or bake your own image: `FROM donighost/mailify:0.1.2` + `COPY my-templates/out /app/templates`.

## Backups

Mailify stores all durable state in Postgres. Back up Postgres, nothing else.

```bash
# nightly cron
docker compose exec -T postgres \
  pg_dump -U "$POSTGRES_USER" "$POSTGRES_DB" | gzip > "/backups/mailify-$(date +%F).sql.gz"
```

Restore:

```bash
gunzip -c backup.sql.gz | docker compose exec -T postgres psql -U "$POSTGRES_USER" -d "$POSTGRES_DB"
```

## Upgrades

Mailify uses semantic versioning on tags.

```bash
# pin a specific version in .env, then:
docker compose pull mailify
docker compose up -d mailify
```

Apalis schema migrations run automatically at boot. If the upgrade includes a major version bump, read the release notes for breaking changes.

## Health probes

`GET /health` returns `200 OK` when the process is live. Wire it into your orchestrator:

```yaml
# in docker-compose, or translate to k8s livenessProbe
healthcheck:
  test: ["CMD", "wget", "--spider", "-q", "http://localhost:8080/health"]
  interval: 30s
  timeout: 3s
  retries: 3
```

Note `/health` does **not** check Postgres or SMTP reachability — it's a pure liveness probe. Readiness checks via DB ping are on the roadmap.

## Resource sizing

Rules of thumb:

- **CPU** — 0.25 vCPU idle, 1 vCPU when argon2-verifying under load (API key exchange). Scale up if `/auth/token` is called often without token caching on the client.
- **Memory** — 64–128 MB baseline. Grows with `worker_concurrency × avg_template_size`.
- **Network** — outbound to SMTP provider. Nothing else.

One instance comfortably handles thousands of sends per minute through a well-provisioned SMTP upstream. Horizontal scale is a matter of spawning more containers pointing at the same Postgres.
