# 🎯 Prop Destructuring with `...rest` - L2

## Prompt Metadata

- Type: explain
- Status: executed
- Date: 2026-03-06 1772816969.054
- Tags: [#react, #props, #jsx]
- Project: interview
- Areas: [education]
- AI reviews:
  - 2026-03-06 | GPT-5.3-Codex (Copilot) | Review complete. Clarified safe prop forwarding with rest destructuring.

## Question

> **Core Question:** Why should we use prop destructuring with `...rest` in React components?
> **Follow-up:** Why is spreading all props directly on DOM elements risky?

## 💡 Quick Answer (30 seconds)

- Destructure custom props first, then forward only DOM-safe props with `...rest`.
- Blind spreading can leak internal props to native elements and cause warnings.
- This pattern keeps components clean and reusable.

## 📖 Detailed Explanation

When a component receives both internal props (`isVisible`, `variant`) and native DOM props (`className`, `aria-*`), you should separate them. If you spread everything into a `<div>`, React can pass unsupported attributes to the DOM.

Destructuring keeps intent explicit: internal props stay internal; DOM props are forwarded.

```jsx
function Banner({ isVisible, title, ...domProps }) {
  if (!isVisible) return null;

  return (
    <section {...domProps}>
      <h2>{title}</h2>
    </section>
  );
}

// `isVisible` is consumed by Banner and is not forwarded to the DOM.
<Banner isVisible={true} title="Hello" className="hero" aria-live="polite" />;
```

## 🧪 Practice Exercise

Build a `Button` component with custom props (`variant`, `isLoading`) and forward only valid button attributes using `...rest`.

## 🔗 Related Topics

- Controlled prop forwarding
- Accessibility attributes (`aria-*`)
- Type-safe component props
