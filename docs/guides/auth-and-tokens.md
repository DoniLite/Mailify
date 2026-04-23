---
title: Auth & tokens
description: How Mailify's API-key-to-JWT handshake works, how to rotate secrets, and how to bootstrap a fresh install.
sidebar:
  order: 3
---

# Auth & tokens

Mailify has two auth layers, serving different needs:

| Layer | Who holds it | Lifetime | Verified by |
|-------|-------------|----------|-------------|
| **API key** | Your backend / CI secret store | Long (months) | argon2 hash compare |
| **JWT** | Any client talking to Mailify | Short (default 1 hour) | HS256 signature |

Clients exchange the API key for a JWT once (usually on startup), then reuse the JWT until it expires. This avoids doing a CPU-heavy argon2 verify on every request.

## Generating an API key

```bash
make hash-key KEY=my-secret-plaintext ID=web
```

This prints something like:

```
MAILIFY_AUTH__API_KEYS__WEB=$argon2id$v=19$m=19456,t=2,p=1$...
```

- **Plaintext** (`my-secret-plaintext`): hand to your client. **Never logged by Mailify.**
- **Hash** (the `$argon2id$...` string): put into the Mailify process environment or `Mailify.toml`.

## Adding keys via TOML

```toml
[auth.api_keys]
web       = "$argon2id$v=19$m=19456,t=2,p=1$...hash-1..."
ci-pipe   = "$argon2id$v=19$m=19456,t=2,p=1$...hash-2..."
mobile    = "$argon2id$v=19$m=19456,t=2,p=1$...hash-3..."
```

Each key has an `id` (the map key) that appears in the JWT `sub` claim — useful for per-client audit trails.

## Adding keys via env

```
MAILIFY_AUTH__API_KEYS__WEB=$argon2id$...
MAILIFY_AUTH__API_KEYS__CI_PIPE=$argon2id$...
```

Double-underscore between env path levels, single underscore inside the leaf name. So `AUTH__API_KEYS__CI_PIPE` maps to `auth.api_keys.ci_pipe`.

## Exchanging key for token

```bash
curl -s -X POST http://localhost:8080/auth/token \
  -H 'content-type: application/json' \
  -d '{ "api_key": "my-secret-plaintext" }'
```

```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "expires_at": "2026-04-23T14:00:00Z",
  "issuer": "mailify"
}
```

Use it as `Authorization: Bearer <token>` on every protected route.

## Token claims

Decoded JWT payload:

```json
{
  "iss": "mailify",
  "sub": "web",                    // the api_key id
  "iat": 1745420400,
  "exp": 1745424000,
  "scopes": ["mail:send"]          // reserved for future scope-based authz
}
```

Scopes are declared in claims today but **not** enforced by middleware. Scope-gated routes are on the roadmap.

## Bootstrap mode

The first-boot ergonomic. When `MAILIFY_AUTH__BOOTSTRAP=true` (the default) **and** `cfg.auth.api_keys` is empty, Mailify:

1. Generates a random plaintext key.
2. Argon2-hashes it in memory.
3. Logs both the plaintext and the `MAILIFY_AUTH__API_KEYS__<ID>=<hash>` env line you should persist.
4. Accepts that key for the lifetime of the process.

Looks like this in logs:

```
INFO mailify::bootstrap: generated ephemeral API key — save the env line below before restart
INFO mailify::bootstrap:   id=bootstrap plaintext=mky_abc123def456...
INFO mailify::bootstrap:   MAILIFY_AUTH__API_KEYS__BOOTSTRAP=$argon2id$v=19$m=19456,t=2,p=1$...
```

**Disable in production** once you've persisted a real key:

```toml
[auth]
bootstrap = false
```

Bootstrap mode is only for zero-config demos and first-boot. Keeping it on in prod means an operator restart with a wiped config file would silently mint a new key and hand it to any log reader.

## JWT secret

Set `MAILIFY_AUTH__JWT_SECRET` to a long random string in production. The default (`CHANGE_ME_IN_PRODUCTION`) is a red flag — Mailify **does not** refuse to start with it, but it will log a warning.

Rotating the secret invalidates every issued JWT. See the [FAQ](../troubleshooting/faq.md#how-do-i-rotate-the-jwt-secret-without-breaking-active-clients) for strategies.

## Offline token minting

For scripts, CI, or tests, you can mint a JWT without round-tripping through `/auth/token`:

```bash
make issue-token SUBJECT=ci SCOPES=mail:send
```

This uses the same `jwt_secret` the server would — works only when you control both ends (i.e., not usable by external clients).

## Revocation

Not implemented. JWTs are valid until `exp`. Practical options:

- **Short TTLs.** `jwt_ttl_secs = 300` limits blast radius of a leaked token.
- **Remove the API key.** Removing the argon2 hash from config + restart prevents new token issuance. Already-issued tokens remain valid until expiry.
- **Rotate the JWT secret.** Nuclear option — invalidates every token.

A stateful revocation list is a reasonable PR opportunity.
