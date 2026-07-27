---
type: "question"
status: draft
date: 2026-07-06 1783382400
tags: [csharp, dotnet, oop, constructors, framework]
---

# 🎯 Static Constructor in C# - L2

## Question

> **Core Question:** When will a static constructor be called?
> **Follow-up:** Can you call a static constructor explicitly? What happens if it throws an exception?

## 💡 Quick Answer (30 seconds)

- A static constructor is called **automatically** by the runtime — never directly
- It runs **once**, before any static member is accessed or any instance of the class is created
- It is **guaranteed to be thread-safe** by the runtime
- It does **not take access modifiers or parameters**

## 📖 Detailed Explanation

A static constructor (also called a type initializer) is used to initialize static fields or perform one-time setup for a class. The runtime ensures it runs exactly once per application domain, before any use of the type.

The CLR guarantees that static constructors are invoked in a thread-safe manner — even in multi-threaded scenarios, the constructor will complete before any thread can access static members.

Key rules:

- Executes before the first instance constructor if both exist in the same class
- Executes before the first access to any static member
- If it throws an exception, the type becomes unusable for the lifetime of the application domain (`TypeInitializationException` is raised on subsequent access)
- Cannot be called explicitly and takes no parameters

```csharp
public class DatabaseConfig
{
    public static readonly string ConnectionString;

    // Static constructor — called automatically before first use
    static DatabaseConfig()
    {
        ConnectionString = Environment.GetEnvironmentVariable("DB_CONN")
            ?? throw new InvalidOperationException("DB_CONN not set");
        Console.WriteLine("DatabaseConfig initialized");
    }

    public DatabaseConfig() { } // Instance constructor runs after static one
}

// First access triggers static constructor
var cs = DatabaseConfig.ConnectionString; // "DatabaseConfig initialized" printed
```

## 🧪 Practice Exercise

Create a class `AppSettings` with a static `Dictionary<string, string>` that is populated in a static constructor from a hypothetical config file. What happens if the file doesn't exist — and how would you handle it safely?

## 🔗 Related Topics

- Instance constructors and initialization order
- `Lazy<T>` for deferred initialization
- Singleton pattern using static constructors
- `TypeInitializationException`

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
