import { useFormatter, useTranslations } from 'use-intl';
import { getProjectStatsSummary } from '@/features/project/api/queries';
import { overviewSummary } from '@/features/project/model/overview-summary';
import type { OverviewPeriod } from '@/features/release/model/session-health';
import { Skeleton } from '@/shared/ui/components/shadcn/skeleton';
import { useAsync } from '@/shared/ui/hooks/use-async';

interface OverviewSummaryProps {
  projectId: number;
  projectName: string;
  period?: OverviewPeriod;
}

/**
 * One sentence that answers "is something wrong?" before the reader looks at any chart. It is
 * decoration over the tiles below: when its query fails it says nothing instead of an error,
 * because each tile reports its own failure.
 */
export function OverviewSummary({
  projectId,
  projectName,
  period,
}: OverviewSummaryProps) {
  const t = useTranslations('projectPages.overview.summary');
  const format = useFormatter();
  const read = useAsync(
    () => getProjectStatsSummary(projectId, period),
    [projectId, period],
  );

  if (read.state === 'pending') {
    return <Skeleton className="mt-3 h-7 w-2/3 max-w-xl" />;
  }
  if (!read.data.success) return null;

  const summary = overviewSummary(read.data.data);
  const accent = (chunks: React.ReactNode) => (
    <em className="font-display italic text-brand-text">{chunks}</em>
  );

  return (
    <p className="mt-3 max-w-3xl font-display text-2xl leading-snug text-foreground/80 md:text-[1.7rem]">
      {summary.kind === 'calm'
        ? t.rich('calm', { project: projectName, accent })
        : t.rich('busy', {
            project: projectName,
            accent,
            count: summary.newIssues,
            events: format.number(summary.events),
          })}
    </p>
  );
}
