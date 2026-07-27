---
type: "question"
status: draft
date: 2026-06-02 1780421651
tags: [question, sitecore, xm-cloud, site-collections, sxa]
---

# 🎯 Site Collections and Shared Content in XM Cloud - L3

## Question

> **Core Question:** How do site collections help organize multiple headless sites, and how can content be shared safely?
> **Follow-up:** What is the risk if teams share content without clear ownership boundaries?

## 💡 Quick Answer (30 seconds)

- A site collection groups related sites under common conventions and reusable assets.
- Shared content should be modeled intentionally (for example global navigation, taxonomy, legal text).

## 📖 Detailed Explanation

In XM Cloud with SXA, site collections create a structural boundary for one or more related sites. They help teams standardize layout, component usage, and authoring conventions while still allowing each site to evolve independently.

Shared content works best when it is explicitly modeled as reusable data, not duplicated per site. Typical shared areas include navigation, brand assets, localization dictionaries, and reusable component content. Production teams usually define ownership and publishing workflows for these shared items to prevent cross-site regressions.

```text
Tenant
  -> Site Collection A
     -> Site A1
     -> Site A2
  -> Shared Content (owned + governed)
```

## 🧪 Practice Exercise

Propose a content ownership model for three country sites that share brand navigation but keep campaign content independent.

## 🔗 Related Topics

- [[2026-04-11-tenants-sxa|SXA tenants and sites]]
- Data source governance
- Content modeling boundaries

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
