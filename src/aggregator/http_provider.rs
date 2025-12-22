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

use crate::programs;
use super::{ContainerInfo, provider::{ContainerProvider, LogLine, LogStream, ProviderError}};

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

/// DAppNode HTTP provider.
pub struct HttpProvider {
    base_url: String,
    filter_pattern: Option<String>,
    http_client: reqwest::Client,
}

impl HttpProvider {
    /// Create a new HTTP provider with default dappmanager URL.
    pub fn new() -> Self {
        Self {
            base_url: DEFAULT_DAPPMANAGER_URL.to_string(),
            filter_pattern: None,
            http_client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Create with a custom base URL.
    pub fn with_url(url: impl Into<String>) -> Self {
        Self {
            base_url: url.into(),
            filter_pattern: None,
            http_client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Set a name filter pattern.
    pub fn with_filter(mut self, pattern: impl Into<String>) -> Self {
        self.filter_pattern = Some(pattern.into());
        self
    }

    /// Check if a container name matches a glob-like pattern.
    fn matches_pattern(name: &str, pattern: &str) -> bool {
        if pattern.is_empty() {
            return true;
        }

        if let Some(suffix) = pattern.strip_prefix('*') {
            name.ends_with(suffix)
        } else if let Some(prefix) = pattern.strip_suffix('*') {
            name.starts_with(prefix)
        } else {
            name.contains(pattern)
        }
    }

    /// Detect phos program from container name or image.
    fn detect_program(
        registry: &crate::ProgramRegistry,
        name: &str,
        image: &str,
    ) -> Option<String> {
        if let Some(program) = registry.detect(name) {
            return Some(program.info().id.to_string());
        }

        if let Some(program) = registry.detect(image) {
            return Some(program.info().id.to_string());
        }

        let base_name = name
            .split('.')
            .next()
            .unwrap_or(name)
            .replace("-beacon", "")
            .replace("-validator", "")
            .replace("-bn", "")
            .replace("-vc", "")
            .replace("DAppNodePackage-", "")
            .replace("DAppNodeCore-", "");

        registry.detect(&base_name).map(|p| p.info().id.to_string())
    }
}

#[async_trait]
impl ContainerProvider for HttpProvider {
    async fn list_containers(&self) -> Result<Vec<ContainerInfo>, ProviderError> {
        let url = format!("{}/public-packages", self.base_url);

        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| ProviderError::Connection(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(ProviderError::Connection(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        let packages: Vec<PublicPackageData> = response
            .json()
            .await
            .map_err(|e| ProviderError::Rpc(format!("Failed to parse packages: {}", e)))?;

        let registry = programs::default_registry();
        let mut containers = Vec::new();

        for pkg in packages {
            let name = pkg.name.unwrap_or_default();

            if name.is_empty() {
                continue;
            }

            if let Some(ref pattern) = self.filter_pattern {
                if !Self::matches_pattern(&name, pattern) {
                    continue;
                }
            }

            // Generate a display name from package name (e.g., "geth.dnp.dappnode.eth" -> "geth")
            let display_name = name.split('.').next().unwrap_or(&name).to_string();
            let program = Self::detect_program(&registry, &name, &display_name);

            containers.push(ContainerInfo {
                id: name.clone(),
                name,
                image: pkg.version.unwrap_or_default(),
                status: pkg.state.unwrap_or_else(|| "unknown".to_string()),
                program,
            });
        }

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

        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| ProviderError::Connection(format!("HTTP request failed: {}", e)))?;

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
            .map_err(|e| ProviderError::Rpc(format!("Failed to parse packages: {}", e)))?;

        println!("  Note: Log streaming requires DAPPMANAGER authentication (not yet implemented)");

        Ok(())
    }

    fn name(&self) -> &'static str {
        "dappnode-http"
    }
}

impl Default for HttpProvider {
    fn default() -> Self {
        Self::new()
    }
}
