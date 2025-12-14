//! Node.js package managers.
//!
//! Provides Program implementations for npm, yarn, and pnpm.

use std::sync::Arc;

use super::common;
use crate::colors::SemanticColor;
use crate::program::SimpleProgram;
use crate::rule::Rule;

// =============================================================================
// NPM
// =============================================================================

fn npm_rules() -> Vec<Rule> {
    vec![
        // npm prefixes
        Rule::new(r"^npm\s+(WARN|warn)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"^npm\s+(ERR|err)!")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"^npm\s+(notice)\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        // Package operations
        Rule::new(r"^\+\s+[\w@\-/]+@[\d\.]+")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"^added\s+\d+\s+packages?")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"^removed\s+\d+\s+packages?")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"^updated\s+\d+\s+packages?")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        // Audit severity
        Rule::new(r"\b(critical)\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\b(high)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\b(moderate)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\b(low)\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\d+\s+vulnerabilities")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        // Package names with versions
        Rule::new(r"[\w@\-/]+@\d+\.\d+\.\d+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        // Dependency tree
        Rule::new(r"^[\s\|`\-\+\\]+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        // Peer dependencies
        Rule::new(r"\bpeer dep missing\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bUNMET PEER DEPENDENCY\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        // Scripts
        Rule::new(r">\s+[\w@\-/]+@[\d\.]+\s+\w+")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"Lifecycle\s+script")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        // Size/timing
        common::size_rule(),
        Rule::new(r"in\s+\d+(\.\d+)?s\b")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        common::number_rule(),
    ]
}

pub fn npm_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "dev.npm",
            "npm",
            "Node.js npm commands and output",
            "dev",
            npm_rules(),
        )
        .with_detect_patterns(vec!["npm", "node"]),
    )
}

// =============================================================================
// YARN
// =============================================================================

fn yarn_rules() -> Vec<Rule> {
    vec![
        // Yarn log levels
        Rule::new(r"^(warning|warn)\s")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"^error\s")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"^success\s")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"^info\s")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        // Package operations
        Rule::new(r"^\[[\d/]+\]\s+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"^Done in \d+(\.\d+)?s")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        // Package names with versions
        Rule::new(r"[\w@\-/]+@\d+\.\d+\.\d+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        // Peer dependency warnings
        Rule::new(r"has unmet peer dependency")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"incompatible with")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        // Resolution/fetch progress
        Rule::new(r"Resolving packages")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"Fetching packages")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"Linking dependencies")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"Building fresh packages")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        // Lockfile
        Rule::new(r"Saved lockfile")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        common::size_rule(),
        common::number_rule(),
    ]
}

pub fn yarn_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "dev.yarn",
            "yarn",
            "Yarn package manager output",
            "dev",
            yarn_rules(),
        )
        .with_detect_patterns(vec!["yarn"]),
    )
}

// =============================================================================
// PNPM
// =============================================================================

fn pnpm_rules() -> Vec<Rule> {
    vec![
        // Progress line
        Rule::new(r"^Progress: resolved \d+, reused \d+, downloaded \d+, added \d+")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        // PNPM specific errors
        Rule::new(r"ERR_PNPM_\w+")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        // Lifecycle scripts
        Rule::new(r"^\s*\.\s*(pre|post)?\w+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        // Done message
        Rule::new(r"^Done in \d+(\.\d+)?s")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        // Package names with versions
        Rule::new(r"[\w@\-/]+@\d+\.\d+\.\d+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        // Peer dependency issues
        Rule::new(r"peer_dep_issues")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"Missing peer dependencies")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        // Lockfile
        Rule::new(r"Lockfile is up to date")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"Already up to date")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        // Packages count
        Rule::new(r"Packages: \+\d+")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"Packages: -\d+")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        common::size_rule(),
        common::number_rule(),
    ]
}

pub fn pnpm_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "dev.pnpm",
            "pnpm",
            "pnpm package manager output",
            "dev",
            pnpm_rules(),
        )
        .with_detect_patterns(vec!["pnpm"]),
    )
}
