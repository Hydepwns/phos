//! K9s Kubernetes CLI colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn k9s_rules() -> Vec<Rule> {
    let mut rules = common::log_level_rules();

    // Kubernetes log level format: level=info
    rules.extend([
        Rule::new(r#"level=error"#)
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r#"level=warn"#)
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r#"level=info"#)
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r#"level=debug"#)
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Kubernetes resource names
    rules.extend([
        Rule::new(r"\b(pod|deployment|service|configmap|secret|namespace|node|ingress|pvc)s?/[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bnamespace=[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Key=value patterns (common in structured logs)
    rules.push(common::key_value_rule());

    // Timestamps
    rules.push(common::iso_timestamp_rule());
    rules.push(common::number_rule());

    rules
}

pub fn k9s_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "devops.k9s",
            "k9s",
            "K9s Kubernetes CLI logs",
            Category::DevOps,
            k9s_rules(),
        )
        .with_detect_patterns(vec!["k9s"]),
    )
}
