# Craftbook Blog

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

1. Add new blog posts as markdown files in the `src/content/articles/` directory
2. Use the following frontmatter format:

```markdown
---
title: "Your Post Title"
description: "A brief description of your post"
pubDate: 2023-05-01
heroImage: "/images/your-image.jpg" # Optional
tags: ["tag1", "tag2"] # Optional
---

Your markdown content here...
```

3. For mathematical equations, use LaTeX syntax:
   - Inline math: `$E = mc^2$`
   - Block math: `$$E = mc^2$$`

## Deployment

This site is configured for deployment on Cloudflare Pages:

1. Connect your GitHub repository to Cloudflare Pages
2. Use the following build settings:
   - Build command: `npm run build`
   - Build output directory: `dist`
3. Set up your custom domain in the Cloudflare Pages dashboard

## License

MIT