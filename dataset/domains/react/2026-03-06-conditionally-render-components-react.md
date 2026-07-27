---
type: "zknotes"
date: 2026-03-06 1772815251.848
tags: [conditional-render,components,react]
---

# conditionally render components react

## Source

From: Interview questions

## Summary

- *short-circuiting* conditional
- ternary operator

## Key Ideas

- *short-circuiting* conditional

```typescript
const MyComponent = ({ name, address }) => (
    <div>
        <h2>{name}</h2>
        {address && <p>{address}</p>}
    </div>
);
```

- ternary operator - `if-else`

```typescript
const MyComponent = ({ name, address }) => (
    <div>
        <h2>{name}</h2>
        {address ?
            <p>{address}</p>
        :   <p>{'Address is not available'}</p>}
    </div>
);
```

## Related

-
