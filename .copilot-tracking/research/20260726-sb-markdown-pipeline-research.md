<!-- markdownlint-disable-file -->

# Task Research Notes: sb — Markdown Pipeline with Embeddings and SQLite-Vec

## Research Executed

### File Analysis

- `sb/Cargo.toml`
  - Package name is currently `embeddings`, version `0.1.0`; uses `foundry-local-sdk = "1.2"` and `tokio`
  - No CLI, no file I/O, no SQLite dependencies — minimal embedding demo
- `sb/src/main.rs`
  - Uses `FoundryLocalManager` + `FoundryLocalConfig` to load `qwen3-embedding-0.6b` model
  - `model.create_embedding_client()` then `generate_embedding(text)` / `generate_embeddings(&[texts])`
  - Returns `response.data[i].embedding` as `Vec<f32>`
- `sb-mcp-local-webserver/src/main.rs`
  - Calls the `sb` binary as a subprocess: `sb search <query> --json -n <k>`, `sb vsearch <query> --json -n <k>`, `sb get <id>`
  - Optional `-c <collection>` flag for all commands
  - **This defines the required CLI contract for the `sb` binary**
- `Cargo.toml` (workspace)
  - Current workspace members: `transcript-merger`, `mic-transcription`, `speaker-transcription`, `sb-mcp-local-webserver`, `meeting-client`
  - `sb` is NOT yet a workspace member — must be added

### External Research

- fetch:"https://docs.rs/gray_matter/0.2.8/gray_matter/"
  - Version 0.2.8 (May 2025); supports YAML/TOML/JSON engines via `Engine` trait
  - `matter.parse_with_struct::<T>(input)` for direct serde deserialization
  - Returns `ParsedEntityStruct { data: T, content: String, excerpt: Option<String> }`
- fetch:"https://alexgarcia.xyz/sqlite-vec/rust.html"
  - `sqlite-vec = "0.1.10-alpha.4"`, requires `rusqlite ^0.31.0`, `zerocopy = "0.7"`
  - Load via: `sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_vec_init as *const ())))`
  - Create table: `CREATE VIRTUAL TABLE vec_items USING vec0(embedding float[N])`
  - Insert: parameterized query with `embedding.as_bytes()` (zerocopy `AsBytes`)
  - KNN query: `SELECT rowid, distance FROM vec_items WHERE embedding MATCH ?1 ORDER BY distance LIMIT k`
- fetch:"https://docs.rs/rusqlite/latest/rusqlite/"
  - Latest version: **0.40.1** (June 6, 2026); use `features = ["bundled"]` to avoid system SQLite dependency
- fetch:"https://docs.rs/foundry-local-sdk/latest/foundry_local_sdk/"
  - Version 1.2.3; full embedding API already demonstrated in existing `sb/src/main.rs`
  - `model.create_embedding_client()` → `EmbeddingClient`; `generate_embeddings(&[&str])` → batch
- fetch:"https://docs.rs/text-splitter/latest/text_splitter/"
  - `TextSplitter::new(max_chars)` or `TextSplitter::new(min..max)` for range-based chunking
  - `MarkdownSplitter` available with `markdown` feature flag for semantic markdown splits
  - Optional tokenizer integration with `ChunkConfig::new(n).with_sizer(tokenizer)`
- fetch:"https://docs.rs/symspell/latest/symspell/"
  - `SymSpell<AsciiStringStrategy>` or `UnicodeStringStrategy`
  - `symspell.load_dictionary(path, term_col, count_col, separator)` — requires frequency dictionary file
  - `symspell.lookup_compound(sentence, max_edit_distance)` for sentence-level correction
- fetch:"https://docs.rs/regex/latest/regex/"
  - Version 1.13.1 (July 2026); `Regex::new(pattern)` + `.replace_all(text, replacement)`
- fetch:"https://docs.rs/clap/latest/clap/"
  - Version 4.6.4 (July 2026); derive API with `#[derive(Parser, Subcommand)]`

## Key Discoveries

### Project Structure

The `sb` directory is a standalone Cargo project (not in workspace). It will be renamed/refactored in place. All logic lives in a single `src/main.rs` — no separate modules. The `sb-mcp-local-webserver` calls `search`, `vsearch`, and `get` as subprocesses; `search` and `vsearch` are not implemented in this version (those MCP tool calls will return an error response until a future release).

### CLI Contract

**Collection management — Docker-style subcommands:**

```
sb collection ls                   — list all collections with status (name, path, docs, chunks, last indexed)
sb collection add <path> --name <name> [--description <desc>]   — register directory as collection + index
sb collection rm  <name>           — remove collection and all its data from DB
sb collection inspect <name>       — full details: path, doc count, chunk count, model, created
```

**Data commands:**

```
sb index  [<path>] [-c <collection>] [--force]   — index/re-index markdown files
```

**Parameter rules:**
- `sb index`: `<path>` optional if `-c` given (uses path stored in collections table); `-c` optional if `<path>` given (defaults to path basename); `--force` re-embeds all docs

**No separate `embed` command** — embedding is always performed inside `index`. Re-embed: `sb index --force`.

**`search` and `vsearch` not implemented in this version.** They are placeholders for a future release.

**Docker analogy:**

| Docker               | sb                          |
|----------------------|-----------------------------|
| `docker ps`          | `sb collection ls`          |
| `docker inspect`     | `sb collection inspect`     |
| `docker rm`          | `sb collection rm`          |
| `docker pull/create` | `sb collection add`         |

### Embedding Model

Uses `qwen3-embedding-0.6b` via the native Foundry Local SDK (not HTTP). The model produces embeddings of a specific dimension — likely 1024 based on Qwen3-Embedding-0.6B specs. The actual dimension must be discovered at runtime or hardcoded from the model spec. **Use 1024 as the vector dimension for `sqlite-vec` table creation.**

### SQLite Schema

```sql
-- Collections registry (docker ps equivalent)
CREATE TABLE IF NOT EXISTS collections (
    name                TEXT PRIMARY KEY,
    path                TEXT NOT NULL,
    description         TEXT,
    created_at_utc      TEXT NOT NULL,    -- ISO 8601
    last_indexed_at_utc TEXT              -- ISO 8601, NULL until first index
);

-- Document metadata (one row per .md file)
CREATE TABLE IF NOT EXISTS documents (
    id             TEXT PRIMARY KEY,       -- UUID
    collection     TEXT NOT NULL,
    path           TEXT NOT NULL,          -- relative to collection root
    title          TEXT,
    tags           TEXT,                   -- JSON array stored as TEXT
    created_at_utc TEXT,
    updated_at_utc TEXT,
    doc_type       TEXT,                   -- from front matter 'type' field
    FOREIGN KEY (collection) REFERENCES collections(name)
);

-- Text chunks
CREATE TABLE IF NOT EXISTS chunks (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id TEXT    NOT NULL,
    chunk_index INTEGER NOT NULL,
    chunk_text  TEXT    NOT NULL,
    FOREIGN KEY (document_id) REFERENCES documents(id)
);

-- Vector embeddings (sqlite-vec virtual table)
-- rowid matches chunks.id for JOIN-free cross-reference
CREATE VIRTUAL TABLE IF NOT EXISTS vec_chunks
USING vec0(embedding float[1024]);
```

**Why a separate `collections` table:**
- Enables `sb collection ls` to show status without slow COUNT queries (last_indexed cached)
- Stores per-collection path — needed for `sb index -c <name>` without repeating the path
- Foreign key enforces referential integrity: removing a collection cascades correctly

### Complete sqlite-vec Rust Example

```rust
use sqlite_vec::sqlite3_vec_init;
use rusqlite::ffi::sqlite3_auto_extension;
use rusqlite::Connection;
use zerocopy::AsBytes;

unsafe {
    sqlite3_auto_extension(Some(
        std::mem::transmute(sqlite3_vec_init as *const ())
    ));
}

let db = Connection::open("notes.db")?;

db.execute_batch("
    CREATE VIRTUAL TABLE IF NOT EXISTS vec_chunks
    USING vec0(embedding float[1024]);
")?;

// Insert
let embedding: Vec<f32> = /* ... from foundry SDK ... */;
db.execute(
    "INSERT INTO vec_chunks(rowid, embedding) VALUES (?1, ?2)",
    rusqlite::params![chunk_id, embedding.as_bytes()],
)?;

// KNN search
let mut stmt = db.prepare(
    "SELECT rowid, distance FROM vec_chunks
     WHERE embedding MATCH ?1
     ORDER BY distance LIMIT ?2"
)?;
let results: Vec<(i64, f64)> = stmt.query_map(
    rusqlite::params![query_embedding.as_bytes(), top_k],
    |row| Ok((row.get(0)?, row.get(1)?)),
)?.filter_map(|r| r.ok()).collect();
```

### gray_matter YAML Parsing

```rust
use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::Deserialize;

#[derive(Deserialize, Default)]
struct DocMeta {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    tags: Option<Vec<String>>,
    #[serde(default)]
    created: Option<String>,
    #[serde(default, rename = "type")]
    doc_type: Option<String>,   // `type` is a Rust keyword — renamed
}

let matter = Matter::<YAML>::new();
let parsed = matter.parse_with_struct::<DocMeta>(&file_contents).unwrap();
let meta   = parsed.data;       // DocMeta
let body   = parsed.content;    // String — markdown body without front matter
```

### Text Processing Pipeline

Pipeline order: `clean_text()` → `correct_spelling()` → `chunk_text()`

```rust
// 1. Clean with regex (OnceLock so patterns compile once)
use std::sync::OnceLock;
static WS: OnceLock<regex::Regex> = OnceLock::new();
static PUNCT: OnceLock<regex::Regex> = OnceLock::new();
let ws    = WS.get_or_init(|| regex::Regex::new(r"\s+").unwrap());
let punct = PUNCT.get_or_init(|| regex::Regex::new(r"[^\w\s]").unwrap());
let cleaned = punct.replace_all(&ws.replace_all(&body, " "), "").to_lowercase();

// 2. Spell-correct with symspell (initialise once per run_index call, not per token)
use symspell::{SymSpell, AsciiStringStrategy, Verbosity};
let mut sym: SymSpell<AsciiStringStrategy> = SymSpell::default();
sym.load_dictionary_line(include_str!("../dict/en-80k.txt"), 0, 1, " ");
let corrected: String = cleaned.split_whitespace()
    .map(|w| sym.lookup(w, Verbosity::Closest, 2)
        .first().map(|s| s.term.clone()).unwrap_or_else(|| w.to_owned()))
    .collect::<Vec<_>>().join(" ");

// 3. Chunk with text-splitter
use text_splitter::TextSplitter;
let splitter = TextSplitter::new(512);
let chunks: Vec<&str> = splitter.chunks(&corrected).collect();
```

**search.exclude filtering:** Before walking the directory, read `.vscode/settings.json` (if present) and extract the `search.exclude` map. Build a `globset::GlobSet` from keys with `true` values. Skip any `walkdir` entry whose path relative to the collection root matches the set.

### Cargo.toml Dependencies

```toml
[package]
name = "sb"
version = "0.1.0"
edition = "2021"
description = "Second Brain CLI — index markdown notes, search by keyword or vector"

[dependencies]
foundry-local-sdk    = "1.2"
tokio                = { version = "1", features = ["rt-multi-thread", "macros"] }
rusqlite             = { version = "0.32", features = ["bundled"] }
sqlite-vec           = "0.1.6"
zerocopy             = { version = "0.7", features = ["derive"] }
gray_matter          = "0.2"
serde                = { version = "1", features = ["derive"] }
serde_json           = "1"
regex                = "1"
text-splitter        = "0.27"
clap                 = { version = "4", features = ["derive"] }
walkdir              = "2"
uuid                 = { version = "1", features = ["v4"] }
chrono               = { version = "0.4", features = ["serde"] }
anyhow               = "1"
symspell             = "0.4"
globset              = "0.4"

[target.'cfg(windows)'.dependencies]
foundry-local-sdk    = { version = "1.2", features = ["winml"] }
```

> **Note on rusqlite version:** The sqlite-vec docs reference `rusqlite ^0.31.0`. Latest rusqlite is 0.40.1 but the sqlite-vec crate may pin its dependency. Use `rusqlite = "0.32"` to match the sqlite-vec constraint while staying current. Adjust based on actual cargo resolution.

### Source Structure

Single file: **`sb/src/main.rs`** contains all logic — no separate modules.

**Comment style** (matching existing `sb/src/main.rs` convention):

```rust
// ── Section Title ────────────────────────────────────────────────────────────
```

Use this `// ── … ──` separator style for every logical section within `main.rs` (CLI definitions, DB helpers, types, pipeline, embed, index, collection commands, get command). This matches the block comment style already established in the file.

Internal sections (private functions / inline logic within `main.rs`):

```
main.rs
 ├── CLI definitions    — clap derive structs: Cli, Commands, CollectionCommands
 ├── DB helpers         — db_open(), create_schema() using rusqlite + sqlite-vec
 ├── Types              — Collection, DocMeta structs (serde + gray_matter)
 ├── Pipeline helpers   — clean_text(), build_symspell(), correct_spelling(), chunk_text()
 ├── Embed helpers      — embed_batch() wrapping foundry-local-sdk
 ├── Index logic        — run_index(): load search.exclude → walk .md → parse → clean → spell-correct → chunk → embed → store
 └── Collection logic   — cmd_collection_add/rm/ls/inspect()
```

Embedding is **always called from `run_index()`** — no standalone embed command.

### Collection ls Output Format

`sb collection ls` human-readable table output:

```
NAME        PATH                    DOCS   CHUNKS   LAST INDEXED
notes       ~/notes                  142     2 840   2026-07-26T14:00
meetings    ~/transcripts/meetings    23       460   2026-07-25T09:30
```

### Database Location

Single database: `~/.sb/sb.db`. All collections share one SQLite file. The `collection` column in `documents` and the `collections` table row act as namespaces. The `--db` flag overrides the path for testing. The `SB_DB` environment variable is an alternative override (checked before default).

## Recommended Approach

**Single Rust binary (`sb`), all logic in one `src/main.rs`, docker-style collection management, no search/vsearch:**

1. **Rename** `Cargo.toml` package name from `embeddings` to `sb`; add to workspace
2. **CLI subcommands** (via clap derive, all in `main.rs`):
   - `sb collection ls` — list all collections with status (docker ps equivalent)
   - `sb collection add <path> --name <name>` — register directory + run index
   - `sb collection rm <name>` — remove collection + all linked data from DB
   - `sb collection inspect <name>` — full details
   - `sb index [<path>] [-c <collection>] [--force]` — (re-)index markdown files
3. **No `embed` command, no `search`, no `vsearch`, no `get`** in this version
4. **4-table schema**: `collections`, `documents`, `chunks`, `vec_chunks` (vec0) — no FTS5
5. **Single file**: everything in `src/main.rs`

## Implementation Guidance

- **Objectives**: Transform `sb` from embedding demo into a minimal index + get CLI tool with docker-style collection management; all in one file
- **Key Tasks**:
  1. Update `Cargo.toml` — rename package to `sb`, add all dependencies, add `sb` to workspace members in root `Cargo.toml`
  2. Replace `src/main.rs` — complete rewrite with:
     - Clap derive CLI: `Commands::Collection(CollectionCommands)`, `Commands::Index`, `Commands::Get`
     - `db_open(path)` — opens SQLite, registers sqlite-vec auto-extension, runs `CREATE TABLE IF NOT EXISTS` for all 4 tables
     - `DocMeta` struct (serde Deserialize) — title, tags, created, doc_type fields (all Option)
     - `clean_text(s)` — regex (OnceLock): collapse whitespace, strip non-word chars, lowercase
     - `build_symspell()` — initialise `SymSpell<AsciiStringStrategy>` with bundled en-80k.txt dictionary
     - `correct_spelling(text, sym)` — word-by-word lookup with `Verbosity::Closest`, max edit distance 2
     - `chunk_text(s, size)` — TextSplitter::new(size).chunks(s).map(str::to_owned).collect()
     - `embed_batch(client, texts)` — calls `client.generate_embeddings(&texts)`, returns `Vec<Vec<f32>>`
     - `run_index(db, path, collection, force)` — load search.exclude via globset → walkdir .md → parse front matter → clean → spell-correct → chunk → embed (batches of 32) → INSERT into documents/chunks/vec_chunks; UPDATE `collections.last_indexed_at_utc`
     - `cmd_collection_add(db, path, name)` — INSERT into collections + call run_index
     - `cmd_collection_rm(db, name)` — DELETE from all tables WHERE collection = name
     - `cmd_collection_ls(db)` — SELECT with COUNT joins, print table
     - `cmd_collection_inspect(db, name)` — SELECT full row + counts, print detail
- **Dependencies**:
  - `foundry-local-sdk 1.2` (already present, Windows winml feature), `rusqlite 0.32 bundled`, `sqlite-vec 0.1.6`, `zerocopy 0.7`, `gray_matter 0.2`, `regex 1`, `text-splitter 0.27`, `clap 4 derive`, `walkdir 2`, `uuid 1 v4`, `serde/serde_json 1`, `anyhow 1`, `tokio 1`, `chrono 0.4`, `symspell 0.4`, `globset 0.4`
- **Success Criteria**:
  - `sb collection add ~/notes --name notes` — registers collection and indexes all `.md` files with embeddings
  - `sb collection ls` — prints table of all collections with doc/chunk counts and last indexed time
  - `sb collection inspect notes` — prints full detail for the `notes` collection
  - `sb collection rm notes` — removes collection and all linked documents/chunks/vectors
  - `sb index -c notes` — re-indexes the `notes` collection from its stored path
  - `sb index ~/notes` — indexes path with `notes` as collection name (basename default)
  - `sb index ~/notes --force` — re-embeds all documents even if already indexed
  - Files matching `search.exclude` patterns in `.vscode/settings.json` are not indexed
  - Binary compiles with `cargo build --release` in the workspace
