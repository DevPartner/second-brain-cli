---
type: "zknotes"
date: 2026-06-02 1780420327.554
tags: []
---

# Experience Edge schema

## Source

From: [Website](https://doc.sitecore.com/sai/en/developers/sitecoreai/experience-edge/experience-edge-apis/the-experience-edge-schema.html)

## Summary

Sitecore Experience Edge has a read-only GraphQL schema designed to accommodate common front-end use cases for headless Sitecore development. It exposes limited information about Sitecore items. For example, there are no Standard Fields available.

## Key Ideas

## Sitecore concepts

- *Items* - in Sitecore, everything is an item: a page, a component data source, a media file, and so on. Items live in a content tree and are identified by a globally unique identifier (GUID) or a path.
- *Templates* - every item has a data template that defines the item properties and fields. In GraphQL, you can cast an item to its template type using an inline fragment to access template-specific fields.
- *Layout* - a layout defines which components appear on a page and where. The `rendered` field on a layout query returns this full structure as JSON, ready for a front-end app to render the layout.
- *Sites* - a Sitecore instance can host multiple sites. Most queries need the `site` argument.

### Query entry points

The GraphQL APIs have four top-level queries:

1. `item` - query a content item by GUID or path. Use this for content that isn't tied to a page route, such as navigation data or settings.
2. `layout` - commonly used for page rendering. When given a site, route path, and language, it returns the full layout data for a SitecoreAI page. The `rendered` field is for retrieving all the page layout data, including components, their field values, and the nested placeholder structure that your front-end app renders.
3. `search` - query items by indexed properties using `where` conditions with operators such as `CONTAINS` or `EQ`. Use this for listings, filtered results, or any time you need multiple items at once.
4. `site` - queries about the site itself rather than its content. For example, route lists (useful for static generation), redirect rules, error pages, sitemap, dictionary entries, and robots configuration.

These entry points are also listed on the [GraphQL IDE](https://doc.sitecore.com/sai/en/developers/sitecoreai/experience-edge/experience-edge-apis/set-up-the-graphql-ides.html) **DOCS** tab, where you can navigate and explore the GraphQL reference documentation.

### Template projection

The types available in the Experience Edge schema reflect the template definitions of the Sitecore instance that published to it. You can enable strongly-typed template fields by using inline fragments that select fields from specific types.

In the Sitecore back end, you can give the same name to templates and fields. However, in GraphQL, the names of types and fields must be unique. When a naming collision occurs, the item or field item with the newest creation date has `_{guid}` appended to the name. The creation date is used because the ordering must be stable, and a graph type name must never change after it has been referenced.

GraphQL types are generated for templates under the following paths:

- `<foundation>/sitecore/templates/Foundation</foundation>`
- `<feature>/sitecore/templates/Feature</feature>`
- `<project>/sitecore/templates/Project</project>`
- `<userdefined>/sitecore/templates/User Defined</userdefined>`

### Pagination

Use pagination to retrieve large result sets in small, predictable chunks.

Paginated queries and fields in the Experience Edge schema, such as the search query, use a cursor-based system to request result pages.

Note the following about paginated queries:

- Query arguments include `first` (number of results to return per request) and `after` (the beginning cursor). `first` has a default value of `10` and a maximum value of `1000`.
- Query results include `pageInfo` with `hasNext` (whether there are more results) and `endCursor` (used in the after argument to obtain the next page).
- Query results also include `total` (provides the total number of available results).

    To retrieve more than 1000 total results, make multiple requests. Use the `endCursor` value from `pageInfo` in each response as the `after` argument in the next request, and repeat until `hasNext` is `false`. There is no limit on the total number of results you can retrieve across multiple paginated requests.

> Note: The first argument has a special meaning to query the complexity calculation used by the underlying GraphQL library of the `Preview` schema. In particular, using the nested `children` field on the Item graph type can cause errors, such as *Query is too complex to execute*. The following section includes recommendations for handling query complexity.

### Query complexity

Experience Edge enforces query complexity limits to protect performance and availability.

Query complexity refers to a numerical score assigned to a GraphQL query based on how resource-intensive it is to execute. This score helps prevent overly complex queries from degrading performance or risking denial-of-service. Experience Edge queries have a complexity limit. There is currently no way to retrieve the calculated complexity score of a query. You will only know that a query has exceeded the limit when Experience Edge rejects it. If your query is too complex, try the following:

- Break the query into multiple smaller queries. For example, use one query to retrieve a dataset, and apply subsequent queries to that dataset. Large queries of multiple objects increase query complexity substantially.
- Remove unnecessary fields from the query.

### Available fields for content search

n Experience Edge, you can use the GraphQL `search` query on the special fields listed in the following table. When querying templates, you can only query user-defined fields and the following special fields. For example, you can query the user-defined `title` field of the sample item template, but not `_Sortorder`.

| Field | Description |
| --- | --- |
| `_templates` | Contains all template GUIDs, including base templates. Can be used to find all the items that use the template in the hierarchy. |
| `_path` | Contains parent items, and can be used to retrieve descendants of an item by its GUID. For example, if you use a `contains` clause and the ID of the `/home` path, the results include `/home` and its children, such as `/home/about_us`. |
| `_parent` | The ID of the item’s immediate parent. |
| `_name` | The item name. |
| `_language` | The item language. |
| `_hasLayout` | Shows whether the item has presentation details/layout data. |
| `_latestversion` | Boolean that determines whether only the latest version of the item is shown. |

> Note: The `_latestversion` field is processed differently in the Preview and Delivery schemas. If you set `_latestversion` to `true` in a query to the Delivery API, it returns the latest publishable version of the item. The value of `_latestversion` can only be set to `true` in calls to the Delivery API. If it is set to `false`, it returns an error. If you set `_latestversion` to `true` in a query to the Preview API, it returns the latest available version. If it is set to `false`, it returns all versions of the item.

## Related

- [[2026-04-20-experience-edge-architecture]]
