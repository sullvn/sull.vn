import { after, before, describe, it } from 'node:test'
import assert from 'node:assert'
import { formatDate, toIsoDay } from './date.ts'

describe('formatDate', () => {
  it('uses the UTC calendar day', () => {
    assert.strictEqual(formatDate('2018-08-24T01:29:14.000Z'), 'August 24, 2018')
  })

  it('agrees with toIsoDay', () => {
    const date = '2015-10-30T02:00:00.000Z'
    assert.strictEqual(toIsoDay(date), '2015-10-30')
    assert.strictEqual(formatDate(date), 'October 30, 2015')
  })

  const originalTZ = process.env.TZ

  before(() => {
    // Simulate build machine with different time zone
    process.env.TZ = 'America/Chicago'
  })

  after(() => {
    if (originalTZ === undefined) {
      delete process.env.TZ
    } else {
      process.env.TZ = originalTZ
    }
  })
})
