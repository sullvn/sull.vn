import { defineConfig } from 'astro/config';

import svelte from '@astrojs/svelte';
import mdx from '@astrojs/mdx';
import remarkMath from 'remark-math';
import rehypeKatex from 'rehype-katex';

export default defineConfig({
  markdown: {
    remarkPlugins: [remarkMath],
    rehypePlugins: [rehypeKatex],
  },
  integrations: [svelte(), mdx()],
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