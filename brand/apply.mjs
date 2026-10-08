#!/usr/bin/env node
// Applies a brand overlay to a copy of the source tree.
//
//   node brand/apply.mjs buglenz --dest /tmp/branded [--var release=v0.16.0-itl.5]
//   node brand/apply.mjs buglenz --in-place          [--var release=...]
//
// `--dest` copies the tree there first (what a developer or a check wants).
// `--in-place` rewrites the checkout it runs in, and is for CI runners whose
// workspace is thrown away: refuse it anywhere else, so nobody brands their
// own source tree by accident. The upstream's files are never edited in git.
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { applyRules, copyTreeForBuild, loadRules, parseArgs } from './lib.mjs';

const here = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(here, '..');
const args = parseArgs(process.argv.slice(2));
const name = args._[0];
if (!name || (!args.dest && !args['in-place'])) {
  console.error('usage: apply.mjs <brand> (--dest DIR | --in-place) [--var name=value]...');
  process.exit(2);
}
const brandDir = path.join(here, name);
if (!fs.existsSync(path.join(brandDir, 'rules.json'))) {
  console.error(`brand "${name}" has no ${path.relative(repo, brandDir)}/rules.json`);
  process.exit(2);
}

let root = repo;
if (args.dest) {
  root = path.resolve(args.dest);
  if (fs.existsSync(root) && fs.readdirSync(root).length > 0) {
    console.error(`${root} is not empty; refusing to overwrite it`);
    process.exit(2);
  }
  copyTreeForBuild(repo, root);
} else if (process.env.CI !== 'true' && !process.env.BRAND_ALLOW_IN_PLACE) {
  console.error('--in-place only runs in CI (CI=true), or with BRAND_ALLOW_IN_PLACE=1 on a disposable checkout');
  process.exit(2);
}

try {
  const report = applyRules(root, brandDir, loadRules(brandDir), args.vars);
  const touched = new Set(report.map((r) => r.file));
  console.log(`brand ${name}: ${report.length} rule applications, ${touched.size} files, tree at ${root}`);
} catch (e) {
  console.error(`brand ${name}: ${e.message}`);
  process.exit(1);
}
