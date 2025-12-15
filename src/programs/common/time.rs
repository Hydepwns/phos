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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_rules_compile() {
        let rules = timestamp_rules();
        assert_eq!(rules.len(), 3);
    }
}
