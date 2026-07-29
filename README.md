# second-brain-cli

A Rust workspace for indexing markdown notes with local vector embeddings and exposing them to AI assistants via MCP (Model Context Protocol).

Two binaries:

| Crate                    | Binary          | Role                                                       |
| ------------------------ | --------------- | ---------------------------------------------------------- |
| `sb`                     | `sb`            | CLI — manage collections, index markdown, store embeddings |
| `sb-mcp-local-webserver` | `sb-mcp-server` | HTTP + MCP adapter — proxies search requests to `sb`       |

## How it works

`sb` walks a directory of markdown files, parses YAML front matter, cleans and chunks the text, and generates vector embeddings using [Microsoft Foundry Local](https://learn.microsoft.com/azure/foundry-local/) with the `qwen3-embedding-0.6b` model. Everything is stored in a single SQLite database at `~/.sb/sb.db` using [sqlite-vec](https://alexgarcia.xyz/sqlite-vec/) for KNN vector queries.

`sb-mcp-server` wraps `sb` as an HTTP service with both a plain REST API and a native [MCP](https://modelcontextprotocol.io/) endpoint, making your notes accessible to any AI assistant that supports MCP tool calls.

## Prerequisites

- [Rust](https://rustup.rs/) 1.80+
- [Foundry Local](https://learn.microsoft.com/azure/foundry-local/) installed and on `PATH`

> [!NOTE]
> On first use, `sb index` will automatically download the `qwen3-embedding-0.6b` embedding model via Foundry Local.

## Build

```sh
cargo build --release
```

Binaries are placed in `target/release/sb` and `target/release/sb-mcp-server`.

## `sb` — CLI reference

The database defaults to `~/.sb/sb.db`. Override with `--db <path>` (global flag) or the `SB_DB` environment variable.

### Collection management

Collections are named groups of markdown files, similar to Docker containers for your notes.

```sh
# Register a directory as a collection and index it
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
NAME                 PATH                                     DOCS   CHUNKS  LAST INDEXED (UTC)
notes                /home/user/notes                          142     2840   2026-07-26T14:00:00+00:00
```

### Indexing

```sh
# Index a directory (collection name defaults to basename)
sb index ~/notes

# Index using a registered collection name
sb index -c notes

# Force re-embed all documents even if already indexed
sb index -c notes --force
sb index ~/notes --force
```

`sb index` is incremental by default — files already present in the database are skipped unless `--force` is used.

**Exclusions:** If a `.vscode/settings.json` exists in or above the collection path, patterns listed under `search.exclude` (with value `true`) are honored during the directory walk.

### Document retrieval (planned)

`sb search` (BM25 keyword) and `sb vsearch` (semantic vector search) are not yet implemented. They are the next planned feature — the MCP server is designed to proxy these commands once available.

## `sb-mcp-server` — HTTP & MCP adapter

Starts an HTTP server that calls `sb` as a subprocess to serve search requests. Supports both a plain JSON REST API and a native MCP streamable HTTP transport.

```sh
sb-mcp-server
# → Listening on http://0.0.0.0:3000
# → MCP endpoint: /mcp
```

Load environment from a `.env` file if present in the working directory.

### REST API

All endpoints accept and return JSON.

**`POST /keyword_search`**

```json
{ "query": "what is reconciliation?", "collection": "notes", "top_k": 5 }
```

- `collection` — optional, defaults to all collections
- `top_k` — optional, defaults to `5`

**`POST /semantic_search`**

Same request shape as `/keyword_search`. Uses vector similarity instead of BM25.

**`POST /get`**

```json
{ "id": "#854b91", "collection": "notes" }
```

Returns the document identified by the given ID.

**`GET /`**

Health check — returns `"sb-mcp server is running"`.

### MCP endpoint

```text
http://localhost:3000/mcp
```

Exposes three MCP tools: `keyword_search`, `semantic_search`, `get_document`. Connect any MCP-compatible client (Claude Desktop, VS Code Copilot, etc.) to this endpoint.

**Example `mcp.json` entry:**

```json
{
  "mcpServers": {
    "second-brain": {
      "url": "http://localhost:3000/mcp"
    }
  }
}
```

### Search result format

Each search result returned from `sb` contains:

```json
{
  "docid": "#854b91",
  "score": 0.88,
  "file": "sb://notes/domains/react/2026-03-05-reconciliation.md",
  "line": 7,
  "title": "What is reconciliation?",
  "context": "Personal notes",
  "snippet": "@@ -6,4 @@ ..."
}
```

## Configuration

| Variable | Default       | Description                                                                     |
| -------- | ------------- | ------------------------------------------------------------------------------- |
| `SB_CMD` | `sb`          | Command used to invoke the `sb` binary. Supports prefix args: `node C:/path/sb` |
| `PORT`   | `3000`        | HTTP port for `sb-mcp-server`                                                   |
| `SB_DB`  | `~/.sb/sb.db` | SQLite database path (also overridable with `--db` in `sb`)                     |

## Database schema

All data lives in a single SQLite file (`~/.sb/sb.db`):

| Table         | Purpose                                                                                |
| ------------- | -------------------------------------------------------------------------------------- |
| `collections` | Registered directories, creation time, last indexed time                               |
| `documents`   | Per-file metadata: title, tags, doc type, front matter fields                          |
| `chunks`      | Text chunks (max 512 chars each) produced by the splitter                              |
| `vec_chunks`  | sqlite-vec virtual table — 1024-dimensional float embeddings, rowid-linked to `chunks` |

## Text processing pipeline

Each document goes through the following pipeline before embedding:

1. **YAML front matter** parsed with `gray_matter` (fields: `title`, `tags`, `created`, `type`)
2. **Clean** — collapse whitespace, strip non-word characters, lowercase
3. **Spell-correct** — word-level correction via `symspell` (dictionary at `~/.sb/en-80k.txt`)
4. **Chunk** — split into 512-character chunks with `text-splitter`
5. **Embed** — batch embed (32 chunks per call) via Foundry Local SDK
6. **Store** — insert into `documents`, `chunks`, and `vec_chunks`
