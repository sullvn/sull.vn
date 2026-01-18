import { defineCollection, z } from 'astro:content'
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
