---
type: "zknotes"
date: 2026-03-05 1772692962.085
tags: [react, shadow-dom, virtual-dom]
---

# Shadow DOM vs Virtual DOM

## Source

From: GPT

## Summary

- Shadow DOM (encapsulation/runtime boundary) and Virtual DOM (UI diffing/render strategy) solve different problems.
- Shadow DOM is browser-native for isolation; Virtual DOM is framework-level for efficient updates.

## Key Ideas

- **Shadow DOM**: Encapsulation for markup and style boundaries.
- **Virtual DOM**: In-memory diffing strategy for efficient rendering updates.
- **Different goals**: Isolation vs update performance.
- **Can coexist**: A React app can render custom elements that use Shadow DOM.
- **Details moved to spokes**: [[2026-03-06-shadow-dom]], [[2026-03-06-virtual-dom]].

### Visual layout

```mermaid
flowchart LR
 A[User Interaction] --> B[React Component State Update]
 B --> C[Virtual DOM Diff]
 C --> D[Real DOM Patch]
 D --> E[Custom Element Host]
 E --> F[Shadow Root]
 F --> G[Encapsulated Markup + Styles]
```

## Related

- [[2026-03-05-reconciliation]]
- [[2026-03-06-shadow-dom]]
- [[2026-03-06-virtual-dom]]
