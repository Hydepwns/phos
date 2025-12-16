//! Grafana server colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn grafana_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Grafana log format: t=timestamp level=info msg="message" logger=component
    rules.push(
        Rule::new(r"t=\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}[^\s]*")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

    // Log levels
    rules.extend(common::structured_log_level_rules());

    // Logger components
    rules.push(
        Rule::new(r"\blogger=[\w\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    // Message strings
    rules.push(
        Rule::new(r#"\bmsg="[^"]*""#)
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // Dashboard and panel operations
    rules.extend([
        Rule::new(r"\bdashboard\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bpanel\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\balert\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bdatasource\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    ]);

    // User/org info
    rules.extend([
        Rule::new(r"\buser=[\w@\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\borgId=\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"\buId=\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    ]);

    // HTTP request info
    rules.extend([
        Rule::new(r"\bmethod=(GET|POST|PUT|DELETE|PATCH)")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bstatus=\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"\bpath=/[^\s]+")
            .unwrap()
            .semantic(SemanticColor::Value)
            .build(),
    ]);

    // Errors
    rules.push(
        Rule::new(r#"\berror="[^"]*""#)
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
    );

    rules.push(common::key_value_rule());
    rules.push(common::ipv4_rule());
    rules.push(common::duration_rule());
    rules.push(common::number_rule());

    rules
}

#[must_use] pub fn grafana_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "monitoring.grafana",
            "grafana",
            "Grafana server logs",
            Category::Monitoring,
            grafana_rules(),
        )
        .with_detect_patterns(vec!["grafana", "grafana-server"]),
    )
}
