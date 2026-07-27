---
agent: agent
description: "Explain CSharp factories vs conditional registrations"
tools: ["read", "edit", "search", "web", "cognitionai/deepwiki/*", "agent"]
---

# Explain: CSharp factories vs conditional registrations

## Prompt Metadata

- Type: explain
- Status: executed
- Date: 2026-07-02 1782978090.889
- Tags: [csharp, dependency-injection, factories, ioc, interview]
- Project: CSharp Interview
- Areas: [education]
- AI reviews: 2026-07-02 Claude Sonnet 4.6

## Mission

- Clarify `CSharp factories vs conditional registrations` for the intended audience using the provided context.

## Inputs

- Role: Research Analyst
- Desired format: Markdown file
- Scale: A simple fact check with a single agent making 3–10 tool calls
- Summary style: Summary (≤ 100 words) + 3 key insights + no follow-up questions

## Response

### Summary

In C# DI (Microsoft.Extensions.DependencyInjection and equivalents), **factory registrations** use a delegate overload to control *how* an instance is built at resolve-time, enabling runtime branching. **Conditional registrations** apply logic at *startup* (composition root) to pick *which* implementation is registered for a given interface. Use factories when the decision depends on per-request runtime state; use conditional registrations when the choice is fixed for the application's lifetime.

---

### Key Insights

**1. Timing is the core difference**

| Approach | Decision point | Scope |
|---|---|---|
| Conditional registration | Startup / `Program.cs` | Fixed for app lifetime |
| Factory delegate | Resolve-time (each `GetService` call) | Per-request or per-scope |

```csharp
// Conditional registration — decided once at startup
if (config["PaymentProvider"] == "Stripe")
    services.AddScoped<IPaymentService, StripeService>();
else
    services.AddScoped<IPaymentService, PayPalService>();

// Factory registration — decided on every resolve
services.AddScoped<IPaymentService>(sp =>
{
    var config = sp.GetRequiredService<IConfiguration>();
    return config["PaymentProvider"] == "Stripe"
        ? sp.GetRequiredService<StripeService>()
        : sp.GetRequiredService<PayPalService>();
});
```

> Factory delegate receives `IServiceProvider`, so it can pull dependencies, read runtime state, or implement the **Strategy** or **Decorator** pattern dynamically.

---

**2. Factories enable runtime parameterization and decorator chains**

Factories are the idiomatic way to build objects that require constructor arguments not available in the container, or to wrap a resolved service:

```csharp
// Decorator pattern via factory
services.AddScoped<IOrderRepository, SqlOrderRepository>();
services.Decorate<IOrderRepository>((inner, sp) =>
    new CachedOrderRepository(inner, sp.GetRequiredService<IMemoryCache>()));

// Keyed / named service selection at runtime (pre-.NET 8 pattern)
services.AddTransient<Func<string, INotifier>>(sp => key => key switch
{
    "email" => sp.GetRequiredService<EmailNotifier>(),
    "sms"   => sp.GetRequiredService<SmsNotifier>(),
    _       => throw new ArgumentException($"Unknown notifier: {key}")
});
```

> **.NET 8+** introduced **keyed services** (`AddKeyedScoped`, `GetRequiredKeyedService`) as a first-class alternative to the `Func<string, T>` factory trick.

---

**3. Double-registration pitfall with conditional logic**

Registering the same interface twice without a guard leads to subtle bugs:

```csharp
// BUG: both registrations survive — GetRequiredService returns the LAST one
services.AddScoped<IFeature, BasicFeature>();
services.AddScoped<IFeature, PremiumFeature>(); // wins silently

// SAFE: guard with TryAdd (registers only if no implementation exists yet)
services.TryAddScoped<IFeature, BasicFeature>();

// SAFE: or remove first, then add
services.RemoveAll<IFeature>();
services.AddScoped<IFeature, PremiumFeature>();

// All registrations are visible via GetServices<IFeature>() (returns IEnumerable)
```

Rule of thumb: prefer `TryAdd*` when writing library/extension code so consumers can override; use explicit `RemoveAll` + `Add` when you intentionally replace a default.
