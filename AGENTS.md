# Agent Instructions

**second-brain-cli** is a local-first RAG system in Rust: `sb` indexes markdown notes into SQLite (vector + BM25), `sb-mcp-server` exposes them to AI assistants via MCP.

Read `README.md` before answering any question — it is the entry point and links to `docs/` for architecture, quickstart, and configuration.

Cargo workspace root is `src/Cargo.toml`. Use `anyhow` for errors; no `unwrap` in library code.
