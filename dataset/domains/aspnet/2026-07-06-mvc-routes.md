---
type: "question"
status: draft
date: 2026-07-06 1783382400
tags: [aspnet, mvc, routing, framework]
---

# 🎯 MVC Routing Configuration - L2

## Question

> **Core Question:** By default, where do you write your routes for an MVC Application?
> **Follow-up:** What is the difference between conventional routing and attribute routing? When would you prefer one over the other?

## 💡 Quick Answer (30 seconds)

- **ASP.NET MVC (classic)**: routes are registered in `RouteConfig.cs` inside `App_Start/`, which calls `RegisterRoutes()` in `Global.asax`
- **ASP.NET Core**: routes are configured in `Program.cs` (or `Startup.cs`), using `app.MapControllerRoute()` or attribute routing with `[Route]`
- Routes are added to a **routing table** evaluated top-to-bottom on each request

## 📖 Detailed Explanation

In **classic ASP.NET MVC**, the routing table is populated at application startup inside `App_Start/RouteConfig.cs`. The `Application_Start` event in `Global.asax` calls `RouteConfig.RegisterRoutes(RouteTable.Routes)`. The default route pattern is `{controller}/{action}/{id}`.

In **ASP.NET Core**, there is no `Global.asax`. Routes are registered in the middleware pipeline (`Program.cs`). You can use:

- **Conventional routing** — patterns defined centrally (`MapControllerRoute`)
- **Attribute routing** — patterns defined on controller/action with `[Route]`, `[HttpGet]` etc.

Attribute routing is preferred for RESTful APIs because each endpoint declares its own URL contract. Conventional routing suits MVC web apps with predictable CRUD patterns.

```csharp
// ASP.NET Core — Program.cs
app.MapControllerRoute(
    name: "default",
    pattern: "{controller=Home}/{action=Index}/{id?}");

// Attribute routing on a controller
[ApiController]
[Route("api/[controller]")]
public class ProductsController : ControllerBase
{
    [HttpGet("{id}")]          // GET api/products/42
    public IActionResult Get(int id) { /* ... */ }

    [HttpPost]                 // POST api/products
    public IActionResult Create([FromBody] Product product) { /* ... */ }
}
```

## 🧪 Practice Exercise

You have two requirements: (1) a public website with `Home/Index`, `About/Index` etc. using conventional routes, and (2) a REST API under `/api/`. How would you configure routing in ASP.NET Core to support both in the same application?

## 🔗 Related Topics

- `MapControllerRoute` vs `MapDefaultControllerRoute`
- Attribute routing with `[Route]`, `[HttpGet]`, `[HttpPost]`
- Route constraints (`{id:int}`, `{slug:alpha}`)
- Area routing in large MVC applications

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
