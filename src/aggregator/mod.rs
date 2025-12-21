//! Log aggregator module for DAppNode integration.
//!
//! This module provides functionality to:
//! - Discover Docker containers via the Docker API or DAppNode APIs
//! - Stream and colorize logs from multiple containers
//! - Serve a web UI for viewing aggregated logs
//!
//! ## Backend Selection
//!
//! The aggregator supports three backends:
//! - `docker`: Direct Docker socket access (default)
//! - `dappnode`: Socket.IO RPC to dappmanager (for DAppNode packages)
//! - `wamp`: Legacy WAMP RPC (deprecated, kept for compatibility)
//!
//! Set `PHOS_BACKEND=dappnode` to use the Socket.IO backend.

mod discovery;
mod docker_provider;
mod dappnode_provider;
mod socketio_provider;
mod html;
mod provider;
mod streamer;
mod web;

pub use discovery::{ContainerDiscovery, ContainerInfo};
pub use docker_provider::DockerProvider;
pub use dappnode_provider::DappnodeProvider;
pub use socketio_provider::SocketIOProvider;
pub use html::ansi_to_html;
pub use provider::{ContainerProvider, LogLine, LogStream, ProviderError};
pub use streamer::{AlertConfig, ColorizedLogEntry, LogStreamer};
pub use web::{AggregatorConfig, AppState, create_router};
