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
#[must_use]
pub fn rust_log_levels() -> Vec<Rule> {
    vec![
        Rule::new(r"\bERROR\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bWARN\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"\bINFO\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bDEBUG\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\bTRACE\b")
            .unwrap()
            .semantic(SemanticColor::Trace)
            .build(),
    ]
}

/// Lighthouse-style log levels (CRIT, ERRO, WARN, INFO, DEBG, TRCE)
#[must_use]
pub fn lighthouse_log_levels() -> Vec<Rule> {
    vec![
        Rule::new(r"\bCRIT\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bERRO\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bWARN\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"\bINFO\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bDEBG\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\bTRCE\b")
            .unwrap()
            .semantic(SemanticColor::Trace)
            .build(),
    ]
}

/// Nimbus-style log levels (FTL, ERR, WRN, INF, DBG, TRC)
#[must_use]
pub fn nimbus_log_levels() -> Vec<Rule> {
    vec![
        Rule::new(r"\bFTL\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bERR\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bWRN\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"\bINF\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bDBG\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\bTRC\b")
            .unwrap()
            .semantic(SemanticColor::Trace)
            .build(),
    ]
}

/// Lodestar-style log levels (error/ERR, warn/WRN, info/INF, debug/DBG, verbose/VRB)
#[must_use]
pub fn lodestar_log_levels() -> Vec<Rule> {
    vec![
        Rule::new(r"\b(error|ERR)\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\b(warn|WRN)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"\b(info|INF)\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\b(debug|DBG)\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\b(verbose|VRB|trace)\b")
            .unwrap()
            .semantic(SemanticColor::Trace)
            .build(),
    ]
}

/// Prysm-style log levels (level=error, level=warning, etc.)
#[must_use]
pub fn prysm_log_levels() -> Vec<Rule> {
    vec![
        Rule::new(r"level=error")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"level=warning")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"level=info")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"level=debug")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"level=trace")
            .unwrap()
            .semantic(SemanticColor::Trace)
            .build(),
    ]
}

/// .NET-style log levels (Error, Warn, Info, Debug, Trace)
#[must_use]
pub fn dotnet_log_levels() -> Vec<Rule> {
    vec![
        Rule::new(r"\bError\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bWarn\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"\bInfo\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bDebug\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\bTrace\b")
            .unwrap()
            .semantic(SemanticColor::Trace)
            .build(),
    ]
}

/// Erigon-style log levels (lvl=eror, lvl=warn, etc.)
#[must_use]
pub fn erigon_log_levels() -> Vec<Rule> {
    vec![
        Rule::new(r"lvl=eror")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"lvl=warn")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"lvl=info")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"lvl=dbug")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"lvl=trce")
            .unwrap()
            .semantic(SemanticColor::Trace)
            .build(),
    ]
}

/// Elixir Logger-style log levels (`[error]`, `[warning]`, `[info]`, `[debug]`).
#[must_use]
pub fn elixir_log_levels() -> Vec<Rule> {
    vec![
        Rule::new(r"\[error\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\[warning\]")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"\[info\]")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\[debug\]")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]
}

// =========================================================================
// COMMON ETHEREUM PATTERNS
// =========================================================================

#[must_use]
pub fn hash_rule() -> Rule {
    Rule::new(r"0x[a-fA-F0-9]{8,}")
        .unwrap()
        .hex(HASH_COLOR)
        .build()
}

#[must_use]
pub fn address_rule() -> Rule {
    Rule::new(r"0x[a-fA-F0-9]{40}")
        .unwrap()
        .hex(ADDRESS_COLOR)
        .build()
}

#[must_use]
pub fn number_rule() -> Rule {
    Rule::new(r"\b\d+(\.\d+)?\b")
        .unwrap()
        .semantic(SemanticColor::Number)
        .build()
}

#[must_use]
pub fn slot_rule() -> Rule {
    Rule::new(r"\bslot[=:\s]+(\d+)")
        .unwrap()
        .hex(SLOT_COLOR)
        .build()
}

#[must_use]
pub fn epoch_rule() -> Rule {
    Rule::new(r"\bepoch[=:\s]+(\d+)")
        .unwrap()
        .hex(EPOCH_COLOR)
        .build()
}

#[must_use]
pub fn peers_rule() -> Rule {
    Rule::new(r"\bpeers?[=:\s]+(\d+)")
        .unwrap()
        .hex(PEER_ID_COLOR)
        .build()
}

#[must_use]
pub fn syncing_rule() -> Rule {
    Rule::new(r"\b(Syncing|Synced|syncing|synced)\b")
        .unwrap()
        .hex(SYNCING_COLOR)
        .build()
}

#[must_use]
pub fn success_rule() -> Rule {
    Rule::new(r"\b(success|valid|verified)\b")
        .unwrap()
        .semantic(SemanticColor::Success)
        .build()
}

#[must_use]
pub fn failure_rule() -> Rule {
    Rule::new(r"\b(failed|invalid|error|timeout|rejected)\b")
        .unwrap()
        .semantic(SemanticColor::Failure)
        .build()
}

#[must_use]
pub fn validator_rule() -> Rule {
    Rule::new(r"\b(validator|idx|validator_index)[=:\s]+(\d+)")
        .unwrap()
        .hex(VALIDATOR_COLOR)
        .build()
}

#[must_use]
pub fn pubkey_rule() -> Rule {
    Rule::new(r"pubkey[=:\s]+0x[a-fA-F0-9]{8,}")
        .unwrap()
        .hex(PUBKEY_COLOR)
        .build()
}

#[must_use]
pub fn duty_rule() -> Rule {
    Rule::new(r"\b(attester|proposer|sync_committee|aggregator|attesting|proposing)\b")
        .unwrap()
        .hex(DUTY_COLOR)
        .build()
}

#[must_use]
pub fn committee_rule() -> Rule {
    Rule::new(r"\b(committee|subnet)[=:\s]+(\d+)")
        .unwrap()
        .hex(COMMITTEE_COLOR)
        .build()
}

#[must_use]
pub fn finality_rule() -> Rule {
    Rule::new(r"\b(finalized|justified|finality|checkpoint)\b")
        .unwrap()
        .hex(FINALITY_COLOR)
        .build()
}

#[must_use]
pub fn root_rule() -> Rule {
    Rule::new(r"(state_root|block_root|root)[=:\s]+0x[a-fA-F0-9]{64}")
        .unwrap()
        .hex(ROOT_COLOR)
        .build()
}

#[must_use]
pub fn attestation_rule() -> Rule {
    Rule::new(r"\b(attestation|attest|attested)\b")
        .unwrap()
        .hex(ATTESTATION_COLOR)
        .build()
}

#[must_use]
pub fn mev_value_rule() -> Rule {
    Rule::new(r"(\d+\.?\d*)\s*(ETH|Gwei|gwei|wei)")
        .unwrap()
        .hex(MEV_VALUE_COLOR)
        .bold()
        .build()
}

#[must_use]
pub fn relay_rule() -> Rule {
    Rule::new(r"\b(flashbots|bloxroute|blocknative|eden|manifold|ultrasound|agnostic)\b")
        .unwrap()
        .hex(RELAY_COLOR)
        .build()
}

#[must_use]
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
#[must_use]
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
#[must_use]
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
#[must_use]
pub fn mev_patterns() -> Vec<Rule> {
    vec![mev_value_rule(), relay_rule(), builder_rule()]
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to check if a rule matches a string
    fn rule_matches(rule: &Rule, input: &str) -> bool {
        rule.is_match(input)
    }

    fn any_rule_matches(rules: &[Rule], input: &str) -> bool {
        rules.iter().any(|r| rule_matches(r, input))
    }

    // =========================================================================
    // LOG LEVEL TESTS
    // =========================================================================

    #[test]
    fn test_rust_log_levels_match() {
        let rules = rust_log_levels();
        assert!(any_rule_matches(&rules, "ERROR: something failed"));
        assert!(any_rule_matches(&rules, "WARN: this is a warning"));
        assert!(any_rule_matches(&rules, "INFO: informational message"));
        assert!(any_rule_matches(&rules, "DEBUG: debug output"));
        assert!(any_rule_matches(&rules, "TRACE: trace level"));
    }

    #[test]
    fn test_rust_log_levels_word_boundary() {
        let rules = rust_log_levels();
        // Should not match partial words
        assert!(!any_rule_matches(&rules, "ERRORS"));
        assert!(!any_rule_matches(&rules, "MYERROR"));
        assert!(!any_rule_matches(&rules, "INFOBOX"));
    }

    #[test]
    fn test_lighthouse_log_levels_match() {
        let rules = lighthouse_log_levels();
        assert!(any_rule_matches(&rules, "CRIT: critical error"));
        assert!(any_rule_matches(&rules, "ERRO: error message"));
        assert!(any_rule_matches(&rules, "WARN: warning"));
        assert!(any_rule_matches(&rules, "INFO: info"));
        assert!(any_rule_matches(&rules, "DEBG: debug"));
        assert!(any_rule_matches(&rules, "TRCE: trace"));
    }

    #[test]
    fn test_nimbus_log_levels_match() {
        let rules = nimbus_log_levels();
        assert!(any_rule_matches(&rules, "FTL: fatal error"));
        assert!(any_rule_matches(&rules, "ERR: error"));
        assert!(any_rule_matches(&rules, "WRN: warning"));
        assert!(any_rule_matches(&rules, "INF: info"));
        assert!(any_rule_matches(&rules, "DBG: debug"));
        assert!(any_rule_matches(&rules, "TRC: trace"));
    }

    #[test]
    fn test_lodestar_log_levels_match() {
        let rules = lodestar_log_levels();
        assert!(any_rule_matches(&rules, "error: something failed"));
        assert!(any_rule_matches(&rules, "ERR: something failed"));
        assert!(any_rule_matches(&rules, "warn: warning message"));
        assert!(any_rule_matches(&rules, "WRN: warning message"));
        assert!(any_rule_matches(&rules, "info: info"));
        assert!(any_rule_matches(&rules, "debug: debug"));
        assert!(any_rule_matches(&rules, "verbose: verbose output"));
        assert!(any_rule_matches(&rules, "trace: trace"));
    }

    #[test]
    fn test_prysm_log_levels_match() {
        let rules = prysm_log_levels();
        assert!(any_rule_matches(&rules, "level=error msg=\"failed\""));
        assert!(any_rule_matches(&rules, "level=warning msg=\"caution\""));
        assert!(any_rule_matches(&rules, "level=info msg=\"status\""));
        assert!(any_rule_matches(&rules, "level=debug msg=\"debug\""));
        assert!(any_rule_matches(&rules, "level=trace msg=\"trace\""));
    }

    #[test]
    fn test_erigon_log_levels_match() {
        let rules = erigon_log_levels();
        assert!(any_rule_matches(&rules, "lvl=eror msg=\"error\""));
        assert!(any_rule_matches(&rules, "lvl=warn msg=\"warning\""));
        assert!(any_rule_matches(&rules, "lvl=info msg=\"info\""));
        assert!(any_rule_matches(&rules, "lvl=dbug msg=\"debug\""));
        assert!(any_rule_matches(&rules, "lvl=trce msg=\"trace\""));
    }

    #[test]
    fn test_elixir_log_levels_match() {
        let rules = elixir_log_levels();
        assert!(any_rule_matches(&rules, "[error] GenServer crashed"));
        assert!(any_rule_matches(&rules, "[warning] deprecated function"));
        assert!(any_rule_matches(&rules, "[info] Application started"));
        assert!(any_rule_matches(&rules, "[debug] state change"));
    }

    // =========================================================================
    // ETHEREUM PATTERN TESTS
    // =========================================================================

    #[test]
    fn test_hash_rule_matches_hex() {
        let rule = hash_rule();
        // Should match 8+ hex chars with 0x prefix
        assert!(rule_matches(&rule, "hash=0x1234567890abcdef"));
        assert!(rule_matches(&rule, "0xabcdef12"));
        assert!(rule_matches(&rule, "0xABCDEF12")); // uppercase
        assert!(rule_matches(&rule, "0xAbCdEf12")); // mixed case
    }

    #[test]
    fn test_hash_rule_minimum_length() {
        let rule = hash_rule();
        // 8 chars minimum
        assert!(rule_matches(&rule, "0x12345678"));
        // Less than 8 should not match
        assert!(!rule_matches(&rule, "0x1234567"));
    }

    #[test]
    fn test_address_rule_matches_40_hex() {
        let rule = address_rule();
        // 40 hex chars (Ethereum address)
        assert!(rule_matches(
            &rule,
            "0x742d35Cc6634C0532925a3b844Bc9e7595f12345"
        ));
        assert!(rule_matches(
            &rule,
            "0x0000000000000000000000000000000000000000"
        ));
    }

    #[test]
    fn test_slot_rule_variations() {
        let rule = slot_rule();
        assert!(rule_matches(&rule, "slot=12345"));
        assert!(rule_matches(&rule, "slot: 12345"));
        assert!(rule_matches(&rule, "slot 12345"));
        // Should not match without number
        assert!(!rule_matches(&rule, "slot="));
        assert!(!rule_matches(&rule, "timeslot"));
    }

    #[test]
    fn test_epoch_rule_variations() {
        let rule = epoch_rule();
        assert!(rule_matches(&rule, "epoch=385"));
        assert!(rule_matches(&rule, "epoch: 385"));
        assert!(rule_matches(&rule, "epoch 385"));
    }

    #[test]
    fn test_peers_rule_matches() {
        let rule = peers_rule();
        assert!(rule_matches(&rule, "peers=50"));
        assert!(rule_matches(&rule, "peer=1"));
        assert!(rule_matches(&rule, "peers: 100"));
    }

    #[test]
    fn test_syncing_rule_matches() {
        let rule = syncing_rule();
        assert!(rule_matches(&rule, "Syncing from peer"));
        assert!(rule_matches(&rule, "Synced slot 12345"));
        assert!(rule_matches(&rule, "syncing in progress"));
        assert!(rule_matches(&rule, "synced successfully"));
    }

    #[test]
    fn test_validator_rule_matches() {
        let rule = validator_rule();
        assert!(rule_matches(&rule, "validator=12345"));
        assert!(rule_matches(&rule, "validator: 100"));
        assert!(rule_matches(&rule, "idx=500"));
        assert!(rule_matches(&rule, "validator_index=999"));
    }

    #[test]
    fn test_duty_rule_matches() {
        let rule = duty_rule();
        assert!(rule_matches(&rule, "attester duty"));
        assert!(rule_matches(&rule, "proposer for slot"));
        assert!(rule_matches(&rule, "sync_committee member"));
        assert!(rule_matches(&rule, "aggregator selected"));
        assert!(rule_matches(&rule, "attesting now"));
        assert!(rule_matches(&rule, "proposing block"));
    }

    #[test]
    fn test_finality_rule_matches() {
        let rule = finality_rule();
        assert!(rule_matches(&rule, "finalized epoch 385"));
        assert!(rule_matches(&rule, "justified checkpoint"));
        assert!(rule_matches(&rule, "finality reached"));
        assert!(rule_matches(&rule, "checkpoint updated"));
    }

    #[test]
    fn test_mev_value_rule_matches() {
        let rule = mev_value_rule();
        assert!(rule_matches(&rule, "value=1.234 ETH"));
        assert!(rule_matches(&rule, "1000000 Gwei"));
        assert!(rule_matches(&rule, "100 gwei"));
        assert!(rule_matches(&rule, "1000000000000000000 wei"));
    }

    #[test]
    fn test_relay_rule_matches() {
        let rule = relay_rule();
        assert!(rule_matches(&rule, "relay=flashbots"));
        assert!(rule_matches(&rule, "bloxroute bid"));
        assert!(rule_matches(&rule, "ultrasound relay"));
        assert!(rule_matches(&rule, "blocknative builder"));
    }

    // =========================================================================
    // COMPOSITE PATTERN TESTS
    // =========================================================================

    #[test]
    fn test_consensus_patterns_not_empty() {
        let patterns = consensus_patterns();
        assert!(!patterns.is_empty());
        // Should contain key patterns
        assert!(patterns.iter().any(|r| rule_matches(r, "slot=12345")));
        assert!(patterns.iter().any(|r| rule_matches(r, "epoch=385")));
        assert!(patterns.iter().any(|r| rule_matches(r, "peers=50")));
    }

    #[test]
    fn test_execution_patterns_not_empty() {
        let patterns = execution_patterns();
        assert!(!patterns.is_empty());
        assert!(patterns.iter().any(|r| rule_matches(r, "peers=50")));
        assert!(
            patterns
                .iter()
                .any(|r| rule_matches(r, "0x1234567890abcdef"))
        );
    }

    #[test]
    fn test_mev_patterns_not_empty() {
        let patterns = mev_patterns();
        assert!(!patterns.is_empty());
        assert!(patterns.iter().any(|r| rule_matches(r, "1.5 ETH")));
        assert!(patterns.iter().any(|r| rule_matches(r, "flashbots")));
    }

    // =========================================================================
    // EDGE CASE TESTS
    // =========================================================================

    #[test]
    fn test_hash_rule_exact_64_chars() {
        let rule = hash_rule();
        // 64 hex chars = full transaction/block hash
        let full_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        assert!(rule_matches(&rule, full_hash));
    }

    #[test]
    fn test_hash_rule_longer_than_64() {
        let rule = hash_rule();
        // Should still match if longer
        let long_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234";
        assert!(rule_matches(&rule, long_hash));
    }

    #[test]
    fn test_hash_rule_no_prefix_fails() {
        let rule = hash_rule();
        // Without 0x prefix, should not match
        assert!(!rule_matches(&rule, "1234567890abcdef"));
    }

    #[test]
    fn test_hash_rule_invalid_hex_chars() {
        let rule = hash_rule();
        // Invalid hex chars (g, h, etc.) should not extend match
        // But partial match before invalid chars might still occur
        assert!(!rule_matches(&rule, "0xGGGGGGGG")); // No valid hex
    }

    #[test]
    fn test_address_rule_exact_40_chars() {
        let rule = address_rule();
        // Exactly 40 hex chars
        assert!(rule_matches(
            &rule,
            "0x742d35Cc6634C0532925a3b844Bc9e7595f12345"
        ));
    }

    #[test]
    fn test_address_rule_39_chars_fails() {
        let rule = address_rule();
        // 39 chars - should not match address rule (but might match hash rule)
        assert!(!rule_matches(
            &rule,
            "0x742d35Cc6634C0532925a3b844Bc9e7595f1234"
        ));
    }

    #[test]
    fn test_address_rule_41_chars() {
        let rule = address_rule();
        // 41 chars - the regex {40} is exact, so only first 40 should match
        // This tests that 40-char addresses embedded in longer strings work
        let input = "0x742d35Cc6634C0532925a3b844Bc9e7595f123456";
        assert!(rule_matches(&rule, input));
    }

    #[test]
    fn test_address_rule_mixed_case() {
        let rule = address_rule();
        // Mixed case (checksummed addresses)
        assert!(rule_matches(
            &rule,
            "0xAbCdEf1234567890AbCdEf1234567890AbCdEf12"
        ));
        // All lowercase
        assert!(rule_matches(
            &rule,
            "0xabcdef1234567890abcdef1234567890abcdef12"
        ));
        // All uppercase
        assert!(rule_matches(
            &rule,
            "0xABCDEF1234567890ABCDEF1234567890ABCDEF12"
        ));
    }

    #[test]
    fn test_slot_rule_large_numbers() {
        let rule = slot_rule();
        // Large slot numbers
        assert!(rule_matches(&rule, "slot=9999999999"));
        assert!(rule_matches(&rule, "slot=0")); // Slot 0 is valid
    }

    #[test]
    fn test_epoch_rule_large_numbers() {
        let rule = epoch_rule();
        // Large epoch numbers
        assert!(rule_matches(&rule, "epoch=999999"));
        assert!(rule_matches(&rule, "epoch=0")); // Epoch 0 is valid
    }

    #[test]
    fn test_peers_rule_zero() {
        let rule = peers_rule();
        // Zero peers (important to detect!)
        assert!(rule_matches(&rule, "peers=0"));
        assert!(rule_matches(&rule, "peer=0"));
    }

    #[test]
    fn test_number_rule_decimal() {
        let rule = number_rule();
        assert!(rule_matches(&rule, "123"));
        assert!(rule_matches(&rule, "123.456"));
        assert!(rule_matches(&rule, "0.5"));
    }

    #[test]
    fn test_pubkey_rule_with_hex() {
        let rule = pubkey_rule();
        assert!(rule_matches(&rule, "pubkey=0xabcdef1234567890"));
        assert!(rule_matches(&rule, "pubkey: 0xABCDEF12")); // 8+ hex chars required
        // Without 0x prefix should not match
        assert!(!rule_matches(&rule, "pubkey=abcdef12345678"));
        // Too few hex chars should not match
        assert!(!rule_matches(&rule, "pubkey=0xABCDEF")); // only 6 chars
    }

    #[test]
    fn test_root_rule_64_hex() {
        let rule = root_rule();
        let state_root =
            "state_root=0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let block_root =
            "block_root=0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        assert!(rule_matches(&rule, state_root));
        assert!(rule_matches(&rule, block_root));
    }
}
