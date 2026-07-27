---
type: "question"
status: draft
date: 2026-04-26 1777203876
tags: [question, sitecore, jss, layout-service]
---

# 🎯 What Must You Do When Adding a Placeholder in Sitecore-First? - L2

## Question

> **Core Question:** What should you do when adding a placeholder to a rendering in the Sitecore-first workflow?
> **Follow-up:** What breaks if you forget this step?

## 💡 Quick Answer (30 seconds)

- Add the placeholder to the `Layout Service Placeholders` field on the rendering item.
- Otherwise, Layout Service will not expose the child placeholder in its JSON output.
- The front end will not receive renderings added into that placeholder.

## 📖 Detailed Explanation

This is one of the most explicit rules in the Sitecore-first documentation. When you create a Json Rendering and expose a child placeholder in code, you must also register that placeholder in the rendering item's `Layout Service Placeholders` field. That field tells Layout Service which placeholders to resolve and include in the response.

If you skip it, the placeholder may exist in the front-end component, but Layout Service will not return its child renderings. The result is a confusing mismatch where authors can add components in Sitecore, yet the rendering host never receives them.

```text
Rendering item
  -> Layout Service Placeholders = child-placeholder
Front end
  -> <Placeholder name="child-placeholder" ... />
```

Official proof: [Sitecore-first development workflow](https://doc.sitecore.com/xp/en/developers/hd/latest/sitecore-headless-development/sitecore-first-development-workflow.html#sitecore-first-development-workflow), [Layout Service data](https://doc.sitecore.com/xp/en/developers/hd/latest/sitecore-headless-development/layout-service.html#layout-service-data).

## 🧪 Practice Exercise

Describe the two changes needed to expose a new `promo-items` placeholder from a Json Rendering: one in Sitecore and one in the front-end app.

## 🔗 Related Topics

- [[2026-04-22-sitecore-placeholders]]
- [[2026-04-26-nextjs-placeholder-needs-a-component]]
- Layout Service output

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
