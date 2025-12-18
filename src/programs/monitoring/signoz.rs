//! `SigNoz` observability platform colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn signoz_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // SigNoz uses structured JSON-like logging
    rules.push(common::iso_timestamp_rule());

    // Log levels
    rules.extend(common::log_level_rules());

    // Component names
    rules.extend([
        Rule::new(r#""caller":"[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r#""component":"[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Traces and spans
    rules.extend([
        Rule::new(r"\btrace_id\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bspan_id\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\b[a-f0-9]{32}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    // Query engine
    rules.extend([
        Rule::new(r"\bClickHouse\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bquery\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r#""query":"[^"]*""#)
            .unwrap()
            .semantic(SemanticColor::Value)
            .build(),
    ]);

    // OpenTelemetry
    rules.extend([
        Rule::new(r"\bOTLP\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bOpenTelemetry\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bexporter\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\breceiver\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    rules.push(common::key_value_rule());
    rules.push(common::ipv4_rule());
    rules.push(common::duration_rule());
    rules.push(common::number_rule());

    rules
}

#[must_use]
pub fn signoz_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "monitoring.signoz",
            "signoz",
            "SigNoz observability platform logs",
            Category::Monitoring,
            signoz_rules(),
        )
        .with_detect_patterns(vec!["signoz", "signoz-otel-collector"]),
    )
}
