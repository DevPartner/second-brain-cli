---
type: "question"
status: draft
date: 2026-03-05 1772716800
tags: [question, aspnet, exception-handling, global-asax, mvc]
reviewed:
  - date: 2026-03-05
---

# 🎯 Exception Handling in ASP.NET - L2

## Question
>
> **Core Question:** How do you handle exceptions in ASP.NET?
> **Category:** ASP.NET

## 💡 Quick Answer (30 seconds)

Use layered handling: local `try/catch` for recoverable cases, global handlers for
unhandled exceptions, centralized logging, and safe error responses/pages.

## 📖 Detailed Explanation

- **Local level:** Use `try/catch` where you can add business context or recover.
- **Global level:** Handle unhandled exceptions in `Global.asax` (Web Forms) or
  middleware/filters (MVC/Core).
- **Observability:** Log error details with correlation IDs.
- **User experience:** Return friendly messages; never expose stack traces.
- **Consistency:** Standardize error contracts for APIs.

## 🧪 Practice Exercise

Describe how you would handle a database timeout at controller level and what
global fallback should do if the exception is unhandled.

## 🔗 Related Topics

- Global.asax
- Filters and middleware
- API error contracts

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
