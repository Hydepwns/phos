//! Container provider abstraction for different backends.
//!
//! This module defines the `ContainerProvider` trait that allows phos to
//! work with different container backends:
//! - Docker API (direct socket access)
//! - DAppNode DAPPMANAGER (via WAMP RPC)

use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;

use super::ContainerInfo;

/// Error type for provider operations.
#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("Connection failed: {0}")]
    Connection(String),

    #[error("Container not found: {0}")]
    NotFound(String),

    #[error("RPC call failed: {0}")]
    Rpc(String),

    #[error("Docker error: {0}")]
    Docker(#[from] bollard::errors::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Other error: {0}")]
    Other(String),
}

/// A log line from a container.
#[derive(Debug, Clone)]
pub struct LogLine {
    /// The log content.
    pub content: String,
    /// Whether this is from stderr (vs stdout).
    pub is_stderr: bool,
    /// Timestamp if available.
    pub timestamp: Option<String>,
}

/// Stream of log lines.
pub type LogStream = Pin<Box<dyn Stream<Item = Result<LogLine, ProviderError>> + Send>>;

/// Trait for container providers.
///
/// Implementors provide access to container listings and log streams.
#[async_trait]
pub trait ContainerProvider: Send + Sync {
    /// List all available containers.
    async fn list_containers(&self) -> Result<Vec<ContainerInfo>, ProviderError>;

    /// Get a stream of logs for a container.
    ///
    /// # Arguments
    /// * `container_id` - The container ID or name
    /// * `tail` - Number of historical lines to fetch (0 = all)
    /// * `follow` - Whether to follow new logs
    async fn get_logs(
        &self,
        container_id: &str,
        tail: usize,
        follow: bool,
    ) -> Result<LogStream, ProviderError>;

    /// Verify the connection to the backend.
    async fn verify_connection(&self) -> Result<(), ProviderError>;

    /// Get the provider name for logging purposes.
    fn name(&self) -> &'static str;
}
