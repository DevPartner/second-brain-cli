## What are the standard folders in an ASP.NET application and what is their purpose?

%

Common folders include `App_Code` (shared server code), `App_Data` (data files/logs), `App_Themes` (themes/skins), `Bin` (assemblies), `Content`/`Scripts` (static assets), and in MVC `Controllers`, `Models`, `Views`.

[#aspnet]() [#folders]() [#interview]()

## What is the purpose of the `App_Code` folder in ASP.NET Web Forms?

%

`App_Code` stores shared classes automatically compiled by ASP.NET, so pages can use those classes without separate build steps.

[#aspnet]() [#webforms]() [#folders]()

## Why is `App_Data` useful in ASP.NET applications?

%

`App_Data` is intended for local data storage such as DB files, XML, logs, and uploads; it is server-side and typically not directly web-accessible.

[#aspnet]() [#webforms]() [#state-management]()

## When do you use Cookies vs Session vs Cache vs Persistent storage?

%

Use `Cookies` for small client preferences, `Session` for per-user temporary server data, `Cache` for frequently reused shared server data, and persistent storage (DB/files) for long-term durable data.

[#aspnet]() [#state-management]() [#interview]()

## What are key security and performance trade-offs of Cookies, Session, and Cache?

%

Cookies add request overhead and can be tampered with (mitigate via secure flags/encryption). Session is safer server-side but consumes server memory. Cache is fastest for shared read-heavy data but must handle expiration/invalidation.

[#aspnet]() [#security]() [#performance]()

## What is the purpose of `Global.asax` in ASP.NET?

%

`Global.asax` handles application- and session-level events for global initialization, lifecycle handling, and centralized error handling.

[#aspnet]() [#global-asax]() [#lifecycle]()

## Which key events are commonly handled in `Global.asax`?

%

`Application_Start`, `Application_End`, `Session_Start`, `Session_End`, and `Application_Error` are the core events used for startup, cleanup, session tracking, and global exception handling.

[#aspnet]() [#global-asax]() [#events]()

## What are HTML Helpers in MVC?

%

HTML Helpers are server-side methods in Razor views that generate HTML markup, often with strong typing and model binding support.

[#aspnet-mvc]() [#html-helpers]() [#views]()

## Why prefer strongly typed helpers like `TextBoxFor` over untyped helpers?

%

Strongly typed helpers provide compile-time safety, better IntelliSense, and cleaner model-binding/validation integration.

[#aspnet-mvc]() [#html-helpers]() [#best-practices]()

## What are MVC Areas and why use them?

%

Areas partition large MVC apps into feature modules, each with its own `Controllers/Views/Models`, improving organization, scalability, and team parallelization.

[#aspnet-mvc]() [#areas]() [#architecture]()

## When are MVC Areas a good design choice?

%

Use Areas for large apps with distinct functional domains (e.g., Admin, Shop, Blog) or multiple teams owning separate product surfaces.

[#aspnet-mvc]() [#areas]() [#system-design]()

## Describe the ASP.NET MVC request lifecycle in order.

%

Typical flow: app start/config → routing → controller creation → model binding/validation → action execution → action result → view rendering (or other formatter) → HTTP response.

[#aspnet-mvc]() [#lifecycle]() [#interview]()

## Where are authentication, filters, and exception handling applied in MVC lifecycle?

%

They are cross-cutting hooks around routing/action/result stages; filters and middleware can intercept before/after actions and on exceptions.

[#aspnet-mvc]() [#lifecycle]() [#security]()

## What is MVC and its core benefit?

%

MVC separates concerns: Model (data/business rules), View (UI), Controller (request orchestration). Core benefit is cleaner architecture with better maintainability/testability.

[#aspnet-mvc]() [#architecture]() [#fundamentals]()

## What practical benefits do teams get from MVC architecture?

%

Parallel development, easier testing, improved reuse, clearer ownership boundaries, and safer feature evolution over time.

[#aspnet-mvc]() [#architecture]() [#teamwork]()

## What are Partial Views in ASP.NET MVC?

%

Partial Views are reusable UI fragments that render part of a page, reducing duplication and improving modularity of large views.

[#aspnet-mvc]() [#partial-views]() [#views]()

## When should you use Partial Views?

%

Use them for repeated page sections (cards, lists, headers, form sections) when the same markup appears across multiple views.

[#aspnet-mvc]() [#partial-views]() [#reusability]()

## What is the difference between `Server.Transfer` and `Response.Redirect`?

%

`Response.Redirect` sends an HTTP redirect to the browser (new request, URL changes). `Server.Transfer` switches execution server-side (same request, URL usually unchanged).

[#aspnet]() [#navigation]() [#webforms]()

## When should you choose `Response.Redirect` vs `Server.Transfer`?

%

Choose `Response.Redirect` for external URLs/bookmarkable URL changes and PRG patterns. Choose `Server.Transfer` for internal Web Forms transfers when preserving server-side context and avoiding client round-trip.

[#aspnet]() [#navigation]() [#performance]()

## What is ViewState in ASP.NET Web Forms?

%

ViewState is hidden page state (`__VIEWSTATE`) that preserves server control values across postbacks.

[#aspnet]() [#webforms]() [#viewstate]()

## What are the pros and cons of ViewState?

%

Pros: easy stateful programming across postbacks. Cons: larger payload/page size, potential security concerns if not protected, and performance cost on large pages.

[#aspnet]() [#viewstate]() [#performance]() [#security]()
