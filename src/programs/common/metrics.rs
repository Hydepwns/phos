//! Metric and quantity patterns.

use crate::colors::SemanticColor;
use crate::rule::Rule;

/// Size/memory with units (10 GB, 256 MB, 1024 KB).
#[must_use] pub fn size_rule() -> Rule {
    Rule::new(r"\d+(\.\d+)?\s*(GB|MB|KB|B|GiB|MiB|KiB)\b")
        .unwrap()
        .semantic(SemanticColor::Metric)
        .build()
}

/// Duration with units (100ms, 5s, 2m, 1h).
#[must_use] pub fn duration_rule() -> Rule {
    Rule::new(r"\b\d+(\.\d+)?\s*(ms|s|m|h|ns|us)\b")
        .unwrap()
        .semantic(SemanticColor::Metric)
        .build()
}

/// Percentage values (50%, 99.9%).
#[must_use] pub fn percentage_rule() -> Rule {
    Rule::new(r"\b\d+(\.\d+)?%")
        .unwrap()
        .semantic(SemanticColor::Metric)
        .build()
}

/// Generic number pattern (should typically be last in rule list).
#[must_use] pub fn number_rule() -> Rule {
    Rule::new(r"\b\d+\b")
        .unwrap()
        .semantic(SemanticColor::Number)
        .build()
}

/// Common metric rules (size, duration, percentage).
#[must_use] pub fn metric_rules() -> Vec<Rule> {
    vec![size_rule(), duration_rule(), percentage_rule()]
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
    fn test_metric_rules_compile() {
        let rules = metric_rules();
        assert_eq!(rules.len(), 3);
    }

    // =========================================================================
    // SIZE MATCHING
    // =========================================================================

    #[test]
    fn test_size_rule_matches_bytes() {
        let rule = size_rule();
        assert!(rule.is_match("100 B"));
        assert!(rule.is_match("1024 KB"));
        assert!(rule.is_match("512 MB"));
        assert!(rule.is_match("10 GB"));
    }

    #[test]
    fn test_size_rule_matches_binary_units() {
        let rule = size_rule();
        assert!(rule.is_match("1024 KiB"));
        assert!(rule.is_match("512 MiB"));
        assert!(rule.is_match("10 GiB"));
    }

    #[test]
    fn test_size_rule_matches_decimal() {
        let rule = size_rule();
        assert!(rule.is_match("1.5 GB"));
        assert!(rule.is_match("256.75 MB"));
        assert!(rule.is_match("0.5 KB"));
    }

    #[test]
    fn test_size_rule_in_context() {
        let rule = size_rule();
        assert!(rule.is_match("Memory: 1024 MB used"));
        assert!(rule.is_match("Disk: 500 GB total"));
    }

    // =========================================================================
    // DURATION MATCHING
    // =========================================================================

    #[test]
    fn test_duration_rule_matches_common_units() {
        let rule = duration_rule();
        assert!(rule.is_match("100ms"));
        assert!(rule.is_match("5s"));
        assert!(rule.is_match("2m"));
        assert!(rule.is_match("1h"));
    }

    #[test]
    fn test_duration_rule_matches_small_units() {
        let rule = duration_rule();
        assert!(rule.is_match("100ns")); // nanoseconds
        assert!(rule.is_match("50us")); // microseconds
    }

    #[test]
    fn test_duration_rule_matches_decimal() {
        let rule = duration_rule();
        assert!(rule.is_match("1.5s"));
        assert!(rule.is_match("0.5ms"));
        assert!(rule.is_match("2.25h"));
    }

    #[test]
    fn test_duration_rule_in_context() {
        let rule = duration_rule();
        assert!(rule.is_match("Request took 150ms"));
        assert!(rule.is_match("Timeout: 30s"));
    }

    // =========================================================================
    // PERCENTAGE MATCHING
    // =========================================================================

    #[test]
    fn test_percentage_rule_matches() {
        let rule = percentage_rule();
        assert!(rule.is_match("50%"));
        assert!(rule.is_match("100%"));
        assert!(rule.is_match("0%"));
    }

    #[test]
    fn test_percentage_rule_matches_decimal() {
        let rule = percentage_rule();
        assert!(rule.is_match("99.9%"));
        assert!(rule.is_match("0.1%"));
        assert!(rule.is_match("33.33%"));
    }

    #[test]
    fn test_percentage_rule_in_context() {
        let rule = percentage_rule();
        assert!(rule.is_match("CPU: 75%"));
        assert!(rule.is_match("Progress: 50% complete"));
    }

    // =========================================================================
    // NUMBER MATCHING
    // =========================================================================

    #[test]
    fn test_number_rule_matches() {
        let rule = number_rule();
        assert!(rule.is_match("12345"));
        assert!(rule.is_match("0"));
        assert!(rule.is_match("999999999"));
    }

    #[test]
    fn test_number_rule_word_boundary() {
        let rule = number_rule();
        // Should match standalone numbers
        assert!(rule.is_match("count=100"));
        assert!(rule.is_match("value: 42"));
    }

    // =========================================================================
    // COMBINED METRIC RULES
    // =========================================================================

    #[test]
    fn test_metric_rules_match_various() {
        let rules = metric_rules();
        // Size
        assert!(any_rule_matches(&rules, "1024 MB"));
        // Duration
        assert!(any_rule_matches(&rules, "150ms"));
        // Percentage
        assert!(any_rule_matches(&rules, "75%"));
    }

    // =========================================================================
    // EDGE CASES
    // =========================================================================

    #[test]
    fn test_size_rule_large_values() {
        let rule = size_rule();
        assert!(rule.is_match("1000000 GB"));
    }

    #[test]
    fn test_percentage_over_100() {
        let rule = percentage_rule();
        // Percentages over 100 are valid (e.g., growth rates)
        assert!(rule.is_match("150%"));
        assert!(rule.is_match("200%"));
    }
}
