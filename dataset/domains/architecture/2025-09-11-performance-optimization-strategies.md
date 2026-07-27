---
type: "question"
status: draft
date: 2025-09-11 2299999999
tags: [question, performance, optimization, asp-net, architecture, scalability]
reviewed: 
  - date: 2025-09-11
---

# 🎯 Performance Optimization Strategies - L3

## Question
>
> **Core Question:** You have a controller action where 3 requests for 3rd party systems are done sequentially and take around 3 seconds in overall. The longest response is about 1 second. How to improve performance here?
> **Follow-up:** What are the key performance optimization techniques in ASP.NET Core applications?

## 💡 Quick Answer (30 seconds)

- **Parallelize requests**: Use `Task.WhenAll()` to execute requests concurrently
- **Async programming**: Convert to async/await to free up threads
- **Caching**: Cache responses to avoid repeated external calls
- **Timeout optimization**: Set appropriate timeouts for third-party calls
- **Result**: Reduce 3 seconds sequential to ~1 second (longest request time)

## 🔗 Related Topics

- **Async/Await Best Practices**
- **HTTP Client Configuration and Pooling**
- **Caching Strategies and Cache Invalidation**
- **Database Query Optimization**
- **Memory Management and Garbage Collection**

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)  
- [ ] 🟢 Very confident (7-10)
