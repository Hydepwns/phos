//! Ethereum client programs.
//!
//! Provides Program implementations for all 15 supported Ethereum clients.

pub mod clients;
pub mod patterns;

use std::collections::HashMap;
use std::sync::Arc;

use clients::{ClientMeta, Layer, ALL_CLIENTS};

use crate::colors::Color;
use crate::program::{Program, ProgramInfo, ProgramRegistry};
use crate::rule::Rule;

/// Ethereum domain-specific colors.
/// These are colors specific to the Ethereum ecosystem.
pub mod colors {
    use crate::colors::Color;
    use std::collections::HashMap;

    /// Get Ethereum domain color definitions.
    pub fn domain_colors() -> HashMap<String, Color> {
        let mut colors = HashMap::new();

        // Core Ethereum concepts
        colors.insert("hash".to_string(), Color::hex("#88AAFF"));
        colors.insert("address".to_string(), Color::hex("#FFAA88"));
        colors.insert("slot".to_string(), Color::hex("#88FFAA"));
        colors.insert("epoch".to_string(), Color::hex("#AAFFFF"));
        colors.insert("block_number".to_string(), Color::hex("#FFAAFF"));
        colors.insert("peer_id".to_string(), Color::hex("#AAAAFF"));

        // Validator operations
        colors.insert("validator".to_string(), Color::hex("#AA88FF"));
        colors.insert("pubkey".to_string(), Color::hex("#88DDFF"));
        colors.insert("duty".to_string(), Color::hex("#FFBB55"));
        colors.insert("committee".to_string(), Color::hex("#88AADD"));

        // Consensus state
        colors.insert("finality".to_string(), Color::hex("#88FF88"));
        colors.insert("root".to_string(), Color::hex("#88DDFF"));
        colors.insert("attestation".to_string(), Color::hex("#FF88DD"));

        // MEV
        colors.insert("mev_value".to_string(), Color::hex("#FFDD55"));
        colors.insert("relay".to_string(), Color::hex("#999999"));
        colors.insert("builder".to_string(), Color::hex("#FF99BB"));

        // Status
        colors.insert("syncing".to_string(), Color::hex("#FFFF55"));

        colors
    }
}

/// An Ethereum client program with full metadata.
pub struct EthereumProgram {
    info: ProgramInfo,
    rules: Vec<Rule>,
    domain_colors: HashMap<String, Color>,
    detect_patterns: Vec<&'static str>,
    meta: &'static ClientMeta,
}

impl EthereumProgram {
    /// Create a new Ethereum program from client metadata.
    pub fn new(meta: &'static ClientMeta) -> Self {
        let info = ProgramInfo::new(
            &format!("ethereum.{}", meta.name.to_lowercase()),
            meta.name,
            meta.description,
            "ethereum",
        );

        let rules = clients::rules_for(meta.name)
            .unwrap_or_default();

        Self {
            info,
            rules,
            domain_colors: colors::domain_colors(),
            detect_patterns: meta.detect_patterns.to_vec(),
            meta,
        }
    }

    /// Get the client layer (Consensus, Execution, Full, Middleware).
    pub fn layer(&self) -> Layer {
        self.meta.layer
    }

    /// Get the implementation language.
    pub fn language(&self) -> &'static str {
        self.meta.language
    }

    /// Get the project website.
    pub fn website(&self) -> &'static str {
        self.meta.website
    }

    /// Get the brand color hex.
    pub fn brand_color(&self) -> &'static str {
        self.meta.brand_color
    }
}

impl Program for EthereumProgram {
    fn info(&self) -> &ProgramInfo {
        &self.info
    }

    fn rules(&self) -> Vec<Rule> {
        self.rules.clone()
    }

    fn domain_colors(&self) -> HashMap<String, Color> {
        self.domain_colors.clone()
    }

    fn detect_patterns(&self) -> Vec<&'static str> {
        self.detect_patterns.clone()
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
pub fn program_for(name: &str) -> Option<Arc<dyn Program>> {
    clients::meta_for(name)
        .map(|meta| Arc::new(EthereumProgram::new(meta)) as Arc<dyn Program>)
}

/// Get client metadata by name.
pub fn client_meta(name: &str) -> Option<&'static ClientMeta> {
    clients::meta_for(name)
}

/// Get brand color for an Ethereum client.
pub fn brand_color(name: &str) -> Option<&'static str> {
    clients::meta_for(name).map(|m| m.brand_color)
}

/// List all client names.
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
    fn test_list_by_category() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let ethereum_programs = registry.list_by_category("ethereum");
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
