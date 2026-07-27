# What is lifecycle methods in React, NextJs, Angular, Asp.Net Core?

## Metadata

- Status: executed
- Date: 2026-03-04 1772582400
- Tags: [#lifecycle-methods, #react, #nextjs, #angular, #asp-net-core]
- Areas: [education]
- AI reviews:
  - 2026-03-04 | GPT-5.3-Codex (Copilot) | Review complete. Wording refined, tags normalized, response added with assumptions and next steps.

## Mission

- Explain lifecycle methods in React
- Explain lifecycle methods in Next.js
- Explain lifecycle hooks in Angular
- Explain request/application lifecycle in ASP.NET Core

## Assumptions

- You want an interview-ready, concise explanation.
- You are comparing component lifecycles (frontend) with app/request lifecycle (backend).
- You are using modern versions: React with Hooks, Next.js App Router, Angular with lifecycle hooks, and ASP.NET Core middleware pipeline.

## Lifecycle Methods Overview

Lifecycle means "what runs during creation, updates, and teardown" of a component, page, or request.

| Technology | Unit of lifecycle | Main phases | Typical APIs |
| --- | --- | --- | --- |
| React | Component | Mount, update, unmount | `useEffect`, `useLayoutEffect`, `useState`; class methods like `componentDidMount` |
| Next.js | Route/page + server render cycle | Request/render, hydrate, revalidate, navigate | Server Components, `fetch` caching/revalidation, route handlers, client `useEffect` |
| Angular | Component/directive | Init, check, content/view init/check, destroy | `ngOnInit`, `ngOnChanges`, `ngAfterViewInit`, `ngOnDestroy` |
| ASP.NET Core | HTTP request through app | Startup, middleware chain, endpoint execution, response | `Program.cs`, middleware (`Use...`), controllers/minimal APIs, filters |

## By Framework

### React

- In class components, lifecycle is explicit (`componentDidMount`, `componentDidUpdate`, `componentWillUnmount`).
- In function components, Hooks represent lifecycle behavior.
- Most interview answers should emphasize `useEffect` cleanup for unmount and dependency arrays for update control.

### Next.js

- Next.js uses React lifecycle plus framework-level rendering lifecycle.
- In App Router, prefer Server Components for data loading; use client Hooks only where browser interactivity is needed.
- Lifecycle includes server render, client hydration, navigation re-render, and optional ISR revalidation.

### Angular

- Angular provides explicit lifecycle hooks on components/directives.
- Common flow: constructor -> `ngOnChanges` -> `ngOnInit` -> checks (`ngDoCheck`) -> view/content hooks -> `ngOnDestroy`.
- Interview focus: when each hook fires and cleanup in `ngOnDestroy`.

### ASP.NET Core

- Not a UI component lifecycle; it is an application + request pipeline lifecycle.
- Startup config wires services and middleware once; each request flows through middleware then endpoint.
- Interview focus: middleware order, DI scopes (singleton/scoped/transient), and where cross-cutting logic belongs.

## Clear Comparison

- React/Angular: lifecycle is component instance-centric.
- Next.js: combines React component lifecycle with server rendering/caching lifecycle.
- ASP.NET Core: lifecycle is request pipeline-centric, not component-centric.
- Cleanup concept exists everywhere: unmount/destroy in UI, disposal/end-of-request in backend.
