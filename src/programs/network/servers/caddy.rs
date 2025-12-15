//! Caddy web server colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::SimpleProgram;
use crate::programs::common;
use crate::rule::Rule;

fn caddy_rules() -> Vec<Rule> {
    let mut rules = common::log_level_rules();

    // JSON structured log keys
    rules.extend([
        Rule::new(r#""level"\s*:\s*"error""#)
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r#""level"\s*:\s*"warn""#)
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r#""level"\s*:\s*"info""#)
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r#""level"\s*:\s*"debug""#)
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // HTTP methods and status
    rules.push(common::http_method_rule());
    rules.extend(common::http_status_rules());

    // JSON structured fields
    rules.extend([
        Rule::new(r#""status"\s*:\s*\d+"#)
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r#""duration"\s*:\s*[\d\.]+"#)
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r#""size"\s*:\s*\d+"#)
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r#""ts"\s*:\s*[\d\.]+"#)
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    ]);

    // Request/response info
    rules.extend([
        Rule::new(r#""request"\s*:\s*\{[^}]+\}"#)
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r#""uri"\s*:\s*"[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r#""remote_ip"\s*:\s*"[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    // Console format (non-JSON)
    rules.extend([
        Rule::new(r"\bserving initial configuration\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\btls\.\w+\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bhttp\.\w+\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    ]);

    rules.push(common::ipv4_rule());
    rules.push(common::iso_timestamp_rule());
    rules.push(common::number_rule());

    rules
}

pub fn caddy_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.caddy",
            "caddy",
            "Caddy web server logs",
            Category::Network,
            caddy_rules(),
        )
        .with_detect_patterns(vec!["caddy"]),
    )
}
