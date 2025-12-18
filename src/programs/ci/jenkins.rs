//! Jenkins build output colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

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

#[must_use]
pub fn jenkins_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "ci.jenkins",
            "jenkins",
            "Jenkins build output",
            Category::CI,
            jenkins_rules(),
        )
        .with_detect_patterns(vec!["jenkins", "[Pipeline]", "Jenkinsfile"]),
    )
}
