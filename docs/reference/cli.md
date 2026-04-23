---
title: CLI reference
description: The mailify binary, the Make targets, and the cargo-run examples that ship with Mailify.
sidebar:
  order: 3
---

# CLI reference

Mailify ships a single long-running binary (`mailify`) plus a handful of Cargo examples that stand in for ops subcommands. A first-class `mailify` CLI with subcommands (`init`, `config check`, `config print`) is on the roadmap — see [TODO.md §3.3](https://github.com/donilite/mailify/blob/master/TODO.md).

## `mailify`

The server binary. Takes no arguments — all configuration comes from `Mailify.toml`, dotenv, and env vars.

```bash
mailify
```

Startup order:

1. Load dotenv (unless `MAILIFY_DOTENV=false`).
2. Discover and merge `Mailify.toml` (see [config discovery](./config.md#discovery)).
3. Merge `MAILIFY_*` env vars on top.
4. Initialize tracing (respects `RUST_LOG`).
5. Ping Postgres with `SELECT 1` and run apalis migrations.
6. Load `TemplateRegistry` from `MAILIFY_TEMPLATES__PATH`.
7. Maybe-bootstrap auth (generate ephemeral key if none configured).
8. Spawn the queue worker.
9. Start the axum HTTP server on `MAILIFY_SERVER__HOST:MAILIFY_SERVER__PORT`.

## Ops helpers (via Cargo examples)

These are thin binaries kept as `cargo run -p mailify-auth --example <name>` so they share the crate's code without needing a second published binary.

### `hash-key` — argon2-hash an API key

```bash
cargo run -p mailify-auth --example hash-key -- <plaintext> <id>

# via Make:
make hash-key KEY=my-secret-key ID=web
```

Prints a `MAILIFY_AUTH__API_KEYS__<ID>=<hash>` line you can paste into your env source.

### `issue-token` — mint a JWT offline

```bash
cargo run -p mailify-auth --example issue-token -- <subject> <scopes-csv>

# via Make:
make issue-token SUBJECT=dev SCOPES=mail:send,mail:admin
```

Uses the same JWT secret the server would — useful for scripted tests or for bootstrapping clients without round-tripping through `/auth/token`.

## Make targets

The project's `Makefile` is the canonical entry point for routine tasks.

```bash
make help                  # list every target with inline help text
```

Most-used targets:

| Target | What it does |
|--------|--------------|
| `make dev` | `cargo run --bin mailify` — assumes deps are up. |
| `make build` | `cargo build --release` |
| `make test` | `cargo test --workspace` |
| `make check` | `cargo check --workspace --all-targets` |
| `make clippy` | `cargo clippy --workspace --all-targets -- -D warnings` |
| `make fmt` / `make fmt-check` | rustfmt apply / verify |
| `make ci` | Full CI parity: fmt-check + clippy + test |
| `make setup` | `bun install` inside `templates-parser/` (once) |
| `make gen` | Regenerate `.tsx` + sidecars from `scripts/templates.config.ts` |
| `make build-templates` | Full template pipeline (generate → export → post-build) |
| `make dev-templates` | React Email preview server on `:3000` |
| `make up` | `docker-compose up` (postgres + mailpit + mailify) |
| `make up-deps` | Just postgres + mailpit — for local `make dev` |
| `make down` / `make down-volumes` | Tear down |
| `make docker-build` | Build local image `mailify:local` |
| `make openapi` | Curl running server's `/api-docs/openapi.json` into `./openapi.json` |
| `make hash-key KEY=… ID=…` | argon2-hash an API key (see above) |
| `make issue-token SUBJECT=… SCOPES=…` | Mint a JWT offline (see above) |

## Running a single test

```bash
cargo test -p <crate> <test_name>

# examples:
cargo test -p mailify-auth jwt::tests::issues_and_validates
cargo test -p mailify-config tests::user_config_dir_prefers_xdg
```
