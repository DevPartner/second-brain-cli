---
type: "zknotes"
date: 2026-04-27 1777318521.773
tags: [sitecore, sitecore-headless]
---

# Sitecore IRenderingContentsResolver

## Source

From: GitHub Copilot

## Summary

`IRenderingContentsResolver` is the Sitecore Layout Service extension point that controls what JSON a rendering returns. It is used when default datasource/context serialization is not enough and you need custom output shaping for headless components.

## Key insights

- Scope and purpose:
    `IRenderingContentsResolver` is invoked during Layout Service rendering and lets you fully customize serialized rendering output via `ResolveContents(...)`.
    Source: <https://doc.sitecore.com/xp/developers/hd/latest/sitecore-headless-development/en/customizing-the-layout-service-rendering-output.html#creating-an-irenderingcontentsresolver-interface>

- Where it is configured:
    The default resolver is set in Layout Service named configuration (`/sitecore/layoutService/configurations/.../rendering/renderingContentsResolver`) and can be overridden per rendering item via the `Rendering Contents Resolver` field.
    Source: <https://doc.sitecore.com/xp/en/developers/hd/latest/sitecore-headless-development/using-a-custom-layout-service-configuration-with-jss.html#layout-service-configuration>

- Practical rule:
    Prefer built-in resolvers first (Datasource, Context, Children, Folder Filter), and only implement custom resolvers when needed because Sitecore recommends no-code options for compatibility and maintainability.
    Source: <https://doc.sitecore.com/xp/developers/hd/latest/sitecore-headless-development/en/customizing-the-layout-service-rendering-output.html#choosing-or-configuring-a-built-in-rendering-contents-resolver>

## Related

- [[2026-04-22-sitecore-presentation-layer]]
- [[2026-04-27-sitecore-built-in-rendering-contents-resolver]]
