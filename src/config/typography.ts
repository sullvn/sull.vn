import { fontScale } from '../utils/typography'
import { GOLDEN_RATIO } from '../utils/math'

/**
 * Base font size (px)
 *
 *
 * Directly used for:
 *
 * - Prose text size
 * - Definition of `rem`, for various text-relative sizing
 *
 *
 * Should be the default browser size (16px) or something
 * super close.
 *
 * In this instance, we use 17px as it's what Apple uses
 * for the same font stack and I'm a talentless sheep.
 *
 */
export const baseFontSizePx = 17

/**
 * Root font size (in non-adjusted rem)
 *
 *
 * There's a problem if you just set the root font size to
 * the base font size, in pixels. This tends to override
 * user font preferences, if they've set their browser to
 * display larger or smaller text.
 *
 * A way around this is to set it in em, rem, or percent.
 *
 * Setting it using `rem` feels kinda weird, but I like it,
 * as it's basically saying "define the current rem in the
 * old rem".
 *
 */
export const rootFontSize = `${baseFontSizePx / 16}rem`

interface Typography {
  fontSize: string
  lineHeight: string
  paragraphGap: string
  listGap: string
  listMarginBlock: string
  headingGap: string
  headingGapTop: string
  codeFontSize: string
  codeBlockGap: string
}

export const headings = {
  h1: { fontSize: fontScale(5) },
  h2: { fontSize: fontScale(4) },
  h3: { fontSize: fontScale(3) },
  h4: { fontSize: fontScale(2) },
  h5: { fontSize: fontScale(1) },
  h6: { fontSize: fontScale(0) },
}

const paragraphGapEm = 2
const listGapEm = 1.5
const listMarginBlockEm = paragraphGapEm * GOLDEN_RATIO
const headingGapEm = paragraphGapEm
const headingGapTopEm = 2 * GOLDEN_RATIO

export const prose: Typography = {
  fontSize: fontScale(0),
  lineHeight: 'calc(1em + 0.5rem)',
  paragraphGap: `${paragraphGapEm}em`,
  listGap: `${listGapEm}em`,
  listMarginBlock: `${listMarginBlockEm}em`,
  headingGap: `${headingGapEm}em`,
  headingGapTop: `${headingGapTopEm}em`,
  codeFontSize: fontScale(-1),
  codeBlockGap: '8em',
}

export const backArrow = {
  fontSize: headings.h1.fontSize,
}

export const notFound = {
  fontSize: fontScale(8),
}

/**
 * Light-on-dark is punchier. Reduce weight a bit to compensate.
 */
export const headingFontWeight = {
  light: 900,
  dark: 800,
}
