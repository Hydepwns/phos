//! MySQL/MariaDB database colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn mysql_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Timestamps (2024-12-05T00:12:36.123456Z or YYMMDD HH:MM:SS)
    rules.push(common::mysql_iso_timestamp_rule());
    rules.push(common::mysql_legacy_timestamp_rule());

    // Thread ID
    rules.push(
        Rule::new(r"\[\d+\]")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // Log levels
    rules.extend(common::bracketed_log_level_rules());
    // MySQL-specific [System] level
    rules.push(
        Rule::new(r"\[System\]")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

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
    rules.extend(common::server_lifecycle_rules());

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

#[must_use] pub fn mysql_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "data.mysql",
            "mysql",
            "MySQL/MariaDB database logs",
            Category::Data,
            mysql_rules(),
        )
        .with_detect_patterns(vec!["mysql", "mariadb", "mysqld", "mariadbd"]),
    )
}
