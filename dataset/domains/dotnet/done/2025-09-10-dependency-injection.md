---
type: "question"
status: draft
date: 2025-09-10 1757523973
tags: [question, csharp, di]
reviewed:
  - date: 2025-09-10
---

# 🎯 Dependency Injection in .NET Core - Q5

## Question

How can you achieve dependency injection in .NET Core?

## 💡 Quick Answer (30 seconds)

- .NET Core has a built-in DI container configured via IServiceCollection in Startup or Program; register services with AddTransient/AddScoped/AddSingleton and inject via constructor.

## 📖 Detailed Explanation

[Show registration examples and service lifetimes.]

## 🧪 Practice Exercise

- Register a service and consume it in a controller or console app via constructor injection.

## 🔗 Related Topics

- Autofac (3rd-party DI container)
- Service lifetimes

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
