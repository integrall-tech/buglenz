#!/usr/bin/env node
// Drives the built app in headless Chromium and fires the four errors of the
// GAP §3 script: a click-handler exception (twice), an unhandled promise
// rejection and a render error caught by Sentry.ErrorBoundary. The SDK sends
// the events and the session updates on its own; the assertions live in
// scripts/e2e-react-assert.sh, against the server's API.
//
//   node run.mjs http://127.0.0.1:4173
import { chromium } from 'playwright';

const url = process.argv[2];
if (!url) {
  console.error('usage: node run.mjs <app url>');
  process.exit(2);
}

const browser = await chromium.launch();
const page = await browser.newPage();
const envelopes = [];
page.on('request', (req) => {
  if (req.url().includes('/envelope/')) envelopes.push(req.url());
});
page.on('pageerror', (err) => console.log(`[page] ${err.name}: ${err.message}`));

await page.goto(url, { waitUntil: 'networkidle' });
await page.waitForSelector('#resumo');

await page.click('#btn-click');
await page.click('#btn-click');
await page.click('#btn-promise');
await page.click('#btn-render');
await page.waitForSelector('#fallback');

// Let the SDK flush its queue, then end the page so the session closes.
await page.waitForTimeout(3000);
await page.close();
await page.context().close();
await browser.close();

console.log(`envelopes sent: ${envelopes.length}`);
if (envelopes.length < 4) {
  console.error('expected at least 4 envelope requests (3 errors + session)');
  process.exit(1);
}
