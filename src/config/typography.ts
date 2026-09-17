import { fontScale } from '../utils/typography'
import { GOLDEN_RATIO } from '../utils/math'

// Follow apple.com's 17px body size; our system font stack uses the same
// Apple system font family on Apple devices.
export const baseFontSizePx = 17

// Express the design target relative to a 16px default, preserving user font preferences.
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
  h1: { fontSize: fontScale(6) },
  h2: { fontSize: fontScale(5) },
  h3: { fontSize: fontScale(4) },
  h4: { fontSize: fontScale(3) },
  h5: { fontSize: fontScale(2) },
  h6: { fontSize: fontScale(1) },
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
