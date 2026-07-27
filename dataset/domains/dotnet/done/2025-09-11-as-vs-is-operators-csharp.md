---
type: "question"
status: draft
date: 2025-09-11 1757601633.852
tags: [question, csharp, operators]
reviewed: 
  - date: 2025-09-11
---

# 🎯 'as' vs 'is' operators in C# - L1

## Question
>
> **Core Question:** What are the differences between 'as' and 'is' operators in C#?
> **Category:** Framework

## 💡 Quick Answer (30 seconds)

- **'is' operator:** Checks if an object is a certain type and returns a boolean
- **'as' operator:** Casts an object from one type to another (nullable) type

## 📖 Detailed Explanation

The `is` and `as` operators in C# serve different purposes in type checking and casting:

**'is' operator:**

- Performs type checking
- Returns `true` if the object is of the specified type, `false` otherwise
- Does not perform casting
- Safe to use without null reference exceptions

```csharp
object obj = "Hello";
if (obj is string)
{
    // obj is a string, but still treated as object here
    Console.WriteLine("It's a string!");
}
```

**'as' operator:**

- Performs type casting
- Returns the object cast to the specified type if successful, `null` if not
- Only works with reference types and nullable value types
- More efficient than traditional casting when you expect the cast might fail

```csharp
object obj = "Hello";
string str = obj as string;
if (str != null)
{
    // str is now a string and we can use string methods
    Console.WriteLine(str.ToUpper());
}
```

## 🧪 Practice Exercise

Write code that demonstrates both operators:

1. Use `is` to check if an object is a string
2. Use `as` to safely cast an object to string
3. Handle the case where the cast fails

## 🔗 Related Topics

- Type casting in C#
- Pattern matching with `is`
- Null checking patterns

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)  
- [ ] 🟢 Very confident (7-10)
