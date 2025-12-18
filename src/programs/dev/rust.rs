//! Rust development tools.
//!
//! Provides Program implementations for cargo and rustc.

use std::sync::Arc;

use super::common;
use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

// =============================================================================
// CARGO
// =============================================================================

fn cargo_rules() -> Vec<Rule> {
    vec![
        // Build status
        Rule::new(r"^\s*Compiling\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^\s*Downloading\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^\s*Finished\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"^\s*Running\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^\s*Fresh\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        // Errors and warnings
        Rule::new(r"^error(\[E\d+\])?:")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"^warning:")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"^note:")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^help:")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        // Test results
        Rule::new(r"\b(ok)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\b(FAILED)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\b(ignored)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"test result:.*passed")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"test result:.*failed")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        // Crate names
        Rule::new(r"\b[\w_]+-[\w_\-]+ v\d+\.\d+\.\d+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\b[\w_]+ v\d+\.\d+\.\d+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        // File locations
        Rule::new(r"-->\s*[\w\-\./]+:\d+:\d+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\s*\|\s*")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"^\s*\d+\s*\|")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        // Timings and test counts
        common::duration_rule(),
        Rule::new(r"Doc-tests\s+\w+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\b\d+\s+(passed|failed|ignored|measured|filtered)")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    ]
}

pub fn cargo_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "dev.cargo",
            "cargo",
            "Rust cargo build and test output",
            Category::Dev,
            cargo_rules(),
        )
        .with_detect_patterns(vec!["cargo", "rustc"]),
    )
}
