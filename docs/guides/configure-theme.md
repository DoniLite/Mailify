---
title: Configure theme
description: Override Mailify's branding tokens — colors, fonts, logo, footer — via a single config block.
sidebar:
  order: 2
---

# Configure theme

The **theme** is Mailify's headline feature. Every template references `{{ theme.* }}` at render time, so changing the theme object re-brands every outgoing mail without editing a single template.

## What's in the theme

See the [full config reference](../reference/config.md#theme) for the authoritative list. At a glance:

```toml
[theme]
brand_name     = "Acme"
brand_logo_url = "https://cdn.acme.com/logo.svg"
radius         = "12px"
footer_text    = "© Acme Corp · 100 Main St · San Francisco"

[theme.colors]
primary             = "#2D5BFF"
primary_foreground  = "#ffffff"
secondary           = "#1F2937"
secondary_foreground = "#F8FAFC"
background          = "#F8FAFC"
foreground          = "#0B1020"
muted               = "#94A3B8"
border              = "#E2E8F0"
danger              = "#EF4444"
success             = "#10B981"

[theme.fonts]
body    = "Inter, -apple-system, sans-serif"
heading = "Geist Sans, Inter, sans-serif"

[theme.social_links]
twitter  = "https://x.com/acme"
linkedin = "https://linkedin.com/company/acme"
github   = "https://github.com/acme"

[theme.extra]
support_email = "support@acme.com"
docs_url      = "https://docs.acme.com"
```

## How templates consume it

Inside a React Email component (`.tsx`):

```tsx
import { Button, Container, Text } from "@react-email/components";

export default function Welcome() {
  return (
    <Container
      style={{
        backgroundColor: "{{ theme.colors.background }}",
        color: "{{ theme.colors.foreground }}",
        fontFamily: "{{ theme.fonts.body }}",
      }}
    >
      <Text>Hello {{ vars.first_name }},</Text>
      <Button
        href="{{ vars.cta_url }}"
        style={{
          backgroundColor: "{{ theme.colors.primary }}",
          color: "{{ theme.colors.primary_foreground }}",
          borderRadius: "{{ theme.radius }}",
        }}
      >
        {{ theme.brand_name }} →
      </Button>
      <Text style={{ color: "{{ theme.colors.muted }}" }}>
        {{ theme.footer_text }}
      </Text>
    </Container>
  );
}
```

The `post-build.ts` step decodes the HTML-entity-escaped `{{ }}` spans so minijinja can resolve them at send time. See [Template contract](../reference/template-contract.md) for the full pipeline.

## The `extra` bag

`theme.extra` is an arbitrary `HashMap<String, String>` for tenant-specific or template-specific values that don't fit elsewhere:

```toml
[theme.extra]
help_center_url = "https://help.acme.com"
app_store_badge = "https://cdn.acme.com/badges/app-store.svg"
```

```tsx
<Link href="{{ theme.extra.help_center_url }}">Need help?</Link>
```

This beats encoding everything in `vars` when the value is a constant per install (not per email).

## Re-theming in production

Two paths:

1. **Edit config + restart.** Cheapest and most correct. The theme is read once at boot and cloned into `AppState`. A restart takes seconds with Docker.
2. **Hot-reload config.** Not implemented. If you want it, file an issue — there's a reasonable PR here that polls the TOML file for changes.

## Branding multiple tenants from one install

Today the theme is **global per process**. To run multi-tenant with distinct branding:

- Easiest: one Mailify instance per tenant. Cheap in Docker — ~20 MB per container.
- Or: pass tenant-specific values in `vars` and reference `vars.*` instead of `theme.*` in your templates. Lose the "re-theme via config" ergonomics, gain per-request flexibility.
- Or: contribute a `theme_override` field on `MailJob` (same pattern as `smtp_override`).

## Preview the theme locally

Pair `MAILIFY_THEME__*` overrides with `make dev-templates` to iterate:

```bash
cd templates-parser
MAILIFY_THEME__COLORS__PRIMARY=#ff4500 bun run dev
```

The React Email dev server runs on `:3000` and hot-reloads as you edit `.tsx`. Themes are hard-coded in the dev preview by default — you'll want to wire your `.tsx` components to read theme values from a shared constants file and swap it out during build.
