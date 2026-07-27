---
type: "question"
status: draft
date: 2026-03-05 1772678402
tags: [question, sql-server, transactions, isolation-levels]
---

# 🎯 SQL Server Isolation Levels - L3

## Question
>
> **Core Question:** Can you describe the different types of isolation levels available in SQL Server and when you might use each one?
> **Follow-up:** What anomalies does each level prevent?

## 💡 Quick Answer (30 seconds)

- `READ UNCOMMITTED`: fastest reads, allows dirty reads.
- `READ COMMITTED` (default): no dirty reads.
- `REPEATABLE READ`: protects rows already read from updates.
- `SERIALIZABLE`: strongest locking, prevents phantom reads.
- `SNAPSHOT`: row-versioning, consistent view without shared locks on reads.

## 📖 Detailed Explanation

Choose by balancing consistency and concurrency:

- Use `READ COMMITTED` for most OLTP workloads.
- Use stronger levels for financial or critical transactional logic.
- Use `SNAPSHOT` when you need consistent reads with reduced blocking.

Lower isolation gives better concurrency but can expose anomalies.
Higher isolation improves correctness but may reduce throughput.

## 🧪 Practice Exercise

- Simulate two sessions updating and reading the same rows.
- Observe behavior under `READ COMMITTED` vs `SERIALIZABLE` vs `SNAPSHOT`.

## 🔗 Related Topics

- Locking vs row versioning
- Deadlocks and transaction design

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
