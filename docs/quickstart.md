---
type: guide
date: 2026-08-04
tags: [quickstart, setup, installation]
---

# Quick Start

Get second-brain-cli running: set up a collection, index your notes, search, and connect an AI assistant.

---

## 1. Install prerequisites

- [Rust](https://rustup.rs/) 1.80 or later
- [Microsoft Foundry Local](https://learn.microsoft.com/azure/foundry-local/) — on-device model runtime, must be on your `PATH`

Verify both are available:

```sh
rustc --version
foundry --version
```

---

## 2. Build from source

```sh
cd src
cargo build --release
```

This produces two binaries in `src/target/release/`:

| Binary | Purpose |
| --- | --- |
| `sb` | CLI — index and search notes |
| `sb-mcp-server` | HTTP + MCP adapter for AI assistants |

Add `src/target/release/` to your `PATH` for convenience.

---

## 3. Manage collections

Collections are named groups of markdown files. Register a directory before indexing it.

```sh
# Register a directory
sb collection add ~/notes --name notes --description "Personal knowledge base"

# List all collections
sb collection ls

# Show detailed info for one collection
sb collection inspect notes

# Remove a collection and all its indexed data
sb collection rm notes
```

**`sb collection ls` output:**

```text
NAME   PATH              DOCS   CHUNKS  LAST INDEXED (UTC)
notes  /home/user/notes   142     2840   2026-07-26T14:00:00+00:00
```

---

## 4. Index your notes

```sh
# Index by directory path (collection name defaults to basename)
sb index ~/notes

# Index using a registered collection name
sb index -c notes

# Force re-embed all documents (even if already indexed)
sb index -c notes --force
```

Indexing is incremental by default — files already in the database are skipped unless `--force` is used. Downloads the `qwen3-embedding-0.6b` embedding model on first run.

> [!NOTE]
> Patterns listed under `search.exclude` (value `true`) in `.vscode/settings.json` at or above the collection path are honored during the directory walk.

---

## 5. Search your notes

```sh
# Keyword search (BM25, no model required)
sb search "rust lifetimes"

# Semantic vector search (requires Foundry Local)
sb vsearch "how does ownership work"

# Retrieve a full document by ID
sb get "#854b91"
```

Each result includes:

```json
{
  "docid": "#854b91",
  "score": 0.88,
  "file": "sb://notes/rust/2026-03-05-lifetimes.md",
  "line": 7,
  "title": "Rust lifetimes explained",
  "snippet": "..."
}
```

---

## 6. Connect an AI assistant via MCP

Start the MCP server:

```sh
sb-mcp-server
# Listening on http://0.0.0.0:3000
# MCP endpoint: http://localhost:3000/mcp
```

Add the server to your MCP client config (Claude Desktop, VS Code Copilot, etc.):

```json
{
  "mcpServers": {
    "second-brain": {
      "url": "http://localhost:3000/mcp"
    }
  }
}
```

The server exposes three MCP tools:

| Tool | Description |
| --- | --- |
| `keyword_search` | BM25 full-text search |
| `semantic_search` | Vector similarity search |
| `get_document` | Retrieve full document by ID |

---

## Next steps

- See [`architecture.md`](architecture.md) for system design, component diagrams, and database schema.
- Configure `SB_DB`, `SB_MODEL`, `SB_DICT` environment variables — see [`configuration.md`](configuration.md).
- Run `sb --help` or `sb <subcommand> --help` for the full option reference.
