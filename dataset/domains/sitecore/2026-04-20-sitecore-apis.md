---
type: "zknotes"
date: 2026-04-20 1776710533.915
tags: [sitecore-api]
---

# SitecoreAI APIs

## Source

- [Website](https://learning.sitecore.com/partners/learn/learning-plans/119/sitecoreai-cms-for-developers/courses/1593/sitecore-apis-and-webhooks/lessons/3044:1214/sitecore-apis-and-webhooks)

- [MarketplaceApps]: [Website](https://developers.sitecore.com/learn/getting-started/marketplace#5-dashboard-widgets--xmc-dashboard)

## Summary

SitecoreAI offers the following API endpoints for managing your implementations and accessing content.

## Key Ideas

- **Preview API** - Used primarily within the Content Management environment for previewing content *before* it is published to the delivery environment (Edge). Accesses content directly from the CM instance's publishing target.
- **Delivery API** - Used by front-end applications to retrieve *published* content on the Edge platform for live websites and applications.
- [XM Apps REST API](https://api-docs.sitecore.com/xmc/sites-api) Use the Pages API for managing your site pages in XM Cloud. [1][MarketplaceApps]
- [XM Cloud Pages REST API.](https://api-docs.sitecore.com/xmc/pages-api)XM Cloud Pages REST API. [1][MarketplaceApps]
- **[[2026-04-20-authoring-and-management-api|Authoring and Management API]]** - Provides a single endpoint and schema that allows you to manage your Sitecore content, offering read/write flexibility using GraphQL.
- [Experience Edge Token API](https://doc.sitecore.com/xmc/en/developers/xm-cloud/token-api.html) Manage API keys for the Delivery API. Use this to securely access published content via Experience Edge.
- [[2026-04-20-experience-edge-admin-api|Experience Edge Admin API]] - A REST API that enables you to administer Edge configuration and webhooks.

### Experience Edge GraphQL endpoints

Experience Edge provides two primary graphQL endpoints for read-only access to your content:

### Preview API

The Preview API is a GraphQL endpoint that provides access to your content in its current state, including drafts and content items not yet approved. Use this API to test content in staging environments before publishing to production.

- Endpoint - [https://<your host>/sitecore/api/graph/edge(opens in a new tab)](https://xmc-sitecoresaa7422-scxmcdemo6f9d-dev89cc.sitecorecloud.io/sitecore/api/graph/edge)
- GraphQL IDE - [https://<your host>/sitecore/api/graph/edge/ide(opens in a new tab)](https://xmc-sitecoresaa7422-scxmcdemo6f9d-dev89cc.sitecorecloud.io/sitecore/api/graph/edge/ide/)

### Delivery (Live) API

The Delivery API is a high-performance and highly available GraphQL API designed to access your approved and published content. Utilize this API for all production-related operations.

- Endpoint - [https://edge.sitecorecloud.io/api/graphql/v1(opens in a new tab)](https://edge.sitecorecloud.io/api/graphql/v1)
- GraphQL IDE - [https://edge.sitecorecloud.io/api/graphql/ide(opens in a new tab)](https://edge.sitecorecloud.io/api/graphql/ide)

### Accessing and setting up GraphQL IDE

💡 Before you setup the GraphQL IDE (aka playground), you need to:

1. Choose your Preview API or Delivery (live) API GraphQL IDE.
2. Obtain the API token or key for the `sc_apikey` header.

### Sample Query

```graphql
query { 
  layout(language: "en", routePath: "/", site: "skate-park") { 
    item { 
      rendered 
    } 
  } 
  site { 
    allSiteInfo {
        results { 
          name 
          routes(first: 10, language: "en") { 
            results { 
              routePath 
              route { 
                id 
              } 
            } 
          } 
        } 
      } 
    }
 } 
```

## Sitecore schema

According to the Preview GraphQL IDE DOCS, these query entry points are provided:

- **item** - Permits querying a content item via its path or unique identifier.
- **layout** - Facilitates querying an item based on its associated site and route path, typically to retrieve its Layout Service JSON.
- **search** - Enables the construction of Boolean search queries to locate items by field values or common attributes.
- **site** - Allows for the retrieval of information pertaining to content sites.
  
## Related

- [[2026-04-20-sitecore-client-api]]
- [[2026-04-20-experience-edge-architecture|Experience Edge]]
- [[2026-04-20-authoring-and-management-api|Authoring and Management API]]
- [[2026-04-22-sitecore-marketplace-custom-apps]]
