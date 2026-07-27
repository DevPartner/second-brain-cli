---
type: "question"
status: draft
date: 2026-04-26 1777203876
tags: [question, sitecore, sxa, composites, personalization]
---

# 🎯 Must a Personalized Composite Use a Different Datasource? - L2

## Question

> **Core Question:** True or false: when personalizing composites, the composite datasource must be distinct from the default rule datasource.
> **Follow-up:** What breaks if the datasources differ?

## 💡 Quick Answer (30 seconds)

- False.
- The composite datasource must point to the same datasource as the default rule in the personalization settings.
- If they differ, the composite content can fail to render correctly.

## 📖 Detailed Explanation

The SXA composite documentation states this directly: if you use personalization for composites, the datasource for the composite must point to the same datasource as the default rule in the personalization settings. If the composite and the personalization do not use the same default datasource item, the composite cannot render properly.

So the correct answer is not just "False" but "False, and the opposite rule applies: keep the datasource aligned with the default rule."

```text
Composite datasource
  == default rule datasource

If not equal
  -> rendering can break or render incorrect content
```

Official proof: [Edit a composite](https://doc.sitecore.com/xp/en/users/sxa/latest/sitecore-experience-accelerator/edit-a-composite.html#edit-a-composite).

## 🧪 Practice Exercise

Explain why a Tabs composite that uses one datasource in the component and a different datasource in its default personalization rule can produce unstable rendering behavior.

## 🔗 Related Topics

- Composite renderings
- Tabs and Accordions
- Datasource strategy in SXA

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
