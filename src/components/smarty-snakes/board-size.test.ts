import { describe, it } from 'node:test'
import assert from 'node:assert'
import { boardSize } from './board-size.ts'

describe('boardSize', () => {
  it('fills the column with cells close to 18 pixels', () => {
    assert.deepStrictEqual(boardSize(360, 560), { width: 20, height: 31 })
  })

  it('fits whole rows under the maximum height', () => {
    assert.deepStrictEqual(boardSize(864, 600), { width: 48, height: 33 })
  })

  it('keeps at least 9 cells on each side', () => {
    assert.deepStrictEqual(boardSize(100, 50), { width: 9, height: 9 })
  })
})
