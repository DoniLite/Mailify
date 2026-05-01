/**
 * Pre-build step: rasterize public/og-default.svg → public/og-default.png at 1200×630.
 *
 * Twitter, LinkedIn, Slack, and most messaging clients prefer raster OG images
 * — SVG OG cards either get downscaled or rejected outright.
 *
 * Run via `bun run prebuild` (called automatically before `astro build`).
 */

import sharp from 'sharp';
import { copyFile, readFile, writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';

const here = dirname(fileURLToPath(import.meta.url));
const publicDir = resolve(here, '../public');

// 1. Rasterize OG image
const svgPath = resolve(publicDir, 'og-default.svg');
const pngPath = resolve(publicDir, 'og-default.png');
const svg = await readFile(svgPath);
const png = await sharp(svg, { density: 192 })
  .resize(1200, 630, { fit: 'contain', background: '#0B1020' })
  .png({ compressionLevel: 9 })
  .toBuffer();
await writeFile(pngPath, png);
console.log(`✓ Wrote ${pngPath} (${(png.byteLength / 1024).toFixed(1)} KB)`);

// 2. Copy universal install scripts into public/ so they're served at the domain root.
//    Source of truth is `install/` at the repo root.
const installSrc = resolve(here, '../../install');
for (const name of ['install.sh', 'install.ps1']) {
  await copyFile(resolve(installSrc, name), resolve(publicDir, name));
  console.log(`✓ Copied ${name} → public/${name}`);
}
