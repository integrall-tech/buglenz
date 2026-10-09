import type { ProjectStatsSummary } from '@rustrak/client';

/** What the overview's headline says about the period. */
export type OverviewSummary =
  | { kind: 'calm' }
  | { kind: 'busy'; newIssues: number; events: number };

/**
 * A project is calm when nothing new broke in the period. Open issues from before do not count:
 * they are in the list below, and a backlog that is not growing is not news.
 */
export function overviewSummary(
  stats: Pick<ProjectStatsSummary, 'events' | 'new_issues' | 'open_issues'>,
): OverviewSummary {
  if (stats.new_issues.current === 0) return { kind: 'calm' };
  return {
    kind: 'busy',
    newIssues: stats.new_issues.current,
    events: stats.events.current,
  };
}
