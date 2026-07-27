---
type: "question"
status: draft
date: 2026-04-26 1777203876
tags: [question, sitecore, docker, rendering-host]
---

# 🎯 What Docker Command Shows Rendering Container Logs? - L1

## Question

> **Core Question:** What Docker command do you use to follow the logs for the rendering container?
> **Follow-up:** What is the Compose V2 equivalent?

## 💡 Quick Answer (30 seconds)

- The expected classic answer is `docker-compose logs -f rendering`.
- With Docker Compose V2, the equivalent is `docker compose logs -f rendering`.
- If you know the container name, `docker logs -f <container-name>` also works.

## 📖 Detailed Explanation

Sitecore's Docker guidance consistently uses Compose logging to inspect the rendering host. The exact service name depends on the compose file, but the exam wording with `rendering` as the service name maps to `docker-compose logs -f rendering`.

In newer Docker versions, the preferred syntax is `docker compose` without the hyphen. Interviews often accept both if you explain the version difference clearly.

```bash
docker-compose logs -f rendering

# Docker Compose V2+
docker compose logs -f rendering

docker logs -f <container-name>.
```

Official proof: [Publish, run, and test the Getting Started template projects](https://doc.sitecore.com/xp/en/developers/hd/latest/sitecore-headless-development/publish,-run,-and-test-the-getting-started-template-projects.html#publish-run-and-test-the-getting-started-template-projects), [Sitecore Docker cheat sheet](https://doc.sitecore.com/xp/en/developers/latest/developer-tools/sitecore-docker-cheat-sheet.html#use-docker-compose).

## 🧪 Practice Exercise

Write the command you would use to tail rendering logs in a project that uses Compose V2 and a service name of `rendering`.

## 🔗 Related Topics

- `docker compose logs`
- Rendering host debugging
- Container names versus service names

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
