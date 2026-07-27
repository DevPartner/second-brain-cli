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

// ── DB ────────────────────────────────────────────────────────────────────────

fn db_path_default() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".sb").join("sb.db")
}

fn db_open(path: &Path) -> anyhow::Result<Connection> {
    use std::sync::OnceLock;
    static VEC_REGISTERED: OnceLock<()> = OnceLock::new();

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    VEC_REGISTERED.get_or_init(|| unsafe {
        sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_vec_init as *const ())));
    });
    let conn = Connection::open(path)?;
    create_schema(&conn)?;
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

fn build_symspell() -> symspell::SymSpell<symspell::AsciiStringStrategy> {
    let mut sym = symspell::SymSpell::default();
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());
    let dict_path = PathBuf::from(&home).join(".sb").join("en-80k.txt");
    if dict_path.exists() {
        sym.load_dictionary(dict_path.to_str().unwrap_or(""), 0, 1, " ");
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

fn chunk_text(text: &str, max_chars: usize) -> Vec<String> {
    text_splitter::TextSplitter::new(max_chars)
        .chunks(text)
        .filter(|s| !s.trim().is_empty())
        .map(str::to_owned)
        .collect()
}

// ── Embed ─────────────────────────────────────────────────────────────────────

async fn embed_batch(
    client: &foundry_local_sdk::openai::EmbeddingClient,
    texts: &[String],
) -> anyhow::Result<Vec<Vec<f32>>> {
    if texts.is_empty() {
        return Ok(vec![]);
    }
    let refs: Vec<&str> = texts.iter().map(String::as_str).collect();
    let response = client.generate_embeddings(&refs).await?;
    Ok(response.data.into_iter().map(|d| d.embedding).collect())
}

// ── Index ─────────────────────────────────────────────────────────────────────

fn load_search_exclusions(root: &Path) -> globset::GlobSet {
    let mut current = Some(root);
    while let Some(dir) = current {
        let settings_path = dir.join(".vscode").join("settings.json");
        if settings_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&settings_path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(exclude) = json.get("search.exclude") {
                        let mut builder = globset::GlobSetBuilder::new();
                        if let Some(map) = exclude.as_object() {
                            for (pattern, enabled) in map {
                                if enabled.as_bool().unwrap_or(false) {
                                    if let Ok(glob) = globset::Glob::new(pattern) {
                                        builder.add(glob);
                                    }
                                }
                            }
                        }
                        if let Ok(set) = builder.build() {
                            return set;
                        }
                    }
                }
            }
        }
        current = dir.parent();
    }
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
) -> anyhow::Result<()> {
    let manager = FoundryLocalManager::create(FoundryLocalConfig::new("sb"))?;
    let model = manager.catalog().get_model(ALIAS).await?;
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
    let client = model.create_embedding_client();

    let conn = db_open(db_path)?;

    if force {
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
    }

    let exclusions = load_search_exclusions(path);
    let sym = build_symspell();
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

        let cleaned = clean_text(&body);
        let corrected = correct_spelling(&cleaned, &sym);
        let chunks = chunk_text(&corrected, 512);

        for (idx, chunk) in chunks.iter().enumerate() {
            conn.execute(
                "INSERT INTO chunks(document_id, chunk_index, chunk_text) VALUES (?1, ?2, ?3)",
                rusqlite::params![doc_id, idx as i64, chunk],
            )?;
            let chunk_id = conn.last_insert_rowid();
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
        return Ok(());
    }

    println!("Embedding {} chunks...", chunk_records.len());
    for batch in chunk_records.chunks(32) {
        let texts: Vec<String> = batch.iter().map(|r| r.chunk_text.clone()).collect();
        let embeddings = embed_batch(&client, &texts).await?;
        for (record, embedding) in batch.iter().zip(embeddings.iter()) {
            conn.execute(
                "INSERT INTO vec_chunks(rowid, embedding) VALUES (?1, ?2)",
                rusqlite::params![record.chunk_id, embedding.as_slice().as_bytes()],
            )?;
        }
    }

    conn.execute(
        "UPDATE collections SET last_indexed_at_utc = ?1 WHERE name = ?2",
        rusqlite::params![Utc::now().to_rfc3339(), collection],
    )?;

    println!(
        "Indexed {} chunks into collection '{collection}'.",
        chunk_records.len()
    );
    Ok(())
}

// ── Collection ────────────────────────────────────────────────────────────────

fn cmd_collection_ls(conn: &Connection) -> anyhow::Result<()> {
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
) -> anyhow::Result<()> {
    let canonical = std::fs::canonicalize(path)?;
    {
        let conn = db_open(db_path)?;
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
    run_index(db_path, &canonical, name, false).await
}

fn cmd_collection_rm(conn: &Connection, name: &str) -> anyhow::Result<()> {
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
    println!("Removed collection '{name}'.");
    Ok(())
}

fn cmd_collection_inspect(conn: &Connection, name: &str) -> anyhow::Result<()> {
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
    Ok(())
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn resolve_db_path(flag: Option<&Path>) -> PathBuf {
    if let Some(p) = flag {
        return p.to_path_buf();
    }
    if let Ok(val) = std::env::var("SB_DB") {
        return PathBuf::from(val);
    }
    db_path_default()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let db_path = resolve_db_path(cli.db.as_deref());

    match cli.command {
        Commands::Collection { action } => match action {
            CollectionCommands::Ls => {
                let conn = db_open(&db_path)?;
                cmd_collection_ls(&conn)?;
            }
            CollectionCommands::Add {
                path,
                name,
                description,
            } => {
                cmd_collection_add(&db_path, &path, &name, description.as_deref()).await?;
            }
            CollectionCommands::Rm { name } => {
                let conn = db_open(&db_path)?;
                cmd_collection_rm(&conn, &name)?;
            }
            CollectionCommands::Inspect { name } => {
                let conn = db_open(&db_path)?;
                cmd_collection_inspect(&conn, &name)?;
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
                    let conn = db_open(&db_path)?;
                    let exists: bool = conn
                        .query_row(
                            "SELECT COUNT(*) FROM collections WHERE name = ?1",
                            [&coll_name],
                            |row| row.get::<_, i64>(0),
                        )
                        .map(|n| n > 0)
                        .unwrap_or(false);
                    if !exists {
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
                run_index(&db_path, &p, &coll_name, force).await?;
            }
            (None, Some(col)) => {
                let stored_path = {
                    let conn = db_open(&db_path)?;
                    conn.query_row(
                        "SELECT path FROM collections WHERE name = ?1",
                        [&col],
                        |row| row.get::<_, String>(0),
                    )?
                };
                run_index(&db_path, Path::new(&stored_path), &col, force).await?;
            }
        },
    }
    Ok(())
}
