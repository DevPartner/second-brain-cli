---
type: "zknotes"
date: 2026-07-07 1783421723.097
tags: [blazor]
---

# What is Blazor

## Source

From: GitHub Copilot + [website](https://jsakamoto.github.io/awesome-blazor-browser/#introduction-what-is-blazor-)

## Summary

Blazor is a .NET web framework to build client web apps with C#.

Blazor lets you build interactive web UIs using C# instead of JavaScript. Blazor apps are composed of reusable web UI components implemented using C#, HTML, and CSS. Both client and server code is written in C#, allowing you to share code and libraries

Blazor Server uses a persistent SignalR connection. UI events go to the server over WebSocket, the component tree is diffed, and only the delta is sent back — no full HTTP round-trips per interaction.

## Key Ideas

```csharp
// ASP.NET Core controller action — the heart of the pipeline
[HttpGet("{id}")]
public async Task<IActionResult> GetOrder(int id)
{
    // Model binding: id comes from route
    var order = await _orderService.GetByIdAsync(id);
    if (order is null) return NotFound();          // 404
    return Ok(order);                              // 200 + JSON
}
```

## Related

- [[2026-07-06-asp-net-request-lifecycle]]
