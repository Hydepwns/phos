//! Log aggregator module for DAppNode integration.
//!
//! This module provides functionality to:
//! - Discover Docker containers via the Docker API
//! - Stream and colorize logs from multiple containers
//! - Serve a web UI for viewing aggregated logs

mod discovery;
mod html;
mod streamer;
mod web;

pub use discovery::{ContainerDiscovery, ContainerInfo};
pub use html::ansi_to_html;
pub use streamer::{ColorizedLogEntry, LogStreamer};
pub use web::{AppState, create_router};
