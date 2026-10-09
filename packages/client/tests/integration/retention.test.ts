import { describe, expect, it } from 'vitest';
import { RustrakClient } from '../../src/index.js';
import { expectErr, expectOk } from '../helpers/result.js';

const client = new RustrakClient({
  baseUrl: 'http://localhost:8080',
  token: 'test-token',
});

describe('RetentionResource', () => {
  describe('get()', () => {
    it('returns the defaults, every project and the last pass', async () => {
      const overview = expectOk(await client.retention.get());

      expect(overview.defaults).toEqual({
        events_days: 90,
        transactions_days: 30,
        logs_days: null,
      });
      expect(overview.interval_hours).toBe(24);
      expect(overview.projects).toHaveLength(2);
      expect(overview.projects[0].protected).toBe(false);
      expect(overview.projects[0].missing).toEqual(['logs']);
      expect(overview.projects[1].protected).toBe(true);
      expect(overview.last_run?.removed.events).toBe(3);
      expect(overview.last_run?.sessions_removed).toBe(12);
      expect(overview.last_run?.alerts_removed).toBe(3);
      expect(overview.last_run?.unprotected).toEqual([
        { project_id: 1, missing: ['logs'] },
      ]);
    });
  });

  describe('updateProject()', () => {
    it('sends the periods and returns the project’s own', async () => {
      const own = expectOk(
        await client.retention.updateProject(1, {
          events_days: 14,
          logs_days: 60,
        }),
      );

      expect(own).toEqual({
        events_days: 14,
        transactions_days: null,
        logs_days: 60,
      });
    });

    it('accepts null to clear a period', async () => {
      const own = expectOk(
        await client.retention.updateProject(1, { events_days: null }),
      );

      expect(own.events_days).toBeNull();
    });

    it('refuses a period outside 7..3650 before calling the server', async () => {
      for (const bad of [0, 1, 6, -1, 3651, 1.5]) {
        const error = expectErr(
          await client.retention.updateProject(1, { events_days: bad }),
        );
        expect(error.kind).toBe('invalid_request');
      }
    });

    it('maps a project that does not exist', async () => {
      const error = expectErr(
        await client.retention.updateProject(999, { events_days: 30 }),
      );
      expect(error.kind).toBe('not_found');
    });
  });
});
