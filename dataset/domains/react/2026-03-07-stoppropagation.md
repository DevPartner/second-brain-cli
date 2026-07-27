# What does stopPropagation do in React events? - L1

## Prompt Metadata

- Type: explain
- Status: executed
- Date: 2026-03-09 1773079985
- Tags: [#react, #events, #stop-propagation]
- Project: interview
- Areas: [education]
- AI reviews:
  - 2026-03-09 | GPT-5.3-Codex (Copilot) | Review complete. Converted response to interview question-card format.

## Question

> **Core Question:** What does stopPropagation do in React events?
> **Follow-up:** How is it different from preventDefault?

## 💡 Quick Answer (30-60 seconds)

- stopPropagation prevents the event from bubbling to parent handlers.
- preventDefault cancels default browser behavior like link navigation.
- Use each only when needed to keep event flow predictable.

## 📖 Detailed Explanation

React synthetic events follow standard bubbling concepts. If a child click should not trigger a parent click handler, call e.stopPropagation in the child handler. preventDefault is separate and controls browser actions. In interviews, emphasize minimal event interference and clear intent.

```jsx
function Row() {
  return (
    <div onClick={() => console.log("row click")}>
      <button
        onClick={(e) => {
          e.stopPropagation();
          console.log("button click only");
        }}
      >
        Edit
      </button>
    </div>
  );
}
```

## 🧪 Practice Exercise

Add keyboard accessibility to the row and button pattern while preserving propagation behavior.

## 🔗 Related Topics

- #synthetic-events
- #event-bubbling
- #prevent-default
