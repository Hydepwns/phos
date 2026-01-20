//! Docker provider using bollard (direct Docker socket access).

use async_trait::async_trait;
use bollard::container::{ListContainersOptions, LogsOptions};
use bollard::Docker;
use futures::StreamExt;
use std::collections::HashMap;

use super::{
    discovery::matches_glob,
    provider::{ContainerProvider, LogLine, LogStream, ProviderError},
    ContainerInfo,
};
use crate::programs;

/// Docker provider using direct socket access via bollard.
pub struct DockerProvider {
    docker: Docker,
    filter_pattern: Option<String>,
}

impl DockerProvider {
    /// Create a new Docker provider with default connection.
    pub fn new() -> Result<Self, ProviderError> {
        let docker = Docker::connect_with_local_defaults()
            .map_err(|e| ProviderError::Connection(e.to_string()))?;
        Ok(Self {
            docker,
            filter_pattern: None,
        })
    }

    /// Create from an existing Docker client.
    pub fn from_client(docker: Docker) -> Self {
        Self {
            docker,
            filter_pattern: None,
        }
    }

    /// Set a name filter pattern (e.g., "*.dappnode.eth").
    pub fn with_filter(mut self, pattern: impl Into<String>) -> Self {
        self.filter_pattern = Some(pattern.into());
        self
    }

    /// Get a reference to the Docker client.
    pub fn docker(&self) -> &Docker {
        &self.docker
    }
}

#[async_trait]
impl ContainerProvider for DockerProvider {
    async fn list_containers(&self) -> Result<Vec<ContainerInfo>, ProviderError> {
        let mut filters = HashMap::new();
        filters.insert("status", vec!["running"]);

        let options = ListContainersOptions {
            filters,
            ..Default::default()
        };

        let containers = self.docker.list_containers(Some(options)).await?;
        let registry = programs::default_registry();
        let pattern = self.filter_pattern.as_deref().unwrap_or("");

        let result = containers
            .into_iter()
            .filter_map(|c| {
                let id: String = c.id?.chars().take(12).collect();
                let name = c.names?.first()?.trim_start_matches('/').to_string();

                matches_glob(&name, pattern).then(|| {
                    ContainerInfo::new(
                        &registry,
                        id,
                        name,
                        c.image.unwrap_or_default(),
                        c.status.unwrap_or_default(),
                    )
                })
            })
            .collect();

        Ok(result)
    }

    async fn get_logs(
        &self,
        container_id: &str,
        tail: usize,
        follow: bool,
    ) -> Result<LogStream, ProviderError> {
        let options = LogsOptions::<String> {
            follow,
            stdout: true,
            stderr: true,
            timestamps: true,
            tail: if tail > 0 {
                tail.to_string()
            } else {
                "all".to_string()
            },
            ..Default::default()
        };

        let stream = self.docker.logs(container_id, Some(options));

        let mapped = stream.map(|result| {
            result
                .map(|log_output| {
                    let content = log_output.to_string();
                    let is_stderr =
                        matches!(log_output, bollard::container::LogOutput::StdErr { .. });
                    LogLine {
                        content,
                        is_stderr,
                        timestamp: None, // Timestamp is embedded in content when timestamps=true
                    }
                })
                .map_err(ProviderError::from)
        });

        Ok(Box::pin(mapped))
    }

    async fn verify_connection(&self) -> Result<(), ProviderError> {
        self.docker
            .ping()
            .await
            .map_err(|e| ProviderError::Connection(format!("Docker ping failed: {}", e)))?;
        Ok(())
    }

    fn name(&self) -> &'static str {
        "docker"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_glob_via_discovery() {
        // Tests use the shared matches_glob function from discovery module
        assert!(matches_glob("geth.dnp.dappnode.eth", "*.dappnode.eth"));
        assert!(matches_glob(
            "lighthouse-beacon.dnp.dappnode.eth",
            "*.dappnode.eth"
        ));
        assert!(!matches_glob("my-container", "*.dappnode.eth"));
        assert!(matches_glob("anything", ""));
    }
}
