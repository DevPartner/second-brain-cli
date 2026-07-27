---
type: "zknotes"
date: 2026-06-01 1780328033.059
tags: [react-intl, i18n, formatjs, react]
---

# React Intl

## Source

From: GitHub Copilot

## Summary

React Intl is the React integration layer of FormatJS used for internationalization (i18n). It helps you make an app locale-aware by formatting translated messages, dates, numbers, times, currencies, and plurals using standards-based APIs.

In practice, you usually wrap your app with `IntlProvider` and then consume i18n features via hooks/components such as `useIntl`, `FormattedMessage`, `FormattedDate`, and `FormattedNumber`.

## Key Ideas

- React Intl is part of FormatJS and focuses on React bindings for i18n.
- `react-intl` is built on top of the JavaScript `Intl` API and provides React-friendly APIs like `IntlProvider`, `FormattedMessage`, and `useIntl`.
- `locale` means language + region preferences (for example, `en-US`, `fr-FR`, `de-DE`) and affects how dates/numbers/messages are formatted.
- Use component APIs (`<FormattedMessage />`) for rendering UI text, and use imperative APIs (`intl.formatMessage`, `intl.formatDate`) when you need plain strings (for placeholders, titles, alt text, etc.).

## Related

-
