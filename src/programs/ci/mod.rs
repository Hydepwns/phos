//! CI/CD tool programs.
//!
//! Provides Program implementations for GitHub Actions, Jenkins, etc.

use std::sync::Arc;

use super::common;
use crate::colors::SemanticColor;
use crate::program::{ProgramRegistry, SimpleProgram};
use crate::rule::Rule;

/// Register all CI programs with the registry.
pub fn register_all(registry: &mut ProgramRegistry) {
    registry.register(Arc::new(github_actions_program()));
    registry.register(Arc::new(jenkins_program()));
}

// =============================================================================
// GITHUB ACTIONS
// =============================================================================

fn github_actions_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Workflow/job/step markers
    rules.extend([
        Rule::new(r"^##\[group\]")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"^##\[endgroup\]")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"^##\[error\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"^##\[warning\]")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"^##\[notice\]")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^##\[debug\]")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Action annotations
    rules.extend([
        Rule::new(r"::error::")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"::warning::")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"::notice::")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"::debug::")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Step status indicators
    rules.extend([
        Rule::new(r"^\s*\[command\]")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"^Run\s+")
            .unwrap()
            .semantic(SemanticColor::Info)
            .bold()
            .build(),
        Rule::new(r"^\s*with:")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"^\s*env:")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Action names
    rules.push(
        Rule::new(r"uses:\s*[\w\-]+/[\w\-]+@[\w\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // Job/step timing
    rules.extend([
        Rule::new(r"\bSucceeded\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bFailed\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\bCancelled\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bSkipped\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Checkout/setup
    rules.extend([
        Rule::new(r"\bactions/checkout\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bactions/setup-\w+\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bactions/cache\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    ]);

    // Cache hits
    rules.extend([
        Rule::new(r"\bCache hit\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bCache miss\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bCache restored\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    ]);

    // Download/upload artifacts
    rules.extend([
        Rule::new(r"\bDownloading\s+artifact\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bUploading\s+artifact\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]);

    // Standard output markers
    rules.extend(common::log_level_rules());
    rules.push(common::duration_rule());
    rules.push(common::size_rule());
    rules.push(common::number_rule());

    rules
}

fn github_actions_program() -> SimpleProgram {
    SimpleProgram::new(
        "ci.github-actions",
        "github-actions",
        "GitHub Actions workflow output",
        "ci",
        github_actions_rules(),
    )
    .with_detect_patterns(vec!["github-actions", "actions/", "::group::", "##["])
}

// =============================================================================
// JENKINS
// =============================================================================

fn jenkins_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Timestamps
    rules.push(
        Rule::new(r"\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+Z?\]")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

    // Pipeline stage markers
    rules.extend([
        Rule::new(r"\[Pipeline\]")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"\{\s*\(")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\bstage\s*\([^)]+\)")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    ]);

    // Build status
    rules.extend([
        Rule::new(r"\bSUCCESS\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bFAILURE\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\bUNSTABLE\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"\bABORTED\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bNOT_BUILT\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Step indicators
    rules.extend([
        Rule::new(r"\bRunning\s+in\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bBuilding\s+in\s+workspace\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bRunning\s+on\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]);

    // Agent/node info
    rules.extend([
        Rule::new(r"\bRunning\s+on\s+[\w\-]+\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bagent\s+\{[^}]*\}")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Script blocks
    rules.extend([
        Rule::new(r"^\+\s+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\bsh\s+'[^']*'")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bbat\s+'[^']*'")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    ]);

    // SCM operations
    rules.extend([
        Rule::new(r"\bChecking out\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bCloning\s+into\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bFetching\s+changes\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]);

    // Archived artifacts
    rules.extend([
        Rule::new(r"\bArchiving\s+artifacts\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bRecording\s+test\s+results\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]);

    // Test results
    rules.extend([
        Rule::new(r"Tests run: \d+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"Failures: \d+")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"Errors: \d+")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"Skipped: \d+")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    // Build complete message
    rules.push(
        Rule::new(r"\bFinished:\s*(SUCCESS|FAILURE|UNSTABLE|ABORTED)")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
    );

    // Standard log levels
    rules.extend(common::log_level_rules());
    rules.push(common::duration_rule());
    rules.push(common::size_rule());
    rules.push(common::number_rule());

    rules
}

fn jenkins_program() -> SimpleProgram {
    SimpleProgram::new(
        "ci.jenkins",
        "jenkins",
        "Jenkins build output",
        "ci",
        jenkins_rules(),
    )
    .with_detect_patterns(vec!["jenkins", "[Pipeline]", "Jenkinsfile"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ci_programs_registered() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn test_jenkins_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("jenkins build");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "jenkins");
    }
}
