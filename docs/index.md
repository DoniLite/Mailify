---
title: Mailify
description: Self-hosted, theme-aware transactional mail server in Rust. One Docker image, your brand, zero lock-in.
sidebar:
  order: 0
---

# Mailify

**The mail server that wears your brand.**

Mailify is a self-hosted transactional email server written in Rust. You give it a theme, a template id, and a recipient — it sends a branded email through your own SMTP provider. No SaaS, no vendor lock-in, no external service holding your template library hostage.

## Why Mailify

- **Branded out of the box** — palette, fonts, logo, footer text injected into every template via a single `Theme` config object.
- **One Docker image** — `donighost/mailify:<tag>` ships binary + compiled templates + built-in migrations. `docker run`, done.
- **Templates as code** — React Email (`.tsx`) compiles to pre-rendered HTML with minijinja placeholders preserved for runtime variables. Edit in your editor, ship in CI.
- **Durable queue** — jobs persist in Postgres via [apalis](https://github.com/geofmureithi/apalis). A worker crash does not eat your outbound.
- **Per-job SMTP override** — one install can fan out to many tenants, each using their own SMTP provider, with credentials accepted in-memory only.
- **Argon2 + JWT auth** — API keys are argon2-hashed at rest; clients exchange them for short-lived JWTs per session.
- **Rust-fast, distroless-small** — final image ≈20 MB, non-root, nothing but `cc` libs and the binary.

## Who it's for

- **Indie / solo backend devs** who don't want to write a `MJML` template every time.
- **Small SaaS teams** needing Postmark-like ergonomics without paying per email.
- **Agencies** running multi-tenant stacks where each client needs its own visual identity.

If you need drag-and-drop campaign editors, audience segmentation, or bounce-analytics dashboards — Mailify is not for you. This is a *sending* server, not a marketing suite.

## Quick links

- [Install →](./getting-started/installation.md)
- [Send your first mail →](./getting-started/quickstart.md)
- [Concepts & vocabulary →](./getting-started/concepts.md)
- [Full config reference →](./reference/config.md)
- [HTTP API →](./reference/http-api.md)
- [Troubleshooting →](./troubleshooting/common-errors.md)
- [Contribute →](./contributing/overview.md)

## License

MIT. See [`LICENSE`](https://github.com/donilite/mailify/blob/master/LICENSE).
