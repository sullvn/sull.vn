import type { Cell, Input } from '#rust/smarty-snakes'

/**
 * Messages the page sends the game worker
 */
export type ToWorker = Start | Press

/**
 * Messages the game worker sends the page
 */
export type FromWorker = Ready | Frame

/**
 * Starts a game on a board `width` by `height` cells
 */
export interface Start {
  type: 'start'
  width: number
  height: number
}

/**
 * A key press or tap, by its name in `Input`
 *
 * Names rather than codes keep the page from loading the game's WASM.
 *
 */
export interface Press {
  type: 'press'
  input: keyof typeof Input
}

/**
 * The worker has loaded the game and is listening
 *
 * Messages sent before this are lost, as the worker only starts listening
 * once its WASM loads.
 *
 */
export interface Ready {
  type: 'ready'
}

/**
 * The board after a move, as one `Cell` code per cell, row by row
 */
export interface Frame {
  type: 'frame'
  cells: Uint8Array
}

/**
 * Names for each `Cell` code, in code order
 *
 * They double as CSS classes. The type ties each name to its code, so
 * renaming or reordering a `Cell` in the crate fails to compile here.
 *
 */
export const cellNames = [
  'empty',
  'food',
  'player-tail',
  'player-head',
  'bot-tail',
  'bot-head',
] as const satisfies CellNames

/**
 * Each `Cell` code paired with its name in kebab case
 */
type CellNames = {
  readonly [Name in keyof typeof Cell & string as (typeof Cell)[Name]]: KebabCase<Name>
}

/**
 * `PlayerTail` as `player-tail`
 */
type KebabCase<Name extends string> = Name extends `${infer First}${infer Rest}`
  ? Rest extends Uncapitalize<Rest>
    ? `${Lowercase<First>}${KebabCase<Rest>}`
    : `${Lowercase<First>}-${KebabCase<Rest>}`
  : Name
