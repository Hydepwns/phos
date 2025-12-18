//! Redis server colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

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
    rules.push(common::redis_timestamp_rule());

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
    rules.extend(common::server_lifecycle_rules());
    rules.push(
        Rule::new(r"\bDB loaded from disk\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    );

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

#[must_use]
pub fn redis_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "data.redis",
            "redis",
            "Redis server logs",
            Category::Data,
            redis_rules(),
        )
        .with_detect_patterns(vec!["redis", "redis-server", "redis-cli"]),
    )
}
