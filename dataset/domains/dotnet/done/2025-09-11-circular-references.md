---
type: "question"
status: draft
date: 2025-09-11 1757601633.853
tags: [question, circular-reference, memory-management, object-design, serialization, json]
reviewed: 
  - date: 2025-09-11
---

# 🎯 Circular References in C# - L3

## Question
>
> **Core Question:** What are circular references? How do they affect memory management and serialization? How can you prevent and handle them?
> **Category:** Framework

## 💡 Quick Answer

Circular reference = object A points to B, and B points back to A.
In .NET, GC can collect cycles, but JSON serialization may fail without settings.

```csharp
using System.Text.Json;
using System.Text.Json.Serialization;

class Parent { public string Name { get; set; } = ""; public Child? Child { get; set; } }
class Child { public string Name { get; set; } = ""; public Parent? Parent { get; set; } }

var p = new Parent { Name = "John" };
var c = new Child { Name = "Alice", Parent = p };
p.Child = c; // Parent -> Child -> Parent

var json = JsonSerializer.Serialize(
    p,
    new JsonSerializerOptions { ReferenceHandler = ReferenceHandler.IgnoreCycles, WriteIndented = true });

Console.WriteLine(json);
```
