//! Message queue and streaming programs.
//!
//! Provides Program implementations for Kafka, RabbitMQ, etc.

mod kafka;
mod rabbitmq;

use crate::program::ProgramRegistry;

pub use kafka::kafka_program;
pub use rabbitmq::rabbitmq_program;

/// Register all Messaging programs with the registry.
pub fn register_all(registry: &mut ProgramRegistry) {
    registry.register(kafka_program());
    registry.register(rabbitmq_program());
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
