---
title: Dev setup
description: Local toolchain, make targets, and running the full Mailify stack for development.
sidebar:
  order: 2
---

# Dev setup

## Prerequisites

- **Rust** ≥ 1.88 (pinned in `rust-toolchain.toml`)
- **Bun** ≥ 1.3 (for React Email templates)
- **Docker** + **Docker Compose** (for Postgres + Mailpit)
- **make** (the Makefile is the canonical entry point)

Optional:

- `swaks` for SMTP smoke testing.
- `jq` for piping JSON responses.
- `psql` for poking the apalis job table.

## First clone

```bash
git clone https://github.com/donilite/mailify.git
cd mailify

# Install JS deps for the templates workspace
make setup

# Bring up Postgres + Mailpit
make up-deps

# Compile templates once
make build-templates

# Run the server
make dev
```

Visit <http://localhost:8080/swagger-ui> for the API explorer. Mailpit UI at <http://localhost:8025>.

## Daily workflow

| Task | Command |
|------|---------|
| Start deps | `make up-deps` |
| Run server with hot restart | `make dev` (Ctrl+C and re-run on edits) |
| Preview templates | `make dev-templates` |
| Rebuild templates after `.tsx` edit | `make build-templates` |
| Run all tests | `make test` |
| Run a single test | `cargo test -p <crate> <test_name>` |
| Format | `make fmt` |
| Lint (CI-strict) | `make clippy` |
| Full CI check | `make ci` |

## Running integration tests

Integration tests under `crates/mailify-api/tests/` require Postgres + Mailpit.

```bash
# In one terminal, bring up deps:
make up-deps

# In another:
cargo test -p mailify-api --test it_e2e
```

Environment variables the tests expect:

```bash
MAILIFY_DATABASE__URL=postgres://mailify:mailify@localhost:5432/mailify
MAILIFY_SMTP__HOST=localhost
MAILIFY_SMTP__PORT=1025
MAILIFY_SMTP__TLS=none
MAILPIT_API_URL=http://localhost:8025
MAILIFY_DOTENV=false
```

These are set automatically by the CI workflow — see [.github/workflows/ci.yml](https://github.com/donilite/mailify/blob/master/.github/workflows/ci.yml).

## Editor setup

### VSCode

Recommended extensions:

- `rust-analyzer` (Rust)
- `Even Better TOML` (config files — once we publish a JSON schema, this will autocomplete `Mailify.toml`)
- `Prettier` or `Biome` (templates-parser/)

`.vscode/settings.json` suggestions:

```json
{
  "rust-analyzer.cargo.allTargets": true,
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.check.extraArgs": ["--", "-D", "warnings"],
  "[rust]": {
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

### Neovim / Zed / Helix

Configure LSP to target rust-analyzer with the same `--all-targets` + `clippy` args. Format-on-save is enforced by CI (`make fmt-check`), so wire it up in your editor to save headaches.

## Commit style

[Conventional Commits](https://www.conventionalcommits.org/) — prefixes like `feat:`, `fix:`, `chore:`, `refactor:`, `test:`, `docs:`. These drive the auto-generated changelog on release.

Good examples from history:

```
feat: enhance CI/CD pipeline with multi-platform Docker builds and configuration file discovery
test: update expected status from "queued" to "pending" in mailpit delivery test
chore: update package versions to 0.1.2 in Cargo.lock and Cargo.toml
fix: propagate MAILIFY_AUTH__JWT_SECRET through AppState for require_jwt middleware
```

## Before opening a PR

Run:

```bash
make ci
```

That runs fmt-check + clippy (`-D warnings`) + tests. If `make ci` is green, CI will be green too.

Small things to remember:

- **New public API?** Doc-comment it, and if it affects a user-facing endpoint, update `docs/reference/http-api.md`.
- **New config key?** Add it to `docs/reference/config.md` with its TOML path, env var, type, default, and purpose.
- **New error?** Consider adding it to `docs/troubleshooting/common-errors.md` with a diagnosis + fix.
- **Breaking change?** Call it out in the PR description. It'll show up in the release notes.
