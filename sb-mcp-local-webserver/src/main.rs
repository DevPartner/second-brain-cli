use axum::{
    extract::Json,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{Implementation, ServerCapabilities, ServerInfo},
    schemars,
    tool, tool_handler, tool_router,
};
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::process::Command;

// ── Shared search service ─────────────────────────────────────────────────────

fn default_top_k() -> usize {
    5
}

fn sb_cmd() -> (String, Vec<String>) {
    let s = std::env::var("SB_CMD").unwrap_or_else(|_| "sb".into());
    let mut parts: Vec<String> = s.split_whitespace().map(String::from).collect();
    if parts.is_empty() {
        parts.push("sb".into());
    }
    let prog = parts.remove(0);
    (prog, parts)
}

async fn execute_search(
    subcommand: &str,
    query: &str,
    collection: &str,
    top_k: usize,
) -> Result<serde_json::Value, String> {
    let (prog, prefix) = sb_cmd();
    println!("Executing: {subcommand} \"{query}\" --json -n {top_k}");
    let mut cmd = Command::new(&prog);
    cmd.args(&prefix)
        .arg(subcommand)
        .arg(query)
        .arg("--json")
        .arg("-n")
        .arg(top_k.to_string());
    if !collection.is_empty() {
        cmd.arg("-c").arg(collection);
    }
    let output = cmd.output().await.map_err(|e| format!("sb launch failed: {e}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    serde_json::from_slice(&output.stdout).map_err(|e| format!("JSON parse error: {e}"))
}

async fn execute_get(id: &str, collection: &str) -> Result<serde_json::Value, String> {
    let (prog, prefix) = sb_cmd();
    println!("Executing: get \"{id}\"");
    let mut cmd = Command::new(&prog);
    cmd.args(&prefix).arg("get").arg(id);
    if !collection.is_empty() {
        cmd.arg("-c").arg(collection);
    }
    let output = cmd.output().await.map_err(|e| format!("sb launch failed: {e}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    let raw = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(serde_json::from_str(&raw).unwrap_or(serde_json::Value::String(raw)))
}

// ── MCP tool input type ───────────────────────────────────────────────────────

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchInput {
    /// Search query text
    pub query: String,
    /// Optional collection name to search within
    #[serde(default)]
    pub collection: Option<String>,
    /// Number of results to return
    #[serde(default = "default_top_k")]
    pub top_k: usize,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetInput {
    /// Document ID to retrieve
    pub id: String,
    /// Optional collection name
    #[serde(default)]
    pub collection: Option<String>,
}

// ── MCP server handler ────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct SbMcpServer {
    #[allow(dead_code)]
    tool_router: ToolRouter<SbMcpServer>,
}

#[tool_router]
impl SbMcpServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Search documents using keyword matching.")]
    async fn keyword_search(
        &self,
        Parameters(input): Parameters<SearchInput>,
    ) -> String {
        let collection = input.collection.as_deref().unwrap_or("");
        println!("MCP tool call:\n  name=keyword_search\n  query={:?}\n  top_k={}", input.query, input.top_k);
        match execute_search("search", &input.query, collection, input.top_k).await {
            Ok(results) => serde_json::to_string_pretty(&results).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Search documents using semantic vector search.")]
    async fn semantic_search(
        &self,
        Parameters(input): Parameters<SearchInput>,
    ) -> String {
        let collection = input.collection.as_deref().unwrap_or("");
        println!("MCP tool call:\n  name=semantic_search\n  query={:?}\n  top_k={}", input.query, input.top_k);
        match execute_search("vsearch", &input.query, collection, input.top_k).await {
            Ok(results) => serde_json::to_string_pretty(&results).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Retrieve a document by its ID.")]
    async fn get_document(
        &self,
        Parameters(input): Parameters<GetInput>,
    ) -> String {
        let collection = input.collection.as_deref().unwrap_or("");
        println!("MCP tool call:\n  name=get_document\n  id={:?}", input.id);
        match execute_get(&input.id, collection).await {
            Ok(result) => serde_json::to_string_pretty(&result).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    // #[tool(description = "Search documents using combined keyword and semantic search.")]
    // async fn hybrid_search(
    //     &self,
    //     Parameters(input): Parameters<SearchInput>,
    // ) -> String {
    //     let collection = input.collection.as_deref().unwrap_or("");
    //     println!("MCP tool call:\n  name=hybrid_search\n  query={:?}\n  top_k={}", input.query, input.top_k);
    //     match execute_search("query", &input.query, collection, input.top_k).await {
    //         Ok(results) => serde_json::to_string_pretty(&results).unwrap_or_default(),
    //         Err(e) => format!("Error: {e}"),
    //     }
    // }
}

#[tool_handler]
impl ServerHandler for SbMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder().enable_tools().build(),
        )
        .with_server_info(Implementation::from_build_env())
    }
}

// ── REST types ────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct SearchRequest {
    query: String,
    #[serde(default)]
    collection: String,
    #[serde(default = "default_top_k")]
    top_k: usize,
}

#[derive(Deserialize)]
struct GetRequest {
    id: String,
    #[serde(default)]
    collection: String,
}

#[derive(Serialize)]
struct SearchResponse {
    results: serde_json::Value,
}

// ── REST handlers ─────────────────────────────────────────────────────────────

async fn run_search_rest(
    subcommand: &str,
    req: SearchRequest,
) -> Result<Json<SearchResponse>, (StatusCode, String)> {
    let results = execute_search(subcommand, &req.query, &req.collection, req.top_k)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok(Json(SearchResponse { results }))
}

async fn keyword_search_rest(
    Json(req): Json<SearchRequest>,
) -> Result<Json<SearchResponse>, (StatusCode, String)> {
    run_search_rest("search", req).await
}

async fn semantic_search_rest(
    Json(req): Json<SearchRequest>,
) -> Result<Json<SearchResponse>, (StatusCode, String)> {
    run_search_rest("vsearch", req).await
}

async fn get_document_rest(
    Json(req): Json<GetRequest>,
) -> Result<Json<SearchResponse>, (StatusCode, String)> {
    let results = execute_get(&req.id, &req.collection)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok(Json(SearchResponse { results }))
}

// async fn hybrid_search_rest(
//     Json(req): Json<SearchRequest>,
// ) -> Result<Json<SearchResponse>, (StatusCode, String)> {
//     run_search_rest("query", req).await
// }

async fn health() -> &'static str {
    "sb-mcp server is running"
}

// ── Main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    match dotenvy::dotenv() {
        Ok(path) => eprintln!(".env loaded from {:?}", path),
        Err(e) => eprintln!(".env not loaded: {e}  (cwd={:?})", std::env::current_dir().unwrap()),
    }

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let mcp_service = StreamableHttpService::new(
        || Ok(SbMcpServer::new()),
        LocalSessionManager::default().into(),
        StreamableHttpServerConfig::default(),
    );

    let app = Router::new()
        .route("/", get(health))
        .route("/keyword_search", post(keyword_search_rest))
        .route("/semantic_search", post(semantic_search_rest))
        .route("/get", post(get_document_rest))
        // .route("/hybrid_search", post(hybrid_search_rest))
        .nest_service("/mcp", mcp_service);

    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("sb-mcp-server started");
    println!("HTTP port: {port}");
    println!("MCP endpoint: /mcp");
    axum::serve(listener, app).await.unwrap();
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_constructs() {
        let _ = SbMcpServer::new();
    }

    #[test]
    fn test_server_info_has_tools() {
        let server = SbMcpServer::new();
        let info = server.get_info();
        assert!(info.capabilities.tools.is_some());
    }

    #[tokio::test]
    async fn test_keyword_search_callable() {
        let server = SbMcpServer::new();
        let input = SearchInput { query: "test".into(), collection: None, top_k: 1 };
        let result = server.keyword_search(Parameters(input)).await;
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn test_semantic_search_callable() {
        let server = SbMcpServer::new();
        let input = SearchInput { query: "test".into(), collection: None, top_k: 1 };
        let result = server.semantic_search(Parameters(input)).await;
        assert!(!result.is_empty());
    }

    // #[tokio::test]
    // async fn test_hybrid_search_callable() {
    //     let server = SbMcpServer::new();
    //     let input = SearchInput { query: "test".into(), collection: None, top_k: 1 };
    //     let result = server.hybrid_search(Parameters(input)).await;
    //     assert!(!result.is_empty());
    // }
}
