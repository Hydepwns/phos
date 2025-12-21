//! phos-aggregator: Log aggregator for DAppNode.
//!
//! This binary serves a web UI that aggregates and colorizes logs
//! from Docker containers, with auto-detection of Ethereum clients
//! and other supported programs.
//!
//! ## Environment Variables
//!
//! - `PHOS_BACKEND`: Backend to use ("docker", "dappnode", or "wamp", default: "docker")
//! - `PHOS_DAPPNODE_URL`: Custom URL for DAppNode Socket.IO backend (default: "http://my.dappnode:80")
//! - `PHOS_WAMP_URL`: Legacy WAMP URL (deprecated, use PHOS_DAPPNODE_URL)
//! - `PHOS_THEME`: Color theme (default: "default-dark")
//! - `PHOS_PORT`: Server port (default: 8180)
//! - `PHOS_CONTAINER_FILTER`: Optional container name filter
//! - `PHOS_MAX_LINES`: Max log lines to buffer (default: 10000)
//! - `PHOS_ALERT_WEBHOOK`: Optional Discord/Telegram webhook URL
//! - `PHOS_ALERT_CONDITIONS`: Comma-separated alert conditions (default: "error")

use anyhow::Result;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use phos::Theme;
use phos::alert::AlertCondition;
use phos::aggregator::{
    AggregatorConfig, AppState, ContainerProvider,
    DockerProvider, DappnodeProvider, SocketIOProvider, HttpProvider, create_router,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Read configuration from environment
    let backend = env::var("PHOS_BACKEND").unwrap_or_else(|_| "docker".to_string());
    let dappnode_url = env::var("PHOS_DAPPNODE_URL").ok();
    let wamp_url = env::var("PHOS_WAMP_URL").ok(); // Legacy, deprecated
    let theme_name = env::var("PHOS_THEME").unwrap_or_else(|_| "default-dark".to_string());
    let port: u16 = env::var("PHOS_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8180);
    let filter = env::var("PHOS_CONTAINER_FILTER").ok();
    let max_lines: usize = env::var("PHOS_MAX_LINES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(10000);
    let alert_webhook = env::var("PHOS_ALERT_WEBHOOK").ok();
    let alert_conditions: Vec<AlertCondition> = env::var("PHOS_ALERT_CONDITIONS")
        .unwrap_or_else(|_| "error".to_string())
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    // Get theme
    let theme = Theme::builtin(&theme_name).unwrap_or_else(Theme::default_dark);

    // Create provider based on backend selection
    let provider: Arc<dyn ContainerProvider> = match backend.to_lowercase().as_str() {
        "dappnode" => {
            println!("Using DAppNode HTTP backend");
            let mut provider = if let Some(url) = dappnode_url.clone() {
                HttpProvider::with_url(url)
            } else {
                HttpProvider::new()
            };

            if let Some(ref f) = filter {
                provider = provider.with_filter(f);
            }

            // Verify connection
            provider
                .verify_connection()
                .await
                .expect("Failed to connect to DAppNode. Is DAPPMANAGER running?");

            Arc::new(provider)
        }
        "socketio" => {
            println!("Using DAppNode Socket.IO backend (requires auth)");
            let mut provider = if let Some(url) = dappnode_url {
                SocketIOProvider::with_url(url)
            } else {
                SocketIOProvider::new()
            };

            if let Some(ref f) = filter {
                provider = provider.with_filter(f);
            }

            // Verify connection
            provider
                .verify_connection()
                .await
                .expect("Failed to connect to DAppNode. Is DAPPMANAGER running?");

            Arc::new(provider)
        }
        "wamp" => {
            // Legacy WAMP backend (deprecated)
            println!("Using DAppNode WAMP backend (deprecated, use PHOS_BACKEND=dappnode)");
            let mut provider = if let Some(url) = wamp_url {
                DappnodeProvider::with_url(url)
            } else {
                DappnodeProvider::new()
            };

            if let Some(ref f) = filter {
                provider = provider.with_filter(f);
            }

            // Verify connection
            provider
                .verify_connection()
                .await
                .expect("Failed to connect to DAppNode WAMP. Is DAPPMANAGER running?");

            Arc::new(provider)
        }
        _ => {
            println!("Using Docker backend");
            let mut provider = DockerProvider::new()
                .expect("Failed to connect to Docker socket. Is Docker running?");

            if let Some(ref f) = filter {
                provider = provider.with_filter(f);
            }

            // Verify connection
            provider
                .verify_connection()
                .await
                .expect("Failed to ping Docker daemon");

            Arc::new(provider)
        }
    };

    println!("phos-aggregator starting...");
    println!("  Backend: {}", provider.name());
    println!("  Theme: {theme_name}");
    println!("  Port: {port}");
    println!("  Max lines: {max_lines}");
    if let Some(ref f) = filter {
        println!("  Filter: {f}");
    }
    if alert_webhook.is_some() {
        println!("  Alerts: enabled ({} conditions)", alert_conditions.len());
    }

    // Create configuration
    let config = AggregatorConfig {
        theme,
        filter: None, // Filter is already applied to provider
        max_lines,
        alert_webhook,
        alert_conditions,
    };

    // Create application state
    let state = AppState::from_config(provider, config);

    // Create router
    let app = create_router(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
