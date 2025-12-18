//! GitHub Actions colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

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

#[must_use]
pub fn github_actions_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "ci.github-actions",
            "github-actions",
            "GitHub Actions workflow output",
            Category::CI,
            github_actions_rules(),
        )
        .with_detect_patterns(vec!["github-actions", "actions/", "::group::", "##["]),
    )
}
