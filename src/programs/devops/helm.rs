//! Helm package manager colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn helm_rules() -> Vec<Rule> {
    let mut rules = common::log_level_rules();

    // Release status
    rules.extend([
        Rule::new(r"\bSTATUS:\s*deployed\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bSTATUS:\s*(failed|superseded)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .bold()
            .build(),
        Rule::new(r"\bSTATUS:\s*(pending|pending-install|pending-upgrade|pending-rollback)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bSTATUS:\s*uninstalled\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Chart info
    rules.extend([
        Rule::new(r"\bCHART:\s*[\w\-]+[\d\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\bNAME:\s*[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bNAMESPACE:\s*[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bREVISION:\s*\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    ]);

    // Hooks and resources
    rules.extend([
        Rule::new(r"\b(pre-install|post-install|pre-upgrade|post-upgrade|pre-delete|post-delete)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"created|configured|unchanged")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    ]);

    rules.push(common::iso_timestamp_rule());
    rules.push(common::number_rule());

    rules
}

pub fn helm_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "devops.helm",
            "helm",
            "Helm package manager output",
            Category::DevOps,
            helm_rules(),
        )
        .with_detect_patterns(vec!["helm"]),
    )
}
