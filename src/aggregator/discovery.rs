//! Container discovery via Docker API.

use bollard::Docker;
use bollard::container::ListContainersOptions;
use std::collections::HashMap;

use crate::programs;

/// Information about a discovered container.
#[derive(Debug, Clone)]
pub struct ContainerInfo {
    /// Container ID (short form).
    pub id: String,
    /// Container name (without leading slash).
    pub name: String,
    /// Container image.
    pub image: String,
    /// Container status.
    pub status: String,
    /// Detected phos program ID (if any).
    pub program: Option<String>,
}

impl ContainerInfo {
    /// Serialize to JSON-compatible format.
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "name": self.name,
            "image": self.image,
            "status": self.status,
            "program": self.program
        })
    }
}

/// Discovers and lists Docker containers.
pub struct ContainerDiscovery {
    docker: Docker,
    filter_pattern: Option<String>,
}

impl ContainerDiscovery {
    /// Create a new container discovery instance.
    pub fn new(docker: Docker) -> Self {
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

    /// List all running containers, optionally filtered.
    pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>, bollard::errors::Error> {
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

                // Detect program from container name or image
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

    /// Check if a container name matches a glob-like pattern.
    fn matches_pattern(name: &str, pattern: &str) -> bool {
        if pattern.is_empty() {
            return true;
        }

        // Simple glob matching: *.dappnode.eth matches foo.dappnode.eth
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
        // Try to detect from container name first
        if let Some(program) = registry.detect(name) {
            return Some(program.info().id.to_string());
        }

        // Try to detect from image name
        if let Some(program) = registry.detect(image) {
            return Some(program.info().id.to_string());
        }

        // Try common DAppNode naming patterns
        // e.g., "geth.dnp.dappnode.eth" -> "geth"
        // e.g., "lighthouse-beacon.dnp.dappnode.eth" -> "lighthouse"
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

    /// Get the Docker client reference.
    pub fn docker(&self) -> &Docker {
        &self.docker
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_pattern() {
        assert!(ContainerDiscovery::matches_pattern(
            "geth.dnp.dappnode.eth",
            "*.dappnode.eth"
        ));
        assert!(ContainerDiscovery::matches_pattern(
            "lighthouse-beacon.dnp.dappnode.eth",
            "*.dappnode.eth"
        ));
        assert!(!ContainerDiscovery::matches_pattern(
            "my-container",
            "*.dappnode.eth"
        ));
        assert!(ContainerDiscovery::matches_pattern("anything", ""));
    }
}
