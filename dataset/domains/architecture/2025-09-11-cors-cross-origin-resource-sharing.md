---
type: "question"
status: draft
date: 2025-09-11 1757601633.860
tags: [question, cors, cross-origin, security, web-api, http-headers]
reviewed: 
  - date: 2025-09-11
---

# 🎯 CORS (Cross-Origin Resource Sharing) - L3

## Question

> **Core Question:** What is CORS? Why does it exist and how do you configure it in web applications? Explain preflight requests.
> **Category:** HTTP, Security

## Sources

- GitHub Copilot
- [1]: https://en.wikipedia.org/wiki/Cross-origin_resource_sharing

## 💡 Quick Answer (30 seconds)

CORS is a security mechanism that allows servers to specify which origins can access their resources from browsers. It exists to prevent malicious websites from accessing sensitive data. Configure it using headers like Access-Control-Allow-Origin.
Basically, it allows a web page to access restricted resources from a web server on a domain name different from the domain that served the web page.[1]

## 📖 Detailed Explanation

### What is CORS?

Cross-Origin Resource Sharing (CORS) is a security feature implemented by web browsers that restricts web pages from making requests to a different domain, protocol, or port than the one serving the web page, unless explicitly allowed by the target server.

### Why CORS Exists - Same-Origin Policy

```javascript
// Same-origin examples (allowed without CORS)
// Current page: https://example.com/page.html

// ✅ Same origin - same protocol, domain, and port
fetch('/api/data') // https://example.com/api/data

// ✅ Same origin - different path is OK
fetch('https://example.com/api/users')

// ❌ Cross-origin - different subdomain (blocked without CORS)
fetch('https://api.example.com/data')

// ❌ Cross-origin - different protocol (blocked without CORS)  
fetch('http://example.com/data')

// ❌ Cross-origin - different port (blocked without CORS)
fetch('https://example.com:8080/data')

// ❌ Cross-origin - different domain (blocked without CORS)
fetch('https://anotherdomain.com/api/data')
```

## 🧪 Practice Exercise

Build a comprehensive CORS implementation:

1. Configure CORS policies for different environments (dev/prod)
2. Implement dynamic CORS origin validation from database
3. Handle preflight requests with custom middleware
4. Create a frontend CORS debugging utility
5. Set up CORS for file uploads and complex requests
6. Implement security best practices and error handling

## 🔗 Related Topics

- Web security fundamentals
- HTTP headers and methods
- Authentication and authorization
- API design best practices
- Browser security policies
- [[2025-09-11-cors-implementation-security]]

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)  
- [ ] 🟢 Very confident (7-10)
