---
type: "question"
status: executed
date: 2026-03-05 1772690341
project: knowledge-system
areas: education
tags: [javascript-interview, angularjs, angular, filters, pipes, promises]
ai_reviews:
  - "2026-03-05 | GPT-5.3-Codex (Copilot) | Converted to question-card format with concise JavaScript examples."
---

# What is the difference between AngularJS filters, Angular pipes, and JavaScript promises?

> **Core Question:** What is the difference between AngularJS filters, Angular pipes, and JavaScript promises?
> **Follow-up:** How can you explain this safely in interviews when terminology gets mixed up?

## 💡 Quick Answer (30 seconds)

- Filters belong to AngularJS (1.x) and format values for display in templates.
- Pipes belong to Angular (2+) and do the same role in modern Angular.
- Promises are JavaScript async primitives for future values; they are not template display transforms.

## 📖 Detailed Explanation

### Comparison

| Concept  | Ecosystem       | Purpose                             | Typical Use                                                  | Key Point                                                 |
| -------- | --------------- | ----------------------------------- | ------------------------------------------------------------ | --------------------------------------------------------- |
| Filters  | AngularJS (1.x) | Format/transform values for display | Templates: `{{ value \| filterName }}` and `$filter` service | View-level transformation in AngularJS                    |
| Pipes    | Angular (2+)    | Format/transform values for display | Templates: `{{ value \| pipeName }}` with `@Pipe` classes    | Modern Angular replacement for AngularJS-style transforms |
| Promises | JavaScript      | Handle async results                | `fetch(...).then(...).catch(...)`                            | Async control flow, not display formatting                |

### Why this gets confused

- Filters and pipes look similar (`|`) but belong to different Angular generations.
- Promises are often discussed in the same app context, so people incorrectly group them with template transforms.

### Interview-safe phrasing

In AngularJS 1.x, template value formatting uses filters. In Angular 2+, this role is handled by pipes. Promises are separate JavaScript async primitives used for future values, not template formatting.

### Short JavaScript examples

```javascript
// Promise example (JavaScript async primitive)
fetch('/api/users')
  .then((response) => response.json())
  .then((users) => console.log(users.length))
  .catch((error) => console.error(error));
```

```javascript
// Similar syntax marker used by Angular templates, shown as strings
const angularJsTemplate = '{{ amount | currency }}';
const angularTemplate = '{{ amount | currency }}';
console.log(angularJsTemplate, angularTemplate);
```

## 🧪 Practice Exercise

1. Explain the difference in 30 seconds without using framework jargon incorrectly.
2. Write one AngularJS filter usage and one Angular pipe usage as template snippets.
3. Implement one Promise chain and describe why it is unrelated to template formatting.

## 🔗 Related Topics

- Observables vs Promises
- AngularJS to Angular migration terminology

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
