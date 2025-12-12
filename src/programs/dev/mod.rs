//! Development tool programs.
//!
//! Provides Program implementations for git, cargo, npm, etc.

use std::sync::Arc;

use super::common;
use crate::colors::SemanticColor;
use crate::program::{ProgramRegistry, SimpleProgram};
use crate::rule::Rule;

/// Register all Dev programs with the registry.
pub fn register_all(registry: &mut ProgramRegistry) {
    registry.register(Arc::new(git_program()));
    registry.register(Arc::new(cargo_program()));
    registry.register(Arc::new(npm_program()));
    registry.register(Arc::new(go_program()));
    registry.register(Arc::new(make_program()));
    registry.register(Arc::new(yarn_program()));
    registry.register(Arc::new(pnpm_program()));
    registry.register(Arc::new(elixir_program()));
}

// =============================================================================
// GIT
// =============================================================================

fn git_rules() -> Vec<Rule> {
    let mut rules = vec![
        // Diff: additions
        Rule::new(r"^\+[^+].*$")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"^\+\+\+.*$")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        // Diff: deletions
        Rule::new(r"^-[^-].*$")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"^---.*$")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        // Diff: hunk headers
        Rule::new(r"^@@.*@@")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        // Diff: file headers
        Rule::new(r"^diff --git.*$")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"^index [a-f0-9]+\.\.[a-f0-9]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ];

    // Commit hashes (40-char and 7-8 char)
    rules.extend(
        common::hex_id_rules()
            .into_iter()
            .filter(|_| true) // Take 40-char and 7-8 char
            .take(2)
            .skip(1), // Skip 64-char, take 40-char
    );
    rules.push(
        Rule::new(r"\b[a-f0-9]{7,8}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // Branch names and status
    rules.extend([
        Rule::new(r"\b(HEAD|master|main|develop|feature|bugfix|hotfix|release)/?\S*")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"^\s*M\s")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"^\s*A\s")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"^\s*D\s")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"^\s*R\s")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^\s*\?\?\s")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Status keywords
    rules.extend([
        Rule::new(r"\b(modified|new file|deleted|renamed|copied)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\b(Untracked files|Changes not staged|Changes to be committed)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .bold()
            .build(),
        Rule::new(r"\b(fetch|push|pull|merge|rebase|cherry-pick)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\b(origin|upstream)/\S+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Author/date and files
    rules.extend([
        Rule::new(r"Author:\s+.*$")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"Date:\s+.*$")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
        Rule::new(r"\s[\w\-\./]+\.(rs|js|ts|py|go|java|c|cpp|h|hpp|md|json|yaml|yml|toml)\b")
            .unwrap()
            .semantic(SemanticColor::String)
            .build(),
        Rule::new(r"\b\d+\s+(insertions?|deletions?|files?\s+changed)")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    ]);

    rules
}

fn git_program() -> SimpleProgram {
    SimpleProgram::new(
        "dev.git",
        "git",
        "Git commands and output",
        "dev",
        git_rules(),
    )
    .with_detect_patterns(vec!["git"])
}

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

fn cargo_program() -> SimpleProgram {
    SimpleProgram::new(
        "dev.cargo",
        "cargo",
        "Rust cargo build and test output",
        "dev",
        cargo_rules(),
    )
    .with_detect_patterns(vec!["cargo", "rustc"])
}

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

fn npm_program() -> SimpleProgram {
    SimpleProgram::new(
        "dev.npm",
        "npm",
        "Node.js npm commands and output",
        "dev",
        npm_rules(),
    )
    .with_detect_patterns(vec!["npm", "yarn", "pnpm", "node"])
}

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

fn go_program() -> SimpleProgram {
    SimpleProgram::new("dev.go", "go", "Go build and test output", "dev", go_rules())
        .with_detect_patterns(vec!["go build", "go run", "go mod", "go get", "go fmt", "go vet"])
}

// =============================================================================
// MAKE
// =============================================================================

fn make_rules() -> Vec<Rule> {
    vec![
        // Error markers
        Rule::new(r"\*\*\*\s+")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"^make(\[\d+\])?:\s+\*\*\*")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bError\s+\d+")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"\bStop\.")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        // File:line references
        Rule::new(r"^[\w\./\-]+:\d+:")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        // Entering/leaving directory
        Rule::new(r"make(\[\d+\])?: Entering directory")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"make(\[\d+\])?: Leaving directory")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        // Target names
        Rule::new(r"make(\[\d+\])?: Nothing to be done for")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"make(\[\d+\])?: `\S+' is up to date")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        // Recipe execution
        Rule::new(r"^\t")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        common::number_rule(),
    ]
}

fn make_program() -> SimpleProgram {
    SimpleProgram::new(
        "dev.make",
        "make",
        "Make build output",
        "dev",
        make_rules(),
    )
    .with_detect_patterns(vec!["make", "gmake", "cmake"])
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

fn yarn_program() -> SimpleProgram {
    SimpleProgram::new(
        "dev.yarn",
        "yarn",
        "Yarn package manager output",
        "dev",
        yarn_rules(),
    )
    .with_detect_patterns(vec!["yarn"])
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

fn pnpm_program() -> SimpleProgram {
    SimpleProgram::new(
        "dev.pnpm",
        "pnpm",
        "pnpm package manager output",
        "dev",
        pnpm_rules(),
    )
    .with_detect_patterns(vec!["pnpm"])
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

fn elixir_program() -> SimpleProgram {
    SimpleProgram::new(
        "dev.elixir",
        "elixir",
        "Elixir/Mix build and test output",
        "dev",
        elixir_rules(),
    )
    .with_detect_patterns(vec!["mix", "elixir", "iex"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dev_programs_registered() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);
        assert_eq!(registry.len(), 8);
    }

    #[test]
    fn test_git_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("git diff");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "git");
    }

    #[test]
    fn test_cargo_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("cargo test");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "cargo");
    }

    #[test]
    fn test_npm_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("npm install");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "npm");
    }
}
