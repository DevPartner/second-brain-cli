---
type: "zknotes"
date: 2026-04-20 1776712105.293
tags: [sitecore-api]
---

# Authoring and Management API

## Source

From: [Website](https://learning.sitecore.com/partners/learn/learning-plans/119/sitecoreai-cms-for-developers/courses/1593/sitecore-apis-and-webhooks/lessons/3044:1214/sitecore-apis-and-webhooks)

- [See More](https://doc.sitecore.com/xmc/en/developers/xm-cloud/sitecore-authoring-and-management-graphql-api.html)

## Summary

## What is Authoring and Management API

The Authoring and Management API is a dedicated GraphQL API with a single endpoint and schema, specifically designed for managing Sitecore content and configuration. It's not to be confused with Sitecore's Delivery GraphQL API, which focuses on retrieving content for frontend rendering.

The primary GraphQL operations supported by Authoring and Management API are:

- Queries
- Mutations

## Key Ideas

### API Overview

The API is structured around several key GraphQL types that represent different aspects of the Sitecore ecosystem:

- **Item** - Core content operations (create, read, update, delete)
- **Media** - Managing media assets
- **Template** - Working with content templates and fields
- **Search** - Finding content using various criteria
- **Site** - Managing site definitions
- **Other Management Types** - Including Archiving, Database, Indexing, Job, Language, Publishing, Security, Workflow, and Rules

## Getting access to Authoring GraphQL IDE

Before you can leverage the power of Authoring and Management API, you need to establish a secure connection to it.

- Endpoint - [https://{your-sitecore-instance}/sitecore/api/authoring/graphql/v1(opens in a new tab)](https://{your-sitecore-instance}/sitecore/api/authoring/graphql/v1)
- GraphQL IDE - [https://{your-sitecore-instance}/sitecore/api/authoring/graphql/ide(opens in a new tab)](https://{your-sitecore-instance}/sitecore/api/authoring/graphql/ide)

> **Note:** To use the IDE, you need to be a member of the `sitecore\Sitecore Client Users` role or higher.

## Related

- [[2026-04-20-sitecore-apis]]
- [Walkthrough: Enabling and authorizing requests to the Authoring and Management API](https://doc.sitecore.com/sai/en/developers/sitecoreai/content-modeling-and-presentation/sitecore-authoring-and-management-graphql-api/walkthrough--enabling-and-authorizing-requests-to-the-authoring-and-management-api.html)
- [Query examples for management operations](https://doc.sitecore.com/sai/en/developers/sitecoreai/content-modeling-and-presentation/sitecore-authoring-and-management-graphql-api/query-examples-for-management-operations.html)
- [Query examples for authoring operations](https://doc.sitecore.com/sai/en/developers/sitecoreai/content-modeling-and-presentation/sitecore-authoring-and-management-graphql-api/query-examples-for-authoring-operations.html)
