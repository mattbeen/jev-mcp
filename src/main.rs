use std::sync::Arc;

use jev_mcp_rust::clients::typesafe_client::TypesafeClient;
use jev_mcp_rust::config::settings::Settings;
use jev_mcp_rust::server::JevServer;
use jev_mcp_rust::utils::http_client::HttpClientManager;
use jev_mcp_rust::enums::transport_mode::TransportMode;
use axum::Router;
use rmcp::transport::stdio;
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig,
    session::local::LocalSessionManager,
    tower::StreamableHttpService,
};
use rmcp::ServiceExt;
use std::net::SocketAddr;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::load()?;
    tracing_subscriber::registry()
        .with(EnvFilter::new(settings.log_level.clone()))
        .with(
            fmt::layer()
                .json()
                .flatten_event(true)
                .with_current_span(false)
                .with_span_list(false)
                .with_writer(std::io::stderr),
        )
        .init();
    let http = HttpClientManager::new(&settings)?;
    let client = Arc::new(TypesafeClient::new(&http, &settings));
    let server = JevServer { client: client.clone() };

    match settings.transport_mode {
        TransportMode::Stdio => {
            let service = server.serve(stdio()).await?;
            service.waiting().await?;
        }
        TransportMode::Http => {
            let client_for_factory = client.clone();
            let mcp = StreamableHttpService::new(
                move || {
                    Ok(JevServer { client: client_for_factory.clone() })
                },
                LocalSessionManager::default().into(),
                StreamableHttpServerConfig::default(),
            );
            let app = Router::new().nest_service(&settings.app_prefix, mcp);
            let addr: SocketAddr =
            format!("{}:{}", settings.service_host, settings.service_port).parse()?;
            let listener = tokio::net::TcpListener::bind(addr).await?;
            eprintln!(
                "MCP HTTP on http://{}:{}{}",
                settings.service_host, settings.service_port, settings.app_prefix
            );
            axum::serve(listener, app).await?;
        }
    }
    Ok(())
}