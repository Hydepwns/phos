//! Container discovery via Docker API.
//!
//! This module provides shared utilities for container discovery across all providers.

use bollard::container::ListContainersOptions;
use bollard::Docker;
use std::collections::HashMap;

use crate::programs;
use crate::ProgramRegistry;

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
    /// Create a new ContainerInfo, auto-detecting the program from name/image.
    ///
    /// Uses `detect_program` to identify the phos program for colorization.
    #[must_use]
    pub fn new(
        registry: &ProgramRegistry,
        id: impl Into<String>,
        name: impl Into<String>,
        image: impl Into<String>,
        status: impl Into<String>,
    ) -> Self {
        let name = name.into();
        let image = image.into();
        Self {
            program: detect_program(registry, &name, &image),
            id: id.into(),
            name,
            image,
            status: status.into(),
        }
    }

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

// =============================================================================
// Shared utilities for all providers
// =============================================================================

/// Check if a container name matches a glob-like pattern.
///
/// Supports simple glob patterns:
/// - `*suffix` matches names ending with suffix
/// - `prefix*` matches names starting with prefix
/// - `pattern` matches names containing pattern
/// - Empty pattern matches everything
#[must_use]
pub fn matches_glob(name: &str, pattern: &str) -> bool {
    pattern.is_empty()
        || pattern.strip_prefix('*').map_or_else(
            || {
                pattern
                    .strip_suffix('*')
                    .map_or_else(|| name.contains(pattern), |p| name.starts_with(p))
            },
            |s| name.ends_with(s),
        )
}

/// Normalize a container name by removing common suffixes.
///
/// Strips DAppNode-style suffixes like `-beacon`, `-validator`, `-bn`, `-vc`,
/// and prefixes like `DAppNodePackage-`, `DAppNodeCore-`.
#[must_use]
pub fn normalize_container_name(name: &str) -> String {
    name.split('.')
        .next()
        .unwrap_or(name)
        .replace("-beacon", "")
        .replace("-validator", "")
        .replace("-bn", "")
        .replace("-vc", "")
        .replace("DAppNodePackage-", "")
        .replace("DAppNodeCore-", "")
}

/// Detect phos program from container name and/or image.
///
/// Tries detection in order:
/// 1. Direct match on container name
/// 2. Direct match on image name
/// 3. Match on normalized container name (strips DAppNode suffixes)
#[must_use]
pub fn detect_program(registry: &ProgramRegistry, name: &str, image: &str) -> Option<String> {
    registry
        .detect(name)
        .or_else(|| registry.detect(image))
        .or_else(|| registry.detect(&normalize_container_name(name)))
        .map(|p| p.info().id.to_string())
}

// =============================================================================
// ContainerDiscovery struct (for direct Docker access)
// =============================================================================

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

    /// Get the Docker client reference.
    pub fn docker(&self) -> &Docker {
        &self.docker
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_glob() {
        // Suffix pattern
        assert!(matches_glob("geth.dnp.dappnode.eth", "*.dappnode.eth"));
        assert!(matches_glob(
            "lighthouse-beacon.dnp.dappnode.eth",
            "*.dappnode.eth"
        ));
        assert!(!matches_glob("my-container", "*.dappnode.eth"));

        // Prefix pattern
        assert!(matches_glob("geth-mainnet", "geth*"));
        assert!(!matches_glob("nethermind", "geth*"));

        // Contains pattern
        assert!(matches_glob("my-geth-container", "geth"));
        assert!(!matches_glob("lighthouse", "geth"));

        // Empty pattern matches everything
        assert!(matches_glob("anything", ""));
    }

    #[test]
    fn test_normalize_container_name() {
        assert_eq!(
            normalize_container_name("lighthouse-beacon.dnp.dappnode.eth"),
            "lighthouse"
        );
        assert_eq!(
            normalize_container_name("prysm-validator.dnp.dappnode.eth"),
            "prysm"
        );
        assert_eq!(normalize_container_name("geth-bn"), "geth");
        assert_eq!(normalize_container_name("lodestar-vc"), "lodestar");
        assert_eq!(normalize_container_name("DAppNodePackage-geth"), "geth");
        assert_eq!(
            normalize_container_name("DAppNodeCore-dappmanager"),
            "dappmanager"
        );
    }
}
