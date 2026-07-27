---
type: "question"
status: draft
date: 2026-04-26 1777203876
tags: [sitecore, headless, content-resolver]
---

# 🎯 When Would You Use a Content Resolver in Sitecore? - L3

## Question

> **Core Question:** Why would you use a content resolver, especially when you need data that is not the default rendering datasource?
> **Follow-up:** How does the answer differ between classic Headless Services and XM Cloud/SitecoreAI?

## 💡 Quick Answer (30 seconds)

- In classic Headless Services, a contents resolver lets you reshape Layout Service output beyond the default datasource fields.
- That can include child items, context items, GraphQL-shaped data, or custom computed output via C#.
- In XM Cloud/SitecoreAI, custom content resolvers are not supported, so you should prefer supported built-ins or other patterns.

## 📖 Detailed Explanation

The exam answer is correct for traditional Sitecore Headless Services on XP: a custom `IRenderingContentsResolver` lets you write C# that changes what Layout Service serializes for a rendering. That is exactly how you would inject custom computed output or reshape data coming from Sitecore.

However, current SitecoreAI/XM Cloud documentation adds an important constraint: custom content resolvers are not supported there. You can still use supported built-in resolvers such as Datasource Resolver, Context Item Resolver, or Navigation Contents Resolver, but not arbitrary custom C# resolvers. So the safe interview answer is version-aware: the principle is "reshape Layout Service output", but the allowed implementation depends on product/runtime.

```csharp
public object ResolveContents(Rendering rendering, IRenderingConfiguration config)
{
    return new
    {
        message = "custom layout output"
    };
}
```

Official proof: [Customizing the Layout Service rendering output](https://doc.sitecore.com/xp/en/developers/hd/latest/sitecore-headless-development/customizing-the-layout-service-rendering-output.html#customizing-the-layout-service-rendering-output), [Content resolvers](https://doc.sitecore.com/sai/en/developers/sitecoreai/content-resolvers.html#content-resolvers).

## 🧪 Practice Exercise

Compare these two scenarios and choose the right approach for each: adding datasource children to Layout Service output on XP, and shaping XM Cloud component output on Experience Edge.

## 🔗 Related Topics

- Layout Service
- Integrated GraphQL
- XM Cloud limitations for custom resolvers

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
