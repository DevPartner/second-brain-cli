---
type: "question"
status: draft
date: 2026-03-05 1772716800
tags: [question, rest, web-services, api-design]
reviewed:
  - date: 2026-03-05
---

# 🎯 Key Features of a RESTful Web Service - L2

## Question
>
> **Core Question:** What are the key features of a RESTful web service?
> **Category:** REST APIs

## 💡 Quick Answer (30 seconds)

RESTful services are resource-based, stateless, use standard HTTP semantics,
return clear representations, and are designed for cacheability and evolvability.

## 📖 Detailed Explanation

- **Resource orientation:** Nouns in URIs represent resources.
- **HTTP semantics:** Correct use of `GET`, `POST`, `PUT/PATCH`, `DELETE`.
- **Statelessness:** Each request carries required context.
- **Representation:** Typically JSON with explicit status codes.
- **Cacheability:** Responses can be cache-friendly when appropriate.
- **Uniform interface:** Predictable endpoint conventions.

## 🧪 Practice Exercise

Design endpoints for `orders` and map each CRUD operation to an HTTP verb and
status code.

## 🔗 Related Topics

- Idempotency
- HATEOAS (optional)
- API consistency standards

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
