---
title: Common errors
description: Mailify error symptoms mapped to causes and fixes.
sidebar:
  order: 1
---

Errors are grouped by stage. If yours isn't here, check [Debugging](./debugging.md) and [FAQ](./faq.md).

:::tip[Speed up debugging]
Set `RUST_LOG=mailify=debug,mailify_api=debug,mailify_queue=debug` before reproducing. Most errors below are easier to identify with the structured logs surfaced by these targets.
:::

## Startup

### `error: figment error: missing field smtp.host`

**Cause** — no `[smtp]` block in `Mailify.toml` and no `MAILIFY_SMTP__HOST` env var. Smtp defaults exist, so this means the TOML file was found but declared `[smtp]` without `host`.

**Fix** — either add `host = "…"` under `[smtp]`, or drop the partial `[smtp]` block entirely so defaults apply.

### `error: pool timed out while waiting for an open connection`

**Cause** — Postgres is unreachable or rejecting connections.

**Diagnostic**

```bash
# inside the Mailify container/host:
pg_isready -h <host> -p <port> -U mailify
psql "$MAILIFY_DATABASE__URL" -c 'SELECT 1'
```

**Fix**

- Check `MAILIFY_DATABASE__URL` for typos in user/password/host.
- Confirm the Postgres container is healthy: `docker compose ps postgres`.
- Inside a Docker network, don't use `localhost` — use the service name (`postgres`, not `localhost`).

### `error: template "<id>" not found for locale "<loc>"`

**Cause** — `TemplateRegistry` scanned `MAILIFY_TEMPLATES__PATH` and found no `<id>/<locale>.html`.

**Fix**

- Did you run `make build-templates` after adding/editing the template?
- Is `MAILIFY_TEMPLATES__PATH` pointing at the right directory? Check with:

  ```bash
  ls "$MAILIFY_TEMPLATES__PATH"/<id>/
  ```

- In Docker, the default is `/app/templates` — don't override unless you mount your own.

### `error: invalid bearer token`

**Cause** — JWT rejected by `require_jwt` middleware. Reasons:

- **Expired** — default TTL is 1 hour. Refresh via `POST /auth/token`.
- **Wrong secret** — the server was restarted with a different `MAILIFY_AUTH__JWT_SECRET`, invalidating every previously issued token.
- **Issuer mismatch** — you minted the token with a different `jwt_issuer`.

**Fix** — request a new token. Verify the server's active secret with `GET /config` (it's redacted, but you can correlate against what you configured).

### `error: argon2 verification failed`

**Cause** — API key did not match any hash in `cfg.auth.api_keys`.

**Fix**

- Double-check you copied the **plaintext** key, not the hash.
- Make sure `MAILIFY_AUTH__API_KEYS__<ID>` contains the **hash**, not the plaintext.
- Regenerate with `make hash-key KEY=… ID=…` if in doubt.

### `MAILIFY_AUTH__BOOTSTRAP=true` but no key printed

**Cause** — `cfg.auth.api_keys` is not actually empty. Bootstrap only runs when there are zero configured keys.

**Fix** — list env vars matching `MAILIFY_AUTH__API_KEYS__*`. Remove any stale values before relying on bootstrap mode.

## Sending

### Mailpit shows nothing

**Cause** — SMTP host resolution inside Docker.

**Fix**

- In `docker-compose.yml`, set `MAILIFY_SMTP__HOST=mailpit` (the service name), not `localhost`.
- When running Mailify **on the host** (`make dev`), `localhost:1025` is fine — Mailpit is published to host ports.
- When running Mailify **in Docker** with Mailpit **on host**, use `host.docker.internal` on Docker Desktop, or the host's LAN IP on Linux.

### `apalis: job stuck in Running forever`

**Cause** — worker process crashed without releasing the job's lock. apalis heartbeats will eventually reclaim it, but in the meantime `GET /mail/jobs/:id` shows `Running`.

**Fix**

- Check `docker compose logs mailify` for the worker panic.
- If stuck past `retry_backoff_secs`, restart the server — apalis will requeue on next poll.
- Persistent hangs often mean an external dependency (SMTP provider) is timing out silently; raise `MAILIFY_SMTP__TIMEOUT_SECS` briefly to confirm, then fix the upstream.

:::danger[SMTP error 550 = upstream rejection]
A `5xx` SMTP code from your provider means the provider refused the message — Mailify just relays the rejection. Don't bury this; investigate domain auth (SPF/DKIM/DMARC) before retrying.
:::

### `relay access denied` / `550 5.7.1`

**Cause** — SMTP provider refused the `From:` domain. Most transactional providers (SES, Mailgun, Postmark) require you to prove domain ownership via DNS records (SPF, DKIM, DMARC) before accepting mail from `@yourdomain.com`.

**Fix** — complete the provider's domain verification flow. Mailify faithfully relays whatever the provider tells it; this is not a Mailify bug.

## Build

### `docker build` on arm64 takes 40+ minutes

**Cause** — you are emulating arm64 on an x86 runner via QEMU. Rust release builds under QEMU are 10–20× slower than native.

**Fix** — switch to native arm runners. See `.github/workflows/docker.yml` in this repo for the matrix (`ubuntu-24.04` + `ubuntu-24.04-arm`) plus a `manifest` job that stitches the digests into a multi-arch image.

### `cargo chef cook` fails with `error: no such subcommand`

**Cause** — `cargo-chef` not installed in the Docker build stage.

**Fix** — the `docker/Dockerfile` installs it in the `chef` base stage:

```dockerfile
RUN cargo install cargo-chef --locked --version ^0.1
```

Make sure the stage that uses `cargo chef …` inherits from `chef`, not from raw `rust:1.88-slim`.

## Config

### Env var does not take effect

**Cause** — wrong nesting syntax. Double underscore separates levels, single underscore is a literal character.

**Fix**

- `MAILIFY_SMTP__HOST` ✅ (smtp.host)
- `MAILIFY_SMTP_HOST` ❌ (parsed as `smtp_host`, which is not a field)
- `MAILIFY_AUTH__API_KEYS__WEB` ✅ (auth.api_keys.web)

Hit `GET /config` to see which values actually landed in the resolved `AppConfig`.

### Mailify picks up an unexpected config file

**Cause** — discovery is hierarchical. If you have a stale `/etc/mailify/config.toml` from an earlier install, it wins over nothing.

**Fix** — check which file was loaded at startup. With `RUST_LOG=mailify=debug`, the server logs the resolved path.
