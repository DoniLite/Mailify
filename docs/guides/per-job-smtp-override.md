---
title: Per-job SMTP override
description: Drive one Mailify install that fans out to many SMTP providers — one per tenant, request, or environment.
sidebar:
  order: 4
---

# Per-job SMTP override

By default every `MailJob` is dispatched through the **process-wide default sender** built from `MAILIFY_SMTP__*`. That works for the common case: one Mailify install, one upstream SMTP.

For multi-tenant SaaS, you often want each tenant's mail to go through their own SMTP provider, using their own domain's reputation. Mailify supports this natively: attach a `smtp_override` to any send request, and the worker builds a one-off sender for just that job.

## Request shape

```json
POST /mail/send-custom
Authorization: Bearer <token>
Content-Type: application/json

{
  "from": { "address": "billing@tenant-a.com", "name": "Tenant A" },
  "to":   [{ "address": "customer@example.com" }],
  "subject": "Your invoice",
  "html": "<h1>Invoice #1234</h1>",
  "locale": "en",
  "smtp_override": {
    "host": "smtp.sendgrid.net",
    "port": 587,
    "tls": "starttls",
    "username": "apikey",
    "password": "SG.tenant-a-api-key"
  }
}
```

Every field in `smtp_override` mirrors the global `[smtp]` block. Missing fields are **not** inherited from the default — if you pass an override, it must be self-contained.

## Security properties

- **Never logged.** The `Debug` impl on `SmtpOverride` redacts `username` and `password`. The tracing layer never sees them.
- **Never persisted.** Credentials live in the in-memory `MailJob` while it's being processed. They are skipped on serialization (`#[serde(skip_serializing_if = "Option::is_none")]` is combined with an explicit redaction helper used when the job is stored in Postgres).
- **Scoped to one send.** The override sender is constructed, used once, and dropped. There is no credential cache.

## Use cases

### Multi-tenant per-domain reputation

Each tenant's customers receive mail from `mail.<tenant-domain>` with its own DKIM/SPF setup. You store each tenant's SMTP creds in your own DB; when enqueueing a send, look up the tenant and inject `smtp_override`.

### Environment-scoped routing

Route staging mail through Mailpit, production through Resend — without running two Mailify instances. Your backend decides based on env.

### Per-campaign throttling

Some providers apply per-account rate limits. If you spread large sends across multiple provider accounts, each campaign can carry its own credentials.

## When not to use

- **Single-tenant / single-domain.** Just configure `[smtp]` globally and skip overrides.
- **Shared infrastructure.** If tenants share an SMTP provider, no override needed — set the `From:` per request, let the global sender handle the rest.

## Implementation notes

The override is plumbed through:

1. HTTP handler in `crates/mailify-api/src/routes/mail.rs` deserializes `smtp_override` into `SmtpOverride`.
2. It's attached to the `MailJob` and pushed through `QueueHandle::push`.
3. The apalis worker pops the job and calls `SmtpSender::from_override(&override)` to build a dedicated `lettre` transport.
4. The sender is dropped as soon as the send resolves.

If `smtp_override` is `None`, the worker uses the shared `default_sender` from `AppState`.

## Example: per-tenant sender in your backend

```rust
let smtp_override = db
    .get_tenant_smtp(tenant_id)
    .await?
    .map(|creds| SmtpOverride {
        host: creds.host,
        port: creds.port,
        tls: creds.tls,
        username: Some(creds.user),
        password: Some(creds.password),
    });

let body = SendCustomRequest {
    from: tenant.from_addr(),
    to: vec![customer],
    subject,
    html,
    text: None,
    locale: "en".into(),
    priority: Priority::Normal,
    smtp_override,
};

mailify_client.send_custom(&body).await?;
```

Your backend owns the tenant → credentials mapping. Mailify owns the send.
