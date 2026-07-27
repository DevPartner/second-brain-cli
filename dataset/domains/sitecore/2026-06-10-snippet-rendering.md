---
type: "zknotes"
description: "Explain What is snippet rendering?"
tags: [sitecore, rendering, component]
---

# What is snippet rendering?

## Sources

- [Composite renderings (SXA Users)](https://doc.sitecore.com/xp/en/users/sxa/latest/sitecore-experience-accelerator/composite-renderings.html#composite-renderings)
- [Composites overview (SXA Users)](https://doc.sitecore.com/xp/en/users/sxa/latest/sitecore-experience-accelerator/composites.html#composites)
- [Recommendations: Using renderings (SXA Developers)](https://doc.sitecore.com/xp/developers/sxa/latest/sitecore-experience-accelerator/en/recommendations--using-renderings.html#recommendations-using-renderings)

## Summary

In Sitecore SXA, a Snippet rendering is a composite rendering that groups multiple renderings into one reusable unit. Authors can design its inner renderings in Experience Editor, reuse the snippet in different page locations, and use it multiple times on the same page. It is intended for repeated page sections made of several components.

## Key Insights

1. **Composite and reusable**: Sitecore defines Snippet as a reusable group of renderings (a composite rendering), not a single standalone rendering.

2. **Designed in Experience Editor**: The grouped renderings inside a snippet can be designed separately, then reused where needed.

3. **Different from partial design**: Snippets are extensible, support styling changes, and can be used multiple times on the same page.

## Assumptions

- The question refers to Sitecore SXA/XM Cloud usage of Snippet as documented in SXA composite renderings.
- "Copied into placeholders" is interpreted as adding configured snippet content/groups into page layout areas during authoring.

## Related

- [[2026-04-22-sitecore-presentation-layer]]
- [[2026-04-22-sitecore-renderings-components]]
