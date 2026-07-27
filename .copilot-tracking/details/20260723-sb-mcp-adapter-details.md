<!-- markdownlint-disable-file -->

# Task Details: sb util (Second Brain) MCP Adapter

## Research Reference

**Source Research**: `.copilot-tracking/research/20260723-sb-mcp-adapter-research.md`

## Phase 1: Project Foundation

### Task 1.1: Replace Cargo.toml

Replace the existing Foundry Local SDK `Cargo.toml` with a minimal axum + serde configuration for the sb MCP adapter.

- **Files**:
  - `rust/sb-mcp-local-webserver/Cargo.toml` — replace entirely with sb-mcp-server package definition
- **Success**:
  - `cargo build` succeeds with no errors
  - Package name is `sb-mcp-server`
  - No foundry or reqwest dependencies remain
- **Research references**:
  - `.copilot-tracking/research/20260723-sb-mcp-adapter-research.md` (Lines 214–227) — Cargo.toml replacement content
- **Dependencies**:
  - None (first task)

### Task 1.2: Replace src/main.rs

Replace the existing Foundry Local demo `src/main.rs` with the complete axum-based HTTP server implementing three search endpoints.

- **Files**:
  - `rust/sb-mcp-local-webserver/src/main.rs` — replace entirely with the sb MCP adapter implementation
- **Success**:
  - Server starts and prints `sb-mcp server listening on http://0.0.0.0:3000`
  - Server prints configured `SB_CMD` value on startup
  - All three endpoints respond to POST requests
- **Research references**:
  - `.copilot-tracking/research/20260723-sb-mcp-adapter-research.md` (Lines 118–211) — complete `src/main.rs` implementation
- **Dependencies**:
  - Task 1.1 completion

## Phase 2: Verification

### Task 2.1: End-to-End Endpoint Testing

Verify all three endpoints work correctly by running the server and sending test POST requests.

- **Files**:
  - No file changes — verification only
- **Success**:
  - `POST /keyword_search {"query":"reconciliation"}` returns `{"results":[...]}` with sb result objects
  - `POST /semantic_search {"query":"test"}` returns results
  - `POST /hybrid_search {"query":"test","collection":"notes","top_k":3}` returns filtered results
  - Server returns HTTP 500 with error string on sb invocation failure
- **Research references**:
  - `.copilot-tracking/research/20260723-sb-mcp-adapter-research.md` (Lines 229–275) — request/response contract and success criteria
- **Dependencies**:
  - Phase 1 completion

### Task 2.2: Update README.md

Replace the Foundry Local README with usage instructions for the sb MCP adapter, documenting all endpoints and environment variables.

- **Files**:
  - `rust/sb-mcp-local-webserver/README.md` — replace with sb-mcp-server documentation
- **Success**:
  - README documents all three endpoints (`/keyword_search`, `/semantic_search`, `/hybrid_search`)
  - README documents `SB_CMD` and `PORT` env vars with defaults
  - README includes request/response JSON examples
- **Research references**:
  - `.copilot-tracking/research/20260723-sb-mcp-adapter-research.md` (Lines 229–258) — request/response contract for docs
- **Dependencies**:
  - Task 1.2 completion

## Dependencies

- axum 0.8
- tokio 1 (features = ["full"])
- serde 1 (features = ["derive"])
- serde_json 1

## Success Criteria

- `cargo build` succeeds with the new Cargo.toml
- All three POST endpoints return `{"results":[...]}` with real sb output
- Server startup output includes the configured `SB_CMD` value
- `SB_CMD` env var override routes subprocess calls correctly
