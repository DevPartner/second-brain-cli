---
type: "question"
status: draft
date: 2026-03-05 1772716800
tags: [question, web-services, rpc, rest, soap]
reviewed:
  - date: 2026-03-05
---

# 🎯 RPC vs Other Web Services - L2

## Question
>
> **Core Question:** What is RPC and how is it different from other web services?
> **Category:** Web Services

## 💡 Quick Answer (30 seconds)

RPC (Remote Procedure Call) is action-oriented: clients call remote methods.
REST is resource-oriented, while SOAP is protocol-standardized and message-heavy.

## 📖 Detailed Explanation

- **RPC focus:** Invoke operations (e.g., `CreateInvoice`).
- **REST focus:** Manipulate resources via HTTP verbs and URIs.
- **SOAP focus:** Structured messages with formal standards/compliance.
- **Trade-off:** RPC can be intuitive but may couple clients tightly to method
  contracts.

## 🧪 Practice Exercise

Rewrite an RPC-style operation (`ApproveOrder`) into a RESTful resource-based
endpoint design.

## 🔗 Related Topics

- gRPC
- Resource modeling
- API evolvability

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
