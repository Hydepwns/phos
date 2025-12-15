//! RabbitMQ message broker colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn rabbitmq_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Erlang-style timestamp
    rules.push(
        Rule::new(r"\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2}\.\d+")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

    // Log levels
    rules.extend([
        Rule::new(r"\[emergency\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\[alert\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\[critical\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\[error\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"\[warning\]")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\[notice\]")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\[info\]")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\[debug\]")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Node names (Erlang format)
    rules.push(
        Rule::new(r"\brabbit@[\w\-\.]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // Queue/exchange/binding
    rules.extend([
        Rule::new(r"\bqueue\s+'[^']+'")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bexchange\s+'[^']+'")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bvhost\s+'[^']+'")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Connection events
    rules.extend([
        Rule::new(r"\baccepting AMQP connection\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bconnection .* closed\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\buser '[\w\-]+' authenticated\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    ]);

    // Cluster events
    rules.extend([
        Rule::new(r"\bcluster\s+(joined|left)\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bsynchronising\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\b(promoted|demoted)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    ]);

    // Server lifecycle
    rules.extend(common::server_lifecycle_rules());
    // RabbitMQ-specific lifecycle messages
    rules.extend([
        Rule::new(r"\bStarting RabbitMQ\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bStarted\s+\w+\s+application\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bServer startup complete\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
    ]);

    // Memory/resource alerts
    rules.extend([
        Rule::new(r"\bmemory\s+alarm\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"\bdisk\s+alarm\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"\bfile\s+descriptor\s+limit\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    // Erlang PIDs
    rules.push(
        Rule::new(r"<\d+\.\d+\.\d+>")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    rules.push(common::ipv4_rule());
    rules.push(common::size_rule());
    rules.push(common::number_rule());

    rules
}

pub fn rabbitmq_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "messaging.rabbitmq",
            "rabbitmq",
            "RabbitMQ message broker logs",
            Category::Messaging,
            rabbitmq_rules(),
        )
        .with_detect_patterns(vec!["rabbitmq", "rabbit", "rabbitmqctl"]),
    )
}
