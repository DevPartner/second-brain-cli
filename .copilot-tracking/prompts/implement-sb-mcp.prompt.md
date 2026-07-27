# Specification: Add MCP Support to sb-mcp-local-webserver

## Overview

Extend the existing `sb-mcp-server` Rust application to support the Model Context Protocol (MCP) while keeping the existing REST API endpoints for debugging and direct access.

Use the Rust MCP SDK instead of hand-writing JSON-RPC routing.

Recommended implementation path:

- `rmcp` for MCP server behavior
- `rmcp` streamable HTTP server transport mounted into the existing Axum app
- typed Rust tool inputs with generated JSON Schema
- one shared search execution service used by both REST and MCP

The current codebase already contains a manual MCP shim. That is acceptable as a temporary adapter, but it should not be the target architecture for this task if the SDK can cover the same behavior with less protocol code.

## Goals

- Add MCP support to the existing Axum server.
- Keep current REST endpoints unchanged.
- Reuse existing search execution logic.
- Expose search functionality as MCP tools.
- Support MCP over HTTP transport.
- Prefer SDK-managed MCP lifecycle and tool wiring over manual JSON-RPC dispatch.
- Provide clear logging for MCP requests and tool executions.

## Current Architecture

Current flow:

```text
Client -> HTTP POST -> Axum REST API -> tokio::process::Command -> sb CLI
```

Target architecture:

```text
MCP Client -> HTTP /mcp -> rmcp Streamable HTTP transport -> rmcp tool handler -> shared Search Service -> sb CLI
                                                  \
                                                   -> existing REST endpoints -> shared Search Service -> sb CLI
```

The same search service must be used by both REST and MCP.

---

# Functional Requirements

## 1. MCP Endpoint

Expose MCP over HTTP at:

```text
POST /mcp
```

Implementation requirements:

- Mount the MCP endpoint using `rmcp`'s HTTP transport support.
- Keep existing REST routes such as `/keyword_search`, `/semantic_search`, and `/hybrid_search` unchanged.
- Do not implement a custom Axum handler that manually switches on MCP method names unless a very small adapter is required by the transport API.

## 2. MCP Lifecycle

The server must support MCP initialization and tool discovery through the SDK.

Implementation requirements:

- Server metadata and capabilities must come from the SDK server definition, not from a hand-built JSON response.
- Do not hardcode one protocol version in application logic unless a specific client requires it.
- Prefer SDK-negotiated lifecycle behavior for `initialize` and related notifications.

Notes:

- Example MCP payloads may still appear in docs or tests for clarity.
- The implementation should not contain a manual `match` over raw method strings such as `"initialize"`, `"tools/list"`, and `"tools/call"` if `rmcp` already handles them.

---

# 3. MCP Tools

Expose the following tools through `rmcp`.

Implementation requirements:

- Define tools using the SDK's Rust tool macros and typed parameter structs.
- Generate tool schemas from Rust types instead of manually building JSON schema objects.
- Tool handlers must delegate to the shared search execution logic.

## Tool: keyword_search

Description:

```text
Search documents using keyword matching.
```

Maps to:

```text
sb search
```

Input shape:

```json
{
  "query": "string",
  "collection": "string, optional",
  "top_k": "integer, default 5"
}
```

## Tool: semantic_search

Description:

```text
Search documents using semantic vector search.
```

Maps to:

```text
sb vsearch
```

Input shape:

```json
{
  "query": "string",
  "collection": "string, optional",
  "top_k": "integer, default 5"
}
```

## Tool: hybrid_search

Description:

```text
Search documents using combined keyword and semantic search.
```

Maps to:

```text
sb query
```

Input shape:

```json
{
  "query": "string",
  "collection": "string, optional",
  "top_k": "integer, default 5"
}
```

---

# 4. Shared Search Service

Both REST and MCP must call the same search execution code.

Implementation requirements:

- Keep one function or service that resolves `SB_CMD`, invokes `sb`, and parses JSON output.
- REST handlers should translate HTTP input into the shared service call.
- MCP tool handlers should translate tool input into the same shared service call.
- Search execution errors should be surfaced in the native form expected by each transport:
  - REST: HTTP error response
  - MCP: SDK-native MCP tool error/result

---

# 5. Recommended Rust Implementation

Prefer the simplest supported SDK implementation.

Required direction:

- Use `rmcp` rather than a hand-written MCP envelope parser.
- Use the SDK server/tool macros for registration and schema generation.
- Use the SDK's streamable HTTP transport support for `/mcp`.

Recommended dependency shape:

```toml
rmcp = { version = "<current>", features = ["server", "macros", "schemars", "transport-streamable-http-server"] }
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
dotenvy = "0.15"
```

Recommended server shape:

```text
- define typed tool parameter structs
- implement tool methods with rmcp macros
- provide server metadata through the SDK handler
- mount the rmcp HTTP service at /mcp inside the Axum router
- keep REST endpoints beside it
```

Not required:

- manual `McpRequest` / `McpResponse` structs
- manual JSON-RPC `match` dispatch for MCP methods
- manually constructed `tools/list` schema payloads
- hardcoded `initialize` result JSON in application code
- forcing all code to remain in `main.rs`

If the final implementation needs a small amount of transport glue, keep it minimal and let the SDK own protocol semantics.

---

# Logging Requirements

Log:

Server startup:

```text
sb-mcp-server started
HTTP port: 3000
MCP endpoint: /mcp
```

MCP lifecycle:

```text
MCP connection or initialize lifecycle event received
Client: <if available from SDK context>
Protocol: <negotiated protocol version>
```

Tool execution:

```text
MCP tool call:
  name=semantic_search
  query="Azure Functions"
  top_k=5
```

Search execution:

```text
Executing: sb vsearch "Azure Functions" --json -n 5
```

Notes:

- Prefer structured logging if the crate already uses it.
- Do not reimplement MCP internals only to log raw method names.

---

# Testing Requirements

Add tests for the MCP behavior that matters externally.

## MCP startup and discovery

Validate that an MCP client can connect and discover server metadata and tools.

Preferred test strategy:

- integration test through the mounted `/mcp` endpoint
- or SDK-level server test if that is simpler and more stable

Expected:

```text
MCP server initializes successfully
server info is exposed through the SDK
tools capability is present
```

## tools/list

Expected:

```text
keyword_search
semantic_search
hybrid_search
```

## tools/call

Example:

```text
semantic_search
query="test"
```

Expected:

```text
tool call succeeds when sb is available
returned content shape matches MCP tool response expectations
```

## REST compatibility

Expected:

```text
existing REST endpoints continue to work without behavior regressions
```

Avoid tests that only validate an internal manual method-dispatch helper if the production implementation no longer uses one.

---

# Non-Goals

The implementation does not need:

- MCP resources
- MCP prompts
- Streaming tool responses beyond what the SDK transport already does by default
- Authentication
- Multi-user sessions
- Custom protocol implementation when the SDK already supports the required behavior

---

# Definition of Done

The implementation is complete when:

- MCP-compatible clients can connect to `/mcp`.
- Client can discover available tools.
- Client can call search tools.
- Search results are returned from `sb`.
- REST API continues working.
- MCP behavior is implemented through the Rust SDK rather than a hand-written JSON-RPC layer.
- Logs clearly show MCP activity and search execution.
