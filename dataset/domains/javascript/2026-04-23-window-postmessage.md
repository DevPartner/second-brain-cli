---
type: "zknotes"
date: 2026-04-23 1776929950.713
tags: []
---

# window.postMessage

## Source

From: [Website](https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage)

## Summary

The **`window.postMessage()`** method safely enables cross-origin communication between [`Window`](https://developer.mozilla.org/en-US/docs/Web/API/Window) objects; *e.g.,* between a page and a pop-up that it spawned, or between a page and an iframe embedded within it.

Normally, scripts on different pages are allowed to access each other if and only if the pages they originate from share the same [origin](https://developer.mozilla.org/en-US/docs/Web/API/Location/origin) (also known as the "[same-origin policy](https://developer.mozilla.org/en-US/docs/Web/Security/Defenses/Same-origin_policy)"). `window.postMessage()` provides a controlled mechanism to securely circumvent this restriction (if used properly).

Furthermore, an accessing script must have obtained the window object of the accessed document beforehand. This can occur through methods such as [`window.open()`](https://developer.mozilla.org/en-US/docs/Web/API/Window/open) for popups or [`iframe.contentWindow`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/contentWindow) for iframes.

Broadly, one window may obtain a reference to another (*e.g.,* via `targetWindow = window.opener`), and then dispatch a [`MessageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent) on it with `targetWindow.postMessage()`. The receiving window is then free to [handle this event](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model/Events#registering_event_handlers) as needed. The arguments passed to `window.postMessage()` (*i.e.,* the "message") are [exposed to the receiving window through the event object](https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage#the_dispatched_event).

## Key Ideas

### The dispatched event

A window can listen for dispatched messages by executing the following JavaScript

```js
window.addEventListener("message", (event) => {
  if (event.origin !== "http://example.org:8080") return;

  // …
});
```

## [Security concerns](https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage#security_concerns)

**If you do not expect to receive messages from other sites, *do not* add any event listeners for `message` events.** This is a completely foolproof way to avoid security problems.

If you do expect to receive messages from other sites, **always verify the sender's identity** using the `origin` and possibly `source` properties. Any window (including, for example, `http://evil.example.com`) can send a message to any other window within the iframe hierarchy from top to every iframe below of the current document. Having verified identity, however, you still should **always verify the syntax of the received message**. Otherwise, a security hole in the site you trusted to send only trusted messages could then open a cross-site scripting hole in your site.

**Always specify an exact target origin, not `*`, when you use `postMessage` to dispatch data to other windows.** A malicious site can change the location of the window without your knowledge, and therefore it can intercept the data sent using `postMessage`.

## Related

-
