import type { Result, RetentionOverview, RustrakError } from '@rustrak/client';
import { createClient } from '@/shared/api/rustrak';

/** The instance defaults, every project's retention periods and the last pass. */
export async function getRetention(): Promise<
  Result<RetentionOverview, RustrakError>
> {
  const client = await createClient();
  return client.retention.get();
}
