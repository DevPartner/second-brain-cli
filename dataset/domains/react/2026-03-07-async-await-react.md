# How do you use async and await correctly in React components? - L2

## Prompt Metadata

- Type: explain
- Status: executed
- Date: 2026-03-09 1773079985
- Tags: [#react, #async-await, #data-fetching]
- Project: interview
- Areas: [education]
- AI reviews:
  - 2026-03-09 | GPT-5.3-Codex (Copilot) | Review complete. Converted response to interview question-card format.

## Question

> **Core Question:** How do you use async and await correctly in React components?
> **Follow-up:** Why should useEffect callback itself not be async?

## 💡 Quick Answer (30-60 seconds)

- Keep useEffect callback sync and call an inner async function.
- Handle loading and error states explicitly.
- Use cancellation guards to avoid setState after unmount.

## 📖 Detailed Explanation

useEffect expects either no return value or a cleanup function, so an async effect returns a Promise and breaks that contract. The safe pattern is defining an inner async function and calling it. For interviews, mention cancellation via boolean flags or AbortController and clear state transitions to avoid race conditions.

```jsx
import { useEffect, useState } from "react";

function Users() {
  const [data, setData] = useState([]);
  const [error, setError] = useState(null);

  useEffect(() => {
    let cancelled = false;

    async function load() {
      try {
        const res = await fetch("/api/users");
        const json = await res.json();
        if (!cancelled) setData(json);
      } catch (e) {
        if (!cancelled) setError(e);
      }
    }

    load();
    return () => {
      cancelled = true;
    };
  }, []);

  return <pre>{JSON.stringify({ data, error }, null, 2)}</pre>;
}
```

## 🧪 Practice Exercise

Update the example to use AbortController and handle non-200 HTTP responses.

## 🔗 Related Topics

- #useeffect
- #fetch-api
- #error-handling
