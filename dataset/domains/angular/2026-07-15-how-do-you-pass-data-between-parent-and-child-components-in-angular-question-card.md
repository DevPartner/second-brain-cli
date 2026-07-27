---
type: "question"
status: draft
date: 2026-07-15 1784091837.568
tags: [angular, component-communication]
---

# How do you pass data between parent and child components in Angular? - L2

## Question

> **Core Question:** How do you pass data between parent and child components in Angular?
> **Follow-up:** When would you choose a shared service over @Input/@Output?

## 💡 Quick Answer (30-60 seconds)

- Use @Input for data flowing from parent to child.
- Use @Output and EventEmitter for events flowing back to the parent.
- For shared state or cross-cutting data, use a service with RxJS or signals.

## 📖 Detailed Explanation

Angular components are usually connected through well-defined boundaries. @Input is the simplest choice when a child needs configuration or data from its parent, while @Output is appropriate when the child must notify the parent about an action. If multiple components need the same data or the communication becomes more complex, a shared service is usually a better fit because it centralizes state and avoids tightly coupled component relationships.

```typescript
@Component({
  selector: "app-child",
  template: "<p>{{ name }}</p>",
})
export class ChildComponent {
  @Input() name = "";
}
```

## 🧪 Practice Exercise

Refactor a small parent-child form so the child component emits a submit event back to the parent.

## 🔗 Related Topics

- #angular-inputs-outputs
- #angular-services
- #rxjs
