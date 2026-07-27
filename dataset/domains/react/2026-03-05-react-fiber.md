---
type: "zknotes"
date: 2026-03-05 1772693780.75
tags: [react, react-fiber, frontend-architecture,]
---

# React Fiber

## Source

From: GPT

## Summary

React Fiber is React’s reconciliation engine rewrite that enables interruptible, prioritized rendering. Instead of processing the entire component tree in one blocking pass, Fiber breaks work into units, schedules high-priority updates first, and can pause/resume work between frames. This improves UI responsiveness for interactions and animations while preserving React’s declarative programming model. Fiber also enabled features such as better error boundaries, concurrent rendering capabilities, and smoother handling of large update workloads.

## Key Ideas

1. **Core idea:** Fiber represents work as linked units (fibers), so React can process updates incrementally rather than all at once.
2. **Scheduling benefit:** React can prioritize urgent updates (e.g., user input) over less urgent rendering, reducing perceived lag.
3. **Feature impact:** Fiber is the foundation for modern React concurrency features and more resilient rendering/error handling.

## Related

- [[2026-03-05-reconciliation]]
