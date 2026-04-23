---
title: Template contract
description: The directory layout, minijinja-in-HTML convention, and build pipeline Mailify's TemplateRegistry expects.
sidebar:
  order: 4
---

# Template contract

Mailify's `TemplateRegistry` is a flat, file-based store. It does not parse React Email at runtime — the `.tsx` compilation happens ahead of time, and the Rust side just reads prebuilt HTML + sidecar files.

## Directory layout

At the path pointed to by `MAILIFY_TEMPLATES__PATH` (default `./templates-parser/out`):

```
<template_id>/
  <locale>.html              # required — pre-rendered React Email HTML
  subject.<locale>.txt       # optional — minijinja-rendered at send time
  text.<locale>.txt          # optional — plaintext alternative
```

Plus a `catalog.json` at the root listing every `(id, locale)` pair for registry boot.

Example:

```
templates-parser/out/
├── catalog.json
├── welcome/
│   ├── en.html
│   ├── fr.html
│   ├── subject.en.txt
│   ├── subject.fr.txt
│   ├── text.en.txt
│   └── text.fr.txt
└── password-reset/
    ├── en.html
    └── subject.en.txt
```

## Minijinja in HTML

React Email HTML-encodes `{{ }}` and `{% %}` during export. Mailify's `post-build.ts` step **entity-decodes** those spans so the server-side minijinja can parse them at render time.

This means:

- Variables: `{{ vars.first_name }}`, `{{ theme.brand_name }}`, `{{ theme.colors.primary }}`.
- Control flow: `{% if vars.admin %}…{% endif %}`, `{% for item in vars.items %}…{% endfor %}`.
- Built-in filters: `{{ vars.email | escape }}`, etc.

If you skip `post-build.ts`, your templates will ship with literal `&#123;&#123; vars.foo &#125;&#125;` and never resolve.

## `RenderContext`

Every render receives:

- `theme` — the full `Theme` config object, with colors, fonts, logo URL, social links, and `extra` bag.
- `vars` — the caller-supplied JSON blob from the send request.
- `locale` — the resolved locale (after fallback chain).

Example minijinja snippet inside a React Email component:

```tsx
<Text style={{ color: "{{ theme.colors.primary }}" }}>
  Hi {{ vars.first_name | default("there") }},
</Text>
```

## Strict mode

If `templates.strict = true`, Mailify fails startup whenever any **built-in** template id is missing for the default locale. This catches "I renamed a template but forgot to rebuild" before it reaches a user.

What counts as "built-in" is defined in `mailify-templates`'s registry code — custom user templates are always optional.

## Build pipeline

```
templates-parser/scripts/templates.config.ts      (source of truth — ids + metadata)
        ↓  make gen
templates-parser/emails/<id>.tsx                   (React Email components)
        ↓  make build-templates  →  email export
templates-parser/out/<id>/<locale>.html            (HTML-encoded placeholders)
        ↓  post-build.ts
templates-parser/out/<id>/<locale>.html            (decoded, + subject/text sidecars)
        ↓  build ./target/release/mailify
./target/release/mailify                            (reads out/ at boot)
```

### Adding a new template

1. Add its entry to `templates-parser/scripts/templates.config.ts`:

   ```ts
   {
     id: "invoice-reminder",
     subject: { en: "Payment due: {{ vars.invoice_number }}", fr: "..." },
     locales: ["en", "fr"],
   }
   ```

2. Run `make gen` — generates a `.tsx` scaffold in `templates-parser/emails/` and sidecar files.
3. Fill in the React Email component.
4. `make build-templates` — regenerates `out/` with your new template.
5. Restart Mailify (or `docker compose up -d --force-recreate mailify`).

## Serving precompiled templates from a different location

For production installs where you don't want to ship `node_modules` + bun, pre-build elsewhere and point Mailify at the output directory:

```bash
# on your build machine or in CI:
cd templates-parser && bun install && bun run build

# ship templates-parser/out to the server, then:
export MAILIFY_TEMPLATES__PATH=/opt/mailify/templates
mailify
```

The universal install script ([installation](../getting-started/installation.md#2-universal-install-script)) extracts this bundle into `~/.local/share/mailify/templates` automatically.
