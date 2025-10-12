import { defineConfig } from 'astro/config';

import svelte from '@astrojs/svelte';

export default defineConfig({
  integrations: [svelte()],
  server: {
    host: true, // Listen on all addresses
    allowedHosts: true, // Accept connections as any host
  },
  vite: {
    css: {
      transformer: "lightningcss",
    },
  },
});
