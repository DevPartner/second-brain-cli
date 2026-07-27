---
type: "question"
status: draft
date: 2026-03-05 1772716800
tags: [question, web-services, soap, rest, api-design]
reviewed:
  - date: 2026-03-05
---

# 🎯 SOAP vs REST - L2

## Question
>
> **Core Question:** What is the difference between SOAP and REST services?
> **Category:** Web Services

## 💡 Quick Answer (30 seconds)

SOAP is a strict protocol with XML envelopes and strong standards support.
REST is an architectural style over HTTP, usually lighter and often JSON-based.

## 📖 Detailed Explanation

- **Type:** SOAP = protocol; REST = architectural style.
- **Format:** SOAP commonly uses XML; REST supports multiple formats (JSON common).
- **Contract:** SOAP often uses WSDL; REST commonly uses OpenAPI or conventions.
- **Features:** SOAP has built-in standards (WS-*); REST favors HTTP simplicity.
- **Fit:** SOAP for strict enterprise contracts; REST for web/mobile APIs.

## 🧪 Practice Exercise

For a banking integration and a public mobile API, choose SOAP or REST and
justify each decision in one sentence.

## 🔗 Related Topics

- WSDL
- OpenAPI
- API security (OAuth, mTLS)

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
