---
type: "zknotes"
date: 2026-03-05 1772693410.332
tags: [javascript, react, event-propagation]
---

# React SyntheticEvent

## Source

From: GPT

## Summary

`event.stopPropagation()` stops an event from bubbling (or capturing further) to parent listeners, but it does not cancel default browser behavior; use `preventDefault()` for that. In React, calling `e.stopPropagation()` on a `SyntheticEvent` prevents propagation in React's event system, and you can use `e.nativeEvent.stopPropagation()` when needed for native listeners.

## Key Ideas

- `stopPropagation()` controls event flow; `preventDefault()` controls default action.
- React's event model wraps native events; propagation behavior can differ when mixing React and native listeners.

## Related
