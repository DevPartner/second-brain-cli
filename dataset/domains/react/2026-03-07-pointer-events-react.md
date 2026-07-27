# How does CSS pointer-events affect React interactions?

## Prompt Metadata

- Date: 2026-03-09 1773079985
- Tags: [#react, #css, #pointer-events]
- Project: interview
- Areas: [education]
- Difficulty: L2
- AI reviews:
  - 2026-03-09 | GPT-5.3-Codex (Copilot) | Review complete. Converted response to interview question-card format.

## Question

> **Core Question:** How does CSS pointer-events affect React interactions?
> **Follow-up:** When is pointer-events none useful in UI design?

## 💡 Quick Answer (30-60 seconds)

- React supports 10 pointer handlers: down, move, up, cancel, capture start/end,
  enter/leave, and over/out.
- CSS `pointer-events` controls hit-testing. With `pointer-events: none`, that
  element cannot be the target, so events pass through to elements below.
- Prefer Pointer Events over Mouse Events for modern UI because one API works
  for mouse, touch, and pen.

## ✅ Pointer Events in React DOM

1. `onPointerDown`: Fires when a pointer becomes active (mouse press, touch
   contact, pen contact).
2. `onPointerMove`: Fires when pointer coordinates change.
3. `onPointerUp`: Fires when the active pointer is released.
4. `onPointerCancel`: Fires when the browser/system cancels the pointer stream
   (gesture takeover, device interruption, etc.).
5. `onGotPointerCapture`: Fires when an element captures a specific pointer.
6. `onLostPointerCapture`: Fires when pointer capture is released.
7. `onPointerEnter`: Fires when pointer enters an element (non-bubbling,
   similar to `mouseenter`).
8. `onPointerLeave`: Fires when pointer leaves an element (non-bubbling,
   similar to `mouseleave`).
9. `onPointerOver`: Fires when pointer moves over an element (bubbling,
   similar to `mouseover`).
10. `onPointerOut`: Fires when pointer moves out of an element (bubbling,
    similar to `mouseout`).

## 🧭 Pointer vs Mouse Events (When to Use Which)

| Legacy mouse event | Pointer event equivalent                       | Use in modern React                                       |
| ------------------ | ---------------------------------------------- | --------------------------------------------------------- |
| `onMouseDown`      | `onPointerDown`                                | Use pointer for device-agnostic press interactions.       |
| `onMouseMove`      | `onPointerMove`                                | Use pointer for drag/paint/resize with mouse, touch, pen. |
| `onMouseUp`        | `onPointerUp`                                  | Use pointer to complete gesture across all input types.   |
| `onMouseEnter`     | `onPointerEnter`                               | Use when you need non-bubbling enter semantics.           |
| `onMouseLeave`     | `onPointerLeave`                               | Use when you need non-bubbling leave semantics.           |
| `onMouseOver`      | `onPointerOver`                                | Use bubbling over behavior in delegated handlers.         |
| `onMouseOut`       | `onPointerOut`                                 | Use bubbling out behavior in delegated handlers.          |
| _(none)_           | `onPointerCancel`                              | Needed for robust touch/pen gesture cancellation.         |
| _(none)_           | `onGotPointerCapture` / `onLostPointerCapture` | Needed for reliable drag across element boundaries.       |

## 📖 Detailed Explanation

React handlers still depend on browser hit-testing. If an element has
`pointer-events: none`, it will not become the event target, so handlers on
that element will not fire. This is useful for decorative overlays and icon
layers.

When `pointer-events: none` is useful:

- non-interactive visual layers (glow, gradient, watermark)
- text/icons inside a clickable parent where the parent should receive the click
- temporary interaction lock while keeping layout visible

Accessibility note: `pointer-events: none` is not the same as semantic disable.
For controls, also use `disabled`, `aria-disabled`, focus management, and clear
state styling.

```jsx
function Card() {
  const handlePointerDown = (e) => {
    e.currentTarget.setPointerCapture(e.pointerId);
    console.log("pointer down", e.pointerType);
  };

  const handlePointerMove = (e) => {
    if (e.currentTarget.hasPointerCapture(e.pointerId)) {
      console.log("pointer move", e.clientX, e.clientY);
    }
  };

  const handlePointerUp = (e) => {
    if (e.currentTarget.hasPointerCapture(e.pointerId)) {
      e.currentTarget.releasePointerCapture(e.pointerId);
    }
    console.log("pointer up");
  };

  return (
    <div style={{ position: "relative" }}>
      <button
        onPointerDown={handlePointerDown}
        onPointerMove={handlePointerMove}
        onPointerUp={handlePointerUp}
      >
        Drag or tap me
      </button>
      <div style={{ position: "absolute", inset: 0, pointerEvents: "none" }}>
        <span>Decorative overlay</span>
      </div>
    </div>
  );
}
```

## 🧪 Practice Exercise

Build a loading overlay that blocks interactions while loading and allows interactions after loading completes.

## 🔗 Related Topics

- #event-targeting
- #css-layout
- #accessibility
