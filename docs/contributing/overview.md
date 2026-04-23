---
title: Contributing
description: Mailify's philosophy, where it's going, and the three kinds of help it needs.
sidebar:
  order: 0
---

# Contributing to Mailify

Mailify is a solo-built, MIT-licensed project with a clear scope: **give dev teams a self-hosted, theme-aware transactional mail server that takes 5 minutes to deploy and 0 minutes to re-brand.** Every feature added should make that sentence more true.

If you're reading this, you're considering contributing — thank you. This page is the orientation; the rest of this section has the operational details.

## Three kinds of help

### 1. Code (most valuable)

- **Bug fixes.** Real reports tied to real reproductions beat architectural musings every time. Check the [common errors](../troubleshooting/common-errors.md) page — if it's not there and you can reproduce it, we want to know.
- **Feature implementations** that already have a tracked issue or appear in [`TODO.md`](https://github.com/donilite/mailify/blob/master/TODO.md). Drive-by PRs that introduce new scope without prior discussion will be closed with a request to open an issue first.
- **Documentation fixes.** These docs live in `docs/` and are rendered by the website. Typos, outdated commands, missing provider recipes — all welcome as drive-by PRs.
- **Test coverage.** Integration tests for `/mail/send*` edge cases, retries, multi-tenant `smtp_override` scenarios, locale fallbacks. See [dev-setup](./dev-setup.md) for how to run them.

### 2. Reach / promotion

- Write a blog post about deploying Mailify for your use case.
- Compare it against alternatives honestly — strengths *and* weaknesses.
- Star the repo, share it where it fits.
- File issues describing your use case even if you can't fix them — "here's what I tried, here's what broke" is signal.

### 3. Financial

Once Mailify has a GitHub Sponsors / OpenCollective presence, sponsorship tiers will be published there. For now, the best financial support is to deploy it at work and push for your employer to sponsor infrastructure-level OSS.

## Philosophy (the non-negotiable parts)

- **Self-hosted, always.** There will never be a managed "Mailify Cloud" tier maintained by the core project. The docker image is the distribution.
- **Templates are code.** React Email + your IDE beat any WYSIWYG builder for long-term maintainability. This is not up for debate.
- **Rust-fast, but pragmatic.** Clean crate boundaries, but no over-engineering. We'd rather ship a simple sync path that works than a beautiful actor-model abstraction that almost works.
- **Env-var configurable end-to-end.** Every knob reachable via `MAILIFY_*`. No "config is code" patterns that require a rebuild.
- **Durable by default.** Jobs in Postgres. No Redis side-channel for state. Losing a server restart does not lose mail.
- **Secrets never on disk, never in logs.** SMTP credentials, argon2 plaintexts, JWT tokens.
- **Multi-tenant first-class.** Per-job SMTP override is a feature, not a plugin.

## Philosophy (open to debate)

- Scope of the lib crates and whether to publish them to crates.io.
- Whether to grow a `mailify` CLI with subcommands (`init`, `config check`, `config print`).
- Observability story — Prometheus metrics, OTLP traces.
- Rate limiting as a first-class feature vs. "your reverse proxy's problem".
- Admin dashboard UI — currently out of scope but a reasonable experimental fork.

File issues, argue in threads.

## Where to start

Easiest first PRs:

1. Add an SMTP provider recipe to [Configure SMTP](../guides/configure-smtp.md).
2. Add a common error you hit to [Common errors](../troubleshooting/common-errors.md).
3. Improve an existing error message in the Rust code — match it to the troubleshooting page phrasing.
4. Add integration tests for an untested code path (see `crates/mailify-api/tests/`).
5. Pick any item tagged `good-first-issue` on GitHub.

## Next steps

- [Architecture →](./architecture.md)
- [Dev setup →](./dev-setup.md)
