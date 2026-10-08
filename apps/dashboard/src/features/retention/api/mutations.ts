import type {
  Result,
  RetentionPeriods,
  RustrakError,
  UpdateRetention,
} from '@rustrak/client';
import { createClient } from '@/shared/api/rustrak';

/**
 * Set or clear a project's own retention periods. A number sets one, `null`
 * clears it (the instance default applies), an omitted field is left alone.
 */
export async function updateProjectRetention(
  projectId: number,
  input: UpdateRetention,
): Promise<Result<RetentionPeriods, RustrakError>> {
  const client = await createClient();
  return client.retention.updateProject(projectId, input);
}
