import type { z } from 'zod';
import type {
  retentionOverviewSchema,
  retentionPeriodsSchema,
  retentionProjectSchema,
  retentionReportSchema,
  updateRetentionSchema,
} from '../schemas/retention.js';

/** Retention periods in days, one per data type; `null` means none. */
export type RetentionPeriods = z.infer<typeof retentionPeriodsSchema>;

/** One project's retention periods. */
export type RetentionProject = z.infer<typeof retentionProjectSchema>;

/** What one retention pass did. */
export type RetentionReport = z.infer<typeof retentionReportSchema>;

/** The instance defaults, every project's periods and the last pass. */
export type RetentionOverview = z.infer<typeof retentionOverviewSchema>;

/** Input of {@link RetentionResource.updateProject}. */
export type UpdateRetention = z.infer<typeof updateRetentionSchema>;
