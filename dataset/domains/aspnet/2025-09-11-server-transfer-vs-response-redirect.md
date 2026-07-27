---
type: "question"
status: draft
date: 2025-09-11 1699999999
tags: [question, asp-net, web-forms, server-transfer, response-redirect]
reviewed: 
  - date: 2025-09-11
---

# 🎯 Server.Transfer vs Response.Redirect - L3

## Question
>
> **Core Question:** What is the difference between Server.Transfer and Response.Redirect?
> **Follow-up:** When would you use each method and what are the performance implications?

## 💡 Quick Answer (30 seconds)

- **Response.Redirect**: Client-side redirect (HTTP 302), new browser request, URL changes
- **Server.Transfer**: Server-side transfer, same request, URL stays the same, faster
- **Key Difference**: Redirect involves round-trip to client, Transfer happens server-side only

## 📖 Detailed Explanation

### Response.Redirect

**How it works:**

1. Server sends HTTP 302 status code to browser
2. Browser receives redirect response with new URL
3. Browser makes new request to the new URL
4. Server processes the new request

```csharp
// Basic redirect
Response.Redirect("NewPage.aspx");

// Redirect with query parameters
Response.Redirect("UserProfile.aspx?id=" + userId);

// Redirect to external site
Response.Redirect("https://www.example.com");

// Redirect with end response (default behavior)
Response.Redirect("NewPage.aspx", true);

// Redirect without ending response (ASP.NET 2.0+)
Response.Redirect("NewPage.aspx", false);
```

**Characteristics:**

- **Round-trip**: Involves client browser
- **URL Change**: Browser URL updates to new page
- **Status Code**: HTTP 302 (Temporary Redirect) or 301 (Permanent)
- **Context**: New request context created
- **ViewState**: Lost between pages
- **Performance**: Slower due to round-trip

## 🔗 Related Topics

- **ASP.NET Page Lifecycle**
- **HTTP Status Codes**
- **Post-Redirect-Get Pattern**
- **Session State Management**
- **ASP.NET Core Routing**
- **Open Redirect Vulnerabilities**

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)  
- [ ] 🟢 Very confident (7-10)
