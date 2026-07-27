<!-- markdownlint-disable-file -->

# Task Research Notes: sb util (Second Brain) MCP Adapter (HTTP Web Server)

## Research Executed

### File Analysis

- `rust/sb-mcp-local-webserver/src/main.rs`
  - Currently a Foundry Local SDK demo (OpenAI-compatible web server + chat). Completely unrelated to the new task — will be fully replaced.
- `rust/sb-mcp-local-webserver/Cargo.toml`
  - Package name `foundry-local-webserver`, depends on `foundry-local-sdk`, `tokio`, `serde_json`, `reqwest`. All deps will be replaced.
- `rust/sb-mcp-local-webserver/README.md`
  - Foundry Local docs — will be replaced.

### Code Search Results

- Existing rust `.copilot-tracking` research
  - `rust/.copilot-tracking/research/20260722-transcript-cli-research.md` — unrelated (transcript CLI)

### External Research

- fetch:"upstream CLI README"
  - Three search subcommands: `search` (BM25/keyword), `vsearch` (vector/semantic), `query` (hybrid with reranking)
  - Flags: `-n <num>` (result count, default 5), `-c <name>` (collection filter, repeatable), `--json` (structured JSON output)
  - sb util (Second Brain) already has a native MCP server via `sb mcp --http` — but the user wants a custom lightweight adapter

### Live Testing (PowerShell)

- sb available via `sb`
- Binary is available as an installed CLI command
- Collections confirmed: 1 collection named `notes` (176 files)

#### Confirmed: progress/info lines go to **stderr**, JSON goes to **stdout**

```powershell
sb search "what is the concept" --json -n 2   # stdout = JSON array
sb vsearch "what is the concept" --json -n 2  # stderr = progress, stdout = JSON array
sb query "what is the concept" --json -n 2 -c notes  # same pattern
```

### Project Conventions

- Standards referenced: `rust/CLAUDE.md` — task implementation workflow
- Existing Rust project uses `tokio`, `serde_json` — consistent with new deps

## Key Discoveries

### sb CLI Command Mapping

| MCP Method | sb Subcommand | Description |
|---|---|---|
| `keyword_search` | `sb search` | BM25 full-text, fastest |
| `semantic_search` | `sb vsearch` | Vector/embedding search |
| `hybrid_search` | `sb query` | Hybrid + reranking (highest quality) |

**Command structure:**
```sh
sb search  "<query>" --json -n <top_k> [-c <collection>]
sb vsearch "<query>" --json -n <top_k> [-c <collection>]
sb query   "<query>" --json -n <top_k> [-c <collection>]
```

### Actual JSON Output Format (verified)

```json
[
  {
    "docid": "#6f805a",
    "score": 0.73,
    "file": "sb://notes/domains/react/2026-03-07-stoppropagation.md",
    "line": 27,
    "title": "What does stopPropagation do in React events? - L1",
    "context": "Personal notes",
    "snippet": "@@ -26,4 @@ (25 before, 26 after)\n\r\nReact synthetic events follow standard bubbling..."
  }
]
```

Fields: `docid`, `score`, `file` (`sb://` URI), `line`, `title`, `context`, `snippet`

### sb Invocation on Windows

- The Rust server should accept `SB_CMD` env var (default: `sb`) to allow flexibility
- Pattern: split `SB_CMD` on whitespace → first token = program, rest = prefix args
  - `SB_CMD=sb` (default) → `sb search ...`
  - `SB_CMD=node C:/path/sb` → `node C:/path/sb search ...`

### Rust HTTP Stack Decision

**axum 0.8** is the best fit:
- Minimal boilerplate for a 3-endpoint server
- Async-native (tokio), composable extractors
- Actively maintained, idiomatic Rust

```toml
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

No `reqwest` needed (no outbound HTTP calls — only subprocess).

## Recommended Approach

**Single-file Rust HTTP server (`src/main.rs`)** using axum + tokio::process::Command:

1. Three `POST` endpoints: `/keyword_search`, `/semantic_search`, `/hybrid_search`
2. Each accepts `{ "query": "...", "collection": "", "top_k": 5 }` (collection and top_k optional)
3. Builds the appropriate `sb` subprocess command, captures stdout, parses as JSON
4. Returns the raw result array from sb util (Second Brain) as `{ "results": [...] }`
5. Configuration via env vars: `SB_CMD` (default `sb`), `PORT` (default `3000`)

### Complete Implementation (src/main.rs)

```rust
use axum::{
    extract::Json,
    http::StatusCode,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::process::Command;

#[derive(Deserialize)]
struct SearchRequest {
    query: String,
    #[serde(default)]
    collection: String,
    #[serde(default = "default_top_k")]
    top_k: usize,
}

fn default_top_k() -> usize { 5 }

#[derive(Serialize)]
struct SearchResponse {
    results: serde_json::Value,
}

fn sb_cmd() -> (String, Vec<String>) {
  let s = std::env::var("SB_CMD").unwrap_or_else(|_| "sb".into());
    let mut parts: Vec<String> = s.split_whitespace().map(String::from).collect();
  if parts.is_empty() { parts.push("sb".into()); }
    let prog = parts.remove(0);
    (prog, parts)
}

async fn run_search(
    subcommand: &str,
    req: SearchRequest,
) -> Result<Json<SearchResponse>, (StatusCode, String)> {
  let (prog, prefix) = sb_cmd();
    let mut cmd = Command::new(&prog);
    cmd.args(&prefix)
        .arg(subcommand)
        .arg(&req.query)
        .arg("--json")
        .arg("-n")
        .arg(req.top_k.to_string());
    if !req.collection.is_empty() {
        cmd.arg("-c").arg(&req.collection);
    }
    let output = cmd.output().await.map_err(|e| {
      (StatusCode::INTERNAL_SERVER_ERROR, format!("sb launch failed: {e}"))
    })?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        return Err((StatusCode::INTERNAL_SERVER_ERROR, err));
    }
    let results: serde_json::Value = serde_json::from_slice(&output.stdout).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("JSON parse error: {e}"))
    })?;
    Ok(Json(SearchResponse { results }))
}

async fn keyword_search(Json(req): Json<SearchRequest>) -> Result<Json<SearchResponse>, (StatusCode, String)> {
    run_search("search", req).await
}

async fn semantic_search(Json(req): Json<SearchRequest>) -> Result<Json<SearchResponse>, (StatusCode, String)> {
    run_search("vsearch", req).await
}

async fn hybrid_search(Json(req): Json<SearchRequest>) -> Result<Json<SearchResponse>, (StatusCode, String)> {
    run_search("query", req).await
}

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let app = Router::new()
        .route("/keyword_search", post(keyword_search))
        .route("/semantic_search", post(semantic_search))
        .route("/hybrid_search", post(hybrid_search));

    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("sb-mcp server listening on http://{addr}");
    println!("SB_CMD={}", std::env::var("SB_CMD").unwrap_or_else(|_| "sb".into()));
    axum::serve(listener, app).await.unwrap();
}
```

### Cargo.toml (replacement)

```toml
[package]
name = "sb-mcp-server"
version = "0.1.0"
edition = "2021"
description = "Simple HTTP adapter exposing sb util (Second Brain) search as MCP-compatible endpoints"

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

### Request/Response Contract

**Request** (POST to any endpoint):
```json
{ "query": "What is reconciliation?", "collection": "notes", "top_k": 5 }
```
- `collection`: optional, default `""` (all collections)
- `top_k`: optional, default `5`

**Response** (200 OK):
```json
{
  "results": [
    {
      "docid": "#854b91",
      "score": 0.88,
          "file": "sb://notes/domains/react/2026-03-05-reconciliation.md",
      "line": 7,
      "title": "What is reconciliation?",
      "context": "Personal notes",
      "snippet": "@@ -6,4 @@ ..."
    }
  ]
}
```

**Error** (500):
```json
"sb launch failed: No such file or directory"
```

## Implementation Guidance

- **Objectives**: Replace the Foundry Local SDK demo in `rust/sb-mcp-local-webserver` with a minimal sb util (Second Brain) HTTP adapter
- **Key Tasks**:
  1. Replace `Cargo.toml` — remove foundry/reqwest deps, add axum + serde
  2. Replace `src/main.rs` — implement the 3-endpoint server shown above
  3. Update `README.md` with usage instructions and `SB_CMD` env var docs
- **Dependencies**: axum 0.8, tokio 1 (full), serde 1 (derive), serde_json 1
- **Configuration**: `SB_CMD` env var (default `sb`), `PORT` env var (default `3000`)
- **Success Criteria**:
  - `cargo build` succeeds
  - `POST /keyword_search` with `{"query":"reconciliation"}` returns JSON results array
  - `POST /semantic_search` returns results
  - `POST /hybrid_search` with `{"collection":"notes","top_k":3}` returns filtered results
  - Server startup prints the configured `SB_CMD` for easy debugging
