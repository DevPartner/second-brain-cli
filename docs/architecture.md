---
type: "architecture"
date: 2026-08-01
tags: [architecture, rag, second-brain]
---

# second-brain-cli — Architecture

> Local-first RAG knowledge retrieval: Rust CLI + SQLite + Foundry Local

---

## RAG Technology Choices

### Choosing an LLM for RAG

```mermaid
graph LR
    LLM["LLM\nGeneration Layer"] --> A[Closed Source]
    LLM --> B[Open Source]
    LLM --> C[Domain Specific]

    A --> A1["GPT · Claude · Gemini · Cohere
    ✅ Easy adoption · strong out-of-box quality
    ⚠️ Vendor lock-in · privacy constraints"]

    B --> B1["Llama · Mistral
    ✅ Control · flexible deployment
    ⚠️ GPU infrastructure · in-house expertise"]

    C --> C1["BioBERT · FinBERT
    ✅ Domain accuracy for narrow terminology
    ⚠️ Limited to specific problem space"]

    style A1 fill:#f0f8ff,stroke:#93c5fd,color:#000000
    style B1 fill:#f0f8ff,stroke:#93c5fd,color:#000000
    style C1 fill:#f0f8ff,stroke:#93c5fd,color:#000000
```

---

### Choosing an Embedding Model for RAG

```mermaid
graph LR
    EM["Embedding Model\nRetrieval Layer"] --> A[Commercial API]
    EM --> B[Open Source]

    A --> A1["OpenAI text-embedding-3 · Cohere embed-v3
    ✅ Easy API · no infra
    ⚠️ Cost · data leaves your machine"]

    B --> B1["sentence-transformers · E5 · Instructor
    ✅ Control · task-specific accuracy
    ⚠️ Self-hosting required"]

    style A1 fill:#f0f8ff,stroke:#93c5fd,color:#000000
    style B1 fill:#f0f8ff,stroke:#93c5fd,color:#000000
```

**This workspace:** `qwen3-embedding-0.6b` via Foundry Local — open source, on-device (WinML), 1024-dim, no API key

---

### Choosing a Vector Database for RAG

```mermaid
graph LR
    VDB["Vector Database\nEmbedding Store"] --> A[Cloud Managed]
    VDB --> B[Self-Hosted]
    VDB --> C[Embedded]

    A --> A1["Pinecone · Weaviate Cloud · Qdrant Cloud
    ✅ Fast path to production
    ⚠️ Ongoing cost · data egress"]

    B --> B1["ChromaDB · Milvus · Elasticsearch · pgvector
    ✅ Control · cheaper long-term
    ⚠️ DevOps: capacity · upgrades · backups"]

    C --> C1["sqlite-vec · LanceDB
    ✅ Zero infra · local-first · co-located
    ⚠️ Scale ceiling for very large collections"]

    style A1 fill:#f0f8ff,stroke:#93c5fd,color:#000000
    style B1 fill:#f0f8ff,stroke:#93c5fd,color:#000000
    style C1 fill:#dcfce7,stroke:#86efac,color:#000000
```

**This workspace:** `sqlite-vec` embedded in SQLite — driven by: local-first constraint, low data volume, zero DevOps

---

## RAG Lifecycle

### Document Preparation Phase (Offline)

```mermaid
flowchart LR
    SRC["📄 Raw Documents
    PDFs · Word · Web · Markdown"]
    --> EXT["1 · Text Extraction
    clean content
    from formatting"]
    --> CHK["2 · Chunking
    500–1000 words
    + boundary overlap"]
    --> EMB["3 · Embedding Model
    text → dense vectors
    semantic meaning encoded"]
    --> STR[("4 · Vector Database
    embeddings + original text
    + source metadata")]

    style SRC fill:#dbeafe,stroke:#3b82f6,color:#000000
    style STR fill:#dcfce7,stroke:#22c55e,color:#000000
```

**This workspace pipeline:**

```mermaid
flowchart LR
    MD["*.md files <br/> ~/notes"]
    YAML["gray_matter<br/> YAML front matter <br/> title · tags · type"]
    Clean["clean_text() <br/> lowercase <br/> strip punct · whitespace"]
    Spell["symspell <br/> word-level <br/>  spell correction"]
    Split["text-splitter <br/> ≤ 512 chars / chunk <br/> Vec&lt;String&gt;"]
    Embed_Model["Foundry Local <br/> qwen3-embedding-0.6b <br/> Vec&lt;f32&gt;[1024]"]
    DB[("🗄️ SQLite DB (~/.sb/sb.db) <br/> documents + chunks <br/> vec_chunks + fts_chunks")]

    MD ==> YAML ==> Clean ==> Spell ==> Split ==> Embed_Model ==> DB
    Split ==> DB

    style MD fill:#dbeafe,stroke:#3b82f6,color:#000000
    style DB fill:#dcfce7,stroke:#22c55e,color:#000000
```

---

### Query Processing Phase (Real-time)

```mermaid
flowchart LR
    Q["🔍 User Query"]
    --> QE["Query Embedding
    same model as documents
    ensures vector alignment"]
    --> KNN["Vector DB
    KNN similarity search
    top 3–10 chunks"]
    --> ASM["Context Assembly
    rank by relevance
    filter by metadata"]
    --> LLM["LLM
    query + retrieved context
    + answer instructions"]
    --> PP["Post-processing
    citations · formatting
    source attribution"]
    --> ANS["✅ Answer"]

    style Q fill:#dbeafe,stroke:#3b82f6,color:#000000
    style ANS fill:#dcfce7,stroke:#22c55e,color:#000000
```

**This workspace — two query modes:**

```mermaid
graph LR
    Q["🔍 User Query"] --> BM["sb search
    BM25 keyword
    ⚡ no model · offline"]
    Q --> VS["sb vsearch
    KNN vector
    🤖 Foundry Local required"]

    BM --> FTS["fts5 virtual table
    prefix OR terms
    bm25(title×2, tags×1.5, body×1)"]
    VS --> QE["embed_one(query)
    Vec&lt;f32&gt;[1024]"]
    QE --> KNN["vec_chunks KNN
    WHERE embedding MATCH k"]

    FTS --> S1["score = abs(bm25) / (1+abs)"]
    KNN --> S2["score = 1.0 / (1.0+distance)"]

    S1 --> OUT["SearchResult
    docid · score · file: sb://collection/path"]
    S2 --> OUT
```

---

## System Architecture

### System Context

```mermaid
C4Context
    title System Context — second-brain-cli

    Person(developer, "User / Developer", "Connects an AI assistant")
    System_Ext(ai, "AI agent (Claude Desktop, GitHub Copilot)", "MCP client; keyword_search / semantic_search / get_document")
    System(mcp, "sb-mcp-server", "HTTP + MCP adapter; proxies search requests to sb")
 
    Person(user, "User / Developer", "Runs CLI commands")
    System(sb, "sb CLI", "Indexes markdown notes; executes BM25 and vector search")
    System_Ext(foundry, "Microsoft Foundry Local", "On-device model runtime; qwen3-embedding-0.6b via WinML")
    System_Ext(sqlite, "SQLite + sqlite-vec + fts5", "Single-file DB: documents, BM25 index, vector index")
    System_Ext(fs, "Local Filesystem", "Markdown notes (*.md) with optional YAML front matter")

    Rel(developer, ai, "Query")
    Rel(ai, mcp, "POST /keyword_search, /semantic_search, /get")
    Rel(user, sb, "sb index / search / vsearch / get / collection")
    Rel(sb, foundry, "Generate embeddings via OpenAI-compatible SDK")
    Rel(sb, sqlite, "Read / write documents, chunks, embeddings")
    Rel(sb, fs, "Walk directories; read .md files")
    Rel(mcp, sb, "Spawns sb as subprocess")

```

### Deployment

```mermaid
graph LR
    subgraph Machine["User's Machine (Windows)"]
        subgraph Processes["Processes"]
            CLI["sb (CLI)\nRust binary"]
            MCP["sb-mcp-server\nport 3000"]
        end
        subgraph Storage["Local Storage"]
            DB[("~/.sb/sb.db\nSQLite")]
            DICT["~/.sb/en-80k.txt\nSymSpell dict"]
            NOTES["~/notes/*.md"]
        end
        subgraph WinML["WinML Runtime"]
            FL["Foundry Local\ndaemon"]
            MODEL["qwen3-embedding-0.6b\n1024-dim"]
        end

        CLI -->|reads| NOTES
        CLI -->|reads/writes| DB
        CLI -->|reads| DICT
        CLI -->|HTTP local| FL --> MODEL
        MCP -->|subprocess| CLI
    end

    AI["AI Assistant\nClaude Desktop · VS Code"] -->|MCP localhost:3000| MCP
```

### Component Architecture

```mermaid
graph TD
    subgraph CLI["sb binary"]
        A[clap CLI Parser] --> B{Command Router}
        B --> C[Collection Manager]
        B --> D[Indexing Pipeline]
        B --> E[BM25 Search]
        B --> F[Vector Search]
        B --> G[Document Retrieval]
    end

    subgraph IP["Indexing Pipeline detail"]
        D1[Directory Walker] --> D2[Exclusion Filter]
        D2 --> D3[YAML Parser]
        D3 --> D4[Text Cleaner]
        D4 --> D5[Spell Corrector]
        D5 --> D6[Text Chunker]
        D6 --> D7[DB Writer]
        D6 --> D8[Embedding Client]
        D8 --> D9[Vec DB Writer]
    end

    subgraph DB["SQLite ~/.sb/sb.db"]
        S1[(collections)]
        S2[(documents)]
        S3[(chunks)]
        S4[(vec_chunks\nsqlite-vec virtual)]
        S5[(fts_chunks\nfts5 virtual)]
    end

    D --> IP
    D7 --> S2 & S3 & S5
    D9 --> S4
    C --> S1
    E --> S5 & S3
    F --> S4 & S3
    G --> S2 & S3
```

---

## Database Schema

```mermaid
erDiagram
    collections {
        TEXT name PK
        TEXT path
        TEXT created_at_utc
        TEXT last_indexed_at_utc
    }
    documents {
        TEXT id PK
        TEXT collection FK
        TEXT path
        TEXT title
        TEXT tags
        TEXT doc_type
    }
    chunks {
        INTEGER id PK
        TEXT document_id FK
        INTEGER chunk_index
        TEXT chunk_text
    }
    vec_chunks {
        INTEGER rowid FK
        FLOAT_ARRAY embedding
    }
    fts_chunks {
        INTEGER rowid FK
        TEXT title
        TEXT tags
        TEXT chunk_text
    }

    collections ||--o{ documents : "has"
    documents ||--o{ chunks : "split into"
    chunks ||--|| vec_chunks : "embedding"
    chunks ||--|| fts_chunks : "BM25 index"
```

> `vec_chunks.rowid` and `fts_chunks.rowid` align to `chunks.id` — single integer join key across both indices.

---

## Technology Stack

| Layer            | Technology                             | Note                                       |
| ---------------- | -------------------------------------- | ------------------------------------------ |
| Language         | Rust                                   | Memory safety · native binary · no runtime |
| CLI              | `clap`                                 | Type-safe derive-based arg parsing         |
| Database         | SQLite + `rusqlite`                    | Zero-dependency single file                |
| Full-text search | SQLite fts5                            | BM25 · co-located with vectors             |
| Vector search    | `sqlite-vec`                           | Embedded KNN · no separate process         |
| Embeddings       | Foundry Local + `qwen3-embedding-0.6b` | On-device WinML · 1024-dim · no API key    |
| Chunking         | `text-splitter`                        | ≤ 512 chars per chunk                      |
| Spell correction | `symspell`                             | Symmetric-delete algorithm                 |
| Front matter     | `gray_matter`                          | YAML extraction                            |
| Async            | `tokio`                                | Required by Foundry Local SDK              |
| MCP transport    | Axum + `rmcp` (`sb-mcp-server`)        | HTTP gateway · spawns `sb` subprocess      |

---

## Next Steps

- **Model caching** — keep embedding model loaded in `sb-mcp-server` between requests
- **Dict caching** — keep `SymSpell dict` loaded in `sb-mcp-server` between requests
- **Hybrid re-ranking** — combine BM25 + vector scores (reciprocal rank fusion)
- **Cross-platform** — abstract Foundry Local behind a provider trait (Linux/macOS via llama.cpp or ONNX)
