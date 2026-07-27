---
type: "question"
status: draft
date: 2026-06-02 1780421651
tags: [question, sitecore, xm-cloud, cli, dev-workflow]
---

# 🎯 Create an XM Cloud Project with Sitecore CLI - L3

## Question

> **Core Question:** What are the main steps to create and initialize an XM Cloud (SitecoreAI) project with Sitecore CLI, and how do `ser` commands fit into team workflows?
> **Follow-up:** What CI checks should you add to prevent serialization drift and failed deployments?

## 💡 Quick Answer (30 seconds)

- Install Sitecore CLI and required plugins, then authenticate.
- Create the project with `dotnet sitecore cloud project create --name <project-name>` and connect environments.
- Use `dotnet sitecore ser pull|push|validate` to keep items synchronized and CI-safe.

## 📖 Detailed Explanation

In XM Cloud teams, Sitecore CLI is the automation layer for both project bootstrap and content-model synchronization. You install CLI plugins, authenticate, and create the project/environment from the command line. Then, day-to-day schema and item changes are serialized into files and committed, so environments can be reproduced through Git and pipelines.

Serialization commands are critical for team consistency: `ser pull` brings remote item changes locally, `ser push` applies committed changes to an environment, and `ser validate` checks file correctness. In CI, this helps catch serialization drift early and keeps deployments deterministic.

```powershell
# Typical XM Cloud + serialization flow
dotnet sitecore cloud project create --name MyXMCloudProject
dotnet sitecore ser pull -n development
dotnet sitecore ser validate
dotnet sitecore ser push -n development
```

## 🧪 Practice Exercise

Create a CI job that runs `dotnet sitecore ser validate` and fails the build when serialization output is invalid. Then document when developers should run `ser pull` vs `ser push`.

## 🔗 Related Topics

- [[2026-04-15-sitecore-content-serialization-module-json|Content serialization module config]]
- Sitecore CLI plugins and command reference: <https://doc.sitecore.com/sai/en/developers/sitecoreai/sitecore-cli-command-reference.html>
- Create project with CLI: <https://doc.sitecore.com/sai/en/developers/sitecoreai/create-a-sitecoreai-project-using-the-sitecore-cli.html>
- Serialization command details: <https://doc.sitecore.com/sai/en/developers/sitecoreai/the-cli-serialization-command.html>

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
