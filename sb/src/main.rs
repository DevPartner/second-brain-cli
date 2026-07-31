use chrono::Utc;
use clap::{Parser, Subcommand};
use foundry_local_sdk::{FoundryLocalConfig, FoundryLocalManager};
use gray_matter::{engine::YAML, Matter};
use rusqlite::{ffi::sqlite3_auto_extension, Connection};
use sqlite_vec::sqlite3_vec_init;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zerocopy::AsBytes;

const ALIAS: &str = "qwen3-embedding-0.6b";

macro_rules! vlog {
    ($verbose:expr, $($arg:tt)*) => {
        if $verbose {
            let ts = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
            println!("[VERBOSE] {} {}", ts, format!($($arg)*));
        }
    };
}

// ── CLI ───────────────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(name = "sb", about = "Second Brain — index and retrieve markdown notes", arg_required_else_help = true)]
struct Cli {
    /// Path to the SQLite database (default: ~/.sb/sb.db, override with SB_DB)
    #[arg(long, global = true)]
    db: Option<PathBuf>,
    /// Enable step-by-step diagnostic output
    #[arg(long, global = true)]
    verbose: bool,
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
    /// Search notes using BM25 keyword matching
    Search {
        /// Search query text
        query: String,
        /// Output results as JSON array
        #[arg(long)]
        json: bool,
        /// Maximum number of results to return
        #[arg(short = 'n', long, default_value = "10")]
        top_k: usize,
        /// Restrict search to this collection
        #[arg(short, long)]
        collection: Option<String>,
    },
    /// Search notes using semantic vector similarity
    Vsearch {
        /// Search query text
        query: String,
        /// Output results as JSON array
        #[arg(long)]
        json: bool,
        /// Maximum number of results to return
        #[arg(short = 'n', long, default_value = "10")]
        top_k: usize,
        /// Restrict search to this collection
        #[arg(short, long)]
        collection: Option<String>,
    },
    /// Retrieve a document by its ID
    Get {
        /// Document ID (short hex #prefix or full UUID)
        id: String,
        /// Restrict lookup to this collection
        #[arg(short, long)]
        collection: Option<String>,
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

// ── DB ────────────────────────────────────────────────────────────────────────

fn sb_home() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".sb")
}

fn db_path_default() -> PathBuf {
    sb_home().join("sb.db")
}

fn db_open(path: &Path, verbose: bool) -> anyhow::Result<Connection> {
    use std::sync::OnceLock;
    static VEC_REGISTERED: OnceLock<()> = OnceLock::new();

    let t = std::time::Instant::now();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    VEC_REGISTERED.get_or_init(|| unsafe {
        sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_vec_init as *const ())));
    });
    let conn = Connection::open(path)?;
    create_schema(&conn)?;
    vlog!(verbose, "db_open({}) -> ok (duration={:.1}ms)", path.display(), t.elapsed().as_secs_f64() * 1000.0);
    Ok(conn)
}

fn create_schema(conn: &Connection) -> anyhow::Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS collections (
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
        USING vec0(embedding float[1024]);",
    )?;
    // Migrate fts_chunks from old single-column content table to 3-column standalone
    // (required for weighted bm25() scoring across title, tags, chunk_text)
    let has_title_col: i64 = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('fts_chunks') WHERE name = 'title'",
        [],
        |row| row.get(0),
    )?;
    if has_title_col == 0 {
        conn.execute_batch(
            "DROP TABLE IF EXISTS fts_chunks;
             CREATE VIRTUAL TABLE fts_chunks USING fts5(title, tags, chunk_text);",
        )?;
        eprintln!("FTS index schema upgraded — run: sb index --force to rebuild");
    }
    Ok(())
}

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

// ── Pipeline ──────────────────────────────────────────────────────────────────

fn build_symspell(verbose: bool) -> symspell::SymSpell<symspell::AsciiStringStrategy> {
    if std::env::var("SB_NO_SPELL").as_deref() == Ok("1") {
        vlog!(verbose, "build_symspell() -> spell correction disabled (SB_NO_SPELL=1)");
        return symspell::SymSpell::default();
    }
    let t = std::time::Instant::now();
    let mut sym = symspell::SymSpell::default();
    let (dict_path, dict_source) = if let Ok(val) = std::env::var("SB_DICT") {
        let raw = PathBuf::from(val);
        let p = if raw.is_absolute() { raw } else { sb_home().join(&raw) };
        (p, "SB_DICT env")
    } else {
        (sb_home().join("en-80k.txt"), "default")
    };
    if dict_path.exists() {
        sym.load_dictionary(dict_path.to_str().unwrap_or(""), 0, 1, " ");
        vlog!(verbose, "build_symspell() -> dict loaded from {} (source: {dict_source}, duration={:.1}ms)", dict_path.display(), t.elapsed().as_secs_f64() * 1000.0);
    } else {
        vlog!(verbose, "build_symspell() -> no dict at {}, spell correction disabled (source: {dict_source}, duration={:.1}ms)", dict_path.display(), t.elapsed().as_secs_f64() * 1000.0);
    }
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
    let ws = WS.get_or_init(|| regex::Regex::new(r"\s+").unwrap());
    let punct = PUNCT.get_or_init(|| regex::Regex::new(r"[^\w\s]").unwrap());
    punct
        .replace_all(&ws.replace_all(text, " "), "")
        .to_lowercase()
}

fn preprocess_text(
    text: &str,
    sym: &symspell::SymSpell<symspell::AsciiStringStrategy>,
) -> String {
    correct_spelling(&clean_text(text), sym)
}

fn build_fts5_query(preprocessed: &str) -> Option<String> {
    let tokens: Vec<&str> = preprocessed
        .split_whitespace()
        .filter(|t| t.len() >= 2)
        .collect();
    if tokens.is_empty() {
        return None;
    }
    let fts_query = tokens
        .iter()
        .map(|t| format!("\"{}\"*", t))
        .collect::<Vec<_>>()
        .join(" OR ");
    Some(fts_query)
}

fn chunk_text(text: &str, max_chars: usize) -> Vec<String> {
    text_splitter::TextSplitter::new(max_chars)
        .chunks(text)
        .filter(|s| !s.trim().is_empty())
        .map(str::to_owned)
        .collect()
}

// ── Embed ─────────────────────────────────────────────────────────────────────

// WinML does not support batch array input — embed one text at a time with retry.
async fn embed_one(
    client: &foundry_local_sdk::openai::EmbeddingClient,
    text: &str,
    verbose: bool,
) -> anyhow::Result<Vec<f32>> {
    for attempt in 1..=3u32 {
        if attempt > 1 {
            let delay_ms = 500u64 * (1u64 << (attempt - 2));
            vlog!(verbose, "embed_one: retry {attempt}/3 (waiting {delay_ms}ms)");
            tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
        }
        let t_attempt = std::time::Instant::now();
        match client.generate_embedding(text).await {
            Ok(response) => {
                let embedding = response
                    .data
                    .into_iter()
                    .next()
                    .map(|d| d.embedding)
                    .ok_or_else(|| anyhow::anyhow!("empty embedding response"))?;
                vlog!(
                    verbose,
                    "embed_one: successful embedding on attempt {attempt}/3 (duration={:.1}ms)",
                    t_attempt.elapsed().as_secs_f64() * 1000.0
                );
                return Ok(embedding);
            }
            Err(e) if attempt < 3 => {
                vlog!(verbose, "embed_one attempt {attempt}/3 failed: {e}");
            }
            Err(e) => return Err(e.into()),
        }
    }
    unreachable!()
}

async fn embed_batch(
    client: &foundry_local_sdk::openai::EmbeddingClient,
    texts: &[String],
    verbose: bool,
) -> anyhow::Result<Vec<Vec<f32>>> {
    use std::sync::atomic::{AtomicBool, Ordering};
    static BATCH_UNSUPPORTED: AtomicBool = AtomicBool::new(false);

    let t_batch = std::time::Instant::now();
    if texts.is_empty() {
        vlog!(verbose, "embed_batch() -> 0 embeddings (empty input)");
        return Ok(vec![]);
    }

    if !BATCH_UNSUPPORTED.load(Ordering::Relaxed) {
        let refs: Vec<&str> = texts.iter().map(String::as_str).collect();
        match client.generate_embeddings(&refs).await {
            Ok(response) => {
                let results: Vec<Vec<f32>> =
                    response.data.into_iter().map(|d| d.embedding).collect();
                vlog!(
                    verbose,
                    "embed_batch(texts={}) -> {} embeddings (batch, duration={:.1}ms)",
                    texts.len(),
                    results.len(),
                    t_batch.elapsed().as_secs_f64() * 1000.0
                );
                return Ok(results);
            }
            Err(e) => {
                BATCH_UNSUPPORTED.store(true, Ordering::Relaxed);
                vlog!(
                    verbose,
                    "embed_batch: batch operation failed ({e}); WinML does not support batch array input — falling back to embed_one (slower)"
                );
                eprintln!(
                    "Warning: batch embedding unsupported ({e}); falling back to one-at-a-time."
                );
            }
        }
    } else {
        vlog!(verbose, "embed_batch: batch known unsupported, using embed_one");
    }

    let mut results = Vec::with_capacity(texts.len());
    for text in texts {
        results.push(embed_one(client, text, verbose).await?);
    }
    vlog!(
        verbose,
        "embed_batch(texts={}) -> {} embeddings (sequential fallback, duration={:.1}ms)",
        texts.len(),
        results.len(),
        t_batch.elapsed().as_secs_f64() * 1000.0
    );
    Ok(results)
}

// ── Index ─────────────────────────────────────────────────────────────────────

fn load_search_exclusions(root: &Path, verbose: bool) -> globset::GlobSet {
    let t = std::time::Instant::now();
    let mut current = Some(root);
    while let Some(dir) = current {
        let settings_path = dir.join(".vscode").join("settings.json");
        if settings_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&settings_path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(exclude) = json.get("search.exclude") {
                        let mut builder = globset::GlobSetBuilder::new();
                        let mut count = 0usize;
                        if let Some(map) = exclude.as_object() {
                            for (pattern, enabled) in map {
                                if enabled.as_bool().unwrap_or(false) {
                                    if let Ok(glob) = globset::Glob::new(pattern) {
                                        builder.add(glob);
                                        count += 1;
                                    }
                                }
                            }
                        }
                        if let Ok(set) = builder.build() {
                            vlog!(
                                verbose,
                                "load_search_exclusions({}) -> {} patterns from {} (duration={:.1}ms)",
                                root.display(),
                                count,
                                settings_path.display(),
                                t.elapsed().as_secs_f64() * 1000.0
                            );
                            return set;
                        }
                    }
                }
            }
        }
        current = dir.parent();
    }
    vlog!(verbose, "load_search_exclusions({}) -> no exclusions found (duration={:.1}ms)", root.display(), t.elapsed().as_secs_f64() * 1000.0);
    globset::GlobSetBuilder::new()
        .build()
        .unwrap_or_else(|_| globset::GlobSet::empty())
}

struct ChunkRecord {
    chunk_id: i64,
    chunk_text: String,
}

async fn run_index(
    db_path: &Path,
    path: &Path,
    collection: &str,
    force: bool,
    verbose: bool,
) -> anyhow::Result<()> {
    vlog!(
        verbose,
        "run_index(path={}, collection={}, force={})",
        path.display(),
        collection,
        force
    );

    let conn = db_open(db_path, verbose)?;

    if force {
        vlog!(verbose, "run_index: force-clearing existing data for collection '{collection}'");
        conn.execute(
            "DELETE FROM vec_chunks WHERE rowid IN \
             (SELECT id FROM chunks WHERE document_id IN \
             (SELECT id FROM documents WHERE collection = ?1))",
            [collection],
        )?;
        conn.execute(
            "DELETE FROM chunks WHERE document_id IN \
             (SELECT id FROM documents WHERE collection = ?1)",
            [collection],
        )?;
        conn.execute("DELETE FROM documents WHERE collection = ?1", [collection])?;
        conn.execute_batch("INSERT INTO fts_chunks(fts_chunks) VALUES('rebuild')")?;
    }

    let exclusions = load_search_exclusions(path, verbose);
    let sym = build_symspell(verbose);
    let mut chunk_records: Vec<ChunkRecord> = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_entry(|e| {
        if e.depth() == 0 {
            return true;
        }
        let rel = e.path().strip_prefix(path).unwrap_or(e.path());
        !exclusions.is_match(rel)
    }) {
        let entry = entry?;
        if entry.file_type().is_dir() {
            continue;
        }
        if entry.path().extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let file_path = entry.path();
        let rel_path = file_path.strip_prefix(path).unwrap_or(file_path);
        let rel_path_str = rel_path.to_string_lossy().to_string();

        if !force {
            let exists: bool = conn
                .query_row(
                    "SELECT COUNT(*) FROM documents WHERE path = ?1 AND collection = ?2",
                    rusqlite::params![rel_path_str, collection],
                    |row| row.get::<_, i64>(0),
                )
                .map(|n| n > 0)
                .unwrap_or(false);
            if exists {
                vlog!(verbose, "run_index: skipping already-indexed file {rel_path_str}");
                continue;
            }
        }

        let content = match std::fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Warning: could not read {:?}: {e}", file_path);
                continue;
            }
        };

        let matter = Matter::<YAML>::new();
        let (meta, body) = match matter.parse_with_struct::<DocMeta>(&content) {
            Some(parsed) => (parsed.data, parsed.content),
            None => (DocMeta::default(), content.clone()),
        };

        let tags_json = serde_json::to_string(&meta.tags).unwrap_or_else(|_| "null".into());
        let doc_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT OR REPLACE INTO documents \
             (id, collection, path, title, tags, created_at_utc, updated_at_utc, doc_type) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                doc_id,
                collection,
                rel_path_str,
                meta.title,
                tags_json,
                meta.created,
                now,
                meta.doc_type,
            ],
        )?;

        let doc_title = meta.title.as_deref().unwrap_or("").to_string();
        let doc_tags = meta.tags.as_ref().map(|t| t.join(" ")).unwrap_or_default();

        let preprocessed = preprocess_text(&body, &sym);
        let chunks = chunk_text(&preprocessed, 512);

        for (idx, chunk) in chunks.iter().enumerate() {
            conn.execute(
                "INSERT INTO chunks(document_id, chunk_index, chunk_text) VALUES (?1, ?2, ?3)",
                rusqlite::params![doc_id, idx as i64, chunk],
            )?;
            let chunk_id = conn.last_insert_rowid();
            conn.execute(
                "INSERT INTO fts_chunks(rowid, title, tags, chunk_text) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![chunk_id, doc_title, doc_tags, chunk],
            )?;
            chunk_records.push(ChunkRecord {
                chunk_id,
                chunk_text: chunk.clone(),
            });
        }
    }

    if chunk_records.is_empty() {
        println!("No new content to index in collection '{collection}'.");
        conn.execute(
            "UPDATE collections SET last_indexed_at_utc = ?1 WHERE name = ?2",
            rusqlite::params![Utc::now().to_rfc3339(), collection],
        )?;
        vlog!(verbose, "run_index(collection={collection}) -> 0 new chunks");
        return Ok(());
    }

    let model_alias = std::env::var("SB_MODEL").unwrap_or_else(|_| ALIAS.to_string());
    vlog!(verbose, "run_index: loading embedding model '{model_alias}'");
    let t_model = std::time::Instant::now();
    let manager = FoundryLocalManager::create(FoundryLocalConfig::new("sb"))?;
    let model = manager.catalog().get_model(&model_alias).await?;
    if !model.is_cached().await? {
        println!("Downloading embedding model...");
        model
            .download(Some(|p: f64| {
                print!("\r  {p:.1}%");
                std::io::Write::flush(&mut std::io::stdout()).ok();
            }))
            .await?;
        println!();
    }
    model.load().await?;
    vlog!(verbose, "run_index: embedding model loaded (duration={:.1}ms)", t_model.elapsed().as_secs_f64() * 1000.0);
    let client = model.create_embedding_client();
    vlog!(verbose, "run_index: embedding client ready");

    println!("Embedding {} chunks...", chunk_records.len());
    let t_embed_total = std::time::Instant::now();
    for batch in chunk_records.chunks(32) {
        let texts: Vec<String> = batch.iter().map(|r| r.chunk_text.clone()).collect();
        vlog!(verbose, "run_index: embedding batch of {} chunks", texts.len());
        let embeddings = embed_batch(&client, &texts, verbose).await?;
        for (record, embedding) in batch.iter().zip(embeddings.iter()) {
            conn.execute(
                "INSERT INTO vec_chunks(rowid, embedding) VALUES (?1, ?2)",
                rusqlite::params![record.chunk_id, embedding.as_slice().as_bytes()],
            )?;
        }
    }
    vlog!(verbose, "run_index: total embedding time (duration={:.1}ms)", t_embed_total.elapsed().as_secs_f64() * 1000.0);

    conn.execute(
        "UPDATE collections SET last_indexed_at_utc = ?1 WHERE name = ?2",
        rusqlite::params![Utc::now().to_rfc3339(), collection],
    )?;

    println!(
        "Indexed {} chunks into collection '{collection}'.",
        chunk_records.len()
    );
    vlog!(
        verbose,
        "run_index(collection={collection}) -> {} chunks indexed",
        chunk_records.len()
    );
    Ok(())
}

// ── Collection ────────────────────────────────────────────────────────────────

fn cmd_collection_ls(conn: &Connection, verbose: bool) -> anyhow::Result<()> {
    vlog!(verbose, "cmd_collection_ls()");
    let mut stmt = conn.prepare(
        "SELECT c.name, c.path, COUNT(DISTINCT d.id) as docs, COUNT(ch.id) as chunks, \
         c.last_indexed_at_utc \
         FROM collections c \
         LEFT JOIN documents d ON d.collection = c.name \
         LEFT JOIN chunks ch ON ch.document_id = d.id \
         GROUP BY c.name",
    )?;
    let rows: Vec<(String, String, i64, i64, Option<String>)> = stmt
        .query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?
        .filter_map(|r| r.ok())
        .collect();

    vlog!(verbose, "cmd_collection_ls() -> {} collections", rows.len());

    if rows.is_empty() {
        println!("No collections.");
        return Ok(());
    }

    println!(
        "{:<20} {:<40} {:>6} {:>8}  {}",
        "NAME", "PATH", "DOCS", "CHUNKS", "LAST INDEXED (UTC)"
    );
    println!("{}", "-".repeat(90));
    for (name, path, docs, chunks, last_indexed) in &rows {
        println!(
            "{:<20} {:<40} {:>6} {:>8}  {}",
            name,
            path,
            docs,
            chunks,
            last_indexed.as_deref().unwrap_or("-")
        );
    }
    Ok(())
}

async fn cmd_collection_add(
    db_path: &Path,
    path: &Path,
    name: &str,
    description: Option<&str>,
    verbose: bool,
) -> anyhow::Result<()> {
    vlog!(
        verbose,
        "cmd_collection_add(path={}, name={}, description={:?})",
        path.display(),
        name,
        description
    );
    let canonical = std::fs::canonicalize(path)?;
    {
        let conn = db_open(db_path, verbose)?;
        let exists: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM collections WHERE name = ?1",
                [name],
                |row| row.get::<_, i64>(0),
            )
            .map(|n| n > 0)
            .unwrap_or(false);
        if exists {
            anyhow::bail!("Collection '{}' already exists", name);
        }
        conn.execute(
            "INSERT INTO collections(name, path, description, created_at_utc) \
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                name,
                canonical.to_string_lossy().as_ref(),
                description,
                Utc::now().to_rfc3339()
            ],
        )?;
    }
    run_index(db_path, &canonical, name, false, verbose).await?;
    vlog!(verbose, "cmd_collection_add(name={name}) -> ok");
    Ok(())
}

fn cmd_collection_rm(conn: &Connection, name: &str, verbose: bool) -> anyhow::Result<()> {
    vlog!(verbose, "cmd_collection_rm(name={name})");
    conn.execute(
        "DELETE FROM vec_chunks WHERE rowid IN \
         (SELECT id FROM chunks WHERE document_id IN \
         (SELECT id FROM documents WHERE collection = ?1))",
        [name],
    )?;
    conn.execute(
        "DELETE FROM chunks WHERE document_id IN \
         (SELECT id FROM documents WHERE collection = ?1)",
        [name],
    )?;
    conn.execute("DELETE FROM documents WHERE collection = ?1", [name])?;
    conn.execute("DELETE FROM collections WHERE name = ?1", [name])?;
    conn.execute_batch("INSERT INTO fts_chunks(fts_chunks) VALUES('rebuild')")?;
    println!("Removed collection '{name}'.");
    vlog!(verbose, "cmd_collection_rm(name={name}) -> ok");
    Ok(())
}

fn cmd_collection_inspect(conn: &Connection, name: &str, verbose: bool) -> anyhow::Result<()> {
    vlog!(verbose, "cmd_collection_inspect(name={name})");
    let row: Option<(String, String, Option<String>, String, Option<String>)> = conn
        .query_row(
            "SELECT name, path, description, created_at_utc, last_indexed_at_utc \
             FROM collections WHERE name = ?1",
            [name],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
        )
        .ok();

    let (col_name, path, description, created, last_indexed) = match row {
        Some(r) => r,
        None => anyhow::bail!("Collection '{}' not found", name),
    };

    let doc_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM documents WHERE collection = ?1",
            [name],
            |row| row.get(0),
        )
        .unwrap_or(0);

    let chunk_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM chunks WHERE document_id IN \
             (SELECT id FROM documents WHERE collection = ?1)",
            [name],
            |row| row.get(0),
        )
        .unwrap_or(0);

    println!("Name:         {col_name}");
    println!("Path:         {path}");
    println!("Description:  {}", description.as_deref().unwrap_or("-"));
    println!("Created:      {created}");
    println!("Last indexed: {}", last_indexed.as_deref().unwrap_or("-"));
    println!("Documents:    {doc_count}");
    println!("Chunks:       {chunk_count}");
    vlog!(
        verbose,
        "cmd_collection_inspect(name={name}) -> docs={doc_count} chunks={chunk_count}"
    );
    Ok(())
}

// ── Search ────────────────────────────────────────────────────────────────────

#[derive(Debug, serde::Serialize)]
struct SearchResult {
    docid: String,
    score: f64,
    file: String,
    title: Option<String>,
    context: String,
    snippet: String,
}

fn short_docid(uuid: &str) -> String {
    format!("#{}", &uuid[..uuid.len().min(7)])
}

fn build_file_uri(collection: &str, path: &str) -> String {
    format!("sb://{collection}/{path}")
}

fn output_search_results(
    results: &[SearchResult],
    json_output: bool,
    query: &str,
) -> anyhow::Result<()> {
    if json_output {
        println!("{}", serde_json::to_string_pretty(results)?);
    } else if results.is_empty() {
        println!("No results for {query:?}.");
    } else {
        for (i, r) in results.iter().enumerate() {
            println!("\n[{}] {} (score: {:.4})", i + 1, r.file, r.score);
            if let Some(t) = &r.title {
                println!("    Title: {t}");
            }
            let preview: String = r.snippet.chars().take(120).collect();
            println!("    {preview}");
        }
    }
    Ok(())
}

fn cmd_search(
    conn: &Connection,
    query: &str,
    collection: Option<&str>,
    top_k: usize,
    json_output: bool,
    verbose: bool,
) -> anyhow::Result<()> {
    let t = std::time::Instant::now();
    vlog!(verbose, "cmd_search(query={:?}, collection={:?}, top_k={})", query, collection, top_k);

    let sym = build_symspell(verbose);
    let preprocessed = preprocess_text(query, &sym);
    let fts_query = match build_fts5_query(&preprocessed) {
        Some(q) => q,
        None => {
            vlog!(verbose, "cmd_search() -> empty query after preprocessing (duration={:.1}ms)", t.elapsed().as_secs_f64() * 1000.0);
            println!("No results for {:?}.", query);
            return Ok(());
        }
    };
    vlog!(verbose, "cmd_search: fts_query={:?}", fts_query);

    type Row6 = (f64, String, String, String, Option<String>, String);

    let rows: Vec<Row6> = if let Some(coll) = collection {
        let fetch_k = top_k * 10;
        let mut stmt = conn.prepare(
            "WITH fts_matches AS (
                 SELECT rowid, bm25(fts_chunks, 2.0, 1.5, 1.0) AS bm25_score
                 FROM fts_chunks
                 WHERE fts_chunks MATCH ?1
                 ORDER BY bm25_score ASC
                 LIMIT ?2
             )
             SELECT fm.bm25_score, ch.chunk_text, d.id, d.path, d.title, d.collection
             FROM fts_matches fm
             JOIN chunks ch ON ch.id = fm.rowid
             JOIN documents d ON d.id = ch.document_id
             WHERE d.collection = ?3
             ORDER BY fm.bm25_score ASC
             LIMIT ?4",
        )?;
        let r: Vec<Row6> = stmt
            .query_map(
                rusqlite::params![fts_query, fetch_k as i64, coll, top_k as i64],
                |row| {
                    Ok((
                        row.get::<_, f64>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, Option<String>>(4)?,
                        row.get::<_, String>(5)?,
                    ))
                },
            )?
            .filter_map(|r| r.ok())
            .collect();
        r
    } else {
        let mut stmt = conn.prepare(
            "WITH fts_matches AS (
                 SELECT rowid, bm25(fts_chunks, 2.0, 1.5, 1.0) AS bm25_score
                 FROM fts_chunks
                 WHERE fts_chunks MATCH ?1
                 ORDER BY bm25_score ASC
                 LIMIT ?2
             )
             SELECT fm.bm25_score, ch.chunk_text, d.id, d.path, d.title, d.collection
             FROM fts_matches fm
             JOIN chunks ch ON ch.id = fm.rowid
             JOIN documents d ON d.id = ch.document_id
             ORDER BY fm.bm25_score ASC
             LIMIT ?2",
        )?;
        let r: Vec<Row6> = stmt
            .query_map(
                rusqlite::params![fts_query, top_k as i64],
                |row| {
                    Ok((
                        row.get::<_, f64>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, Option<String>>(4)?,
                        row.get::<_, String>(5)?,
                    ))
                },
            )?
            .filter_map(|r| r.ok())
            .collect();
        r
    };

    let results: Vec<SearchResult> = rows
        .into_iter()
        .map(|(raw_score, snippet, doc_id, path, title, coll)| {
            let score = raw_score.abs() / (1.0 + raw_score.abs());
            SearchResult {
                docid: short_docid(&doc_id),
                score,
                file: build_file_uri(&coll, &path),
                title,
                context: coll,
                snippet,
            }
        })
        .collect();

    vlog!(
        verbose,
        "cmd_search() -> {} results (duration={:.1}ms)",
        results.len(),
        t.elapsed().as_secs_f64() * 1000.0
    );
    output_search_results(&results, json_output, query)
}

async fn cmd_vsearch(
    db_path: &Path,
    query: &str,
    collection: Option<&str>,
    top_k: usize,
    json_output: bool,
    verbose: bool,
) -> anyhow::Result<()> {
    let t = std::time::Instant::now();
    vlog!(verbose, "cmd_vsearch(query={:?}, collection={:?}, top_k={})", query, collection, top_k);

    let model_alias = std::env::var("SB_MODEL").unwrap_or_else(|_| ALIAS.to_string());
    vlog!(verbose, "cmd_vsearch: loading embedding model '{model_alias}'");
    let t_model = std::time::Instant::now();
    let manager = FoundryLocalManager::create(FoundryLocalConfig::new("sb"))?;
    let model = manager.catalog().get_model(&model_alias).await?;
    if !model.is_cached().await? {
        anyhow::bail!(
            "Embedding model '{model_alias}' not cached. Run `sb index` first to download it."
        );
    }
    model.load().await?;
    vlog!(
        verbose,
        "cmd_vsearch: model loaded (duration={:.1}ms)",
        t_model.elapsed().as_secs_f64() * 1000.0
    );
    let client = model.create_embedding_client();

    let sym = build_symspell(verbose);
    let preprocessed_query = preprocess_text(query, &sym);
    vlog!(verbose, "cmd_vsearch: preprocessed_query={:?}", preprocessed_query);
    let t_embed = std::time::Instant::now();
    let embedding = embed_one(&client, &preprocessed_query, verbose).await?;
    let embedding_bytes = embedding.as_slice().as_bytes();
    vlog!(
        verbose,
        "cmd_vsearch: query embedded (duration={:.1}ms)",
        t_embed.elapsed().as_secs_f64() * 1000.0
    );

    // sqlite-vec applies k to the vector index before the JOIN, so when a collection filter
    // is used the post-JOIN filter may reduce the count below top_k — fetch more to compensate.
    let fetch_k = if collection.is_some() { top_k * 10 } else { top_k };

    let conn = db_open(db_path, verbose)?;
    let t_query = std::time::Instant::now();
    let mut stmt = conn.prepare(
        "WITH knn AS (
             SELECT rowid, distance
             FROM vec_chunks
             WHERE embedding MATCH ?1 AND k = ?2
         )
         SELECT knn.distance, ch.chunk_text, d.id, d.path, d.title, d.collection
         FROM knn
         JOIN chunks ch ON ch.id = knn.rowid
         JOIN documents d ON d.id = ch.document_id
         ORDER BY knn.distance",
    )?;
    let raw: Vec<_> = stmt
        .query_map(rusqlite::params![embedding_bytes, fetch_k as i64], |row| {
            Ok((
                row.get::<_, f64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, String>(5)?,
            ))
        })?
        .filter_map(|r| r.ok())
        .collect();

    let results: Vec<SearchResult> = raw
        .into_iter()
        .filter(|(_, _, _, _, _, coll)| collection.map_or(true, |c| coll == c))
        .take(top_k)
        .map(|(distance, snippet, doc_id, path, title, coll)| SearchResult {
            docid: short_docid(&doc_id),
            score: 1.0 / (1.0 + distance),
            file: build_file_uri(&coll, &path),
            title,
            context: coll,
            snippet,
        })
        .collect();

    vlog!(
        verbose,
        "cmd_vsearch: KNN query -> {} results (duration={:.1}ms)",
        results.len(),
        t_query.elapsed().as_secs_f64() * 1000.0
    );
    vlog!(
        verbose,
        "cmd_vsearch() total (duration={:.1}ms)",
        t.elapsed().as_secs_f64() * 1000.0
    );
    output_search_results(&results, json_output, query)
}

fn cmd_get(
    conn: &Connection,
    id: &str,
    collection: Option<&str>,
    verbose: bool,
) -> anyhow::Result<()> {
    vlog!(verbose, "cmd_get(id={:?}, collection={:?})", id, collection);
    let id_clean = id.trim_start_matches('#');

    let row = if let Some(coll) = collection {
        conn.query_row(
            "SELECT d.id, d.path, d.title, d.tags, d.doc_type, d.collection, \
             d.created_at_utc, d.updated_at_utc, \
             (SELECT GROUP_CONCAT(chunk_text, char(10)) FROM (SELECT chunk_text FROM chunks WHERE document_id = d.id ORDER BY chunk_index)) AS full_text \
             FROM documents d \
             WHERE d.id LIKE ?1 || '%' AND d.collection = ?2 \
             GROUP BY d.id",
            rusqlite::params![id_clean, coll],
            |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, Option<String>>(2)?,
                    r.get::<_, Option<String>>(3)?,
                    r.get::<_, Option<String>>(4)?,
                    r.get::<_, String>(5)?,
                    r.get::<_, Option<String>>(6)?,
                    r.get::<_, Option<String>>(7)?,
                    r.get::<_, String>(8)?,
                ))
            },
        )
        .ok()
    } else {
        conn.query_row(
            "SELECT d.id, d.path, d.title, d.tags, d.doc_type, d.collection, \
             d.created_at_utc, d.updated_at_utc, \
             GROUP_CONCAT(ch.chunk_text, char(10)) AS full_text \
             FROM documents d \
             JOIN chunks ch ON ch.document_id = d.id \
             WHERE d.id LIKE ?1 || '%' \
             GROUP BY d.id",
            rusqlite::params![id_clean],
            |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, Option<String>>(2)?,
                    r.get::<_, Option<String>>(3)?,
                    r.get::<_, Option<String>>(4)?,
                    r.get::<_, String>(5)?,
                    r.get::<_, Option<String>>(6)?,
                    r.get::<_, Option<String>>(7)?,
                    r.get::<_, String>(8)?,
                ))
            },
        )
        .ok()
    };

    let (doc_id, path, title, tags_json, doc_type, coll, created, updated, full_text) =
        row.ok_or_else(|| anyhow::anyhow!("Document '{}' not found", id))?;

    let tags: Option<serde_json::Value> = tags_json
        .as_deref()
        .and_then(|s| serde_json::from_str(s).ok());

    let result = serde_json::json!({
        "docid":          short_docid(&doc_id),
        "file":           build_file_uri(&coll, &path),
        "title":          title,
        "tags":           tags,
        "doc_type":       doc_type,
        "collection":     coll,
        "created_at_utc": created,
        "updated_at_utc": updated,
        "full_text":      full_text,
    });
    println!("{}", serde_json::to_string_pretty(&result)?);
    vlog!(verbose, "cmd_get(id={:?}) -> found document in '{coll}'", id);
    Ok(())
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn resolve_db_path(flag: Option<&Path>, verbose: bool) -> PathBuf {
    if let Some(p) = flag {
        vlog!(verbose, "resolve_db_path() -> {} (source: --db flag)", p.display());
        return p.to_path_buf();
    }
    if let Ok(val) = std::env::var("SB_DB") {
        let raw = PathBuf::from(&val);
        let p = if raw.is_absolute() { raw } else { sb_home().join(&raw) };
        vlog!(verbose, "resolve_db_path() -> {} (source: SB_DB env)", p.display());
        return p;
    }
    let p = db_path_default();
    vlog!(verbose, "resolve_db_path() -> {} (source: default)", p.display());
    p
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();
    let verbose = cli.verbose;
    let db_path = resolve_db_path(cli.db.as_deref(), verbose);

    match cli.command {
        Commands::Collection { action } => match action {
            CollectionCommands::Ls => {
                let conn = db_open(&db_path, verbose)?;
                cmd_collection_ls(&conn, verbose)?;
            }
            CollectionCommands::Add {
                path,
                name,
                description,
            } => {
                cmd_collection_add(&db_path, &path, &name, description.as_deref(), verbose).await?;
            }
            CollectionCommands::Rm { name } => {
                let conn = db_open(&db_path, verbose)?;
                cmd_collection_rm(&conn, &name, verbose)?;
            }
            CollectionCommands::Inspect { name } => {
                let conn = db_open(&db_path, verbose)?;
                cmd_collection_inspect(&conn, &name, verbose)?;
            }
        },
        Commands::Index {
            path,
            collection,
            force,
        } => match (path, collection) {
            (None, None) => {
                anyhow::bail!("Provide a <path> or -c <collection>.");
            }
            (Some(p), col) => {
                let coll_name = col.unwrap_or_else(|| {
                    p.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned()
                });
                {
                    let conn = db_open(&db_path, verbose)?;
                    let exists: bool = conn
                        .query_row(
                            "SELECT COUNT(*) FROM collections WHERE name = ?1",
                            [&coll_name],
                            |row| row.get::<_, i64>(0),
                        )
                        .map(|n| n > 0)
                        .unwrap_or(false);
                    if !exists {
                        vlog!(verbose, "main: auto-registering collection '{coll_name}'");
                        conn.execute(
                            "INSERT INTO collections(name, path, description, created_at_utc) \
                             VALUES (?1, ?2, NULL, ?3)",
                            rusqlite::params![
                                coll_name,
                                p.to_string_lossy().as_ref(),
                                Utc::now().to_rfc3339()
                            ],
                        )?;
                    }
                }
                run_index(&db_path, &p, &coll_name, force, verbose).await?;
            }
            (None, Some(col)) => {
                let stored_path = {
                    let conn = db_open(&db_path, verbose)?;
                    conn.query_row(
                        "SELECT path FROM collections WHERE name = ?1",
                        [&col],
                        |row| row.get::<_, String>(0),
                    )?
                };
                vlog!(verbose, "main: resolved collection '{col}' -> path={stored_path}");
                run_index(&db_path, Path::new(&stored_path), &col, force, verbose).await?;
            }
        },
        Commands::Search {
            query,
            json,
            top_k,
            collection,
        } => {
            let conn = db_open(&db_path, verbose)?;
            cmd_search(&conn, &query, collection.as_deref(), top_k, json, verbose)?;
        }
        Commands::Vsearch {
            query,
            json,
            top_k,
            collection,
        } => {
            cmd_vsearch(&db_path, &query, collection.as_deref(), top_k, json, verbose).await?;
        }
        Commands::Get { id, collection } => {
            let conn = db_open(&db_path, verbose)?;
            cmd_get(&conn, &id, collection.as_deref(), verbose)?;
        }
    }
    Ok(())
}
