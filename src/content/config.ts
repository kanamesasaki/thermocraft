import { defineCollection, z } from 'astro:content';

const articleSchema = z.object({
  title: z.string(),
  description: z.string(),
  pubDate: z.date(),
  updatedDate: z.date().optional(),
  heroImage: z.string().optional(),
  tags: z.array(z.string()).optional(),
});

const articlesCollection = defineCollection({
  type: 'content',
  schema: articleSchema,
});

const articlesJaCollection = defineCollection({
  type: 'content',
  schema: articleSchema,
});

export const collections = {
  articles: articlesCollection,
  'articles-ja': articlesJaCollection,
};
