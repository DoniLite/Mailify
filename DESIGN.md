# Mailify — Design System

> Single source of truth for brand, visual tokens, typography, and UI patterns used across the site, the docs, the email templates, and any future UI surface (admin dashboard, CLI output styling, OG images).

---

## 1. Brand essence

### 1.1 Name & etymology
**Mailify** = *mail* + *-ify* (the suffix "to make into"). The product takes plain mail and *makes it yours* — branded, themed, shaped to your visual identity. The name is a promise: give us a theme, we give you back mail that looks like you.

### 1.2 Positioning statement
> *Mailify is the self-hosted transactional email server that wears your brand. One Docker image, your colors, your templates, zero vendor lock-in.*

### 1.3 Tone of voice
- **Direct, technical, confident** — we talk to developers, no marketing fluff.
- **Pragmatic over poetic** — "send branded mail in 5 minutes" > "elevate your communication".
- **Self-deprecating OK** — "yes, it's just a wrapper around lettre + axum, but it's a well-dressed one".
- **FR/EN switch** — the site is EN-first for reach. Internal docs + CLAUDE.md + commit messages stay FR when natural.

### 1.4 What we are NOT
- Not a SaaS. No pricing page with tiers. No "enterprise" upsell.
- Not a full marketing suite. No campaigns, no segmentation, no drag-and-drop editor.
- Not trying to replace Postmark / Resend for everyone — trying to be the right tool for devs who want to **own** their mail stack.

---

## 2. Color tokens

All tokens exposed as CSS custom properties on `:root`. Dark mode flips via `[data-theme="dark"]`.

### 2.1 Brand scale

| Token | Light | Dark | Contrast target |
|-------|-------|------|-----------------|
| `--brand-primary` | `#2D5BFF` | `#5B82FF` | WCAG AA on paper + ink |
| `--brand-primary-hover` | `#1E43D9` | `#7A9CFF` | — |
| `--brand-primary-muted` | `#E7EDFF` | `#1A2449` | bg for subtle highlights |
| `--brand-accent` | `#FF8A3D` | `#FFA466` | highlights, never body text |
| `--brand-accent-muted` | `#FFF0E5` | `#3D2414` | bg for callouts |

### 2.2 Neutral scale (ink → paper)

| Token | Light | Dark |
|-------|-------|------|
| `--ink-900` | `#0B1020` | `#F8FAFC` |
| `--ink-700` | `#1F2937` | `#E2E8F0` |
| `--ink-500` | `#475569` | `#94A3B8` |
| `--ink-300` | `#94A3B8` | `#475569` |
| `--ink-100` | `#E2E8F0` | `#1F2937` |
| `--paper` | `#F8FAFC` | `#0B1020` |
| `--paper-raised` | `#FFFFFF` | `#111832` |
| `--border` | `#E2E8F0` | `#1F2937` |

### 2.3 Semantic tokens (role, not color)

| Token | Maps to |
|-------|---------|
| `--text` | `--ink-900` |
| `--text-muted` | `--ink-500` |
| `--link` | `--brand-primary` |
| `--bg` | `--paper` |
| `--bg-raised` | `--paper-raised` |
| `--success` | `#10B981` |
| `--warning` | `#F59E0B` |
| `--danger` | `#EF4444` |
| `--info` | `--brand-primary` |

**Rule:** components always reference *semantic* tokens, never brand/neutral tokens directly. Easier to re-theme.

### 2.4 Email-template palette
The mail templates have their own theme object (`cfg.theme.colors`) injected at render time. The **default** theme ships with these same brand values, so out-of-the-box a Mailify install sends mails in Mailify's own brand. Users override per install.

---

## 3. Typography

### 3.1 Typefaces

| Role | Family | Weights | Fallback |
|------|--------|---------|----------|
| Display | **Geist Sans** | 500, 600, 700 | Inter, -apple-system, system-ui |
| Body | **Inter** | 400, 500 | -apple-system, Segoe UI, Roboto |
| Mono | **Geist Mono** | 400, 500 | JetBrains Mono, Menlo, Consolas, monospace |

Self-host via `@fontsource/*` packages (no Google Fonts request → better SEO + privacy + Lighthouse).

### 3.2 Scale (modular, ratio 1.250 "major third")

| Token | Size | Line-height | Use |
|-------|------|-------------|-----|
| `--fs-xs` | 0.75rem / 12px | 1.5 | captions, labels |
| `--fs-sm` | 0.875rem / 14px | 1.5 | body small, UI controls |
| `--fs-base` | 1rem / 16px | 1.65 | body default |
| `--fs-md` | 1.125rem / 18px | 1.6 | lede paragraphs |
| `--fs-lg` | 1.5rem / 24px | 1.3 | h3, section intros |
| `--fs-xl` | 1.875rem / 30px | 1.25 | h2 |
| `--fs-2xl` | 2.5rem / 40px | 1.15 | h1 docs |
| `--fs-3xl` | 3.5rem / 56px | 1.05 | hero landing |
| `--fs-4xl` | 4.5rem / 72px | 1 | mega hero (desktop only) |

### 3.3 Headings
- `h1` hero landing: `--fs-3xl` Geist Sans 700, tracking `-0.03em`.
- `h1` doc page: `--fs-2xl` Geist Sans 600, tracking `-0.02em`.
- `h2`..`h4` docs: Geist Sans 600, tracking `-0.01em`.
- Body: Inter 400, no tracking adjustment.
- **Always semibold, never bold** for UI surfaces (Geist is already heavy at 600).

### 3.4 Mono rules
- Inline code: `--fs-[0.925em]` relative to parent + `--brand-primary-muted` background + 4px horizontal padding + 4px radius.
- Code blocks: `--fs-sm`, full width, no inline background on tokens, `--paper-raised` bg, border `1px --border`, radius `--radius-md`.

---

## 4. Space & layout

### 4.1 Spacing scale (4px base)

| Token | Value |
|-------|-------|
| `--space-1` | 4px |
| `--space-2` | 8px |
| `--space-3` | 12px |
| `--space-4` | 16px |
| `--space-6` | 24px |
| `--space-8` | 32px |
| `--space-12` | 48px |
| `--space-16` | 64px |
| `--space-24` | 96px |
| `--space-32` | 128px |

### 4.2 Radii

| Token | Value | Use |
|-------|-------|-----|
| `--radius-sm` | 4px | inline code, badges |
| `--radius-md` | 8px | buttons, inputs, cards |
| `--radius-lg` | 12px | larger cards, modals |
| `--radius-full` | 9999px | pills, avatars |

### 4.3 Elevation (shadows)

| Token | Value |
|-------|-------|
| `--shadow-sm` | `0 1px 2px rgba(11,16,32,0.06)` |
| `--shadow-md` | `0 4px 12px rgba(11,16,32,0.08)` |
| `--shadow-lg` | `0 12px 32px rgba(11,16,32,0.12)` |

Dark mode: reduce alpha by half and switch to `rgba(0,0,0,...)`.

### 4.4 Container widths

| Token | Value | Use |
|-------|-------|-----|
| `--container-prose` | 720px | doc article max-width |
| `--container-main` | 1120px | landing sections |
| `--container-wide` | 1280px | full-width marketing blocks |

### 4.5 Breakpoints

| Token | Value |
|-------|-------|
| `--bp-sm` | 640px |
| `--bp-md` | 768px |
| `--bp-lg` | 1024px |
| `--bp-xl` | 1280px |

Mobile-first everywhere.

---

## 5. Components — visual rules

### 5.1 Button

| Variant | Bg | Text | Border | Hover |
|---------|-----|------|--------|-------|
| primary | `--brand-primary` | white | none | `--brand-primary-hover` |
| secondary | `--paper-raised` | `--text` | `1px --border` | `--ink-100` bg |
| ghost | transparent | `--link` | none | `--brand-primary-muted` bg |
| danger | `--danger` | white | none | darker red |

- Padding: `--space-3 --space-6`.
- Radius: `--radius-md`.
- Font: Geist Sans 500, `--fs-sm`.
- Focus ring: `2px solid --brand-primary` + `2px` offset.
- No shadow by default on buttons — keep them flat.

### 5.2 Code block
- Header row (optional): filename left, language badge right, copy button far right.
- Line numbers optional, off by default (on for install/config snippets > 5 lines).
- Syntax theme: **Shiki** with dual-theme support — `github-light` / `github-dark`, matched to our `[data-theme]`.
- Copy button on hover top-right, icon `clipboard` from Lucide.

### 5.3 Callouts / admonitions (for docs)
Four variants, all share left border 3px + tinted bg + icon:

| Kind | Border | Bg | Icon |
|------|--------|-----|------|
| note | `--brand-primary` | `--brand-primary-muted` | info circle |
| tip | `--success` | green-muted | lightbulb |
| warning | `--warning` | amber-muted | triangle |
| danger | `--danger` | red-muted | octagon |

### 5.4 Cards (features on landing)
- `--paper-raised` bg, `1px --border`, `--radius-lg`, padding `--space-6`.
- No shadow at rest; `--shadow-md` on hover with `translateY(-2px)` transition 150ms ease-out.
- Icon top-left, 32px, colored `--brand-primary`.
- Title Geist Sans 600 `--fs-md`, body Inter 400 `--fs-sm` `--text-muted`.

### 5.5 Navigation
- Top bar 64px, sticky, `--paper` bg with blur `backdrop-filter: blur(12px)` and 80% alpha when scrolled.
- Logo left (mark + wordmark), nav links center (Docs, GitHub, Sponsors), CTA right (primary button "Get started").
- Sidebar docs: 280px wide, sticky, collapsible groups. Active item = left border 2px `--brand-primary` + `--brand-primary-muted` bg.

### 5.6 Footer
- 3 columns on desktop, stacks on mobile.
- Columns: *Product* (Docs, Changelog, Roadmap), *Project* (GitHub, Sponsors, Discussions), *Community* (Twitter/X, RSS).
- Bottom row: logo + copyright + "made in Rust" badge + theme toggle.

---

## 6. Iconography

- **Lucide Icons** (open source, tree-shakeable, matches our geometric feel).
- Stroke width: 1.75 default, 2 for small sizes.
- Size tokens: `--icon-sm` 16px, `--icon-md` 20px, `--icon-lg` 24px, `--icon-xl` 32px.
- Never fill icons — always stroke.
- Use sparingly in body text; more generously in UI chrome.

---

## 7. Imagery & illustration

### 7.1 Landing illustrations
- Abstract, geometric, flat vector — same language as the logo.
- Palette restricted to brand tokens; no external colors.
- No stock photography, no 3D, no AI-gen "corporate glow".
- One signature illustration on the hero; smaller spots on feature sections.

### 7.2 OG / social cards
- 1200×630, generated per page via Astro's `@vercel/og`-equivalent (`satori` + `astro-og-canvas` or similar).
- Template: dark `--ink-900` bg, big wordmark top-left, page title center-left Geist 600 72px white, subtitle `--ink-300` 32px below, accent diagonal stripe `--brand-primary` bottom-right.
- One shared template, dynamic title from page frontmatter.

### 7.3 Architecture diagrams
- **Excalidraw** hand-drawn style OR clean SVG with brand palette — pick one and stay consistent.
- Export as SVG, never PNG, so they scale and theme-invert cleanly.

---

## 8. Motion

- **Default transition:** `150ms ease-out` for color/opacity/transform.
- **Page transitions:** none (SSG, hard reloads are fine).
- **View transitions API** enabled via Astro for same-origin nav (smooth fade between docs pages).
- **Scroll reveal:** subtle `opacity 0 → 1` + `translateY(12px → 0)` on landing feature cards, via `Intersection Observer`, once per element.
- **Respect `prefers-reduced-motion`** — disable all non-essential motion.
- **No auto-playing anything.** No carousels that move on their own.

---

## 9. Accessibility

Non-negotiable baseline:

- **WCAG 2.1 AA** contrast on all text (verified via Lighthouse CI).
- **Keyboard nav** — every interactive element reachable via Tab, visible focus ring always.
- **Skip-to-content** link on every page (first Tab stop).
- **Semantic HTML** — `<nav>`, `<main>`, `<article>`, `<aside>`, heading hierarchy never skipped.
- **Alt text** required on all images; decorative images get `alt=""`.
- **Dark mode preference** respected via `prefers-color-scheme`, overridable via manual toggle stored in `localStorage`.
- **Focus management** on route change (focus the `<h1>`).
- **Form labels** always explicit, never placeholder-only.

---

## 10. Responsive behavior

| Breakpoint | Layout adjustments |
|-----------|--------------------|
| `< 640px` | Single column everywhere. Nav collapses to hamburger. Hero type drops to `--fs-2xl`. Code blocks horizontal-scroll. |
| `640–1023px` | 2-column feature grids. Sidebar docs goes to drawer. |
| `≥ 1024px` | Full desktop: 3-column footer, sticky sidebar + TOC on docs, `--fs-3xl` or `--fs-4xl` hero. |

Touch targets ≥ 44×44px on mobile. No hover-only interactions.

---

## 11. Applied examples

### 11.1 Landing hero (structure)
```
[Top nav —————————————————————————————————— CTA]

  Eyebrow badge: "v0.2 — now on crates.io"          (Geist 500 fs-xs, --brand-primary)

  # The mail server that                            (h1, fs-3xl, --text)
    wears your brand.
  
  Self-hosted, theme-aware transactional mail.      (fs-md, --text-muted, max 620px)
  One Docker image. Your colors. Zero lock-in.
  
  [ Get started →  ]  [ docker pull donighost/mailify ]
  (primary button)    (mono pill, --paper-raised bg, copy-on-click)

  — illustration: envelope morphing into color swipe, right side, 1/3 width —
```

### 11.2 Docs page (structure)
```
[Top nav                                                          ]
[Sidebar —————————— | Article prose max 720px | ————— TOC (sticky)]
                    |                          |
                    |  # Page title           |
                    |  meta: last updated     |
                    |                         |
                    |  Body with code blocks, |
                    |  callouts, links.       |
                    |                         |
                    |  "Edit this page"       |
                    |  + prev/next links      |
[Footer                                                           ]
```

---

## 12. Asset inventory (to produce)

- [ ] Logo mark SVG (light + dark) — prompt in `TODO.md §0.3`
- [ ] Logo wordmark SVG
- [ ] Logo lockup (mark + wordmark horizontal)
- [ ] Favicon (SVG + 32×32 PNG fallback)
- [ ] Apple touch icon 180×180
- [ ] OG default 1200×630
- [ ] Hero illustration (envelope → palette)
- [ ] 3 feature-card spot illustrations
- [ ] Architecture SVG (7 crates, data flow)
- [ ] Screenshot: CLI output of `mailify config print`
- [ ] Screenshot: an example branded email rendered

---

## 13. Design principles (decision heuristics)

When in doubt, apply these in order:

1. **Remove before you add.** Every element must justify its pixels.
2. **One primary action per screen.** If there are two, one is secondary.
3. **Semantic over aesthetic.** A component named `<Card>` is a card because of its role, not because of its shadow.
4. **Brand expresses through restraint.** The orange accent is powerful *because* it's rare. Don't spray it.
5. **Mobile is not a fallback.** Start there; desktop adds breathing room, not new content.
6. **Performance is a design concern.** A 200KB JS bundle is a design failure, not a dev one.
7. **Accessibility is not an extra pass.** It's the baseline; you don't ship without it.

---

## 15. Tailwind v4 integration

The marketing site (`site/`) uses **Tailwind v4** wired through `@tailwindcss/vite`. There is no `tailwind.config.{js,ts}` — Tailwind v4 is **CSS-first**: tokens declared in `@theme {}` inside [`site/src/styles/tokens.css`](site/src/styles/tokens.css) automatically become utilities (`bg-brand-primary`, `text-ink-700`, `rounded-lg`, …).

### 15.1 Wiring

- `astro.config.mjs` → `vite: { plugins: [tailwindcss()] }`.
- `site/src/styles/tokens.css` is the single source of truth:
  1. **Tailwind imports are split, NOT `@import "tailwindcss"`** :
     ```css
     @import 'tailwindcss/theme.css'     layer(theme);
     @import 'tailwindcss/utilities.css' layer(utilities);
     ```
     Preflight is **intentionally omitted**. The full `@import "tailwindcss"` pulls a global CSS reset that resets `h1`/`p`/`ul` margins and list markers — that mangles Starlight's docs typography (heading sizes collapse, lists lose bullets, links default-color). Starlight ships its own scoped reset; ours adds nothing. If a marketing component needs a real reset, scope it manually with `.reset { all: revert; }` etc.
  2. `@variant dark (&:where([data-theme='dark'], [data-theme='dark'] *))` rebinds Tailwind's built-in `dark:` variant to our `[data-theme]` attribute (instead of `prefers-color-scheme`), so it stays in sync with `ThemeToggle.astro` + Starlight's theme provider.
  3. `@theme { … }` declares **every** token Tailwind should expose as utilities.
  4. `:root[data-theme='dark']` overrides the same `--color-*` vars for dark mode — utilities and raw `var(--color-…)` references both flip automatically.
  5. Legacy aliases (`--brand-primary`, `--space-4`, …) are kept as `var(--color-brand-primary)` etc. so older components written before Tailwind landed keep working without a rewrite.

### 15.2 Token → utility mapping

Tailwind v4 generates utilities from the prefix of each `--*-name` declared in `@theme`. The naming convention is **enforced by Tailwind**, so it's worth memorizing.

| `@theme` prefix | Utilities generated | Example |
|----------------|---------------------|---------|
| `--color-*` | `bg-*`, `text-*`, `border-*`, `ring-*`, `from-*`, `to-*`, `via-*`, `outline-*`, `decoration-*`, `accent-*`, `caret-*`, `divide-*`, `placeholder-*` | `bg-brand-primary`, `text-ink-700` |
| `--font-*` | `font-*` | `font-display`, `font-mono` |
| `--text-*` | `text-*` (typography scale) | `text-md`, `text-3xl` |
| `--spacing` (single) | `p-N`, `m-N`, `gap-N`, `w-N`, `h-N`, etc. — `N × spacing` | `p-4` = `4 × 4px` = `16px` |
| `--radius-*` | `rounded-*` | `rounded-md`, `rounded-full` |
| `--shadow-*` | `shadow-*` | `shadow-md`, `shadow-lg` |
| `--container-*` | `max-w-*`, `mx-auto` containers | `max-w-prose`, `max-w-main` |
| `--breakpoint-*` | responsive variants | `sm:`, `md:`, `lg:`, `xl:` |

### 15.3 When to use utility vs CSS var

- **Tailwind utility** — for layout, spacing, typography, color in JSX-style markup. Reads cleanly inline. Default choice for new components.
- **Raw `var(--color-…)`** — for SVG fills/strokes, CSS-in-JS dynamic values, gradients with `color-mix()`, anywhere a utility doesn't exist or you need the raw value. The two systems share the same `--color-*` vars, so they stay in sync.
- **Scoped `<style>`** — for complex, single-use styles (the hero's orb glows, the architecture diagram). Compose with Tailwind, don't replace it.

### 15.4 Component pattern (recommended)

```astro
---
// Hero.astro — utility-first with raw vars only where utilities can't reach.
---
<section class="bg-paper-raised dark:bg-paper">
  <div class="mx-auto grid max-w-wide gap-12 px-4 py-24 lg:grid-cols-[1.1fr_minmax(22rem,30rem)]">
    <div>
      <p class="mb-6 inline-flex rounded-full bg-brand-primary-muted px-3 py-1 text-xs font-semibold uppercase tracking-wide text-brand-primary">
        Self-hosted · Theme-aware · Rust-fast
      </p>
      <h1 class="text-3xl font-semibold leading-tight tracking-tighter text-ink-900 lg:text-4xl">
        The mail server that wears your brand.
      </h1>
      <p class="mt-6 max-w-xl text-md text-ink-500">
        Self-hosted transactional mail with your colors, your templates, and your SMTP provider.
      </p>
    </div>

    {/* SVG strokes use raw vars so they flip with [data-theme] */}
    <svg viewBox="0 0 64 64" class="size-32" aria-hidden="true">
      <path d="M..." stroke="var(--color-brand-primary)" stroke-width="4" fill="none"/>
    </svg>
  </div>
</section>
```

### 15.5 Dark mode

Already wired. Use `dark:` variant on any utility:

```astro
<button class="bg-brand-primary text-paper-raised dark:bg-brand-primary dark:text-ink-900">
  CTA
</button>
```

The `--color-*` vars are redefined under `[data-theme='dark']` in `tokens.css`, so `bg-brand-primary` already flips. Use `dark:` only when you need a *different token* in dark mode (rare with our system — most of the time the auto-flip is enough).

### 15.6 What NOT to do

- ❌ Don't add a `tailwind.config.js`. Tailwind v4 is CSS-first; that file is legacy.
- ❌ Don't switch to `@import "tailwindcss"` — it pulls preflight, which destroys Starlight's docs typography (h1/p/ul margins, list markers, link colors all reset). Keep the split imports.
- ❌ Don't redeclare tokens elsewhere (in component `<style>` blocks, in other CSS files). One source of truth — `tokens.css`.
- ❌ Don't use `class:list={...}` to compose colors at runtime — Tailwind needs to see literal class names at build time. Use full conditional ternaries with literal classes instead.
- ❌ Don't bypass `@variant dark (...)` and use Tailwind's default `prefers-color-scheme` dark mode — it would desync from Starlight + the ThemeToggle.

### 15.7 Migrating existing components

Existing components in `site/src/components/*.astro` use scoped `<style>` blocks with raw CSS vars. They keep working because of the legacy aliases in §15.1.5. Migrate incrementally — pick a component, replace its `<style>` block with Tailwind utilities, delete the legacy `<style>`.

Suggested migration order (simplest → most painful):
1. `404.astro`, `pricing.astro` — small, mostly text.
2. `FeatureGrid.astro`, `CodeTabs.astro` — clear card/tab patterns.
3. `SiteFooter.astro`, `SiteHeader.astro` — chrome.
4. `Hero.astro`, `ThemeShowcase.astro`, `ArchitectureDiagram.astro` — keep their bespoke `<style>` for the SVG/animation parts; convert layout chrome only.

---

## 14. Versioning this document

- Bump `## N. …` section number only when adding new sections (never renumber).
- Changes to tokens (colors, sizes) = note in `CHANGELOG.md` under `### Design`.
- Major palette/typography changes = bump to a new major doc version in a `DESIGN-v2.md` and link back, so historical context is preserved.

**Current version:** `v2` — 2026-04-26 — Tailwind v4 integration via `@theme` + `@variant dark`.

### Changelog
- `v2` (2026-04-26) — Added §15: Tailwind v4 wired through `@tailwindcss/vite`. Tokens exposed via `@theme {}` in `tokens.css`. Dark variant rebound to `[data-theme]` attribute. Legacy `--brand-*` / `--space-*` aliases preserved.
- `v1` (2026-04-23) — Initial system.
