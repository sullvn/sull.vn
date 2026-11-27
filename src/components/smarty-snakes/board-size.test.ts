import { describe, it } from 'node:test'
import assert from 'node:assert'
import { boardSize } from './board-size.ts'

describe('boardSize', () => {
  it('fills wide columns with cells close to 18 pixels', () => {
    assert.deepStrictEqual(boardSize(864, 600), { width: 48, height: 33 })
  })

  it('shrinks cells on narrow columns to fit 24 across', () => {
    assert.deepStrictEqual(boardSize(360, 560), { width: 24, height: 37 })
  })

  it('keeps at least 9 rows', () => {
    assert.deepStrictEqual(boardSize(240, 50), { width: 24, height: 9 })
  })
})
