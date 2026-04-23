---
title: FAQ
description: Frequently asked questions about Mailify's scope, ops, and roadmap.
sidebar:
  order: 2
---

# FAQ

## Is Mailify a replacement for Postmark / Resend / SES?

Partially. Mailify replaces the **templating + sending + queueing** layers. It does **not** replace the underlying SMTP relay — you still need a provider (or a self-run Postfix) to actually deliver mail to `gmail.com`. Mailify sits *in front of* that relay and handles everything between your app and it.

## Can I use it without Postgres?

No. The durable queue is the reason to use Mailify at all — Postgres is not optional. Sqlite-backed queue is not on the roadmap; if you don't need durability, you don't need Mailify.

## Can I have multiple Mailify instances sharing one Postgres?

Yes. apalis with `PostgresStorage` supports multiple workers on the same `apalis.jobs` table out of the box — jobs are locked atomically. Horizontal scale works.

## How do I rotate the JWT secret without breaking active clients?

Short answer: you can't — rotating `MAILIFY_AUTH__JWT_SECRET` invalidates every token signed with the old secret. Options:

- **Short TTLs + forced refresh.** Set `jwt_ttl_secs` low (e.g. 300). Clients re-call `/auth/token` often. Rotate at a time when fresh tokens will issue soon.
- **Accept downtime.** Rotate during a planned window and tell clients to re-auth.
- **Roll-your-own dual-secret middleware.** Not in core yet.

## How do I rotate an API key?

- Generate a new hash: `make hash-key KEY=new-plaintext ID=new-web`.
- Add it to `cfg.auth.api_keys` alongside the old one. Restart. Both now work.
- Roll clients over to the new plaintext.
- Remove the old entry. Restart.

## Why is a job "Failed" after exactly 5 retries?

Default `MAILIFY_QUEUE__MAX_RETRIES=5`. Apalis counts from zero — the first attempt is attempt 1, and it'll give up after the 5th failed attempt.

## Why does `GET /mail/jobs/:id` return 404 with the UUID I got from my send call?

It shouldn't — as of the current release, `job_id` returned by `/mail/send*` is the apalis ULID, which is what `/mail/jobs/:id` expects. If you stored a `MailJob.id` UUID from a pre-ULID version, that one won't work — apalis indexes by its own ULID.

## Can I use Mailify as a library (not a server)?

Not right now. The lib crates (`mailify-core`, `mailify-templates`, etc.) are workspace-internal — not published to crates.io. Publishing is gated on API-stability review; see [TODO.md §4.1](https://github.com/donilite/mailify/blob/master/TODO.md).

## What happens if Postgres goes away mid-flight?

- **Enqueued jobs** — persisted in Postgres. They resume when Postgres comes back.
- **In-flight jobs** — the worker loses its DB connection; depending on timing, the job is either retried by the next worker poll or marked as stuck in `Running` until another instance reclaims the lock.
- **Requests during the outage** — `/mail/send*` returns 500 because enqueue writes to Postgres.

## Does Mailify store the message body?

Yes — the full `MailJob` (including `kind.Custom.html`) is serialized into the `apalis.jobs` table. If you send PII, treat your Postgres as sensitive. `smtp_override` credentials are **never** stored (skipped at serialization).

## How do I test Mailify in CI without a real SMTP provider?

Use Mailpit as a test double. `docker-compose up postgres mailpit mailify` and point your tests at `http://localhost:8080`. Mailpit has its own JSON API for inspecting received mail, which makes assertions easy. See the integration tests under `crates/mailify-api/tests/` for working examples.

## Can I brand emails per-tenant at render time?

Yes, with a caveat: the `theme` is currently **global** — baked into `AppState` at boot. Per-request theme overrides are not exposed in the public HTTP API today. If you need per-tenant theming, options are:

1. Run one Mailify instance per tenant (cheap in Docker).
2. Pass tenant-specific values via the `vars` JSON blob and render them in your `.tsx` instead of using `{{ theme.* }}`.
3. Contribute a `theme_override` field on `MailJob` — there's a PR opportunity here.

## What's the lock-in story?

Near-zero. The durable state lives in Postgres in a well-known apalis schema. Your templates are `.tsx` files in your repo. Your SMTP provider is yours. If Mailify disappears tomorrow, you can migrate queues with a `pg_dump` and re-render templates through any React Email pipeline.
