//! Shared Ethereum log patterns used across multiple clients.

use crate::colors::SemanticColor;
use crate::rule::Rule;

// =========================================================================
// ETHEREUM DOMAIN COLORS (hex values)
// =========================================================================

pub const HASH_COLOR: &str = "#88AAFF";
pub const ADDRESS_COLOR: &str = "#FFAA88";
pub const SLOT_COLOR: &str = "#88FFAA";
pub const EPOCH_COLOR: &str = "#AAFFFF";
pub const BLOCK_NUMBER_COLOR: &str = "#FFAAFF";
pub const PEER_ID_COLOR: &str = "#AAAAFF";
pub const VALIDATOR_COLOR: &str = "#AA88FF";
pub const PUBKEY_COLOR: &str = "#88DDFF";
pub const DUTY_COLOR: &str = "#FFBB55";
pub const COMMITTEE_COLOR: &str = "#88AADD";
pub const FINALITY_COLOR: &str = "#88FF88";
pub const ROOT_COLOR: &str = "#88DDFF";
pub const ATTESTATION_COLOR: &str = "#FF88DD";
pub const MEV_VALUE_COLOR: &str = "#FFDD55";
pub const RELAY_COLOR: &str = "#999999";
pub const BUILDER_COLOR: &str = "#FF99BB";
pub const SYNCING_COLOR: &str = "#FFFF55";

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

/// Elixir Logger-style log levels (`[error]`, `[warning]`, `[info]`, `[debug]`).
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
        .hex(HASH_COLOR)
        .build()
}

pub fn address_rule() -> Rule {
    Rule::new(r"0x[a-fA-F0-9]{40}")
        .unwrap()
        .hex(ADDRESS_COLOR)
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
        .hex(SLOT_COLOR)
        .build()
}

pub fn epoch_rule() -> Rule {
    Rule::new(r"\bepoch[=:\s]+(\d+)")
        .unwrap()
        .hex(EPOCH_COLOR)
        .build()
}

pub fn peers_rule() -> Rule {
    Rule::new(r"\bpeers?[=:\s]+(\d+)")
        .unwrap()
        .hex(PEER_ID_COLOR)
        .build()
}

pub fn syncing_rule() -> Rule {
    Rule::new(r"\b(Syncing|Synced|syncing|synced)\b")
        .unwrap()
        .hex(SYNCING_COLOR)
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
        .hex(VALIDATOR_COLOR)
        .build()
}

pub fn pubkey_rule() -> Rule {
    Rule::new(r"pubkey[=:\s]+0x[a-fA-F0-9]{8,}")
        .unwrap()
        .hex(PUBKEY_COLOR)
        .build()
}

pub fn duty_rule() -> Rule {
    Rule::new(r"\b(attester|proposer|sync_committee|aggregator|attesting|proposing)\b")
        .unwrap()
        .hex(DUTY_COLOR)
        .build()
}

pub fn committee_rule() -> Rule {
    Rule::new(r"\b(committee|subnet)[=:\s]+(\d+)")
        .unwrap()
        .hex(COMMITTEE_COLOR)
        .build()
}

pub fn finality_rule() -> Rule {
    Rule::new(r"\b(finalized|justified|finality|checkpoint)\b")
        .unwrap()
        .hex(FINALITY_COLOR)
        .build()
}

pub fn root_rule() -> Rule {
    Rule::new(r"(state_root|block_root|root)[=:\s]+0x[a-fA-F0-9]{64}")
        .unwrap()
        .hex(ROOT_COLOR)
        .build()
}

pub fn attestation_rule() -> Rule {
    Rule::new(r"\b(attestation|attest|attested)\b")
        .unwrap()
        .hex(ATTESTATION_COLOR)
        .build()
}

pub fn mev_value_rule() -> Rule {
    Rule::new(r"(\d+\.?\d*)\s*(ETH|Gwei|gwei|wei)")
        .unwrap()
        .hex(MEV_VALUE_COLOR)
        .bold()
        .build()
}

pub fn relay_rule() -> Rule {
    Rule::new(r"\b(flashbots|bloxroute|blocknative|eden|manifold|ultrasound|agnostic)\b")
        .unwrap()
        .hex(RELAY_COLOR)
        .build()
}

pub fn builder_rule() -> Rule {
    Rule::new(r"builder[=:\s]+(\S+)")
        .unwrap()
        .hex(BUILDER_COLOR)
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
