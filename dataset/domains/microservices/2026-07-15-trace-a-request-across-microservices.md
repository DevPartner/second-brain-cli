---
type: "question"
status: draft
date: 2026-07-15 1784091837.568
tags: [microservices, observability, azure]
---

# How do you trace a request across multiple microservices? - L3

## Question

> **Core Question:** How do you trace a single user request as it travels through multiple microservices?
> **Follow-up:** What information do you include in the logs to make the flow easier to debug?

## 💡 Quick Answer (30-60 seconds)

- Use correlation IDs and distributed tracing so every hop can be linked together.
- Add structured logs with request IDs, service names, and status codes.
- Combine traces with metrics and alerts to spot latency and failures quickly.

## 📖 Detailed Explanation

In a distributed system, a single user action often spans several services and databases. Distributed tracing gives you a timeline of that flow, while correlation IDs make it possible to connect logs from different services to the same request. A solid approach includes propagating the trace context through synchronous HTTP calls and asynchronous messages, plus writing structured logs that make it easy to search for the same request later. This improves debugging, incident response, and performance tuning.

```csharp
using var activity = new Activity("process.order").SetTag("correlation.id", requestId);
```

## 🧪 Practice Exercise

Outline the logging and tracing strategy for an order checkout flow that calls billing, inventory, and notification services.

## 🔗 Related Topics

- #distributed-tracing
- #correlation-id
- #azure-monitor
