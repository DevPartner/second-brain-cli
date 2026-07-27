---
type: "question"
status: draft
date: 2026-03-10 1773149029.778
tags: []
---

# 🎯 Portals in React - L2

## Prompt Metadata

- Type: explain
- Status: executed
- Date: 2026-03-09 1773079985
- Tags: [#react, #portals, #dom]
- Project: interview
- Areas: [education]
- AI reviews:
  - 2026-03-09 | GPT-5.3-Codex (Copilot) | Review complete. Converted response to interview question-card format.
  - 2026-03-05 | GPT-5.3-Codex (Copilot) | Review complete. Clarified portals and practical use-cases.

## Question

> **Core Question:** What are React portals and why are they useful?
> **Follow-up:** Do portals break React event bubbling?

## 💡 Quick Answer (30-60 seconds)

- Portals render JSX into a different DOM node outside the parent container.
- They are ideal for modals and overlays escaping overflow or z-index issues.
- Events still bubble through the React tree.

## 📖 Detailed Explanation

A portal lets you keep component ownership and state in one place while rendering UI elsewhere in the DOM. This solves layout constraints for dialogs and floating layers. In interviews, highlight that context and event propagation still work through the React tree, so parent handlers can still catch events from portal content.

```jsx
import { createPortal } from "react-dom";

function Modal({ open, onClose, children }) {
  if (!open) return null;

  return createPortal(
    <div className="backdrop" onClick={onClose}>
      <div className="dialog" onClick={(e) => e.stopPropagation()}>{children}</div>
    </div>,
    document.getElementById("modal-root")
  );
}
```

## 🧪 Practice Exercise

Build a tooltip rendered into #overlay-root using createPortal.

## 🔗 Related Topics

- #react-dom
- #event-bubbling
- #z-index
