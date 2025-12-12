//! Message queue and streaming programs.
//!
//! Provides Program implementations for Kafka, RabbitMQ, etc.

use std::sync::Arc;

use super::common;
use crate::colors::SemanticColor;
use crate::program::{ProgramRegistry, SimpleProgram};
use crate::rule::Rule;

/// Register all Messaging programs with the registry.
pub fn register_all(registry: &mut ProgramRegistry) {
    registry.register(Arc::new(kafka_program()));
    registry.register(Arc::new(rabbitmq_program()));
}

// =============================================================================
// KAFKA
// =============================================================================

fn kafka_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Log4j timestamp format
    rules.push(
        Rule::new(r"\[\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2},\d{3}\]")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

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
    rules.extend([
        Rule::new(r"\bstarted\s*\(kafka\.server")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bshutting down\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    rules.push(common::ipv4_rule());
    rules.push(common::size_rule());
    rules.push(common::duration_rule());
    rules.push(common::number_rule());

    rules
}

fn kafka_program() -> SimpleProgram {
    SimpleProgram::new(
        "messaging.kafka",
        "kafka",
        "Apache Kafka broker logs",
        "messaging",
        kafka_rules(),
    )
    .with_detect_patterns(vec!["kafka", "kafka-server", "zookeeper"])
}

// =============================================================================
// RABBITMQ
// =============================================================================

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
        Rule::new(r"\bShutdown\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
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

fn rabbitmq_program() -> SimpleProgram {
    SimpleProgram::new(
        "messaging.rabbitmq",
        "rabbitmq",
        "RabbitMQ message broker logs",
        "messaging",
        rabbitmq_rules(),
    )
    .with_detect_patterns(vec!["rabbitmq", "rabbit", "rabbitmqctl"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_messaging_programs_registered() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn test_kafka_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("kafka-server-start.sh config/server.properties");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "kafka");
    }

    #[test]
    fn test_rabbitmq_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("rabbitmq-server");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "rabbitmq");
    }
}
