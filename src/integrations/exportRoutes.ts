import type { AstroIntegration } from 'astro'
import { writeFile } from 'node:fs/promises'

/**
 * Astro integration that writes a `routes.json` file to the build output
 * directory after each build. The file contains an array of all generated
 * route pathnames (e.g. `["/", "/resume/", "/babbles/my-post/"]`).
 *
 * This is consumed by Playwright visual regression tests to automatically
 * discover and screenshot every page without hardcoding routes.
 */
export function exportRoutes(): AstroIntegration {
  return {
    name: 'export-routes',
    hooks: {
      'astro:build:done': async ({ pages, dir }) => {
        const routes = pages.map((p) => `/${p.pathname}`)
        await writeFile(new URL('./routes.json', dir), JSON.stringify(routes))
      },
    },
  }
}
