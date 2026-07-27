# 🎯 Node-Based Servers for React SSR - L2

## Prompt Metadata

- Type: explain
- Status: executed
- Date: 2026-03-06 1772789972.292
- Tags: [#react, #ssr, #nodejs]
- Project: interview
- Areas: [education]
- AI reviews:
  - 2026-03-06 | GPT-5.3-Codex (Copilot) | Review complete. Clarified Express/Hapi/Koa roles and Nginx comparison for SSR.

## Question

> **Core Question:** Why use Express, Hapi, or Koa for React SSR?
> **Follow-up:** How are they different from Nginx, and when should each be used?

## 💡 Quick Answer (30 seconds)

- Express/Hapi/Koa are Node frameworks that can run React SSR logic directly.
- Nginx is a web server/reverse proxy, not a Node application framework.
- In production, they are often combined: Nginx in front, Node SSR app behind.

## 📖 Detailed Explanation

React SSR requires server-side JavaScript execution (`renderToString` or streaming APIs). Express, Hapi, and Koa provide request routing and middleware in Node, so they are natural SSR hosts.

Differences are mostly style and ecosystem:

- Express: minimal and very common.
- Hapi: stronger plugin/config-driven approach.
- Koa: lightweight middleware with modern async flow.

Nginx is still important for TLS termination, caching, static files, and reverse proxying.

```jsx
import express from "express";
import { renderToString } from "react-dom/server";
import App from "./App.js";

const app = express();

app.get("/", (req, res) => {
  const html = renderToString(<App />);
  res.send(
    `<!doctype html><html><body><div id="root">${html}</div></body></html>`,
  );
});

app.listen(3000);
```

## 🧪 Practice Exercise

Draw a simple deployment diagram for SSR: browser, Nginx, Node (Express), and static asset storage. Explain each layer in one sentence.

## 🔗 Related Topics

- ReactDOMServer
- Hydration
- Next.js and Remix
- [[2026-03-06-server-side-rendering-ssr]]
