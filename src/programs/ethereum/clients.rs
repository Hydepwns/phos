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

define_client!(
    LIGHTHOUSE,
    "Lighthouse",
    "Ethereum consensus client in Rust",
    Layer::Consensus,
    "Rust",
    "https://lighthouse.sigmaprime.io/",
    [
        "lighthouse",
        "lighthouse-bn",
        "lighthouse-vc",
        "lighthouse-validator",
        "lighthouse_beacon",
        "lighthouse_validator",
        "lighthouse-consensus",
        "lighthouse.log"
    ],
    "#9933FF"
);

define_client!(
    PRYSM,
    "Prysm",
    "Ethereum consensus client in Go",
    Layer::Consensus,
    "Go",
    "https://prysmaticlabs.com/",
    [
        "prysm",
        "beacon-chain",
        "validator",
        "prysm-bn",
        "prysm-vc",
        "prysm_beacon",
        "prysm_validator",
        "prysm-consensus",
        "prysm.log"
    ],
    "#22CC88"
);

define_client!(
    TEKU,
    "Teku",
    "Ethereum consensus client in Java",
    Layer::Consensus,
    "Java",
    "https://consensys.net/knowledge-base/ethereum-2/teku/",
    [
        "teku",
        "teku-bn",
        "teku-vc",
        "teku_beacon",
        "teku_validator",
        "teku-consensus",
        "teku.log"
    ],
    "#3366FF"
);

define_client!(
    NIMBUS,
    "Nimbus",
    "Ethereum consensus client in Nim",
    Layer::Consensus,
    "Nim",
    "https://nimbus.team/",
    [
        "nimbus",
        "nimbus-bn",
        "nimbus-vc",
        "nimbus_beacon",
        "nimbus_validator",
        "nimbus-eth2",
        "nimbus-consensus",
        "nimbus.log"
    ],
    "#CC9933"
);

define_client!(
    LODESTAR,
    "Lodestar",
    "Ethereum consensus client in TypeScript",
    Layer::Consensus,
    "TypeScript",
    "https://lodestar.chainsafe.io/",
    [
        "lodestar",
        "lodestar-bn",
        "lodestar-vc",
        "lodestar_beacon",
        "lodestar_validator",
        "lodestar-consensus",
        "lodestar.log"
    ],
    "#AA44FF"
);

define_client!(
    GRANDINE,
    "Grandine",
    "High-performance consensus client in Rust",
    Layer::Consensus,
    "Rust",
    "https://grandine.io/",
    [
        "grandine",
        "grandine-bn",
        "grandine-vc",
        "grandine_beacon",
        "grandine-consensus",
        "grandine.log"
    ],
    "#FF6633"
);

define_client!(
    LAMBDA,
    "Lambda",
    "Ethereum consensus client in Elixir",
    Layer::Consensus,
    "Elixir",
    "https://github.com/lambdaclass/lambda_ethereum_consensus",
    [
        "lambda_ethereum",
        "lambda-bn",
        "lambda-consensus",
        "lambda.log"
    ],
    "#9966FF"
);

define_client!(
    GETH,
    "Geth",
    "Go Ethereum - the official Go implementation",
    Layer::Execution,
    "Go",
    "https://geth.ethereum.org/",
    [
        "geth",
        "geth-el",
        "geth_execution",
        "go-ethereum",
        "geth.log"
    ],
    "#6699FF"
);

define_client!(
    NETHERMIND,
    "Nethermind",
    "Ethereum client in .NET",
    Layer::Execution,
    ".NET",
    "https://nethermind.io/",
    [
        "nethermind",
        "nethermind-el",
        "nethermind_execution",
        "nethermind.log"
    ],
    "#33CCCC"
);

define_client!(
    BESU,
    "Besu",
    "Hyperledger Besu - enterprise Ethereum client",
    Layer::Execution,
    "Java",
    "https://besu.hyperledger.org/",
    [
        "besu",
        "besu-el",
        "besu_execution",
        "hyperledger-besu",
        "besu.log"
    ],
    "#009999"
);

define_client!(
    ERIGON,
    "Erigon",
    "Efficiency-focused Ethereum client",
    Layer::Execution,
    "Go",
    "https://github.com/erigontech/erigon",
    ["erigon", "erigon-el", "erigon_execution", "erigon.log"],
    "#66CC33"
);

define_client!(
    RETH,
    "Reth",
    "Modular Ethereum client in Rust by Paradigm",
    Layer::Execution,
    "Rust",
    "https://paradigmxyz.github.io/reth/",
    ["reth", "reth-el", "reth_execution", "reth.log"],
    "#FF9966"
);

define_client!(
    MANA,
    "Mana",
    "Full Ethereum client (EL+CL) in Elixir with distributed features",
    Layer::Full,
    "Elixir",
    "https://github.com/axol-io/mana",
    ["mana", "mana-el", "mana-cl", "mana_node", "mana.log"],
    "#CC66FF"
);

define_client!(
    CHARON,
    "Charon",
    "Obol distributed validator middleware",
    Layer::Middleware,
    "Go",
    "https://obol.tech/",
    ["charon", "charon-dv", "obol-charon", "charon.log"],
    "#6633FF"
);

define_client!(
    MEVBOOST,
    "MEV-Boost",
    "Flashbots MEV relay",
    Layer::Middleware,
    "Go",
    "https://boost.flashbots.net/",
    ["mev-boost", "mev_boost", "mevboost", "mev-boost.log"],
    "#FF6699"
);

/// All client metadata in order.
pub const ALL_CLIENTS: &[&ClientMeta] = &[
    &LIGHTHOUSE,
    &PRYSM,
    &TEKU,
    &NIMBUS,
    &LODESTAR,
    &GRANDINE,
    &LAMBDA,
    &GETH,
    &NETHERMIND,
    &BESU,
    &ERIGON,
    &RETH,
    &MANA,
    &CHARON,
    &MEVBOOST,
];

// =============================================================================
// CONSENSUS LAYER CLIENTS
// =============================================================================

pub fn lodestar_rules() -> Result<Vec<Rule>, regex::Error> {
    let lodestar_specific = vec![
        Rule::new(r"Eph\s+\d+/\d+")?
            .hex(patterns::EPOCH_COLOR)
            .build(),
        Rule::new(r"slot=\d+")?.hex(patterns::SLOT_COLOR).build(),
        Rule::new(r"epoch=\d+")?.hex(patterns::EPOCH_COLOR).build(),
        Rule::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}")?
            .semantic(SemanticColor::Timestamp)
            .build(),
    ];

    Ok(patterns::lodestar_log_levels()?
        .into_iter()
        .chain(lodestar_specific)
        .chain(patterns::consensus_patterns()?)
        .collect())
}

pub fn lighthouse_rules() -> Result<Vec<Rule>, regex::Error> {
    let timestamp = Rule::new(r"\w{3}\s+\d{2}\s+\d{2}:\d{2}:\d{2}\.\d{3}")?
        .semantic(SemanticColor::Timestamp)
        .build();

    Ok(patterns::lighthouse_log_levels()?
        .into_iter()
        .chain(std::iter::once(timestamp))
        .chain(patterns::consensus_patterns()?)
        .collect())
}

pub fn prysm_rules() -> Result<Vec<Rule>, regex::Error> {
    let prysm_specific = vec![
        Rule::new(r#"msg="[^"]*""#)?
            .semantic(SemanticColor::String)
            .build(),
        Rule::new(r#"prefix="[^"]*""#)?.named("cyan").build(),
    ];

    Ok(patterns::prysm_log_levels()?
        .into_iter()
        .chain(prysm_specific)
        .chain(patterns::consensus_patterns()?)
        .collect())
}

pub fn teku_rules() -> Result<Vec<Rule>, regex::Error> {
    let timestamp = Rule::new(r"\d{2}:\d{2}:\d{2}\.\d{3}")?
        .semantic(SemanticColor::Timestamp)
        .build();

    Ok(patterns::rust_log_levels()?
        .into_iter()
        .chain(std::iter::once(timestamp))
        .chain(patterns::consensus_patterns()?)
        .collect())
}

pub fn nimbus_rules() -> Result<Vec<Rule>, regex::Error> {
    Ok(patterns::nimbus_log_levels()?
        .into_iter()
        .chain(patterns::consensus_patterns()?)
        .collect())
}

pub fn grandine_rules() -> Result<Vec<Rule>, regex::Error> {
    Ok(patterns::rust_log_levels()?
        .into_iter()
        .chain(patterns::consensus_patterns()?)
        .collect())
}

pub fn lambda_rules() -> Result<Vec<Rule>, regex::Error> {
    elixir_common_rules()
}

// =============================================================================
// EXECUTION LAYER CLIENTS
// =============================================================================

pub fn geth_rules() -> Result<Vec<Rule>, regex::Error> {
    let geth_specific = vec![
        Rule::new(r"\[\d{2}-\d{2}\|\d{2}:\d{2}:\d{2}\.\d{3}\]")?
            .semantic(SemanticColor::Timestamp)
            .build(),
        Rule::new(r"number=\d+")?
            .hex(patterns::BLOCK_NUMBER_COLOR)
            .build(),
        Rule::new(r"txs=\d+")?
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"\b(forkchoice|FCU)\b")?
            .hex(patterns::FINALITY_COLOR)
            .bold()
            .build(),
    ];

    Ok(patterns::rust_log_levels()?
        .into_iter()
        .chain(geth_specific)
        .chain(patterns::execution_patterns()?)
        .collect())
}

pub fn nethermind_rules() -> Result<Vec<Rule>, regex::Error> {
    let block_num = Rule::new(r"number=\d+")?
        .hex(patterns::BLOCK_NUMBER_COLOR)
        .build();

    Ok(patterns::dotnet_log_levels()?
        .into_iter()
        .chain(std::iter::once(block_num))
        .chain(patterns::execution_patterns()?)
        .collect())
}

pub fn besu_rules() -> Result<Vec<Rule>, regex::Error> {
    let block_num = Rule::new(r"Block #\d+")?
        .hex(patterns::BLOCK_NUMBER_COLOR)
        .build();

    Ok(patterns::rust_log_levels()?
        .into_iter()
        .chain(std::iter::once(block_num))
        .chain(patterns::execution_patterns()?)
        .collect())
}

pub fn erigon_rules() -> Result<Vec<Rule>, regex::Error> {
    let block_num = Rule::new(r"number=\d+")?
        .hex(patterns::BLOCK_NUMBER_COLOR)
        .build();

    Ok(patterns::erigon_log_levels()?
        .into_iter()
        .chain(std::iter::once(block_num))
        .chain(patterns::execution_patterns()?)
        .collect())
}

pub fn reth_rules() -> Result<Vec<Rule>, regex::Error> {
    let reth_specific = vec![
        Rule::new(r"stage=\w+")?
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"progress=\d+\.\d+%")?
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"number=\d+")?
            .hex(patterns::BLOCK_NUMBER_COLOR)
            .build(),
    ];

    Ok(patterns::rust_log_levels()?
        .into_iter()
        .chain(reth_specific)
        .chain(patterns::execution_patterns()?)
        .collect())
}

pub fn mana_rules() -> Result<Vec<Rule>, regex::Error> {
    let mana_specific = vec![
        Rule::new(r"\b(Blockchain|EVM|ExWire|MerklePatriciaTree|JSONRPC2)\b")?
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"\b(L2|layer2|rollup|optimistic|zk-?proof)\b")?
            .hex(patterns::BUILDER_COLOR)
            .build(),
        Rule::new(r"\b(verkle|crdt|antidote|distributed)\b")?
            .hex(patterns::COMMITTEE_COLOR)
            .build(),
        Rule::new(r"\b(eth_\w+|web3_\w+|net_\w+)\b")?
            .semantic(SemanticColor::Value)
            .build(),
    ];

    Ok(elixir_common_rules()?
        .into_iter()
        .chain(mana_specific)
        .collect())
}

// =============================================================================
// MIDDLEWARE
// =============================================================================

pub fn charon_rules() -> Result<Vec<Rule>, regex::Error> {
    let charon_specific = vec![
        Rule::new(r"\bQBFT\b")?
            .hex(patterns::COMMITTEE_COLOR)
            .bold()
            .build(),
        Rule::new(r"\b(pre-prepare|prepare|commit|round-change)\b")?
            .hex(patterns::COMMITTEE_COLOR)
            .build(),
        Rule::new(r"\bthreshold\b")?
            .hex(patterns::ATTESTATION_COLOR)
            .build(),
        Rule::new(r"partial[_\s]?sig")?
            .hex(patterns::ATTESTATION_COLOR)
            .build(),
        Rule::new(r"peer=\w+")?.hex(patterns::PEER_ID_COLOR).build(),
        Rule::new(r"node=\d+")?
            .semantic(SemanticColor::Number)
            .build(),
    ];

    Ok(patterns::rust_log_levels()?
        .into_iter()
        .chain(charon_specific)
        .chain(patterns::consensus_patterns()?)
        .collect())
}

pub fn mevboost_rules() -> Result<Vec<Rule>, regex::Error> {
    let mevboost_specific = vec![
        Rule::new(r"slot=\d+")?.hex(patterns::SLOT_COLOR).build(),
        Rule::new(r"value=[\d\.]+")?
            .hex(patterns::MEV_VALUE_COLOR)
            .bold()
            .build(),
        Rule::new(r"bid=[\d\.]+")?
            .hex(patterns::MEV_VALUE_COLOR)
            .bold()
            .build(),
        Rule::new(r"block_value=[\d\.]+")?
            .hex(patterns::MEV_VALUE_COLOR)
            .bold()
            .build(),
        Rule::new(r"relay=\S+")?.hex(patterns::RELAY_COLOR).build(),
        Rule::new(r"builder=\S+")?
            .hex(patterns::BUILDER_COLOR)
            .build(),
        Rule::new(r"builder_pubkey=0x[a-fA-F0-9]+")?
            .hex(patterns::BUILDER_COLOR)
            .build(),
        patterns::hash_rule()?,
        patterns::success_rule()?,
        patterns::failure_rule()?,
    ];

    Ok(patterns::rust_log_levels()?
        .into_iter()
        .chain(mevboost_specific)
        .chain(patterns::mev_patterns()?)
        .chain(std::iter::once(patterns::number_rule()?))
        .collect())
}

// =============================================================================
// ELIXIR CLIENT PATTERNS
// =============================================================================

fn elixir_common_rules() -> Result<Vec<Rule>, regex::Error> {
    let elixir_specific = vec![
        Rule::new(r"\d{2}:\d{2}:\d{2}\.\d{3}")?
            .semantic(SemanticColor::Timestamp)
            .build(),
        Rule::new(r"\b[A-Z][a-zA-Z0-9]*(\.[A-Z][a-zA-Z0-9]*)+")?
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"#PID<[\d\.]+>")?
            .hex(patterns::PEER_ID_COLOR)
            .build(),
        Rule::new(r":\w+")?.semantic(SemanticColor::Value).build(),
        Rule::new(r#""[^"]*""#)?
            .semantic(SemanticColor::String)
            .build(),
    ];

    Ok(patterns::elixir_log_levels()?
        .into_iter()
        .chain(elixir_specific)
        .chain(patterns::consensus_patterns()?)
        .collect())
}

/// Get rules for a client by name (case-insensitive).
pub fn rules_for(name: &str) -> Option<Result<Vec<Rule>, regex::Error>> {
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
#[must_use]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn rules_match(rules: &[Rule], input: &str) -> bool {
        rules.iter().any(|r| r.is_match(input))
    }

    #[test]
    fn test_lodestar_rules_match_real_logs() {
        let rules = lodestar_rules().unwrap();
        assert!(rules_match(
            &rules,
            "Dec 05 00:12:36.557[] info: Synced - slot 5375712 - Peers 47"
        ));
        assert!(rules_match(
            &rules,
            "Dec 05 00:12:36.000[] error: Failed to connect"
        ));
        assert!(rules_match(&rules, "Eph 167991/6"));
        assert!(rules_match(&rules, "slot=12345"));
        assert!(rules_match(&rules, "epoch=385"));
    }

    #[test]
    fn test_lighthouse_rules_match_real_logs() {
        let rules = lighthouse_rules().unwrap();
        assert!(rules_match(
            &rules,
            "Dec 05 00:12:36.557 INFO Synced slot: 12345, epoch: 385"
        ));
        assert!(rules_match(
            &rules,
            "Dec 05 00:12:36.557 ERRO Error occurred"
        ));
        assert!(rules_match(
            &rules,
            "Dec 05 00:12:36.557 WARN Warning message"
        ));
        assert!(rules_match(
            &rules,
            "Dec 05 00:12:36.557 CRIT Critical failure"
        ));
        assert!(rules_match(&rules, "peers=50"));
    }

    #[test]
    fn test_prysm_rules_match_real_logs() {
        let rules = prysm_rules().unwrap();
        assert!(rules_match(
            &rules,
            "level=info msg=\"Synced to slot 12345\""
        ));
        assert!(rules_match(&rules, "level=error msg=\"Failed to connect\""));
        assert!(rules_match(&rules, r#"prefix="validator""#));
        assert!(rules_match(&rules, r#"msg="Submitted attestation""#));
    }

    #[test]
    fn test_teku_rules_match_real_logs() {
        let rules = teku_rules().unwrap();
        assert!(rules_match(
            &rules,
            "10:30:45.123 INFO Synced to slot 12345"
        ));
        assert!(rules_match(&rules, "ERROR: Failed to connect"));
        assert!(rules_match(&rules, "slot=12345"));
    }

    #[test]
    fn test_nimbus_rules_match_real_logs() {
        let rules = nimbus_rules().unwrap();
        assert!(rules_match(&rules, "INF 2024-01-15 Synced"));
        assert!(rules_match(&rules, "ERR Connection failed"));
        assert!(rules_match(&rules, "WRN Low peer count"));
        assert!(rules_match(&rules, "slot=12345"));
    }

    #[test]
    fn test_grandine_rules_match_real_logs() {
        let rules = grandine_rules().unwrap();
        assert!(rules_match(&rules, "INFO: Syncing blocks"));
        assert!(rules_match(&rules, "ERROR: Failed to sync"));
        assert!(rules_match(&rules, "slot=12345"));
    }

    #[test]
    fn test_lambda_rules_match_real_logs() {
        let rules = lambda_rules().unwrap();
        assert!(rules_match(&rules, "[info] Syncing from peer"));
        assert!(rules_match(&rules, "[error] GenServer crashed"));
        assert!(rules_match(&rules, "12:34:56.789"));
        assert!(rules_match(&rules, "Lambda.Beacon.Node"));
    }

    #[test]
    fn test_geth_rules_match_real_logs() {
        let rules = geth_rules().unwrap();
        assert!(rules_match(
            &rules,
            "INFO [12-05|10:30:45.123] Imported new chain segment number=19630289"
        ));
        assert!(rules_match(&rules, "[12-05|10:30:45.123]"));
        assert!(rules_match(&rules, "number=19630289"));
        assert!(rules_match(&rules, "txs=150"));
        assert!(rules_match(&rules, "forkchoice updated"));
    }

    #[test]
    fn test_nethermind_rules_match_real_logs() {
        let rules = nethermind_rules().unwrap();
        assert!(rules_match(&rules, "Info Processed block"));
        assert!(rules_match(&rules, "Error Connection failed"));
        assert!(rules_match(&rules, "number=19630289"));
    }

    #[test]
    fn test_besu_rules_match_real_logs() {
        let rules = besu_rules().unwrap();
        assert!(rules_match(&rules, "INFO Processing transactions"));
        assert!(rules_match(&rules, "ERROR Block validation failed"));
        assert!(rules_match(&rules, "Block #19630289"));
    }

    #[test]
    fn test_erigon_rules_match_real_logs() {
        let rules = erigon_rules().unwrap();
        assert!(rules_match(&rules, "lvl=info msg=\"Syncing\""));
        assert!(rules_match(&rules, "lvl=eror msg=\"Error\""));
        assert!(rules_match(&rules, "lvl=warn msg=\"Warning\""));
        assert!(rules_match(&rules, "number=19630289"));
    }

    #[test]
    fn test_reth_rules_match_real_logs() {
        let rules = reth_rules().unwrap();
        assert!(rules_match(&rules, "INFO reth::node Syncing"));
        assert!(rules_match(&rules, "ERROR Failed to connect"));
        assert!(rules_match(&rules, "stage=Headers"));
        assert!(rules_match(&rules, "progress=50.5%"));
        assert!(rules_match(&rules, "number=19630289"));
    }

    #[test]
    fn test_mana_rules_match_real_logs() {
        let rules = mana_rules().unwrap();
        assert!(rules_match(&rules, "[info] Block imported"));
        assert!(rules_match(&rules, "[error] EVM execution failed"));
        assert!(rules_match(&rules, "Blockchain.Block"));
        assert!(rules_match(&rules, "MerklePatriciaTree"));
        assert!(rules_match(&rules, "eth_getBlockByNumber"));
    }

    #[test]
    fn test_charon_rules_match_real_logs() {
        let rules = charon_rules().unwrap();
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
        let rules = mevboost_rules().unwrap();
        assert!(rules_match(&rules, "slot=12345"));
        assert!(rules_match(&rules, "value=1.234"));
        assert!(rules_match(&rules, "bid=0.5"));
        assert!(rules_match(&rules, "block_value=2.0"));
        assert!(rules_match(&rules, "relay=flashbots"));
        assert!(rules_match(&rules, "builder=builder0x69"));
        assert!(rules_match(&rules, "builder_pubkey=0xabcdef1234567890"));
        assert!(rules_match(&rules, "1.5 ETH"));
    }

    #[test]
    fn test_rules_for_all_clients() {
        for client in ALL_CLIENTS {
            let rules = rules_for(client.name);
            assert!(
                rules.is_some(),
                "rules_for({}) should return Some",
                client.name
            );
            let rules = rules.unwrap().unwrap();
            assert!(
                !rules.is_empty(),
                "rules for {} should not be empty",
                client.name
            );
        }
    }

    #[test]
    fn test_meta_for_all_clients() {
        for client in ALL_CLIENTS {
            let meta = meta_for(client.name);
            assert!(
                meta.is_some(),
                "meta_for({}) should return Some",
                client.name
            );
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
        assert_eq!(ALL_CLIENTS.len(), 15);
    }

    #[test]
    fn test_client_metadata_fields() {
        assert_eq!(LIGHTHOUSE.name, "Lighthouse");
        assert_eq!(LIGHTHOUSE.layer, Layer::Consensus);
        assert!(!LIGHTHOUSE.detect_patterns.is_empty());
        assert!(LIGHTHOUSE.brand_color.starts_with('#'));
    }
}
