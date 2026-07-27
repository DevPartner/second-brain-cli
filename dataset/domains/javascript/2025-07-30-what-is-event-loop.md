---
type: "question"
status: executed
date: 2026-03-05 1772690341
project: knowledge-system
areas: education
evaluation_criteria: concise, actionable
tags: [javascript-interview, event-loop, call-stack, microtasks, macrotasks]
ai_reviews:
	- "2026-03-05 | GPT-5.3-Codex (Copilot) | Converted to question-card format with concise JavaScript examples."
---

# Question

> **Core Question:** What is the JavaScript event loop?
> **Follow-up:** Why do microtasks run before macrotasks, and what interview pitfall should you mention?

## 💡 Quick Answer (30 seconds)

- JavaScript runs synchronous code on a single call stack.
- Async work is handled by runtime APIs and queued for later execution.
- The event loop runs microtasks (like Promise callbacks) before the next macrotask (like `setTimeout`).

## 📖 Detailed Explanation

### Core parts

- **Call stack**: Executes synchronous code.
- **Runtime APIs**: Browser Web APIs or Node.js runtime perform async operations.
- **Macrotask queue**: `setTimeout`, I/O callbacks, UI events.
- **Microtask queue**: `Promise.then/catch/finally`, `queueMicrotask`, `MutationObserver`.
- **Rule**: Once current sync code finishes, all microtasks run before the next macrotask.

### Practical example

`setTimeout(..., 0)` is not immediate. It waits until:

1. Current call stack is empty.
2. Pending microtasks have been drained.

### Common pitfall

Creating very long microtask chains can delay timers, rendering, and input handling.

### Short JavaScript examples

```javascript
console.log('A');

setTimeout(() => {
  console.log('B - macrotask');
}, 0);

Promise.resolve().then(() => {
  console.log('C - microtask');
});

console.log('D');
// Output order: A, D, C, B
```

```javascript
let count = 0;
function schedule() {
  if (count < 3) {
    count += 1;
    queueMicrotask(schedule);
  }
}
schedule();
// Many chained microtasks can postpone macrotasks.
```

## 🧪 Practice Exercise

1. Predict output order for mixed sync code, Promises, and `setTimeout`.
2. Explain in one minute why microtasks run first.
3. Add a Node.js note on `process.nextTick` vs Promise microtasks.

## 🔗 Related Topics

- `process.nextTick` vs Promise microtasks (Node.js)
- Rendering and event loop performance

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
