//! Programming language tools.
//!
//! Provides Program implementations for go, elixir, and php.

use std::sync::Arc;

use super::common;
use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

// =============================================================================
// GO
// =============================================================================

fn go_rules() -> Vec<Rule> {
    vec![
        // Test status markers
        Rule::new(r"^===\s+RUN\s+")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^---\s+PASS:")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"^---\s+FAIL:")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"^---\s+SKIP:")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        // Package-level results
        Rule::new(r"^ok\s+\S+")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"^FAIL\s+\S+")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"^\?\s+\S+\s+\[no test files\]")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        // Build/compile errors
        Rule::new(r"^#\s+\S+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"^\S+\.go:\d+:\d+:")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        // Error messages
        Rule::new(r"\bundefined:\s+\S+")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"\bcannot\s+")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        // Package paths
        Rule::new(r"\b[\w\-\.]+/[\w\-\.]+/[\w\-\.]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        // Timing
        common::duration_rule(),
        common::number_rule(),
    ]
}

pub fn go_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "dev.go",
            "go",
            "Go build and test output",
            Category::Dev,
            go_rules(),
        )
        // Use space prefix to avoid matching "cargo build" -> "car[go build]"
        .with_detect_patterns(vec![
            " go build",
            " go run",
            " go mod",
            " go get",
            " go fmt",
            " go vet",
            " go test",
        ]),
    )
}

// =============================================================================
// ELIXIR / MIX
// =============================================================================

fn elixir_rules() -> Vec<Rule> {
    vec![
        // Compilation status
        Rule::new(r"^Compiling \d+ files? \(\.ex\)")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^Compiled \S+")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"^Generated \S+")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        // Warnings and errors
        Rule::new(r"^warning:")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"^\*\* \([\w\.]+\)")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"^\([\w\.]+\) ")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        // Test results
        Rule::new(r"^\.$")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"^F$")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"^\*$")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"Finished in \d+(\.\d+)? (seconds?|ms)")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\d+ tests?, \d+ failures?")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"\d+ tests?, 0 failures")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        // Module names
        Rule::new(r"\b[A-Z][\w\.]+\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        // File:line references
        Rule::new(r"\([\w/\.\-]+:\d+\)")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"lib/[\w/\.\-]+\.ex:\d+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        // Dependencies
        Rule::new(r"^Resolving Hex dependencies")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^Dependency resolution completed")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        common::duration_rule(),
        common::number_rule(),
    ]
}

pub fn elixir_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "dev.elixir",
            "elixir",
            "Elixir/Mix build and test output",
            Category::Dev,
            elixir_rules(),
        )
        .with_detect_patterns(vec!["mix", "elixir", "iex"]),
    )
}

// =============================================================================
// PHP
// =============================================================================

fn php_rules() -> Vec<Rule> {
    vec![
        // Error types
        Rule::new(r"\bFatal error\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bParse error\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bWarning\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"\bNotice\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bDeprecated\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bStrict Standards\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        // Stack trace
        Rule::new(r"^Stack trace:")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"^#\d+\s+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"thrown in\s+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        // File:line references
        Rule::new(r"in\s+/[\w\.\-/]+\.php(:\d+)?")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"on line\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        // Common PHP errors
        Rule::new(r"Undefined (variable|index|offset|property):")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"Call to undefined (function|method)")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"Class '[^']+' not found")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        // PHPUnit output
        Rule::new(r"^OK \(\d+ tests?, \d+ assertions?\)")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"^FAILURES!")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"^ERRORS!")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"^Tests: \d+, Assertions: \d+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        // Composer output
        Rule::new(r"^Installing\s+")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^Updating\s+")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^Removing\s+")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"Package \w+/\w+ is")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        // Version strings
        Rule::new(r"\bv?\d+\.\d+(\.\d+)?(-[\w\.]+)?\b")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        common::number_rule(),
    ]
}

pub fn php_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "dev.php",
            "php",
            "PHP interpreter and composer output",
            Category::Dev,
            php_rules(),
        )
        .with_detect_patterns(vec!["php", "composer", "phpunit", "artisan"]),
    )
}
