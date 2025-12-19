use clap::{Parser, Subcommand, ValueEnum};

/// Storage type for namespace organization
#[derive(Debug, Clone, ValueEnum)]
pub enum StorageType {
    /// Agent memory storage for AI agent state
    AgentMemory,
    /// Recipe storage for reusable workflows
    Recipes,
}

impl StorageType {
    pub fn as_str(&self) -> &str {
        match self {
            StorageType::AgentMemory => "AGENT_MEMORY",
            StorageType::Recipes => "RECIPES",
        }
    }
}

#[derive(Parser)]
#[command(name = "surrealmcp")]
#[command(about = "SurrealDB MCP Server")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the MCP server
    Start {
        /// The SurrealDB endpoint URL to connect to
        #[arg(short, long, env = "SURREALDB_URL")]
        endpoint: Option<String>,
        /// Storage type for namespace (AGENT_MEMORY, RECIPES, etc.)
        #[arg(long, env = "SURREALDB_STORAGE_TYPE", value_enum)]
        storage_type: Option<StorageType>,
        /// Project name that maps to the database name
        #[arg(long, env = "SURREALDB_PROJECT_NAME")]
        project_name: Option<String>,
        /// The SurrealDB namespace to use (deprecated: use --storage-type instead)
        #[arg(long, env = "SURREALDB_NS")]
        ns: Option<String>,
        /// The SurrealDB database to use (deprecated: use --project-name instead)
        #[arg(long, env = "SURREALDB_DB")]
        db: Option<String>,
        /// The SurrealDB username to use
        #[arg(short, long, env = "SURREALDB_USER")]
        user: Option<String>,
        /// The SurrealDB password to use
        #[arg(short, long, env = "SURREALDB_PASS")]
        pass: Option<String>,
        /// The MCP server bind address (host:port)
        #[arg(long, env = "SURREAL_MCP_BIND_ADDRESS", group = "server")]
        bind_address: Option<String>,
        /// The MCP server Unix socket path
        #[arg(long, env = "SURREAL_MCP_SOCKET_PATH", group = "server")]
        socket_path: Option<String>,
        /// Rate limit requests per second (default: 100)
        #[arg(long, env = "SURREAL_MCP_RATE_LIMIT_RPS", default_value = "100")]
        rate_limit_rps: u32,
        /// Rate limit burst size (default: 200)
        #[arg(long, env = "SURREAL_MCP_RATE_LIMIT_BURST", default_value = "200")]
        rate_limit_burst: u32,
        /// Whether to require authentication for the MCP server
        #[arg(long, env = "SURREAL_MCP_AUTH_DISABLED", default_value = "false")]
        auth_disabled: bool,
        /// The URL address that the MCP server is accessible at
        #[arg(
            long,
            env = "SURREAL_MCP_SERVER_URL",
            default_value = "https://mcp.surrealdb.com"
        )]
        server_url: String,
        /// The SurrealDB Cloud authentication server URL
        #[arg(
            long,
            env = "SURREAL_MCP_AUTH_SERVER",
            default_value = "https://auth.surrealdb.com"
        )]
        auth_server: String,
        /// The expected audience for authentication tokens
        #[arg(
            long,
            env = "SURREAL_MCP_AUTH_AUDIENCE",
            default_value = "https://mcp.surrealdb.com/"
        )]
        auth_audience: String,
        /// SurrealDB Cloud access token (used instead of fetching tokens)
        #[arg(long, env = "SURREAL_MCP_CLOUD_ACCESS_TOKEN")]
        cloud_access_token: Option<String>,
        /// SurrealDB Cloud refresh token (used instead of fetching tokens)
        #[arg(long, env = "SURREAL_MCP_CLOUD_REFRESH_TOKEN")]
        cloud_refresh_token: Option<String>,
        /// Enable SurrealDB Cloud tools (default: false to prevent accidental calls)
        #[arg(long, env = "SURREAL_MCP_ENABLE_CLOUD_TOOLS", default_value = "false")]
        enable_cloud_tools: bool,
    },
}
