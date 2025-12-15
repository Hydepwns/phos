//! Package management programs.
//!
//! Provides Program implementations for dnf and other package managers.

use std::sync::Arc;

use super::common;
use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::SimpleProgram;
use crate::rule::Rule;

// =============================================================================
// DNF (Dandified YUM)
// =============================================================================

fn dnf_rules() -> Vec<Rule> {
    vec![
        // Transaction actions
        Rule::new(r"^\s*Installing\s+:")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^\s*Upgrading\s+:")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^\s*Removing\s+:")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"^\s*Downgrading\s+:")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"^\s*Reinstalling\s+:")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        // Package names with version/release/arch
        Rule::new(r"\b[\w\-]+-\d+[\.\d]*-[\w\.]+\.(x86_64|i686|noarch|aarch64)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        // Repository names
        Rule::new(r"@[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        // Transaction summary
        Rule::new(r"^Transaction Summary$")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"^\s*Install\s+\d+\s+Packages?")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"^\s*Upgrade\s+\d+\s+Packages?")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^\s*Remove\s+\d+\s+Packages?")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        // Download progress
        Rule::new(r"^\(\d+/\d+\):")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\[\s*=+>\s*\]")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        // Completion status
        Rule::new(r"^Complete!$")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"^Nothing to do\.$")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        // Errors
        Rule::new(r"^Error:")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"^Problem:")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"^No match for argument:")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        // Dependencies
        Rule::new(r"^Dependencies resolved\.$")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"^\s*dependency\s+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        // Size
        common::size_rule(),
        // Metadata operations
        Rule::new(r"^Last metadata expiration check:")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"^Updating Subscription Management")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        // GPG keys
        Rule::new(r"^Importing GPG key")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^Retrieving key from")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        common::number_rule(),
    ]
}

pub fn dnf_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "system.dnf",
            "dnf",
            "DNF package manager output",
            Category::System,
            dnf_rules(),
        )
        .with_detect_patterns(vec!["dnf", "yum", "rpm"]),
    )
}
