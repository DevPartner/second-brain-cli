---
type: "question"
status: draft
date: 2026-04-26 1777203876
tags: [question, sitecore, xm-cloud, pages, local-dev]
---

# 🎯 Why Set a Local Storage Key When Connecting Pages to Local XM Cloud? - L2

## Question

> **Core Question:** What is the purpose of setting a local storage key in the browser when connecting XM Cloud Pages to a local XM Cloud instance?
> **Follow-up:** What key and value do you set?

## 💡 Quick Answer (30 seconds)

- It establishes the connection between the Sitecore Pages app and your local XM Cloud instance.
- The key tells Pages which local editing host to use instead of the default remote editing host.
- You set `Sitecore.Pages.LocalXmCloudUrl` to the URL of your local instance.

## 📖 Detailed Explanation

The Sitecore docs are direct here: setting a local storage key in the browser lets you connect your local application to the Page Builder. The documented key is `Sitecore.Pages.LocalXmCloudUrl`, and the value is the local XM Cloud URL, such as `https://xmcloudcm.localhost/`.

This is not about caching, encryption, or generic browser performance. It is specifically an editor-integration switch that lets the Pages UI target your local development instance.

```text
Local Storage
  Key:   Sitecore.Pages.LocalXmCloudUrl
  Value: https://xmcloudcm.localhost/
```

Official proof: [Connect SitecoreAI Page builder to your local XM instance](https://doc.sitecore.com/sai/en/developers/sitecoreai/connect-sitecoreai-page-builder-to-your-local-xm-instance.html#connect-sitecoreai-page-builder-to-your-local-xm-instance).

## 🧪 Practice Exercise

Walk through the browser steps required to point Pages at a local containerized XM Cloud instance and explain how you would verify the connection worked.

## 🔗 Related Topics

- Local editing host
- Pages metadata integration
- XM Cloud local development

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
