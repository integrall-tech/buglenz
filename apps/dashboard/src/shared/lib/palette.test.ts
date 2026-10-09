import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';
import {
  contrastRatio,
  parseColor,
} from '../../../../../packages/ui/src/styles/color';

/**
 * The dashboard's palette (BugLenz, ADR-0022): warm paper in light, warm ink in
 * dark, one orange. Light is the default theme. The pairs below are the ones a
 * reader meets on every screen; they are pinned to AA so a palette edit that
 * makes text unreadable fails here instead of in somebody's browser.
 */
const css = readFileSync(
  fileURLToPath(new URL('../../styles.css', import.meta.url)),
  'utf8',
).replace(/\/\*[\s\S]*?\*\//g, '');

function block(selector: RegExp): Map<string, string> {
  const body = css.match(selector)?.[1] ?? '';
  const out = new Map<string, string>();
  for (const [, name, value] of body.matchAll(/(--[a-z-]+)\s*:\s*([^;]+);/g)) {
    if (name && value) out.set(name, value.trim());
  }
  return out;
}

const light = block(/:root\s*\{([\s\S]*?)\n\}/);
const dark = block(/\.dark\s*\{([\s\S]*?)\n\}/);

function ratio(theme: Map<string, string>, fg: string, bg: string): number {
  const f = theme.get(fg);
  const b = theme.get(bg);
  if (!f || !b) throw new Error(`undeclared: ${fg} / ${bg}`);
  return contrastRatio(parseColor(f), parseColor(b));
}

const AA = 4.5;

describe.each([
  ['light', light],
  ['dark', dark],
])('%s theme reads at AA', (_name, theme) => {
  it.each([
    ['--foreground', '--background'],
    ['--card-foreground', '--card'],
    ['--popover-foreground', '--popover'],
    ['--muted-foreground', '--background'],
    ['--muted-foreground', '--card'],
    ['--primary-foreground', '--primary'],
    ['--secondary-foreground', '--secondary'],
    ['--sidebar-foreground', '--sidebar'],
    ['--sidebar-primary-foreground', '--sidebar-primary'],
    ['--sidebar-accent-foreground', '--sidebar-accent'],
    // The orange as text (the italic word of the overview sentence).
    ['--brand-text', '--background'],
    ['--brand-text', '--card'],
  ])('%s on %s', (fg, bg) => {
    expect(ratio(theme, fg, bg)).toBeGreaterThanOrEqual(AA);
  });
});

describe('the palette is the warm one', () => {
  it('has no lime left', () => {
    expect(css).not.toMatch(/oklch\(0\.(65|91) 0\.18 127\)/);
  });

  it('paints light paper warm: the page is off-white, the card is white', () => {
    expect(light.get('--card')).toBe('oklch(1 0 0)');
    expect(light.get('--background')).not.toBe('oklch(1 0 0)');
  });

  it('keeps the primary action off the brand orange in light, where white text on it would be 3:1', () => {
    expect(ratio(light, '--primary-foreground', '--primary')).toBeGreaterThan(
      12,
    );
  });
});

/*
 * The rail is ink in both themes, but the components inside it write `text-muted-foreground`,
 * `hover:text-foreground` and `bg-primary` (the active row). Those resolve to the page theme, which
 * in light is dark-on-dark. The stylesheet re-points them inside the rail; this proves the result.
 */
describe('the rail reads on its own ink in light', () => {
  const rail = block(/\[data-sidebar="sidebar"\]\s*\{([\s\S]*?)\n\}/);
  const inRail = new Map([...light, ...rail]);

  it('re-points the page tokens the sidebar components use', () => {
    for (const name of [
      '--foreground',
      '--muted-foreground',
      '--primary',
      '--primary-foreground',
      '--border',
    ]) {
      expect(rail.has(name), name).toBe(true);
    }
  });

  it.each([
    ['--foreground', '--sidebar'],
    ['--muted-foreground', '--sidebar'],
    ['--primary-foreground', '--primary'],
    ['--primary', '--sidebar'],
  ])('%s on %s', (fg, bg) => {
    const min = fg === '--primary' ? 3 : AA;
    expect(ratio(inRail, fg, bg)).toBeGreaterThanOrEqual(min);
  });
});
