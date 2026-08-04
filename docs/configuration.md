---
type: guide
date: 2026-08-04
tags: [configuration, environment, mcp]
---

# Configuration

---

## `sb` environment variables

The database defaults to `~/.sb/sb.db`. Override with `--db <path>` (global flag) or `SB_DB`.

| Variable      | Default                | Description                                                                  |
| ------------- | ---------------------- | ---------------------------------------------------------------------------- |
| `SB_DB`       | `~/.sb/sb.db`          | SQLite database path (also overridable with `--db`)                          |
| `SB_MODEL`    | `qwen3-embedding-0.6b` | Foundry Local embedding model used during indexing and `vsearch`             |
| `SB_DICT`     | `~/.sb/en-80k.txt`     | SymSpell dictionary path. Spell correction is silently disabled when absent. |
| `SB_NO_SPELL` | _(unset)_              | Set to `1` to skip spell correction (indexing, `search`, `vsearch`)          |

---

## `sb-mcp-server`

Starts an HTTP server that spawns `sb` as a subprocess to serve search requests.

```sh
sb-mcp-server
# → Listening on http://0.0.0.0:3000
# → MCP endpoint: /mcp
```

Loads environment from a `.env` file in the working directory if present.

### Environment variables

| Variable | Default | Description                                                       |
| -------- | ------- | ----------------------------------------------------------------- |
| `SB_CMD` | `sb`    | Command used to invoke `sb`. Supports prefix args: `node C:/path/sb` |
| `PORT`   | `3000`  | HTTP listen port                                                  |

### MCP client configuration

Add to your MCP client config (Claude Desktop, VS Code Copilot, etc.):

```json
{
  "mcpServers": {
    "second-brain": {
      "url": "http://localhost:3000/mcp"
    }
  }
}
```

Exposes three MCP tools: `keyword_search`, `semantic_search`, `get_document`.
