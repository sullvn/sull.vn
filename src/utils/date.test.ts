import { after, before, describe, it } from 'node:test'
import assert from 'node:assert'
import { formatDate, toIsoDay } from './date.ts'

describe('formatDate', () => {
  // Build machines in different time zones must agree, so test from one west of UTC.
  const originalTZ = process.env.TZ
  before(() => {
    process.env.TZ = 'America/Chicago'
  })
  after(() => {
    if (originalTZ === undefined) {
      delete process.env.TZ
    } else {
      process.env.TZ = originalTZ
    }
  })

  it('uses the UTC calendar day', () => {
    assert.strictEqual(formatDate('2018-08-24T01:29:14.000Z'), 'August 24, 2018')
  })

  it('agrees with toIsoDay', () => {
    const date = '2015-10-30T02:00:00.000Z'
    assert.strictEqual(toIsoDay(date), '2015-10-30')
    assert.strictEqual(formatDate(date), 'October 30, 2015')
  })
})
