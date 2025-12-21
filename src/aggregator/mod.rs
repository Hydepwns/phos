//! Log aggregator module for DAppNode integration.
//!
//! This module provides functionality to:
//! - Discover Docker containers via the Docker API or DAppNode APIs
//! - Stream and colorize logs from multiple containers
//! - Serve a web UI for viewing aggregated logs
//!
//! ## Backend Selection
//!
//! The aggregator supports four backends:
//! - `docker`: Direct Docker socket access (default)
//! - `dappnode`: HTTP to dappmanager public endpoints + Docker for logs (recommended)
//! - `socketio`: Socket.IO RPC to dappmanager (requires auth, may not work)
//! - `wamp`: Legacy WAMP RPC (deprecated, kept for compatibility)
//!
//! Set `PHOS_BACKEND=dappnode` to use the HTTP backend with Docker socket for logs.

mod discovery;
mod docker_provider;
mod dappnode_provider;
mod socketio_provider;
mod http_provider;
mod html;
mod provider;
mod streamer;
mod web;

pub use discovery::{ContainerDiscovery, ContainerInfo};
pub use docker_provider::DockerProvider;
pub use dappnode_provider::DappnodeProvider;
pub use socketio_provider::SocketIOProvider;
pub use http_provider::HttpProvider;
pub use html::ansi_to_html;
pub use provider::{ContainerProvider, LogLine, LogStream, ProviderError};
pub use streamer::{AlertConfig, ColorizedLogEntry, LogStreamer};
pub use web::{AggregatorConfig, AppState, create_router};
