---
type: "question"
status: draft
date: 2026-04-26 1777203876
tags: [question, sitecore, sxa, partial-designs, personalization]
---

# 🎯 How Does SXA Centralize Partial Design Changes? - L3

## Question

> **Core Question:** If a feature such as personalization is configured on a partial design, does [[2026-04-11-sxa|SXA]] let that change flow across all uses of that partial design?
> **Follow-up:** When would you avoid central reuse and choose a page-specific change instead?

## 💡 Quick Answer (30 seconds)

- [[2026-04-11-sxa|SXA]] is built so shared layout elements such as headers and footers are managed centrally.
- Use a page-specific override only when one page genuinely needs different behavior.

## 📖 Detailed Explanation

The closest official wording in the docs is that partial designs are reusable layout elements and that changes to an existing partial design are applied to all pages that use it. Sitecore also recommends central management of shared design elements through partial designs and inheritance. That is the basis for the exam answer that you configure the shared behavior once and leverage it everywhere the partial design is used.

The documentation I found is stronger on reuse and propagation than on personalization specifically, so the safe interpretation is: if personalization is part of the partial design configuration, it follows the same reuse model as other partial design changes. If one page needs a different experience, use a different partial design, inheritance, or page-level override rather than breaking the shared abstraction.

```text
Base partial design
  -> header components
  -> shared rules/configuration
All pages using that partial design inherit the same shared setup
```

Official proof: [Work with partial designs](https://doc.sitecore.com/sai/en/users/sitecoreai/work-with-partial-designs.html#work-with-partial-designs), [Create and change a partial design](https://doc.sitecore.com/xp/en/developers/sxa/latest/sitecore-experience-accelerator/create-and-change-a-partial-design.html#create-and-change-a-partial-design), [Recommendations: Sharing content](https://doc.sitecore.com/xp/en/developers/sxa/latest/sitecore-experience-accelerator/recommendations--sharing-content.html#recommendations-sharing-content).

## 🧪 Practice Exercise

Design a header strategy for a multisite tenant where most sites share one authenticated-user experience but one country site needs a special variant.

## 🔗 Related Topics

- Partial design inheritance
- Page designs
- Shared site patterns

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
