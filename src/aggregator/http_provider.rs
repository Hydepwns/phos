//! DAppNode HTTP provider using public endpoints.
//!
//! This provider uses DAPPMANAGER's public HTTP endpoints that don't require
//! authentication. For container listing it uses `/public-packages`.
//!
//! Note: Log streaming is not available without Docker socket access or
//! dappmanager authentication. This provider shows container status only.

use async_trait::async_trait;
use futures::stream;
use serde::Deserialize;

use super::{
    discovery::matches_glob,
    provider::{ContainerProvider, LogLine, LogStream, ProviderError},
    ContainerInfo,
};
use crate::programs;

/// Default dappmanager URL for public packages endpoint.
const DEFAULT_DAPPMANAGER_URL: &str = "http://dappmanager.dappnode";

/// Container info from public-packages endpoint.
/// API returns: {"name":"geth.dnp.dappnode.eth","version":"0.1.49","state":"running","ip":"172.33.0.44"}
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct PublicPackageData {
    /// Package name (e.g., "geth.dnp.dappnode.eth")
    name: Option<String>,
    /// Package version
    version: Option<String>,
    /// Current state (running, exited, etc.)
    state: Option<String>,
    /// Container IP address
    ip: Option<String>,
}

/// Build the HTTP client with standard timeout.
fn build_http_client() -> Result<reqwest::Client, ProviderError> {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| ProviderError::Connection(format!("Failed to create HTTP client: {e}")))
}

/// DAppNode HTTP provider.
pub struct HttpProvider {
    base_url: String,
    filter_pattern: Option<String>,
    http_client: reqwest::Client,
}

impl HttpProvider {
    /// Create a new HTTP provider with default dappmanager URL.
    ///
    /// # Errors
    ///
    /// Returns `ProviderError::Connection` if the HTTP client cannot be created.
    pub fn new() -> Result<Self, ProviderError> {
        Ok(Self {
            base_url: DEFAULT_DAPPMANAGER_URL.to_string(),
            filter_pattern: None,
            http_client: build_http_client()?,
        })
    }

    /// Create with a custom base URL.
    ///
    /// # Errors
    ///
    /// Returns `ProviderError::Connection` if the HTTP client cannot be created.
    pub fn with_url(url: impl Into<String>) -> Result<Self, ProviderError> {
        Ok(Self {
            base_url: url.into(),
            filter_pattern: None,
            http_client: build_http_client()?,
        })
    }

    /// Set a name filter pattern.
    #[must_use]
    pub fn with_filter(mut self, pattern: impl Into<String>) -> Self {
        self.filter_pattern = Some(pattern.into());
        self
    }
}

#[async_trait]
impl ContainerProvider for HttpProvider {
    async fn list_containers(&self) -> Result<Vec<ContainerInfo>, ProviderError> {
        let url = format!("{}/public-packages", self.base_url);

        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| ProviderError::Connection(format!("HTTP request failed: {e}")))?;

        if !response.status().is_success() {
            return Err(ProviderError::Connection(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        let packages: Vec<PublicPackageData> = response
            .json()
            .await
            .map_err(|e| ProviderError::Rpc(format!("Failed to parse packages: {e}")))?;

        let registry = programs::default_registry();
        let pattern = self.filter_pattern.as_deref().unwrap_or("");

        let containers = packages
            .into_iter()
            .filter_map(|pkg| {
                let name = pkg.name.filter(|n| !n.is_empty())?;
                matches_glob(&name, pattern).then(|| {
                    ContainerInfo::new(
                        &registry,
                        name.clone(),
                        name,
                        pkg.version.unwrap_or_default(),
                        pkg.state.unwrap_or_else(|| "unknown".to_string()),
                    )
                })
            })
            .collect();

        Ok(containers)
    }

    async fn get_logs(
        &self,
        container_id: &str,
        _tail: usize,
        _follow: bool,
    ) -> Result<LogStream, ProviderError> {
        // DAPPMANAGER requires authentication for log access.
        // Without Docker socket or auth, we cannot stream logs.
        let info_line = LogLine {
            content: format!(
                "[phos] Log streaming not available for '{}'. \
                 DAPPMANAGER requires authentication for log access. \
                 Container status is available in the sidebar.",
                container_id
            ),
            is_stderr: false,
            timestamp: None,
        };

        let stream = stream::iter(vec![Ok(info_line)]);
        Ok(Box::pin(stream))
    }

    async fn verify_connection(&self) -> Result<(), ProviderError> {
        // Verify we can reach the public-packages endpoint
        let url = format!("{}/public-packages", self.base_url);

        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| ProviderError::Connection(format!("HTTP request failed: {e}")))?;

        if !response.status().is_success() {
            return Err(ProviderError::Connection(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        // Parse to verify format
        let _packages: Vec<PublicPackageData> = response
            .json()
            .await
            .map_err(|e| ProviderError::Rpc(format!("Failed to parse packages: {e}")))?;

        println!("  Note: Log streaming requires DAPPMANAGER authentication (not yet implemented)");

        Ok(())
    }

    fn name(&self) -> &'static str {
        "dappnode-http"
    }
}
