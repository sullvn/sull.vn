import { test as base, expect } from '@playwright/test'

export interface VisualOptions {
  route: string
}

const test = base.extend<VisualOptions>({
  route: ['/', { option: true }],
})

test('page appearance', async ({ page, route }) => {
  const response = await page.goto(route)
  expect(response?.status()).toBe(200)

  await page.evaluate(async () => {
    const imgs = document.querySelectorAll<HTMLImageElement>('img')
    for (const img of imgs) {
      img.setAttribute('loading', 'eager')
    }
    await Promise.all([...imgs].map((img) => img.decode()))

    // Let layout settle after image decoding before capturing the page.
    await new Promise<void>((r) => requestAnimationFrame(() => requestAnimationFrame(() => r())))
  })

  await expect(page).toHaveScreenshot('page.png', { fullPage: true, animations: 'disabled' })
})
