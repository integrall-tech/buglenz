import type { RetentionPeriods } from '@rustrak/client';

/** The three kinds of data a period applies to. Spans follow transactions. */
export const RETENTION_TYPES = ['events', 'transactions', 'logs'] as const;
export type RetentionType = (typeof RETENTION_TYPES)[number];

/** The server refuses anything below this: the first pass runs a minute after every start. */
export const MIN_DAYS = 7;
export const MAX_DAYS = 3650;

/** The field of {@link RetentionPeriods} that holds a type's period. */
export function fieldOf(type: RetentionType): keyof RetentionPeriods {
  return `${type}_days` as keyof RetentionPeriods;
}

/**
 * What an input holds, as the API understands it: `null` for an empty box (the
 * instance default applies), a whole number of days, or `'invalid'`.
 */
export function parseDays(text: string): number | null | 'invalid' {
  const trimmed = text.trim();
  if (trimmed === '') return null;
  if (!/^\d+$/.test(trimmed)) return 'invalid';
  const days = Number(trimmed);
  return days >= MIN_DAYS && days <= MAX_DAYS ? days : 'invalid';
}

/** The text a period shows in its input. */
export function daysText(days: number | null): string {
  return days === null ? '' : String(days);
}
