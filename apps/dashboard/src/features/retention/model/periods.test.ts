import { describe, expect, it } from 'vitest';
import { daysText, fieldOf, MAX_DAYS, MIN_DAYS, parseDays } from './periods';

describe('parseDays', () => {
  it('reads a whole number of days in range', () => {
    expect(parseDays(String(MIN_DAYS))).toBe(MIN_DAYS);
    expect(parseDays(' 30 ')).toBe(30);
    expect(parseDays(String(MAX_DAYS))).toBe(MAX_DAYS);
  });

  it('reads an empty box as "use the default"', () => {
    expect(parseDays('')).toBeNull();
    expect(parseDays('   ')).toBeNull();
  });

  it('refuses anything else', () => {
    for (const bad of [
      '0',
      '1',
      '6',
      '-5',
      '3651',
      '1.5',
      'abc',
      '10d',
      '1e3',
      '٣',
    ]) {
      expect(parseDays(bad), bad).toBe('invalid');
    }
  });
});

describe('daysText', () => {
  it('round-trips a period and shows none as empty', () => {
    expect(daysText(null)).toBe('');
    expect(daysText(90)).toBe('90');
    expect(parseDays(daysText(45))).toBe(45);
  });
});

describe('fieldOf', () => {
  it('names the API field of each type', () => {
    expect(fieldOf('events')).toBe('events_days');
    expect(fieldOf('transactions')).toBe('transactions_days');
    expect(fieldOf('logs')).toBe('logs_days');
  });
});
