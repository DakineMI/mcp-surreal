use crate::server::ServerConfig;
use anyhow::Result;
use clap::Parser;

mod cli;
mod cloud;
mod db;
mod engine;
mod logs;
mod prompts;
mod resources;
mod server;
mod tools;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    if rustls::crypto::ring::default_provider()
        .install_default()
        .is_err()
    {
        tracing::error!("Failed to install default crypto provider");
    }

    // Parse command line arguments
    let cli = cli::Cli::parse();
    // Run the specified command
    match cli.command {
        cli::Commands::Start {
            endpoint,
            storage_type,
            project_name,
            ns,
            db,
            user,
            pass,
            server_url,
            bind_address,
            socket_path,
            auth_disabled,
            rate_limit_rps,
            rate_limit_burst,
            auth_server,
            auth_audience,
            cloud_access_token,
            cloud_refresh_token,
            enable_cloud_tools,
        } => {
            // Determine namespace: use storage_type if provided, otherwise fall back to ns
            let namespace = storage_type.as_ref().map(|st| st.as_str().to_string()).or(ns);
            
            // Determine database: use project_name if provided, otherwise fall back to db
            let database = project_name.or(db);
            
            // Create the server config
            let config = ServerConfig {
                endpoint,
                ns: namespace,
                db: database,
                user,
                pass,
                server_url,
                bind_address,
                socket_path,
                auth_disabled,
                rate_limit_rps,
                rate_limit_burst,
                auth_server,
                auth_audience,
                cloud_access_token,
                cloud_refresh_token,
                enable_cloud_tools,
            };
            server::start_server(config).await
        }
    }
}
