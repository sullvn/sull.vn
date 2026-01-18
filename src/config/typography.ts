import { fontScale } from '../utils/typography'
import { GOLDEN_RATIO } from '../utils/math'

interface Typography {
  fontSize: string
  paragraphGap: string
  listGap: string
  headingGap: string
  headingGapTop: string
  codeFontSize: string
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
const headingGapEm = paragraphGapEm
const headingGapTopEm = 2 * GOLDEN_RATIO

export const prose: Typography = {
  fontSize: fontScale(1),
  paragraphGap: `${paragraphGapEm}em`,
  listGap: '1em',
  headingGap: `${headingGapEm}em`,
  headingGapTop: `${headingGapTopEm}em`,
  codeFontSize: fontScale(-1),
}

export const logo = {
  fontSize: fontScale(12),
}
