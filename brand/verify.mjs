#!/usr/bin/env node
// Checks a branded tree against the original one it came from.
//
//   node brand/verify.mjs buglenz --branded /tmp/branded [--original .]
//
// What it guards (ADR-0006, OpenSpec 007):
//   1. zone C is untouched: identifiers the upstream's code and its users depend
//      on (RUSTRAK_* variables, rustrak_* metrics, @rustrak/* packages, the
//      rustrak:* storage keys, the X-Rustrak-* webhook headers, the exported
//      RustrakError/RustrakClient/RustrakWordmark names) occur exactly as many
//      times in the branded tree as in the original;
//      (files an override replaces whole are exempt from the count but must keep
//      their exported names);
//   2. catalogs keep every key (the overlay adds keys, never removes or renames);
//   3. no "Rustrak" is left in a catalog value except the attribution allowed in
//      brand/<name>/verify.json;
//   4. the attribution is actually there: the About page reads it, and the English
//      text names Rustrak and the licence.
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { listFiles, parseArgs } from './lib.mjs';

const ZONE_C = [
  'RUSTRAK_',
  'rustrak_',
  '@rustrak/',
  'rustrak:',
  'X-Rustrak-',
  'RustrakError',
  'RustrakClient',
  'RustrakWordmark',
  'rustrak-wordmark',
  'rustrak_session',
];
const TEXT = /\.(rs|ts|tsx|mjs|js|json|toml|yml|yaml|html|css|md|sql|sh)$|Dockerfile$/;

function tokenCounts(root, skip = new Set()) {
  const counts = Object.fromEntries(ZONE_C.map((t) => [t, 0]));
  for (const rel of listFiles(root)) {
    if (!TEXT.test(rel) || skip.has(rel)) continue;
    const raw = fs.readFileSync(path.join(root, rel), 'utf8');
    for (const t of ZONE_C) counts[t] += raw.split(t).length - 1;
  }
  return counts;
}

function strings(value, visit, trail = '') {
  if (typeof value === 'string') visit(value, trail);
  else if (Array.isArray(value)) value.forEach((v, i) => strings(v, visit, `${trail}[${i}]`));
  else if (value && typeof value === 'object') {
    for (const [k, v] of Object.entries(value)) strings(v, visit, trail ? `${trail}.${k}` : k);
  }
}

function keyPaths(value, trail = '', out = new Set()) {
  if (value && typeof value === 'object' && !Array.isArray(value)) {
    for (const [k, v] of Object.entries(value)) keyPaths(v, trail ? `${trail}.${k}` : k, out);
  } else out.add(trail);
  return out;
}

export function verify({ original, branded, config }) {
  const problems = [];
  // Files an override replaces whole are not counted (their comments differ by
  // design); they must keep every exported name instead.
  const overridden = new Set((config.overrides ?? []).map((o) => o.file));
  const before = tokenCounts(original, overridden);
  const after = tokenCounts(branded, overridden);
  for (const o of config.overrides ?? []) {
    const file = path.join(branded, o.file);
    const src = fs.existsSync(file) ? fs.readFileSync(file, 'utf8') : '';
    for (const name of o.exports) {
      if (!new RegExp(`export (function|const|class) ${name}\\b`).test(src)) {
        problems.push(`override ${o.file} no longer exports ${name}`);
      }
    }
  }
  for (const t of ZONE_C) {
    if (before[t] !== after[t]) {
      problems.push(`zone C: "${t}" occurs ${before[t]}x in the original and ${after[t]}x after the brand`);
    }
  }

  const dir = 'apps/dashboard/src/shared/i18n/messages';
  const allowed = new Set(config.attributionKeys ?? []);
  for (const name of fs.readdirSync(path.join(original, dir)).filter((n) => n.endsWith('.json')).sort()) {
    const o = JSON.parse(fs.readFileSync(path.join(original, dir, name), 'utf8'));
    const bFile = path.join(branded, dir, name);
    if (!fs.existsSync(bFile)) {
      problems.push(`catalog ${name} is missing from the branded tree`);
      continue;
    }
    const b = JSON.parse(fs.readFileSync(bFile, 'utf8'));
    const ok = keyPaths(o);
    const bk = keyPaths(b);
    for (const k of ok) if (!bk.has(k)) problems.push(`catalog ${name}: key "${k}" was removed`);
    strings(b, (s, trail) => {
      if (s.includes('Rustrak') && !allowed.has(trail)) {
        problems.push(`catalog ${name}: "${trail}" still says Rustrak`);
      }
    });
  }

  const about = path.join(branded, 'apps/dashboard/src/routes/_authenticated/settings/about.tsx');
  if (!fs.existsSync(about) || !fs.readFileSync(about, 'utf8').includes(config.attributionKey)) {
    problems.push(`the About page does not render ${config.attributionKey}`);
  }
  const en = JSON.parse(fs.readFileSync(path.join(branded, dir, 'en.json'), 'utf8'));
  let attribution = '';
  strings(en, (s, trail) => {
    if (trail === config.attributionKey.replace(/^about\./, 'settings.about.')) attribution = s;
  });
  for (const must of ['Rustrak', 'GPL-3.0']) {
    if (!attribution.includes(must)) problems.push(`the attribution text does not mention ${must}`);
  }
  return problems;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const here = path.dirname(fileURLToPath(import.meta.url));
  const args = parseArgs(process.argv.slice(2));
  const name = args._[0];
  if (!name || !args.branded) {
    console.error('usage: verify.mjs <brand> --branded DIR [--original DIR]');
    process.exit(2);
  }
  const config = JSON.parse(fs.readFileSync(path.join(here, name, 'verify.json'), 'utf8'));
  const problems = verify({
    original: path.resolve(args.original ?? path.join(here, '..')),
    branded: path.resolve(args.branded),
    config,
  });
  if (problems.length) {
    console.error(`brand ${name}: ${problems.length} problem(s)`);
    for (const p of problems) console.error(`  - ${p}`);
    process.exit(1);
  }
  console.log(`brand ${name}: zone C intact, catalogs keep their keys, attribution present`);
}
