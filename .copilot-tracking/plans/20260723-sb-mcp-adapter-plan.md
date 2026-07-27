<!-- markdownlint-disable-file -->

# Task Checklist: sb util (Second Brain) MCP Adapter

## Overview

Replace the Foundry Local SDK demo in `rust/sb-mcp-local-webserver` with a minimal axum-based HTTP server that exposes sb util (Second Brain) search as three MCP-compatible POST endpoints.

## Objectives

- Replace `Cargo.toml` with axum + serde dependencies (remove foundry/reqwest)
- Replace `src/main.rs` with a 3-endpoint HTTP server wrapping `sb search`, `sb vsearch`, `sb query`
- Support `SB_CMD` and `PORT` env vars for configuration
- Update `README.md` with endpoint and env var documentation

## Research Summary

### Project Files

- `rust/sb-mcp-local-webserver/src/main.rs` — Foundry Local demo to be fully replaced
- `rust/sb-mcp-local-webserver/Cargo.toml` — package definition to be replaced
- `rust/sb-mcp-local-webserver/README.md` — docs to be replaced

### External References

- `.copilot-tracking/research/20260723-sb-mcp-adapter-research.md` — live-tested sb CLI commands, verified JSON output format, complete axum implementation

### Standards References

- `rust/CLAUDE.md` — task implementation workflow and changes file conventions

## Implementation Checklist

### [x] Phase 1: Project Foundation

- [x] Task 1.1: Replace Cargo.toml with sb-mcp-server package definition
  - Details: `.copilot-tracking/details/20260723-sb-mcp-adapter-details.md` (Lines 11–24)

- [x] Task 1.2: Replace src/main.rs with the axum 3-endpoint server
  - Details: `.copilot-tracking/details/20260723-sb-mcp-adapter-details.md` (Lines 26–39)

### [ ] Phase 2: Verification

- [ ] Task 2.1: End-to-end endpoint testing
  - Details: `.copilot-tracking/details/20260723-sb-mcp-adapter-details.md` (Lines 43–57)

- [ ] Task 2.2: Update README.md with endpoint and env var documentation
  - Details: `.copilot-tracking/details/20260723-sb-mcp-adapter-details.md` (Lines 59–72)

## Dependencies

- axum 0.8
- tokio 1 (features = ["full"])
- serde 1 (features = ["derive"])
- serde_json 1
- `sb` CLI available in PATH (or configured via `SB_CMD`)

## Success Criteria

- `cargo build` succeeds with new Cargo.toml
- All three POST endpoints return `{"results":[...]}` with real sb util (Second Brain) output
- `SB_CMD` env var override routes subprocess calls correctly
- Server startup prints configured `SB_CMD` for debugging
