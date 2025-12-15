//! Metric and quantity patterns.

use crate::colors::SemanticColor;
use crate::rule::Rule;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_rules_compile() {
        let rules = metric_rules();
        assert_eq!(rules.len(), 3);
    }
}
