# 🎯 Why Use `PropTypes.shape` in React - L2

## Prompt Metadata

- Type: explain
- Status: executed
- Date: 2026-03-06 1772826248.796
- Tags: [#react, #prop-types, #validation]
- Project: interview
- Areas: [education]
- AI reviews:
  - 2026-03-06 | GPT-5.3-Codex (Copilot) | Review complete. Clarified `PropTypes.shape` purpose and modern alternatives.

## Question

> **Core Question:** Why do we use `PropTypes.shape` in React?
> **Follow-up:** Is it related to Sass/CSS or 3D rendering?

## 💡 Quick Answer (30 seconds)

- `PropTypes.shape` validates object prop structure at runtime in JavaScript React apps.
- It is not related to styling tools like Sass and not related to 3D rendering.
- In modern projects, TypeScript often replaces this with compile-time typing.

## 📖 Detailed Explanation

`PropTypes.shape` is used to define expected keys and value types for object props. It helps catch incorrect prop data during development by logging warnings. It is useful in plain JavaScript React codebases where TypeScript is not used.

For arrays of objects, combine `PropTypes.arrayOf` and `PropTypes.shape`.

```jsx
import PropTypes from "prop-types";

function Palette({ swatches }) {
  return (
    <ul>
      {swatches.map((s) => (
        <li key={s.color} style={{ color: s.color, fontSize: s.fontSize }}>
          {s.color}
        </li>
      ))}
    </ul>
  );
}

Palette.propTypes = {
  swatches: PropTypes.arrayOf(
    PropTypes.shape({
      color: PropTypes.string.isRequired,
      fontSize: PropTypes.number.isRequired,
    }),
  ).isRequired,
};
```

## 🧪 Practice Exercise

Create a `UserList` component that accepts `users`, where each user must have `id`, `name`, and optional `avatarUrl`, and validate it using `arrayOf(shape(...))`.

## 🔗 Related Topics

- PropTypes vs TypeScript
- Runtime vs compile-time validation
- Component API contracts
