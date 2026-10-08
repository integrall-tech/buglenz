import { sentryVitePlugin } from '@sentry/vite-plugin';
import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

// Source maps go to the server under test (SENTRY_URL), never to sentry.io:
// the plugin's own telemetry is off and the org slug is ignored by Rustrak.
export default defineConfig({
  plugins: [
    react(),
    sentryVitePlugin({
      url: process.env.SENTRY_URL,
      authToken: process.env.SENTRY_AUTH_TOKEN,
      org: 'e2e',
      project: process.env.SENTRY_PROJECT,
      release: { name: 'e2e-react@1.0.0' },
      telemetry: false,
      sourcemaps: { filesToDeleteAfterUpload: ['./dist/**/*.map'] },
    }),
  ],
  build: {
    sourcemap: true,
    minify: true,
  },
});
