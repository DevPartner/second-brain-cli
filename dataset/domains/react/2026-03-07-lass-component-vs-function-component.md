# Explain: Class components vs function components in React: what is the practical difference today? L2

## Prompt Metadata

- Type: explain
- Status: executed
- Date: 2026-03-09 1773079985
- Tags: [#react, #class-components, #function-components]
- Project: interview
- Areas: [education]
- AI reviews:
  - 2026-03-09 | GPT-5.3-Codex (Copilot) | Review complete. Converted response to interview question-card format.

## Question

> **Core Question:** Class components vs function components in React: what is the practical difference today?
> **Follow-up:** When might you still keep a class component?

## 💡 Quick Answer (30-60 seconds)

- Function components with hooks are the modern default.
- Class components use lifecycle methods and this; function components use hooks.
- Keep class components mainly for legacy stability or specific edge cases.

## 📖 Detailed Explanation

Function components are simpler and more composable for modern React. Hooks replaced many lifecycle use cases with reusable logic. Class components remain valid in mature codebases where rewrite risk is high. In interviews, show migration awareness rather than saying classes are wrong.

```jsx
import React, { useState } from "react";

function CounterFn() {
  const [count, setCount] = useState(0);
  return <button onClick={() => setCount(count + 1)}>{count}</button>;
}

class CounterClass extends React.Component {
  state = { count: 0 };

  render() {
    return (
      <button onClick={() => this.setState({ count: this.state.count + 1 })}>
        {this.state.count}
      </button>
    );
  }
}
```

## 🧪 Practice Exercise

Pick one class component and describe an incremental migration plan to a hook-based function component.

## 🔗 Related Topics

- #react-hooks
- #lifecycle-methods
- #migration-strategy
