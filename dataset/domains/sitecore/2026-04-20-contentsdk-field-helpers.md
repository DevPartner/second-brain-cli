---
type: "zknotes"
date: 2026-04-20 1776688194.186
tags: [content-sdk]
---

# ContentSDK field helpers

## Source

From: [Website](https://learning.sitecore.com/partners/learn/learning-plans/119/sitecoreai-cms-for-developers/courses/1597/wed-development-with-sitecoreai-cms/lessons/3055:1218/web-development-with-sitecoreai-cms)

## Summary

Field helpers are utility functions that simplify the rendering of Sitecore field values in React/Next.js components. In SitecoreAI CMS, these helpers provide a bridge between Sitecore's content management capabilities and your Next.js frontend application.

## Key Ideas

Field helpers offer several advantages:

- **Type Safety** - When combined with TypeScript, they provide compile-time type checking
- **Automatic Rendering** - Handle complex field types with appropriate HTML output
- **Editor Support** -Enable inline editing capabilities in SitecoreAI CMS Page builder
- **Consistent Output** - Ensure standardized rendering across your application
- **Error Handling** - Gracefully handle empty or invalid field values

👇 The following table shows the correspondence between Sitecore field types and JSS Next.js components.

| Field Type                            | Next.js Utility Field Type         | Component     | Example Usage                                                 |
| ------------------------------------- | ---------------------------------- | ------------- | ------------------------------------------------------------- |
| Single-Line Text or Multi-Line Text   | TextField or `Field<string>`       | `<Text>`      | `<Text field={fields.Title} />`                               |
| Rich Text                             | RichTextField or `Field<string>`   | `<RichText>`  | `<RichText field={fields.Content} />`                         |
| Image                                 | ImageField                         | `<Image>`     | `<Image field={fields.ContentImage} className="img-fluid" />` |
| General Link                          | LinkField                          | `<Link>`      | `<Link field={fields.Link} className="btn btn-primary" />`    |
| Date / DateTime                       | `Field<string>`                    | `<DateField>` | `<DateField field={fields.EventDate} />`                      |
| Checkbox                              | `Field<boolean>`                   | Direct Access | `{fields.HideEventDate.value && <span>true</span>}`           |
| Number                                | `Field<number>`                    | `<Text>`      | `<Text field={fields.EventDuration} />`                       |
| File                                  | FileField                          | `<File>`      | `<File field={fields.File} />`                                |

### Links and Multi-Links field type example

- **Field Type**
    Droplink
    Droptree
    Grouped Droplink

- **Field type Definition example in Next.js**

    ```ts
    type OptionItem = Item & {  
    fields: {Text: TextField;
    }
    }
    ```

- **Example usage**

    ```ts
    type EventDetailsRouteData =
    RouteData & {
        fields: {
        EventLocation: OptionItem;
        HolidayTypes: OptionItem[];
        }
    }

    {fields.EventLocation && (
    <Text field={fields.EventLocation.fields.Text} />   )}
    ```

- **Field Type**
    Multilist
    Checklist
    TreelistEx
    Treelist

- **Field type Definition example in Next.js**

    ```ts
    {fields.HolidayTypes && (
    <ul>
        {fields.HolidayTypes.map((value, index) => (            <li key={index}>{value.displayName}</li>           )
        )}
    </ul>
    )}
    ```

## Related

- [[2026-04-20-what-is-sitecore-content-sdk|ContentSDK]]
