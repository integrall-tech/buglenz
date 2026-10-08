/**
 * PROVISIONAL BugLenz wordmark (brand overlay, OpenSpec 007; decision D9 open).
 *
 * Replaces the upstream's `rustrak-wordmark.tsx` at build time. The file keeps
 * the upstream's name and the exported `RustrakWordmark` (same props) so no
 * importer changes; it is a typographic mark until the real artwork arrives,
 * and it is replaced by dropping the final SVG paths in here.
 *
 * The hover classes below are spelled out in full on purpose: Tailwind finds
 * them by scanning source text, so a class assembled at run time would ship
 * unstyled while the build stays green.
 */
const FILL =
  '[clip-path:inset(0_100%_0_0)] transition-[clip-path] group-hover:[clip-path:inset(0_0_0_0)] motion-reduce:transition-none';

const PRESS =
  'transition-transform active:translate-y-px motion-reduce:transition-none';

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

export function RustrakWordmark({
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
      className={still ? className : `group ${PRESS} ${className ?? ''}`}
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
        <g className={`fill-primary ${FILL}`}>
          <text {...TEXT_PROPS}>{WORD}</text>
        </g>
      )}
    </svg>
  );
}
