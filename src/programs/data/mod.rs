//! Database and data store programs.
//!
//! Provides Program implementations for PostgreSQL, Redis, etc.

mod elasticsearch;
mod mongodb;
mod mysql;
mod postgres;
mod redis;

use crate::program::ProgramRegistry;

pub use elasticsearch::elasticsearch_program;
pub use mongodb::mongodb_program;
pub use mysql::mysql_program;
pub use postgres::postgres_program;
pub use redis::redis_program;

/// Register all Data programs with the registry.
pub fn register_all(registry: &mut ProgramRegistry) {
    registry.register(postgres_program());
    registry.register(redis_program());
    registry.register(mysql_program());
    registry.register(mongodb_program());
    registry.register(elasticsearch_program());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_programs_registered() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);
        assert_eq!(registry.len(), 5);
    }

    #[test]
    fn test_postgres_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("postgres -D /var/lib/postgresql/data");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "postgres");
    }

    #[test]
    fn test_redis_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("redis-server /etc/redis/redis.conf");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "redis");
    }
}
