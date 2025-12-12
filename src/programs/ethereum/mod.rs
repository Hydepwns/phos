//! Ethereum client programs.
//!
//! Provides Program implementations for all 15 supported Ethereum clients.

use std::collections::HashMap;
use std::sync::Arc;

use crate::clients::Client;
use crate::colors::Color;
use crate::program::{Program, ProgramInfo, ProgramRegistry};
use crate::rule::Rule;

/// Ethereum domain-specific colors.
/// These are colors specific to the Ethereum ecosystem.
pub mod colors {
    use crate::colors::Color;

    /// Get Ethereum domain color definitions.
    pub fn domain_colors() -> std::collections::HashMap<String, Color> {
        let mut colors = std::collections::HashMap::new();

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

/// Wrapper that makes a Client implement the Program trait.
pub struct EthereumProgram {
    client: Client,
    info: ProgramInfo,
    domain_colors: HashMap<String, Color>,
}

impl EthereumProgram {
    /// Create a new EthereumProgram for a client.
    pub fn new(client: Client) -> Self {
        let client_info = client.info();

        let info = ProgramInfo::new(
            &format!("ethereum.{}", client_info.name.to_lowercase()),
            client_info.name,
            client_info.description,
            "ethereum",
        );

        Self {
            client,
            info,
            domain_colors: colors::domain_colors(),
        }
    }

    /// Get detection patterns for this client.
    fn detection_patterns(&self) -> Vec<&'static str> {
        match self.client {
            Client::Lighthouse => vec!["lighthouse"],
            Client::Prysm => vec!["prysm", "beacon-chain", "validator"],
            Client::Teku => vec!["teku"],
            Client::Nimbus => vec!["nimbus"],
            Client::Lodestar => vec!["lodestar"],
            Client::Grandine => vec!["grandine"],
            Client::Lambda => vec!["lambda_ethereum"],
            Client::Geth => vec!["geth"],
            Client::Nethermind => vec!["nethermind"],
            Client::Besu => vec!["besu"],
            Client::Erigon => vec!["erigon"],
            Client::Reth => vec!["reth"],
            Client::Mana => vec!["mana"],
            Client::Charon => vec!["charon"],
            Client::MevBoost => vec!["mev-boost", "mev_boost", "mevboost"],
        }
    }
}

impl Program for EthereumProgram {
    fn info(&self) -> &ProgramInfo {
        &self.info
    }

    fn rules(&self) -> Vec<Rule> {
        self.client.rules()
    }

    fn domain_colors(&self) -> HashMap<String, Color> {
        self.domain_colors.clone()
    }

    fn detect_patterns(&self) -> Vec<&'static str> {
        self.detection_patterns()
    }
}

/// Register all Ethereum clients as programs.
pub fn register_all(registry: &mut ProgramRegistry) {
    for client in Client::all() {
        let program = Arc::new(EthereumProgram::new(client));
        registry.register(program);
    }
}

/// Get an Ethereum program by client.
pub fn program_for_client(client: Client) -> Arc<dyn Program> {
    Arc::new(EthereumProgram::new(client))
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
        let program = EthereumProgram::new(Client::Lodestar);
        let colors = program.domain_colors();
        assert!(colors.contains_key("slot"));
        assert!(colors.contains_key("epoch"));
        assert!(colors.contains_key("hash"));
    }
}
