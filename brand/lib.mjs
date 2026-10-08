// BugLenz brand overlay: the rules engine (ADR-0006, OpenSpec 007).
//
// The upstream's files stay byte-identical in git. The brand is applied to a
// disposable copy of the tree, at build time, by declarative rules that each
// state how many matches they expect. When a merge from the upstream adds or
// removes a branded string, the count changes and the build fails naming the
// rule: somebody looks, and adjusts the number.
//
// Rule types (brand/<name>/rules.json, an array applied in order):
//   replace      {id, file | files, find, replace, expect}   literal text in files
//   catalog      {id, files, find, replace, expect}          literal text, in the
//                                                            VALUES of JSON catalogs
//                                                            only; never in a key
//   catalog-set  {id, files, key, values: {<lang>: text}}    set or add one key
//   copy         {id, from, to}                              asset into the tree
//   override     {id, dir}                                   whole files replace
//                                                            same-named upstream ones
//
// `find` and `replace` are a string or an array of lines. `${name}` in a
// replacement is filled from --var name=value. `expect` is a number, or an
// object keyed by file basename when files differ.

import fs from 'node:fs';
import path from 'node:path';

export class RuleError extends Error {
  constructor(rule, message) {
    super(`rule "${rule.id ?? '(no id)'}": ${message}`);
    this.rule = rule;
  }
}

const text = (v) => (Array.isArray(v) ? v.join('\n') : v);

export function loadRules(brandDir) {
  const file = path.join(brandDir, 'rules.json');
  const rules = JSON.parse(fs.readFileSync(file, 'utf8'));
  if (!Array.isArray(rules)) throw new Error(`${file}: expected an array of rules`);
  const ids = new Set();
  for (const r of rules) {
    if (!r.id) throw new Error(`${file}: every rule needs an id`);
    if (ids.has(r.id)) throw new Error(`${file}: duplicate rule id "${r.id}"`);
    ids.add(r.id);
  }
  return rules;
}

/** Files a rule targets: `file`, or `files` where the last segment may hold one `*`. */
export function targets(root, rule) {
  const specs = rule.files ?? (rule.file ? [rule.file] : []);
  const out = [];
  for (const spec of specs) {
    if (!spec.includes('*')) {
      out.push(spec);
      continue;
    }
    const dir = path.dirname(spec);
    const re = new RegExp(`^${path.basename(spec).replace(/[.+^${}()|[\]\\]/g, '\\$&').replace(/\*/g, '.*')}$`);
    const abs = path.join(root, dir);
    if (!fs.existsSync(abs)) throw new RuleError(rule, `directory ${dir} does not exist`);
    for (const name of fs.readdirSync(abs).sort()) {
      if (re.test(name)) out.push(path.posix.join(dir, name));
    }
  }
  if (out.length === 0) throw new RuleError(rule, 'matches no file');
  return out;
}

function expected(rule, file) {
  const e = rule.expect;
  if (typeof e === 'number') return e;
  if (e && typeof e === 'object') {
    const v = e[path.basename(file)];
    if (typeof v === 'number') return v;
    throw new RuleError(rule, `expect has no count for ${path.basename(file)}`);
  }
  throw new RuleError(rule, '"expect" is required');
}

function count(haystack, needle) {
  return needle === '' ? 0 : haystack.split(needle).length - 1;
}

function fill(rule, value, vars) {
  return text(value).replace(/\$\{(\w+)\}/g, (_, name) => {
    if (!(name in vars)) throw new RuleError(rule, `uses \${${name}} but --var ${name}=... was not given`);
    return vars[name];
  });
}

function checkCount(rule, file, found, want) {
  if (found !== want) {
    throw new RuleError(rule, `${file}: expected ${want} match(es), found ${found}`);
  }
}

function walkStrings(value, visit, trail = '') {
  if (typeof value === 'string') visit(value, trail);
  else if (Array.isArray(value)) value.forEach((v, i) => walkStrings(v, visit, `${trail}[${i}]`));
  else if (value && typeof value === 'object') {
    for (const [k, v] of Object.entries(value)) walkStrings(v, visit, trail ? `${trail}.${k}` : k);
  }
}

function setPath(obj, dotted, value) {
  const parts = dotted.split('.');
  let cur = obj;
  for (const p of parts.slice(0, -1)) {
    if (typeof cur[p] !== 'object' || cur[p] === null) cur[p] = {};
    cur = cur[p];
  }
  cur[parts.at(-1)] = value;
}

function copyTree(from, to, rule, mustExist) {
  for (const entry of fs.readdirSync(from, { withFileTypes: true })) {
    const src = path.join(from, entry.name);
    const dst = path.join(to, entry.name);
    if (entry.isDirectory()) {
      copyTree(src, dst, rule, mustExist);
    } else {
      if (mustExist && !fs.existsSync(dst)) {
        throw new RuleError(rule, `override ${path.relative(to, dst)} replaces a file that does not exist; the upstream moved it`);
      }
      fs.mkdirSync(path.dirname(dst), { recursive: true });
      fs.copyFileSync(src, dst);
    }
  }
}

const langOf = (file) => path.basename(file, '.json');

/** Applies every rule to `root`. Returns a report of what each rule did. */
export function applyRules(root, brandDir, rules, vars = {}) {
  const report = [];
  for (const rule of rules) {
    switch (rule.type) {
      case 'replace': {
        for (const file of targets(root, rule)) {
          const abs = path.join(root, file);
          const raw = fs.readFileSync(abs, 'utf8');
          const find = text(rule.find);
          const found = count(raw, find);
          checkCount(rule, file, found, expected(rule, file));
          fs.writeFileSync(abs, raw.split(find).join(fill(rule, rule.replace, vars)));
          report.push({ id: rule.id, file, replaced: found });
        }
        break;
      }
      case 'catalog': {
        for (const file of targets(root, rule)) {
          const abs = path.join(root, file);
          const raw = fs.readFileSync(abs, 'utf8');
          const data = JSON.parse(raw);
          const find = text(rule.find);
          let inValues = 0;
          walkStrings(data, (s) => {
            inValues += count(s, find);
          });
          checkCount(rule, file, inValues, expected(rule, file));
          // The text is replaced in the raw file so formatting is untouched;
          // if it also occurs outside a value (in a key), the counts differ.
          const inRaw = count(raw, find);
          if (inRaw !== inValues) {
            throw new RuleError(rule, `${file}: "${find}" also occurs in a key (${inRaw} in the file, ${inValues} in values)`);
          }
          fs.writeFileSync(abs, raw.split(find).join(fill(rule, rule.replace, vars)));
          report.push({ id: rule.id, file, replaced: inValues });
        }
        break;
      }
      case 'catalog-set': {
        for (const file of targets(root, rule)) {
          const abs = path.join(root, file);
          const raw = fs.readFileSync(abs, 'utf8');
          const data = JSON.parse(raw);
          if (JSON.stringify(data, null, 2) + '\n' !== raw) {
            throw new RuleError(rule, `${file}: not round-trippable as 2-space JSON; the catalog format changed`);
          }
          const value = rule.values[langOf(file)] ?? rule.values.en;
          if (value === undefined) throw new RuleError(rule, `no value for ${langOf(file)} and no "en" fallback`);
          setPath(data, rule.key, fill(rule, value, vars));
          fs.writeFileSync(abs, JSON.stringify(data, null, 2) + '\n');
          report.push({ id: rule.id, file, replaced: 1 });
        }
        break;
      }
      case 'copy': {
        const from = path.join(brandDir, rule.from);
        if (!fs.existsSync(from)) throw new RuleError(rule, `asset ${rule.from} does not exist`);
        const to = path.join(root, rule.to);
        fs.mkdirSync(path.dirname(to), { recursive: true });
        fs.copyFileSync(from, to);
        report.push({ id: rule.id, file: rule.to, replaced: 1 });
        break;
      }
      case 'override': {
        const dir = path.join(brandDir, rule.dir);
        if (!fs.existsSync(dir)) throw new RuleError(rule, `${rule.dir} does not exist`);
        copyTree(dir, root, rule, true);
        report.push({ id: rule.id, file: rule.dir, replaced: 1 });
        break;
      }
      default:
        throw new RuleError(rule, `unknown type "${rule.type}"`);
    }
  }
  return report;
}

// Never read by a build, at any depth.
const SKIP_ANYWHERE = new Set(['.git', 'node_modules', 'target', 'dist', '.turbo', '.next']);
// The fork's own top-level directories; skipped at the root only, so
// apps/server/tests/e2e and the like are kept.
const SKIP_AT_ROOT = new Set(['brand', 'governance', 'e2e', 'deploy']);

const skipped = (name, rel) => SKIP_ANYWHERE.has(name) || (rel === '' && SKIP_AT_ROOT.has(name));

/** Copies the working tree to `dest`, leaving out what a build never reads. */
export function copyTreeForBuild(src, dest, rel = '') {
  fs.mkdirSync(dest, { recursive: true });
  for (const entry of fs.readdirSync(src, { withFileTypes: true })) {
    if (skipped(entry.name, rel)) continue;
    const from = path.join(src, entry.name);
    const to = path.join(dest, entry.name);
    if (entry.isDirectory()) copyTreeForBuild(from, to, rel ? `${rel}/${entry.name}` : entry.name);
    else if (entry.isFile()) fs.copyFileSync(from, to);
  }
}

/** Every regular file under `root` that a build would read, as relative paths. */
export function listFiles(root, rel = '') {
  const out = [];
  for (const entry of fs.readdirSync(path.join(root, rel), { withFileTypes: true })) {
    if (skipped(entry.name, rel)) continue;
    const r = rel ? `${rel}/${entry.name}` : entry.name;
    if (entry.isDirectory()) out.push(...listFiles(root, r));
    else if (entry.isFile()) out.push(r);
  }
  return out;
}

export function parseArgs(argv) {
  const args = { _: [], vars: {} };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === '--var') {
      const [k, ...v] = argv[++i].split('=');
      args.vars[k] = v.join('=');
    } else if (a.startsWith('--')) {
      const next = argv[i + 1];
      if (next === undefined || next.startsWith('--')) args[a.slice(2)] = true;
      else args[a.slice(2)] = argv[++i];
    } else args._.push(a);
  }
  return args;
}
