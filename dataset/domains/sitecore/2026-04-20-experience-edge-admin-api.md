---
type: "zknotes"
date: 2026-04-20 1776712561.35
tags: [experience-edge, sitecore-api]
---

# Experience Edge Admin API

## Source

From: [Website](https://learning.sitecore.com/partners/learn/learning-plans/119/sitecoreai-cms-for-developers/courses/1593/sitecore-apis-and-webhooks/lessons/3044:1214/sitecore-apis-and-webhooks)
More - [See](https://doc.sitecore.com/xmc/en/developers/xm-cloud/admin-api.html)

## Summary

The [[2026-04-20-experience-edge-architecture|Experience Edge]] Admin API is a REST API that providing endpoints for specific administrative operations on [[2026-04-20-experience-edge-architecture|Experience Edge]] tenant.

The Admin REST API provides following key endpoints:

## Key Ideas

### What is Edge Admin API

- **ClearCacheForTenant** - clears the entire cache for a given tenant.
- **DeleteContent** - removes tenant data from the data storage.
- **GetSettings** - lists all available settings for a tenant.
- **UpdateSettings** - updates all available settings for a tenant.
- **PatchSettings** - updates a setting for a tenant using one or more Patch operations.
- **CreateWebhook** - creates a new webhook.
- **UpdateWebhook** - updates an existing webhook.
- **DeleteWebhook** - deletes a specific webhook.
- **ListWebhooks** - lists all webhooks for a tenant.
- **GetWebhookById** - gets a specific tenant webhook.

### Base URL

[https://edge.sitecorecloud.io/api/admin/v1(opens in a new tab)](https://edge.sitecorecloud.io/api/admin/v1)

### Authentication

All endpoints are required to use bearer authorization with the following header:

| Header | Authorization |
| --- | --- |
| Format | Bearer <token> |
| Purpose | A JSON Web Token (JWT) |

### Requesting JSON Web Token

Before obtaining a JWT, you must set up client credentials (client ID and secret). Choose the credential type based on the API access required:

- **Edge administration client** - For environment-specific [[2026-04-20-experience-edge-architecture|Experience Edge]] API access.
- **Organization automation client** -For organization-wide access to [[2026-04-20-experience-edge-architecture|Experience Edge]] APIs (including SitecoreAI Deploy API, and Authoring and Management APIs)

Then make a POST request to [https://auth.sitecorecloud.io/oauth/token(opens in a new tab)](https://auth.sitecorecloud.io/oauth/token), and include the following properties:

| audience | <https://api.sitecorecloud.io> |
| --- | --- |
| grant\_type | client\_credentials |
| client\_id | The client ID of your Edge administration or automation client |
| client\_secret | The client secret of your Edge administration or automation client. |

## Related

- [[2026-04-20-sitecore-apis]]
- [[2026-04-20-experience-edge-webhooks]]
