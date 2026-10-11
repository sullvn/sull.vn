import { defineConfig } from '@playwright/test'
import type { VisualOptions } from './tests/visual.spec'

const previewPort = 4322
const previewURL = `http://127.0.0.1:${previewPort}`

interface VisualCase {
  name: string
  route: string
  browserName: 'chromium' | 'firefox' | 'webkit'
  width: number
  colorScheme: 'light' | 'dark'
  height?: number
  media?: 'screen' | 'print'
}

const visualCases: VisualCase[] = [
  {
    name: 'home-chromium-1280-light',
    route: '/',
    browserName: 'chromium',
    width: 1280,
    colorScheme: 'light',
  },
  {
    name: 'home-webkit-400-dark',
    route: '/',
    browserName: 'webkit',
    width: 400,
    colorScheme: 'dark',
  },
  {
    name: 'resume-chromium-800-light',
    route: '/resume',
    browserName: 'chromium',
    width: 800,
    colorScheme: 'light',
  },
  {
    name: 'resume-firefox-400-light',
    route: '/resume',
    browserName: 'firefox',
    width: 400,
    colorScheme: 'light',
  },
  {
    name: 'lunchables-firefox-1280-dark',
    route: '/babbles/could-we-code-with-lunchables',
    browserName: 'firefox',
    width: 1280,
    colorScheme: 'dark',
  },
  {
    name: 'spelunking-chromium-400-light',
    route: '/babbles/notes-from-hyper-dimensional-spelunking',
    browserName: 'chromium',
    width: 400,
    colorScheme: 'light',
  },
  {
    name: 'resume-chromium-print-a4',
    route: '/resume',
    browserName: 'chromium',
    width: 716,
    height: 1026,
    colorScheme: 'light',
    media: 'print',
  },
]

export default defineConfig<VisualOptions>({
  testDir: './tests',
  fullyParallel: true,
  reporter: [['html', { host: '0.0.0.0' }]],
  use: {
    baseURL: previewURL,
  },
  projects: visualCases.map(
    ({ name, route, browserName, width, height = 800, colorScheme, media = 'screen' }) => ({
      name,
      use: {
        route,
        media,
        browserName,
        viewport: { width, height },
        colorScheme,
      },
    }),
  ),
  webServer: {
    command: `pnpm exec astro build && pnpm exec astro preview --port ${previewPort} --ignore-lock`,
    url: previewURL,

    // A fresh build must encode the site's images before preview can start.
    timeout: 5 * 60 * 1000,
    reuseExistingServer: false,

    // Keep Astro in the foreground so Playwright controls the preview's lifetime.
    env: { ASTRO_PREVIEW_BACKGROUND: '1' },
  },
})
