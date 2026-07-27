---
agent: agent
description: "Explain ga-in-react-nextjs"
tools: ["read", "edit", "search", "web", "cognitionai/deepwiki/*", "agent"]
---

# How do you integrate Google Analytics in React and Next.js correctly? - L3

## Prompt Metadata

- Type: explain
- Status: executed
- Date: 2026-03-09 1773079985
- Tags: [#react, #nextjs, #analytics]
- Project: interview
- Areas: [education]
- AI reviews:
  - 2026-03-09 | GPT-5.3-Codex (Copilot) | Review complete. Converted response to interview question-card format.

## Response

- Difficulty: L3

## Question

> **Core Question:** How do you integrate Google Analytics in React and Next.js correctly?
> **Follow-up:** What is the difference between client-side page tracking and SSR rendering?

## 💡 Quick Answer (30-60 seconds)

- Initialize analytics once and send pageviews on every route change.
- For React Router, attach tracking to navigation changes (history listener or `useLocation`).
- For Next.js, track initial load plus client-side transitions.
- Respect privacy: require consent and avoid sensitive payloads.

## 📖 Detailed Explanation

Analytics setup depends on routing behavior. In React Router SPA navigation, URL changes do not reload the page, so you must manually send a pageview when route changes. Your `history.listen(...)` approach is correct for React Router v5. In React Router v6, `useLocation` is the standard way.

In Next.js, SSR only renders HTML. Google Analytics runs in the browser, so pageview tracking still happens client-side. You typically initialize `gtag` once, then send a pageview on client route transitions.

```javascript
// React Router v5 pattern (matches your example)
history.listen(function (location) {
  window.ga("set", "page", location.pathname + location.search);
  window.ga("send", "pageview", location.pathname + location.search);
});
```

```jsx
// React Router v6 pattern
import { useEffect } from "react";
import { useLocation } from "react-router-dom";

function GaPageTracker() {
  const location = useLocation();

  useEffect(() => {
    const page = location.pathname + location.search;
    window.gtag?.("config", process.env.REACT_APP_GA_ID, { page_path: page });
  }, [location]);

  return null;
}
```

```jsx
import Script from "next/script";
import { useEffect } from "react";
import { usePathname, useSearchParams } from "next/navigation";

export function AnalyticsInit() {
  const gaId = process.env.NEXT_PUBLIC_GA_ID;

  return (
    <>
      <Script
        src={`https://www.googletagmanager.com/gtag/js?id=${gaId}`}
        strategy="afterInteractive"
      />
      <Script id="ga-init" strategy="afterInteractive">
        {`window.dataLayer = window.dataLayer || []; function gtag(){dataLayer.push(arguments);} gtag('js', new Date()); gtag('config', '${gaId}');`}
      </Script>
    </>
  );
}

export function AnalyticsPageViews() {
  const pathname = usePathname();
  const searchParams = useSearchParams();

  useEffect(() => {
    const query = searchParams.toString();
    const pagePath = query ? `${pathname}?${query}` : pathname;
    window.gtag?.("config", process.env.NEXT_PUBLIC_GA_ID, {
      page_path: pagePath,
    });
  }, [pathname, searchParams]);

  return null;
}
```

## 🧪 Practice Exercise

Implement route-change pageview tracking and ensure events fire only after consent is granted.

## 🔗 Related Topics

- #gtag
- #nextjs-routing
- #privacy-compliance
