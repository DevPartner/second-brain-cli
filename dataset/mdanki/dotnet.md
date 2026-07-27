## What are the differences between async/await and Task in C#? When would you use one over the other?

%

async/await is a syntactic sugar that makes asynchronous code easier to read and write, while Task is the underlying type that represents asynchronous operations. async/await provides a more natural control flow, better exception handling, and easier debugging, while direct Task manipulation offers more control but requires more careful handling of continuations and exception propagation.

[#interview]() [#dotnet]()

## How do you manage configuration in .NET applications? What are the different sources and best practices for secure configuration management?

%

Configuration in .NET is managed through the Configuration API which supports multiple sources (appsettings.json, environment variables, command line, user secrets, Azure Key Vault) with a specific precedence order. Best practices include using strong typing with IOptions, securing sensitive data with user secrets or key vaults, implementing validation, and organizing configuration by environment with proper inheritance.

[#interview]() [#dotnet]()

## What are the different dependency injection patterns in .NET? Explain service lifetimes, and how would you implement complex scenarios like factories, decorators, and conditional registrations?

%

.NET DI (Microsoft.Extensions.DependencyInjection) supports three lifetimes: Transient (new instance per request), Scoped (one per scope/request), Singleton (one for the app lifetime).

Factory registrations use a delegate to control *how* an instance is built at resolve-time (runtime branching).
Conditional registrations pick *which* implementation is registered at startup (composition root).

Decorator pattern: wrap a resolved service inside another implementation.
Use `TryAdd*` in library code to let consumers override; use `RemoveAll` + `Add` to replace a default.

[#interview]() [#dotnet]()

## Can you explain the differences between .NET Framework and .NET Core?

%

.NET Framework: Windows-only, mature, full API surface.
.NET Core / .NET (modern): cross-platform, modular, improved performance, built-in DI and modern tooling.

[#interview]() [#dotnet]()

## How does garbage collection work in the .NET framework?

%

The CLR GC tracks object references and frees memory for unreferenced objects. It uses generations, compaction, and finalization.

[#interview]() [#dotnet]()

## Can you describe what a delegate is in C# and provide an example of how you would use one?

%

Delegates are type-safe function references. Use them to pass methods as parameters or implement callback patterns.

[#interview]() [#dotnet]()

## How can multithreading be achieved in C#?

%

Use threads, ThreadPool, Tasks, and async/await for concurrency and asynchronous programming.

[#interview]() [#dotnet]()

## How can you achieve dependency injection in .NET Core?

%

.NET Core has a built-in DI container configured via IServiceCollection in Startup or Program; register services with AddTransient/AddScoped/AddSingleton and inject via constructor.

[#interview]() [#dotnet]()

## What are the differences between 'as' and 'is' operators in C#?

%

'is' operator: Checks if an object is a certain type and returns a boolean
'as' operator: Casts an object from one type to another (nullable) type

[#interview]() [#dotnet]()

## What are circular references? How do they affect memory management and serialization? How can you prevent and handle them?

%

Circular reference = object A points to B, and B points back to A.
In .NET, GC can collect cycles, but JSON serialization may fail without settings.

[#interview]() [#dotnet]()

## Describe a scenario in which you would use DataAnnotations from the System.ComponentModel.DataAnnotations namespace

%

Use DataAnnotations to enforce validation on model properties
Common scenarios: form validation, API input validation, database constraints

[#interview]() [#dotnet]()

## Explain Generics in C# and provide examples of their usage

%

Generics allow you to define classes, interfaces, and methods with placeholder types (type parameters) that are specified when the generic is used. They provide type safety, performance benefits, and code reusability without sacrificing compile-time type checking.

[#interview]() [#dotnet]()

## What is the purpose of the 'using' statement and IDisposable interface?

%

IDisposable: Interface for deterministic cleanup of unmanaged resources
using statement: Ensures Dispose() is called automatically, even if exceptions occur
Purpose: Proper resource management (files, database connections, network streams)
Pattern: Implement both Dispose() and finalizer for robust cleanup

[#interview]() [#dotnet]()

## Explain the differences between Interfaces and Abstract Classes in C #

%

Interface defines a contract with method signatures that implementing classes must provide, supporting multiple inheritance. Abstract Class provides a partial implementation with both abstract and concrete members, supporting single inheritance and shared functionality.

[#interview]() [#dotnet]()

## Explain the difference between Managed and Unmanaged code in .NET

%

Managed Code runs under the control of the Common Language Runtime (CLR) with automatic memory management, type safety, and garbage collection. Unmanaged Code runs directly on the operating system without CLR supervision, requiring manual memory management and offering direct hardware access.

[#interview]() [#dotnet]()

## Explain the differences between method overloading, method hiding (shadowing), and method overriding in C#. When would you use each?

%

Method overloading means same method name, different parameter signatures, resolved at compile time.
Method hiding uses `new` to shadow a base member, and behavior depends on the reference type.
Method overriding uses `virtual` + `override` to replace base behavior polymorphically, and dispatch happens at runtime.
Use overloading for related operations with different inputs, hiding for intentional API redefinition with backward compatibility caveats, and overriding for extensible inheritance-based behavior.

[#interview]() [#dotnet]()

## Can you return multiple values from a method in C#?

%

Yes - Multiple approaches available:
Tuples (C# 7+): Simple and clean syntax
out parameters: Traditional approach
Custom objects/classes: For complex scenarios
ref parameters: For modifying existing values

[#interview]() [#dotnet]()

## What is a Nullable type?

%

A type that can return a null value
Allows value types to represent null in addition to their normal range of values

[#interview]() [#dotnet]()

## Explain different ways of passing parameters to a method in C #

%

C# supports four parameter passing mechanisms: Value parameters (default, copies value), Reference parameters (ref keyword, passes reference), Output parameters (out keyword, must be assigned), and Parameter arrays (params keyword, variable number of arguments).

[#interview]() [#dotnet]()

## Explain the 'params' keyword in C# and provide usage examples

%

The params keyword allows a method to accept a variable number of arguments of the same type. It must be the last parameter in the method signature and creates an array internally to hold the passed arguments.

[#interview]() [#dotnet]()

## What is the difference between a Property and a Variable?

%

Variable: Direct data storage location in memory
Property: Abstraction layer with getter/setter methods for controlled access
Key Difference: Properties provide encapsulation, validation, and computed values

[#interview]() [#dotnet]()

## Explain the differences between String and StringBuilder in C# and when to use each

%

String is immutable - each modification creates a new string object, causing performance issues with frequent concatenations. StringBuilder is mutable and designed for efficient string manipulation operations, especially when performing multiple concatenations.

[#interview]() [#dotnet]()

## Explain the purpose and usage of the 'using' statement in C #

%

The using statement ensures that objects implementing IDisposable are properly disposed of when they go out of scope, providing automatic resource cleanup and preventing memory leaks and resource exhaustion.

[#interview]() [#dotnet]()

## What are virtual methods in C# and how do they work?

%

Virtual methods are methods in a base class that can be optionally overridden in derived classes. They enable polymorphism by allowing derived classes to provide their own implementation while maintaining the ability to call the base implementation.

Derived classes are classes that inherit from a base class. They reuse shared members (fields, properties, methods) and can extend or override behavior, especially virtual methods. Common synonyms are `child class`, `subclass`, and `extended class`. This mechanism supports inheritance and polymorphism, making code more reusable and easier to maintain.

[#interview]() [#dotnet]()

## What is the difference between delegates and events in .NET?

%

Delegates represent executable method references — they can be assigned, combined (multicast), passed as parameters, and invoked by anyone holding a reference.

Events wrap a delegate with restricted access: outside the declaring type, consumers can only `+=` (subscribe) or `-=` (unsubscribe). Only the declaring type can invoke the event, preventing accidental external invocation.

Use delegates for callbacks and strategy injection; use events for notification / Observer pattern to preserve encapsulation.

[#interview]() [#dotnet]()

## What is an ExpressionTree in C# and when would you use it?

%

An ExpressionTree (`System.Linq.Expressions`) is a data structure that represents code as a tree of nodes (method calls, operations, values) that can be **inspected, modified, or compiled at runtime** — unlike a delegate which is already compiled.

Key use cases:

- **LINQ providers** (Entity Framework) translate C# query trees into SQL.
- **Dynamic query building** at runtime.
- **Code generation** — compile and execute logic discovered at runtime.
- **Rule engines** and tools that need to analyze or transform code structure.

```csharp
Expression<Func<int, int>> expr = x => x + 1;
Console.WriteLine(expr.Body); // (x + 1)  — inspectable tree
var compiled = expr.Compile(); // now callable like a delegate
Console.WriteLine(compiled(5)); // 6
```

[#interview]() [#dotnet]()

## Explain .NET GC generations, compaction, and finalization in detail

%

**Generations**: The GC divides the heap into Gen 0, Gen 1, Gen 2. Short-lived objects start in Gen 0 (collected frequently). Survivors are promoted to older generations (collected less often). The Large Object Heap (LOH) is treated like Gen 2.

**Compaction**: After a collection, surviving objects are relocated to close memory gaps, updating all references. This reduces fragmentation and improves cache locality.

**Finalization**: Objects with finalizers (`~Destructor`) need at least two GC passes — one to discover they're unreferenced and queue them for finalization, a second to reclaim memory after the finalizer runs. This makes finalization slow and non-deterministic. Prefer `IDisposable` + `using` for deterministic cleanup; add a finalizer only as a safety net.

[#interview]() [#dotnet]()

## How do you declare and raise C# events? Provide a code example

%

An event is declared with the `event` keyword on a delegate type. By convention use `EventHandler<TEventArgs>`. Raise with `?.Invoke` to handle the null (no-subscriber) case safely.

```csharp
public class OrderPlacedEventArgs : EventArgs
{
    public int OrderId { get; }
    public OrderPlacedEventArgs(int id) => OrderId = id;
}

public class OrderService
{
    public event EventHandler<OrderPlacedEventArgs>? OrderPlaced;

    public void PlaceOrder(int id)
    {
        // business logic…
        OrderPlaced?.Invoke(this, new OrderPlacedEventArgs(id));
    }
}

// Subscriber
var svc = new OrderService();
svc.OrderPlaced += (_, e) => Console.WriteLine($"Order {e.OrderId} placed");
svc.PlaceOrder(42); // prints: Order 42 placed
```

Key rules: subscribers use `+=`/`-=` only; only the publisher can invoke. Always unsubscribe long-lived subscribers to prevent memory leaks.

[#interview]() [#dotnet]()

## What is the difference between factory registrations and conditional registrations in .NET DI?

%

**Timing is the core difference:**

| Approach | Decision point | Scope |
|---|---|---|
| Conditional registration | Startup / `Program.cs` | Fixed for app lifetime |
| Factory delegate | Every `GetService` call | Per-request / per-scope |

```csharp
// Conditional — decided once at startup
if (config["Provider"] == "Stripe")
    services.AddScoped<IPayment, StripeService>();
else
    services.AddScoped<IPayment, PayPalService>();

// Factory — decided on every resolve
services.AddScoped<IPayment>(sp =>
    sp.GetRequiredService<IConfiguration>()["Provider"] == "Stripe"
        ? sp.GetRequiredService<StripeService>()
        : sp.GetRequiredService<PayPalService>());
```

Use **factory** when the decision depends on per-request runtime state or you need decorator/strategy chains. Use **conditional** when the choice is fixed for the app lifetime. Guard double-registrations with `TryAdd*`; .NET 8+ keyed services replace the `Func<string,T>` factory trick.

[#interview]() [#dotnet]()
