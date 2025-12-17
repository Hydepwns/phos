//! phos-aggregator: Log aggregator for DAppNode.
//!
//! This binary serves a web UI that aggregates and colorizes logs
//! from Docker containers, with auto-detection of Ethereum clients
//! and other supported programs.

use anyhow::Result;
use bollard::Docker;
use std::env;
use std::net::SocketAddr;

use phos::aggregator::{create_router, AppState};
use phos::Theme;

#[tokio::main]
async fn main() -> Result<()> {
    // Read configuration from environment
    let theme_name = env::var("PHOS_THEME").unwrap_or_else(|_| "default-dark".to_string());
    let port: u16 = env::var("PHOS_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let filter = env::var("PHOS_CONTAINER_FILTER").ok();

    // Get theme
    let theme = Theme::builtin(&theme_name).unwrap_or_else(Theme::default_dark);

    // Connect to Docker
    let docker = Docker::connect_with_local_defaults()
        .expect("Failed to connect to Docker socket. Is Docker running?");

    // Verify Docker connection
    docker.ping().await.expect("Failed to ping Docker daemon");

    println!("phos-aggregator starting...");
    println!("  Theme: {}", theme_name);
    println!("  Port: {}", port);
    if let Some(ref f) = filter {
        println!("  Filter: {}", f);
    }

    // Create application state
    let state = if let Some(ref f) = filter {
        AppState::with_filter(docker, theme, f)
    } else {
        AppState::new(docker, theme)
    };

    // Create router
    let app = create_router(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
