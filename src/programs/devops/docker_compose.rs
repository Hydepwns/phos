//! Docker Compose colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn docker_compose_rules() -> Vec<Rule> {
    let mut rules = common::log_level_rules();

    // Service prefix pattern: service-1 | log message
    rules.push(
        Rule::new(r"^[\w\-]+\-\d+\s*\|")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // Container lifecycle
    rules.extend([
        Rule::new(r"\b(Creating|Starting|Recreating)\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\b(Started|Created|Running)\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\b(Stopping|Removing|Killing)\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\b(Stopped|Removed|exited with code \d+)\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"\bAttaching to\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]);

    // Network and volume operations
    rules.extend([
        Rule::new(r"\bNetwork\s+[\w\-]+\s+(Created|Removed)")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bVolume\s+[\w\-]+\s+(Created|Removed)")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]);

    // Container and image IDs
    rules.extend(common::hex_id_rules().into_iter().take(2));
    rules.push(common::iso_timestamp_rule());
    rules.push(common::number_rule());

    rules
}

#[must_use]
pub fn docker_compose_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "devops.docker-compose",
            "docker-compose",
            "Docker Compose logs and output",
            Category::DevOps,
            docker_compose_rules(),
        )
        .with_detect_patterns(vec!["docker-compose", "docker compose"]),
    )
}
