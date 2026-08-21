// apps/registry/front-end/scripts/generate-sitemaps.mjs
import { mkdir, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const apiOrigin = process.env.SITEMAP_SOURCE_ORIGIN ?? 'http://127.0.0.1:6011';
const outputRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../public/sitemaps');
const sitemapPaths = [
  'index.xml',
  'en/users.xml',
  'en/organizations.xml',
  'en/packages.xml',
];

/** Fetches one XML sitemap from the database-backed registry API. */
async function fetchSitemap(pathname) {
  const response = await fetch(new URL(`/sitemaps/${pathname}`, apiOrigin));
  if (!response.ok) {
    throw new Error(`Sitemap request failed (${response.status}): ${pathname}`);
  }
  const contentType = response.headers.get('content-type') ?? '';
  if (!contentType.includes('application/xml')) {
    throw new Error(`Sitemap response is not XML (${contentType}): ${pathname}`);
  }
  const content = (await response.text()).trim();
  const hasSitemapRoot =
    content.includes('<urlset') || content.includes('<sitemapindex');
  if (!content.startsWith('<?xml') || !hasSitemapRoot) {
    throw new Error(`Sitemap response is malformed: ${pathname}`);
  }
  return `${content}\n`;
}

await Promise.all(
  sitemapPaths.map(async (pathname) => {
    const outputPath = resolve(outputRoot, pathname);
    await mkdir(dirname(outputPath), { recursive: true });
    await writeFile(outputPath, await fetchSitemap(pathname), 'utf8');
  }),
);

console.log(`Generated ${sitemapPaths.length} sitemap files in ${outputRoot}`);
