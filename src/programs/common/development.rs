//! Development and build-related patterns.

use crate::colors::SemanticColor;
use crate::rule::Rule;

/// Diff rules for unified diff format.
#[must_use]
pub fn diff_rules() -> Vec<Rule> {
    vec![
        // Added lines
        Rule::new(r"^\+[^+].*$")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        // Removed lines
        Rule::new(r"^-[^-].*$")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        // Hunk headers
        Rule::new(r"^@@.*@@")
            .unwrap()
            .semantic(SemanticColor::Info)
            .bold()
            .build(),
        // File headers
        Rule::new(r"^(\+\+\+|---).*$")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        // Index/diff headers
        Rule::new(r"^(diff|index|similarity|rename|copy).*$")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]
}

/// Build status rules (SUCCESS, FAILED, PASSED, etc.).
#[must_use]
pub fn build_status_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\b(PASSED|SUCCESS|SUCCEEDED|OK)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\b(FAILED|FAILURE|ERROR)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\b(SKIPPED|IGNORED)\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]
}

/// Key=value structured log patterns.
#[must_use]
pub fn key_value_rule() -> Rule {
    Rule::new(r"\b\w+=\S+")
        .unwrap()
        .semantic(SemanticColor::Value)
        .build()
}

/// Filesystem mount point patterns.
#[must_use]
pub fn mount_point_rule() -> Rule {
    Rule::new(r"/[^\s]+")
        .unwrap()
        .semantic(SemanticColor::Identifier)
        .build()
}

/// Filesystem type patterns.
#[must_use]
pub fn filesystem_type_rule() -> Rule {
    Rule::new(r"\b(ext[234]|xfs|btrfs|zfs|ntfs|vfat|fat32|tmpfs|devtmpfs|sysfs|proc|cgroup|overlay|nfs|cifs|squashfs)\b")
        .unwrap()
        .semantic(SemanticColor::Label)
        .build()
}

/// Process states for ps/top (R, S, D, Z, T, etc.).
#[must_use]
pub fn process_state_rules() -> Vec<Rule> {
    vec![
        Rule::new(r"\b[R]\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\b[S]\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\b[DT]\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\b[Z]\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
    ]
}
