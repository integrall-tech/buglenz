import { z } from 'zod';
import { cleanupCountsSchema } from './storage.js';

/** Retention periods in days, one per data type; `null` means none. */
export const retentionPeriodsSchema = z.object({
  events_days: z.number().int().nullable(),
  transactions_days: z.number().int().nullable(),
  logs_days: z.number().int().nullable(),
});

/** One project: its own periods, what applies, and whether every type is covered. */
export const retentionProjectSchema = z.object({
  project_id: z.number(),
  name: z.string(),
  own: retentionPeriodsSchema,
  effective: retentionPeriodsSchema,
  protected: z.boolean(),
  missing: z.array(z.string()),
});

/** What one retention pass did. */
export const retentionReportSchema = z.object({
  started_at: z.string(),
  finished_at: z.string(),
  projects: z.number(),
  removed: cleanupCountsSchema,
  /** Release-health rows removed by the `events` period (absent on servers before 2026-10-09). */
  sessions_removed: z.number().optional(),
  /** Alert-history rows removed by the `events` period (absent on servers before 2026-10-09). */
  alerts_removed: z.number().optional(),
  unprotected: z.array(
    z.object({ project_id: z.number(), missing: z.array(z.string()) }),
  ),
  failed: z.array(z.number()),
});

/** The instance defaults, every project's periods and the last pass. */
export const retentionOverviewSchema = z.object({
  defaults: retentionPeriodsSchema,
  interval_hours: z.number(),
  last_run: retentionReportSchema.nullable(),
  projects: z.array(retentionProjectSchema),
});

const days = z.number().int().min(7).max(3650);

/**
 * A number sets a period, `null` clears it (the instance default applies), an
 * omitted field is left as it is.
 */
export const updateRetentionSchema = z.object({
  events_days: days.nullable().optional(),
  transactions_days: days.nullable().optional(),
  logs_days: days.nullable().optional(),
});
