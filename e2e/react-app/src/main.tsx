import * as Sentry from '@sentry/react';
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';

// The DSN is baked in at build time by the workflow, after it provisioned the
// project on the server under test.
const dsn = import.meta.env.VITE_SENTRY_DSN as string | undefined;
if (!dsn) {
  throw new Error('VITE_SENTRY_DSN is required at build time');
}

Sentry.init({
  dsn,
  release: 'e2e-react@1.0.0',
  environment: 'e2e',
  tracesSampleRate: 0,
  // The browser SDK drops a repeated identical error client-side (Dedupe).
  // The script clicks the same button twice on purpose, to prove that the
  // server groups the two events into one issue, so the filter is off here.
  integrations: (defaults) => defaults.filter((i) => i.name !== 'Dedupe'),
});

// Personal data the server must scrub (ADR-0009): the e-mail is masked, the
// id is kept, the extra under a denied key is filtered. The app sets them the
// way a real SDK wrapper would.
Sentry.setUser({ id: 'u-1', email: 'ana@example.com' });
Sentry.setExtra('password', 'hunter2');

createRoot(document.getElementById('root') as HTMLElement).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
