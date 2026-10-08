import type { RustrakError } from '../errors.js';
import type { Result } from '../result.js';
import {
  retentionOverviewSchema,
  retentionPeriodsSchema,
  updateRetentionSchema,
} from '../schemas/retention.js';
import type {
  RetentionOverview,
  RetentionPeriods,
  UpdateRetention,
} from '../types/retention.js';
import { BaseResource } from './base.js';

/**
 * Retention API resource (admin only, BugLenz).
 *
 * The server applies the periods by itself, every day; this is how an
 * administrator reads and edits them.
 */
export class RetentionResource extends BaseResource {
  /** The instance defaults, every project's periods and the last pass. */
  async get(): Promise<Result<RetentionOverview, RustrakError>> {
    return this.request(
      () => this.http.get('api/retention'),
      retentionOverviewSchema,
    );
  }

  /**
   * Set or clear a project's own periods. A number sets one, `null` clears it,
   * an omitted field is left as it is. Returns the project's own periods.
   */
  async updateProject(
    projectId: number,
    input: UpdateRetention,
  ): Promise<Result<RetentionPeriods, RustrakError>> {
    const validatedInput = this.validateInput(input, updateRetentionSchema);
    if (!validatedInput.success) {
      return validatedInput;
    }

    return this.request(
      () =>
        this.http.put(`api/projects/${projectId}/retention`, {
          json: validatedInput.data,
        }),
      retentionPeriodsSchema,
    );
  }
}
