import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { defineConfig } from 'astro/config'
import { escapePath } from 'tinyglobby'

import { unified } from '@astrojs/markdown-remark'
import svelte from '@astrojs/svelte'
import mdx from '@astrojs/mdx'
import { rust } from './src/integrations/rust.ts'
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
      defaultColor: 'light-dark()',
    },
  },
  integrations: [svelte(), mdx(), rust()],
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
        ignored: [...gitignored(), `${root}**/.*/**`],
      },
    },
  },
})

function gitignored(): string[] {
  return readFileSync(new URL('.gitignore', import.meta.url), 'utf8')
    .split('\n')
    .filter((line) => line && !line.startsWith('#'))
    .flatMap((line) => {
      const path = line.replace(/\/$/, '')
      const glob = path.includes('/') ? `${root}${path.replace(/^\//, '')}` : `${root}**/${path}`
      return [glob, `${glob}/**`]
    })
}
