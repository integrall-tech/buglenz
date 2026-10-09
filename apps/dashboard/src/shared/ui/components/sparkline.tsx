import { useId } from 'react';
import { sparkline } from '@/shared/lib/sparkline';

interface SparklineProps {
  values: readonly number[];
  /** Said once, for a screen reader; the line itself carries no readable data. */
  label: string;
  width?: number;
  height?: number;
  className?: string;
}

/** A trend line with a soft fill, in the first chart colour. A hint of shape, not a chart. */
export function Sparkline({
  values,
  label,
  width = 96,
  height = 36,
  className,
}: SparklineProps) {
  const gradient = useId();
  const shape = sparkline(values, width, height, 3);
  if (!shape) return null;
  const last = shape.points[shape.points.length - 1];

  return (
    <svg
      role="img"
      aria-label={label}
      viewBox={`0 0 ${width} ${height}`}
      width={width}
      height={height}
      className={className}
    >
      <defs>
        <linearGradient id={gradient} x1="0" x2="0" y1="0" y2="1">
          <stop offset="0" stopColor="var(--chart-2)" stopOpacity="0.28" />
          <stop offset="1" stopColor="var(--chart-2)" stopOpacity="0" />
        </linearGradient>
      </defs>
      <path d={shape.area} fill={`url(#${gradient})`} />
      <path
        d={shape.line}
        fill="none"
        stroke="var(--chart-2)"
        strokeWidth="1.75"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
      {last ? (
        <circle cx={last.x} cy={last.y} r="2.5" fill="var(--chart-2)" />
      ) : null}
    </svg>
  );
}
