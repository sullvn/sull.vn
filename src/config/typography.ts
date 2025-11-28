import { GOLDEN_RATIO } from "../utils/math";
import { typographyScale } from "../utils/typography";

const scaleOptions = {
  base: 1,
  ratio: Math.pow(GOLDEN_RATIO, 3), // ≈ 4.236
  notes: 6,
};

function fontScale(position: number): string {
  return `${typographyScale(position, scaleOptions)}rem`;
}

export const headings = {
  h1: { fontSize: fontScale(6) },
  h2: { fontSize: fontScale(5) },
  h3: { fontSize: fontScale(4) },
  h4: { fontSize: fontScale(3) },
  h5: { fontSize: fontScale(2) },
  h6: { fontSize: fontScale(1) },
};

export const prose: Typography = {
  fontSize: fontScale(1),
  paragraphGap: "2em",
};

interface Typography {
  fontSize: string;
  paragraphGap: string;
}
