---
type: "question"
status: draft
date: 2026-04-26 1777203876
tags: [question, sitecore, graphql, authoring-api]
---

# 🎯 Which GraphQL Parameter Permanently Deletes an Item? - L2

## Question

> **Core Question:** To permanently delete an item in Sitecore GraphQL, which request input parameter must be set to `true`?
> **Follow-up:** What happens if you do not set that parameter?

## 💡 Quick Answer (30 seconds)

- Set `permanently: true` in the `deleteItem` mutation input.
- That requests a hard delete instead of a recycle-bin delete.
- Without it, deletion follows the normal non-permanent path.

## 📖 Detailed Explanation

The authoring GraphQL examples are explicit: the `deleteItem` mutation accepts an input object where `permanently: true` requests a permanent deletion. The docs even call this out in plain language before the example query.

This is exactly why the correct exam answer is `permanently`, not `permanent`, `recycle`, or `temporary`.

```graphql
mutation {
  deleteItem(
    input: {
      path: "/sitecore/content/Home/Example"
      permanently: true
    }
  ) {
    successful
  }
}
```

Official proof: [Query examples for authoring operations](https://doc.sitecore.com/xp/en/developers/latest/sitecore-experience-manager/query-examples-for-authoring-operations.html#query-examples).

## 🧪 Practice Exercise

Write the `deleteItem` mutation twice: once for a normal delete and once for a permanent delete, then explain the semantic difference.

## 🔗 Related Topics

- Authoring and Management API
- `deleteItem` mutation
- Recycle Bin behavior

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
