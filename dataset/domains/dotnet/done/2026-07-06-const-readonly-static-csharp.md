---
type: "question"
status: draft
date: 2026-07-06 1783382400
tags: [csharp, dotnet, framework, const, readonly, static, keywords]
---

# 🎯 const vs readonly vs static in C# - L2

## Question

> **Core Question:** What are the differences between `const`, `readonly`, and `static` in C#?
> **Follow-up:** Can a `static` field also be `readonly`? When would you use `static readonly` over `const`?

## 💡 Quick Answer (30 seconds)

- `const` — compile-time constant; value baked into the IL, cannot change, implicitly `static`
- `readonly` — runtime constant; value set only at declaration or in a constructor, per-instance by default
- `static` — belongs to the type, not to any instance; shared across all instances

## 📖 Detailed Explanation

These three modifiers control how and when a value is set and who owns it.

**`const`** is resolved at compile time. Its value is embedded directly into the consuming assembly, which means a version mismatch can occur if a library changes the value — consuming code still uses the old one until recompiled. `const` is always implicitly `static`.

**`readonly`** is resolved at runtime. The value can only be assigned at the point of declaration or inside a constructor (including a static constructor for static readonly fields). Unlike `const`, it can hold reference types and non-constant expressions.

**`static`** declares that the member belongs to the type itself rather than to any particular instance. It can be combined with `readonly` to create a shared, immutable value that is determined at runtime — a common pattern for singleton-like defaults.

```csharp
public class Config
{
    // const: compile-time, always static, primitive/string only
    public const int MaxRetries = 3;

    // readonly: runtime, per-instance, set in constructor
    public readonly string Region;

    // static readonly: runtime, shared, set in static constructor or inline
    public static readonly TimeSpan Timeout = TimeSpan.FromSeconds(30);

    public Config(string region)
    {
        Region = region; // ✅ allowed — constructor
        // Region = "other"; would fail outside constructor
    }
}

// MaxRetries is inlined by compiler — changing it requires recompile of all consumers
// Timeout is resolved once at runtime — safe for cross-assembly sharing
```

## 🧪 Practice Exercise

You need a class-level cache key prefix that is computed from the assembly name at startup and never changes. Which modifier(s) would you use — `const`, `static readonly`, or something else? Why?

## 🔗 Related Topics

- Compile-time vs runtime constants
- `static` constructors and type initialization
- `readonly struct` in C# 7.2+
- Value types vs reference types in readonly fields

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
