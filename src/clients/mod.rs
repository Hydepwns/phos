//! Ethereum client configurations.
//!
//! Supports 15 clients across consensus, execution, and middleware layers.

use crate::colors::SemanticColor;
use crate::rule::Rule;

/// All supported Ethereum clients.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Client {
    // Consensus Layer
    Lighthouse,
    Prysm,
    Teku,
    Nimbus,
    Lodestar,
    Grandine,
    Lambda,

    // Execution Layer
    Geth,
    Nethermind,
    Besu,
    Erigon,
    Reth,
    Mana,

    // Middleware
    Charon,
    MevBoost,
}

/// Layer type for a client.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    Consensus,
    Execution,
    Full,
    Middleware,
}

/// Client metadata.
#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub layer: Layer,
    pub language: &'static str,
    pub website: &'static str,
}

impl Client {
    /// Get all clients.
    pub fn all() -> Vec<Self> {
        vec![
            // Consensus
            Self::Lighthouse,
            Self::Prysm,
            Self::Teku,
            Self::Nimbus,
            Self::Lodestar,
            Self::Grandine,
            Self::Lambda,
            // Execution
            Self::Geth,
            Self::Nethermind,
            Self::Besu,
            Self::Erigon,
            Self::Reth,
            Self::Mana,
            // Middleware
            Self::Charon,
            Self::MevBoost,
        ]
    }

    /// Get clients by layer.
    pub fn by_layer(layer: Layer) -> Vec<Self> {
        Self::all()
            .into_iter()
            .filter(|c| c.info().layer == layer)
            .collect()
    }

    /// Get client info.
    pub fn info(&self) -> ClientInfo {
        match self {
            // Consensus Layer
            Self::Lighthouse => ClientInfo {
                name: "Lighthouse",
                description: "Ethereum consensus client in Rust",
                layer: Layer::Consensus,
                language: "Rust",
                website: "https://lighthouse.sigmaprime.io/",
            },
            Self::Prysm => ClientInfo {
                name: "Prysm",
                description: "Ethereum consensus client in Go",
                layer: Layer::Consensus,
                language: "Go",
                website: "https://prysmaticlabs.com/",
            },
            Self::Teku => ClientInfo {
                name: "Teku",
                description: "Ethereum consensus client in Java",
                layer: Layer::Consensus,
                language: "Java",
                website: "https://consensys.net/knowledge-base/ethereum-2/teku/",
            },
            Self::Nimbus => ClientInfo {
                name: "Nimbus",
                description: "Ethereum consensus client in Nim",
                layer: Layer::Consensus,
                language: "Nim",
                website: "https://nimbus.team/",
            },
            Self::Lodestar => ClientInfo {
                name: "Lodestar",
                description: "Ethereum consensus client in TypeScript",
                layer: Layer::Consensus,
                language: "TypeScript",
                website: "https://lodestar.chainsafe.io/",
            },
            Self::Grandine => ClientInfo {
                name: "Grandine",
                description: "High-performance consensus client in Rust",
                layer: Layer::Consensus,
                language: "Rust",
                website: "https://grandine.io/",
            },
            Self::Lambda => ClientInfo {
                name: "Lambda",
                description: "Ethereum consensus client in Elixir",
                layer: Layer::Consensus,
                language: "Elixir",
                website: "https://github.com/lambdaclass/lambda_ethereum_consensus",
            },

            // Execution Layer
            Self::Geth => ClientInfo {
                name: "Geth",
                description: "Go Ethereum - the official Go implementation",
                layer: Layer::Execution,
                language: "Go",
                website: "https://geth.ethereum.org/",
            },
            Self::Nethermind => ClientInfo {
                name: "Nethermind",
                description: "Ethereum client in .NET",
                layer: Layer::Execution,
                language: ".NET",
                website: "https://nethermind.io/",
            },
            Self::Besu => ClientInfo {
                name: "Besu",
                description: "Hyperledger Besu - enterprise Ethereum client",
                layer: Layer::Execution,
                language: "Java",
                website: "https://besu.hyperledger.org/",
            },
            Self::Erigon => ClientInfo {
                name: "Erigon",
                description: "Efficiency-focused Ethereum client",
                layer: Layer::Execution,
                language: "Go",
                website: "https://github.com/erigontech/erigon",
            },
            Self::Reth => ClientInfo {
                name: "Reth",
                description: "Modular Ethereum client in Rust by Paradigm",
                layer: Layer::Execution,
                language: "Rust",
                website: "https://paradigmxyz.github.io/reth/",
            },
            Self::Mana => ClientInfo {
                name: "Mana",
                description: "Full Ethereum client (EL+CL) in Elixir with distributed features",
                layer: Layer::Full,
                language: "Elixir",
                website: "https://github.com/axol-io/mana",
            },

            // Middleware
            Self::Charon => ClientInfo {
                name: "Charon",
                description: "Obol distributed validator middleware",
                layer: Layer::Middleware,
                language: "Go",
                website: "https://obol.tech/",
            },
            Self::MevBoost => ClientInfo {
                name: "MEV-Boost",
                description: "Flashbots MEV relay",
                layer: Layer::Middleware,
                language: "Go",
                website: "https://boost.flashbots.net/",
            },
        }
    }

    /// Get colorization rules for this client.
    pub fn rules(&self) -> Vec<Rule> {
        match self {
            Self::Lodestar => lodestar_rules(),
            Self::Lighthouse => lighthouse_rules(),
            Self::Prysm => prysm_rules(),
            Self::Teku => teku_rules(),
            Self::Nimbus => nimbus_rules(),
            Self::Grandine => grandine_rules(),
            Self::Lambda => lambda_rules(),
            Self::Geth => geth_rules(),
            Self::Nethermind => nethermind_rules(),
            Self::Besu => besu_rules(),
            Self::Erigon => erigon_rules(),
            Self::Reth => reth_rules(),
            Self::Mana => mana_rules(),
            Self::Charon => charon_rules(),
            Self::MevBoost => mevboost_rules(),
        }
    }

    /// Parse client from string.
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "lighthouse" => Some(Self::Lighthouse),
            "prysm" => Some(Self::Prysm),
            "teku" => Some(Self::Teku),
            "nimbus" => Some(Self::Nimbus),
            "lodestar" => Some(Self::Lodestar),
            "grandine" => Some(Self::Grandine),
            "lambda" => Some(Self::Lambda),
            "geth" => Some(Self::Geth),
            "nethermind" => Some(Self::Nethermind),
            "besu" => Some(Self::Besu),
            "erigon" => Some(Self::Erigon),
            "reth" => Some(Self::Reth),
            "mana" => Some(Self::Mana),
            "charon" => Some(Self::Charon),
            "mev-boost" | "mevboost" | "mev_boost" => Some(Self::MevBoost),
            _ => None,
        }
    }
}

/// Shared Ethereum log patterns used across multiple clients.
mod ethereum_patterns {
    use super::*;

    // =========================================================================
    // LOG LEVEL PATTERNS (by client style)
    // =========================================================================

    /// Rust-style log levels (ERROR, WARN, INFO, DEBUG, TRACE)
    pub fn rust_log_levels() -> Vec<Rule> {
        vec![
            Rule::new(r"\bERROR\b").unwrap().semantic(SemanticColor::Error).bold().build(),
            Rule::new(r"\bWARN\b").unwrap().semantic(SemanticColor::Warn).bold().build(),
            Rule::new(r"\bINFO\b").unwrap().semantic(SemanticColor::Info).build(),
            Rule::new(r"\bDEBUG\b").unwrap().semantic(SemanticColor::Debug).build(),
            Rule::new(r"\bTRACE\b").unwrap().semantic(SemanticColor::Trace).build(),
        ]
    }

    /// Lighthouse-style log levels (CRIT, ERRO, WARN, INFO, DEBG, TRCE)
    pub fn lighthouse_log_levels() -> Vec<Rule> {
        vec![
            Rule::new(r"\bCRIT\b").unwrap().semantic(SemanticColor::Error).bold().build(),
            Rule::new(r"\bERRO\b").unwrap().semantic(SemanticColor::Error).bold().build(),
            Rule::new(r"\bWARN\b").unwrap().semantic(SemanticColor::Warn).bold().build(),
            Rule::new(r"\bINFO\b").unwrap().semantic(SemanticColor::Info).build(),
            Rule::new(r"\bDEBG\b").unwrap().semantic(SemanticColor::Debug).build(),
            Rule::new(r"\bTRCE\b").unwrap().semantic(SemanticColor::Trace).build(),
        ]
    }

    /// Nimbus-style log levels (FTL, ERR, WRN, INF, DBG, TRC)
    pub fn nimbus_log_levels() -> Vec<Rule> {
        vec![
            Rule::new(r"\bFTL\b").unwrap().semantic(SemanticColor::Error).bold().build(),
            Rule::new(r"\bERR\b").unwrap().semantic(SemanticColor::Error).bold().build(),
            Rule::new(r"\bWRN\b").unwrap().semantic(SemanticColor::Warn).bold().build(),
            Rule::new(r"\bINF\b").unwrap().semantic(SemanticColor::Info).build(),
            Rule::new(r"\bDBG\b").unwrap().semantic(SemanticColor::Debug).build(),
            Rule::new(r"\bTRC\b").unwrap().semantic(SemanticColor::Trace).build(),
        ]
    }

    /// Lodestar-style log levels (error/ERR, warn/WRN, info/INF, debug/DBG, verbose/VRB)
    pub fn lodestar_log_levels() -> Vec<Rule> {
        vec![
            Rule::new(r"\b(error|ERR)\b").unwrap().semantic(SemanticColor::Error).bold().build(),
            Rule::new(r"\b(warn|WRN)\b").unwrap().semantic(SemanticColor::Warn).bold().build(),
            Rule::new(r"\b(info|INF)\b").unwrap().semantic(SemanticColor::Info).build(),
            Rule::new(r"\b(debug|DBG)\b").unwrap().semantic(SemanticColor::Debug).build(),
            Rule::new(r"\b(verbose|VRB|trace)\b").unwrap().semantic(SemanticColor::Trace).build(),
        ]
    }

    /// Prysm-style log levels (level=error, level=warning, etc.)
    pub fn prysm_log_levels() -> Vec<Rule> {
        vec![
            Rule::new(r"level=error").unwrap().semantic(SemanticColor::Error).bold().build(),
            Rule::new(r"level=warning").unwrap().semantic(SemanticColor::Warn).bold().build(),
            Rule::new(r"level=info").unwrap().semantic(SemanticColor::Info).build(),
            Rule::new(r"level=debug").unwrap().semantic(SemanticColor::Debug).build(),
            Rule::new(r"level=trace").unwrap().semantic(SemanticColor::Trace).build(),
        ]
    }

    /// .NET-style log levels (Error, Warn, Info, Debug, Trace)
    pub fn dotnet_log_levels() -> Vec<Rule> {
        vec![
            Rule::new(r"\bError\b").unwrap().semantic(SemanticColor::Error).bold().build(),
            Rule::new(r"\bWarn\b").unwrap().semantic(SemanticColor::Warn).bold().build(),
            Rule::new(r"\bInfo\b").unwrap().semantic(SemanticColor::Info).build(),
            Rule::new(r"\bDebug\b").unwrap().semantic(SemanticColor::Debug).build(),
            Rule::new(r"\bTrace\b").unwrap().semantic(SemanticColor::Trace).build(),
        ]
    }

    /// Erigon-style log levels (lvl=eror, lvl=warn, etc.)
    pub fn erigon_log_levels() -> Vec<Rule> {
        vec![
            Rule::new(r"lvl=eror").unwrap().semantic(SemanticColor::Error).bold().build(),
            Rule::new(r"lvl=warn").unwrap().semantic(SemanticColor::Warn).bold().build(),
            Rule::new(r"lvl=info").unwrap().semantic(SemanticColor::Info).build(),
            Rule::new(r"lvl=dbug").unwrap().semantic(SemanticColor::Debug).build(),
            Rule::new(r"lvl=trce").unwrap().semantic(SemanticColor::Trace).build(),
        ]
    }

    /// Elixir Logger-style log levels ([error], [warning], [info], [debug])
    pub fn elixir_log_levels() -> Vec<Rule> {
        vec![
            Rule::new(r"\[error\]").unwrap().semantic(SemanticColor::Error).bold().build(),
            Rule::new(r"\[warning\]").unwrap().semantic(SemanticColor::Warn).bold().build(),
            Rule::new(r"\[info\]").unwrap().semantic(SemanticColor::Info).build(),
            Rule::new(r"\[debug\]").unwrap().semantic(SemanticColor::Debug).build(),
        ]
    }

    // =========================================================================
    // COMMON ETHEREUM PATTERNS
    // =========================================================================

    pub fn hash_rule() -> Rule {
        Rule::new(r"0x[a-fA-F0-9]{8,}")
            .unwrap()
            .semantic(SemanticColor::Hash)
            .build()
    }

    pub fn address_rule() -> Rule {
        Rule::new(r"0x[a-fA-F0-9]{40}")
            .unwrap()
            .semantic(SemanticColor::Address)
            .build()
    }

    pub fn number_rule() -> Rule {
        Rule::new(r"\b\d+(\.\d+)?\b")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build()
    }

    pub fn slot_rule() -> Rule {
        Rule::new(r"\bslot[=:\s]+(\d+)")
            .unwrap()
            .semantic(SemanticColor::Slot)
            .build()
    }

    pub fn epoch_rule() -> Rule {
        Rule::new(r"\bepoch[=:\s]+(\d+)")
            .unwrap()
            .semantic(SemanticColor::Epoch)
            .build()
    }

    pub fn peers_rule() -> Rule {
        Rule::new(r"\bpeers?[=:\s]+(\d+)")
            .unwrap()
            .semantic(SemanticColor::PeerId)
            .build()
    }

    pub fn syncing_rule() -> Rule {
        Rule::new(r"\b(Syncing|Synced|syncing|synced)\b")
            .unwrap()
            .semantic(SemanticColor::Syncing)
            .build()
    }

    pub fn success_rule() -> Rule {
        Rule::new(r"\b(success|valid|verified)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build()
    }

    pub fn failure_rule() -> Rule {
        Rule::new(r"\b(failed|invalid|error|timeout|rejected)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build()
    }

    pub fn validator_rule() -> Rule {
        Rule::new(r"\b(validator|idx|validator_index)[=:\s]+(\d+)")
            .unwrap()
            .semantic(SemanticColor::Validator)
            .build()
    }

    pub fn pubkey_rule() -> Rule {
        Rule::new(r"pubkey[=:\s]+0x[a-fA-F0-9]{8,}")
            .unwrap()
            .semantic(SemanticColor::Pubkey)
            .build()
    }

    pub fn duty_rule() -> Rule {
        Rule::new(r"\b(attester|proposer|sync_committee|aggregator|attesting|proposing)\b")
            .unwrap()
            .semantic(SemanticColor::Duty)
            .build()
    }

    pub fn committee_rule() -> Rule {
        Rule::new(r"\b(committee|subnet)[=:\s]+(\d+)")
            .unwrap()
            .semantic(SemanticColor::Committee)
            .build()
    }

    pub fn finality_rule() -> Rule {
        Rule::new(r"\b(finalized|justified|finality|checkpoint)\b")
            .unwrap()
            .semantic(SemanticColor::Finality)
            .build()
    }

    pub fn root_rule() -> Rule {
        Rule::new(r"(state_root|block_root|root)[=:\s]+0x[a-fA-F0-9]{64}")
            .unwrap()
            .semantic(SemanticColor::Root)
            .build()
    }

    pub fn attestation_rule() -> Rule {
        Rule::new(r"\b(attestation|attest|attested)\b")
            .unwrap()
            .semantic(SemanticColor::Attestation)
            .build()
    }

    pub fn mev_value_rule() -> Rule {
        Rule::new(r"(\d+\.?\d*)\s*(ETH|Gwei|gwei|wei)")
            .unwrap()
            .semantic(SemanticColor::MevValue)
            .bold()
            .build()
    }

    pub fn relay_rule() -> Rule {
        Rule::new(r"\b(flashbots|bloxroute|blocknative|eden|manifold|ultrasound|agnostic)\b")
            .unwrap()
            .semantic(SemanticColor::Relay)
            .build()
    }

    pub fn builder_rule() -> Rule {
        Rule::new(r"builder[=:\s]+(\S+)")
            .unwrap()
            .semantic(SemanticColor::Builder)
            .build()
    }

    // =========================================================================
    // COMPOSITE RULE SETS
    // =========================================================================

    /// Common consensus layer patterns (validators, duties, attestations, etc.)
    pub fn consensus_patterns() -> Vec<Rule> {
        vec![
            slot_rule(),
            epoch_rule(),
            address_rule(), // Before hash_rule for specific address matching
            hash_rule(),
            peers_rule(),
            syncing_rule(),
            success_rule(),
            failure_rule(),
            validator_rule(),
            pubkey_rule(),
            duty_rule(),
            committee_rule(),
            finality_rule(),
            root_rule(),
            attestation_rule(),
            number_rule(),
        ]
    }

    /// Common execution layer patterns (blocks, forkchoice, etc.)
    pub fn execution_patterns() -> Vec<Rule> {
        vec![
            address_rule(), // Before hash_rule for specific address matching
            hash_rule(),
            peers_rule(),
            syncing_rule(),
            success_rule(),
            failure_rule(),
            root_rule(),
            finality_rule(),
            number_rule(),
        ]
    }

    /// MEV/relay patterns
    pub fn mev_patterns() -> Vec<Rule> {
        vec![
            mev_value_rule(),
            relay_rule(),
            builder_rule(),
        ]
    }
}

// =============================================================================
// CONSENSUS LAYER CLIENTS
// =============================================================================

fn lodestar_rules() -> Vec<Rule> {
    let mut rules = ethereum_patterns::lodestar_log_levels();
    // Lodestar-specific patterns
    rules.extend([
        Rule::new(r"Eph\s+\d+/\d+").unwrap().semantic(SemanticColor::Epoch).build(),
        Rule::new(r"slot=\d+").unwrap().semantic(SemanticColor::Slot).build(),
        Rule::new(r"epoch=\d+").unwrap().semantic(SemanticColor::Epoch).build(),
        Rule::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}").unwrap().semantic(SemanticColor::Timestamp).build(),
    ]);
    rules.extend(ethereum_patterns::consensus_patterns());
    rules
}

fn lighthouse_rules() -> Vec<Rule> {
    let mut rules = ethereum_patterns::lighthouse_log_levels();
    rules.push(
        Rule::new(r"\w{3}\s+\d{2}\s+\d{2}:\d{2}:\d{2}\.\d{3}")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );
    rules.extend(ethereum_patterns::consensus_patterns());
    rules
}

fn prysm_rules() -> Vec<Rule> {
    let mut rules = ethereum_patterns::prysm_log_levels();
    rules.extend([
        Rule::new(r#"msg="[^"]*""#).unwrap().semantic(SemanticColor::String).build(),
        Rule::new(r#"prefix="[^"]*""#).unwrap().named("cyan").build(),
    ]);
    rules.extend(ethereum_patterns::consensus_patterns());
    rules
}

fn teku_rules() -> Vec<Rule> {
    let mut rules = ethereum_patterns::rust_log_levels();
    rules.push(
        Rule::new(r"\d{2}:\d{2}:\d{2}\.\d{3}")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );
    rules.extend(ethereum_patterns::consensus_patterns());
    rules
}

fn nimbus_rules() -> Vec<Rule> {
    let mut rules = ethereum_patterns::nimbus_log_levels();
    rules.extend(ethereum_patterns::consensus_patterns());
    rules
}

fn grandine_rules() -> Vec<Rule> {
    let mut rules = ethereum_patterns::rust_log_levels();
    rules.extend(ethereum_patterns::consensus_patterns());
    rules
}

fn lambda_rules() -> Vec<Rule> {
    elixir_common_rules()
}

// =============================================================================
// EXECUTION LAYER CLIENTS
// =============================================================================

fn geth_rules() -> Vec<Rule> {
    let mut rules = ethereum_patterns::rust_log_levels();
    // Geth-specific patterns
    rules.extend([
        // Geth timestamp: [10-03|15:34:01.336]
        Rule::new(r"\[\d{2}-\d{2}\|\d{2}:\d{2}:\d{2}\.\d{3}\]").unwrap().semantic(SemanticColor::Timestamp).build(),
        // Block number
        Rule::new(r"number=\d+").unwrap().semantic(SemanticColor::BlockNumber).build(),
        // Transaction count
        Rule::new(r"txs=\d+").unwrap().semantic(SemanticColor::Number).build(),
        // Forkchoice
        Rule::new(r"\b(forkchoice|FCU)\b").unwrap().semantic(SemanticColor::Finality).bold().build(),
    ]);
    rules.extend(ethereum_patterns::execution_patterns());
    rules
}

fn nethermind_rules() -> Vec<Rule> {
    let mut rules = ethereum_patterns::dotnet_log_levels();
    rules.push(
        Rule::new(r"number=\d+").unwrap().semantic(SemanticColor::BlockNumber).build(),
    );
    rules.extend(ethereum_patterns::execution_patterns());
    rules
}

fn besu_rules() -> Vec<Rule> {
    let mut rules = ethereum_patterns::rust_log_levels();
    rules.push(
        Rule::new(r"Block #\d+").unwrap().semantic(SemanticColor::BlockNumber).build(),
    );
    rules.extend(ethereum_patterns::execution_patterns());
    rules
}

fn erigon_rules() -> Vec<Rule> {
    let mut rules = ethereum_patterns::erigon_log_levels();
    rules.push(
        Rule::new(r"number=\d+").unwrap().semantic(SemanticColor::BlockNumber).build(),
    );
    rules.extend(ethereum_patterns::execution_patterns());
    rules
}

fn reth_rules() -> Vec<Rule> {
    let mut rules = ethereum_patterns::rust_log_levels();
    // Reth-specific patterns
    rules.extend([
        Rule::new(r"stage=\w+").unwrap().semantic(SemanticColor::Key).build(),
        Rule::new(r"progress=\d+\.\d+%").unwrap().semantic(SemanticColor::Number).build(),
        Rule::new(r"number=\d+").unwrap().semantic(SemanticColor::BlockNumber).build(),
    ]);
    rules.extend(ethereum_patterns::execution_patterns());
    rules
}

fn mana_rules() -> Vec<Rule> {
    let mut rules = elixir_common_rules();
    // Mana-specific patterns
    rules.extend([
        Rule::new(r"\b(Blockchain|EVM|ExWire|MerklePatriciaTree|JSONRPC2)\b")
            .unwrap().semantic(SemanticColor::Key).bold().build(),
        Rule::new(r"\b(L2|layer2|rollup|optimistic|zk-?proof)\b")
            .unwrap().semantic(SemanticColor::Builder).build(),
        Rule::new(r"\b(verkle|crdt|antidote|distributed)\b")
            .unwrap().semantic(SemanticColor::Committee).build(),
        Rule::new(r"\b(eth_\w+|web3_\w+|net_\w+)\b")
            .unwrap().semantic(SemanticColor::Value).build(),
    ]);
    rules
}

// =============================================================================
// MIDDLEWARE
// =============================================================================

fn charon_rules() -> Vec<Rule> {
    let mut rules = ethereum_patterns::rust_log_levels();
    // Charon-specific patterns
    rules.extend([
        // QBFT consensus
        Rule::new(r"\bQBFT\b").unwrap().semantic(SemanticColor::Committee).bold().build(),
        Rule::new(r"\b(pre-prepare|prepare|commit|round-change)\b")
            .unwrap().semantic(SemanticColor::Committee).build(),
        // Threshold signatures
        Rule::new(r"\bthreshold\b").unwrap().semantic(SemanticColor::Attestation).build(),
        Rule::new(r"partial[_\s]?sig").unwrap().semantic(SemanticColor::Attestation).build(),
        // Peer/node info
        Rule::new(r"peer=\w+").unwrap().semantic(SemanticColor::PeerId).build(),
        Rule::new(r"node=\d+").unwrap().semantic(SemanticColor::Number).build(),
    ]);
    rules.extend(ethereum_patterns::consensus_patterns());
    rules
}

fn mevboost_rules() -> Vec<Rule> {
    let mut rules = ethereum_patterns::rust_log_levels();
    // MEV-Boost-specific patterns
    rules.extend([
        Rule::new(r"slot=\d+").unwrap().semantic(SemanticColor::Slot).build(),
        Rule::new(r"value=[\d\.]+").unwrap().semantic(SemanticColor::MevValue).bold().build(),
        Rule::new(r"bid=[\d\.]+").unwrap().semantic(SemanticColor::MevValue).bold().build(),
        Rule::new(r"block_value=[\d\.]+").unwrap().semantic(SemanticColor::MevValue).bold().build(),
        Rule::new(r"relay=\S+").unwrap().semantic(SemanticColor::Relay).build(),
        Rule::new(r"builder=\S+").unwrap().semantic(SemanticColor::Builder).build(),
        Rule::new(r"builder_pubkey=0x[a-fA-F0-9]+").unwrap().semantic(SemanticColor::Builder).build(),
        ethereum_patterns::hash_rule(),
        ethereum_patterns::success_rule(),
        ethereum_patterns::failure_rule(),
    ]);
    rules.extend(ethereum_patterns::mev_patterns());
    rules.push(ethereum_patterns::number_rule());
    rules
}

// =============================================================================
// ELIXIR CLIENT PATTERNS
// =============================================================================

fn elixir_common_rules() -> Vec<Rule> {
    let mut rules = ethereum_patterns::elixir_log_levels();
    // Elixir-specific patterns
    rules.extend([
        Rule::new(r"\d{2}:\d{2}:\d{2}\.\d{3}").unwrap().semantic(SemanticColor::Timestamp).build(),
        Rule::new(r"\b[A-Z][a-zA-Z0-9]*(\.[A-Z][a-zA-Z0-9]*)+")
            .unwrap().semantic(SemanticColor::Key).build(),
        Rule::new(r"#PID<[\d\.]+>").unwrap().semantic(SemanticColor::PeerId).build(),
        Rule::new(r":\w+").unwrap().semantic(SemanticColor::Value).build(),
        Rule::new(r#""[^"]*""#).unwrap().semantic(SemanticColor::String).build(),
    ]);
    rules.extend(ethereum_patterns::consensus_patterns());
    rules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_clients() {
        assert_eq!(Client::all().len(), 15);
    }

    #[test]
    fn test_parse_client() {
        assert_eq!(Client::parse("lodestar"), Some(Client::Lodestar));
        assert_eq!(Client::parse("GETH"), Some(Client::Geth));
        assert_eq!(Client::parse("mev-boost"), Some(Client::MevBoost));
        assert_eq!(Client::parse("mana"), Some(Client::Mana));
    }

    #[test]
    fn test_mana_is_full_node() {
        assert_eq!(Client::Mana.info().layer, Layer::Full);
    }

    #[test]
    fn test_elixir_clients() {
        assert_eq!(Client::Lambda.info().language, "Elixir");
        assert_eq!(Client::Mana.info().language, "Elixir");
    }

    #[test]
    fn test_all_clients_have_rules() {
        for client in Client::all() {
            let rules = client.rules();
            assert!(!rules.is_empty(), "{client:?} has no rules");
        }
    }
}
