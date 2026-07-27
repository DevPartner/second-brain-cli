---
type: "question"
status: draft
date: 2026-04-26 1777203876
tags: [question, sitecore, marketplace, blok]
---

# 🎯 Which Design System Should Marketplace Public Apps Follow? - L2

## Question

> **Core Question:** Which design system should Sitecore Marketplace public apps follow for UI consistency?
> **Follow-up:** Does Sitecore require you to install the package, or only follow the guidelines?

## 💡 Quick Answer (30 seconds)

- Sitecore Blok.
- Sitecore strongly recommends designing apps based on Blok so they match the product look and feel.
- For public Marketplace apps, matching the Sitecore look and feel is treated as a requirement.

## 📖 Detailed Explanation

The official Marketplace developer documentation names Blok as the Sitecore product design system. It says developers should design Marketplace apps based on Blok and clarifies that, while you do not have to install Blok itself, you should implement its guidelines so the app matches Sitecore's look and feel.

That is why the correct answer is `Sitecore Blok`, not Bootstrap, Tailwind-only, or Material UI.

```text
Marketplace app UI standard
  -> Follow Blok design guidelines
  -> Match Sitecore look and feel
```

Official proof: [Introduction to Sitecore Marketplace for custom and public apps](https://doc.sitecore.com/mp/en/developers/marketplace/introduction-to-sitecore-marketplace-for-custom-and-public-apps.html#workflow-for-creating-apps), [Blok design system reference](https://doc.sitecore.com/mp/en/developers/marketplace/blok-design-system-reference.html#blok-design-system-reference).

## 🧪 Practice Exercise

Explain to a teammate why building a public Marketplace app with an arbitrary custom design language could block approval even if the app works technically.

## 🔗 Related Topics

- Marketplace SDK
- Public app review expectations
- Sitecore product look and feel

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
