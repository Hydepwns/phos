//! Datadog agent colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn datadog_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Datadog agent timestamp format
    rules.push(
        Rule::new(r"\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2}\s+\w+")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

    // Log levels (Datadog format)
    rules.extend([
        Rule::new(r"\bERROR\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bWARN\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bINFO\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bDEBUG\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Component identifiers
    rules.extend([
        Rule::new(r"\([\w\.\-]+\)")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bcheck=[\w\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Metrics and checks
    rules.extend([
        Rule::new(r"\bmetric\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bservice_check\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bhost=[\w\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    // Status indicators
    rules.extend([
        Rule::new(r"\bOK\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bCRITICAL\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bWARNING\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bUNKNOWN\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // API key (masked)
    rules.push(
        Rule::new(r"\bapi_key=\*+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    rules.push(common::key_value_rule());
    rules.push(common::ipv4_rule());
    rules.push(common::duration_rule());
    rules.push(common::size_rule());
    rules.push(common::number_rule());

    rules
}

#[must_use] pub fn datadog_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "monitoring.datadog",
            "datadog",
            "Datadog agent logs",
            Category::Monitoring,
            datadog_rules(),
        )
        .with_detect_patterns(vec!["datadog", "datadog-agent", "dd-agent"]),
    )
}
