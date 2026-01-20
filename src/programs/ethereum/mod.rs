//! Ethereum client programs.
//!
//! Provides Program implementations for all 15 supported Ethereum clients.

#[macro_use]
mod macros;

pub mod clients;
pub mod patterns;

use std::collections::HashMap;
use std::sync::Arc;

use clients::{ClientMeta, Layer, ALL_CLIENTS};

use crate::category::Category;
use crate::colors::Color;
use crate::program::{Program, ProgramInfo, ProgramRegistry};
use crate::rule::Rule;

/// Ethereum domain-specific colors.
/// These are colors specific to the Ethereum ecosystem.
pub mod colors {
    use crate::colors::Color;
    use std::collections::HashMap;

    /// Color definitions for Ethereum domain concepts.
    const DOMAIN_COLOR_DEFS: &[(&str, &str)] = &[
        // Core Ethereum concepts
        ("hash", "#88AAFF"),
        ("address", "#FFAA88"),
        ("slot", "#88FFAA"),
        ("epoch", "#AAFFFF"),
        ("block_number", "#FFAAFF"),
        ("peer_id", "#AAAAFF"),
        // Validator operations
        ("validator", "#AA88FF"),
        ("pubkey", "#88DDFF"),
        ("duty", "#FFBB55"),
        ("committee", "#88AADD"),
        // Consensus state
        ("finality", "#88FF88"),
        ("root", "#88DDFF"),
        ("attestation", "#FF88DD"),
        // MEV
        ("mev_value", "#FFDD55"),
        ("relay", "#999999"),
        ("builder", "#FF99BB"),
        // Status
        ("syncing", "#FFFF55"),
    ];

    /// Get Ethereum domain color definitions.
    #[must_use]
    pub fn domain_colors() -> HashMap<String, Color> {
        DOMAIN_COLOR_DEFS
            .iter()
            .map(|(name, hex)| ((*name).to_string(), Color::hex(hex)))
            .collect()
    }
}

/// An Ethereum client program with full metadata.
pub struct EthereumProgram {
    info: ProgramInfo,
    rules: Arc<[Rule]>,
    domain_colors: HashMap<String, Color>,
    detect_patterns: Vec<&'static str>,
    meta: &'static ClientMeta,
}

impl EthereumProgram {
    /// Create a new Ethereum program from client metadata.
    #[must_use]
    pub fn new(meta: &'static ClientMeta) -> Self {
        let info = ProgramInfo::new(
            &format!("ethereum.{}", meta.name.to_lowercase()),
            meta.name,
            meta.description,
            Category::Ethereum,
        );

        let rules: Arc<[Rule]> = clients::rules_for(meta.name)
            .map(|result| {
                result.unwrap_or_else(|e| {
                    eprintln!("phos: rule compilation error for {}: {e}", meta.name);
                    Vec::new()
                })
            })
            .unwrap_or_default()
            .into();

        Self {
            info,
            rules,
            domain_colors: colors::domain_colors(),
            detect_patterns: meta.detect_patterns.to_vec(),
            meta,
        }
    }

    /// Get the client layer (Consensus, Execution, Full, Middleware).
    #[must_use]
    pub fn layer(&self) -> Layer {
        self.meta.layer
    }

    /// Get the implementation language.
    #[must_use]
    pub fn language(&self) -> &'static str {
        self.meta.language
    }

    /// Get the project website.
    #[must_use]
    pub fn website(&self) -> &'static str {
        self.meta.website
    }

    /// Get the brand color hex.
    #[must_use]
    pub fn brand_color(&self) -> &'static str {
        self.meta.brand_color
    }
}

impl Program for EthereumProgram {
    fn info(&self) -> &ProgramInfo {
        &self.info
    }

    fn rules(&self) -> Arc<[Rule]> {
        Arc::clone(&self.rules)
    }

    fn domain_colors(&self) -> HashMap<String, Color> {
        self.domain_colors.clone()
    }

    fn detect_patterns(&self) -> &[&str] {
        &self.detect_patterns
    }
}

/// Register all Ethereum clients as programs.
pub fn register_all(registry: &mut ProgramRegistry) {
    for meta in ALL_CLIENTS {
        let program = Arc::new(EthereumProgram::new(meta));
        registry.register(program);
    }
}

/// Get an Ethereum program by client name.
#[must_use]
pub fn program_for(name: &str) -> Option<Arc<dyn Program>> {
    clients::meta_for(name).map(|meta| Arc::new(EthereumProgram::new(meta)) as Arc<dyn Program>)
}

/// Get client metadata by name.
#[must_use]
pub fn client_meta(name: &str) -> Option<&'static ClientMeta> {
    clients::meta_for(name)
}

/// Get brand color for an Ethereum client.
#[must_use]
pub fn brand_color(name: &str) -> Option<&'static str> {
    clients::meta_for(name).map(|m| m.brand_color)
}

/// List all client names.
#[must_use]
pub fn all_client_names() -> Vec<&'static str> {
    ALL_CLIENTS.iter().map(|m| m.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_clients_registered() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);
        assert_eq!(registry.len(), 15);
    }

    #[test]
    fn test_get_by_client_name() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        assert!(registry.get("ethereum.lodestar").is_some());
        assert!(registry.get("lodestar").is_some());
        assert!(registry.get("geth").is_some());
    }

    #[test]
    fn test_detect_client() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("docker logs lighthouse-beacon");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "Lighthouse");
    }

    #[test]
    fn test_detect_eth_docker_patterns() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        // Test eth-docker beacon node patterns
        let detected = registry.detect("docker logs eth-docker-lighthouse-bn-1");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "Lighthouse");

        // Test eth-docker validator client patterns
        let detected = registry.detect("docker logs eth-docker-prysm-vc-1");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "Prysm");

        // Test eth-docker execution layer patterns
        let detected = registry.detect("docker logs eth-docker-geth-el-1");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "Geth");

        // Test underscore variants
        let detected = registry.detect("docker logs lodestar_beacon");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "Lodestar");

        // Test log file patterns
        let detected = registry.detect("tail -f /var/log/nethermind.log");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "Nethermind");
    }

    #[test]
    fn test_list_by_category() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let ethereum_programs = registry.list_by_category(Category::Ethereum);
        assert_eq!(ethereum_programs.len(), 15);
    }

    #[test]
    fn test_domain_colors() {
        let program = EthereumProgram::new(&clients::LODESTAR);
        let colors = program.domain_colors();
        assert!(colors.contains_key("slot"));
        assert!(colors.contains_key("epoch"));
        assert!(colors.contains_key("hash"));
    }

    #[test]
    fn test_client_metadata() {
        let meta = client_meta("lodestar").unwrap();
        assert_eq!(meta.name, "Lodestar");
        assert_eq!(meta.layer, Layer::Consensus);
        assert_eq!(meta.language, "TypeScript");
    }

    #[test]
    fn test_brand_colors() {
        assert!(brand_color("geth").is_some());
        assert!(brand_color("unknown").is_none());
    }

    #[test]
    fn test_mana_is_full_node() {
        let meta = client_meta("mana").unwrap();
        assert_eq!(meta.layer, Layer::Full);
    }

    #[test]
    fn test_elixir_clients() {
        assert_eq!(client_meta("lambda").unwrap().language, "Elixir");
        assert_eq!(client_meta("mana").unwrap().language, "Elixir");
    }
}
