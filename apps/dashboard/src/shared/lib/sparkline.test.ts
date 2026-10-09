import { describe, expect, it } from 'vitest';
import { sparkline } from './sparkline';

describe('sparkline', () => {
  it('draws nothing for fewer than two points', () => {
    expect(sparkline([], 100, 20)).toBeNull();
    expect(sparkline([5], 100, 20)).toBeNull();
  });

  it('spans the whole width and keeps the line inside the height', () => {
    const s = sparkline([0, 10, 5], 100, 20, 2);
    expect(s).not.toBeNull();
    expect(s?.points[0]).toEqual({ x: 0, y: 18 });
    expect(s?.points[2]).toEqual({ x: 100, y: 10 });
    // The highest value sits at the top padding, the lowest at the bottom one.
    expect(Math.min(...(s?.points.map((p) => p.y) ?? []))).toBe(2);
    expect(Math.max(...(s?.points.map((p) => p.y) ?? []))).toBe(18);
  });

  it('puts a flat series in the middle instead of dividing by zero', () => {
    const s = sparkline([4, 4, 4], 100, 20);
    expect(s?.points.every((p) => p.y === 10)).toBe(true);
  });

  it('closes the area down to the baseline', () => {
    const s = sparkline([1, 2], 100, 20, 0);
    expect(s?.area.endsWith('L100,20 L0,20 Z')).toBe(true);
    expect(s?.line.startsWith('M0,')).toBe(true);
  });
});
