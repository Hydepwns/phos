//! Timestamp patterns.

use crate::colors::SemanticColor;
use crate::rule::Rule;

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

// Database-specific timestamp formats

/// PostgreSQL timestamp (2024-12-05 00:12:36.123 UTC).
pub fn postgres_timestamp_rule() -> Rule {
    Rule::new(r"\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2}\.\d{3}\s+\w+")
        .unwrap()
        .semantic(SemanticColor::Timestamp)
        .build()
}

/// MySQL ISO timestamp with fractional seconds (2024-12-05T00:12:36.123456Z).
pub fn mysql_iso_timestamp_rule() -> Rule {
    Rule::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+Z?")
        .unwrap()
        .semantic(SemanticColor::Timestamp)
        .build()
}

/// MySQL legacy timestamp (YYMMDD HH:MM:SS format).
pub fn mysql_legacy_timestamp_rule() -> Rule {
    Rule::new(r"\d{6}\s+\d{1,2}:\d{2}:\d{2}")
        .unwrap()
        .semantic(SemanticColor::Timestamp)
        .build()
}

/// Redis timestamp (05 Dec 2024 00:12:36.123).
pub fn redis_timestamp_rule() -> Rule {
    Rule::new(r"\d{2}\s+\w{3}\s+\d{4}\s+\d{2}:\d{2}:\d{2}\.\d{3}")
        .unwrap()
        .semantic(SemanticColor::Timestamp)
        .build()
}

/// MongoDB timestamp with timezone offset (2024-12-05T00:12:36.123+0000).
pub fn mongodb_timestamp_rule() -> Rule {
    Rule::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+[+-]\d{4}")
        .unwrap()
        .semantic(SemanticColor::Timestamp)
        .build()
}

/// Log4j/Elasticsearch bracketed timestamp ([2024-12-05 00:12:36,123] or [2024-12-05T00:12:36,123]).
pub fn log4j_timestamp_rule() -> Rule {
    Rule::new(r"\[\d{4}-\d{2}-\d{2}[T\s]\d{2}:\d{2}:\d{2},\d{3}\]")
        .unwrap()
        .semantic(SemanticColor::Timestamp)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_rules_compile() {
        let rules = timestamp_rules();
        assert_eq!(rules.len(), 3);
    }

    #[test]
    fn test_database_timestamp_rules_compile() {
        // Verify all database timestamp rules compile
        let _ = postgres_timestamp_rule();
        let _ = mysql_iso_timestamp_rule();
        let _ = mysql_legacy_timestamp_rule();
        let _ = redis_timestamp_rule();
        let _ = mongodb_timestamp_rule();
        let _ = log4j_timestamp_rule();
    }

    #[test]
    fn test_postgres_timestamp_matches() {
        let rule = postgres_timestamp_rule();
        assert!(rule.regex.is_match("2024-12-05 00:12:36.123 UTC"));
    }

    #[test]
    fn test_log4j_timestamp_matches() {
        let rule = log4j_timestamp_rule();
        assert!(rule.regex.is_match("[2024-12-05 00:12:36,123]"));
        assert!(rule.regex.is_match("[2024-12-05T00:12:36,123]"));
    }
}
