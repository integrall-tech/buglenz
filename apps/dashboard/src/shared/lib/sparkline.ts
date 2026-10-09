/**
 * Geometry of a tiny trend line: no axes, no labels, no interaction. The overview's stat tiles use
 * it as a hint of shape beside a number the reader already has; anything the reader has to measure
 * belongs in a real chart.
 */
export interface Sparkline {
  points: { x: number; y: number }[];
  /** SVG path of the line. */
  line: string;
  /** SVG path of the line closed down to the baseline, for the fill. */
  area: string;
}

/**
 * Maps `values` onto a `width` x `height` box, `pad` px inside the top and bottom so the stroke is
 * not clipped. A flat series sits in the middle; fewer than two points draw nothing.
 */
export function sparkline(
  values: readonly number[],
  width: number,
  height: number,
  pad = 2,
): Sparkline | null {
  if (values.length < 2) return null;
  const min = Math.min(...values);
  const max = Math.max(...values);
  const span = max - min;
  const inner = height - pad * 2;
  const points = values.map((v, i) => ({
    x: Math.round((i / (values.length - 1)) * width * 100) / 100,
    y:
      span === 0
        ? height / 2
        : Math.round((pad + (1 - (v - min) / span) * inner) * 100) / 100,
  }));
  const line = points
    .map((p, i) => `${i === 0 ? 'M' : 'L'}${p.x},${p.y}`)
    .join(' ');
  return { points, line, area: `${line} L${width},${height} L0,${height} Z` };
}
