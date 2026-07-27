---
type: "question"
status: draft
date: 2025-09-11 1757601633.852
tags: [question, csharp, virtual-methods, inheritance, polymorphism, override]
reviewed: 
  - date: 2025-09-11
---

# 🎯 Virtual Methods in C# - L2

## Question
>
> **Core Question:** What are virtual methods in C# and how do they work?
> **Category:** Framework

## 💡 Quick Answer (30 seconds)

Virtual methods are methods in a base class that can be optionally overridden in derived classes. They enable polymorphism by allowing derived classes to provide their own implementation while maintaining the ability to call the base implementation.

## 📖 Detailed Explanation

**What are Virtual Methods?**

Virtual methods provide a mechanism for runtime polymorphism in C#. When a method is declared as `virtual` in a base class, derived classes can override it to provide specialized behavior while maintaining the same method signature.

**Virtual vs Abstract vs Override:**

| Keyword | Purpose | Must Implement | Can Have Body |
|---------|---------|----------------|---------------|
| `virtual` | Can be overridden | No | Yes |
| `abstract` | Must be overridden | Yes | No |
| `override` | Overrides virtual/abstract | Yes | Yes |

**Method Hiding vs Method Overriding:**

```csharp
public class BaseClass
{
    public virtual void VirtualMethod()
    {
        Console.WriteLine("Base Virtual Method");
    }
    
    public void RegularMethod()
    {
        Console.WriteLine("Base Regular Method");
    }
}

public class DerivedClass : BaseClass
{
    // Method overriding (polymorphic)
    public override void VirtualMethod()
    {
        Console.WriteLine("Derived Override Method");
    }
    
    // Method hiding (non-polymorphic)
    public new void RegularMethod()
    {
        Console.WriteLine("Derived New Method");
    }
}

// Demonstration
BaseClass baseRef = new DerivedClass();
baseRef.VirtualMethod(); // Output: "Derived Override Method" (polymorphic)
baseRef.RegularMethod(); // Output: "Base Regular Method" (not polymorphic)
```

**Best Practices:**

- Use virtual methods when you want to provide default behavior that derived classes can optionally customize
- Always use the `override` keyword when implementing virtual methods in derived classes
- Consider whether a method should be virtual, abstract, or sealed based on your design requirements
- Use `base.MethodName()` to call the base implementation when extending behavior
- Virtual method calls have slight performance overhead compared to non-virtual calls
- Be careful with virtual method calls in constructors, as the derived class may not be fully initialized

## 🔗 Related Topics

- Abstract classes and methods
- Interface implementation
- Polymorphism and inheritance
- Method hiding with `new` keyword
- Sealed methods and classes

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)  
- [ ] 🟢 Very confident (7-10)
