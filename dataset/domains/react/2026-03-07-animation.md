---
agent: agent
description: "Explain What are the popular packages for animation?"
tools: ["read", "edit", "search", "web", "cognitionai/deepwiki/*", "agent"]
---

# What are popular animation libraries in React and when should you use each? - L2

## Prompt Metadata

- Type: explain
- Status: executed
- Date: 2026-03-09 1773079985
- Tags: [#react, #animation, #frontend]
- Project: interview
- Areas: [education]
- AI reviews:
  - 2026-03-09 | GPT-5.3-Codex (Copilot) | Review complete. Converted response to interview question-card format.

## Question

> **Core Question:** What are popular animation libraries in React and when should you use each?
> **Follow-up:** How do you choose between Framer Motion and GSAP?

## 💡 Quick Answer (30-60 seconds)

- Popular options include Framer Motion, React Spring, GSAP, and AutoAnimate.
- Framer Motion is ergonomic for component transitions and layout animation.
- GSAP is strongest for advanced timeline choreography.

## 📖 Detailed Explanation

Choose animation libraries based on complexity, team familiarity, and performance needs. Framer Motion is a common default for product UI and micro-interactions. React Spring is useful for physics-driven animation. GSAP excels in complex sequences and cross-framework animation work. Mention reduced-motion accessibility support in interview answers.

```jsx
import { motion } from "framer-motion";

export default function FadeCard() {
  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.3 }}
    >
      Hello motion
    </motion.div>
  );
}
```

## 🧪 Practice Exercise

Implement the same fade-in using Framer Motion and React Spring, then compare readability and control.

## 🔗 Related Topics

- #framer-motion
- #gsap
- #performance
