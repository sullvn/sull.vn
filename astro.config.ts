import { fileURLToPath } from 'node:url'
import { defineConfig } from 'astro/config'
import { escapePath } from 'tinyglobby'

import { unified } from '@astrojs/markdown-remark'
import svelte from '@astrojs/svelte'
import mdx from '@astrojs/mdx'
import remarkMath from 'remark-math'
import rehypeKatex from 'rehype-katex'

const root = escapePath(fileURLToPath(new URL('.', import.meta.url)))

export default defineConfig({
  publicDir: './src/public',
  image: {
    service: {
      entrypoint: 'astro/assets/services/sharp',
      config: {
        avif: { quality: 80 },
      },
    },
  },
  markdown: {
    processor: unified({
      remarkPlugins: [remarkMath],
      rehypePlugins: [rehypeKatex],
    }),
    shikiConfig: {
      themes: {
        light: 'material-theme-lighter',
        dark: 'material-theme-darker',
      },
    },
  },
  integrations: [svelte(), mdx()],
  build: {
    format: 'file',
  },
  server: {
    host: true, // Listen on all addresses
    allowedHosts: true, // Accept connections as any host
  },
  vite: {
    css: {
      transformer: 'lightningcss',
    },
    server: {
      watch: {
        ignored: [`${root}**/dist/**`, `${root}**/.*/**`],
      },
    },
  },
})
