---
type: "question"
status: draft
date: 2026-03-10 1773149315.473
tags: []
---

# 🎯 react-dom - L2

## Prompt Metadata

- Type: explain
- Status: executed
- Date: 2026-03-09 1773079985
- Tags: [#react, #react-dom, #hydration]
- Project: interview
- Areas: [education]
- AI reviews:
  - 2026-03-09 | GPT-5.3-Codex (Copilot) | Review complete. Converted response to interview question-card format.
  - 2026-03-05 | GPT-5.3-Codex (Copilot) | Review complete. Clarified react-dom role and hydrate behavior.

## Question

> **Core Question:** What is react-dom?
> **Follow-up:** What is hydration and when do you use hydrateRoot?

## 💡 Quick Answer (30-60 seconds)

- react defines components and hooks; react-dom connects React to browser DOM.
- Use createRoot for client rendering and hydrateRoot for SSR HTML.
- createPortal from react-dom renders UI into external DOM containers.

## 📖 Detailed Explanation

react-dom contains browser-specific rendering APIs. In client-only apps it mounts your React tree. In SSR apps hydration attaches event listeners and behavior to server-rendered HTML without rebuilding from scratch. Mention modern APIs createRoot and hydrateRoot; render and hydrate are legacy.

```jsx
import { createRoot, hydrateRoot } from "react-dom/client";
import App from "./App";

const el = document.getElementById("root");

if (el?.hasChildNodes()) {
  hydrateRoot(el, <App />);
} else {
  createRoot(el).render(<App />);
}
```

## 🧪 Practice Exercise

Explain to a teammate why SSR first paint is fast but hydration is still required for interactivity.

## 🔗 Related Topics

- #server-side-rendering
- #create-root
- #react-runtime
