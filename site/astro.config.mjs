// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import sitemap from '@astrojs/sitemap';

export default defineConfig({
  site: 'https://mailify.donilite.me',
  trailingSlash: 'never',

  integrations: [
    starlight({
      title: 'Mailify',
      description: 'Self-hosted, theme-aware transactional mail server in Rust.',
      logo: {
        light: './public/logo.png',
        dark: './public/logo-white.png',
        alt: 'Mailify',
        replacesTitle: false,
      },
      favicon: '/favicon.svg',
      customCss: [
        './src/styles/tokens.css',
        './src/styles/global.css',
      ],
      social:[
        { icon: 'github', label: 'GitHub', href: 'https://github.com/donilite/mailify' },
      ],
      editLink: {
        baseUrl: 'https://github.com/donilite/mailify/edit/master/',
      },
      head: [
        {
          tag: 'meta',
          attrs: { property: 'og:image', content: 'https://mailify.donilite.me/og-default.svg' },
        },
        {
          tag: 'meta',
          attrs: { name: 'twitter:card', content: 'summary_large_image' },
        },
        {
          tag: 'script',
          attrs: { type: 'application/ld+json' },
          content: JSON.stringify({
            '@context': 'https://schema.org',
            '@type': 'SoftwareApplication',
            name: 'Mailify',
            applicationCategory: 'DeveloperApplication',
            operatingSystem: 'Linux, macOS, Windows',
            offers: { '@type': 'Offer', price: '0', priceCurrency: 'USD' },
            description: 'Self-hosted, theme-aware transactional mail server in Rust.',
            url: 'https://mailify.donilite.me',
          }),
        },
      ],
      sidebar: [
        {
          label: 'Getting started',
          items: [
            { label: 'Overview', link: '/docs' },
            { label: 'Installation', link: '/docs/getting-started/installation' },
            { label: 'Quickstart', link: '/docs/getting-started/quickstart' },
            { label: 'Concepts', link: '/docs/getting-started/concepts' },
          ],
        },
        {
          label: 'Guides',
          items: [
            { label: 'Configure SMTP', link: '/docs/guides/configure-smtp' },
            { label: 'Configure theme', link: '/docs/guides/configure-theme' },
            { label: 'Auth & tokens', link: '/docs/guides/auth-and-tokens' },
            { label: 'Per-job SMTP override', link: '/docs/guides/per-job-smtp-override' },
            { label: 'Deploy with Docker', link: '/docs/guides/deploy-docker' },
          ],
        },
        {
          label: 'Reference',
          items: [
            { label: 'Configuration', link: '/docs/reference/config' },
            { label: 'HTTP API', link: '/docs/reference/http-api' },
            { label: 'CLI', link: '/docs/reference/cli' },
            { label: 'Template contract', link: '/docs/reference/template-contract' },
          ],
        },
        {
          label: 'Troubleshooting',
          items: [
            { label: 'Common errors', link: '/docs/troubleshooting/common-errors' },
            { label: 'FAQ', link: '/docs/troubleshooting/faq' },
            { label: 'Debugging', link: '/docs/troubleshooting/debugging' },
          ],
        },
        {
          label: 'Contributing',
          items: [
            { label: 'Overview', link: '/docs/contributing/overview' },
            { label: 'Architecture', link: '/docs/contributing/architecture' },
            { label: 'Dev setup', link: '/docs/contributing/dev-setup' },
          ],
        },
      ],
    }),
    sitemap(),
  ],
});
