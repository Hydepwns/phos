//! Docker provider using bollard (direct Docker socket access).

use async_trait::async_trait;
use bollard::container::{ListContainersOptions, LogsOptions};
use bollard::Docker;
use futures::StreamExt;
use std::collections::HashMap;

use super::{
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

        // Try common DAppNode naming patterns
        let base_name = name
            .split('.')
            .next()
            .unwrap_or(name)
            .replace("-beacon", "")
            .replace("-validator", "")
            .replace("-bn", "")
            .replace("-vc", "");

        registry.detect(&base_name).map(|p| p.info().id.to_string())
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

        let result: Vec<ContainerInfo> = containers
            .into_iter()
            .filter_map(|c| {
                let id = c.id?.chars().take(12).collect();
                let name = c.names?.first()?.trim_start_matches('/').to_string();

                // Apply filter if set
                if let Some(ref pattern) = self.filter_pattern {
                    if !Self::matches_pattern(&name, pattern) {
                        return None;
                    }
                }

                let image = c.image.unwrap_or_default();
                let status = c.status.unwrap_or_default();
                let program = Self::detect_program(&registry, &name, &image);

                Some(ContainerInfo {
                    id,
                    name,
                    image,
                    status,
                    program,
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
    fn test_matches_pattern() {
        assert!(DockerProvider::matches_pattern(
            "geth.dnp.dappnode.eth",
            "*.dappnode.eth"
        ));
        assert!(DockerProvider::matches_pattern(
            "lighthouse-beacon.dnp.dappnode.eth",
            "*.dappnode.eth"
        ));
        assert!(!DockerProvider::matches_pattern(
            "my-container",
            "*.dappnode.eth"
        ));
        assert!(DockerProvider::matches_pattern("anything", ""));
    }
}
