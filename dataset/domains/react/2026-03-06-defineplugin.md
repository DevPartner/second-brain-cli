# 🎯 Is `DefinePlugin` Still Relevant in Modern React - L2

## Prompt Metadata

- Type: explain
- Status: executed
- Date: 2026-03-06 1772822305.023
- Tags: [#react, #webpack, #build]
- Project: interview
- Areas: [education]
- AI reviews:
  - 2026-03-06 | GPT-5.3-Codex (Copilot) | Review complete. Updated guidance for modern bundlers and production mode behavior.

## Question

> **Core Question:** Is Webpack `DefinePlugin` still used to enable production mode in React apps?
> **Follow-up:** How does this compare with modern tooling like Vite or Next.js?

## 💡 Quick Answer (30 seconds)

- Yes, `DefinePlugin` is still valid in Webpack projects.
- In modern React stacks, production mode is often configured by the framework/build command.
- `DefinePlugin` is mainly useful for compile-time constants and feature flags.

## 📖 Detailed Explanation

`DefinePlugin` replaces values at build time. In older Webpack-centric setups, this helped optimize bundles by enabling production-only dead-code elimination. Today, many tools (Vite, Next.js) handle production defaults automatically, so manual `DefinePlugin` setup is less common unless custom constants are required.

Key interview framing: it is still correct, but context-dependent.

```javascript
// webpack.config.js
const webpack = require("webpack");

module.exports = {
  mode: "production",
  plugins: [
    new webpack.DefinePlugin({
      "process.env.NODE_ENV": JSON.stringify("production"),
      __FEATURE_X__: JSON.stringify(false),
    }),
  ],
};
```

## 🧪 Practice Exercise

Create two build-time flags (`__FEATURE_CHAT__` and `__FEATURE_BETA__`) and explain how they can remove code paths in production bundles.

## 🔗 Related Topics

- Tree shaking and dead-code elimination
- Vite `import.meta.env`
- Next.js environment variable conventions
