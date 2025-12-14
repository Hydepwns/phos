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
    detect_patterns: &["lighthouse"],
    brand_color: "#9933FF",
};

pub const PRYSM: ClientMeta = ClientMeta {
    name: "Prysm",
    description: "Ethereum consensus client in Go",
    layer: Layer::Consensus,
    language: "Go",
    website: "https://prysmaticlabs.com/",
    detect_patterns: &["prysm", "beacon-chain", "validator"],
    brand_color: "#22CC88",
};

pub const TEKU: ClientMeta = ClientMeta {
    name: "Teku",
    description: "Ethereum consensus client in Java",
    layer: Layer::Consensus,
    language: "Java",
    website: "https://consensys.net/knowledge-base/ethereum-2/teku/",
    detect_patterns: &["teku"],
    brand_color: "#3366FF",
};

pub const NIMBUS: ClientMeta = ClientMeta {
    name: "Nimbus",
    description: "Ethereum consensus client in Nim",
    layer: Layer::Consensus,
    language: "Nim",
    website: "https://nimbus.team/",
    detect_patterns: &["nimbus"],
    brand_color: "#CC9933",
};

pub const LODESTAR: ClientMeta = ClientMeta {
    name: "Lodestar",
    description: "Ethereum consensus client in TypeScript",
    layer: Layer::Consensus,
    language: "TypeScript",
    website: "https://lodestar.chainsafe.io/",
    detect_patterns: &["lodestar"],
    brand_color: "#AA44FF",
};

pub const GRANDINE: ClientMeta = ClientMeta {
    name: "Grandine",
    description: "High-performance consensus client in Rust",
    layer: Layer::Consensus,
    language: "Rust",
    website: "https://grandine.io/",
    detect_patterns: &["grandine"],
    brand_color: "#FF6633",
};

pub const LAMBDA: ClientMeta = ClientMeta {
    name: "Lambda",
    description: "Ethereum consensus client in Elixir",
    layer: Layer::Consensus,
    language: "Elixir",
    website: "https://github.com/lambdaclass/lambda_ethereum_consensus",
    detect_patterns: &["lambda_ethereum"],
    brand_color: "#9966FF",
};

pub const GETH: ClientMeta = ClientMeta {
    name: "Geth",
    description: "Go Ethereum - the official Go implementation",
    layer: Layer::Execution,
    language: "Go",
    website: "https://geth.ethereum.org/",
    detect_patterns: &["geth"],
    brand_color: "#6699FF",
};

pub const NETHERMIND: ClientMeta = ClientMeta {
    name: "Nethermind",
    description: "Ethereum client in .NET",
    layer: Layer::Execution,
    language: ".NET",
    website: "https://nethermind.io/",
    detect_patterns: &["nethermind"],
    brand_color: "#33CCCC",
};

pub const BESU: ClientMeta = ClientMeta {
    name: "Besu",
    description: "Hyperledger Besu - enterprise Ethereum client",
    layer: Layer::Execution,
    language: "Java",
    website: "https://besu.hyperledger.org/",
    detect_patterns: &["besu"],
    brand_color: "#009999",
};

pub const ERIGON: ClientMeta = ClientMeta {
    name: "Erigon",
    description: "Efficiency-focused Ethereum client",
    layer: Layer::Execution,
    language: "Go",
    website: "https://github.com/erigontech/erigon",
    detect_patterns: &["erigon"],
    brand_color: "#66CC33",
};

pub const RETH: ClientMeta = ClientMeta {
    name: "Reth",
    description: "Modular Ethereum client in Rust by Paradigm",
    layer: Layer::Execution,
    language: "Rust",
    website: "https://paradigmxyz.github.io/reth/",
    detect_patterns: &["reth"],
    brand_color: "#FF9966",
};

pub const MANA: ClientMeta = ClientMeta {
    name: "Mana",
    description: "Full Ethereum client (EL+CL) in Elixir with distributed features",
    layer: Layer::Full,
    language: "Elixir",
    website: "https://github.com/axol-io/mana",
    detect_patterns: &["mana"],
    brand_color: "#CC66FF",
};

pub const CHARON: ClientMeta = ClientMeta {
    name: "Charon",
    description: "Obol distributed validator middleware",
    layer: Layer::Middleware,
    language: "Go",
    website: "https://obol.tech/",
    detect_patterns: &["charon"],
    brand_color: "#6633FF",
};

pub const MEVBOOST: ClientMeta = ClientMeta {
    name: "MEV-Boost",
    description: "Flashbots MEV relay",
    layer: Layer::Middleware,
    language: "Go",
    website: "https://boost.flashbots.net/",
    detect_patterns: &["mev-boost", "mev_boost", "mevboost"],
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

pub fn lodestar_rules() -> Vec<Rule> {
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

pub fn lighthouse_rules() -> Vec<Rule> {
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

pub fn prysm_rules() -> Vec<Rule> {
    let mut rules = patterns::prysm_log_levels();
    rules.extend([
        Rule::new(r#"msg="[^"]*""#).unwrap().semantic(SemanticColor::String).build(),
        Rule::new(r#"prefix="[^"]*""#).unwrap().named("cyan").build(),
    ]);
    rules.extend(patterns::consensus_patterns());
    rules
}

pub fn teku_rules() -> Vec<Rule> {
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

pub fn nimbus_rules() -> Vec<Rule> {
    let mut rules = patterns::nimbus_log_levels();
    rules.extend(patterns::consensus_patterns());
    rules
}

pub fn grandine_rules() -> Vec<Rule> {
    let mut rules = patterns::rust_log_levels();
    rules.extend(patterns::consensus_patterns());
    rules
}

pub fn lambda_rules() -> Vec<Rule> {
    elixir_common_rules()
}

// =============================================================================
// EXECUTION LAYER CLIENTS
// =============================================================================

pub fn geth_rules() -> Vec<Rule> {
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

pub fn nethermind_rules() -> Vec<Rule> {
    let mut rules = patterns::dotnet_log_levels();
    rules.push(
        Rule::new(r"number=\d+").unwrap().hex(patterns::BLOCK_NUMBER_COLOR).build(),
    );
    rules.extend(patterns::execution_patterns());
    rules
}

pub fn besu_rules() -> Vec<Rule> {
    let mut rules = patterns::rust_log_levels();
    rules.push(
        Rule::new(r"Block #\d+").unwrap().hex(patterns::BLOCK_NUMBER_COLOR).build(),
    );
    rules.extend(patterns::execution_patterns());
    rules
}

pub fn erigon_rules() -> Vec<Rule> {
    let mut rules = patterns::erigon_log_levels();
    rules.push(
        Rule::new(r"number=\d+").unwrap().hex(patterns::BLOCK_NUMBER_COLOR).build(),
    );
    rules.extend(patterns::execution_patterns());
    rules
}

pub fn reth_rules() -> Vec<Rule> {
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

pub fn mana_rules() -> Vec<Rule> {
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

pub fn charon_rules() -> Vec<Rule> {
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

pub fn mevboost_rules() -> Vec<Rule> {
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
pub fn rules_for(name: &str) -> Option<Vec<Rule>> {
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
pub fn meta_for(name: &str) -> Option<&'static ClientMeta> {
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
