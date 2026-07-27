---
type: "question"
status: draft
date: 2026-04-26 1777203876
tags: [question, sitecore, sxa, content-testing, partial-designs]
---

# 🎯 How Does Sitecore Handle Content Testing on Partial Designs? - L3

## Question

> **Core Question:** Which statement best describes how Sitecore manages content testing on pages and partial designs?
> **Follow-up:** Why can only one partial design be tested at a time on a page?

## 💡 Quick Answer (30 seconds)

- Sitecore allows only one active content test per page or impression.
- Because of that limit, you can only test one partial design included in the page design at a time.
- Page-level tests and component tests on the page take priority over partial design tests.

## 📖 Detailed Explanation

The [[2026-04-11-sxa|SXA]] docs for testing on partial designs are explicit. They say that if you run a content test on a partial design, it tests that partial design across all pages where it is used. They also say Sitecore allows only one active content test per page or impression, so only one partial design included in a page design can be tested at a time.

There is also an operational detail worth remembering: partial design testing is disabled by default and must be enabled in configuration before use. That does not change the exam answer, but it is a strong interview detail.

```text
Page impression
  -> max 1 active content test

Priority:
  page/component test on page
  before
  partial design test
```

Official proof: [Running a content test on a partial design](https://doc.sitecore.com/xp/en/developers/sxa/latest/sitecore-experience-accelerator/running-a-content-test-on-a-partial-design.html#running-a-content-test-on-a-partial-design).

## 🧪 Practice Exercise

Describe what happens if a page already has an active page content test and you try to add a partial design test to a footer used on that page.

## 🔗 Related Topics

- A/B/N testing in [[2026-04-11-sxa|SXA]]
- Partial design reuse
- Experience Optimization limits

## 📊 Confidence Level

- [ ] 🔴 Need to study (0-3)
- [ ] 🟡 Somewhat confident (4-6)
- [ ] 🟢 Very confident (7-10)
