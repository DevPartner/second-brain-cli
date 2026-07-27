<!-- markdownlint-disable-file -->

# Task Details: sb — Markdown Pipeline with Embeddings and SQLite-Vec

## Research Reference

**Source Research**: `.copilot-tracking/research/20260726-sb-markdown-pipeline-research.md`

---

## Phase 1: Project Configuration

### Task 1.1: Update `sb/Cargo.toml`

Replace the existing `sb/Cargo.toml` completely. The current file only has `foundry-local-sdk` and `tokio`. All new dependencies must be added and the package renamed from `embeddings` to `sb`.

- **Files**:
  - `sb/Cargo.toml` — complete replacement
- **New content**:

```toml
[package]
name = "sb"
version = "0.1.0"
edition = "2021"
description = "Second Brain CLI — index markdown notes and retrieve by vector"

[dependencies]
foundry-local-sdk = "1.2"
tokio             = { version = "1", features = ["rt-multi-thread", "macros"] }
rusqlite          = { version = "0.32", features = ["bundled"] }
sqlite-vec        = "0.1.6"
zerocopy          = { version = "0.7", features = ["derive"] }
gray_matter       = "0.2"
serde             = { version = "1", features = ["derive"] }
serde_json        = "1"
regex             = "1"
text-splitter     = "0.27"
clap              = { version = "4", features = ["derive"] }
walkdir           = "2"
uuid              = { version = "1", features = ["v4"] }
chrono            = { version = "0.4", features = ["serde"] }
anyhow            = "1"
symspell          = "0.4"
globset           = "0.4"

[target.'cfg(windows)'.dependencies]
foundry-local-sdk = { version = "1.2", features = ["winml"] }
```

- **Success**:
  - `cargo check -p sb` produces no unresolved dependency errors
- **Research references**:
  - `.copilot-tracking/research/20260726-sb-markdown-pipeline-research.md` (Lines 225–255) — Cargo.toml dependencies section
- **Dependencies**:
  - None (first task)

---

### Task 1.2: Add `sb` to workspace `Cargo.toml`

Edit the root `Cargo.toml` to add `"sb"` to the `members` array.

- **Files**:
  - `Cargo.toml` (workspace root) — add `"sb"` to members
- **Change**: Add `"sb"` to the members list:

```toml
[workspace]
members = [
    "sb-mcp-local-webserver",
    "sb"
]
resolver = "2"
```

- **Success**:
  - `cargo metadata --no-deps` lists `sb` as a workspace member
- **Research references**:
  - `.copilot-tracking/research/20260726-sb-markdown-pipeline-research.md` (Lines 20–22) — workspace membership note
- **Dependencies**:
  - Task 1.1 completion

---

## Phase 2: Rewrite `src/main.rs`

All code goes into `sb/src/main.rs` as a complete replacement of the existing file. Use `// ── Section Title ────────────────────────────────────────────────────────────` separator comments between logical sections. This matches the style in the existing file.

---

### Task 2.1: CLI structure

Add the clap derive CLI definitions at the top of `main.rs` (after imports). Define `Cli`, `Commands`, and `CollectionCommands`.

- **Files**:
  - `sb/src/main.rs` — add at top (after imports section)
- **Required code shape**:

```rust
// ── CLI ───────────────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(name = "sb", about = "Second Brain — index and retrieve markdown notes")]
struct Cli {
    /// Path to the SQLite database (default: ~/.sb/sb.db, override with SB_DB)
    #[arg(long, global = true)]
    db: Option<PathBuf>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage collections
    Collection {
        #[command(subcommand)]
        action: CollectionCommands,
    },
    /// Index markdown files into a collection
    Index {
        /// Directory path to index
        path: Option<PathBuf>,
        /// Collection name (defaults to path basename)
        #[arg(short, long)]
        collection: Option<String>,
        /// Re-embed all documents even if already indexed
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
enum CollectionCommands {
    /// List all collections
    Ls,
    /// Register a directory as a collection and index it
    Add {
        /// Directory path
        path: PathBuf,
        /// Collection name
        #[arg(long)]
        name: String,
        /// Optional description
        #[arg(long)]
        description: Option<String>,
    },
    /// Remove a collection and all its data
    Rm {
        /// Collection name
        name: String,
    },
    /// Show detailed info about a collection
    Inspect {
        /// Collection name
        name: String,
    },
}
```

- **Success**:
  - `cargo check -p sb` compiles the CLI definitions without error
- **Research references**:
  - `.copilot-tracking/research/20260726-sb-markdown-pipeline-research.md` (Lines 62–84) — CLI contract
- **Dependencies**:
  - Task 1.1, 1.2 completion

---

### Task 2.2: DB helpers

Implement `db_open()` and `create_schema()`. `db_open` must register the sqlite-vec auto-extension before opening the connection.

- **Files**:
  - `sb/src/main.rs` — add DB helpers section
- **Required code shape**:

```rust
// ── DB ────────────────────────────────────────────────────────────────────────

fn db_path_default() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".sb").join("sb.db")
}

fn db_open(path: &Path) -> anyhow::Result<Connection> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    unsafe {
        sqlite3_auto_extension(Some(
            std::mem::transmute(sqlite3_vec_init as *const ()),
        ));
    }
    let conn = Connection::open(path)?;
    create_schema(&conn)?;
    Ok(conn)
}

fn create_schema(conn: &Connection) -> anyhow::Result<()> {
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS collections (
            name                TEXT PRIMARY KEY,
            path                TEXT NOT NULL,
            description         TEXT,
            created_at_utc      TEXT NOT NULL,
            last_indexed_at_utc TEXT
        );
        CREATE TABLE IF NOT EXISTS documents (
            id             TEXT PRIMARY KEY,
            collection     TEXT NOT NULL,
            path           TEXT NOT NULL,
            title          TEXT,
            tags           TEXT,
            created_at_utc TEXT,
            updated_at_utc TEXT,
            doc_type       TEXT,
            FOREIGN KEY (collection) REFERENCES collections(name)
        );
        CREATE TABLE IF NOT EXISTS chunks (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            document_id TEXT    NOT NULL,
            chunk_index INTEGER NOT NULL,
            chunk_text  TEXT    NOT NULL,
            FOREIGN KEY (document_id) REFERENCES documents(id)
        );
        CREATE VIRTUAL TABLE IF NOT EXISTS vec_chunks
        USING vec0(embedding float[1024]);
    ")?;
    Ok(())
}
```

- **Success**:
  - `db_open` creates the file and all 4 tables without error
  - `sqlite-vec` extension loads without panic
- **Research references**:
  - `.copilot-tracking/research/20260726-sb-markdown-pipeline-research.md` (Lines 101–181) — schema and sqlite-vec Rust example
- **Dependencies**:
  - Task 2.1 completion

---

### Task 2.3: Types

Define `DocMeta` for gray_matter YAML front matter deserialization. Note: `type` is a Rust keyword — use `doc_type` with `#[serde(rename = "type")]`.

- **Files**:
  - `sb/src/main.rs` — add Types section
- **Required code shape**:

```rust
// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Debug, serde::Deserialize, Default)]
struct DocMeta {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    tags: Option<Vec<String>>,
    #[serde(default)]
    created: Option<String>,
    #[serde(default, rename = "type")]
    doc_type: Option<String>,
}
```

- **Success**:
  - `gray_matter` can deserialize a YAML front matter block into `DocMeta` without error
- **Research references**:
  - `.copilot-tracking/research/20260726-sb-markdown-pipeline-research.md` (Lines 183–206) — gray_matter YAML parsing example
- **Dependencies**:
  - Task 2.1 completion

---

### Task 2.4: Pipeline helpers

Implement `clean_text()`, `correct_spelling()`, and `chunk_text()`. Regex instances should be created once with `OnceLock` or passed in to avoid recompilation on every call. Use `symspell` for spell correction — it is significantly faster than Levenshtein-based approaches.

- **Files**:
  - `sb/src/main.rs` — add Pipeline section
- **Required code shape**:

```rust
// ── Pipeline ──────────────────────────────────────────────────────────────────

fn build_symspell() -> symspell::SymSpell<symspell::AsciiStringStrategy> {
    let mut sym = symspell::SymSpell::default();
    // Load bundled frequency dictionary (en-80k.txt shipped with the crate)
    sym.load_dictionary_line(include_str!("../dict/en-80k.txt"), 0, 1, " ");
    sym
}

fn correct_spelling(
    text: &str,
    sym: &symspell::SymSpell<symspell::AsciiStringStrategy>,
) -> String {
    text.split_whitespace()
        .map(|w| {
            sym.lookup(w, symspell::Verbosity::Closest, 2)
                .first()
                .map(|s| s.term.clone())
                .unwrap_or_else(|| w.to_owned())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn clean_text(text: &str) -> String {
    use std::sync::OnceLock;
    static WS: OnceLock<regex::Regex> = OnceLock::new();
    static PUNCT: OnceLock<regex::Regex> = OnceLock::new();
    let ws    = WS.get_or_init(|| regex::Regex::new(r"\s+").unwrap());
    let punct = PUNCT.get_or_init(|| regex::Regex::new(r"[^\w\s]").unwrap());
    punct.replace_all(&ws.replace_all(text, " "), "").to_lowercase()
}

fn chunk_text(text: &str, max_chars: usize) -> Vec<String> {
    use text_splitter::TextSplitter;
    TextSplitter::new(max_chars)
        .chunks(text)
        .map(str::to_owned)
        .collect()
}
```

- **Pipeline order**: `clean_text()` → `correct_spelling()` → `chunk_text()`
- **SymSpell initialization**: build once per `run_index` call, not per token
- **Success**:
  - `clean_text("Hello, World!  \n\t test")` returns `"hello world  test"` (lowercased, punctuation removed)
  - `correct_spelling("speling mistakee", &sym)` returns corrected words
  - `chunk_text("a".repeat(2000).as_str(), 512)` returns at least 3 chunks
- **Research references**:
  - `.copilot-tracking/research/20260726-sb-markdown-pipeline-research.md` (Lines 208–223) — text processing pipeline
- **Dependencies**:
  - Task 2.1 completion

---

### Task 2.5: Embed helper

Implement `embed_batch()` wrapping the foundry-local-sdk `EmbeddingClient`. The function must take slices of strings and return `Vec<Vec<f32>>`.

- **Files**:
  - `sb/src/main.rs` — add Embed section
- **Required code shape**:

```rust
// ── Embed ─────────────────────────────────────────────────────────────────────

async fn embed_batch(
    client: &foundry_local_sdk::EmbeddingClient,
    texts: &[String],
) -> anyhow::Result<Vec<Vec<f32>>> {
    let refs: Vec<&str> = texts.iter().map(String::as_str).collect();
    let response = client.generate_embeddings(&refs).await?;
    Ok(response.data.into_iter().map(|d| d.embedding).collect())
}
```

- **Success**:
  - Function signature compiles; embeddings length matches input texts length
- **Research references**:
  - `.copilot-tracking/research/20260726-sb-markdown-pipeline-research.md` (Lines 38–40) — foundry-local-sdk embedding API
- **Dependencies**:
  - Task 2.3 completion

---

### Task 2.6: Index logic

Implement `run_index()`. This is the core pipeline: load exclusions → walk directory → parse front matter → clean → spell-correct → chunk → embed (batched) → INSERT into DB → UPDATE `collections.last_indexed_at_utc`.

- **Files**:
  - `sb/src/main.rs` — add Index section
- **Logic steps**:
  1. Load `search.exclude` patterns:
     - Walk up from `path` toward filesystem root, checking each `.vscode/settings.json` found
     - Parse `search.exclude` from the first settings file that contains it (closest to `path` wins)
     - Build a `globset::GlobSet` from the enabled patterns (value `true`)
     - If no settings file is found, use an empty exclusion set (index everything)
  2. Walk `path` with `walkdir::WalkDir`, filter `.md` files; skip any entry whose path relative to `path` matches the `GlobSet`
  3. For each non-excluded file:
     a. Read file contents with `std::fs::read_to_string`
     b. Parse with `gray_matter::Matter::<YAML>::new().parse_with_struct::<DocMeta>(&content)` — handle parse failure gracefully (warn, skip)
     c. If `--force` is false, check if document path already exists in `documents` table — skip if found
     d. Clean body text with `clean_text()`
     e. Apply spell correction with `correct_spelling(&cleaned, &sym)` (initialize SymSpell once before the loop)
     f. Chunk with `chunk_text(&corrected, 512)`
     g. Generate UUID for the document: `uuid::Uuid::new_v4().to_string()`
     h. INSERT or REPLACE into `documents` (id, collection, path, title, tags as JSON, created_at_utc from front matter, updated_at_utc = now, doc_type)
     i. INSERT all chunks into `chunks` (document_id, chunk_index, chunk_text)
  4. After processing all files, batch-embed all collected chunk texts:
     - Call `embed_batch(&client, &all_chunks)` — process in batches of 32 to avoid memory pressure
     - INSERT each embedding into `vec_chunks(rowid, embedding)` matching `chunks.id`
  5. UPDATE `collections SET last_indexed_at_utc = <now_iso8601> WHERE name = <collection>`
- **Handling `--force`**: Delete existing `documents`, `chunks`, and `vec_chunks` rows for the collection before re-indexing
- **Files**:
  - `sb/src/main.rs` — add `run_index` async fn
- **Success**:
  - After running, `SELECT COUNT(*) FROM documents WHERE collection = ?` returns the number of non-excluded `.md` files found
  - `SELECT COUNT(*) FROM vec_chunks` is non-zero
  - `collections.last_indexed_at_utc` is updated to a non-NULL ISO 8601 timestamp
  - Files matching `search.exclude` patterns are absent from the `documents` table
- **Research references**:
  - `.copilot-tracking/research/20260726-sb-markdown-pipeline-research.md` (Lines 143–181) — sqlite-vec insert example
  - `.copilot-tracking/research/20260726-sb-markdown-pipeline-research.md` (Lines 183–223) — gray_matter parsing and pipeline
- **Dependencies**:
  - Tasks 2.2, 2.3, 2.4, 2.5 completion

---

### Task 2.7: Collection commands

Implement four functions: `cmd_collection_ls`, `cmd_collection_add`, `cmd_collection_rm`, `cmd_collection_inspect`.

- **Files**:
  - `sb/src/main.rs` — add Collection commands section

**`cmd_collection_ls(conn)`**:
- Query: `SELECT c.name, c.path, COUNT(DISTINCT d.id) as docs, COUNT(ch.id) as chunks, c.last_indexed_at_utc FROM collections c LEFT JOIN documents d ON d.collection = c.name LEFT JOIN chunks ch ON ch.document_id = d.id GROUP BY c.name`
- Print aligned table with headers: `NAME  PATH  DOCS  CHUNKS  LAST INDEXED (UTC)`

**`cmd_collection_add(conn, path, name, description)`**:
- Canonicalize path with `std::fs::canonicalize`
- INSERT into `collections(name, path, description, created_at_utc)` with `created_at_utc = Utc::now().to_rfc3339()`
- Return error if collection name already exists (don't overwrite)
- Call `run_index(conn, &path, &name, false)` to index immediately

**`cmd_collection_rm(conn, name)`**:
- DELETE `vec_chunks` rows where `rowid IN (SELECT id FROM chunks WHERE document_id IN (SELECT id FROM documents WHERE collection = ?1))`
- DELETE FROM `chunks WHERE document_id IN (SELECT id FROM documents WHERE collection = ?1)`
- DELETE FROM `documents WHERE collection = ?1`
- DELETE FROM `collections WHERE name = ?1`
- Print confirmation: `Removed collection '<name>'`

**`cmd_collection_inspect(conn, name)`**:
- SELECT collection row + COUNT of documents + COUNT of chunks
- Print multi-line detail output

- **Success**:
  - `ls` shows newly added collections
  - `add` inserts row and triggers indexing
  - `rm` removes all rows (verified with COUNT = 0)
  - `inspect` prints correct details
- **Research references**:
  - `.copilot-tracking/research/20260726-sb-markdown-pipeline-research.md` (Lines 62–93) — CLI contract and docker analogy
  - `.copilot-tracking/research/20260726-sb-markdown-pipeline-research.md` (Lines 300–306) — `collection ls` output format
- **Dependencies**:
  - Tasks 2.2, 2.6 completion

---

### Task 2.9: Wire `main()`
Implement `main()` to parse CLI args, resolve the DB path, open DB, and dispatch to the correct command function.

- **Files**:
  - `sb/src/main.rs` — add main section
- **Logic**:
  1. Parse `Cli` with `Cli::parse()`
  2. Resolve DB path: `cli.db` → `SB_DB` env var → `~/.sb/sb.db` default
  3. Call `db_open(&db_path)?`
  4. Match on `cli.command`:
     - `Commands::Collection { action: CollectionCommands::Ls }` → `cmd_collection_ls(&conn)?`
     - `Commands::Collection { action: CollectionCommands::Add { path, name, description } }` → `cmd_collection_add(&conn, &path, &name, description.as_deref())?`
     - `Commands::Collection { action: CollectionCommands::Rm { name } }` → `cmd_collection_rm(&conn, &name)?`
     - `Commands::Collection { action: CollectionCommands::Inspect { name } }` → `cmd_collection_inspect(&conn, &name)?`
     - `Commands::Index { path, collection, force }` → resolve path/collection, call `run_index()`
  5. For `Commands::Index`: if both `path` and `collection` are `None`, print usage error. If only `collection` given, look up path from `collections` table. If only `path` given, derive collection name from `path.file_name()`.
- **Async note**: `run_index` and `embed_batch` are async. The `main` function must be `#[tokio::main]` and `run_index` must be `.await`ed. Non-async commands (collection management) can use synchronous `rusqlite` calls directly.
- **Success**:
  - All subcommands dispatch correctly
  - `sb --help` shows all commands
  - `sb collection --help` shows collection subcommands
- **Research references**:
  - `.copilot-tracking/research/20260726-sb-markdown-pipeline-research.md` (Lines 307–311) — DB path resolution
- **Dependencies**:
  - Tasks 2.1–2.7 completion

---

## Phase 3: Build Validation

### Task 3.1: Build verification

Run `cargo build` (or `cargo check`) for the `sb` package from the workspace root to confirm all dependencies resolve and the code compiles.

- **Files**: No file changes — verification only
- **Command**: `cargo build -p sb`
- **Success**:
  - Exit code 0
  - No unresolved import or type errors
  - Binary produced at `target/debug/sb`
- **Research references**:
  - `.copilot-tracking/research/20260726-sb-markdown-pipeline-research.md` (Lines 225–255) — dependency versions
- **Dependencies**:
  - All Phase 2 tasks complete

---

## Dependencies

- `foundry-local-sdk 1.2` (already in `sb/Cargo.toml`, Windows winml feature required)
- `rusqlite 0.32` with `bundled` feature
- `sqlite-vec 0.1.6`
- `zerocopy 0.7` with `derive` feature
- `gray_matter 0.2`
- `serde 1` with `derive` feature, `serde_json 1`
- `regex 1`
- `text-splitter 0.27`
- `clap 4` with `derive` feature
- `walkdir 2`
- `uuid 1` with `v4` feature
- `chrono 0.4` with `serde` feature
- `anyhow 1`
- `tokio 1` with `rt-multi-thread` and `macros` features
- `symspell 0.4` — spell correction (Rust port of SymSpell algorithm)
- `globset 0.4` — glob pattern matching for `search.exclude` filtering

## Success Criteria

- `cargo build -p sb` exits with code 0
- `sb collection add <path> --name <name>` registers and indexes `.md` files with embeddings stored in `vec_chunks`
- `sb collection ls` prints a table showing doc/chunk counts and last-indexed time
- `sb collection inspect <name>` shows full collection detail
- `sb collection rm <name>` removes all collection data from all 4 tables
- `sb index -c <name>` re-indexes from stored collection path
- `sb index <path>` indexes using path basename as collection name
- `sb index <path> --force` deletes and re-embeds all documents
- Excluded paths (per `search.exclude` in `.vscode/settings.json`) are not indexed
