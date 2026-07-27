---
type: "zknotes"
date: 2026-03-06 1772880001
tags: [virtual-dom, react, reconciliation]
---

# What is Virtual DOM?

## Source

From: [[2026-03-05-shadow-dom-vs-virtual-dom]]

## Summary

- Virtual DOM is a framework-level in-memory representation of UI.
- React compares trees and applies minimal real DOM updates through [[2026-03-05-reconciliation|reconciliation]].

## Key Ideas

- On state or props change, React builds a new virtual tree.
- A diff step determines what changed between previous and new trees.
- [[2026-03-05-reconciliation|Reconciliation]] applies only required mutations to the real DOM.
- Virtual DOM improves update strategy; it does not provide style encapsulation.
- Shadow DOM and Virtual DOM solve different problems and can be used together.

### React example

```tsx
import { useState } from 'react';

export function Counter() {
  const [count, setCount] = useState(0);

  return (
    <button onClick={() => setCount((c) => c + 1)}>
      Count: {count}
    </button>
  );
}
```

When `count` changes, React creates a new virtual tree, diffs it, and patches only the text node that changed.

## Related

- [[2026-03-05-shadow-dom-vs-virtual-dom]]
- [[2026-03-05-reconciliation]]
- [[2026-03-06-shadow-dom]]
