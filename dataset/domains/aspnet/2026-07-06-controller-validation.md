---
type: "question"
status: draft
date: 2026-07-06 1783382400
tags: [aspnet, mvc, validation, dataannotations, modelstate, fluentvalidation]
---

# 🎯 Controller Validation in ASP.NET MVC / Core - L2

## Question

> **Core Question:** You need to have validation on your controller — how would you implement it?
> **Follow-up:** What is the difference between model-level Data Annotations and a custom validation attribute? When would you reach for FluentValidation instead?

## 💡 Quick Answer (30 seconds)

- Apply **Data Annotations** (`[Required]`, `[Range]`, `[StringLength]`) to model properties — ASP.NET validates automatically on model binding
- Check `ModelState.IsValid` in the action and return `BadRequest(ModelState)` if invalid
- Use **custom validation attributes** (inherit `ValidationAttribute`) for reusable domain rules
- Use **FluentValidation** for complex, conditional, or cross-property rules

## 📖 Detailed Explanation

ASP.NET Core performs model validation automatically during model binding. If any annotation fails, the corresponding error is added to `ModelState`.

**Approach 1 — Data Annotations (most common)**

Decorate model properties and check `ModelState.IsValid` in the action.

```csharp
public class CreateOrderRequest
{
    [Required]
    [StringLength(100, MinimumLength = 3)]
    public string ProductName { get; set; } = string.Empty;

    [Range(1, 1000)]
    public int Quantity { get; set; }

    [EmailAddress]
    public string? CustomerEmail { get; set; }
}

[HttpPost]
public IActionResult CreateOrder([FromBody] CreateOrderRequest request)
{
    if (!ModelState.IsValid)
        return BadRequest(ModelState); // 400 with field-level errors

    // business logic...
    return CreatedAtAction(nameof(GetOrder), new { id = 1 }, null);
}
```

**Approach 2 — `[ApiController]` attribute (ASP.NET Core shortcut)**

Adding `[ApiController]` to a controller makes `ModelState.IsValid` checks automatic — no need to check manually. Invalid requests get a `400 ProblemDetails` response before the action body executes.

**Approach 3 — Custom validation attribute**

```csharp
public class FutureDateAttribute : ValidationAttribute
{
    protected override ValidationResult? IsValid(object? value, ValidationContext ctx)
    {
        if (value is DateTime date && date > DateTime.UtcNow)
            return ValidationResult.Success;
        return new ValidationResult("Date must be in the future.");
    }
}

public class BookingRequest
{
    [FutureDate]
    public DateTime CheckIn { get; set; }
}
```

**Approach 4 — Action filter for cross-cutting validation**

Register a global `IActionFilter` to handle `ModelState` uniformly across all controllers.

## 🧪 Practice Exercise

Create a `RegistrationRequest` model with: `Username` (required, 3-20 chars), `Email` (required, valid format), `Password` (required, min 8 chars), and `ConfirmPassword` (must match `Password`). Which approach handles the cross-field `ConfirmPassword` check best, and why?

## 🔗 Related Topics

- `[ApiController]` automatic model validation
- `IValidatableObject` for object-level validation
- FluentValidation library as a replacement for Data Annotations
- `ProblemDetails` response format (RFC 7807)
- Action filters (`IActionFilter`, `IAsyncActionFilter`)

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
