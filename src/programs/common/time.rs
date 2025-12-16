//! Timestamp patterns.

use crate::colors::SemanticColor;
use crate::rule::Rule;

/// ISO timestamp (2024-01-15T10:30:45).
#[must_use] pub fn iso_timestamp_rule() -> Rule {
    Rule::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}")
        .unwrap()
        .semantic(SemanticColor::Timestamp)
        .build()
}

/// ISO timestamp with space separator (2024-01-15 10:30:45).
#[must_use] pub fn iso_timestamp_space_rule() -> Rule {
    Rule::new(r"\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2}")
        .unwrap()
        .semantic(SemanticColor::Timestamp)
        .build()
}

/// Traditional syslog timestamp (Jan 15 10:30:45).
#[must_use] pub fn syslog_timestamp_rule() -> Rule {
    Rule::new(r"\w{3}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2}")
        .unwrap()
        .semantic(SemanticColor::Timestamp)
        .build()
}

/// Common timestamp formats.
#[must_use] pub fn timestamp_rules() -> Vec<Rule> {
    vec![
        iso_timestamp_rule(),
        iso_timestamp_space_rule(),
        syslog_timestamp_rule(),
    ]
}

// Database-specific timestamp formats

/// `PostgreSQL` timestamp (2024-12-05 00:12:36.123 UTC).
#[must_use] pub fn postgres_timestamp_rule() -> Rule {
    Rule::new(r"\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2}\.\d{3}\s+\w+")
        .unwrap()
        .semantic(SemanticColor::Timestamp)
        .build()
}

/// `MySQL` ISO timestamp with fractional seconds (2024-12-05T00:12:36.123456Z).
#[must_use] pub fn mysql_iso_timestamp_rule() -> Rule {
    Rule::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+Z?")
        .unwrap()
        .semantic(SemanticColor::Timestamp)
        .build()
}

/// `MySQL` legacy timestamp (YYMMDD HH:MM:SS format).
#[must_use] pub fn mysql_legacy_timestamp_rule() -> Rule {
    Rule::new(r"\d{6}\s+\d{1,2}:\d{2}:\d{2}")
        .unwrap()
        .semantic(SemanticColor::Timestamp)
        .build()
}

/// Redis timestamp (05 Dec 2024 00:12:36.123).
#[must_use] pub fn redis_timestamp_rule() -> Rule {
    Rule::new(r"\d{2}\s+\w{3}\s+\d{4}\s+\d{2}:\d{2}:\d{2}\.\d{3}")
        .unwrap()
        .semantic(SemanticColor::Timestamp)
        .build()
}

/// `MongoDB` timestamp with timezone offset (2024-12-05T00:12:36.123+0000).
#[must_use] pub fn mongodb_timestamp_rule() -> Rule {
    Rule::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+[+-]\d{4}")
        .unwrap()
        .semantic(SemanticColor::Timestamp)
        .build()
}

/// Log4j/Elasticsearch bracketed timestamp ([2024-12-05 00:12:36,123] or [2024-12-05T00:12:36,123]).
#[must_use] pub fn log4j_timestamp_rule() -> Rule {
    Rule::new(r"\[\d{4}-\d{2}-\d{2}[T\s]\d{2}:\d{2}:\d{2},\d{3}\]")
        .unwrap()
        .semantic(SemanticColor::Timestamp)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn any_rule_matches(rules: &[Rule], input: &str) -> bool {
        rules.iter().any(|r| r.is_match(input))
    }

    // =========================================================================
    // COMPILE TESTS
    // =========================================================================

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

    // =========================================================================
    // ISO TIMESTAMP MATCHING
    // =========================================================================

    #[test]
    fn test_iso_timestamp_rule_matches() {
        let rule = iso_timestamp_rule();
        assert!(rule.is_match("2024-01-15T10:30:45"));
        assert!(rule.is_match("2024-12-31T23:59:59"));
        assert!(rule.is_match("2000-01-01T00:00:00"));
    }

    #[test]
    fn test_iso_timestamp_with_timezone() {
        let rule = iso_timestamp_rule();
        // Should match the timestamp part even with trailing timezone
        assert!(rule.is_match("2024-01-15T10:30:45Z"));
        assert!(rule.is_match("2024-01-15T10:30:45+00:00"));
    }

    #[test]
    fn test_iso_timestamp_space_rule_matches() {
        let rule = iso_timestamp_space_rule();
        assert!(rule.is_match("2024-01-15 10:30:45"));
        assert!(rule.is_match("2024-12-31  23:59:59")); // extra space ok
    }

    // =========================================================================
    // SYSLOG TIMESTAMP MATCHING
    // =========================================================================

    #[test]
    fn test_syslog_timestamp_rule_matches() {
        let rule = syslog_timestamp_rule();
        assert!(rule.is_match("Jan 15 10:30:45"));
        assert!(rule.is_match("Dec  5 00:12:36")); // single digit day
        assert!(rule.is_match("Feb 29 12:00:00")); // leap year day
    }

    #[test]
    fn test_syslog_timestamp_in_context() {
        let rule = syslog_timestamp_rule();
        assert!(rule.is_match("Jan 15 10:30:45 hostname sshd[1234]: message"));
    }

    // =========================================================================
    // COMMON TIMESTAMP RULES
    // =========================================================================

    #[test]
    fn test_timestamp_rules_match_various() {
        let rules = timestamp_rules();
        // ISO with T
        assert!(any_rule_matches(&rules, "2024-01-15T10:30:45"));
        // ISO with space
        assert!(any_rule_matches(&rules, "2024-01-15 10:30:45"));
        // Syslog format
        assert!(any_rule_matches(&rules, "Jan 15 10:30:45"));
    }

    // =========================================================================
    // DATABASE TIMESTAMP MATCHING
    // =========================================================================

    #[test]
    fn test_postgres_timestamp_matches() {
        let rule = postgres_timestamp_rule();
        assert!(rule.is_match("2024-12-05 00:12:36.123 UTC"));
        assert!(rule.is_match("2024-01-15 10:30:45.999 PST"));
    }

    #[test]
    fn test_mysql_iso_timestamp_matches() {
        let rule = mysql_iso_timestamp_rule();
        assert!(rule.is_match("2024-12-05T00:12:36.123456Z"));
        assert!(rule.is_match("2024-01-15T10:30:45.999"));
    }

    #[test]
    fn test_mysql_legacy_timestamp_matches() {
        let rule = mysql_legacy_timestamp_rule();
        assert!(rule.is_match("240115 10:30:45")); // YYMMDD format
        assert!(rule.is_match("241205 0:12:36")); // single digit hour
    }

    #[test]
    fn test_redis_timestamp_matches() {
        let rule = redis_timestamp_rule();
        assert!(rule.is_match("05 Dec 2024 00:12:36.123"));
        assert!(rule.is_match("15 Jan 2024 10:30:45.999"));
    }

    #[test]
    fn test_mongodb_timestamp_matches() {
        let rule = mongodb_timestamp_rule();
        assert!(rule.is_match("2024-12-05T00:12:36.123+0000"));
        assert!(rule.is_match("2024-01-15T10:30:45.999-0500"));
    }

    #[test]
    fn test_log4j_timestamp_matches() {
        let rule = log4j_timestamp_rule();
        assert!(rule.is_match("[2024-12-05 00:12:36,123]"));
        assert!(rule.is_match("[2024-12-05T00:12:36,123]"));
    }

    // =========================================================================
    // EDGE CASES
    // =========================================================================

    #[test]
    fn test_timestamp_boundary_values() {
        let rules = timestamp_rules();
        // Midnight
        assert!(any_rule_matches(&rules, "2024-01-01T00:00:00"));
        // End of day
        assert!(any_rule_matches(&rules, "2024-12-31T23:59:59"));
    }
}
