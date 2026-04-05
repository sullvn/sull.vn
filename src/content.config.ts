import { defineCollection } from 'astro:content'
import { z } from 'astro/zod'
import { glob } from 'astro/loaders'

const babbles = defineCollection({
  loader: glob({ pattern: '**/*.mdx', base: './src/babbles' }),
  schema: z.object({
    title: z.string(),
    date: z.coerce.date(),
    slug: z.string(),
  }),
})

export const collections = { babbles }
