import { defineConfig, devices } from '@playwright/test'

export default defineConfig({
  testDir: './tests',
  fullyParallel: true,
  reporter: 'html',
  // Safari/WebKit omitted: nixpkgs' WPE backend requires EGL display, which
  // isn't available in headless mode on NixOS without extra plumbing.
  projects: [
    { name: 'chrome-desktop', use: { ...devices['Desktop Chrome'] } },
    { name: 'firefox-desktop', use: { ...devices['Desktop Firefox'] } },
    {
      name: 'chrome-mobile',
      use: { ...devices['iPhone 15'], defaultBrowserType: 'chromium' },
    },
    {
      name: 'firefox-mobile',
      use: {
        ...devices['Desktop Firefox'],
        viewport: devices['iPhone 15'].viewport,
      },
    },
  ],
  webServer: {
    command: 'pnpm build && pnpm preview',
    port: 4321,
    reuseExistingServer: true,
  },
})
