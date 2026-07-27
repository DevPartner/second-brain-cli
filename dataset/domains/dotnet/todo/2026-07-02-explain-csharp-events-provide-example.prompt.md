---
agent: Plan
description: "Explain CSharp Events. Provide example"
tools: ["read", "edit", "search", "web", "cognitionai/deepwiki/*", "agent"]
---

# Explain: CSharp Events. Provide example

## Prompt Metadata

- Type: explain
- Status: executed
- Date: 2026-07-02 1782978455.535
- Tags: [csharp, events, delegates, observer-pattern, interview]
- Project: CSharp Interview
- Areas: [education]
- AI reviews:
  - 2026-07-02 | Claude Sonnet 4.6 (Copilot) | Explained events end-to-end with a practical publisher/subscriber example.

## Mission

- Explain what C# events are, how they relate to delegates, and when to use them. Provide a concise, practical code example.

## Inputs

- Role: Research Analyst
- Desired format: Markdown file
- Scale: A simple fact check with a single agent making 3–10 tool calls
- Summary style: Summary (≤ 100 words) + 3 key insights + no follow-up questions

## Response

### Summary

A **C# event** is a notification mechanism built on top of **delegates**. The class that raises the event is the _publisher_; classes that react to it are _subscribers_. Events enforce encapsulation — subscribers can only add (`+=`) or remove (`-=`) handlers; they cannot invoke the event or clear all handlers directly. This makes events ideal for the **Observer pattern** without exposing implementation details.

---

### 3 Key Insights

1. **Events are delegate wrappers with access restriction.**
   Internally an event is a multicast delegate, but the `event` keyword restricts outside code to `+=` / `-=` only — preventing accidental invocations or wholesale replacement of the handler list.

2. **`EventHandler<TEventArgs>` is the standard signature.**
   The BCL convention is `void Handler(object sender, TEventArgs e)`. Stick to this signature so your events integrate cleanly with the framework and third-party code.

3. **Always null-check before raising (or use `?.Invoke`).**
   If no subscriber is attached, invoking a null delegate throws `NullReferenceException`. The pattern `EventName?.Invoke(this, args)` is thread-safer and more concise than the classic null-check pattern.

---

### Code Example

```csharp
using System;

// 1. Custom EventArgs carrying event data
public class OrderPlacedEventArgs : EventArgs
{
    public int    OrderId  { get; }
    public string Customer { get; }

    public OrderPlacedEventArgs(int orderId, string customer)
    {
        OrderId  = orderId;
        Customer = customer;
    }
}

// 2. Publisher — the class that raises the event
public class OrderService
{
    // Declare the event using the standard BCL signature
    public event EventHandler<OrderPlacedEventArgs>? OrderPlaced;

    public void PlaceOrder(int orderId, string customer)
    {
        Console.WriteLine($"[OrderService] Placing order #{orderId} for {customer}…");

        // Business logic here …

        // Raise the event (null-conditional Invoke is thread-safer)
        OnOrderPlaced(new OrderPlacedEventArgs(orderId, customer));
    }

    // Protected virtual so derived classes can override the raise behaviour
    protected virtual void OnOrderPlaced(OrderPlacedEventArgs e)
    {
        OrderPlaced?.Invoke(this, e);
    }
}

// 3. Subscriber — reacts to the event
public class EmailNotificationService
{
    public void Subscribe(OrderService orderService)
    {
        orderService.OrderPlaced += HandleOrderPlaced;
    }

    public void Unsubscribe(OrderService orderService)
    {
        orderService.OrderPlaced -= HandleOrderPlaced;
    }

    private void HandleOrderPlaced(object? sender, OrderPlacedEventArgs e)
    {
        Console.WriteLine(
            $"[Email] Sending confirmation to {e.Customer} for order #{e.OrderId}.");
    }
}

// 4. Wiring it all together
class Program
{
    static void Main()
    {
        var orderService      = new OrderService();
        var emailNotification = new EmailNotificationService();

        // Subscribe
        emailNotification.Subscribe(orderService);

        orderService.PlaceOrder(101, "Alice");
        // Output:
        //   [OrderService] Placing order #101 for Alice…
        //   [Email] Sending confirmation to Alice for order #101.

        // Unsubscribe — handler no longer called
        emailNotification.Unsubscribe(orderService);
        orderService.PlaceOrder(102, "Bob");
        // Output:
        //   [OrderService] Placing order #102 for Bob…
    }
}
```

---

### Quick-Reference Cheat Sheet

| Concept           | Details                                                               |
| ----------------- | --------------------------------------------------------------------- |
| Declaration       | `public event EventHandler<TArgs>? EventName;`                        |
| Subscribe         | `publisher.EventName += MyHandler;`                                   |
| Unsubscribe       | `publisher.EventName -= MyHandler;`                                   |
| Raise             | `EventName?.Invoke(this, args);` inside publisher only                |
| Handler signature | `void Handler(object? sender, TArgs e)`                               |
| Multicast         | Multiple handlers allowed; called in subscription order               |
| Memory leak risk  | Always unsubscribe long-lived subscribers from short-lived publishers |
