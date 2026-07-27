---
type: "question"
status: draft
date: 2025-09-11 1757573830.329
tags: [dotnet, delegates, events]
---

# 🎯 [[2026-03-04-action-update-delegates-and-events-in-net-zknotes.prompt]] - L2

## Question
>
> **Core Question:** What is the difference between delegates and events in .NET?
> **Follow-up:** How would you implement a custom event handler?

## 💡 Quick Answer (30 seconds)

1. Delegates represent executable method references; events represent controlled notifications.
2. Events enforce encapsulation by preventing external invocation.
3. In practice: use delegates to pass behavior, use events to model notifications and preserve encapsulation.

## 📖 Detailed Explanation

- **Delegate**
  - Represents a method signature.
  - Can be assigned, combined (multicast), passed as a parameter, and invoked by the holder.
  - Typical use: callbacks, strategy injection, LINQ-style function passing.

- **Event**
  - Wraps a delegate field with restricted access.
  - Consumers can only use `+=` and `-=` from outside the declaring type.
  - Only the declaring type can trigger notifications, which prevents accidental external invocation.

### Short C# examples

```csharp
using System;

public class DelegateDemo
{
  public delegate int MathOp(int left, int right);

  public static int Add(int left, int right) => left + right;

  public static void Run()
  {
    MathOp op = Add;
    int result = op(2, 3);
    Console.WriteLine(result); // 5
  }
}
```

```csharp
using System;

public class DownloadService
{
  public event EventHandler<string>? Completed;

  public void Finish(string fileName)
  {
    Completed?.Invoke(this, fileName);
  }
}

public class EventDemo
{
  public static void Run()
  {
    var service = new DownloadService();
    service.Completed += (_, fileName) => Console.WriteLine($"Done: {fileName}");
    service.Finish("report.pdf");
  }
}
```

- **Why interviewers care**
  - This distinction demonstrates understanding of API design and encapsulation.
  - Delegates express "what to execute"; events express "what happened".

- **Custom event handler implementation (conceptual flow)**
  1. Define event payload (if needed) via `EventArgs`-like data.
  2. Define event using a handler signature (`sender` + payload).
  3. Raise the event inside protected/internal logic when state changes.
  4. Subscribe and unsubscribe in consumer lifecycle boundaries.
  5. Keep handlers short and non-blocking to avoid coupling and latency spikes.

## 🧪 Practice Exercise

1. Design a `FileProcessor` publisher with these events: `Started`, `ProgressChanged`, `Completed`, and `Failed`.
2. For each event, define what payload data a subscriber needs.
3. Explain which members should be public vs internal to preserve encapsulation.
4. Describe how you prevent memory leaks caused by long-lived subscriptions.
5. Bonus: explain when `Action<T>` is enough and when a named delegate/event is better for readability.

## 🔗 Related Topics

- Observer Pattern
- `Action` / `Func` delegates
- Multicast delegates
- `EventHandler` and `EventHandler<TEventArgs>`
- Subscription lifecycle management (unsubscribe/dispose)

## 📊 Confidence Level

- [ ] 🟡 Somewhat confident (4-6)  

## 📅 Last Reviewed: 2026-03-04

## 🎯 Next Review: 2026-04-03
