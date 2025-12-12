//! Database and data store programs.
//!
//! Provides Program implementations for PostgreSQL, Redis, etc.

use std::sync::Arc;

use super::common;
use crate::colors::SemanticColor;
use crate::program::{ProgramRegistry, SimpleProgram};
use crate::rule::Rule;

/// Register all Data programs with the registry.
pub fn register_all(registry: &mut ProgramRegistry) {
    registry.register(Arc::new(postgres_program()));
    registry.register(Arc::new(redis_program()));
    registry.register(Arc::new(mysql_program()));
    registry.register(Arc::new(mongodb_program()));
    registry.register(Arc::new(elasticsearch_program()));
}

// =============================================================================
// POSTGRESQL
// =============================================================================

fn postgres_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Timestamps (2024-12-05 00:12:36.123 UTC)
    rules.push(
        Rule::new(r"\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2}\.\d{3}\s+\w+")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

    // PID in brackets
    rules.push(
        Rule::new(r"\[\d+\]")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // Log levels (PostgreSQL style)
    rules.extend([
        Rule::new(r"\bPANIC:")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bFATAL:")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bERROR:")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"\bWARNING:")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bNOTICE:")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bLOG:")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bDEBUG\d?:")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\bINFO:")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]);

    // SQL-related keywords
    rules.extend([
        Rule::new(r"\b(SELECT|INSERT|UPDATE|DELETE|CREATE|DROP|ALTER|TRUNCATE)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bstatement:\s*")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Connection info
    rules.extend([
        Rule::new(r"\bconnection received:")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bconnection authorized:")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bdisconnection:")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bauthentication failed")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    ]);

    // Database/user names
    rules.extend([
        Rule::new(r#"\bdatabase\s+"\w+""#)
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r#"\buser\s+"\w+""#)
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    // Error details
    rules.extend([
        Rule::new(r"\bDETAIL:")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\bHINT:")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bCONTEXT:")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\bSTATEMENT:")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Checkpoint and WAL
    rules.extend([
        Rule::new(r"\bcheckpoint (starting|complete)\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\brecovery\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    rules.push(common::ipv4_rule());
    rules.push(common::duration_rule());
    rules.push(common::number_rule());

    rules
}

fn postgres_program() -> SimpleProgram {
    SimpleProgram::new(
        "data.postgres",
        "postgres",
        "PostgreSQL database logs",
        "data",
        postgres_rules(),
    )
    .with_detect_patterns(vec!["postgres", "postgresql", "psql", "pg_"])
}

// =============================================================================
// REDIS
// =============================================================================

fn redis_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // PID:role format (12345:M, 12345:S, 12345:C)
    rules.push(
        Rule::new(r"\d+:[MSCRX]\s+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // Timestamps (05 Dec 2024 00:12:36.123)
    rules.push(
        Rule::new(r"\d{2}\s+\w{3}\s+\d{4}\s+\d{2}:\d{2}:\d{2}\.\d{3}")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

    // Log level symbols
    rules.extend([
        Rule::new(r"^\s*#\s+")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"^\s*\*\s+")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"^\s*-\s+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"^\s*\.\s+")
            .unwrap()
            .semantic(SemanticColor::Trace)
            .build(),
    ]);

    // Warning messages
    rules.extend([
        Rule::new(r"\bWARNING\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"\bovercommit_memory\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    // Server lifecycle
    rules.extend([
        Rule::new(r"\bReady to accept connections\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bServer initialized\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bShutdown completed\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bDB loaded from disk\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    ]);

    // Replication
    rules.extend([
        Rule::new(r"\bSYNC\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bREPLICATION\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\bmaster\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\breplica\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    ]);

    // Memory and persistence
    rules.extend([
        Rule::new(r"\bRDB\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\bAOF\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\bBackground saving\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]);

    rules.push(common::ipv4_rule());
    rules.push(common::size_rule());
    rules.push(common::number_rule());

    rules
}

fn redis_program() -> SimpleProgram {
    SimpleProgram::new(
        "data.redis",
        "redis",
        "Redis server logs",
        "data",
        redis_rules(),
    )
    .with_detect_patterns(vec!["redis", "redis-server", "redis-cli"])
}

// =============================================================================
// MYSQL / MARIADB
// =============================================================================

fn mysql_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Timestamps (2024-12-05T00:12:36.123456Z or 2024-12-05 00:12:36)
    rules.extend([
        Rule::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+Z?")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
        Rule::new(r"\d{6}\s+\d{1,2}:\d{2}:\d{2}")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    ]);

    // Thread ID
    rules.push(
        Rule::new(r"\[\d+\]")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // Log levels
    rules.extend([
        Rule::new(r"\[ERROR\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\[Warning\]")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\[Note\]")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\[System\]")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    ]);

    // InnoDB specific
    rules.extend([
        Rule::new(r"\bInnoDB:\s*")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\bbuffer pool\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bredo log\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    ]);

    // Server lifecycle
    rules.extend([
        Rule::new(r"\bready for connections\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bShutdown complete\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bStarting\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]);

    // SQL keywords
    rules.push(
        Rule::new(r"\b(SELECT|INSERT|UPDATE|DELETE|CREATE|DROP|ALTER|GRANT|REVOKE)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    // Replication
    rules.extend([
        Rule::new(r"\b(Binlog|binlog)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\b(Slave|Master|Replica|Source)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bGTID\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    // Errors
    rules.extend([
        Rule::new(r"\bAccess denied\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
        Rule::new(r"\bAborted connection\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bDeadlock found\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
    ]);

    rules.push(common::ipv4_rule());
    rules.push(common::size_rule());
    rules.push(common::number_rule());

    rules
}

fn mysql_program() -> SimpleProgram {
    SimpleProgram::new(
        "data.mysql",
        "mysql",
        "MySQL/MariaDB database logs",
        "data",
        mysql_rules(),
    )
    .with_detect_patterns(vec!["mysql", "mariadb", "mysqld", "mariadbd"])
}

// =============================================================================
// MONGODB
// =============================================================================

fn mongodb_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // JSON-style timestamp
    rules.push(
        Rule::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+[+-]\d{4}")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

    // Severity levels (MongoDB style: F, E, W, I, D)
    rules.extend([
        Rule::new(r"\s+F\s+")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\s+E\s+")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"\s+W\s+")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\s+I\s+")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\s+D\d?\s+")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Component names
    rules.push(
        Rule::new(r"\b(ACCESS|COMMAND|CONTROL|FTDC|GEO|INDEX|NETWORK|QUERY|REPL|SHARDING|STORAGE|WRITE)\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // Connection info
    rules.extend([
        Rule::new(r"\bconnection accepted\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bend connection\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bconn\d+\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    // Operations
    rules.extend([
        Rule::new(r"\b(find|insert|update|delete|aggregate|getMore)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bplanSummary:\s*\w+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Replica set
    rules.extend([
        Rule::new(r"\b(PRIMARY|SECONDARY|ARBITER|RECOVERING)\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .bold()
            .build(),
        Rule::new(r"\breplSet\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\bvote\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]);

    // Server lifecycle
    rules.extend([
        Rule::new(r"\bWaiting for connections\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bshutting down\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    // Slow queries
    rules.push(
        Rule::new(r"\d+ms\b")
            .unwrap()
            .semantic(SemanticColor::Metric)
            .build(),
    );

    // JSON attributes
    rules.push(
        Rule::new(r#""\w+":\s*"#)
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    rules.push(common::ipv4_rule());
    rules.push(common::size_rule());
    rules.push(common::number_rule());

    rules
}

fn mongodb_program() -> SimpleProgram {
    SimpleProgram::new(
        "data.mongodb",
        "mongodb",
        "MongoDB database logs",
        "data",
        mongodb_rules(),
    )
    .with_detect_patterns(vec!["mongod", "mongodb", "mongos", "mongo "])
}

// =============================================================================
// ELASTICSEARCH
// =============================================================================

fn elasticsearch_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Timestamp in brackets
    rules.push(
        Rule::new(r"\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2},\d{3}\]")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

    // Log levels
    rules.extend([
        Rule::new(r"\[FATAL\s*\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\[ERROR\s*\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"\[WARN\s*\]")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\[INFO\s*\]")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\[DEBUG\s*\]")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\[TRACE\s*\]")
            .unwrap()
            .semantic(SemanticColor::Trace)
            .build(),
    ]);

    // Component/logger names in brackets
    rules.push(
        Rule::new(r"\[[\w\.]+\s*\]")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // Node info
    rules.extend([
        Rule::new(r"\[node-\d+\]")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bnode\s*\{[^}]+\}")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    // Cluster health
    rules.extend([
        Rule::new(r"\bstatus\s*\[green\]")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bstatus\s*\[yellow\]")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .bold()
            .build(),
        Rule::new(r"\bstatus\s*\[red\]")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
    ]);

    // Index operations
    rules.extend([
        Rule::new(r"\bindex\s*\[[^\]]+\]")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bshard\s*\[\d+\]")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\b(creating|closing|deleting)\s+index\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
    ]);

    // Cluster events
    rules.extend([
        Rule::new(r"\bcluster state updated\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bmaster node changed\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bnew master\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .bold()
            .build(),
    ]);

    // Server lifecycle
    rules.extend([
        Rule::new(r"\bstarted\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\binitialized\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bstopping\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    // GC logs
    rules.extend([
        Rule::new(r"\[gc\]\[\w+\]")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"GC overhead")
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

fn elasticsearch_program() -> SimpleProgram {
    SimpleProgram::new(
        "data.elasticsearch",
        "elasticsearch",
        "Elasticsearch cluster logs",
        "data",
        elasticsearch_rules(),
    )
    .with_detect_patterns(vec!["elasticsearch", "elastic"])
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
