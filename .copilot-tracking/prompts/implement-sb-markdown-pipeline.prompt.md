<!-- markdownlint-disable-file -->

# Implementation Prompt: sb — Markdown Pipeline with Embeddings and SQLite-Vec

## Task Overview

Transform the `sb` Cargo project from a minimal embedding demo into a full CLI tool. All logic goes into a single `src/main.rs`. The tool indexes Markdown files with YAML front matter, generates embeddings via Azure AI Foundry Local (native SDK), stores them in SQLite with `sqlite-vec`, and provides docker-style collection management.

## Implementation Instructions

### Step 1: Execute implementation

Follow `.copilot-tracking/plans/20260726-sb-markdown-pipeline-plan.md` task-by-task, checking off each item (`[ ]` → `[x]`) as it is completed. Follow ALL project standards and conventions defined in `CLAUDE.md`.

Key conventions to follow:
- All code lives in `sb/src/main.rs` — no additional source files
- Use `// ── Section Title ────────────────────────────────────────────────────────────` separator comments between sections, matching the style in the existing file
- `type` is a Rust keyword — use `doc_type` with `#[serde(rename = "type")]` in `DocMeta`
- `main()` must be `#[tokio::main]` because `run_index` and `embed_batch` are async
- Regex patterns must be compiled once via `OnceLock` — not inside loops
- `sqlite-vec` must be registered via `sqlite3_auto_extension` before `Connection::open`
- DB path resolution order: `--db` flag → `SB_DB` env var → `~/.sb/sb.db`
- All datetime columns use `_utc` suffix (e.g. `created_at_utc`, `last_indexed_at_utc`, `updated_at_utc`)
- Tags stored as JSON string in `documents.tags` column
- `run_index()` initialises symspell via `build_symspell()` once before the file loop, then calls `correct_spelling()` on each chunk's cleaned text
- `.vscode/settings.json` in the indexed directory is parsed for `search.exclude` globs; use `globset` to build a matcher and skip matching paths during `walkdir` traversal

### Step 2: After each phase

After completing each phase, update the plan file to mark the phase header `[x]` and append to `.copilot-tracking/changes/20260726-sb-markdown-pipeline-changes.md` (create if it doesn't exist) following the changes file template in `CLAUDE.md`.

### Step 3: Cleanup

When ALL phases are checked off (`[x]`) and the build passes:

1. Provide a brief summary of all changes made, with markdown links to each modified file.
2. Provide links to:
   - `.copilot-tracking/plans/20260726-sb-markdown-pipeline-plan.md`
   - `.copilot-tracking/details/20260726-sb-markdown-pipeline-details.md`
   - `.copilot-tracking/research/20260726-sb-markdown-pipeline-research.md`
   Recommend cleaning these tracking files up after review.
3. Delete this prompt file: `.copilot-tracking/prompts/implement-sb-markdown-pipeline.prompt.md`

## Success Criteria

- [ ] `cargo build -p sb` exits with code 0
- [ ] `sb collection add <path> --name <name>` registers and indexes `.md` files
- [ ] `sb collection ls` prints collection table
- [ ] `sb collection inspect <name>` shows full detail
- [ ] `sb collection rm <name>` removes all linked data
- [ ] `sb index <path>` indexes using basename as collection name
- [ ] `sb index -c <name>` re-indexes from stored path
- [ ] `sb index <path> --force` re-embeds all documents
