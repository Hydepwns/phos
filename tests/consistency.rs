//! Consistency tests to prevent future regressions.
//!
//! These tests verify that the codebase maintains consistent patterns
//! and that documentation matches implementation.

use phos::{programs, Category};

/// Test that verifies the total number of built-in programs matches documentation.
/// Update CLAUDE.md if programs are added or removed.
#[test]
fn test_program_count_matches_documentation() {
    let registry = programs::default_registry();
    let count = registry.len();

    // Expected: 15 Ethereum + 8 DevOps + 26 System + 15 Dev + 21 Network
    //         + 5 Data + 4 Monitoring + 2 Messaging + 2 CI = 98
    assert_eq!(
        count, 98,
        "Program count mismatch. Expected 98, got {count}. Update CLAUDE.md if programs were added/removed."
    );
}

/// Test that each category has the expected number of programs.
#[test]
fn test_category_counts() {
    let registry = programs::default_registry();

    let test_cases = [
        (Category::Ethereum, 15, "Ethereum"),
        (Category::DevOps, 8, "DevOps"),
        (Category::System, 26, "System"),
        (Category::Dev, 15, "Development"),
        (Category::Network, 21, "Network"),
        (Category::Data, 5, "Data"),
        (Category::Monitoring, 4, "Monitoring"),
        (Category::Messaging, 2, "Messaging"),
        (Category::CI, 2, "CI"),
    ];

    let mismatches: Vec<_> = test_cases
        .iter()
        .filter_map(|(category, expected, name)| {
            let actual = registry.list_by_category(*category).len();
            (actual != *expected).then(|| format!("{name}: expected {expected}, got {actual}"))
        })
        .collect();

    assert!(
        mismatches.is_empty(),
        "Category count mismatches: {}",
        mismatches.join(", ")
    );
}

/// Test that all programs have at least one detection pattern.
#[test]
fn test_all_programs_have_detection_patterns() {
    let registry = programs::default_registry();

    let missing: Vec<_> = registry
        .list()
        .iter()
        .filter_map(|info| {
            registry
                .get(&info.id)
                .filter(|p| p.detect_patterns().is_empty())
                .map(|_| info.id.to_string())
        })
        .collect();

    assert!(
        missing.is_empty(),
        "Programs without detection patterns: {}",
        missing.join(", ")
    );
}

/// Test that all programs have at least one rule.
#[test]
fn test_all_programs_have_rules() {
    let registry = programs::default_registry();

    let missing: Vec<_> = registry
        .list()
        .iter()
        .filter_map(|info| {
            registry
                .get(&info.id)
                .filter(|p| p.rules().is_empty())
                .map(|_| info.id.to_string())
        })
        .collect();

    assert!(
        missing.is_empty(),
        "Programs without rules: {}",
        missing.join(", ")
    );
}

/// Test that program IDs follow the expected format (category.name).
#[test]
fn test_program_id_format() {
    let registry = programs::default_registry();

    let invalid: Vec<_> = registry
        .list()
        .iter()
        .filter(|info| info.id.split('.').count() != 2)
        .map(|info| info.id.to_string())
        .collect();

    assert!(
        invalid.is_empty(),
        "Program IDs not in 'category.name' format: {}",
        invalid.join(", ")
    );
}
