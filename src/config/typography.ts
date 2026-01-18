import { fontScale } from '../utils/typography'

interface Typography {
  fontSize: string
  paragraphGap: string
}

export const headings = {
  h1: { fontSize: fontScale(6) },
  h2: { fontSize: fontScale(5) },
  h3: { fontSize: fontScale(4) },
  h4: { fontSize: fontScale(3) },
  h5: { fontSize: fontScale(2) },
  h6: { fontSize: fontScale(1) },
}

export const prose: Typography = {
  fontSize: fontScale(1),
  paragraphGap: '2em',
}

export const logo = {
  fontSize: fontScale(12),
}
