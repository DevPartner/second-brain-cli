<!-- markdownlint-disable-file -->

# Task Checklist: sb — Markdown Pipeline with Embeddings and SQLite-Vec

## Overview

Transform the `sb` project from a minimal embedding demo into a full CLI tool that indexes Markdown files with YAML front matter, generates embeddings via Azure AI Foundry Local, stores them in SQLite with `sqlite-vec`, and exposes docker-style collection management — all in a single `src/main.rs`.

## Objectives

- Rename the `sb` Cargo package from `embeddings` to `sb` and add it to the workspace
- Implement docker-style collection management: `sb collection ls/add/rm/inspect`
- Implement `sb index [<path>] [-c <collection>] [--force]` that ingests `.md` files end-to-end
- All logic in a single `src/main.rs` using `// ── Section ──` comment style

## Research Summary

### Project Files

- `sb/Cargo.toml` — package name `embeddings`; only `foundry-local-sdk` and `tokio` dependencies; must be replaced
- `sb/src/main.rs` — existing embedding demo; establishes `// ── Section ──` comment style to follow
- `Cargo.toml` (workspace root) — `sb` not yet a member; must be added

### External References

- `.copilot-tracking/research/20260726-sb-markdown-pipeline-research.md` — full crate versions, schema, API examples, CLI contract

## Implementation Checklist

### [ ] Phase 1: Project Configuration

- [ ] Task 1.1: Update `sb/Cargo.toml` — rename package to `sb`, replace dependencies
  - Details: `.copilot-tracking/details/20260726-sb-markdown-pipeline-details.md` (Lines 13–55)

- [ ] Task 1.2: Add `sb` to workspace `Cargo.toml` members list
  - Details: `.copilot-tracking/details/20260726-sb-markdown-pipeline-details.md` (Lines 57–72)

### [ ] Phase 2: Rewrite `src/main.rs`

- [ ] Task 2.1: CLI structure — clap derive `Cli`, `Commands`, `CollectionCommands`
  - Details: `.copilot-tracking/details/20260726-sb-markdown-pipeline-details.md` (Lines 75–118)

- [ ] Task 2.2: DB helpers — `db_open()` and `create_schema()` with sqlite-vec extension
  - Details: `.copilot-tracking/details/20260726-sb-markdown-pipeline-details.md` (Lines 121–175)

- [ ] Task 2.3: Types — `DocMeta` struct for gray_matter deserialization
  - Details: `.copilot-tracking/details/20260726-sb-markdown-pipeline-details.md` (Lines 178–202)

- [ ] Task 2.4: Pipeline helpers — `clean_text()`, `build_symspell()`, `correct_spelling()`, and `chunk_text()`
  - Details: `.copilot-tracking/details/20260726-sb-markdown-pipeline-details.md` (Lines 205–230)

- [ ] Task 2.5: Embed helper — `embed_batch()` wrapping foundry-local-sdk
  - Details: `.copilot-tracking/details/20260726-sb-markdown-pipeline-details.md` (Lines 233–265)

- [ ] Task 2.6: Index logic — `run_index()` end-to-end pipeline with `.vscode/settings.json` search.exclude filtering via `globset`
  - Details: `.copilot-tracking/details/20260726-sb-markdown-pipeline-details.md` (Lines 268–335)

- [ ] Task 2.7: Collection commands — `cmd_collection_add/rm/ls/inspect()`
  - Details: `.copilot-tracking/details/20260726-sb-markdown-pipeline-details.md` (Lines 338–415)

- [ ] Task 2.8: Wire `main()` — dispatch all commands, resolve DB path from env/flag
  - Details: `.copilot-tracking/details/20260726-sb-markdown-pipeline-details.md` (Lines 418–455)

### [ ] Phase 3: Build Validation

- [ ] Task 3.1: `cargo build` succeeds from workspace root
  - Details: `.copilot-tracking/details/20260726-sb-markdown-pipeline-details.md` (Lines 498–515)

## Dependencies

- `foundry-local-sdk 1.2` (already in `sb/Cargo.toml`)
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
- `symspell 0.4`
- `globset 0.4`
- `tokio 1` with `rt-multi-thread` and `macros` features

## Success Criteria

- `cargo build --release` succeeds in the workspace with no errors
- `sb collection add <path> --name <name>` registers and indexes `.md` files
- `sb collection ls` prints a table with doc/chunk counts and last-indexed time
- `sb collection inspect <name>` prints full collection detail
- `sb collection rm <name>` removes collection and all linked data
- `sb index -c <name>` re-indexes from the stored collection path
- `sb index <path>` indexes using path basename as collection name
- `sb index <path> --force` re-embeds all documents
