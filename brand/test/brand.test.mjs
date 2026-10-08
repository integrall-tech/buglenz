// Tests of the brand overlay engine and its guards. Run: node --test brand/test/
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { applyRules, copyTreeForBuild, listFiles, loadRules, parseArgs, targets } from '../lib.mjs';
import { verify } from '../verify.mjs';

function tree(files) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'brand-test-'));
  for (const [rel, content] of Object.entries(files)) {
    fs.mkdirSync(path.dirname(path.join(root, rel)), { recursive: true });
    fs.writeFileSync(path.join(root, rel), content);
  }
  return root;
}
const read = (root, rel) => fs.readFileSync(path.join(root, rel), 'utf8');
const json = (o) => JSON.stringify(o, null, 2) + '\n';

test('replace swaps literal text and checks the count', () => {
  const root = tree({ 'a.txt': 'Rustrak and Rustrak\n' });
  applyRules(root, root, [{ id: 'r', type: 'replace', file: 'a.txt', find: 'Rustrak', replace: 'BugLenz', expect: 2 }]);
  assert.equal(read(root, 'a.txt'), 'BugLenz and BugLenz\n');
});

test('a wrong count fails and names the rule, the file and both numbers', () => {
  const root = tree({ 'a.txt': 'Rustrak\n' });
  assert.throws(
    () => applyRules(root, root, [{ id: 'drift', type: 'replace', file: 'a.txt', find: 'Rustrak', replace: 'X', expect: 2 }]),
    /rule "drift": a\.txt: expected 2 match\(es\), found 1/,
  );
});

test('multi-line find and ${var} replacement work; a missing var fails', () => {
  const root = tree({ 'a.txt': 'one\ntwo\nthree\n' });
  const rule = { id: 'm', type: 'replace', file: 'a.txt', find: ['one', 'two', ''], replace: 'v=${release}\n', expect: 1 };
  applyRules(root, root, [rule], { release: 'v1' });
  assert.equal(read(root, 'a.txt'), 'v=v1\nthree\n');
  const again = tree({ 'a.txt': 'one\ntwo\n' });
  assert.throws(() => applyRules(again, again, [rule], {}), /\$\{release\}/);
});

test('catalog rules change values only and refuse a brand name that is also a key', () => {
  const ok = tree({ 'm/en.json': json({ a: { title: 'Rustrak' }, b: 'Hi Rustrak' }) });
  applyRules(ok, ok, [{ id: 'c', type: 'catalog', files: ['m/*.json'], find: 'Rustrak', replace: 'BugLenz', expect: 2 }]);
  assert.deepEqual(JSON.parse(read(ok, 'm/en.json')), { a: { title: 'BugLenz' }, b: 'Hi BugLenz' });

  const bad = tree({ 'm/en.json': json({ Rustrak: 'x', b: 'Rustrak' }) });
  assert.throws(
    () => applyRules(bad, bad, [{ id: 'c', type: 'catalog', files: ['m/*.json'], find: 'Rustrak', replace: 'B', expect: 1 }]),
    /also occurs in a key/,
  );
});

test('catalog-set adds a nested key, per language with an English fallback', () => {
  const root = tree({ 'm/en.json': json({ s: { a: 'x' } }), 'm/es.json': json({ s: { a: 'x' } }), 'm/fr.json': json({ s: { a: 'x' } }) });
  applyRules(root, root, [{ id: 's', type: 'catalog-set', files: ['m/*.json'], key: 's.attr', values: { en: 'EN', es: 'ES' } }]);
  assert.equal(JSON.parse(read(root, 'm/es.json')).s.attr, 'ES');
  assert.equal(JSON.parse(read(root, 'm/fr.json')).s.attr, 'EN');
});

test('catalog-set refuses a catalog it could not write back identically', () => {
  const root = tree({ 'm/en.json': '{"a":"x"}' });
  assert.throws(() => applyRules(root, root, [{ id: 's', type: 'catalog-set', files: ['m/*.json'], key: 'b', values: { en: 'v' } }]), /not round-trippable/);
});

test('an override must replace a file that exists upstream', () => {
  const brand = tree({ 'overrides/src/logo.tsx': 'new\n' });
  const root = tree({ 'src/logo.tsx': 'old\n' });
  applyRules(root, brand, [{ id: 'o', type: 'override', dir: 'overrides' }]);
  assert.equal(read(root, 'src/logo.tsx'), 'new\n');

  const empty = tree({ 'src/other.tsx': 'x\n' });
  assert.throws(() => applyRules(empty, brand, [{ id: 'o', type: 'override', dir: 'overrides' }]), /does not exist; the upstream moved it/);
});

test('copy puts an asset where the rule says', () => {
  const brand = tree({ 'assets/icon.png': 'PNG' });
  const root = tree({ 'keep': '1' });
  applyRules(root, brand, [{ id: 'i', type: 'copy', from: 'assets/icon.png', to: 'public/icon.png' }]);
  assert.equal(read(root, 'public/icon.png'), 'PNG');
});

test('a rule that matches no file, an unknown type and duplicate ids are errors', () => {
  const root = tree({ 'a.txt': 'x' });
  assert.throws(() => targets(root, { id: 't', files: ['none/*.json'] }), /does not exist/);
  assert.throws(() => applyRules(root, root, [{ id: 'u', type: 'nope' }]), /unknown type/);
  const dup = tree({ 'rules.json': JSON.stringify([{ id: 'a' }, { id: 'a' }]) });
  assert.throws(() => loadRules(dup), /duplicate rule id/);
});

test('the build copy keeps tests/e2e under the server but drops the fork top-level directories', () => {
  const src = tree({ 'apps/server/tests/e2e/a.rs': '1', 'e2e/react-app/x': '2', 'brand/x': '3', 'governance/x': '4', 'node_modules/y': '5', 'README.md': '6' });
  const dest = path.join(fs.mkdtempSync(path.join(os.tmpdir(), 'brand-dest-')), 'out');
  copyTreeForBuild(src, dest);
  assert.deepEqual(listFiles(dest).sort(), ['README.md', 'apps/server/tests/e2e/a.rs']);
});

test('parseArgs reads flags and repeated --var', () => {
  const a = parseArgs(['buglenz', '--dest', '/x', '--var', 'release=v1', '--var', 'k=a=b', '--in-place']);
  assert.deepEqual(a._, ['buglenz']);
  assert.equal(a.dest, '/x');
  assert.equal(a['in-place'], true);
  assert.deepEqual(a.vars, { release: 'v1', k: 'a=b' });
});

// ---------------------------------------------------------------- verify

const CFG = {
  attributionKey: 'about.attribution',
  attributionKeys: ['settings.about.attribution'],
  overrides: [{ file: 'src/wm.tsx', exports: ['RustrakWordmark'] }],
};
const DIR = 'apps/dashboard/src/shared/i18n/messages';
const ABOUT = 'apps/dashboard/src/routes/_authenticated/settings/about.tsx';

function pair(extraBranded = {}) {
  const original = tree({
    [`${DIR}/en.json`]: json({ settings: { about: { title: 'About Rustrak' } } }),
    'src/code.ts': 'process.env.RUSTRAK_X; x.rustrak_total; import "@rustrak/client"; RustrakError; "X-Rustrak-Signature"\n',
    'src/wm.tsx': 'export function RustrakWordmark() {}\n',
    [ABOUT]: "t('x')\n",
  });
  const branded = tree({
    [`${DIR}/en.json`]: json({ settings: { about: { title: 'About BugLenz', attribution: 'BugLenz is a fork of Rustrak, GPL-3.0.' } } }),
    'src/code.ts': 'process.env.RUSTRAK_X; x.rustrak_total; import "@rustrak/client"; RustrakError; "X-Rustrak-Signature"\n',
    'src/wm.tsx': 'export function RustrakWordmark() { /* new */ }\n',
    [ABOUT]: "t('about.attribution')\n",
    ...extraBranded,
  });
  return { original, branded };
}

test('verify passes a faithful branding', () => {
  const { original, branded } = pair();
  assert.deepEqual(verify({ original, branded, config: CFG }), []);
});

test('verify catches a renamed identifier (zone C)', () => {
  const { original, branded } = pair({ 'src/code.ts': 'process.env.BUGLENZ_X; x.rustrak_total; import "@rustrak/client"; RustrakError; "X-Rustrak-Signature"\n' });
  const problems = verify({ original, branded, config: CFG });
  assert.ok(problems.some((p) => p.includes('"RUSTRAK_"')), problems.join('\n'));
});

test('verify catches a webhook header that was renamed', () => {
  const { original, branded } = pair({ 'src/code.ts': 'process.env.RUSTRAK_X; x.rustrak_total; import "@rustrak/client"; RustrakError; "X-BugLenz-Signature"\n' });
  assert.ok(verify({ original, branded, config: CFG }).some((p) => p.includes('X-Rustrak-')));
});

test('verify catches a Rustrak left in a catalog value and a removed key', () => {
  const left = pair({ [`${DIR}/en.json`]: json({ settings: { about: { title: 'About Rustrak', attribution: 'a fork of Rustrak, GPL-3.0' } } }) });
  assert.ok(verify({ ...left, config: CFG }).some((p) => p.includes('settings.about.title') && p.includes('still says Rustrak')));
  const gone = pair({ [`${DIR}/en.json`]: json({ settings: { about: { attribution: 'a fork of Rustrak, GPL-3.0' } } }) });
  assert.ok(verify({ ...gone, config: CFG }).some((p) => p.includes('key "settings.about.title" was removed')));
});

test('verify requires the attribution to be rendered and to name Rustrak and the licence', () => {
  const unrendered = pair({ [ABOUT]: "t('x')\n" });
  assert.ok(verify({ ...unrendered, config: CFG }).some((p) => p.includes('does not render')));
  const weak = pair({ [`${DIR}/en.json`]: json({ settings: { about: { title: 'About BugLenz', attribution: 'made by us' } } }) });
  const problems = verify({ ...weak, config: CFG });
  assert.ok(problems.some((p) => p.includes('mention Rustrak')) && problems.some((p) => p.includes('mention GPL-3.0')));
});

test('verify requires an override to keep its exported names', () => {
  const { original, branded } = pair({ 'src/wm.tsx': 'export function Other() {}\n' });
  assert.ok(verify({ original, branded, config: CFG }).some((p) => p.includes('no longer exports RustrakWordmark')));
});
