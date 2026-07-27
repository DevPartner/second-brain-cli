---
type: "question"
status: draft
date: 2026-04-26 1777203876
tags: [question, sitecore, xm-cloud, byoc]
---

# 🎯 How Do You Bring Your Own Code into XM Cloud Component Builder? - L2

## Question

> **Core Question:** How can developers bring their own code into XM Cloud for use in the Component Builder interface?
> **Follow-up:** What must be true about the rendering host before registration works?

## 💡 Quick Answer (30 seconds)

- Reference an external code base that contains the component.
- Host that code on an accessible rendering host URL.
- Complete the registration flow so the component becomes available in Component Builder and Pages.

## 📖 Detailed Explanation

Sitecore documents BYOC as a registration flow, not as a simple folder export or drag-and-drop embed. Developers create the React component in their external code base, make that code reachable through a rendering host, and then register it so Component Builder and Pages can discover it. After registration, authors can add the component from the component library and configure its input properties.

The important distinction is that BYOC is not just "copy some files into a project folder". Sitecore expects an external component registration workflow driven by `registerComponent(...)` and an accessible rendering host.

```javascript
import { registerComponent } from '@sitecore-feaas/clientside';

registerComponent(MyComponent, {
  name: 'MyComponent',
  description: 'Reusable BYOC component'
});
```

Official proof: [Walkthrough: Registering an external React component](https://doc.sitecore.com/sai/en/developers/sitecoreai/walkthrough--registering-an-external-react-component.html#walkthrough-registering-an-external-react-component), [Bring your own components](https://doc.sitecore.com/sai/en/developers/sitecoreai/bring-your-own-components.html#bring-your-own-components).

## 🧪 Practice Exercise

Explain the minimum steps to make a locally developed React component available inside Component Builder for a demo environment.

## 🔗 Related Topics

- `registerComponent`
- Rendering host
- External component preview

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
