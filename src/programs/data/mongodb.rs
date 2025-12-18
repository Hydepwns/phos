//! `MongoDB` database colorization.

use std::sync::Arc;

use crate::category::Category;
use crate::colors::SemanticColor;
use crate::program::{Program, SimpleProgram};
use crate::rule::Rule;

use super::super::common;

fn mongodb_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // JSON-style timestamp
    rules.push(common::mongodb_timestamp_rule());

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
    rules.extend(common::server_lifecycle_rules());

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

#[must_use]
pub fn mongodb_program() -> Arc<dyn Program> {
    Arc::new(
        SimpleProgram::new(
            "data.mongodb",
            "mongodb",
            "MongoDB database logs",
            Category::Data,
            mongodb_rules(),
        )
        .with_detect_patterns(vec!["mongod", "mongodb", "mongos", "mongo "]),
    )
}
