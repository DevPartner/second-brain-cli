---
type: "question"
status: draft
date: 2026-04-26 1777203876
tags: [question, sitecore, sxa, redirects]
---

# 🎯 Where Do You Add an Exact Redirect Item in SXA? - L2

## Question

> **Core Question:** Where do you go to add or edit an exact redirect item after changing URL structure in SXA?
> **Follow-up:** How is this different in modern SitecoreAI/XM Cloud?

## 💡 Quick Answer (30 seconds)

- In classic [[2026-04-11-sxa|SXA]]/XP, you use the Content Editor.
- You create a redirect item on the relevant page in the content tree.
- In SitecoreAI/XM Cloud, the docs say redirect items are not supported; use redirect maps instead.

## 📖 Detailed Explanation

The official [[2026-04-11-sxa|SXA]] user docs are explicit: to add a redirect item, open the page in the Content Editor, right-click it, and insert a Redirect item. That is why the correct answer to the exam question is `The Content Editor`, not Experience Editor or a separate redirect editor.

There is an important modern nuance. SitecoreAI/XM Cloud docs state that redirect items are not supported there and that you should use redirect maps instead. So if this comes up in an interview, mention both the exam answer and the product-version caveat.

```text
Classic SXA/XP:
  Content Editor -> page item -> Insert -> Redirect

SitecoreAI/XM Cloud:
  Settings -> Redirects -> Redirect Map
```

Official proof: [Redirect a URL](https://doc.sitecore.com/xp/en/users/sxa/latest/sitecore-experience-accelerator/redirect-a-url.html#redirect-a-url), [Redirect URLs](https://doc.sitecore.com/sai/en/users/sitecoreai/redirect-urls.html#redirect-urls).

## 🧪 Practice Exercise

Explain which redirect mechanism you would choose for these two cases: a single exact page redirect in XP SXA and a batch of regex redirects in XM Cloud.

## 🔗 Related Topics

- Redirect map
- Regex redirects
- Content tree redirect management

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
