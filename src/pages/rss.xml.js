import rss from "@astrojs/rss";
import { getCollection } from "astro:content";

export async function GET(context) {
  const posts = await getCollection("articles");
  return rss({
    title: "Craftbook",
    description:
      "Technical articles on programming, numerical analysis, and engineering topics",
    site: context.site,
    items: posts.map((post) => ({
      title: post.data.title,
      pubDate: post.data.pubDate,
      description: post.data.description,
      link: `/articles/${post.slug}/`,
    })),
  });
}
