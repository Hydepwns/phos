//! Common rule patterns shared across programs.
//!
//! Provides reusable rule builders for common patterns like log levels,
//! IP addresses, timestamps, and size metrics.

use crate::colors::SemanticColor;
use crate::rule::Rule;

/// Standard log level rules (ERROR, WARN, INFO, DEBUG, TRACE).
/// Handles common case variations.
pub fn log_level_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\b(ERROR|error|Error|ERR|err|Err)\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\b(WARN|warn|Warn|WARNING|warning|Warning)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"\b(INFO|info|Info)\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\b(DEBUG|debug|Debug)\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\b(TRACE|trace|Trace)\b")
            .unwrap()
            .semantic(SemanticColor::Trace)
            .build(),
    ]
}

/// Systemd/syslog priority levels (emerg, alert, crit, etc.).
pub fn syslog_priority_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\b(emerg|emergency)\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\balert\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\b(crit|critical)\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\b(err|error)\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"\b(warn|warning)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bnotice\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\binfo\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bdebug\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]
}

/// IPv4 address pattern.
pub fn ipv4_rule() -> Rule {
    Rule::new(r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b")
        .unwrap()
        .semantic(SemanticColor::Identifier)
        .build()
}

/// IPv6 address pattern (simplified).
pub fn ipv6_rule() -> Rule {
    Rule::new(r"\b([a-fA-F0-9:]+:+[a-fA-F0-9:]+)\b")
        .unwrap()
        .semantic(SemanticColor::Identifier)
        .build()
}

/// Both IPv4 and IPv6 rules.
pub fn ip_rules() -> Vec<Rule> {
    vec![ipv4_rule(), ipv6_rule()]
}

/// ISO timestamp (2024-01-15T10:30:45).
pub fn iso_timestamp_rule() -> Rule {
    Rule::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}")
        .unwrap()
        .semantic(SemanticColor::Timestamp)
        .build()
}

/// ISO timestamp with space separator (2024-01-15 10:30:45).
pub fn iso_timestamp_space_rule() -> Rule {
    Rule::new(r"\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2}")
        .unwrap()
        .semantic(SemanticColor::Timestamp)
        .build()
}

/// Traditional syslog timestamp (Jan 15 10:30:45).
pub fn syslog_timestamp_rule() -> Rule {
    Rule::new(r"\w{3}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2}")
        .unwrap()
        .semantic(SemanticColor::Timestamp)
        .build()
}

/// Common timestamp formats.
pub fn timestamp_rules() -> Vec<Rule> {
    vec![
        iso_timestamp_rule(),
        iso_timestamp_space_rule(),
        syslog_timestamp_rule(),
    ]
}

/// Size/memory with units (10 GB, 256 MB, 1024 KB).
pub fn size_rule() -> Rule {
    Rule::new(r"\d+(\.\d+)?\s*(GB|MB|KB|B|GiB|MiB|KiB)\b")
        .unwrap()
        .semantic(SemanticColor::Metric)
        .build()
}

/// Duration with units (100ms, 5s, 2m, 1h).
pub fn duration_rule() -> Rule {
    Rule::new(r"\b\d+(\.\d+)?\s*(ms|s|m|h|ns|us)\b")
        .unwrap()
        .semantic(SemanticColor::Metric)
        .build()
}

/// Percentage values (50%, 99.9%).
pub fn percentage_rule() -> Rule {
    Rule::new(r"\b\d+(\.\d+)?%")
        .unwrap()
        .semantic(SemanticColor::Metric)
        .build()
}

/// Generic number pattern (should typically be last in rule list).
pub fn number_rule() -> Rule {
    Rule::new(r"\b\d+\b")
        .unwrap()
        .semantic(SemanticColor::Number)
        .build()
}

/// Common metric rules (size, duration, percentage).
pub fn metric_rules() -> Vec<Rule> {
    vec![size_rule(), duration_rule(), percentage_rule()]
}

/// Hex identifiers (12-char short, 64-char full - common for container/commit IDs).
pub fn hex_id_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\b[a-f0-9]{64}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\b[a-f0-9]{40}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\b[a-f0-9]{12}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\b[a-f0-9]{7,8}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]
}

/// HTTP status codes (4xx/5xx as errors, 2xx/3xx as success).
pub fn http_status_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\b[45]\d{2}\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"\b[23]\d{2}\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    ]
}

/// HTTP methods (GET, POST, PUT, DELETE, etc.).
pub fn http_method_rule() -> Rule {
    Rule::new(r"\b(GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS)\b")
        .unwrap()
        .semantic(SemanticColor::Key)
        .build()
}

/// Key=value structured log patterns.
pub fn key_value_rule() -> Rule {
    Rule::new(r"\b\w+=\S+")
        .unwrap()
        .semantic(SemanticColor::Value)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_rules_compile() {
        let rules = log_level_rules();
        assert_eq!(rules.len(), 5);
    }

    #[test]
    fn test_ip_rules_compile() {
        let rules = ip_rules();
        assert_eq!(rules.len(), 2);
    }

    #[test]
    fn test_timestamp_rules_compile() {
        let rules = timestamp_rules();
        assert_eq!(rules.len(), 3);
    }

    #[test]
    fn test_metric_rules_compile() {
        let rules = metric_rules();
        assert_eq!(rules.len(), 3);
    }
}
