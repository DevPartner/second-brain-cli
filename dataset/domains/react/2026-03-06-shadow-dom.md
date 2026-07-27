---
type: "zknotes"
date: 2026-03-06 1772880000
tags: [shadow-dom, web-components, react]
---

# What is Shadow DOM?

## Source

From: [[2026-03-05-shadow-dom-vs-virtual-dom]]

## Summary

- Shadow DOM is a browser-native encapsulation mechanism for markup and styles.
- It creates a local boundary so component internals are isolated from page-level CSS.

## Key Ideas

- A shadow root is attached to a host element and contains internal DOM and styles.
- Encapsulation is mainly for style and markup boundaries, not for security.
- Slots allow controlled content projection from light DOM into component templates.
- Event retargeting can change `event.target` outside the boundary; composed events can cross the boundary.

### HTML/CSS example

```html
<user-card name="Ana"></user-card>

<script>
  class UserCard extends HTMLElement {
    connectedCallback() {
      const root = this.attachShadow({ mode: 'open' });
      root.innerHTML = `
        <style>
          .card { border: 1px solid #d1d5db; padding: 8px; border-radius: 8px; }
          .name { color: #0f766e; font-weight: 600; }
        </style>
        <div class="card">
          <span class="name">${this.getAttribute('name')}</span>
          <slot></slot>
        </div>
      `;
    }
  }

  customElements.define('user-card', UserCard);
</script>
```

### React TypeScript interop example

```tsx
import { useEffect, useRef } from 'react';

export function ShadowPanel() {
  const hostRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    if (!hostRef.current) return;

    const root = hostRef.current.attachShadow({ mode: 'open' });
    root.innerHTML = `
      <style>
        .panel { padding: 12px; border: 1px solid #94a3b8; }
      </style>
      <div class="panel">Shadow content rendered outside React tree.</div>
    `;

    return () => {
      root.innerHTML = '';
    };
  }, []);

  return <div ref={hostRef} />;
}
```

## Related

- [[2026-03-05-shadow-dom-vs-virtual-dom]]
- [[2026-03-06-virtual-dom]]
