import { pressNudge, wipeReveal } from '../../lib/motion';

/**
 * PROVISIONAL BugLenz wordmark for the design system (brand overlay, OpenSpec
 * 007; decision D9 open). Replaces the upstream's `wordmark.tsx` at build time
 * and keeps the exported `Wordmark` and its props, so nothing that imports it
 * changes. A typographic mark until the real artwork replaces it.
 */
const WORD = 'BugLenz';
const TEXT_PROPS = {
  x: 0,
  y: 0,
  fontSize: 88,
  fontWeight: 800,
  fontFamily:
    "ui-sans-serif, system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif",
  textLength: 390,
  lengthAdjust: 'spacingAndGlyphs' as const,
};

export function Wordmark({
  className,
  still,
}: {
  className?: string;
  /**
   * Drop the hover entirely. For surfaces that are already lime, where filling
   * the mark with lime fills it with nothing.
   */
  still?: boolean;
}) {
  return (
    <svg
      className={still ? className : `group ${pressNudge} ${className ?? ''}`}
      width="390"
      height="104"
      viewBox="0 -78 390 104"
      xmlns="http://www.w3.org/2000/svg"
      role="img"
      aria-label="BugLenz"
    >
      <text {...TEXT_PROPS} className="fill-current">
        {WORD}
      </text>
      {!still && (
        <g className={`fill-fg-brand ${wipeReveal}`}>
          <text {...TEXT_PROPS}>{WORD}</text>
        </g>
      )}
    </svg>
  );
}
