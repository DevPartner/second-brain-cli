# 🎯 What Are React Mixins - L3

## Prompt MetadataL2

- Type: Refactor
- Status: executed
- Date: 2026-03-09 1773079985
- Tags: [#react, #mixins, #legacy-code]
- Project: interview
- Areas: [education]
- AI reviews:
  - 2026-03-09 | GPT-5.3-Codex (Copilot) | Review complete. Converted response to interview question-card format.

## Question

> **Core Question:** What were React mixins, and what replaced them in modern React?
> **Follow-up:** What was PureRenderMixin and how do you achieve similar optimization now?

## 💡 Quick Answer (30-60 seconds)

- Mixins were used with React.createClass to share behavior.
- They were replaced by ES6 classes, HOCs, and now mostly custom hooks.
- PureRenderMixin behavior maps to React.PureComponent or React.memo.

## 📖 Detailed Explanation

Mixins allowed multiple behavior bundles to be merged into one component, but they introduced naming collisions and implicit dependencies. React moved away from createClass toward ES6 classes and then function components with hooks. PureRenderMixin performed shallow prop and state checks to skip unnecessary renders. Today use React.PureComponent for class components or React.memo for function components. ES6 (ECMAScript 2015) was finalized in 2015.

```jsx
// Legacy idea (pre-ES6 class era)
const PureRenderMixin = require("react-addons-pure-render-mixin");

// Modern replacement
import { memo } from "react";

const Dashboard = memo(function Dashboard({ title }) {
  return <h1>{title}</h1>;
});
```

## 🧪 Practice Exercise

Take a legacy createClass component using mixins and rewrite it with a custom hook plus React.memo.

## 🔗 Related Topics

- #react-memo
- #custom-hooks
- #ecmascript-2015
