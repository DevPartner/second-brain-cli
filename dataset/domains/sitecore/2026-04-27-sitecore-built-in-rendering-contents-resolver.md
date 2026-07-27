---
type: "zknotes"
date: 2026-04-27 1777316502.186
tags: [sitecore, sitecore-headless, layout-service]
---

# Sitecore Built-in Rendering Contents Resolver

## Source

From: Copilot

## Summary

A built-in Rendering Contents Resolver is a predefined Layout
Service component that shapes Sitecore item data into JSON for a
rendering. You use it when the standard output format already matches
what the front end needs.

## Key Ideas

- Built-in resolvers avoid custom code for common rendering payloads.
- They control which item or fields are serialized into the response.
- If the built-in options are not enough, implement a custom resolver,
  typically via [[2026-04-27-sitecore-irenderingcontentsresolver|IRenderingContentsResolver]].

## Related

- [[2026-04-22-sitecore-presentation-layer]]
