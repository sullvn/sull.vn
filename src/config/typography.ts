import { fontScale } from '../utils/typography'
import { GOLDEN_RATIO } from '../utils/math'

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
  fontSize: fontScale(1),
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
  fontSize: fontScale(6),
}
