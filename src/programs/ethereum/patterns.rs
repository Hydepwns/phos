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

define_log_levels!(
    rust_log_levels,
    [
        (r"\bERROR\b", SemanticColor::Error, bold),
        (r"\bWARN\b", SemanticColor::Warn, bold),
        (r"\bINFO\b", SemanticColor::Info),
        (r"\bDEBUG\b", SemanticColor::Debug),
        (r"\bTRACE\b", SemanticColor::Trace),
    ]
);

define_log_levels!(
    lighthouse_log_levels,
    [
        (r"\bCRIT\b", SemanticColor::Error, bold),
        (r"\bERRO\b", SemanticColor::Error, bold),
        (r"\bWARN\b", SemanticColor::Warn, bold),
        (r"\bINFO\b", SemanticColor::Info),
        (r"\bDEBG\b", SemanticColor::Debug),
        (r"\bTRCE\b", SemanticColor::Trace),
    ]
);

define_log_levels!(
    nimbus_log_levels,
    [
        (r"\bFTL\b", SemanticColor::Error, bold),
        (r"\bERR\b", SemanticColor::Error, bold),
        (r"\bWRN\b", SemanticColor::Warn, bold),
        (r"\bINF\b", SemanticColor::Info),
        (r"\bDBG\b", SemanticColor::Debug),
        (r"\bTRC\b", SemanticColor::Trace),
    ]
);

define_log_levels!(
    lodestar_log_levels,
    [
        (r"\b(error|ERR)\b", SemanticColor::Error, bold),
        (r"\b(warn|WRN)\b", SemanticColor::Warn, bold),
        (r"\b(info|INF)\b", SemanticColor::Info),
        (r"\b(debug|DBG)\b", SemanticColor::Debug),
        (r"\b(verbose|VRB|trace)\b", SemanticColor::Trace),
    ]
);

define_log_levels!(
    prysm_log_levels,
    [
        (r"level=error", SemanticColor::Error, bold),
        (r"level=warning", SemanticColor::Warn, bold),
        (r"level=info", SemanticColor::Info),
        (r"level=debug", SemanticColor::Debug),
        (r"level=trace", SemanticColor::Trace),
    ]
);

define_log_levels!(
    dotnet_log_levels,
    [
        (r"\bError\b", SemanticColor::Error, bold),
        (r"\bWarn\b", SemanticColor::Warn, bold),
        (r"\bInfo\b", SemanticColor::Info),
        (r"\bDebug\b", SemanticColor::Debug),
        (r"\bTrace\b", SemanticColor::Trace),
    ]
);

define_log_levels!(
    erigon_log_levels,
    [
        (r"lvl=eror", SemanticColor::Error, bold),
        (r"lvl=warn", SemanticColor::Warn, bold),
        (r"lvl=info", SemanticColor::Info),
        (r"lvl=dbug", SemanticColor::Debug),
        (r"lvl=trce", SemanticColor::Trace),
    ]
);

define_log_levels!(
    elixir_log_levels,
    [
        (r"\[error\]", SemanticColor::Error, bold),
        (r"\[warning\]", SemanticColor::Warn, bold),
        (r"\[info\]", SemanticColor::Info),
        (r"\[debug\]", SemanticColor::Debug),
    ]
);

// =========================================================================
// COMMON ETHEREUM PATTERNS
// =========================================================================

define_rule!(hash_rule, r"0x[a-fA-F0-9]{8,}", hex(HASH_COLOR));
define_rule!(address_rule, r"0x[a-fA-F0-9]{40}", hex(ADDRESS_COLOR));
define_rule!(
    number_rule,
    r"\b\d+(\.\d+)?\b",
    semantic(SemanticColor::Number)
);
define_rule!(slot_rule, r"\bslot[=:\s]+(\d+)", hex(SLOT_COLOR));
define_rule!(epoch_rule, r"\bepoch[=:\s]+(\d+)", hex(EPOCH_COLOR));
define_rule!(peers_rule, r"\bpeers?[=:\s]+(\d+)", hex(PEER_ID_COLOR));
define_rule!(
    syncing_rule,
    r"\b(Syncing|Synced|syncing|synced)\b",
    hex(SYNCING_COLOR)
);
define_rule!(
    success_rule,
    r"\b(success|valid|verified)\b",
    semantic(SemanticColor::Success)
);
define_rule!(
    failure_rule,
    r"\b(failed|invalid|error|timeout|rejected)\b",
    semantic(SemanticColor::Failure)
);
define_rule!(
    validator_rule,
    r"\b(validator|idx|validator_index)[=:\s]+(\d+)",
    hex(VALIDATOR_COLOR)
);
define_rule!(
    pubkey_rule,
    r"pubkey[=:\s]+0x[a-fA-F0-9]{8,}",
    hex(PUBKEY_COLOR)
);
define_rule!(
    duty_rule,
    r"\b(attester|proposer|sync_committee|aggregator|attesting|proposing)\b",
    hex(DUTY_COLOR)
);
define_rule!(
    committee_rule,
    r"\b(committee|subnet)[=:\s]+(\d+)",
    hex(COMMITTEE_COLOR)
);
define_rule!(
    finality_rule,
    r"\b(finalized|justified|finality|checkpoint)\b",
    hex(FINALITY_COLOR)
);
define_rule!(
    root_rule,
    r"(state_root|block_root|root)[=:\s]+0x[a-fA-F0-9]{64}",
    hex(ROOT_COLOR)
);
define_rule!(
    attestation_rule,
    r"\b(attestation|attest|attested)\b",
    hex(ATTESTATION_COLOR)
);
define_rule!(
    mev_value_rule,
    r"(\d+\.?\d*)\s*(ETH|Gwei|gwei|wei)",
    hex(MEV_VALUE_COLOR),
    bold
);
define_rule!(
    relay_rule,
    r"\b(flashbots|bloxroute|blocknative|eden|manifold|ultrasound|agnostic)\b",
    hex(RELAY_COLOR)
);
define_rule!(builder_rule, r"builder[=:\s]+(\S+)", hex(BUILDER_COLOR));

// =========================================================================
// COMPOSITE RULE SETS
// =========================================================================

/// Common consensus layer patterns (validators, duties, attestations, etc.)
pub fn consensus_patterns() -> Result<Vec<Rule>, regex::Error> {
    Ok(vec![
        slot_rule()?,
        epoch_rule()?,
        address_rule()?, // Before hash_rule for specific address matching
        hash_rule()?,
        peers_rule()?,
        syncing_rule()?,
        success_rule()?,
        failure_rule()?,
        validator_rule()?,
        pubkey_rule()?,
        duty_rule()?,
        committee_rule()?,
        finality_rule()?,
        root_rule()?,
        attestation_rule()?,
        number_rule()?,
    ])
}

/// Common execution layer patterns (blocks, forkchoice, etc.)
pub fn execution_patterns() -> Result<Vec<Rule>, regex::Error> {
    Ok(vec![
        address_rule()?, // Before hash_rule for specific address matching
        hash_rule()?,
        peers_rule()?,
        syncing_rule()?,
        success_rule()?,
        failure_rule()?,
        root_rule()?,
        finality_rule()?,
        number_rule()?,
    ])
}

/// MEV/relay patterns
pub fn mev_patterns() -> Result<Vec<Rule>, regex::Error> {
    Ok(vec![mev_value_rule()?, relay_rule()?, builder_rule()?])
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
        let rules = rust_log_levels().unwrap();
        assert!(any_rule_matches(&rules, "ERROR: something failed"));
        assert!(any_rule_matches(&rules, "WARN: this is a warning"));
        assert!(any_rule_matches(&rules, "INFO: informational message"));
        assert!(any_rule_matches(&rules, "DEBUG: debug output"));
        assert!(any_rule_matches(&rules, "TRACE: trace level"));
    }

    #[test]
    fn test_rust_log_levels_word_boundary() {
        let rules = rust_log_levels().unwrap();
        // Should not match partial words
        assert!(!any_rule_matches(&rules, "ERRORS"));
        assert!(!any_rule_matches(&rules, "MYERROR"));
        assert!(!any_rule_matches(&rules, "INFOBOX"));
    }

    #[test]
    fn test_lighthouse_log_levels_match() {
        let rules = lighthouse_log_levels().unwrap();
        assert!(any_rule_matches(&rules, "CRIT: critical error"));
        assert!(any_rule_matches(&rules, "ERRO: error message"));
        assert!(any_rule_matches(&rules, "WARN: warning"));
        assert!(any_rule_matches(&rules, "INFO: info"));
        assert!(any_rule_matches(&rules, "DEBG: debug"));
        assert!(any_rule_matches(&rules, "TRCE: trace"));
    }

    #[test]
    fn test_nimbus_log_levels_match() {
        let rules = nimbus_log_levels().unwrap();
        assert!(any_rule_matches(&rules, "FTL: fatal error"));
        assert!(any_rule_matches(&rules, "ERR: error"));
        assert!(any_rule_matches(&rules, "WRN: warning"));
        assert!(any_rule_matches(&rules, "INF: info"));
        assert!(any_rule_matches(&rules, "DBG: debug"));
        assert!(any_rule_matches(&rules, "TRC: trace"));
    }

    #[test]
    fn test_lodestar_log_levels_match() {
        let rules = lodestar_log_levels().unwrap();
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
        let rules = prysm_log_levels().unwrap();
        assert!(any_rule_matches(&rules, "level=error msg=\"failed\""));
        assert!(any_rule_matches(&rules, "level=warning msg=\"caution\""));
        assert!(any_rule_matches(&rules, "level=info msg=\"status\""));
        assert!(any_rule_matches(&rules, "level=debug msg=\"debug\""));
        assert!(any_rule_matches(&rules, "level=trace msg=\"trace\""));
    }

    #[test]
    fn test_erigon_log_levels_match() {
        let rules = erigon_log_levels().unwrap();
        assert!(any_rule_matches(&rules, "lvl=eror msg=\"error\""));
        assert!(any_rule_matches(&rules, "lvl=warn msg=\"warning\""));
        assert!(any_rule_matches(&rules, "lvl=info msg=\"info\""));
        assert!(any_rule_matches(&rules, "lvl=dbug msg=\"debug\""));
        assert!(any_rule_matches(&rules, "lvl=trce msg=\"trace\""));
    }

    #[test]
    fn test_elixir_log_levels_match() {
        let rules = elixir_log_levels().unwrap();
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
        let rule = hash_rule().unwrap();
        // Should match 8+ hex chars with 0x prefix
        assert!(rule_matches(&rule, "hash=0x1234567890abcdef"));
        assert!(rule_matches(&rule, "0xabcdef12"));
        assert!(rule_matches(&rule, "0xABCDEF12")); // uppercase
        assert!(rule_matches(&rule, "0xAbCdEf12")); // mixed case
    }

    #[test]
    fn test_hash_rule_minimum_length() {
        let rule = hash_rule().unwrap();
        // 8 chars minimum
        assert!(rule_matches(&rule, "0x12345678"));
        // Less than 8 should not match
        assert!(!rule_matches(&rule, "0x1234567"));
    }

    #[test]
    fn test_address_rule_matches_40_hex() {
        let rule = address_rule().unwrap();
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
        let rule = slot_rule().unwrap();
        assert!(rule_matches(&rule, "slot=12345"));
        assert!(rule_matches(&rule, "slot: 12345"));
        assert!(rule_matches(&rule, "slot 12345"));
        // Should not match without number
        assert!(!rule_matches(&rule, "slot="));
        assert!(!rule_matches(&rule, "timeslot"));
    }

    #[test]
    fn test_epoch_rule_variations() {
        let rule = epoch_rule().unwrap();
        assert!(rule_matches(&rule, "epoch=385"));
        assert!(rule_matches(&rule, "epoch: 385"));
        assert!(rule_matches(&rule, "epoch 385"));
    }

    #[test]
    fn test_peers_rule_matches() {
        let rule = peers_rule().unwrap();
        assert!(rule_matches(&rule, "peers=50"));
        assert!(rule_matches(&rule, "peer=1"));
        assert!(rule_matches(&rule, "peers: 100"));
    }

    #[test]
    fn test_syncing_rule_matches() {
        let rule = syncing_rule().unwrap();
        assert!(rule_matches(&rule, "Syncing from peer"));
        assert!(rule_matches(&rule, "Synced slot 12345"));
        assert!(rule_matches(&rule, "syncing in progress"));
        assert!(rule_matches(&rule, "synced successfully"));
    }

    #[test]
    fn test_validator_rule_matches() {
        let rule = validator_rule().unwrap();
        assert!(rule_matches(&rule, "validator=12345"));
        assert!(rule_matches(&rule, "validator: 100"));
        assert!(rule_matches(&rule, "idx=500"));
        assert!(rule_matches(&rule, "validator_index=999"));
    }

    #[test]
    fn test_duty_rule_matches() {
        let rule = duty_rule().unwrap();
        assert!(rule_matches(&rule, "attester duty"));
        assert!(rule_matches(&rule, "proposer for slot"));
        assert!(rule_matches(&rule, "sync_committee member"));
        assert!(rule_matches(&rule, "aggregator selected"));
        assert!(rule_matches(&rule, "attesting now"));
        assert!(rule_matches(&rule, "proposing block"));
    }

    #[test]
    fn test_finality_rule_matches() {
        let rule = finality_rule().unwrap();
        assert!(rule_matches(&rule, "finalized epoch 385"));
        assert!(rule_matches(&rule, "justified checkpoint"));
        assert!(rule_matches(&rule, "finality reached"));
        assert!(rule_matches(&rule, "checkpoint updated"));
    }

    #[test]
    fn test_mev_value_rule_matches() {
        let rule = mev_value_rule().unwrap();
        assert!(rule_matches(&rule, "value=1.234 ETH"));
        assert!(rule_matches(&rule, "1000000 Gwei"));
        assert!(rule_matches(&rule, "100 gwei"));
        assert!(rule_matches(&rule, "1000000000000000000 wei"));
    }

    #[test]
    fn test_relay_rule_matches() {
        let rule = relay_rule().unwrap();
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
        let patterns = consensus_patterns().unwrap();
        assert!(!patterns.is_empty());
        // Should contain key patterns
        assert!(patterns.iter().any(|r| rule_matches(r, "slot=12345")));
        assert!(patterns.iter().any(|r| rule_matches(r, "epoch=385")));
        assert!(patterns.iter().any(|r| rule_matches(r, "peers=50")));
    }

    #[test]
    fn test_execution_patterns_not_empty() {
        let patterns = execution_patterns().unwrap();
        assert!(!patterns.is_empty());
        assert!(patterns.iter().any(|r| rule_matches(r, "peers=50")));
        assert!(patterns
            .iter()
            .any(|r| rule_matches(r, "0x1234567890abcdef")));
    }

    #[test]
    fn test_mev_patterns_not_empty() {
        let patterns = mev_patterns().unwrap();
        assert!(!patterns.is_empty());
        assert!(patterns.iter().any(|r| rule_matches(r, "1.5 ETH")));
        assert!(patterns.iter().any(|r| rule_matches(r, "flashbots")));
    }

    // =========================================================================
    // EDGE CASE TESTS
    // =========================================================================

    #[test]
    fn test_hash_rule_exact_64_chars() {
        let rule = hash_rule().unwrap();
        // 64 hex chars = full transaction/block hash
        let full_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        assert!(rule_matches(&rule, full_hash));
    }

    #[test]
    fn test_hash_rule_longer_than_64() {
        let rule = hash_rule().unwrap();
        // Should still match if longer
        let long_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234";
        assert!(rule_matches(&rule, long_hash));
    }

    #[test]
    fn test_hash_rule_no_prefix_fails() {
        let rule = hash_rule().unwrap();
        // Without 0x prefix, should not match
        assert!(!rule_matches(&rule, "1234567890abcdef"));
    }

    #[test]
    fn test_hash_rule_invalid_hex_chars() {
        let rule = hash_rule().unwrap();
        // Invalid hex chars (g, h, etc.) should not extend match
        // But partial match before invalid chars might still occur
        assert!(!rule_matches(&rule, "0xGGGGGGGG")); // No valid hex
    }

    #[test]
    fn test_address_rule_exact_40_chars() {
        let rule = address_rule().unwrap();
        // Exactly 40 hex chars
        assert!(rule_matches(
            &rule,
            "0x742d35Cc6634C0532925a3b844Bc9e7595f12345"
        ));
    }

    #[test]
    fn test_address_rule_39_chars_fails() {
        let rule = address_rule().unwrap();
        // 39 chars - should not match address rule (but might match hash rule)
        assert!(!rule_matches(
            &rule,
            "0x742d35Cc6634C0532925a3b844Bc9e7595f1234"
        ));
    }

    #[test]
    fn test_address_rule_41_chars() {
        let rule = address_rule().unwrap();
        // 41 chars - the regex {40} is exact, so only first 40 should match
        // This tests that 40-char addresses embedded in longer strings work
        let input = "0x742d35Cc6634C0532925a3b844Bc9e7595f123456";
        assert!(rule_matches(&rule, input));
    }

    #[test]
    fn test_address_rule_mixed_case() {
        let rule = address_rule().unwrap();
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
        let rule = slot_rule().unwrap();
        // Large slot numbers
        assert!(rule_matches(&rule, "slot=9999999999"));
        assert!(rule_matches(&rule, "slot=0")); // Slot 0 is valid
    }

    #[test]
    fn test_epoch_rule_large_numbers() {
        let rule = epoch_rule().unwrap();
        // Large epoch numbers
        assert!(rule_matches(&rule, "epoch=999999"));
        assert!(rule_matches(&rule, "epoch=0")); // Epoch 0 is valid
    }

    #[test]
    fn test_peers_rule_zero() {
        let rule = peers_rule().unwrap();
        // Zero peers (important to detect!)
        assert!(rule_matches(&rule, "peers=0"));
        assert!(rule_matches(&rule, "peer=0"));
    }

    #[test]
    fn test_number_rule_decimal() {
        let rule = number_rule().unwrap();
        assert!(rule_matches(&rule, "123"));
        assert!(rule_matches(&rule, "123.456"));
        assert!(rule_matches(&rule, "0.5"));
    }

    #[test]
    fn test_pubkey_rule_with_hex() {
        let rule = pubkey_rule().unwrap();
        assert!(rule_matches(&rule, "pubkey=0xabcdef1234567890"));
        assert!(rule_matches(&rule, "pubkey: 0xABCDEF12")); // 8+ hex chars required
                                                            // Without 0x prefix should not match
        assert!(!rule_matches(&rule, "pubkey=abcdef12345678"));
        // Too few hex chars should not match
        assert!(!rule_matches(&rule, "pubkey=0xABCDEF")); // only 6 chars
    }

    #[test]
    fn test_root_rule_64_hex() {
        let rule = root_rule().unwrap();
        let state_root =
            "state_root=0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let block_root =
            "block_root=0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        assert!(rule_matches(&rule, state_root));
        assert!(rule_matches(&rule, block_root));
    }
}
