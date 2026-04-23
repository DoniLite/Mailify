---
title: Configure SMTP
description: Wire Mailify to SES, Mailgun, Brevo, Postmark, Resend, SendGrid, or a local relay.
sidebar:
  order: 1
---

# Configure SMTP

Mailify uses [lettre](https://github.com/lettre/lettre) and speaks stock SMTP. Any provider that gives you credentials for `host:port` + `username:password` over SMTPS/STARTTLS will work.

## The four knobs

| Key | Values | Notes |
|-----|--------|-------|
| `smtp.host` | e.g. `smtp.resend.com` | Provider's SMTP endpoint. |
| `smtp.port` | `25`, `587`, `465`, `1025` | 587 = submission + STARTTLS, 465 = implicit TLS, 1025 = Mailpit local. |
| `smtp.tls` | `none`, `starttls`, `tls` | Match the port. |
| `smtp.username` / `smtp.password` | string / secret | Usually from provider dashboard. |

**Rule of thumb**

- Port 587 → `tls = "starttls"`.
- Port 465 → `tls = "tls"`.
- Port 25 / 1025 → `tls = "none"`.

## Provider recipes

### Resend

```toml
[smtp]
host = "smtp.resend.com"
port = 587
tls = "starttls"
username = "resend"
# password = your Resend API key, from env MAILIFY_SMTP__PASSWORD
default_from_email = "you@yourdomain.com"
```

### Amazon SES

Get SMTP credentials from the SES console (different from your AWS access keys).

```toml
[smtp]
host = "email-smtp.us-east-1.amazonaws.com"
port = 587
tls = "starttls"
username = "AKIA..."
# password from MAILIFY_SMTP__PASSWORD
default_from_email = "no-reply@yourdomain.com"
```

Region suffix matters — pick the endpoint that matches where your SES identity is verified.

### Mailgun

```toml
[smtp]
host = "smtp.mailgun.org"
port = 587
tls = "starttls"
username = "postmaster@mg.yourdomain.com"
# password from MAILIFY_SMTP__PASSWORD (Mailgun SMTP password, not the API key)
```

Use `smtp.eu.mailgun.org` if your domain is in the EU region.

### Brevo (Sendinblue)

```toml
[smtp]
host = "smtp-relay.brevo.com"
port = 587
tls = "starttls"
username = "<your-brevo-login-email>"
# password from MAILIFY_SMTP__PASSWORD (SMTP key, not API key)
```

### Postmark

```toml
[smtp]
host = "smtp.postmarkapp.com"
port = 587
tls = "starttls"
username = "<server-api-token>"
# password = same server-api-token
```

Postmark's SMTP uses the same token as username and password.

### SendGrid

```toml
[smtp]
host = "smtp.sendgrid.net"
port = 587
tls = "starttls"
username = "apikey"    # literal string "apikey"
# password from MAILIFY_SMTP__PASSWORD (SendGrid API key)
```

### Local Mailpit (dev)

```toml
[smtp]
host = "localhost"   # or "mailpit" in docker-compose
port = 1025
tls = "none"
```

No auth. Inspect mail at <http://localhost:8025>.

## Where to put the password

**Do not** commit passwords to `Mailify.toml`. Load them from the environment:

```toml
[smtp]
host = "smtp.resend.com"
port = 587
tls = "starttls"
username = "resend"
# password intentionally omitted → comes from MAILIFY_SMTP__PASSWORD
```

```bash
# .env (gitignored)
MAILIFY_SMTP__PASSWORD=re_live_...
```

Or use a secret manager (Doppler, 1Password CLI, AWS Secrets Manager) and export the env var at process start.

## Test the setup

From the running Mailify:

```bash
curl -s -X POST http://localhost:8080/mail/send-custom \
  -H "authorization: Bearer $TOKEN" \
  -H 'content-type: application/json' \
  -d '{
    "from": { "address": "no-reply@yourdomain.com" },
    "to": [{ "address": "you@yourdomain.com" }],
    "subject": "SMTP test from Mailify",
    "html": "<p>It arrives.</p>",
    "locale": "en"
  }'
```

Then hit `GET /mail/jobs/:id` with the returned ULID until you see `status: "Done"`.

If it fails, [Debugging §5](../troubleshooting/debugging.md#5-test-smtp-connectivity-standalone) shows how to bypass Mailify and test the provider directly with `swaks`.
