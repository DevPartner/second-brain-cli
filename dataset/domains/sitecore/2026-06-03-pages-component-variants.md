---
type: "question"
status: draft
date: 2026-06-03 1780488000
tags: [question, sitecore, xm-cloud, pages, component-variants]
---

# 🎯 Component Variants in XM Cloud Pages - L3

## Question

> **Core Question:** What problem do component variants solve in XM Cloud Pages, and when should you use them?
> **Follow-up:** How do variants compare to creating separate components for each visual style?

## 💡 Quick Answer (30 seconds)

- Component variants let you reuse one rendering with multiple presentation options.
- They reduce duplication and keep content structure consistent across styles.
- Variants are best when behavior is similar but layout or styling needs to change.

## 📖 Detailed Explanation

In XM Cloud Pages, component variants provide a controlled way to offer multiple visual outputs from a single component definition. This helps teams avoid creating many near-duplicate renderings just to support style differences such as "compact", "featured", or "hero" views.

A good variant strategy improves maintainability. Authors can switch presentation modes without changing the underlying data model, while developers keep one component contract and a clear set of variant-specific templates. This also improves governance because variant options can be reviewed and versioned as part of a design system.

```text
Component: PromoCard
Variants: default | compact | highlighted
Data source: same template
Outcome: different page presentation, same content contract
```

## 🧪 Practice Exercise

Design a "Teaser" component with three variants for home, listing, and sidebar contexts. Describe which fields stay shared and which visual rules differ.

## 🔗 Related Topics

- Pages authoring patterns
- SXA rendering variants
- Design system governance

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
