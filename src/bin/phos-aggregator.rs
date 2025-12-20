//! phos-aggregator: Log aggregator for DAppNode.
//!
//! This binary serves a web UI that aggregates and colorizes logs
//! from Docker containers, with auto-detection of Ethereum clients
//! and other supported programs.
//!
//! ## Environment Variables
//!
//! - `PHOS_THEME`: Color theme (default: "default-dark")
//! - `PHOS_PORT`: Server port (default: 8080)
//! - `PHOS_CONTAINER_FILTER`: Optional container name filter
//! - `PHOS_MAX_LINES`: Max log lines to buffer (default: 10000)
//! - `PHOS_ALERT_WEBHOOK`: Optional Discord/Telegram webhook URL
//! - `PHOS_ALERT_CONDITIONS`: Comma-separated alert conditions (default: "error")

use anyhow::Result;
use bollard::Docker;
use std::env;
use std::net::SocketAddr;

use phos::Theme;
use phos::alert::AlertCondition;
use phos::aggregator::{AggregatorConfig, AppState, create_router};

#[tokio::main]
async fn main() -> Result<()> {
    // Read configuration from environment
    let theme_name = env::var("PHOS_THEME").unwrap_or_else(|_| "default-dark".to_string());
    let port: u16 = env::var("PHOS_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
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

    // Connect to Docker
    let docker = Docker::connect_with_local_defaults()
        .expect("Failed to connect to Docker socket. Is Docker running?");

    // Verify Docker connection
    docker.ping().await.expect("Failed to ping Docker daemon");

    println!("phos-aggregator starting...");
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
        filter,
        max_lines,
        alert_webhook,
        alert_conditions,
    };

    // Create application state
    let state = AppState::from_config(docker, config);

    // Create router
    let app = create_router(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
