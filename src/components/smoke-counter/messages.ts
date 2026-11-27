/**
 * Messages the page sends the counter worker
 */
export type ToWorker = Increment

/**
 * Messages the counter worker sends the page
 */
export type FromWorker = Count

/**
 * Asks the worker to add one
 */
export interface Increment {
  type: 'increment'
}

/**
 * The worker's count after a change
 */
export interface Count {
  type: 'count'
  value: number
}
