---
type: "question"
status: draft
date: 2026-07-06 1783382400
tags: [aspnet, mvc, dotnet-core, request-lifecycle, routing, middleware]
---

# 🎯 ASP.NET Request Lifecycle (Browser to Server and Back) - L3

## Question

> **Core Question:** Explain basic request handling from a browser to a server and back in an ASP.NET MVC / Core / Blazor application.
> **Follow-up:** How does the middleware pipeline in ASP.NET Core differ from the HTTP modules pipeline in classic ASP.NET?

## 💡 Quick Answer (30 seconds)

- **Browser** sends an HTTP request to the server (IIS / Kestrel)
- **Middleware pipeline** processes the request (auth, logging, routing, etc.)
- **Router** matches URL to a Controller + Action
- **Action** runs: model binding → validation → business logic → returns an `IActionResult`
- **View Engine** (Razor) renders HTML (for MVC) or the result is serialized (for API)
- **Response** travels back through the middleware pipeline to the browser

## 📖 Detailed Explanation

### ASP.NET Core pipeline

```plaintext
Browser
  └─► Kestrel / IIS (HTTP server)
        └─► Middleware pipeline
              ├── UseExceptionHandler
              ├── UseHttpsRedirection
              ├── UseStaticFiles
              ├── UseAuthentication
              ├── UseAuthorization
              └── UseRouting → MapControllers
                    └─► Controller.Action()
                          ├── Model Binding (query/body/route → .NET types)
                          ├── Model Validation (DataAnnotations / FluentValidation)
                          ├── Action execution (business logic / service calls)
                          └── IActionResult (View / JsonResult / RedirectResult)
                                └─► Razor View Engine (for MVC views)
  ◄─── HTTP Response
```

### Classic ASP.NET MVC pipeline

```plaintext
Browser → IIS → Global.asax (Application_Start)
  → HTTP Modules (authentication, session, etc.)
  → RouteConfig (routing table lookup)
  → Controller Factory → Controller
  → Action (model binding + filters)
  → ActionResult → View Engine (Razor/ASPX)
  → HTTP Response
```

### Blazor (Server-side)

[[2026-07-07-what-is-blazor]]

## 🧪 Practice Exercise

Draw (or describe) the sequence of events when a user navigates to `https://myapp.com/orders/42`:

1. What happens at the network level?
2. Which middleware runs before routing?
3. How does the router decide which controller action to invoke?
4. What returns the response to the browser?

## 🔗 Related Topics

- ASP.NET Core middleware pipeline (`IMiddleware`, `Use`, `Run`, `Map`)
- Action filters (`IActionFilter`, `IAsyncActionFilter`)
- Model binding and `[FromBody]`, `[FromQuery]`, `[FromRoute]`
- `IActionResult` and result types (`OkResult`, `ViewResult`, etc.)
- Kestrel vs IIS hosting models

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
