# 🎯 HOC vs Hooks in React - L2

## Prompt Metadata

- Type: explain
- Status: executed
- Date: 2026-03-06 1772818772.514
- Tags: [#react, #hooks, #hoc]
- Project: interview
- Areas: [education]
- AI reviews:
  - 2026-03-06 | GPT-5.3-Codex (Copilot) | Review complete. Clarified Hooks vs HOCs with practical migration example.

## Question

> **Core Question:** What is HOC in the context of higher-order components, and how does it compare to Hooks?
> **Follow-up:** Do Hooks replace HOCs and render props in modern React?

## 💡 Quick Answer (30 seconds)

- A HOC is a function that takes a component and returns an enhanced component.
- Hooks are usually the simpler modern way to reuse stateful logic.
- Hooks do not fully replace HOCs, but they replace many common HOC use-cases.

## 📖 Detailed Explanation

A Higher-Order Component wraps an existing component and injects extra props or behavior without changing the wrapped component implementation. This pattern was very popular before hooks for concerns like data subscriptions, permissions, analytics, and theming. In interviews, mention that HOCs can create deep wrapper trees and prop name collisions if not designed carefully. Prefer hooks for new code, but understand HOCs for maintenance and third-party integrations.

```jsx
function withLoading(Wrapped) {
  return function WithLoading(props) {
    if (props.isLoading) return <p>Loading...</p>;
    return <Wrapped {...props} />;
  };
}

function UserList({ users }) {
  return <ul>{users.map((u) => <li key={u.id}>{u.name}</li>)}</ul>;
}

const UserListWithLoading = withLoading(UserList);
```

## 🧪 Practice Exercise

Create a withAuthorization HOC that renders "Access denied" when isAllowed is false.

## 🔗 Related Topics

- #custom-hooks
- #render-props
- #composition-vs-inheritance
