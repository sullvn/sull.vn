import { GOLDEN_RATIO } from './math.ts'

/**
 * Compute a font size from a typographic scale.
 *
 * A typographic scale provides harmonious font sizes using the formula:
 *
 *     f_i = base × ratio^(i/notes)
 *
 * The classic typographic scale uses ratio=2 and notes=5, meaning the font
 * size doubles every 5 steps. With base=1:
 *
 *     i   |  size
 *    -----+--------
 *     -5  |  0.500
 *     -4  |  0.574
 *     -3  |  0.660
 *     -2  |  0.758
 *     -1  |  0.871
 *      0  |  1.000  ← base
 *      1  |  1.149
 *      2  |  1.320
 *      3  |  1.516
 *      4  |  1.741
 *      5  |  2.000  ← base × ratio
 *     10  |  4.000  ← base × ratio²
 *
 * @see https://spencermortensen.com/articles/typographic-scale/
 */
export function typographyScale(i: number, options: TypographyScaleOptions): number {
  const { base, ratio, notes } = options

  return base * ratio ** (i / notes)
}

export interface TypographyScaleOptions {
  base: number
  ratio: number
  notes: number
}

const scaleOptions = {
  base: 1,
  ratio: GOLDEN_RATIO ** 3, // ≈ 4.236
  notes: 6,
}

export function fontScale(position: number): string {
  return `${typographyScale(position, scaleOptions)}rem`
}
