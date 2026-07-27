---
type: "question"
status: draft
date: 2025-09-11 1999999999
tags: [question, http, cors, web-security, cross-origin, api-design]
reviewed: 
  - date: 2025-09-11
---

# 🎯 CORS Implementation and Security - L2

## Question
>
> **Core Question:** What is CORS? How does it work?
> **Follow-up:** How do you configure CORS in ASP.NET Core? What are the security implications?

## 💡 Quick Answer (30 seconds)

- **CORS**: Browser-enforced policy that controls whether a frontend from one origin can call a server on another origin
- **Purpose**: Controlled relaxation of the Same-Origin Policy
- **How it works**: Browser sends an `Origin` header; for complex requests it first sends an `OPTIONS` preflight
- **Security**: It is not authentication or authorization; it only decides whether the browser may expose the response to frontend JavaScript

## 📖 Detailed Explanation

### Understanding Same-Origin Policy

The **Same-Origin Policy** blocks browser JavaScript from freely reading responses from a different:

- Protocol: `http` vs `https`
- Host: `app.example.com` vs `api.example.com`
- Port: `:3000` vs `:8080`

Example: if the page is served from `https://app.example.com`, then a call to
`https://api.example.com` is cross-origin and needs CORS approval from the API.

### What is CORS?

**Cross-Origin Resource Sharing (CORS)** is an HTTP header-based negotiation between the browser and the server.

- The browser sends the request origin.
- The server decides whether that origin, method, and headers are allowed.
- The browser enforces the result.

Important: non-browser clients like Postman or backend services are not protected by CORS.

### How CORS Works

#### Request flow

```plaintext
Frontend https://app.example.com
        |
        | 1. Sends request with Origin header
        v
API https://api.example.com
        |
        | 2. Returns CORS headers if origin is allowed
        v
Browser decides whether frontend JavaScript can read the response

For complex requests:
1. Browser sends OPTIONS preflight
2. Server returns allowed origins, methods, and headers
3. Browser sends the real request only if preflight succeeds
```

#### When preflight happens

Preflight is common when the request uses:

- Methods other than simple `GET`, `HEAD`, `POST`
- Custom headers such as `Authorization`
- Content types like `application/json` in scenarios that do not qualify as simple requests

### CORS Configuration in ASP.NET Core

Use a named policy with explicit origins and minimal permissions.

```csharp
var builder = WebApplication.CreateBuilder(args);

builder.Services.AddCors(options =>
{
    options.AddPolicy("Frontend", policy => policy
        .WithOrigins("https://app.myapp.com")
        .WithMethods("GET", "POST")
        .WithHeaders("Content-Type", "Authorization")
        .AllowCredentials());
});

builder.Services.AddControllers();

var app = builder.Build();
app.UseCors("Frontend");
app.UseAuthentication();
app.UseAuthorization();
app.MapControllers();
app.Run();
```

Use per-endpoint or per-group policies only when different endpoints need different trust boundaries.

```csharp
app.MapGroup("/partner")
   .RequireCors("PartnerPolicy")
   .MapGet("/health", () => Results.Ok("ok"));
```

### Security Considerations

#### Interview-ready checklist

- CORS protects browser clients only; it does not replace authentication or authorization.
- Prefer explicit origin allowlists over `AllowAnyOrigin()`.
- Never combine wildcard origins with credentials.
- Restrict methods and headers to what the frontend actually needs.
- Keep dev and prod policies separate.
- Use dynamic origin validation only for clearly controlled tenant or partner domains.

## 🧪 Practice Exercise

**Challenge:** Explain how you would configure three policies for:

- Customer SPA
- Admin portal
- Trusted partner domains

**Interview answer:** use separate named policies, keep explicit origins for internal frontends, and use a tightly validated dynamic rule only for trusted partner subdomains.

## 🔗 Related Topics

- **Same-Origin Policy**
- **Web Security Headers**
- **API Authentication and Authorization**
- **Preflight Requests**
- **CSP (Content Security Policy)**
- [[2025-09-11-cors-cross-origin-resource-sharing]]

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)  
- [ ] 🟢 Very confident (7-10)
