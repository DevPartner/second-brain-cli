---
type: "question"
status: draft
date: 2026-07-06 1783382400
tags: [aspnet, iis, http-handler, http-module, pipeline, classic-aspnet]
---

# 🎯 HTTP Handler vs HTTP Module in ASP.NET - L3

## Question

> **Core Question:** What is an HTTP handler and an HTTP module in ASP.NET? What are they used for and when do you use each?
> **Follow-up:** What replaced HTTP handlers and HTTP modules in ASP.NET Core?

## 💡 Quick Answer (30 seconds)

- **HTTP Handler** (`IHttpHandler`) — processes a specific request and generates a response; mapped to file extensions or paths; replaces the default page handler
- **HTTP Module** (`IHttpModule`) — plugs into every request/response cycle; used for cross-cutting concerns like logging, auth, URL rewriting
- In **ASP.NET Core** both are replaced by **middleware** (`IMiddleware` / `app.Use(...)`)

## 📖 Detailed Explanation

### HTTP Handler

An HTTP handler runs in response to a specific request and is fully responsible for generating the response. It implements `IHttpHandler` and is typically mapped to a custom file extension or URL path in `web.config`.

Use case: generating image thumbnails, serving dynamic files, custom REST endpoints in classic WebForms apps.

```csharp
// HTTP Handler — classic ASP.NET
public class ImageHandler : IHttpHandler
{
    public bool IsReusable => false;

    public void ProcessRequest(HttpContext context)
    {
        context.Response.ContentType = "image/png";
        var id = context.Request.QueryString["id"];
        var bytes = ImageService.GetThumbnail(id);
        context.Response.BinaryWrite(bytes);
    }
}
```

```xml
<!-- web.config registration -->
<system.webServer>
  <handlers>
    <add name="ImageHandler" path="*.thumb" verb="GET"
         type="MyApp.ImageHandler, MyApp" />
  </handlers>
</system.webServer>
```

### HTTP Module

An HTTP module subscribes to events in the ASP.NET request pipeline (e.g., `BeginRequest`, `AuthenticateRequest`, `EndRequest`) and runs on **every** request. It implements `IHttpModule`.

Use case: custom authentication, request logging, response compression, URL rewriting.

```csharp
// HTTP Module — classic ASP.NET
public class LoggingModule : IHttpModule
{
    public void Init(HttpApplication app)
    {
        app.BeginRequest += OnBeginRequest;
        app.EndRequest   += OnEndRequest;
    }

    private void OnBeginRequest(object sender, EventArgs e)
    {
        var ctx = ((HttpApplication)sender).Context;
        ctx.Items["StartTime"] = DateTime.UtcNow;
    }

    private void OnEndRequest(object sender, EventArgs e)
    {
        var ctx = ((HttpApplication)sender).Context;
        var elapsed = DateTime.UtcNow - (DateTime)ctx.Items["StartTime"];
        // log elapsed...
    }

    public void Dispose() { }
}
```

### ASP.NET Core equivalent — Middleware

Both concepts are unified into **middleware** in ASP.NET Core. Middleware can short-circuit (handler-like) or pass through (module-like).

```csharp
// Replaces both Handler and Module in ASP.NET Core
app.Use(async (context, next) =>
{
    var start = DateTime.UtcNow;
    await next();                    // pass to next middleware
    var elapsed = DateTime.UtcNow - start;
    // log elapsed
});

app.MapGet("/thumbnail/{id}", async (string id, ImageService svc) =>
{
    var bytes = await svc.GetThumbnailAsync(id);
    return Results.File(bytes, "image/png");  // handler-style endpoint
});
```

## 🧪 Practice Exercise

You need to add request timing to every response header (`X-Response-Time: 42ms`) in a classic ASP.NET MVC app. Would you use an HTTP handler or an HTTP module? Write the skeleton implementation.

## 🔗 Related Topics

- ASP.NET Core middleware pipeline (`IMiddleware`, `app.Use`, `app.Run`, `app.Map`)
- `IApplicationBuilder` and middleware ordering
- Generic host vs WebHost in ASP.NET Core
- `HttpContext` in classic ASP.NET vs ASP.NET Core

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
