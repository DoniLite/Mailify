/**
 * Generate per-locale email .tsx files + subject/text sidecars from TEMPLATES spec.
 *
 * Output layout (what the Rust registry expects):
 *   emails/<id>/<locale>.tsx         → React Email source (Bun/email-build → .html)
 *   emails/<id>/subject.<locale>.txt → minijinja subject template
 *   emails/<id>/text.<locale>.txt    → minijinja plaintext alternative
 */
import { mkdirSync, writeFileSync, existsSync } from "node:fs";
import { join, resolve } from "node:path";
import { TEMPLATES, type BlockSpec, type LocaleContent } from "./templates.config.ts";

const ROOT = resolve(import.meta.dirname, "..");
const EMAILS_DIR = join(ROOT, "emails");

function renderBlock(block: BlockSpec): string {
  if (block.p) {
    return `      <Text className="m-0 mb-4 text-base text-foreground">${escape(block.p)}</Text>`;
  }
  if (block.muted) {
    return `      <Text className="m-0 mb-4 text-sm text-muted">${escape(block.muted)}</Text>`;
  }
  if (block.h2) {
    return `      <H2>${escape(block.h2)}</H2>`;
  }
  if (block.button) {
    return `      <Section className="my-6 text-center"><Button href={"{{ ${block.button.hrefVar} }}"}>${escape(block.button.label)}</Button></Section>`;
  }
  if (block.code) {
    return `      <Section className="my-6 rounded-card bg-secondary p-4 text-center"><Text className="m-0 font-mono text-2xl font-bold tracking-widest text-secondaryFg">${escape(block.code)}</Text></Section>`;
  }
  if (block.kv) {
    const rows = block.kv
      .map(
        (row) =>
          `        <Row><Column className="py-1 text-sm text-muted">${escape(
            row.label
          )}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ ${row.valueExpr} }}"}</Column></Row>`
      )
      .join("\n");
    return `      <Section className="my-4 border border-solid border-border rounded-card p-4">\n${rows}\n      </Section>`;
  }
  return "";
}

function escape(s: string): string {
  // Keep minijinja {{ }} literal by wrapping in JSX expression where needed.
  // For plain paragraphs, embed via template string literal.
  return `{\`${s.replace(/`/g, "\\`")}\`}`;
}

function renderComponent(id: string, locale: string, c: LocaleContent): string {
  const needsRow = c.blocks.some((b) => b.kv);
  const imports = [
    `import * as React from "react";`,
    `import { Section${needsRow ? ", Row, Column" : ""}, Text } from "@react-email/components";`,
    `import { Layout } from "../_components/Layout";`,
    `import { Button } from "../_components/Button";`,
    `import { H1, H2 } from "../_components/Heading";`,
  ].join("\n");

  const body = c.blocks.map(renderBlock).filter(Boolean).join("\n");

  return `${imports}

export default function ${toPascal(id)}_${locale}() {
  return (
    <Layout preview={\`${c.preview}\`} locale="${locale}">
      <H1>{\`${c.heading}\`}</H1>
${body}
    </Layout>
  );
}
`;
}

function toPascal(id: string): string {
  return id
    .split(/[-_]/)
    .map((p) => p.charAt(0).toUpperCase() + p.slice(1))
    .join("");
}

let wrote = 0;
for (const tpl of TEMPLATES) {
  const dir = join(EMAILS_DIR, tpl.id);
  mkdirSync(dir, { recursive: true });
  for (const [locale, content] of Object.entries(tpl.locales)) {
    writeFileSync(join(dir, `${locale}.tsx`), renderComponent(tpl.id, locale, content));
    writeFileSync(join(dir, `subject.${locale}.txt`), content.subject + "\n");
    writeFileSync(join(dir, `text.${locale}.txt`), content.text + "\n");
    wrote += 3;
  }
}

const catalog = TEMPLATES.map((t) => ({
  id: t.id,
  category: t.category,
  locales: Object.keys(t.locales),
}));
writeFileSync(join(EMAILS_DIR, "catalog.json"), JSON.stringify(catalog, null, 2) + "\n");

console.log(`Generated ${wrote} files across ${TEMPLATES.length} templates.`);
console.log(`Catalog: ${join(EMAILS_DIR, "catalog.json")}`);
if (!existsSync(EMAILS_DIR)) {
  throw new Error("emails dir missing after generation");
}
