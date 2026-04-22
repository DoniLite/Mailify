/**
 * Post-process `email export` output:
 *   1. Decode HTML entities inside minijinja spans ({{ ... }} / {% ... %}) so the Rust
 *      renderer sees literal quotes/operators rather than &#x27; / &amp; etc.
 *   2. Copy subject/text sidecars from emails/<id>/ alongside compiled HTML.
 */
import { cpSync, existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const ROOT = resolve(import.meta.dirname, "..");
const EMAILS = join(ROOT, "emails");
const OUT = join(ROOT, "out");

if (!existsSync(OUT)) {
  throw new Error(`out/ not found — run 'bun run build' first`);
}

const ENTITY_MAP: Record<string, string> = {
  "&amp;": "&",
  "&lt;": "<",
  "&gt;": ">",
  "&quot;": '"',
  "&apos;": "'",
  "&#x27;": "'",
  "&#39;": "'",
  "&#34;": '"',
  "&nbsp;": " ",
};

function decodeEntities(s: string): string {
  return s
    .replace(/&#x([0-9a-fA-F]+);/g, (_, hex) => String.fromCodePoint(parseInt(hex, 16)))
    .replace(/&#(\d+);/g, (_, dec) => String.fromCodePoint(parseInt(dec, 10)))
    .replace(/&(amp|lt|gt|quot|apos|nbsp);/g, (m) => ENTITY_MAP[m] ?? m);
}

function decodeMinijinjaSpans(html: string): string {
  // {{ ... }} and {% ... %}
  return html.replace(/\{\{([^}]+)\}\}|\{%([^%]+)%\}/g, (match, expr, stmt) => {
    if (expr !== undefined) return `{{${decodeEntities(expr)}}}`;
    return `{%${decodeEntities(stmt)}%}`;
  });
}

function walkHtml(dir: string, fn: (path: string) => void) {
  for (const entry of readdirSync(dir)) {
    const p = join(dir, entry);
    const st = statSync(p);
    if (st.isDirectory()) walkHtml(p, fn);
    else if (p.endsWith(".html")) fn(p);
  }
}

let rewrote = 0;
walkHtml(OUT, (p) => {
  const original = readFileSync(p, "utf8");
  const patched = decodeMinijinjaSpans(original);
  if (patched !== original) {
    writeFileSync(p, patched);
    rewrote++;
  }
});

let copied = 0;
for (const entry of readdirSync(EMAILS)) {
  const src = join(EMAILS, entry);
  if (!statSync(src).isDirectory()) continue;
  if (entry.startsWith("_")) continue;

  const dstDir = join(OUT, entry);
  if (!existsSync(dstDir)) continue;

  for (const file of readdirSync(src)) {
    if (file.startsWith("subject.") || file.startsWith("text.")) {
      cpSync(join(src, file), join(dstDir, file));
      copied++;
    }
  }
}

console.log(`Rewrote minijinja spans in ${rewrote} html files.`);
console.log(`Copied ${copied} sidecar files into out/.`);
