---
type: "question"
status: draft
date: 2025-09-11 1757601633.854
tags: [question, method-overloading, method-hiding, polymorphism, inheritance, virtual-methods]
reviewed: 
  - date: 2025-09-11
---

# 🎯 Method Overloading vs Method Hiding vs Method Overriding - L3

## Question
>
> **Core Question:** Explain the differences between method overloading, method hiding (shadowing), and method overriding in C#. When would you use each?
> **Category:** Framework

## 💡 Quick Answer (30 seconds)

Method overloading means same method name, different parameter signatures, resolved at compile time.
Method hiding uses `new` to shadow a base member, and behavior depends on the reference type.
Method overriding uses `virtual` + `override` to replace base behavior polymorphically, and dispatch happens at runtime.
Use overloading for related operations with different inputs, hiding for intentional API redefinition with backward compatibility caveats, and overriding for extensible inheritance-based behavior.

### C# Example

```csharp
using System;

class Base
{
 public void Print(int value) => Console.WriteLine($"Base.Print(int): {value}");
 public virtual void Show() => Console.WriteLine("Base.Show()");
 public void HideMe() => Console.WriteLine("Base.HideMe()");
}

class Derived : Base
{
 public void Print(string value) => Console.WriteLine($"Derived.Print(string): {value}"); // Overloading
 public override void Show() => Console.WriteLine("Derived.Show()"); // Overriding
 public new void HideMe() => Console.WriteLine("Derived.HideMe()"); // Hiding
}

class Program
{
 static void Main()
 {
  Derived d = new Derived();
  Base b = d;

  d.Print(42);          // Base.Print(int)
  d.Print("hello");    // Derived.Print(string)

  b.Show();             // Derived.Show()  (override, runtime)
  b.HideMe();           // Base.HideMe()   (hide, reference type)
  d.HideMe();           // Derived.HideMe()
 }
}
```

## 🔗 Related Topics

- Polymorphism and inheritance
- Virtual methods and abstract classes
- Method resolution and binding
- Interface implementation
- Generics and method constraints

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)  
- [ ] 🟢 Very confident (7-10)
