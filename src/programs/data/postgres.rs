//! `PostgreSQL` database colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn postgres_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Timestamps (2024-12-05 00:12:36.123 UTC)
    rules.push(common::postgres_timestamp_rule());

    // PID in brackets
    rules.push(
        Rule::new(r"\[\d+\]")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    // Log levels (PostgreSQL style)
    rules.extend(common::database_log_level_rules());

    // SQL-related keywords
    rules.extend(common::sql_keyword_rules());
    rules.push(
        Rule::new(r"\bstatement:\s*")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

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

#[must_use] pub fn postgres_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "data.postgres",
            "postgres",
            "PostgreSQL database logs",
            Category::Data,
            postgres_rules(),
        )
        .with_detect_patterns(vec!["postgres", "postgresql", "psql", "pg_"]),
    )
}
