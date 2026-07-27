---
type: "question"
status: draft
date: 2026-04-21 1773149315.473
description: "Explain Does React Router v4 deprecated?"
tags: [react-router, react-router-v4, react-router-v6, routing, react-jsx]
---

# Explain: Does React Router v4 deprecated?

## Question

> **Core Question:** Is React Router v4 deprecated?
> **Follow-up:** If I have a v4 codebase that uses `withRouter`, what should I migrate to in modern React Router (v6+/v7 direction)?

## Quick Answer

- React Router v4 is legacy in 2026 and should not be chosen for new apps.
- Modern guidance is v6+ (and the v7 line), which centers hooks and data-router APIs.
- `withRouter` is no longer the pattern; use hooks like `useNavigate`, `useLocation`, and `useParams`.
- Existing v4 apps can continue running, but migration is recommended for long-term support and maintainability.

## Detailed Explanation

React Router v4 was a major step forward when released, but today it is considered a
legacy generation. Interview-safe wording is: v4 is not the current recommended
version for new work, and teams typically target v6+ (with v7 direction) for modern
routing patterns and better ecosystem alignment.

In v4/v5-era code, programmatic navigation often used `withRouter` and
`history.push(...)`. In v6+, navigation uses `useNavigate()` and route configuration is
expressed with `<Routes>` and `element`. The v7 direction builds on this modern API
surface rather than bringing back v4 patterns, so v6-style code is the right migration
target.

```jsx
import { BrowserRouter, Routes, Route, useNavigate } from "react-router-dom";

function GoToProfileButton() {
  const navigate = useNavigate();

  return (
    <button type="button" onClick={() => navigate("/profile")}>
      Go to profile
    </button>
  );
}

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<GoToProfileButton />} />
        <Route path="/profile" element={<h2>Profile</h2>} />
      </Routes>
    </BrowserRouter>
  );
}
```

## Practice Exercise

Take a small v4 component that uses `withRouter` and `history.push("/dashboard")`.
Rewrite it to use `useNavigate()` and define routes with `<Routes>` + `element`.
Then explain in 3-4 sentences why this is the preferred path toward current
v6+/v7-style React Router code.

## Related Topics

- React Router v5 to v6 migration checklist
- `withRouter` replacement strategies with hooks
- Declarative routing with `<Routes>` and nested routes
- Data routers and loader/action patterns in modern React Router

## Request or Context

````markdown
There are three different ways to achieve programmatic routing/navigation within components.

Using the withRouter() higher-order function:

The withRouter() higher-order function will inject the history object as a prop of the component. This object provides push() and replace() methods to avoid the usage of context.

```ts
import { withRouter } from "react-router-dom"; // this also works with 'react-router-native'
const Button = withRouter(({ history }) => (
  <button
    type="button"
    onClick={() => {
      history.push("/new-location");
    }}
  >
    {"Click Me!"}
  </button>
));
```

Using <Route> component and render props pattern:

The <Route> component passes the same props as withRouter(), so you will be able to access the history methods through the history prop.

```ts
import { Route } from "react-router-dom";
const Button = () => (
  <Route
    render={({ history }) => (
      <button
        type="button"
        onClick={() => {
          history.push("/new-location");
        }}
      >
        {"Click Me!"}
      </button>
    )}
  />
);
```

Using context:

This option is not recommended and treated as unstable API.

```ts
const Button = (props, context) => (
  <button
    type="button"
    onClick={() => {
      context.history.push("/new-location");
    }}
  >
    {"Click Me!"}
  </button>
);
Button.contextTypes = {
  history: React.PropTypes.shape({
    push: React.PropTypes.func.isRequired,
  }),
};
```
````
