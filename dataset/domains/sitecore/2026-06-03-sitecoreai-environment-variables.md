---
type: "zknotes"
date: 2026-06-03 1780465286.175
tags: []
---

# SitecoreAI Environment Variables (JSS Next.js + Content SDK)

## Source

From: GitHub Copilot

## Summary

Version context: Sitecore JSS 22.x / SitecoreAI docs (latest at generation time)

## Key Ideas

### 1. Core SitecoreAI / JSS Next.js variables

| Variable | Required | Purpose | Typical usage |
| --- | --- | --- | --- |
| `SITECORE_EDGE_CONTEXT_ID` | Yes (connected/SaaS mode) | Context ID that resolves SitecoreAI connected configuration (Edge and related services). | SitecoreAI-connected environments, local front-end development.
| `SITECORE_SITE_NAME` | Yes | Site name used by the app (overrides app name config). | Site resolution and layout/dictionary requests.
| `DEFAULT_LANGUAGE` | Recommended | Default app language. | Fallback locale.
| `JSS_EDITING_SECRET` | Yes for editing/preview | Shared secret for editor render endpoint. Must match Sitecore-side configuration. | Pages/preview editing host scenarios.
| `GRAPH_QL_SERVICE_RETRIES` | Optional (default `3`) | Retry count for GraphQL layout/dictionary/error page requests on rate limits. | Improves resilience with Edge throttling.
| `FETCH_WITH` | Optional (default `GraphQL`) | Data-fetching mode: `GraphQL` or `REST`. | Runtime content data source mode.
| `DISABLE_SSG_FETCH` | Optional | Controls static path pre-render behavior in SSG scenarios. | Large sites or build-time optimization.

### 2. Local full-stack variables (CM/preview host scenarios)

These remain relevant when Context ID-only mode is not sufficient (for example local Docker CM/preview setups):

| Variable | Required | Purpose |
| --- | --- | --- |
| `SITECORE_API_KEY` | Yes (when direct endpoint auth is used) | API key for GraphQL endpoint authentication.
| `SITECORE_API_HOST` | Yes (same scenarios) | Base host of CM/preview API.
| `GRAPH_QL_ENDPOINT` | Yes/Derived | Endpoint URL. Common values: Edge endpoint or preview CM endpoint.

### 3. Content SDK variables (common mapping)

| Variable | Required | Purpose |
| --- | --- | --- |
| `NEXT_PUBLIC_DEFAULT_SITE_NAME` | Recommended | Default site name used by Content SDK configuration.
| `NEXT_PUBLIC_DEFAULT_LANGUAGE` | Recommended | Default locale fallback.
| `SITECORE_EDITING_SECRET` | Required for editing/preview | Editing secret in Content SDK config.

### 4. Recommended `.env.local` template (safe placeholders)

```dotenv
# SitecoreAI connected mode
SITECORE_EDGE_CONTEXT_ID=<your-context-id>
SITECORE_SITE_NAME=<your-site-name>
DEFAULT_LANGUAGE=en
JSS_EDITING_SECRET=<match-server-editing-secret>

# Optional reliability/runtime tuning
GRAPH_QL_SERVICE_RETRIES=3
FETCH_WITH=GraphQL
# DISABLE_SSG_FETCH=true

# Local full-stack / preview mode (set when applicable)
# SITECORE_API_HOST=https://<your-cm-host>.sitecorecloud.io
# SITECORE_API_KEY=<your-preview-or-edge-api-key>
# GRAPH_QL_ENDPOINT=https://edge.sitecorecloud.io/api/graphql/v1

# Content SDK (if app uses Content SDK config)
# NEXT_PUBLIC_DEFAULT_SITE_NAME=<your-site-name>
# NEXT_PUBLIC_DEFAULT_LANGUAGE=en
# SITECORE_EDITING_SECRET=<match-server-editing-secret>
```

### 5. Source-backed notes

- From JSS 21.6+, `SITECORE_EDGE_CONTEXT_ID` replaces many older configuration variables in connected scenarios.
- For local full-stack development, `SITECORE_API_HOST`, `SITECORE_API_KEY`, and `GRAPH_QL_ENDPOINT` can still be required.
- Environment variable updates in SitecoreAI environment management require rebuild/redeploy to apply.
- Keep secrets out of `.example` files and source control.

### 6. Validation checklist

- Confirm `SITECORE_SITE_NAME` matches Sitecore site definition.
- Confirm editing secret matches both app and server config.
- Verify GraphQL endpoint responds with the provided API key.
- Check variable precedence if duplicates exist (`scjssconfig.json` -> `.env` -> `.env.local`).

## Related

-
