export function formatDate(isoDate: string): string {
  const dateObj = new Date(isoDate)
  return dateObj.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}

export function toTimestamp(value: unknown): number {
  return coerceDate(value).getTime()
}

export function toIsoDay(value: unknown): string {
  // Preserve existing semantics: convert to a UTC ISO calendar day string.
  // Use Intl.DateTimeFormat to avoid manual string slicing/splitting.
  const date = coerceDate(value)

  const parts = new Intl.DateTimeFormat('en-CA', {
    timeZone: 'UTC',
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
  }).formatToParts(date)

  const lookup = (type: string) => parts.find((p) => p.type === type)?.value

  const year = lookup('year')
  const month = lookup('month')
  const day = lookup('day')

  if (!year || !month || !day) {
    throw new Error(`Failed to format date: ${String(value)}`)
  }

  return `${year}-${month}-${day}`
}

function coerceDate(value: unknown): Date {
  const date = value instanceof Date ? value : new Date(value as never)

  if (Number.isNaN(date.getTime())) {
    throw new Error(`Invalid date input: ${String(value)}`)
  }

  return date
}
