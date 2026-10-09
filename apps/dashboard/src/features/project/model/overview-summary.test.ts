import { describe, expect, it } from 'vitest';
import { overviewSummary } from './overview-summary';

const metric = (current: number, previous: number | null = null) => ({
  current,
  previous,
});

describe('overviewSummary', () => {
  it('is calm when no new issue appeared in the period', () => {
    expect(
      overviewSummary({
        events: metric(40, 38),
        new_issues: metric(0, 2),
        open_issues: 3,
      }),
    ).toEqual({ kind: 'calm' });
  });

  it('says how many new issues and events there were otherwise', () => {
    expect(
      overviewSummary({
        events: metric(520),
        new_issues: metric(7),
        open_issues: 7,
      }),
    ).toEqual({ kind: 'busy', newIssues: 7, events: 520 });
  });

  it('does not call a project calm just because it has open issues from before', () => {
    expect(
      overviewSummary({
        events: metric(10),
        new_issues: metric(1),
        open_issues: 30,
      }).kind,
    ).toBe('busy');
  });
});
