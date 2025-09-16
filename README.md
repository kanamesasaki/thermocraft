# ThermoCraft Blog

A static blog site built with Astro that supports markdown content with mathematical equations.

## Features

- Written in TypeScript
- Markdown content with equation support (KaTeX)
- Optimized for deployment on Cloudflare Pages
- Custom domain: craftbook.thermocraft.space
- Docker setup for consistent development environment

## Getting Started

### Prerequisites

- Docker and Docker Compose

### Development

1. Clone this repository
2. Start the development server with Docker:

```bash
docker-compose up
```

3. Open your browser and visit `http://localhost:4321` to see the blog.

### Creating New Content

1. Add new blog posts as markdown files in the `src/content/articles/` directory for English and `src/content/articles-ja/` for Japanese.
2. The article slug should be the same for both English and Japanese versions.
3. Use the following frontmatter format. The `tags` should be written in lowercase.

```markdown
---
title: ''
description: ''
pubDate: YYYY-MM-DD
updatedDate: YYYY-MM-DD
heroImage: ''
tags: ['thermal']
---

Your markdown content here...
```

## Deployment

This site is configured for deployment on Cloudflare Pages:

1. Connect your GitHub repository to Cloudflare Pages
2. Use the following build settings:
   - Build command: `npm run build`
   - Build output directory: `dist`
3. Set up your custom domain in the Cloudflare Pages dashboard
