---
type: "question"
status: draft
date: 2026-04-26 1777203876
tags: [question, sitecore, sxa, grid]
---

# 🎯 Which Grid Systems Does SXA Support by Default? - L3

## Question

> **Core Question:** Which grid systems does SXA support by default?
> **Follow-up:** Why can this answer change depending on SXA version or product flavor?

## 💡 Quick Answer (30 seconds)

- The classic exam answer is `Bootstrap`, `Foundation`, and `Grid 960`.
- In newer docs, SXA also lists Bootstrap 3/4/5 and Tailwind, depending on version and product.
- So the safe answer is version-sensitive: the older training wording matches Bootstrap, Foundation, and Grid 960, but current docs show a broader set.

## 📖 Detailed Explanation

This question is one of the version-sensitive ones. Older SXA release notes explicitly highlight support for Bootstrap, Foundation, and 960GS. Newer 10.4 grid documentation lists a larger catalog that includes Bootstrap 3, 4, and 5, Tailwind, Foundation, and Grid 960. XM Cloud comparison docs narrow that list again depending on product flavor.

So if you are answering the exam question exactly as written, the expected options are `Bootstrap`, `Foundation`, and `Grid 960`. If you are answering from current product knowledge, you should mention that the documented default set evolved over time.

```text
Older exam framing:
  Bootstrap + Foundation + Grid 960

Current SXA docs:
  Bootstrap 3/4/5 + Tailwind + Foundation + Grid 960
```

Official proof: [SXA 13 release notes](https://developers.sitecore.com/downloads/Sitecore_Experience_Accelerator/13/Sitecore_Experience_Accelerator_13_Initial_Release/Release_Notes#new-featuresimprovements), [The grid settings](https://doc.sitecore.com/xp/en/developers/sxa/latest/sitecore-experience-accelerator/the-grid-settings.html#the-grid-settings), [Compare headless and MVC features according to product](https://doc.sitecore.com/xp/en/developers/sxa/latest/sitecore-experience-accelerator/compare-headless-and-mvc-features-according-to-product.html#compare-headless-and-mvc-features-according-to-product).

## 🧪 Practice Exercise

Explain how you would answer this question differently for an older SXA certification exam versus a 10.4 architecture discussion.

## 🔗 Related Topics

- [[2026-04-11-SXA|SXA]]
- Grid mapping
- Responsive layouts

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
