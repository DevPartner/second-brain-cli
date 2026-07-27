---
type: "question"
status: draft
date: 2026-07-15 1784091837.568
tags: [ddd, aggregates, domain-modeling]
---

# How do you decide the boundary of an Aggregate Root? - L3

## Question

> **Core Question:** How do you decide the boundary of an Aggregate Root?
> **Follow-up:** What happens if you need to change data across two different aggregates?

## 💡 Quick Answer (30-60 seconds)

- An Aggregate Root should protect the invariants that must stay consistent together.
- Keep the aggregate small enough to be easy to reason about and modify.
- If two objects need strong consistency, they usually belong in the same aggregate.

## 📖 Detailed Explanation

The main rule is to group entities around a business transaction that must remain consistent. If changing one object always requires changing another object as part of the same rule, they likely belong in the same aggregate. The aggregate root is the single entry point for change and should hide internal details. When changes span multiple aggregates, you usually model that as a domain event or a saga rather than trying to force a single transaction across the whole boundary.

```csharp
public class Order
{
    private readonly List<OrderItem> _items = new();

    public void AddItem(Product product, int quantity)
    {
        ArgumentOutOfRangeException.ThrowIfNegativeOrZero(quantity);
        _items.Add(new OrderItem(product.Id, quantity));
    }
}
```

## 🧪 Practice Exercise

Model a simple order workflow and decide whether the order line and the order itself should live inside the same aggregate.

## 🔗 Related Topics

- #domain-driven-design
- #aggregate-root
- #domain-events
