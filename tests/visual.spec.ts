import { test, expect } from '@playwright/test'
import { readFileSync } from 'node:fs'

const routes: string[] = JSON.parse(
  readFileSync('dist/routes.json', 'utf-8'),
)

for (const route of routes) {
  test(`visual snapshot: ${route}`, async ({ page }) => {
    await page.goto(route)
    await page.evaluate(async () => {
      for (const iframe of document.querySelectorAll('iframe')) {
        iframe.remove()
      }
      const imgs = document.querySelectorAll<HTMLImageElement>(
        'img[loading="lazy"]',
      )
      for (const img of imgs) {
        img.setAttribute('loading', 'eager')
      }
      await Promise.all([...imgs].map((img) => img.decode().catch(() => {})))

      // CSS columns/masonry layout reflows AFTER img.decode() resolves.
      // Double-rAF: first frame = layout scheduled, second = layout painted.
      await new Promise<void>((r) =>
        requestAnimationFrame(() => requestAnimationFrame(() => r())),
      )
    })
    await expect(page).toHaveScreenshot({ fullPage: true })
  })
}
