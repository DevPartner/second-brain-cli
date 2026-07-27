---
type: "question"
status: draft
date: 2026-03-05 1772678400
tags: [question, sql, aggregation, top-n]
---

# 🎯 Top 3 Customers by Revenue - L2

## Question
>
> **Core Question:** How would you write a query to find the top 3 customers by revenue in a sales database?
> **Follow-up:** What if revenue must be calculated as sales amount minus cost?

## 💡 Quick Answer (30 seconds)

- Group sales by customer.
- Compute revenue with `SUM(...)`.
- Sort by revenue descending.
- Use `TOP (3)` (SQL Server) to return the highest three.

## 📖 Detailed Explanation

Typical pattern:

1. Join `sales` with `customers` if needed.
2. Calculate total revenue per customer using `SUM`.
3. Use `GROUP BY` on customer identity fields.
4. Order by the aggregated revenue in descending order.
5. Return only top 3 rows.

You can order by aggregated expressions in SQL Server, so no temp table is required for this basic case.

## 🧪 Practice Exercise

- Write one query using a stored `revenue` column.
- Write a second query where revenue is `SUM(subtotal - cost)`.

## 🔗 Related Topics

- `GROUP BY` and aggregate functions
- Window functions (`ROW_NUMBER`, `DENSE_RANK`)

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
