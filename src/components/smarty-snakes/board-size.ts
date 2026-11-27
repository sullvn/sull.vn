/**
 * Board size in cells, filling a column's width and up to a maximum height
 *
 * Cells are square and close to `cellSize`, but shrink on narrow columns to
 * fit `minWidth` cells across. Rows never drop below the fewest a snake can
 * spawn in.
 *
 */
export function boardSize(columnWidth: number, maxHeight: number): BoardSize {
  const width = Math.max(minWidth, Math.round(columnWidth / cellSize))
  const height = Math.max(minHeight, Math.floor(maxHeight / (columnWidth / width)))

  return { width, height }
}

/**
 * A board's size in cells
 */
export interface BoardSize {
  width: number
  height: number
}

/**
 * Target cell size in CSS pixels, big enough to see and tap
 */
const cellSize = 18

/**
 * Fewest cells across, so phones still have room to play
 */
const minWidth = 24

/**
 * Fewest rows, so a snake has room to spawn away from the walls
 */
const minHeight = 9
