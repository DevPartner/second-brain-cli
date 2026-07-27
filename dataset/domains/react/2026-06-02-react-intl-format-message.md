---
type: "zknotes"
date: 2026-06-02 1780396403.842
tags: [react-intl, format-message, placeholder, typescript]
---

# react-intl-format-message

## Source

From: GitHub Copilot

## Summary

Your understanding is correct: `<FormattedMessage />` returns a React element, so it cannot be used where a plain string is required (like `placeholder`, `title`, `aria-label`, or `alt`). Use `intl.formatMessage(...)` instead. In modern React Intl, `useIntl()` is preferred in function components; `injectIntl()` still works, especially for class components or legacy code.

## Key Ideas

- `<FormattedMessage />` is for rendering UI nodes, not raw strings.
- `formatMessage({ id, defaultMessage }, values?)` returns a localized string for attributes.
- Prefer `useIntl()` in function components; use `injectIntl()` only when hooks are not available.

### TypeScript Examples

```tsx
import { useIntl } from "react-intl";

export function SearchInput() {
    const intl = useIntl();

    const placeholder = intl.formatMessage({
        id: "search.placeholder",
        defaultMessage: "Search products",
    });

    return <input placeholder={placeholder} />;
}
```

```tsx
import { injectIntl, IntlShape } from "react-intl";

type Props = {
    intl: IntlShape;
};

function SearchInputLegacy({ intl }: Props) {
    return (
        <input
            placeholder={intl.formatMessage({
                id: "search.placeholder",
                defaultMessage: "Search products",
            })}
        />
    );
}

export default injectIntl(SearchInputLegacy);
```

### Example Locale Structure

```plaintext
src/
    i18n/
        locales/
            en.json
            fr.json
            de.json
```

`en.json`

```json
{
    "search.placeholder": "Search products",
    "search.button": "Search",
    "search.results": "{count, plural, one {# result} other {# results}}"
}
```

`fr.json`

```json
{
    "search.placeholder": "Rechercher des produits",
    "search.button": "Rechercher",
    "search.results": "{count, plural, one {# resultat} other {# resultats}}"
}
```

How this connects to your code:

- `id: "search.placeholder"` points to the same key in every locale file.
- `defaultMessage: "Search products"` is a safe fallback if a key is missing.
- `intl.formatMessage(...)` returns a plain string suitable for `placeholder`.

### Assumptions

- You are using React Intl v3+ where hooks are available.
- Message IDs are defined in your locale message catalogs.

## Related

-
