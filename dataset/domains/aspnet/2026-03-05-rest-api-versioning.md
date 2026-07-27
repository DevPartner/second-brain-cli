---
type: "question"
status: draft
date: 2026-03-05 1772716800
tags: [question, rest, api-versioning, backward-compatibility]
reviewed:
  - date: 2026-03-05
---

# 🎯 REST API Versioning Strategies - L2

## Question

> **Core Question:** How would you handle versioning in a RESTful API?
> **Category:** REST APIs

## 💡 Quick Answer (30 seconds)

Prefer backward-compatible evolution first; introduce explicit versions only for
breaking changes. Common approaches are URI versioning, header-based versioning,
or media-type versioning.

## 📖 Detailed Explanation

- **Default strategy:** Avoid breaking changes when possible.
- **When needed:** Add new major version for incompatible contract changes.
- **Options:** `/v1/...`, custom header, or media type versioning.
- **Lifecycle:** Deprecate old versions with timelines and migration guidance.
- **Governance:** Track usage before retirement.

## 🧪 Practice Exercise

Define a deprecation plan for `v1` to `v2` including announcement, overlap window,
and final sunset date.

## 🔗 Related Topics

- API governance
- Contract testing
- Consumer-driven contracts

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
