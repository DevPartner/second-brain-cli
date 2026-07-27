---
type: "question"
status: draft
date: 2026-03-05 1772678401
tags: [question, sql, index, performance]
---

# 🎯 When to Use Indexes - L2

## Question
>
> **Core Question:** Can you provide an example of when you would use an index and why you would use it?
> **Follow-up:** What trade-offs do indexes introduce?

## 💡 Quick Answer (30 seconds)

- Use indexes on columns frequently used in `WHERE`, `JOIN`, `ORDER BY`, and `GROUP BY`.
- Main benefit: faster reads and lookups.
- Trade-off: slower `INSERT`, `UPDATE`, `DELETE` and extra storage.

## 📖 Detailed Explanation

Example: a large `orders` table filtered often by `customer_id` and `order_date`.

- Add an index on `customer_id` for quick customer lookups.
- Add a composite index like `(customer_id, order_date)` for common filtered date ranges.

This improves read performance but increases write overhead because indexes must be updated whenever data changes.

## 🧪 Practice Exercise

- Compare execution plans before and after adding an index on a frequently filtered column.
- Measure read speed vs write impact.

## 🔗 Related Topics

- Clustered vs nonclustered indexes
- Covering indexes and included columns

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
