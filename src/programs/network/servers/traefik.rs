//! Traefik reverse proxy colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::SimpleProgram;
use crate::programs::common;
use crate::rule::Rule;

fn traefik_rules() -> Vec<Rule> {
    let mut rules = common::log_level_rules();

    // JSON structured fields
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

    // HTTP status and methods
    rules.push(common::http_method_rule());
    rules.extend(common::http_status_rules());

    // Router/service/middleware names
    rules.extend([
        Rule::new(r#""RouterName"\s*:\s*"[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r#""ServiceName"\s*:\s*"[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r#""entryPointName"\s*:\s*"[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Access log fields
    rules.extend([
        Rule::new(r#""RequestPath"\s*:\s*"[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r#""ClientHost"\s*:\s*"[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r#""Duration"\s*:\s*\d+"#)
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    ]);

    // Provider events
    rules.extend([
        Rule::new(r"\bConfiguration loaded\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bServer configuration reloaded\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bprovider\s+\w+\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // TLS/ACME
    rules.extend([
        Rule::new(r"\bACME\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bLetsEncrypt\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bcertificate\s+(obtained|renewed)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    ]);

    // Health/readiness
    rules.extend([
        Rule::new(r"\b(healthy|unhealthy)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bservers\s+\[\d+\]")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    ]);

    rules.push(common::ipv4_rule());
    rules.push(common::iso_timestamp_rule());
    rules.push(common::number_rule());

    rules
}

pub fn traefik_program() -> Arc<SimpleProgram> {
    Arc::new(
        SimpleProgram::new(
            "network.traefik",
            "traefik",
            "Traefik reverse proxy logs",
            Category::Network,
            traefik_rules(),
        )
        .with_detect_patterns(vec!["traefik"]),
    )
}
