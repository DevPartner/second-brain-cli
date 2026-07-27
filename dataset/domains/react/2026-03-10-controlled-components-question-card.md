---
type: "question"
status: draft
date: 2026-03-10 1773148439.996
tags: []
---

# What are controlled components in React? - L1

## Prompt Metadata

- Type: explain
- Status: executed
- Date: 2026-03-09 1773079985
- Tags: [#react, #forms, #controlled-components]
- Project: interview
- Areas: [education]
- AI reviews:
  - 2026-03-09 | GPT-5.3-Codex (Copilot) | Review complete. Converted response to interview question-card format.
  - 2026-03-05 | GPT-5.3-Codex (Copilot) | Review complete. Added controlled vs uncontrolled comparison and examples.

## Question

> **Core Question:** What are controlled components in React?
> **Follow-up:** When would you choose uncontrolled components instead?

## 💡 Quick Answer (30-60 seconds)

- Controlled components store form values in React state.
- The input value is driven by value plus onChange.
- They make validation and submit logic predictable.

## 📖 Detailed Explanation

In a controlled component, React state is the single source of truth for form inputs. Every change event updates state, then state re-renders the input value. This is ideal for validation, dynamic form behavior, and integrating business rules. Uncontrolled inputs can be useful for quick forms, file inputs, or performance-sensitive cases where values are read via refs only on submit.

```jsx
import { useState } from "react";

export default function LoginForm() {
  const [email, setEmail] = useState("");

  return (
    <form>
      <input
        value={email}
        onChange={(e) => setEmail(e.target.value)}
        placeholder="Email"
      />
    </form>
  );
}
```

## 🧪 Practice Exercise

Add inline validation so the submit button is disabled when email does not contain @.

## 🔗 Related Topics

- #uncontrolled-components
- #react-state
- #form-validation
