---
title: Debugging
description: Tactics for diagnosing Mailify — logs, tracing, queue inspection, SMTP checks.
sidebar:
  order: 3
---

# Debugging

When something's off, start at the top of this list and work down.

## 1. Turn up the logs

Mailify honors `RUST_LOG` with higher priority than the config file. For deep dives:

```bash
RUST_LOG="mailify=debug,mailify_api=debug,mailify_queue=debug,tower_http=debug,apalis=info" mailify
```

What each target is useful for:

- `mailify_api` — HTTP request/response, middleware (auth, trace layer).
- `mailify_queue` — worker lifecycle, job pickup, retries, failures.
- `mailify_templates` — registry load, render errors.
- `mailify_auth` — JWT issue/verify flow.
- `tower_http` — raw request spans with latency + status.
- `apalis` — underlying queue engine.

Set `MAILIFY_OBSERVABILITY__LOG_FORMAT=json` to emit JSON logs for shipping into Loki / Datadog / ELK.

## 2. Inspect the resolved config

```bash
curl -s -H "authorization: Bearer $TOKEN" http://localhost:8080/config | jq
```

Passwords and the JWT secret are redacted. Everything else is authoritative — if a value looks wrong here, check the env vs. TOML precedence order.

## 3. Check health + Postgres

```bash
curl http://localhost:8080/health

# or straight to Postgres:
psql "$MAILIFY_DATABASE__URL" -c 'SELECT count(*) FROM apalis.jobs;'
psql "$MAILIFY_DATABASE__URL" -c "SELECT status, count(*) FROM apalis.jobs GROUP BY status;"
```

## 4. Inspect the queue

```sql
-- last 20 jobs
SELECT id, status, attempts, last_error, run_at, done_at
FROM apalis.jobs
ORDER BY run_at DESC
LIMIT 20;

-- stuck-in-running jobs
SELECT id, attempts, lock_at
FROM apalis.jobs
WHERE status = 'Running'
  AND lock_at < now() - INTERVAL '10 minutes';

-- failed jobs with errors
SELECT id, last_error, attempts
FROM apalis.jobs
WHERE status = 'Failed'
ORDER BY done_at DESC NULLS LAST
LIMIT 20;
```

## 5. Test SMTP connectivity standalone

Bypass Mailify entirely:

```bash
# one-liner to prove the SMTP endpoint works from wherever Mailify is running
swaks --to you@example.com \
      --from no-reply@example.com \
      --server smtp.example.com:587 \
      --auth LOGIN --auth-user postmaster --auth-password 'secret' \
      --tls
```

If `swaks` succeeds and Mailify fails, the issue is in Mailify's config (tls mode, credentials) or auth handshake. If `swaks` also fails, fix the SMTP provider or network route first.

## 6. Render a template in isolation

Don't send to debug a template — use the preview endpoint:

```bash
curl -s -H "authorization: Bearer $TOKEN" \
  -H 'content-type: application/json' \
  -d '{ "locale": "en", "vars": { "first_name": "Test" } }' \
  http://localhost:8080/templates/welcome/preview | less
```

This renders through the same minijinja pipeline but returns the HTML instead of enqueuing a send.

## 7. Reproduce in `docker compose`

Most "it works on my laptop" bugs are environment mismatches (Docker network, env var casing, missing secret). Boot the stack locally with the exact compose file from [Quickstart](../getting-started/quickstart.md), reproduce, and compare.

## 8. When all else fails

Open an issue at <https://github.com/donilite/mailify/issues> with:

- Mailify version (`mailify --version` once implemented, or the image tag you ran).
- The `GET /config` dump (redact anything extra you consider sensitive).
- Relevant log lines at `RUST_LOG=debug`.
- A minimal reproduction (the smallest request + config that triggers it).
