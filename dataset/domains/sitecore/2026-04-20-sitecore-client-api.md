---
type: "zknotes"
date: 2026-04-20 1776687344.603
tags: []
---

# SitecoreClient API

## Source

From: [Website](https://learning.sitecore.com/partners/learn/learning-plans/119/sitecoreai-cms-for-developers/courses/1597/wed-development-with-sitecoreai-cms/lessons/3055:1218/web-development-with-sitecoreai-cms)

## Summary

**SitecoreClient** is the central, unified API used by Content SDK applications to communicate with SitecoreAI CMS. It abstracts all the underlying service calls (Layout, Dictionary, Site Path, GraphQL, Personalization, etc.) into a single, typed, modern API.

## Key Ideas

- **Layout**: Fetch page & component layout via GraphQL Layout Service
- **i18n**: Retrieve dictionary phrases for localization
- **Paths**: Generate site paths for SSG/ISR/exports
- **SEO**: Produce sitemap.xml and robots.txt
- **Personalization**: Carry variant/context IDs end‑to‑end
- **GraphQL**: provide `getData<T>()` for typed custom queries with shared retries/headers

## Related

- [[2026-04-20-what-is-sitecore-content-sdk|ContentSDK]]
- [[2026-04-20-sitecore-apis]]
