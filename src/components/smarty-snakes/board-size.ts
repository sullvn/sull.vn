/**
 * Board size in cells, filling a column's width and up to a maximum height
 *
 * Cells are square and close to `cellSize`. Each side keeps the fewest
 * cells a snake can spawn on, even if that means smaller cells.
 *
 */
export function boardSize(columnWidth: number, maxHeight: number): BoardSize {
  const width = Math.max(minCells, Math.round(columnWidth / cellSize))
  const height = Math.max(minCells, Math.floor(maxHeight / (columnWidth / width)))

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
 * Fewest cells on a side, so a snake has room to spawn away from the walls
 */
const minCells = 9
