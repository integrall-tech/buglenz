import type { RetentionOverview, RetentionProject } from '@rustrak/client';
import { AlertTriangle, Loader2 } from 'lucide-react';
import { useState, useTransition } from 'react';
import { toast } from 'sonner';
import { useFormatter, useTranslations } from 'use-intl';
import { updateProjectRetention } from '@/features/retention/api/mutations';
import { getRetention } from '@/features/retention/api/queries';
import {
  daysText,
  fieldOf,
  MAX_DAYS,
  MIN_DAYS,
  parseDays,
  RETENTION_TYPES,
  type RetentionType,
} from '@/features/retention/model/periods';
import { LoadFailure } from '@/shared/ui/components/load-failure';
import { Badge } from '@/shared/ui/components/shadcn/badge';
import { Button } from '@/shared/ui/components/shadcn/button';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/shared/ui/components/shadcn/card';
import { Input } from '@/shared/ui/components/shadcn/input';
import { Label } from '@/shared/ui/components/shadcn/label';
import { Skeleton } from '@/shared/ui/components/shadcn/skeleton';
import { useAsync } from '@/shared/ui/hooks/use-async';

/** Defaults, the last pass and one editable row per project. */
export function RetentionPanel() {
  const t = useTranslations('settings');
  const [version, setVersion] = useState(0);
  const read = useAsync(() => getRetention(), [version]);

  if (read.state === 'pending') return <RetentionSkeleton />;
  const result = read.data;

  if (!result.success) {
    return (
      <LoadFailure
        error={result.error}
        title={t('retention.loadFailed')}
        notFoundOnMissing={false}
      />
    );
  }

  const overview = result.data;
  const unprotected = overview.projects.filter((p) => !p.protected).length;

  return (
    <div className="space-y-6">
      {unprotected > 0 && (
        <div
          role="alert"
          className="flex items-start gap-3 rounded-lg border border-amber-500/40 bg-amber-500/10 p-4 text-sm"
        >
          <AlertTriangle className="size-5 shrink-0 text-amber-600" />
          <p>{t('retention.warningUnprotected', { count: unprotected })}</p>
        </div>
      )}
      <DefaultsCard overview={overview} />
      <Card>
        <CardHeader>
          <CardTitle>{t('retention.byProject')}</CardTitle>
          <CardDescription>
            {t('retention.byProjectDescription')}
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          {overview.projects.length === 0 ? (
            <p className="text-center text-muted-foreground py-8 text-sm">
              {t('retention.noProjects')}
            </p>
          ) : (
            overview.projects.map((project) => (
              <ProjectRow
                key={project.project_id}
                project={project}
                defaults={overview.defaults}
                onSaved={() => setVersion((v) => v + 1)}
              />
            ))
          )}
        </CardContent>
      </Card>
    </div>
  );
}

function DefaultsCard({ overview }: { overview: RetentionOverview }) {
  const t = useTranslations('settings');
  const format = useFormatter();
  const last = overview.last_run;

  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('retention.defaultsTitle')}</CardTitle>
        <CardDescription>{t('retention.defaultsDescription')}</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <dl className="grid grid-cols-1 sm:grid-cols-3 gap-4">
          {RETENTION_TYPES.map((type) => {
            const days = overview.defaults[fieldOf(type)];
            return (
              <div key={type}>
                <dt className="text-xs text-muted-foreground">
                  {t(`retention.types.${type}`)}
                </dt>
                <dd className="text-lg font-semibold tabular-nums">
                  {days === null
                    ? t('retention.notSet')
                    : t('retention.days', { count: days })}
                </dd>
              </div>
            );
          })}
        </dl>
        <p className="text-sm text-muted-foreground">
          {last
            ? t('retention.lastRun', {
                when: format.dateTime(new Date(last.finished_at), {
                  dateStyle: 'medium',
                  timeStyle: 'short',
                }),
                events: format.number(last.removed.events),
                transactions: format.number(last.removed.transactions),
                logs: format.number(last.removed.logs),
              })
            : t('retention.noRun', { hours: overview.interval_hours })}
        </p>
      </CardContent>
    </Card>
  );
}

function ProjectRow({
  project,
  defaults,
  onSaved,
}: {
  project: RetentionProject;
  defaults: RetentionOverview['defaults'];
  onSaved: () => void;
}) {
  const t = useTranslations('settings');
  const [pending, startTransition] = useTransition();
  const initial = (type: RetentionType) => daysText(project.own[fieldOf(type)]);
  const [text, setText] = useState<Record<RetentionType, string>>({
    events: initial('events'),
    transactions: initial('transactions'),
    logs: initial('logs'),
  });

  const parsed = RETENTION_TYPES.map((type) => parseDays(text[type]));
  const invalid = parsed.includes('invalid');
  const changed = RETENTION_TYPES.some((type) => text[type] !== initial(type));

  const save = () => {
    const events = parseDays(text.events);
    const transactions = parseDays(text.transactions);
    const logs = parseDays(text.logs);
    if (
      events === 'invalid' ||
      transactions === 'invalid' ||
      logs === 'invalid'
    ) {
      return;
    }
    startTransition(async () => {
      const result = await updateProjectRetention(project.project_id, {
        events_days: events,
        transactions_days: transactions,
        logs_days: logs,
      });
      if (!result.success) {
        toast.error(t('retention.saveFailed'), {
          description: result.error.message,
        });
        return;
      }
      toast.success(t('retention.saved', { project: project.name }));
      onSaved();
    });
  };

  return (
    <div className="rounded-lg border p-4 space-y-3">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <p className="font-medium">{project.name}</p>
        {project.protected ? (
          <Badge variant="secondary">{t('retention.protected')}</Badge>
        ) : (
          <Badge variant="destructive">
            {t('retention.unprotectedMissing', {
              types: project.missing
                .map((m) => t(`retention.types.${m}`))
                .join(', '),
            })}
          </Badge>
        )}
      </div>
      <div className="grid grid-cols-1 sm:grid-cols-3 gap-3">
        {RETENTION_TYPES.map((type) => {
          const id = `retention-${project.project_id}-${type}`;
          const fallback = defaults[fieldOf(type)];
          return (
            <div key={type} className="space-y-1.5">
              <Label htmlFor={id}>{t(`retention.types.${type}`)}</Label>
              <Input
                id={id}
                inputMode="numeric"
                value={text[type]}
                aria-invalid={parseDays(text[type]) === 'invalid'}
                placeholder={
                  fallback === null
                    ? t('retention.placeholderNone')
                    : t('retention.placeholderDefault', { days: fallback })
                }
                onChange={(e) =>
                  setText((prev) => ({ ...prev, [type]: e.target.value }))
                }
              />
            </div>
          );
        })}
      </div>
      {invalid && (
        <p className="text-sm text-destructive">
          {t('retention.invalidRange', { min: MIN_DAYS, max: MAX_DAYS })}
        </p>
      )}
      <div className="flex justify-end">
        <Button
          type="button"
          size="sm"
          onClick={save}
          disabled={pending || invalid || !changed}
        >
          {pending && <Loader2 className="size-4 animate-spin" />}
          {t('retention.save')}
        </Button>
      </div>
    </div>
  );
}

function RetentionSkeleton() {
  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <Skeleton className="h-5 w-40" />
          <Skeleton className="h-4 w-72 mt-1" />
        </CardHeader>
        <CardContent>
          <Skeleton className="h-12 w-full" />
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <Skeleton className="h-5 w-28" />
        </CardHeader>
        <CardContent className="space-y-4">
          {Array.from({ length: 2 }).map((_, i) => (
            <Skeleton key={i} className="h-28 w-full" />
          ))}
        </CardContent>
      </Card>
    </div>
  );
}
