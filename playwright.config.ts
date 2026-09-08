import { defineConfig } from '@playwright/test'
import type { VisualOptions } from './tests/visual.spec'

const previewPort = 4322
const previewURL = `http://127.0.0.1:${previewPort}`

const visualCases = [
  {
    name: 'home-chromium-1280-light',
    route: '/',
    browserName: 'chromium',
    width: 1280,
    colorScheme: 'light',
  },
  {
    name: 'home-chromium-400-dark',
    route: '/',
    browserName: 'chromium',
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
] as const

export default defineConfig<VisualOptions>({
  testDir: './tests',
  fullyParallel: true,
  reporter: [['html', { host: '0.0.0.0' }]],
  use: {
    baseURL: previewURL,
  },
  projects: visualCases.map(({ name, route, browserName, width, colorScheme }) => ({
    name,
    use: {
      route,
      browserName,
      viewport: { width, height: 800 },
      colorScheme,
    },
  })),
  webServer: {
    command: `pnpm build && pnpm preview --port ${previewPort} --ignore-lock`,
    url: previewURL,
    reuseExistingServer: false,
    // Keep Astro in the foreground so Playwright controls the preview's lifetime.
    env: { ASTRO_PREVIEW_BACKGROUND: '1' },
  },
})
