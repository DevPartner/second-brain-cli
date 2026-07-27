---
type: "question"
status: draft
date: 2025-09-11 1757601633.852
tags: [question, csharp, managed-code, unmanaged-code, clr, memory-management]
reviewed: 
  - date: 2025-09-11
---

# 🎯 Managed vs Unmanaged Code - L3

## Question
>
> **Core Question:** Explain the difference between Managed and Unmanaged code in .NET.
> **Category:** Framework

## 💡 Quick Answer (30 seconds)

**Managed Code** runs under the control of the Common Language Runtime (CLR) with automatic memory management, type safety, and garbage collection. **Unmanaged Code** runs directly on the operating system without CLR supervision, requiring manual memory management and offering direct hardware access.

## 🧪 Practice Exercise

Create an example that demonstrates:

1. A managed class that uses automatic memory management
2. P/Invoke to call a Windows API function
3. Proper cleanup of unmanaged resources using SafeHandle
4. Compare memory usage patterns between managed and unmanaged approaches

## 🔗 Related Topics

- Common Language Runtime (CLR)
- Garbage collection
- P/Invoke and interoperability
- Memory management patterns
- Performance optimization

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)  
- [ ] 🟢 Very confident (7-10)
