---
type: "question"
status: draft
date: 2025-09-10 1757523973
tags: [question, csharp, gc]
reviewed:
  - date: 2025-09-10
---

# 🎯 Garbage Collection in .NET - Q2

## Question

How does garbage collection work in the .NET framework?

## 💡 Quick Answer (30 seconds)

- The CLR GC tracks object references and frees memory for unreferenced objects. It uses generations, compaction, and finalization.

## 📖 Detailed Explanation

[Explain generations (0,1,2), ephemeral objects, large object heap, and finalizers.]

## 🧪 Practice Exercise

- Allocate many short-lived objects and observe memory/GC behavior with diagnostic tools.

## 🔗 Related Topics

- IDisposable and using pattern
- Finalizers and GC.SuppressFinalize

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
