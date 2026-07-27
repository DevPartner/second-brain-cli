---
type: "question"
status: draft
date: 2026-04-26 1777203876
tags: [question, sitecore, sxa, search]
---

# 🎯 What Does the Search Results Signature Do in [[2026-04-11-sxa|SXA]]? - L2

## Question

> **Core Question:** What is the purpose of the Search Results signature in [[2026-04-11-sxa|SXA]]?
> **Follow-up:** When do you need different signatures on the same page?

## 💡 Quick Answer (30 seconds)

- The signature is a unique identifier that binds search-related renderings to a specific Search Results instance.
- It limits filters, sorters, and related search components to the intended result set.
- You need separate signatures when a page contains multiple independent search areas.

## 📖 Detailed Explanation

[[2026-04-11-sxa|SXA]] search components communicate through a shared signature. Official docs say the Search Results signature is a unique signature of a specific Search Results rendering and that it is useful when a page has more than one search result rendering. The purpose is isolation: only components with the same signature operate on the same result set.

That is why the exam's fill-in-the-blank answer is simply `signature`. It is not a scope, facet, or token. Scope controls which content can match; signature controls which search UI components are bound together.

```text
Search Box (sig = jobs)
Sort Results (sig = jobs)
Search Results (sig = jobs)

Search Box (sig = products)
Search Results (sig = products)
```

Official proof: [Walkthrough: Adding search functionality to your page](https://doc.sitecore.com/xp/en/users/sxa/latest/sitecore-experience-accelerator/walkthrough--adding-search-functionality-to-your-page.html#add-the-search-results-rendering), [The SXA Search service](https://doc.sitecore.com/xp/en/developers/sxa/latest/sitecore-experience-accelerator/the-sxa-search-service.html#the-sxa-search-service).

## 🧪 Practice Exercise

Describe how you would configure two independent search experiences on one homepage, one for products and one for articles, without their filters interfering with each other.

## 🔗 Related Topics

- Search scope
- Search Box rendering
- Sort Results rendering

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
