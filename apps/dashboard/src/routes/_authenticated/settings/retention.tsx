import { createFileRoute } from '@tanstack/react-router';
import { ShieldX } from 'lucide-react';
import { useTranslations } from 'use-intl';
import { RetentionPanel } from '@/features/retention/ui/components/retention-panel';
import { translator } from '@/shared/i18n/intl';
import { Card, CardContent } from '@/shared/ui/components/shadcn/card';
import { useSessionUser } from '@/shared/ui/hooks/use-session-user';

export const Route = createFileRoute('/_authenticated/settings/retention')({
  head: () => {
    const t = translator('settings');
    return {
      meta: [
        { title: t('retention.meta.title') },
        { name: 'description', content: t('retention.meta.description') },
      ],
    };
  },
  component: RetentionPage,
});

function PageHeader() {
  const t = useTranslations('settings');

  return (
    <div className="mb-6 md:mb-8">
      <h1 className="text-xl md:text-2xl font-extrabold tracking-tight">
        {t('retention.title')}
      </h1>
      <p className="text-muted-foreground mt-1">{t('retention.subtitle')}</p>
    </div>
  );
}

function RetentionPage() {
  const t = useTranslations('settings');
  const user = useSessionUser();

  // Retention periods decide what the instance deletes: administrators only.
  if (user.role !== 'admin') {
    return (
      <>
        <PageHeader />
        <Card className="border-dashed">
          <CardContent className="flex flex-col items-center justify-center py-12 text-center">
            <ShieldX className="size-12 text-muted-foreground/50 mb-4" />
            <p className="font-semibold">{t('notAuthorized')}</p>
            <p className="text-muted-foreground mt-1 text-sm max-w-sm">
              {t('retention.notAuthorizedDescription')}
            </p>
          </CardContent>
        </Card>
      </>
    );
  }

  return (
    <>
      <PageHeader />
      <RetentionPanel />
    </>
  );
}
