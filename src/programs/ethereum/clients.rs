//! Ethereum client definitions and rules.

use super::patterns;
use crate::colors::SemanticColor;
use crate::rule::Rule;

/// Layer type for an Ethereum client.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    Consensus,
    Execution,
    Full,
    Middleware,
}

/// Client metadata.
pub struct ClientMeta {
    pub name: &'static str,
    pub description: &'static str,
    pub layer: Layer,
    pub language: &'static str,
    pub website: &'static str,
    pub detect_patterns: &'static [&'static str],
    pub brand_color: &'static str,
}

// =============================================================================
// CLIENT METADATA
// =============================================================================

pub const LIGHTHOUSE: ClientMeta = ClientMeta {
    name: "Lighthouse",
    description: "Ethereum consensus client in Rust",
    layer: Layer::Consensus,
    language: "Rust",
    website: "https://lighthouse.sigmaprime.io/",
    detect_patterns: &[
        "lighthouse",
        "lighthouse-bn",
        "lighthouse-vc",
        "lighthouse-validator",
        "lighthouse_beacon",
        "lighthouse_validator",
        "lighthouse-consensus",
        "lighthouse.log",
    ],
    brand_color: "#9933FF",
};

pub const PRYSM: ClientMeta = ClientMeta {
    name: "Prysm",
    description: "Ethereum consensus client in Go",
    layer: Layer::Consensus,
    language: "Go",
    website: "https://prysmaticlabs.com/",
    detect_patterns: &[
        "prysm",
        "beacon-chain",
        "validator",
        "prysm-bn",
        "prysm-vc",
        "prysm_beacon",
        "prysm_validator",
        "prysm-consensus",
        "prysm.log",
    ],
    brand_color: "#22CC88",
};

pub const TEKU: ClientMeta = ClientMeta {
    name: "Teku",
    description: "Ethereum consensus client in Java",
    layer: Layer::Consensus,
    language: "Java",
    website: "https://consensys.net/knowledge-base/ethereum-2/teku/",
    detect_patterns: &[
        "teku",
        "teku-bn",
        "teku-vc",
        "teku_beacon",
        "teku_validator",
        "teku-consensus",
        "teku.log",
    ],
    brand_color: "#3366FF",
};

pub const NIMBUS: ClientMeta = ClientMeta {
    name: "Nimbus",
    description: "Ethereum consensus client in Nim",
    layer: Layer::Consensus,
    language: "Nim",
    website: "https://nimbus.team/",
    detect_patterns: &[
        "nimbus",
        "nimbus-bn",
        "nimbus-vc",
        "nimbus_beacon",
        "nimbus_validator",
        "nimbus-eth2",
        "nimbus-consensus",
        "nimbus.log",
    ],
    brand_color: "#CC9933",
};

pub const LODESTAR: ClientMeta = ClientMeta {
    name: "Lodestar",
    description: "Ethereum consensus client in TypeScript",
    layer: Layer::Consensus,
    language: "TypeScript",
    website: "https://lodestar.chainsafe.io/",
    detect_patterns: &[
        "lodestar",
        "lodestar-bn",
        "lodestar-vc",
        "lodestar_beacon",
        "lodestar_validator",
        "lodestar-consensus",
        "lodestar.log",
    ],
    brand_color: "#AA44FF",
};

pub const GRANDINE: ClientMeta = ClientMeta {
    name: "Grandine",
    description: "High-performance consensus client in Rust",
    layer: Layer::Consensus,
    language: "Rust",
    website: "https://grandine.io/",
    detect_patterns: &[
        "grandine",
        "grandine-bn",
        "grandine-vc",
        "grandine_beacon",
        "grandine-consensus",
        "grandine.log",
    ],
    brand_color: "#FF6633",
};

pub const LAMBDA: ClientMeta = ClientMeta {
    name: "Lambda",
    description: "Ethereum consensus client in Elixir",
    layer: Layer::Consensus,
    language: "Elixir",
    website: "https://github.com/lambdaclass/lambda_ethereum_consensus",
    detect_patterns: &[
        "lambda_ethereum",
        "lambda-bn",
        "lambda-consensus",
        "lambda.log",
    ],
    brand_color: "#9966FF",
};

pub const GETH: ClientMeta = ClientMeta {
    name: "Geth",
    description: "Go Ethereum - the official Go implementation",
    layer: Layer::Execution,
    language: "Go",
    website: "https://geth.ethereum.org/",
    detect_patterns: &[
        "geth",
        "geth-el",
        "geth_execution",
        "go-ethereum",
        "geth.log",
    ],
    brand_color: "#6699FF",
};

pub const NETHERMIND: ClientMeta = ClientMeta {
    name: "Nethermind",
    description: "Ethereum client in .NET",
    layer: Layer::Execution,
    language: ".NET",
    website: "https://nethermind.io/",
    detect_patterns: &[
        "nethermind",
        "nethermind-el",
        "nethermind_execution",
        "nethermind.log",
    ],
    brand_color: "#33CCCC",
};

pub const BESU: ClientMeta = ClientMeta {
    name: "Besu",
    description: "Hyperledger Besu - enterprise Ethereum client",
    layer: Layer::Execution,
    language: "Java",
    website: "https://besu.hyperledger.org/",
    detect_patterns: &[
        "besu",
        "besu-el",
        "besu_execution",
        "hyperledger-besu",
        "besu.log",
    ],
    brand_color: "#009999",
};

pub const ERIGON: ClientMeta = ClientMeta {
    name: "Erigon",
    description: "Efficiency-focused Ethereum client",
    layer: Layer::Execution,
    language: "Go",
    website: "https://github.com/erigontech/erigon",
    detect_patterns: &[
        "erigon",
        "erigon-el",
        "erigon_execution",
        "erigon.log",
    ],
    brand_color: "#66CC33",
};

pub const RETH: ClientMeta = ClientMeta {
    name: "Reth",
    description: "Modular Ethereum client in Rust by Paradigm",
    layer: Layer::Execution,
    language: "Rust",
    website: "https://paradigmxyz.github.io/reth/",
    detect_patterns: &[
        "reth",
        "reth-el",
        "reth_execution",
        "reth.log",
    ],
    brand_color: "#FF9966",
};

pub const MANA: ClientMeta = ClientMeta {
    name: "Mana",
    description: "Full Ethereum client (EL+CL) in Elixir with distributed features",
    layer: Layer::Full,
    language: "Elixir",
    website: "https://github.com/axol-io/mana",
    detect_patterns: &[
        "mana",
        "mana-el",
        "mana-cl",
        "mana_node",
        "mana.log",
    ],
    brand_color: "#CC66FF",
};

pub const CHARON: ClientMeta = ClientMeta {
    name: "Charon",
    description: "Obol distributed validator middleware",
    layer: Layer::Middleware,
    language: "Go",
    website: "https://obol.tech/",
    detect_patterns: &[
        "charon",
        "charon-dv",
        "obol-charon",
        "charon.log",
    ],
    brand_color: "#6633FF",
};

pub const MEVBOOST: ClientMeta = ClientMeta {
    name: "MEV-Boost",
    description: "Flashbots MEV relay",
    layer: Layer::Middleware,
    language: "Go",
    website: "https://boost.flashbots.net/",
    detect_patterns: &[
        "mev-boost",
        "mev_boost",
        "mevboost",
        "mev-boost.log",
    ],
    brand_color: "#FF6699",
};

/// All client metadata in order.
pub const ALL_CLIENTS: &[&ClientMeta] = &[
    &LIGHTHOUSE, &PRYSM, &TEKU, &NIMBUS, &LODESTAR, &GRANDINE, &LAMBDA,
    &GETH, &NETHERMIND, &BESU, &ERIGON, &RETH, &MANA,
    &CHARON, &MEVBOOST,
];

// =============================================================================
// CONSENSUS LAYER CLIENTS
// =============================================================================

#[must_use] pub fn lodestar_rules() -> Vec<Rule> {
    let mut rules = patterns::lodestar_log_levels();
    // Lodestar-specific patterns
    rules.extend([
        Rule::new(r"Eph\s+\d+/\d+").unwrap().hex(patterns::EPOCH_COLOR).build(),
        Rule::new(r"slot=\d+").unwrap().hex(patterns::SLOT_COLOR).build(),
        Rule::new(r"epoch=\d+").unwrap().hex(patterns::EPOCH_COLOR).build(),
        Rule::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}").unwrap().semantic(SemanticColor::Timestamp).build(),
    ]);
    rules.extend(patterns::consensus_patterns());
    rules
}

#[must_use] pub fn lighthouse_rules() -> Vec<Rule> {
    let mut rules = patterns::lighthouse_log_levels();
    rules.push(
        Rule::new(r"\w{3}\s+\d{2}\s+\d{2}:\d{2}:\d{2}\.\d{3}")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );
    rules.extend(patterns::consensus_patterns());
    rules
}

#[must_use] pub fn prysm_rules() -> Vec<Rule> {
    let mut rules = patterns::prysm_log_levels();
    rules.extend([
        Rule::new(r#"msg="[^"]*""#).unwrap().semantic(SemanticColor::String).build(),
        Rule::new(r#"prefix="[^"]*""#).unwrap().named("cyan").build(),
    ]);
    rules.extend(patterns::consensus_patterns());
    rules
}

#[must_use] pub fn teku_rules() -> Vec<Rule> {
    let mut rules = patterns::rust_log_levels();
    rules.push(
        Rule::new(r"\d{2}:\d{2}:\d{2}\.\d{3}")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );
    rules.extend(patterns::consensus_patterns());
    rules
}

#[must_use] pub fn nimbus_rules() -> Vec<Rule> {
    let mut rules = patterns::nimbus_log_levels();
    rules.extend(patterns::consensus_patterns());
    rules
}

#[must_use] pub fn grandine_rules() -> Vec<Rule> {
    let mut rules = patterns::rust_log_levels();
    rules.extend(patterns::consensus_patterns());
    rules
}

#[must_use] pub fn lambda_rules() -> Vec<Rule> {
    elixir_common_rules()
}

// =============================================================================
// EXECUTION LAYER CLIENTS
// =============================================================================

#[must_use] pub fn geth_rules() -> Vec<Rule> {
    let mut rules = patterns::rust_log_levels();
    // Geth-specific patterns
    rules.extend([
        // Geth timestamp: [10-03|15:34:01.336]
        Rule::new(r"\[\d{2}-\d{2}\|\d{2}:\d{2}:\d{2}\.\d{3}\]").unwrap().semantic(SemanticColor::Timestamp).build(),
        // Block number
        Rule::new(r"number=\d+").unwrap().hex(patterns::BLOCK_NUMBER_COLOR).build(),
        // Transaction count
        Rule::new(r"txs=\d+").unwrap().semantic(SemanticColor::Number).build(),
        // Forkchoice
        Rule::new(r"\b(forkchoice|FCU)\b").unwrap().hex(patterns::FINALITY_COLOR).bold().build(),
    ]);
    rules.extend(patterns::execution_patterns());
    rules
}

#[must_use] pub fn nethermind_rules() -> Vec<Rule> {
    let mut rules = patterns::dotnet_log_levels();
    rules.push(
        Rule::new(r"number=\d+").unwrap().hex(patterns::BLOCK_NUMBER_COLOR).build(),
    );
    rules.extend(patterns::execution_patterns());
    rules
}

#[must_use] pub fn besu_rules() -> Vec<Rule> {
    let mut rules = patterns::rust_log_levels();
    rules.push(
        Rule::new(r"Block #\d+").unwrap().hex(patterns::BLOCK_NUMBER_COLOR).build(),
    );
    rules.extend(patterns::execution_patterns());
    rules
}

#[must_use] pub fn erigon_rules() -> Vec<Rule> {
    let mut rules = patterns::erigon_log_levels();
    rules.push(
        Rule::new(r"number=\d+").unwrap().hex(patterns::BLOCK_NUMBER_COLOR).build(),
    );
    rules.extend(patterns::execution_patterns());
    rules
}

#[must_use] pub fn reth_rules() -> Vec<Rule> {
    let mut rules = patterns::rust_log_levels();
    // Reth-specific patterns
    rules.extend([
        Rule::new(r"stage=\w+").unwrap().semantic(SemanticColor::Key).build(),
        Rule::new(r"progress=\d+\.\d+%").unwrap().semantic(SemanticColor::Number).build(),
        Rule::new(r"number=\d+").unwrap().hex(patterns::BLOCK_NUMBER_COLOR).build(),
    ]);
    rules.extend(patterns::execution_patterns());
    rules
}

#[must_use] pub fn mana_rules() -> Vec<Rule> {
    let mut rules = elixir_common_rules();
    // Mana-specific patterns
    rules.extend([
        Rule::new(r"\b(Blockchain|EVM|ExWire|MerklePatriciaTree|JSONRPC2)\b")
            .unwrap().semantic(SemanticColor::Key).bold().build(),
        Rule::new(r"\b(L2|layer2|rollup|optimistic|zk-?proof)\b")
            .unwrap().hex(patterns::BUILDER_COLOR).build(),
        Rule::new(r"\b(verkle|crdt|antidote|distributed)\b")
            .unwrap().hex(patterns::COMMITTEE_COLOR).build(),
        Rule::new(r"\b(eth_\w+|web3_\w+|net_\w+)\b")
            .unwrap().semantic(SemanticColor::Value).build(),
    ]);
    rules
}

// =============================================================================
// MIDDLEWARE
// =============================================================================

#[must_use] pub fn charon_rules() -> Vec<Rule> {
    let mut rules = patterns::rust_log_levels();
    // Charon-specific patterns
    rules.extend([
        // QBFT consensus
        Rule::new(r"\bQBFT\b").unwrap().hex(patterns::COMMITTEE_COLOR).bold().build(),
        Rule::new(r"\b(pre-prepare|prepare|commit|round-change)\b")
            .unwrap().hex(patterns::COMMITTEE_COLOR).build(),
        // Threshold signatures
        Rule::new(r"\bthreshold\b").unwrap().hex(patterns::ATTESTATION_COLOR).build(),
        Rule::new(r"partial[_\s]?sig").unwrap().hex(patterns::ATTESTATION_COLOR).build(),
        // Peer/node info
        Rule::new(r"peer=\w+").unwrap().hex(patterns::PEER_ID_COLOR).build(),
        Rule::new(r"node=\d+").unwrap().semantic(SemanticColor::Number).build(),
    ]);
    rules.extend(patterns::consensus_patterns());
    rules
}

#[must_use] pub fn mevboost_rules() -> Vec<Rule> {
    let mut rules = patterns::rust_log_levels();
    // MEV-Boost-specific patterns
    rules.extend([
        Rule::new(r"slot=\d+").unwrap().hex(patterns::SLOT_COLOR).build(),
        Rule::new(r"value=[\d\.]+").unwrap().hex(patterns::MEV_VALUE_COLOR).bold().build(),
        Rule::new(r"bid=[\d\.]+").unwrap().hex(patterns::MEV_VALUE_COLOR).bold().build(),
        Rule::new(r"block_value=[\d\.]+").unwrap().hex(patterns::MEV_VALUE_COLOR).bold().build(),
        Rule::new(r"relay=\S+").unwrap().hex(patterns::RELAY_COLOR).build(),
        Rule::new(r"builder=\S+").unwrap().hex(patterns::BUILDER_COLOR).build(),
        Rule::new(r"builder_pubkey=0x[a-fA-F0-9]+").unwrap().hex(patterns::BUILDER_COLOR).build(),
        patterns::hash_rule(),
        patterns::success_rule(),
        patterns::failure_rule(),
    ]);
    rules.extend(patterns::mev_patterns());
    rules.push(patterns::number_rule());
    rules
}

// =============================================================================
// ELIXIR CLIENT PATTERNS
// =============================================================================

fn elixir_common_rules() -> Vec<Rule> {
    let mut rules = patterns::elixir_log_levels();
    // Elixir-specific patterns
    rules.extend([
        Rule::new(r"\d{2}:\d{2}:\d{2}\.\d{3}").unwrap().semantic(SemanticColor::Timestamp).build(),
        Rule::new(r"\b[A-Z][a-zA-Z0-9]*(\.[A-Z][a-zA-Z0-9]*)+")
            .unwrap().semantic(SemanticColor::Key).build(),
        Rule::new(r"#PID<[\d\.]+>").unwrap().hex(patterns::PEER_ID_COLOR).build(),
        Rule::new(r":\w+").unwrap().semantic(SemanticColor::Value).build(),
        Rule::new(r#""[^"]*""#).unwrap().semantic(SemanticColor::String).build(),
    ]);
    rules.extend(patterns::consensus_patterns());
    rules
}

/// Get rules for a client by name (case-insensitive).
#[must_use] pub fn rules_for(name: &str) -> Option<Vec<Rule>> {
    match name.to_lowercase().as_str() {
        "lighthouse" => Some(lighthouse_rules()),
        "prysm" => Some(prysm_rules()),
        "teku" => Some(teku_rules()),
        "nimbus" => Some(nimbus_rules()),
        "lodestar" => Some(lodestar_rules()),
        "grandine" => Some(grandine_rules()),
        "lambda" => Some(lambda_rules()),
        "geth" => Some(geth_rules()),
        "nethermind" => Some(nethermind_rules()),
        "besu" => Some(besu_rules()),
        "erigon" => Some(erigon_rules()),
        "reth" => Some(reth_rules()),
        "mana" => Some(mana_rules()),
        "charon" => Some(charon_rules()),
        "mev-boost" | "mevboost" | "mev_boost" => Some(mevboost_rules()),
        _ => None,
    }
}

/// Get client metadata by name (case-insensitive).
#[must_use] pub fn meta_for(name: &str) -> Option<&'static ClientMeta> {
    match name.to_lowercase().as_str() {
        "lighthouse" => Some(&LIGHTHOUSE),
        "prysm" => Some(&PRYSM),
        "teku" => Some(&TEKU),
        "nimbus" => Some(&NIMBUS),
        "lodestar" => Some(&LODESTAR),
        "grandine" => Some(&GRANDINE),
        "lambda" => Some(&LAMBDA),
        "geth" => Some(&GETH),
        "nethermind" => Some(&NETHERMIND),
        "besu" => Some(&BESU),
        "erigon" => Some(&ERIGON),
        "reth" => Some(&RETH),
        "mana" => Some(&MANA),
        "charon" => Some(&CHARON),
        "mev-boost" | "mevboost" | "mev_boost" => Some(&MEVBOOST),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: check if rules colorize the input (any rule matches)
    fn rules_match(rules: &[Rule], input: &str) -> bool {
        rules.iter().any(|r| r.is_match(input))
    }

    // =========================================================================
    // CONSENSUS CLIENTS
    // =========================================================================

    #[test]
    fn test_lodestar_rules_match_real_logs() {
        let rules = lodestar_rules();
        // Lodestar format: timestamp[] level: message
        assert!(rules_match(&rules, "Dec 05 00:12:36.557[] info: Synced - slot 5375712 - Peers 47"));
        assert!(rules_match(&rules, "Dec 05 00:12:36.000[] error: Failed to connect"));
        assert!(rules_match(&rules, "Eph 167991/6")); // Epoch/slot format
        assert!(rules_match(&rules, "slot=12345"));
        assert!(rules_match(&rules, "epoch=385"));
    }

    #[test]
    fn test_lighthouse_rules_match_real_logs() {
        let rules = lighthouse_rules();
        // Lighthouse format: timestamp LEVEL message
        assert!(rules_match(&rules, "Dec 05 00:12:36.557 INFO Synced slot: 12345, epoch: 385"));
        assert!(rules_match(&rules, "Dec 05 00:12:36.557 ERRO Error occurred"));
        assert!(rules_match(&rules, "Dec 05 00:12:36.557 WARN Warning message"));
        assert!(rules_match(&rules, "Dec 05 00:12:36.557 CRIT Critical failure"));
        assert!(rules_match(&rules, "peers=50"));
    }

    #[test]
    fn test_prysm_rules_match_real_logs() {
        let rules = prysm_rules();
        // Prysm format: logrus-style with level=X
        assert!(rules_match(&rules, "level=info msg=\"Synced to slot 12345\""));
        assert!(rules_match(&rules, "level=error msg=\"Failed to connect\""));
        assert!(rules_match(&rules, r#"prefix="validator""#));
        assert!(rules_match(&rules, r#"msg="Submitted attestation""#));
    }

    #[test]
    fn test_teku_rules_match_real_logs() {
        let rules = teku_rules();
        // Teku uses standard log levels
        assert!(rules_match(&rules, "10:30:45.123 INFO Synced to slot 12345"));
        assert!(rules_match(&rules, "ERROR: Failed to connect"));
        assert!(rules_match(&rules, "slot=12345"));
    }

    #[test]
    fn test_nimbus_rules_match_real_logs() {
        let rules = nimbus_rules();
        // Nimbus uses short log levels: INF, WRN, ERR, etc.
        assert!(rules_match(&rules, "INF 2024-01-15 Synced"));
        assert!(rules_match(&rules, "ERR Connection failed"));
        assert!(rules_match(&rules, "WRN Low peer count"));
        assert!(rules_match(&rules, "slot=12345"));
    }

    #[test]
    fn test_grandine_rules_match_real_logs() {
        let rules = grandine_rules();
        // Grandine uses Rust log levels
        assert!(rules_match(&rules, "INFO: Syncing blocks"));
        assert!(rules_match(&rules, "ERROR: Failed to sync"));
        assert!(rules_match(&rules, "slot=12345"));
    }

    #[test]
    fn test_lambda_rules_match_real_logs() {
        let rules = lambda_rules();
        // Lambda uses Elixir logger format
        assert!(rules_match(&rules, "[info] Syncing from peer"));
        assert!(rules_match(&rules, "[error] GenServer crashed"));
        assert!(rules_match(&rules, "12:34:56.789")); // Elixir timestamp
        assert!(rules_match(&rules, "Lambda.Beacon.Node")); // Module name
    }

    // =========================================================================
    // EXECUTION CLIENTS
    // =========================================================================

    #[test]
    fn test_geth_rules_match_real_logs() {
        let rules = geth_rules();
        // Geth format: LEVEL [MM-DD|HH:MM:SS.mmm] message
        assert!(rules_match(&rules, "INFO [12-05|10:30:45.123] Imported new chain segment number=19630289"));
        assert!(rules_match(&rules, "[12-05|10:30:45.123]")); // Timestamp
        assert!(rules_match(&rules, "number=19630289")); // Block number
        assert!(rules_match(&rules, "txs=150")); // Transaction count
        assert!(rules_match(&rules, "forkchoice updated")); // FCU
    }

    #[test]
    fn test_nethermind_rules_match_real_logs() {
        let rules = nethermind_rules();
        // Nethermind uses .NET style logging
        assert!(rules_match(&rules, "Info Processed block"));
        assert!(rules_match(&rules, "Error Connection failed"));
        assert!(rules_match(&rules, "number=19630289"));
    }

    #[test]
    fn test_besu_rules_match_real_logs() {
        let rules = besu_rules();
        // Besu uses standard log levels
        assert!(rules_match(&rules, "INFO Processing transactions"));
        assert!(rules_match(&rules, "ERROR Block validation failed"));
        assert!(rules_match(&rules, "Block #19630289")); // Block number format
    }

    #[test]
    fn test_erigon_rules_match_real_logs() {
        let rules = erigon_rules();
        // Erigon uses lvl=X format
        assert!(rules_match(&rules, "lvl=info msg=\"Syncing\""));
        assert!(rules_match(&rules, "lvl=eror msg=\"Error\""));
        assert!(rules_match(&rules, "lvl=warn msg=\"Warning\""));
        assert!(rules_match(&rules, "number=19630289"));
    }

    #[test]
    fn test_reth_rules_match_real_logs() {
        let rules = reth_rules();
        // Reth uses Rust tracing format
        assert!(rules_match(&rules, "INFO reth::node Syncing"));
        assert!(rules_match(&rules, "ERROR Failed to connect"));
        assert!(rules_match(&rules, "stage=Headers"));
        assert!(rules_match(&rules, "progress=50.5%"));
        assert!(rules_match(&rules, "number=19630289"));
    }

    #[test]
    fn test_mana_rules_match_real_logs() {
        let rules = mana_rules();
        // Mana uses Elixir format with specific module patterns
        assert!(rules_match(&rules, "[info] Block imported"));
        assert!(rules_match(&rules, "[error] EVM execution failed"));
        assert!(rules_match(&rules, "Blockchain.Block"));
        assert!(rules_match(&rules, "MerklePatriciaTree"));
        assert!(rules_match(&rules, "eth_getBlockByNumber")); // JSON-RPC
    }

    // =========================================================================
    // MIDDLEWARE
    // =========================================================================

    #[test]
    fn test_charon_rules_match_real_logs() {
        let rules = charon_rules();
        // Charon DVT middleware
        assert!(rules_match(&rules, "INFO QBFT consensus reached"));
        assert!(rules_match(&rules, "pre-prepare message received"));
        assert!(rules_match(&rules, "prepare phase complete"));
        assert!(rules_match(&rules, "commit accepted"));
        assert!(rules_match(&rules, "threshold signature"));
        assert!(rules_match(&rules, "partial_sig received"));
        assert!(rules_match(&rules, "peer=abc123"));
    }

    #[test]
    fn test_mevboost_rules_match_real_logs() {
        let rules = mevboost_rules();
        // MEV-Boost relay logs
        assert!(rules_match(&rules, "slot=12345"));
        assert!(rules_match(&rules, "value=1.234"));
        assert!(rules_match(&rules, "bid=0.5"));
        assert!(rules_match(&rules, "block_value=2.0"));
        assert!(rules_match(&rules, "relay=flashbots"));
        assert!(rules_match(&rules, "builder=builder0x69"));
        assert!(rules_match(&rules, "builder_pubkey=0xabcdef1234567890"));
        assert!(rules_match(&rules, "1.5 ETH")); // MEV value
    }

    // =========================================================================
    // METADATA AND LOOKUP
    // =========================================================================

    #[test]
    fn test_rules_for_all_clients() {
        // All clients should return rules
        for client in ALL_CLIENTS {
            let rules = rules_for(client.name);
            assert!(rules.is_some(), "rules_for({}) should return Some", client.name);
            assert!(!rules.unwrap().is_empty(), "rules for {} should not be empty", client.name);
        }
    }

    #[test]
    fn test_meta_for_all_clients() {
        // All clients should have metadata
        for client in ALL_CLIENTS {
            let meta = meta_for(client.name);
            assert!(meta.is_some(), "meta_for({}) should return Some", client.name);
        }
    }

    #[test]
    fn test_rules_for_case_insensitive() {
        assert!(rules_for("LIGHTHOUSE").is_some());
        assert!(rules_for("lighthouse").is_some());
        assert!(rules_for("Lighthouse").is_some());
        assert!(rules_for("LiGhThOuSe").is_some());
    }

    #[test]
    fn test_rules_for_mevboost_aliases() {
        // MEV-Boost has multiple name variants
        assert!(rules_for("mev-boost").is_some());
        assert!(rules_for("mevboost").is_some());
        assert!(rules_for("mev_boost").is_some());
    }

    #[test]
    fn test_rules_for_unknown_returns_none() {
        assert!(rules_for("unknown_client").is_none());
        assert!(rules_for("").is_none());
    }

    #[test]
    fn test_all_clients_count() {
        // Verify we have exactly 15 clients
        assert_eq!(ALL_CLIENTS.len(), 15);
    }

    #[test]
    fn test_client_metadata_fields() {
        // Spot check metadata completeness
        assert_eq!(LIGHTHOUSE.name, "Lighthouse");
        assert_eq!(LIGHTHOUSE.layer, Layer::Consensus);
        assert!(!LIGHTHOUSE.detect_patterns.is_empty());
        assert!(LIGHTHOUSE.brand_color.starts_with('#'));
    }
}
