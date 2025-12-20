//! Log aggregator module for DAppNode integration.
//!
//! This module provides functionality to:
//! - Discover Docker containers via the Docker API or DAppNode WAMP
//! - Stream and colorize logs from multiple containers
//! - Serve a web UI for viewing aggregated logs
//!
//! ## Backend Selection
//!
//! The aggregator supports two backends:
//! - `docker`: Direct Docker socket access (default)
//! - `dappnode`: WAMP RPC to DAPPMANAGER (for DAppNode packages)
//!
//! Set `PHOS_BACKEND=dappnode` to use the WAMP backend.

mod discovery;
mod docker_provider;
mod dappnode_provider;
mod html;
mod provider;
mod streamer;
mod web;

pub use discovery::{ContainerDiscovery, ContainerInfo};
pub use docker_provider::DockerProvider;
pub use dappnode_provider::DappnodeProvider;
pub use html::ansi_to_html;
pub use provider::{ContainerProvider, LogLine, LogStream, ProviderError};
pub use streamer::{AlertConfig, ColorizedLogEntry, LogStreamer};
pub use web::{AggregatorConfig, AppState, create_router};
