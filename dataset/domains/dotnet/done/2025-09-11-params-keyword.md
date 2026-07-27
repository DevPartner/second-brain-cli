---
type: "question"
status: draft
date: 2025-09-11 1757601633.852
tags: [question, csharp, params-keyword, method-parameters, variable-arguments]
reviewed: 
  - date: 2025-09-11
---

# 🎯 Params Keyword in C# - L2

## Question
>
> **Core Question:** Explain the 'params' keyword in C# and provide usage examples.
> **Category:** Framework

## 💡 Quick Answer (30 seconds)

The `params` keyword allows a method to accept a variable number of arguments of the same type. It must be the last parameter in the method signature and creates an array internally to hold the passed arguments.

## 📖 Detailed Explanation

**Best Practices:**

- Use params for methods that logically accept variable numbers of similar items
- Consider providing specific overloads for common scenarios (performance)
- Document whether null arrays are acceptable
- Use meaningful parameter names that indicate plurality
- Consider using IEnumerable<T> instead of params for large collections
- Be careful with performance in tight loops due to array allocation

**Common Use Cases:**

- Mathematical operations (Sum, Max, Min)
- Collection operations (Print, Process)
- Logging methods with multiple messages
- String formatting and concatenation
- Method chaining scenarios

## 🧪 Practice Exercise

Create a utility class with:

1. A method that finds the maximum value using params
2. A method that concatenates strings with a separator using params
3. A generic method that converts all params arguments to a specific format
4. Demonstrate the performance difference between specific overloads and params

## 🔗 Related Topics

- Method overloading
- Arrays and collections
- Generic methods
- Performance optimization
- Variable arguments in other languages

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)  
- [ ] 🟢 Very confident (7-10)
