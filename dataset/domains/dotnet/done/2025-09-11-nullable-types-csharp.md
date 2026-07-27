---
type: "question"
status: draft
date: 2025-09-11 1757601633.852
tags: [question, csharp, nullable, types]
reviewed: 
  - date: 2025-09-11
---

# 🎯 Nullable Types in C# - L2

## Question
>
> **Core Question:** What is a Nullable type?
> **Category:** Framework

## 💡 Quick Answer (30 seconds)

- A type that can return a null value
- Allows value types to represent null in addition to their normal range of values

## 📖 Detailed Explanation

Nullable types in C# allow value types (like `int`, `bool`, `DateTime`) to represent `null` values, which they normally cannot do.

**Key Concepts:**

- Value types normally cannot be `null`
- Nullable types are declared using `T?` syntax or `Nullable<T>`
- Useful for database operations and optional values

**Common Use Cases:**

- Database fields that can be NULL
- Optional method parameters
- Return values that may not have a result

## 🧪 Practice Exercise

Create a method that:

1. Takes a nullable integer parameter
2. Returns the square of the number if it has a value
3. Returns null if the input is null

## 🔗 Related Topics

- Value types vs Reference types
- Null-conditional operators (?. and ?[])
- Nullable reference types (C# 8.0+)

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)  
- [ ] 🟢 Very confident (7-10)
