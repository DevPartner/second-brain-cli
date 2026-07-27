---
type: "question"
status: draft
date: 2026-03-10 1773149648.554
tags: []
---

# 🎯 Combine multiple inline style objects - L1

## Prompt Metadata

- Type: Implement
- Status: executed
- Date: 2026-03-09 1773079985
- Tags: [#react, #inline-styles, #javascript]
- Project: interview
- Areas: [education]
- AI reviews:
  - 2026-03-09 | GPT-5.3-Codex (Copilot) | Review complete. Converted response to interview question-card format.

## Question

> **Core Question:** How do you combine multiple inline style objects in React safely?
> **Follow-up:** How do you conditionally merge styles without mutating shared objects?

## 💡 Quick Answer (30-60 seconds)

- Merge style objects with spread syntax.
- Later objects override earlier keys.
- Keep style objects immutable to avoid side effects.

## 📖 Detailed Explanation

React inline styles are plain JavaScript objects. Compose them with **spread syntax** or Object.assign, but avoid mutating shared constants. For conditional styling, combine base, variant, and runtime fragments. In interviews, mention className is often better for complex styling while inline styles are useful for dynamic values.

```jsx
const base = { padding: 8, borderRadius: 6 };
const primary = { backgroundColor: "#2563eb", color: "white" };

function Button({ danger }) {
  const style = {
    ...base,
    ...(danger ? { backgroundColor: "#dc2626" } : primary),
    cursor: "pointer",
  };

  return <button style={style}>Save</button>;
}
```

## 🧪 Practice Exercise

Refactor a component with duplicated inline styles into reusable base and variant style objects.

## 🔗 Related Topics

- #object-spread
- #css-in-js
- #style-immutability
