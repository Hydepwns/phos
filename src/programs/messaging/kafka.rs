//! Apache Kafka colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn kafka_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Log4j timestamp format
    rules.push(common::log4j_timestamp_rule());

    // Log levels
    rules.extend([
        Rule::new(r"\bFATAL\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bERROR\b")
            .unwrap()
            .semantic(SemanticColor::Error)
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
        Rule::new(r"\bTRACE\b")
            .unwrap()
            .semantic(SemanticColor::Trace)
            .build(),
    ]);

    // Component/logger names
    rules.push(
        Rule::new(r"\b(kafka\.[\w\.]+)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // Broker/controller state
    rules.extend([
        Rule::new(r"\b(leader|follower|controller)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bbroker\.id[=:]\s*\d+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bnode\.id[=:]\s*\d+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    // Topic/partition info
    rules.extend([
        Rule::new(r"\btopic[=:]\s*[\w\-\.]+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bpartition[=:]\s*\d+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\boffset[=:]\s*\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    ]);

    // Consumer groups
    rules.extend([
        Rule::new(r"\bgroup\.id[=:]\s*[\w\-\.]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\bconsumer-\d+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\b(Joining|Leaving|Rebalancing)\s+group\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]);

    // Replication
    rules.extend([
        Rule::new(r"\bISR\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\b(in-sync|out-of-sync)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\breplica[=:]\s*\d+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    // Connection events
    rules.extend([
        Rule::new(r"\bEstablished session\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bClosing socket connection\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bConnection refused\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    ]);

    // Server lifecycle
    rules.extend(common::server_lifecycle_rules());
    // Kafka-specific startup message
    rules.push(
        Rule::new(r"\bstarted\s*\(kafka\.server")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
    );

    rules.push(common::ipv4_rule());
    rules.push(common::size_rule());
    rules.push(common::duration_rule());
    rules.push(common::number_rule());

    rules
}

#[must_use] pub fn kafka_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "messaging.kafka",
            "kafka",
            "Apache Kafka broker logs",
            Category::Messaging,
            kafka_rules(),
        )
        .with_detect_patterns(vec!["kafka", "kafka-server", "zookeeper"]),
    )
}
