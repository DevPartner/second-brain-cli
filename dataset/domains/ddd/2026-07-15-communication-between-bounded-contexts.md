---
type: "question"
status: draft
date: 2026-07-15 1784091837.568
tags: [ddd, bounded-contexts, integration]
---

# How do you handle communication between bounded contexts? - L4

## Question

> **Core Question:** How do you handle communication between two different bounded contexts?
> **Follow-up:** When would you use an Anti-Corruption Layer instead of direct integration?

## 💡 Quick Answer (30-60 seconds)

- Use explicit contracts and clear ownership boundaries between contexts.
- Prefer async events for loose coupling when the relationship is not transactional.
- Introduce an Anti-Corruption Layer when the external model does not fit your domain language.

## 📖 Detailed Explanation

Bounded contexts should not share implementation details directly because that creates coupling and makes the domain harder to evolve. Instead, teams often define a published contract such as an API, message schema, or domain event. If the other context uses a different model or terminology, an Anti-Corruption Layer helps translate between systems without leaking one context's internal concepts into the other. This keeps each team focused on its own ubiquitous language while still enabling collaboration.

```csharp
public class CustomerProfileSyncService
{
    public CustomerProfile Translate(CustomerDto dto)
    {
        return new CustomerProfile(dto.Id, dto.FullName);
    }
}
```

## 🧪 Practice Exercise

Design a simple integration between an ordering context and a customer context, and identify where an Anti-Corruption Layer would help.

## 🔗 Related Topics

- #anti-corruption-layer
- #domain-events
- #ubiquitous-language
