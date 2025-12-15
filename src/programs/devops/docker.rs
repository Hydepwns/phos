//! Docker container colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn docker_rules() -> Vec<Rule> {
    let mut rules = common::log_level_rules();

    // Container status (running, exited, created, restarting, paused, dead)
    rules.extend(common::container_status_rules());

    // Container/image IDs (64-char and 12-char)
    rules.extend(common::hex_id_rules().into_iter().take(2));

    // Image names with tags
    rules.extend([
        Rule::new(r"[\w\-\.]+/[\w\-\.]+:[\w\-\.]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"[\w\-\.]+:[\w\-\.]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Ports
    rules.extend([
        Rule::new(r"\d{1,5}->\d{1,5}/\w+")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
        Rule::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:\d{1,5}")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    ]);

    // Common patterns
    rules.push(common::size_rule());
    rules.push(common::iso_timestamp_rule());
    rules.push(common::iso_timestamp_space_rule());
    rules.push(common::percentage_rule());
    rules.push(common::number_rule());

    rules
}

pub fn docker_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "devops.docker",
            "Docker",
            "Docker container logs and commands",
            Category::DevOps,
            docker_rules(),
        )
        .with_detect_patterns(vec!["docker", "docker-compose", "podman"]),
    )
}
